use anyhow::{anyhow, Context as _, Result};
use client::Client;
use collections::{HashMap, VecDeque};
use futures::AsyncReadExt;
use gpui::{AppContext, Context, Global, Model, ModelContext, Task};
use http_client::{HttpClient, Method};
use language::{
    language_settings::all_language_settings, Anchor, Buffer, BufferSnapshot, OffsetRangeExt,
    Point, ToOffset, ToPoint,
};
use rpc::{proto, PredictEditsParams, PredictEditsResponse};
use std::{
    borrow::Cow,
    cmp,
    fmt::Write,
    mem,
    ops::Range,
    path::Path,
    sync::Arc,
    time::{Duration, Instant},
};
use util::ResultExt;
use uuid::Uuid;

const CURSOR_MARKER: &'static str = "<|user_cursor_is_here|>";
const START_OF_FILE_MARKER: &'static str = "<|start_of_file|>";
const EDITABLE_REGION_START_MARKER: &'static str = "<|editable_region_start|>";
const EDITABLE_REGION_END_MARKER: &'static str = "<|editable_region_end|>";
const BUFFER_CHANGE_GROUPING_INTERVAL: Duration = Duration::from_secs(1);

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct InlineCompletionId(Uuid);

impl InlineCompletionId {
    fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

#[derive(Clone)]
struct ZetaGlobal(Model<Zeta>);

impl Global for ZetaGlobal {}

pub struct Zeta {
    client: Arc<Client>,
    events: VecDeque<Event>,
    registered_buffers: HashMap<gpui::EntityId, RegisteredBuffer>,
}

pub struct InlineCompletion {
    id: InlineCompletionId,
    path: Arc<Path>,
    edits: Vec<(Range<Anchor>, String)>,
    _snapshot: BufferSnapshot,
    _raw_response: String,
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

impl Zeta {
    pub fn register(client: Arc<Client>, cx: &mut AppContext) -> Model<Self> {
        cx.try_global::<ZetaGlobal>()
            .map(|global| global.0.clone())
            .unwrap_or_else(|| {
                let model = cx.new_model(|_cx| Self::new(client));
                cx.set_global(ZetaGlobal(model.clone()));
                model
            })
    }

