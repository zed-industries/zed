mod rate_completion_modal;

pub use rate_completion_modal::*;

use anyhow::{anyhow, Context as _, Result};
use client::Client;
use collections::{HashMap, HashSet, VecDeque};
use futures::AsyncReadExt;
use gpui::{
    actions, AppContext, AsyncAppContext, Context, EntityId, Global, Model, ModelContext,
    Subscription, Task,
};
use http_client::{HttpClient, Method};
use language::{
    language_settings::all_language_settings, Anchor, Buffer, BufferSnapshot, OffsetRangeExt,
    Point, ToOffset, ToPoint,
};
use language_models::LlmApiToken;
use rpc::{PredictEditsParams, PredictEditsResponse, EXPIRED_LLM_TOKEN_HEADER_NAME};
use std::{
    borrow::Cow,
    cmp,
    fmt::Write,
    future::Future,
    mem,
    ops::Range,
    path::Path,
    sync::Arc,
    time::{Duration, Instant},
};
use telemetry_events::InlineCompletionRating;
use util::ResultExt;
use uuid::Uuid;

const CURSOR_MARKER: &'static str = "<|user_cursor_is_here|>";
const START_OF_FILE_MARKER: &'static str = "<|start_of_file|>";
const EDITABLE_REGION_START_MARKER: &'static str = "<|editable_region_start|>";
const EDITABLE_REGION_END_MARKER: &'static str = "<|editable_region_end|>";
const BUFFER_CHANGE_GROUPING_INTERVAL: Duration = Duration::from_secs(1);

actions!(zeta, [ClearHistory]);

#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, Hash)]
pub struct InlineCompletionId(Uuid);

impl From<InlineCompletionId> for gpui::ElementId {
    fn from(value: InlineCompletionId) -> Self {
        gpui::ElementId::Uuid(value.0)
    }
}

impl std::fmt::Display for InlineCompletionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl InlineCompletionId {
    fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

#[derive(Clone)]
struct ZetaGlobal(Model<Zeta>);

impl Global for ZetaGlobal {}

#[derive(Clone)]
pub struct InlineCompletion {
    id: InlineCompletionId,
    path: Arc<Path>,
    excerpt_range: Range<usize>,
    edits: Arc<[(Range<Anchor>, String)]>,
    snapshot: BufferSnapshot,
    input_events: Arc<str>,
    input_excerpt: Arc<str>,
    output_excerpt: Arc<str>,
}

impl InlineCompletion {
    fn interpolate(&self, new_snapshot: BufferSnapshot) -> Option<Vec<(Range<Anchor>, String)>> {
        let mut edits = Vec::new();

        let mut user_edits = new_snapshot
            .edits_since::<usize>(&self.snapshot.version)
            .peekable();
        for (model_old_range, model_new_text) in self.edits.iter() {
            let model_offset_range = model_old_range.to_offset(&self.snapshot);
            while let Some(next_user_edit) = user_edits.peek() {
                if next_user_edit.old.end < model_offset_range.start {
                    user_edits.next();
                } else {
                    break;
                }
            }

            if let Some(user_edit) = user_edits.peek() {
                if user_edit.old.start > model_offset_range.end {
                    edits.push((model_old_range.clone(), model_new_text.clone()));
                } else if user_edit.old == model_offset_range {
                    let user_new_text = new_snapshot
                        .text_for_range(user_edit.new.clone())
                        .collect::<String>();

                    if let Some(model_suffix) = model_new_text.strip_prefix(&user_new_text) {
                        if !model_suffix.is_empty() {
                            edits.push((
                                new_snapshot.anchor_after(user_edit.new.end)
                                    ..new_snapshot.anchor_before(user_edit.new.end),
                                model_suffix.into(),
                            ));
                        }

                        user_edits.next();
                    } else {
                        return None;
                    }
                } else {
                    return None;
                }
            } else {
                edits.push((model_old_range.clone(), model_new_text.clone()));
            }
        }

        Some(edits)
    }
}

impl std::fmt::Debug for InlineCompletion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("InlineCompletion")
            .field("id", &self.id)
            .field("path", &self.path)
            .field("edits", &self.edits)
            .finish_non_exhaustive()
    }
}

pub struct Zeta {
    client: Arc<Client>,
    events: VecDeque<Event>,
    registered_buffers: HashMap<gpui::EntityId, RegisteredBuffer>,
    recent_completions: VecDeque<InlineCompletion>,
    rated_completions: HashSet<InlineCompletionId>,
    llm_token: LlmApiToken,
    _llm_token_subscription: Subscription,
}

impl Zeta {
    pub fn global(cx: &mut AppContext) -> Option<Model<Self>> {
        cx.try_global::<ZetaGlobal>().map(|global| global.0.clone())
    }

    pub fn register(client: Arc<Client>, cx: &mut AppContext) -> Model<Self> {
        Self::global(cx).unwrap_or_else(|| {
            let model = cx.new_model(|cx| Self::new(client, cx));
            cx.set_global(ZetaGlobal(model.clone()));
            model
        })
    }

    pub fn clear_history(&mut self) {
        self.events.clear();
    }

    fn new(client: Arc<Client>, cx: &mut ModelContext<Self>) -> Self {
        let refresh_llm_token_listener = language_models::RefreshLlmTokenListener::global(cx);

        Self {
            client,
            events: VecDeque::new(),
            recent_completions: VecDeque::new(),
            rated_completions: HashSet::default(),
            registered_buffers: HashMap::default(),
            llm_token: LlmApiToken::default(),
            _llm_token_subscription: cx.subscribe(
                &refresh_llm_token_listener,
                |this, _listener, _event, cx| {
                    let client = this.client.clone();
                    let llm_token = this.llm_token.clone();
                    cx.spawn(|_this, _cx| async move {
                        llm_token.refresh(&client).await?;
                        anyhow::Ok(())
                    })
                    .detach_and_log_err(cx);
                },
            ),
        }
    }

    fn push_event(&mut self, event: Event) {
        if let Some(Event::BufferChange {
            new_snapshot: last_new_snapshot,
            timestamp: last_timestamp,
            ..
        }) = self.events.back_mut()
        {
            // Coalesce edits for the same buffer when they happen one after the other.
            let Event::BufferChange {
                old_snapshot,
                new_snapshot,
                timestamp,
            } = &event;

            if timestamp.duration_since(*last_timestamp) <= BUFFER_CHANGE_GROUPING_INTERVAL
                && old_snapshot.remote_id() == last_new_snapshot.remote_id()
                && old_snapshot.version == last_new_snapshot.version
            {
                *last_new_snapshot = new_snapshot.clone();
                *last_timestamp = *timestamp;
                return;
            }
        }

        self.events.push_back(event);
        if self.events.len() > 10 {
            self.events.pop_front();
        }
    }

    pub fn register_buffer(&mut self, buffer: &Model<Buffer>, cx: &mut ModelContext<Self>) {
        let buffer_id = buffer.entity_id();
        let weak_buffer = buffer.downgrade();

        if let std::collections::hash_map::Entry::Vacant(entry) =
            self.registered_buffers.entry(buffer_id)
        {
            let snapshot = buffer.read(cx).snapshot();

            entry.insert(RegisteredBuffer {
                snapshot,
                _subscriptions: [
                    cx.subscribe(buffer, move |this, buffer, event, cx| {
                        this.handle_buffer_event(buffer, event, cx);
                    }),
                    cx.observe_release(buffer, move |this, _buffer, _cx| {
                        this.registered_buffers.remove(&weak_buffer.entity_id());
                    }),
                ],
            });
        };
    }

    fn handle_buffer_event(
        &mut self,
        buffer: Model<Buffer>,
        event: &language::BufferEvent,
        cx: &mut ModelContext<Self>,
    ) {
        match event {
            language::BufferEvent::Edited => {
                self.report_changes_for_buffer(&buffer, cx);
            }
            _ => {}
        }
    }

    pub fn request_completion_impl<F, R>(
        &mut self,
        buffer: &Model<Buffer>,
        position: language::Anchor,
        cx: &mut ModelContext<Self>,
        perform_predict_edits: F,
    ) -> Task<Result<InlineCompletion>>
    where
        F: FnOnce(Arc<Client>, LlmApiToken, PredictEditsParams) -> R + 'static,
        R: Future<Output = Result<PredictEditsResponse>> + Send + 'static,
    {
        let snapshot = self.report_changes_for_buffer(buffer, cx);
        let point = position.to_point(&snapshot);
        let offset = point.to_offset(&snapshot);
        let excerpt_range = excerpt_range_for_position(point, &snapshot);
        let events = self.events.clone();
        let path = snapshot
            .file()
            .map(|f| f.path().clone())
            .unwrap_or_else(|| Arc::from(Path::new("untitled")));

        let client = self.client.clone();
        let llm_token = self.llm_token.clone();

        cx.spawn(|this, mut cx| async move {
            let start = std::time::Instant::now();

            let mut input_events = String::new();
            for event in events {
                if !input_events.is_empty() {
                    input_events.push('\n');
                    input_events.push('\n');
                }
                input_events.push_str(&event.to_prompt());
            }
            let input_excerpt = prompt_for_excerpt(&snapshot, &excerpt_range, offset);

            log::debug!("Events:\n{}\nExcerpt:\n{}", input_events, input_excerpt);

            let body = PredictEditsParams {
                input_events: input_events.clone(),
                input_excerpt: input_excerpt.clone(),
            };

            let response = perform_predict_edits(client, llm_token, body).await?;

            let output_excerpt = response.output_excerpt;
            log::debug!("prediction took: {:?}", start.elapsed());
            log::debug!("completion response: {}", output_excerpt);

            let inline_completion = Self::process_completion_response(
                output_excerpt,
                &snapshot,
                excerpt_range,
                path,
                input_events,
                input_excerpt,
                &cx,
            )
            .await?;

            this.update(&mut cx, |this, cx| {
                this.recent_completions
                    .push_front(inline_completion.clone());
                if this.recent_completions.len() > 50 {
                    this.recent_completions.pop_back();
                }
                cx.notify();
            })?;

            Ok(inline_completion)
        })
    }