    fn new(client: Arc<Client>) -> Self {
        Self {
            client,
            events: VecDeque::new(),
            registered_buffers: HashMap::default(),
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

    pub fn request_inline_completion(
        &mut self,
        buffer: &Model<Buffer>,
        position: language::Anchor,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<Option<InlineCompletion>>> {
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

        cx.spawn(|_this, _cx| async move {
            let start = std::time::Instant::now();

            // todo!("cache this token")
            let response = client.request(proto::GetLlmToken {}).await?;

            let mut event_prompts = String::new();
            for event in events {
                if !event_prompts.is_empty() {
                    event_prompts.push('\n');
                    event_prompts.push('\n');
                }
                event_prompts.push_str(&event.to_prompt());
            }

            let prompt = include_str!("./complete_prompt.md")
                .replace("<events>", &event_prompts)
                .replace(
                    "<excerpt>",
                    &prompt_for_excerpt(&snapshot, &excerpt_range, offset),
                );

            log::debug!("predicting edit:\n{}", prompt);

            let http_client = client.http_client();
            let body = PredictEditsParams { prompt };
            let request_builder = http_client::Request::builder();
            let request = request_builder
                .method(Method::POST)
                .uri(
                    client
                        .http_client()
                        .build_zed_llm_url("/predict_edits", &[])?
                        .as_ref(),
                )
                .header("Content-Type", "application/json")
                .header("Authorization", format!("Bearer {}", response.token))
                .body(serde_json::to_string(&body)?.into())?;
            let mut response = http_client.send(request).await?;
            let mut body = String::new();
            response.body_mut().read_to_string(&mut body).await?;
            if !response.status().is_success() {
                return Err(anyhow!(
                    "error predicting edits.\nStatus: {:?}\nBody: {}",
                    response.status(),
                    body
                ));
            }

            let response = serde_json::from_str::<PredictEditsResponse>(&body)?;
            log::debug!("prediction took: {:?}", start.elapsed());
            log::debug!("completion response: {}", response.text);

            let content = response.text.replace(CURSOR_MARKER, "");
            let mut new_text = content.as_str();

            let codefence_start = new_text
                .find(EDITABLE_REGION_START_MARKER)
                .context("could not find start marker")?;
            new_text = &new_text[codefence_start..];

            let newline_ix = new_text.find('\n').context("could not find newline")?;
            new_text = &new_text[newline_ix + 1..];

            let codefence_end = new_text
                .rfind(&format!("\n{EDITABLE_REGION_END_MARKER}"))
                .context("could not find end marker")?;
            new_text = &new_text[..codefence_end];
            log::debug!("sanitized completion response: {}", new_text);

            let old_text = snapshot
                .text_for_range(excerpt_range.clone())
                .collect::<String>();
            let diff = similar::TextDiff::from_chars(old_text.as_str(), new_text);

            let mut edits: Vec<(Range<usize>, String)> = Vec::new();
            let mut old_start = excerpt_range.start;
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

            if edits.is_empty() {
                Ok(None)
            } else {
                let edits = edits
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

                        let new_text =
                            new_text[prefix_len..new_text.len() - suffix_len].to_string();
                        (
                            snapshot.anchor_after(old_range.start)
                                ..snapshot.anchor_before(old_range.end),
                            new_text,
                        )
                    })
                    .collect();
                Ok(Some(InlineCompletion {
                    id: InlineCompletionId::new(),
                    path,
                    edits,
                    _snapshot: snapshot,
                    _raw_response: response.text,
                }))
            }
        })
    }

    pub fn accept_inline_completion(
        &mut self,
        _completion: &InlineCompletion,
        cx: &mut ModelContext<Self>,
    ) {
        cx.notify();
    }

    pub fn reject_inline_completion(
        &mut self,
        _completion: &InlineCompletion,
        cx: &mut ModelContext<Self>,
    ) {
        cx.notify();
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

    let mut excerpt_start_row = point.row.saturating_sub(context_lines_before);
    let mut excerpt_end_row = cmp::min(point.row + context_lines_after, snapshot.max_point().row);

    // Skip blank lines at the start.
    while excerpt_start_row < excerpt_end_row && snapshot.is_line_blank(excerpt_start_row) {
        excerpt_start_row += 1;
    }

    // Skip blank lines at the end.
    while excerpt_end_row > excerpt_start_row && snapshot.is_line_blank(excerpt_end_row) {
        excerpt_end_row -= 1;
    }

    let excerpt_start = Point::new(excerpt_start_row, 0);
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

pub struct ZetaInlineCompletionProvider {
    zeta: Model<Zeta>,
    current_completion: Option<InlineCompletion>,
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
        self.pending_refresh = cx.spawn(|this, mut cx| async move {
            if debounce {
                cx.background_executor().timer(Self::DEBOUNCE_TIMEOUT).await;
            }

            let completion_request = this.update(&mut cx, |this, cx| {
                this.zeta.update(cx, |zeta, cx| {
                    zeta.request_inline_completion(&buffer, position, cx)
                })
            });

            let mut completion = None;
            if let Ok(completion_request) = completion_request {
                completion = completion_request.await.log_err().flatten();
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

    fn accept(&mut self, cx: &mut ModelContext<Self>) {
        if let Some(completion) = self.current_completion.as_ref() {
            self.zeta
                .update(cx, |zeta, cx| zeta.accept_inline_completion(completion, cx));
        }
    }

    fn discard(
        &mut self,
        _should_report_inline_completion_event: bool,
        cx: &mut ModelContext<Self>,
    ) {
        if let Some(completion) = self.current_completion.take() {
            self.zeta.update(cx, |zeta, cx| {
                zeta.reject_inline_completion(&completion, cx)
            });
        }
    }

    fn predict<'a>(
        &'a self,
        buffer: &Model<Buffer>,
        cursor_position: language::Anchor,
        cx: &'a AppContext,
    ) -> Option<inline_completion::Prediction> {
        let completion = self.current_completion.as_ref()?;

        // todo!(update edits)

        let buffer = buffer.read(cx);

        let cursor_row = cursor_position.to_point(buffer).row;
        let (closest_edit_ix, (closest_edit_range, _)) = completion
            .edits
            .iter()
            .enumerate()
            .min_by_key(|(_, (range, _))| {
                let distance_from_start = cursor_row.abs_diff(range.start.to_point(buffer).row);
                let distance_from_end = cursor_row.abs_diff(range.end.to_point(buffer).row);
                cmp::min(distance_from_start, distance_from_end)
            })?;

        let mut edit_start_ix = closest_edit_ix;
        for (range, _) in completion.edits[..edit_start_ix].iter().rev() {
            let distance_from_closest_edit =
                closest_edit_range.start.to_point(buffer).row - range.end.to_point(buffer).row;
            if distance_from_closest_edit <= 1 {
                edit_start_ix -= 1;
            } else {
                break;
            }
        }

        let mut edit_end_ix = closest_edit_ix + 1;
        for (range, _) in &completion.edits[edit_end_ix..] {
            let distance_from_closest_edit =
                range.start.to_point(buffer).row - closest_edit_range.end.to_point(buffer).row;
            if distance_from_closest_edit <= 1 {
                edit_end_ix += 1;
            } else {
                break;
            }
        }

        Some(inline_completion::Prediction {
            edits: completion.edits[edit_start_ix..edit_end_ix].to_vec(),
        })
    }
}