    // Generates several example completions of various states to fill the Zeta completion modal
    #[cfg(any(test, feature = "test-support"))]
    pub fn fill_with_fake_completions(&mut self, cx: &mut ModelContext<Self>) -> Task<()> {
        let test_buffer_text = indoc::indoc! {r#"a longggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggg line
            And maybe a short line

            Then a few lines

            and then another
            "#};

        let buffer = cx.new_model(|cx| Buffer::local(test_buffer_text, cx));
        let position = buffer.read(cx).anchor_before(Point::new(1, 0));

        let completion_tasks = vec![
            self.fake_completion(
                &buffer,
                position,
                PredictEditsResponse {
                    output_excerpt: format!("{EDITABLE_REGION_START_MARKER}
a longggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggg line
[here's an edit]
And maybe a short line
Then a few lines
and then another
{EDITABLE_REGION_END_MARKER}
                        ", ),
                },
                cx,
            ),
            self.fake_completion(
                &buffer,
                position,
                PredictEditsResponse {
                    output_excerpt: format!(r#"{EDITABLE_REGION_START_MARKER}
a longggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggg line
And maybe a short line
[and another edit]
Then a few lines
and then another
{EDITABLE_REGION_END_MARKER}
                        "#),
                },
                cx,
            ),
            self.fake_completion(
                &buffer,
                position,
                PredictEditsResponse {
                    output_excerpt: format!(r#"{EDITABLE_REGION_START_MARKER}
a longggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggg line
And maybe a short line

Then a few lines

and then another
{EDITABLE_REGION_END_MARKER}
                        "#),
                },
                cx,
            ),
            self.fake_completion(
                &buffer,
                position,
                PredictEditsResponse {
                    output_excerpt: format!(r#"{EDITABLE_REGION_START_MARKER}
a longggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggg line
And maybe a short line

Then a few lines

and then another
{EDITABLE_REGION_END_MARKER}
                        "#),
                },
                cx,
            ),
            self.fake_completion(
                &buffer,
                position,
                PredictEditsResponse {
                    output_excerpt: format!(r#"{EDITABLE_REGION_START_MARKER}
a longggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggg line
And maybe a short line
Then a few lines
[a third completion]
and then another
{EDITABLE_REGION_END_MARKER}
                        "#),
                },
                cx,
            ),
            self.fake_completion(
                &buffer,
                position,
                PredictEditsResponse {
                    output_excerpt: format!(r#"{EDITABLE_REGION_START_MARKER}
a longggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggg line
And maybe a short line
and then another
[fourth completion example]
{EDITABLE_REGION_END_MARKER}
                        "#),
                },
                cx,
            ),
            self.fake_completion(
                &buffer,
                position,
                PredictEditsResponse {
                    output_excerpt: format!(r#"{EDITABLE_REGION_START_MARKER}
a longggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggg line
And maybe a short line
Then a few lines
and then another
[fifth and final completion]
{EDITABLE_REGION_END_MARKER}
                        "#),
                },
                cx,
            ),
        ];

        cx.spawn(|zeta, mut cx| async move {
            for task in completion_tasks {
                task.await.unwrap();
            }

            zeta.update(&mut cx, |zeta, _cx| {
                zeta.recent_completions.get_mut(2).unwrap().edits = Arc::new([]);
                zeta.recent_completions.get_mut(3).unwrap().edits = Arc::new([]);
            })
            .ok();
        })
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn fake_completion(
        &mut self,
        buffer: &Model<Buffer>,
        position: language::Anchor,
        response: PredictEditsResponse,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<InlineCompletion>> {
        use std::future::ready;

        self.request_completion_impl(buffer, position, cx, |_, _, _| ready(Ok(response)))
    }

    pub fn request_completion(
        &mut self,
        buffer: &Model<Buffer>,
        position: language::Anchor,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<InlineCompletion>> {
        self.request_completion_impl(buffer, position, cx, Self::perform_predict_edits)
    }

    fn perform_predict_edits(
        client: Arc<Client>,
        llm_token: LlmApiToken,
        body: PredictEditsParams,
    ) -> impl Future<Output = Result<PredictEditsResponse>> {
        async move {
            let http_client = client.http_client();
            let mut token = llm_token.acquire(&client).await?;
            let mut did_retry = false;

            loop {
                let request_builder = http_client::Request::builder();
                let request = request_builder
                    .method(Method::POST)
                    .uri(
                        http_client
                            .build_zed_llm_url("/predict_edits", &[])?
                            .as_ref(),
                    )
                    .header("Content-Type", "application/json")
                    .header("Authorization", format!("Bearer {}", token))
                    .body(serde_json::to_string(&body)?.into())?;

                let mut response = http_client.send(request).await?;

                if response.status().is_success() {
                    let mut body = String::new();
                    response.body_mut().read_to_string(&mut body).await?;
                    return Ok(serde_json::from_str(&body)?);
                } else if !did_retry
                    && response
                        .headers()
                        .get(EXPIRED_LLM_TOKEN_HEADER_NAME)
                        .is_some()
                {
                    did_retry = true;
                    token = llm_token.refresh(&client).await?;
                } else {
                    let mut body = String::new();
                    response.body_mut().read_to_string(&mut body).await?;
                    return Err(anyhow!(
                        "error predicting edits.\nStatus: {:?}\nBody: {}",
                        response.status(),
                        body
                    ));
                }
            }
        }
    }

    fn process_completion_response(
        output_excerpt: String,
        snapshot: &BufferSnapshot,
        excerpt_range: Range<usize>,
        path: Arc<Path>,
        input_events: String,
        input_excerpt: String,
        cx: &AsyncAppContext,
    ) -> Task<Result<InlineCompletion>> {
        let snapshot = snapshot.clone();
        cx.background_executor().spawn(async move {
            let content = output_excerpt.replace(CURSOR_MARKER, "");

            let codefence_start = content
                .find(EDITABLE_REGION_START_MARKER)
                .context("could not find start marker")?;
            let content = &content[codefence_start..];

            let newline_ix = content.find('\n').context("could not find newline")?;
            let content = &content[newline_ix + 1..];

            let codefence_end = content
                .rfind(&format!("\n{EDITABLE_REGION_END_MARKER}"))
                .context("could not find end marker")?;
            let new_text = &content[..codefence_end];

            let old_text = snapshot
                .text_for_range(excerpt_range.clone())
                .collect::<String>();

            let edits = Self::compute_edits(old_text, new_text, excerpt_range.start, &snapshot);

            Ok(InlineCompletion {
                id: InlineCompletionId::new(),
                path,
                excerpt_range,
                edits: edits.into(),
                snapshot: snapshot.clone(),
                input_events: input_events.into(),
                input_excerpt: input_excerpt.into(),
                output_excerpt: output_excerpt.into(),
            })
        })
    }

    pub fn compute_edits(
        old_text: String,
        new_text: &str,
        offset: usize,
        snapshot: &BufferSnapshot,
    ) -> Vec<(Range<Anchor>, String)> {
        let diff = similar::TextDiff::from_words(old_text.as_str(), new_text);

        let mut edits: Vec<(Range<usize>, String)> = Vec::new();
        let mut old_start = offset;
        for change in diff.iter_all_changes() {
            let value = change.value();
            match change.tag() {
                similar::ChangeTag::Equal => {
                    old_start += value.len();
                }
                similar::ChangeTag::Delete => {
                    let old_end = old_start + value.len();
                    if let Some((last_old_range, _)) = edits.last_mut() {
                        if last_old_range.end == old_start {
                            last_old_range.end = old_end;
                        } else {
                            edits.push((old_start..old_end, String::new()));
                        }
                    } else {
                        edits.push((old_start..old_end, String::new()));
                    }
                    old_start = old_end;
                }
                similar::ChangeTag::Insert => {
                    if let Some((last_old_range, last_new_text)) = edits.last_mut() {
                        if last_old_range.end == old_start {
                            last_new_text.push_str(value);
                        } else {
                            edits.push((old_start..old_start, value.into()));
                        }
                    } else {
                        edits.push((old_start..old_start, value.into()));
                    }
                }
            }
        }

        edits
            .into_iter()
            .map(|(mut old_range, new_text)| {
                let prefix_len = common_prefix(
                    snapshot.chars_for_range(old_range.clone()),
                    new_text.chars(),
                );
                old_range.start += prefix_len;
                let suffix_len = common_prefix(
                    snapshot.reversed_chars_for_range(old_range.clone()),
                    new_text[prefix_len..].chars().rev(),
                );
                old_range.end = old_range.end.saturating_sub(suffix_len);

                let new_text = new_text[prefix_len..new_text.len() - suffix_len].to_string();
                (
                    snapshot.anchor_after(old_range.start)..snapshot.anchor_before(old_range.end),
                    new_text,
                )
            })
            .collect()
    }

    pub fn is_completion_rated(&self, completion_id: InlineCompletionId) -> bool {
        self.rated_completions.contains(&completion_id)
    }

    pub fn rate_completion(
        &mut self,
        completion: &InlineCompletion,
        rating: InlineCompletionRating,
        feedback: String,
        cx: &mut ModelContext<Self>,
    ) {
        self.rated_completions.insert(completion.id);
        self.client
            .telemetry()
            .report_inline_completion_rating_event(
                rating,
                completion.input_events.clone(),
                completion.input_excerpt.clone(),
                completion.output_excerpt.clone(),
                feedback,
            );
        self.client.telemetry().flush_events();
        cx.notify();
    }

    pub fn recent_completions(&self) -> impl DoubleEndedIterator<Item = &InlineCompletion> {
        self.recent_completions.iter()
    }

    pub fn recent_completions_len(&self) -> usize {
        self.recent_completions.len()
    }

    fn report_changes_for_buffer(
        &mut self,
        buffer: &Model<Buffer>,
        cx: &mut ModelContext<Self>,
    ) -> BufferSnapshot {
        self.register_buffer(buffer, cx);

        let registered_buffer = self
            .registered_buffers
            .get_mut(&buffer.entity_id())
            .unwrap();
        let new_snapshot = buffer.read(cx).snapshot();

        if new_snapshot.version != registered_buffer.snapshot.version {
            let old_snapshot = mem::replace(&mut registered_buffer.snapshot, new_snapshot.clone());
            self.push_event(Event::BufferChange {
                old_snapshot,
                new_snapshot: new_snapshot.clone(),
                timestamp: Instant::now(),
            });
        }

        new_snapshot
    }
}

fn common_prefix<T1: Iterator<Item = char>, T2: Iterator<Item = char>>(a: T1, b: T2) -> usize {
    a.zip(b)
        .take_while(|(a, b)| a == b)
        .map(|(a, _)| a.len_utf8())
        .sum()
}

fn prompt_for_excerpt(
    snapshot: &BufferSnapshot,
    excerpt_range: &Range<usize>,
    offset: usize,
) -> String {
    let mut prompt_excerpt = String::new();
    writeln!(
        prompt_excerpt,
        "```{}",
        snapshot
            .file()
            .map_or(Cow::Borrowed("untitled"), |file| file
                .path()
                .to_string_lossy())
    )
    .unwrap();

    if excerpt_range.start == 0 {
        writeln!(prompt_excerpt, "{START_OF_FILE_MARKER}").unwrap();
    }

    let point_range = excerpt_range.to_point(snapshot);
    if point_range.start.row > 0 && !snapshot.is_line_blank(point_range.start.row - 1) {
        let extra_context_line_range = Point::new(point_range.start.row - 1, 0)..point_range.start;
        for chunk in snapshot.text_for_range(extra_context_line_range) {
            prompt_excerpt.push_str(chunk);
        }
    }
    writeln!(prompt_excerpt, "{EDITABLE_REGION_START_MARKER}").unwrap();
    for chunk in snapshot.text_for_range(excerpt_range.start..offset) {
        prompt_excerpt.push_str(chunk);
    }
    prompt_excerpt.push_str(CURSOR_MARKER);
    for chunk in snapshot.text_for_range(offset..excerpt_range.end) {
        prompt_excerpt.push_str(chunk);
    }
    write!(prompt_excerpt, "\n{EDITABLE_REGION_END_MARKER}").unwrap();

    if point_range.end.row < snapshot.max_point().row
        && !snapshot.is_line_blank(point_range.end.row + 1)
    {
        let extra_context_line_range = point_range.end
            ..Point::new(
                point_range.end.row + 1,
                snapshot.line_len(point_range.end.row + 1),
            );
        for chunk in snapshot.text_for_range(extra_context_line_range) {
            prompt_excerpt.push_str(chunk);
        }
    }

    write!(prompt_excerpt, "\n```").unwrap();
    prompt_excerpt
}

fn excerpt_range_for_position(point: Point, snapshot: &BufferSnapshot) -> Range<usize> {
    const CONTEXT_LINES: u32 = 16;

    let mut context_lines_before = CONTEXT_LINES;
    let mut context_lines_after = CONTEXT_LINES;
    if point.row < CONTEXT_LINES {
        context_lines_after += CONTEXT_LINES - point.row;
    } else if point.row + CONTEXT_LINES > snapshot.max_point().row {
        context_lines_before += (point.row + CONTEXT_LINES) - snapshot.max_point().row;
    }

    let excerpt_start_row = point.row.saturating_sub(context_lines_before);
    let excerpt_start = Point::new(excerpt_start_row, 0);
    let excerpt_end_row = cmp::min(point.row + context_lines_after, snapshot.max_point().row);
    let excerpt_end = Point::new(excerpt_end_row, snapshot.line_len(excerpt_end_row));
    excerpt_start.to_offset(snapshot)..excerpt_end.to_offset(snapshot)
}

struct RegisteredBuffer {
    snapshot: BufferSnapshot,
    _subscriptions: [gpui::Subscription; 2],
}

#[derive(Clone)]
enum Event {
    BufferChange {
        old_snapshot: BufferSnapshot,
        new_snapshot: BufferSnapshot,
        timestamp: Instant,
    },
}

impl Event {
    fn to_prompt(&self) -> String {
        match self {
            Event::BufferChange {
                old_snapshot,
                new_snapshot,
                ..
            } => {
                let mut prompt = String::new();

                let old_path = old_snapshot
                    .file()
                    .map(|f| f.path().as_ref())
                    .unwrap_or(Path::new("untitled"));
                let new_path = new_snapshot
                    .file()
                    .map(|f| f.path().as_ref())
                    .unwrap_or(Path::new("untitled"));
                if old_path != new_path {
                    writeln!(prompt, "User renamed {:?} to {:?}\n", old_path, new_path).unwrap();
                }

                let diff =
                    similar::TextDiff::from_lines(&old_snapshot.text(), &new_snapshot.text())
                        .unified_diff()
                        .to_string();
                if !diff.is_empty() {
                    write!(
                        prompt,
                        "User edited {:?}:\n```diff\n{}\n```",
                        new_path, diff
                    )
                    .unwrap();
                }

                prompt
            }
        }
    }
}

struct CurrentInlineCompletion {
    buffer_id: EntityId,
    completion: InlineCompletion,
}

pub struct ZetaInlineCompletionProvider {
    zeta: Model<Zeta>,
    current_completion: Option<CurrentInlineCompletion>,
    pending_refresh: Task<()>,
}

impl ZetaInlineCompletionProvider {
    pub const DEBOUNCE_TIMEOUT: Duration = Duration::from_millis(75);

    pub fn new(zeta: Model<Zeta>) -> Self {
        Self {
            zeta,
            current_completion: None,
            pending_refresh: Task::ready(()),
        }
    }
}

impl inline_completion::InlineCompletionProvider for ZetaInlineCompletionProvider {
    fn name() -> &'static str {
        "Zeta"
    }

    fn is_enabled(
        &self,
        buffer: &Model<Buffer>,
        cursor_position: language::Anchor,
        cx: &AppContext,
    ) -> bool {
        let buffer = buffer.read(cx);
        let file = buffer.file();
        let language = buffer.language_at(cursor_position);
        let settings = all_language_settings(file, cx);
        settings.inline_completions_enabled(language.as_ref(), file.map(|f| f.path().as_ref()), cx)
    }

    fn refresh(
        &mut self,
        buffer: Model<Buffer>,
        position: language::Anchor,
        debounce: bool,
        cx: &mut ModelContext<Self>,
    ) {
        self.pending_refresh =
            cx.spawn(|this, mut cx| async move {
                if debounce {
                    cx.background_executor().timer(Self::DEBOUNCE_TIMEOUT).await;
                }

                let completion_request = this.update(&mut cx, |this, cx| {
                    this.zeta.update(cx, |zeta, cx| {
                        zeta.request_completion(&buffer, position, cx)
                    })
                });

                let mut completion = None;
                if let Ok(completion_request) = completion_request {
                    completion = completion_request.await.log_err().map(|completion| {
                        CurrentInlineCompletion {
                            buffer_id: buffer.entity_id(),
                            completion,
                        }
                    });
                }

                this.update(&mut cx, |this, cx| {
                    this.current_completion = completion;
                    cx.notify();
                })
                .ok();
            });
    }

    fn cycle(
        &mut self,
        _buffer: Model<Buffer>,
        _cursor_position: language::Anchor,
        _direction: inline_completion::Direction,
        _cx: &mut ModelContext<Self>,
    ) {
        // Right now we don't support cycling.
    }

    fn accept(&mut self, _cx: &mut ModelContext<Self>) {}

    fn discard(&mut self, _cx: &mut ModelContext<Self>) {
        self.current_completion.take();
    }

    fn suggest(
        &mut self,
        buffer: &Model<Buffer>,
        cursor_position: language::Anchor,
        cx: &mut ModelContext<Self>,
    ) -> Option<inline_completion::InlineCompletion> {
        let CurrentInlineCompletion {
            buffer_id,
            completion,
        } = self.current_completion.as_mut()?;

        // Invalidate previous completion if it was generated for a different buffer.
        if *buffer_id != buffer.entity_id() {
            self.current_completion.take();
            return None;
        }

        let buffer = buffer.read(cx);
        let Some(edits) = completion.interpolate(buffer.snapshot()) else {
            self.current_completion.take();
            return None;
        };

        let cursor_row = cursor_position.to_point(buffer).row;
        let (closest_edit_ix, (closest_edit_range, _)) =
            edits.iter().enumerate().min_by_key(|(_, (range, _))| {
                let distance_from_start = cursor_row.abs_diff(range.start.to_point(buffer).row);
                let distance_from_end = cursor_row.abs_diff(range.end.to_point(buffer).row);
                cmp::min(distance_from_start, distance_from_end)
            })?;

        let mut edit_start_ix = closest_edit_ix;
        for (range, _) in edits[..edit_start_ix].iter().rev() {
            let distance_from_closest_edit =
                closest_edit_range.start.to_point(buffer).row - range.end.to_point(buffer).row;
            if distance_from_closest_edit <= 1 {
                edit_start_ix -= 1;
            } else {
                break;
            }
        }

        let mut edit_end_ix = closest_edit_ix + 1;
        for (range, _) in &edits[edit_end_ix..] {
            let distance_from_closest_edit =
                range.start.to_point(buffer).row - closest_edit_range.end.to_point(buffer).row;
            if distance_from_closest_edit <= 1 {
                edit_end_ix += 1;
            } else {
                break;
            }
        }

        Some(inline_completion::InlineCompletion {
            edits: edits[edit_start_ix..edit_end_ix].to_vec(),
        })
    }
}

#[cfg(test)]
mod tests {
    use client::test::FakeServer;
    use clock::FakeSystemClock;
    use gpui::TestAppContext;
    use http_client::FakeHttpClient;
    use indoc::indoc;
    use language_models::RefreshLlmTokenListener;
    use rpc::proto;
    use settings::SettingsStore;

    use super::*;

    #[gpui::test]
    fn test_inline_completion_basic_interpolation(cx: &mut AppContext) {
        let buffer = cx.new_model(|cx| Buffer::local("Lorem ipsum dolor", cx));
        let completion = InlineCompletion {
            edits: to_completion_edits(
                [(2..5, "REM".to_string()), (9..11, "".to_string())],
                &buffer,
                cx,
            )
            .into(),
            path: Path::new("").into(),
            snapshot: buffer.read(cx).snapshot(),
            id: InlineCompletionId::new(),
            excerpt_range: 0..0,
            input_events: "".into(),
            input_excerpt: "".into(),
            output_excerpt: "".into(),
        };

        assert_eq!(
            from_completion_edits(
                &completion.interpolate(buffer.read(cx).snapshot()).unwrap(),
                &buffer,
                cx
            ),
            vec![(2..5, "REM".to_string()), (9..11, "".to_string())]
        );

        buffer.update(cx, |buffer, cx| buffer.edit([(2..5, "")], None, cx));
        assert_eq!(
            from_completion_edits(
                &completion.interpolate(buffer.read(cx).snapshot()).unwrap(),
                &buffer,
                cx
            ),
            vec![(2..2, "REM".to_string()), (6..8, "".to_string())]
        );

        buffer.update(cx, |buffer, cx| buffer.undo(cx));
        assert_eq!(
            from_completion_edits(
                &completion.interpolate(buffer.read(cx).snapshot()).unwrap(),
                &buffer,
                cx
            ),
            vec![(2..5, "REM".to_string()), (9..11, "".to_string())]
        );

        buffer.update(cx, |buffer, cx| buffer.edit([(2..5, "R")], None, cx));
        assert_eq!(
            from_completion_edits(
                &completion.interpolate(buffer.read(cx).snapshot()).unwrap(),
                &buffer,
                cx
            ),
            vec![(3..3, "EM".to_string()), (7..9, "".to_string())]
        );

        buffer.update(cx, |buffer, cx| buffer.edit([(3..3, "E")], None, cx));
        assert_eq!(
            from_completion_edits(
                &completion.interpolate(buffer.read(cx).snapshot()).unwrap(),
                &buffer,
                cx
            ),
            vec![(4..4, "M".to_string()), (8..10, "".to_string())]
        );

        buffer.update(cx, |buffer, cx| buffer.edit([(4..4, "M")], None, cx));
        assert_eq!(
            from_completion_edits(
                &completion.interpolate(buffer.read(cx).snapshot()).unwrap(),
                &buffer,
                cx
            ),
            vec![(9..11, "".to_string())]
        );

        buffer.update(cx, |buffer, cx| buffer.edit([(4..5, "")], None, cx));
        assert_eq!(
            from_completion_edits(
                &completion.interpolate(buffer.read(cx).snapshot()).unwrap(),
                &buffer,
                cx
            ),
            vec![(4..4, "M".to_string()), (8..10, "".to_string())]
        );

        buffer.update(cx, |buffer, cx| buffer.edit([(8..10, "")], None, cx));
        assert_eq!(
            from_completion_edits(
                &completion.interpolate(buffer.read(cx).snapshot()).unwrap(),
                &buffer,
                cx
            ),
            vec![(4..4, "M".to_string())]
        );

        buffer.update(cx, |buffer, cx| buffer.edit([(4..6, "")], None, cx));
        assert_eq!(completion.interpolate(buffer.read(cx).snapshot()), None);
    }

    #[gpui::test]
    async fn test_inline_completion_end_of_buffer(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            client::init_settings(cx);
        });

        let buffer_content = "lorem\n";
        let completion_response = indoc! {"
            ```animals.js
            <|start_of_file|>
            <|editable_region_start|>
            lorem
            ipsum
            <|editable_region_end|>
            ```"};

        let http_client = FakeHttpClient::create(move |_| async move {
            Ok(http_client::Response::builder()
                .status(200)
                .body(
                    serde_json::to_string(&PredictEditsResponse {
                        output_excerpt: completion_response.to_string(),
                    })
                    .unwrap()
                    .into(),
                )
                .unwrap())
        });

        let client = cx.update(|cx| Client::new(Arc::new(FakeSystemClock::new()), http_client, cx));
        cx.update(|cx| {
            RefreshLlmTokenListener::register(client.clone(), cx);
        });
        let server = FakeServer::for_client(42, &client, cx).await;

        let zeta = cx.new_model(|cx| Zeta::new(client, cx));
        let buffer = cx.new_model(|cx| Buffer::local(buffer_content, cx));
        let cursor = buffer.read_with(cx, |buffer, _| buffer.anchor_before(Point::new(1, 0)));
        let completion_task =
            zeta.update(cx, |zeta, cx| zeta.request_completion(&buffer, cursor, cx));

        let token_request = server.receive::<proto::GetLlmToken>().await.unwrap();
        server.respond(
            token_request.receipt(),
            proto::GetLlmTokenResponse { token: "".into() },
        );

        let completion = completion_task.await.unwrap();
        buffer.update(cx, |buffer, cx| {
            buffer.edit(completion.edits.iter().cloned(), None, cx)
        });
        assert_eq!(
            buffer.read_with(cx, |buffer, _| buffer.text()),
            "lorem\nipsum"
        );
    }

    fn to_completion_edits(
        iterator: impl IntoIterator<Item = (Range<usize>, String)>,
        buffer: &Model<Buffer>,
        cx: &AppContext,
    ) -> Vec<(Range<Anchor>, String)> {
        let buffer = buffer.read(cx);
        iterator
            .into_iter()
            .map(|(range, text)| {
                (
                    buffer.anchor_after(range.start)..buffer.anchor_before(range.end),
                    text,
                )
            })
            .collect()
    }

    fn from_completion_edits(
        editor_edits: &[(Range<Anchor>, String)],
        buffer: &Model<Buffer>,
        cx: &AppContext,
    ) -> Vec<(Range<usize>, String)> {
        let buffer = buffer.read(cx);
        editor_edits
            .iter()
            .map(|(range, text)| {
                (
                    range.start.to_offset(buffer)..range.end.to_offset(buffer),
                    text.clone(),
                )
            })
            .collect()
    }

    #[ctor::ctor]
    fn init_logger() {
        if std::env::var("RUST_LOG").is_ok() {
            env_logger::init();
        }
    }
}
