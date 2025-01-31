mod completion_diff_element;
mod persistence;
mod rate_completion_modal;

pub(crate) use completion_diff_element::*;
use db::kvp::KEY_VALUE_STORE;
use inline_completion::DataCollectionState;
pub use rate_completion_modal::*;

use anyhow::{anyhow, Context as _, Result};
use arrayvec::ArrayVec;
use client::{Client, UserStore};
use collections::hash_map::Entry;
use collections::{HashMap, HashSet, VecDeque};
use feature_flags::FeatureFlagAppExt as _;
use futures::AsyncReadExt;
use gpui::{
    actions, App, AppContext as _, AsyncApp, Context, Entity, EntityId, Global, Subscription, Task,
    WeakEntity,
};
use http_client::{HttpClient, Method};
use language::{
    language_settings::all_language_settings, Anchor, Buffer, BufferSnapshot, EditPreview,
    OffsetRangeExt, Point, ToOffset, ToPoint,
};
use language_models::LlmApiToken;
use rpc::{PredictEditsParams, PredictEditsResponse, EXPIRED_LLM_TOKEN_HEADER_NAME};
use std::{
    borrow::Cow,
    cmp, env,
    fmt::Write,
    future::Future,
    mem,
    ops::Range,
    path::{Path, PathBuf},
    sync::Arc,
    time::{Duration, Instant},
};
use telemetry_events::InlineCompletionRating;
use util::ResultExt;
use uuid::Uuid;
use workspace::{
    notifications::{simple_message_notification::MessageNotification, NotificationId},
    Workspace,
};

const CURSOR_MARKER: &'static str = "<|user_cursor_is_here|>";
const START_OF_FILE_MARKER: &'static str = "<|start_of_file|>";
const EDITABLE_REGION_START_MARKER: &'static str = "<|editable_region_start|>";
const EDITABLE_REGION_END_MARKER: &'static str = "<|editable_region_end|>";
const BUFFER_CHANGE_GROUPING_INTERVAL: Duration = Duration::from_secs(1);
const ZED_PREDICT_DATA_COLLECTION_NEVER_ASK_AGAIN_KEY: &'static str =
    "zed_predict_data_collection_never_ask_again";

// TODO(mgsloan): more systematic way to choose or tune these fairly arbitrary constants?

/// Typical number of string bytes per token for the purposes of limiting model input. This is
/// intentionally low to err on the side of underestimating limits.
const BYTES_PER_TOKEN_GUESS: usize = 3;

/// Output token limit, used to inform the size of the input. A copy of this constant is also in
/// `crates/collab/src/llm.rs`.
const MAX_OUTPUT_TOKENS: usize = 2048;

/// Total bytes limit for editable region of buffer excerpt.
///
/// The number of output tokens is relevant to the size of the input excerpt because the model is
/// tasked with outputting a modified excerpt. `2/3` is chosen so that there are some output tokens
/// remaining for the model to specify insertions.
const BUFFER_EXCERPT_BYTE_LIMIT: usize = (MAX_OUTPUT_TOKENS * 2 / 3) * BYTES_PER_TOKEN_GUESS;

/// Total line limit for editable region of buffer excerpt.
const BUFFER_EXCERPT_LINE_LIMIT: u32 = 64;

/// Note that this is not the limit for the overall prompt, just for the inputs to the template
/// instantiated in `crates/collab/src/llm.rs`.
const TOTAL_BYTE_LIMIT: usize = BUFFER_EXCERPT_BYTE_LIMIT * 2;

/// Maximum number of events to include in the prompt.
const MAX_EVENT_COUNT: usize = 16;

/// Maximum number of string bytes in a single event. Arbitrarily choosing this to be 4x the size of
/// equally splitting up the the remaining bytes after the largest possible buffer excerpt.
const PER_EVENT_BYTE_LIMIT: usize =
    (TOTAL_BYTE_LIMIT - BUFFER_EXCERPT_BYTE_LIMIT) / MAX_EVENT_COUNT * 4;

actions!(edit_prediction, [ClearHistory]);

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
struct ZetaGlobal(Entity<Zeta>);

impl Global for ZetaGlobal {}

#[derive(Clone)]
pub struct InlineCompletion {
    id: InlineCompletionId,
    path: Arc<Path>,
    excerpt_range: Range<usize>,
    cursor_offset: usize,
    edits: Arc<[(Range<Anchor>, String)]>,
    snapshot: BufferSnapshot,
    edit_preview: EditPreview,
    input_outline: Arc<str>,
    input_events: Arc<str>,
    input_excerpt: Arc<str>,
    output_excerpt: Arc<str>,
    request_sent_at: Instant,
    response_received_at: Instant,
}

impl InlineCompletion {
    fn latency(&self) -> Duration {
        self.response_received_at
            .duration_since(self.request_sent_at)
    }

    fn interpolate(&self, new_snapshot: &BufferSnapshot) -> Option<Vec<(Range<Anchor>, String)>> {
        interpolate(&self.snapshot, new_snapshot, self.edits.clone())
    }
}

fn interpolate(
    old_snapshot: &BufferSnapshot,
    new_snapshot: &BufferSnapshot,
    current_edits: Arc<[(Range<Anchor>, String)]>,
) -> Option<Vec<(Range<Anchor>, String)>> {
    let mut edits = Vec::new();

    let mut model_edits = current_edits.into_iter().peekable();
    for user_edit in new_snapshot.edits_since::<usize>(&old_snapshot.version) {
        while let Some((model_old_range, _)) = model_edits.peek() {
            let model_old_range = model_old_range.to_offset(old_snapshot);
            if model_old_range.end < user_edit.old.start {
                let (model_old_range, model_new_text) = model_edits.next().unwrap();
                edits.push((model_old_range.clone(), model_new_text.clone()));
            } else {
                break;
            }
        }

        if let Some((model_old_range, model_new_text)) = model_edits.peek() {
            let model_old_offset_range = model_old_range.to_offset(old_snapshot);
            if user_edit.old == model_old_offset_range {
                let user_new_text = new_snapshot
                    .text_for_range(user_edit.new.clone())
                    .collect::<String>();

                if let Some(model_suffix) = model_new_text.strip_prefix(&user_new_text) {
                    if !model_suffix.is_empty() {
                        let anchor = old_snapshot.anchor_after(user_edit.old.end);
                        edits.push((anchor..anchor, model_suffix.to_string()));
                    }

                    model_edits.next();
                    continue;
                }
            }
        }

        return None;
    }

    edits.extend(model_edits.cloned());

    if edits.is_empty() {
        None
    } else {
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
    shown_completions: VecDeque<InlineCompletion>,
    rated_completions: HashSet<InlineCompletionId>,
    data_collection_preferences: DataCollectionPreferences,
    llm_token: LlmApiToken,
    _llm_token_subscription: Subscription,
    tos_accepted: bool, // Terms of service accepted
    _user_store_subscription: Subscription,
}

impl Zeta {
    pub fn global(cx: &mut App) -> Option<Entity<Self>> {
        cx.try_global::<ZetaGlobal>().map(|global| global.0.clone())
    }

    pub fn register(
        client: Arc<Client>,
        user_store: Entity<UserStore>,
        cx: &mut App,
    ) -> Entity<Self> {
        Self::global(cx).unwrap_or_else(|| {
            let model = cx.new(|cx| Self::new(client, user_store, cx));
            cx.set_global(ZetaGlobal(model.clone()));
            model
        })
    }

    pub fn clear_history(&mut self) {
        self.events.clear();
    }

    fn new(client: Arc<Client>, user_store: Entity<UserStore>, cx: &mut Context<Self>) -> Self {
        let refresh_llm_token_listener = language_models::RefreshLlmTokenListener::global(cx);
        Self {
            client,
            events: VecDeque::new(),
            shown_completions: VecDeque::new(),
            rated_completions: HashSet::default(),
            registered_buffers: HashMap::default(),
            data_collection_preferences: Self::load_data_collection_preferences(cx),
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
            tos_accepted: user_store
                .read(cx)
                .current_user_has_accepted_terms()
                .unwrap_or(false),
            _user_store_subscription: cx.subscribe(&user_store, |this, user_store, event, cx| {
                match event {
                    client::user::Event::PrivateUserInfoUpdated => {
                        this.tos_accepted = user_store
                            .read(cx)
                            .current_user_has_accepted_terms()
                            .unwrap_or(false);
                    }
                    _ => {}
                }
            }),
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
        if self.events.len() >= MAX_EVENT_COUNT {
            self.events.drain(..MAX_EVENT_COUNT / 2);
        }
    }

    pub fn register_buffer(&mut self, buffer: &Entity<Buffer>, cx: &mut Context<Self>) {
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
        buffer: Entity<Buffer>,
        event: &language::BufferEvent,
        cx: &mut Context<Self>,
    ) {
        if let language::BufferEvent::Edited = event {
            self.report_changes_for_buffer(&buffer, cx);
        }
    }

    pub fn request_completion_impl<F, R>(
        &mut self,
        buffer: &Entity<Buffer>,
        cursor: language::Anchor,
        can_collect_data: bool,
        cx: &mut Context<Self>,
        perform_predict_edits: F,
    ) -> Task<Result<Option<InlineCompletion>>>
    where
        F: FnOnce(Arc<Client>, LlmApiToken, bool, PredictEditsParams) -> R + 'static,
        R: Future<Output = Result<PredictEditsResponse>> + Send + 'static,
    {
        let snapshot = self.report_changes_for_buffer(&buffer, cx);
        let cursor_point = cursor.to_point(&snapshot);
        let cursor_offset = cursor_point.to_offset(&snapshot);
        let events = self.events.clone();
        let path: Arc<Path> = snapshot
            .file()
            .map(|f| Arc::from(f.full_path(cx).as_path()))
            .unwrap_or_else(|| Arc::from(Path::new("untitled")));

        let client = self.client.clone();
        let llm_token = self.llm_token.clone();
        let is_staff = cx.is_staff();

        let buffer = buffer.clone();
        cx.spawn(|_, cx| async move {
            let request_sent_at = Instant::now();

            let (input_events, input_excerpt, excerpt_range, input_outline) = cx
                .background_executor()
                .spawn({
                    let snapshot = snapshot.clone();
                    let path = path.clone();
                    async move {
                        let path = path.to_string_lossy();
                        let (excerpt_range, excerpt_len_guess) = excerpt_range_for_position(
                            cursor_point,
                            BUFFER_EXCERPT_BYTE_LIMIT,
                            BUFFER_EXCERPT_LINE_LIMIT,
                            &path,
                            &snapshot,
                        )?;
                        let input_excerpt = prompt_for_excerpt(
                            cursor_offset,
                            &excerpt_range,
                            excerpt_len_guess,
                            &path,
                            &snapshot,
                        );

                        let bytes_remaining = TOTAL_BYTE_LIMIT.saturating_sub(input_excerpt.len());
                        let input_events = prompt_for_events(events.iter(), bytes_remaining);

                        // Note that input_outline is not currently used in prompt generation and so
                        // is not counted towards TOTAL_BYTE_LIMIT.
                        let input_outline = prompt_for_outline(&snapshot);

                        anyhow::Ok((input_events, input_excerpt, excerpt_range, input_outline))
                    }
                })
                .await?;

            log::debug!("Events:\n{}\nExcerpt:\n{}", input_events, input_excerpt);

            let body = PredictEditsParams {
                input_events: input_events.clone(),
                input_excerpt: input_excerpt.clone(),
                outline: Some(input_outline.clone()),
                can_collect_data,
            };

            let response = perform_predict_edits(client, llm_token, is_staff, body).await?;

            let output_excerpt = response.output_excerpt;
            log::debug!("completion response: {}", output_excerpt);

            Self::process_completion_response(
                output_excerpt,
                buffer,
                &snapshot,
                excerpt_range,
                cursor_offset,
                path,
                input_outline,
                input_events,
                input_excerpt,
                request_sent_at,
                &cx,
            )
            .await
        })
    }

    // Generates several example completions of various states to fill the Zeta completion modal
    #[cfg(any(test, feature = "test-support"))]
    pub fn fill_with_fake_completions(&mut self, cx: &mut Context<Self>) -> Task<()> {
        let test_buffer_text = indoc::indoc! {r#"a longggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggg line
            And maybe a short line

            Then a few lines

            and then another
            "#};

        let buffer = cx.new(|cx| Buffer::local(test_buffer_text, cx));
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
                zeta.shown_completions.get_mut(2).unwrap().edits = Arc::new([]);
                zeta.shown_completions.get_mut(3).unwrap().edits = Arc::new([]);
            })
            .ok();
        })
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn fake_completion(
        &mut self,
        buffer: &Entity<Buffer>,
        position: language::Anchor,
        response: PredictEditsResponse,
        cx: &mut Context<Self>,
    ) -> Task<Result<Option<InlineCompletion>>> {
        use std::future::ready;

        self.request_completion_impl(buffer, position, false, cx, |_, _, _, _| {
            ready(Ok(response))
        })
    }

    pub fn request_completion(
        &mut self,
        buffer: &Entity<Buffer>,
        position: language::Anchor,
        can_collect_data: bool,
        cx: &mut Context<Self>,
    ) -> Task<Result<Option<InlineCompletion>>> {
        self.request_completion_impl(
            buffer,
            position,
            can_collect_data,
            cx,
            Self::perform_predict_edits,
        )
    }

    fn perform_predict_edits(
        client: Arc<Client>,
        llm_token: LlmApiToken,
        _is_staff: bool,
        body: PredictEditsParams,
    ) -> impl Future<Output = Result<PredictEditsResponse>> {
        async move {
            let http_client = client.http_client();
            let mut token = llm_token.acquire(&client).await?;
            let mut did_retry = false;

            loop {
                let request_builder = http_client::Request::builder().method(Method::POST).uri(
                    http_client
                        .build_zed_llm_url("/predict_edits", &[])?
                        .as_ref(),
                );
                let request = request_builder
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

    #[allow(clippy::too_many_arguments)]
    fn process_completion_response(
        output_excerpt: String,
        buffer: Entity<Buffer>,
        snapshot: &BufferSnapshot,
        excerpt_range: Range<usize>,
        cursor_offset: usize,
        path: Arc<Path>,
        input_outline: String,
        input_events: String,
        input_excerpt: String,
        request_sent_at: Instant,
        cx: &AsyncApp,
    ) -> Task<Result<Option<InlineCompletion>>> {
        let snapshot = snapshot.clone();
        cx.spawn(|cx| async move {
            let output_excerpt: Arc<str> = output_excerpt.into();

            let edits: Arc<[(Range<Anchor>, String)]> = cx
                .background_executor()
                .spawn({
                    let output_excerpt = output_excerpt.clone();
                    let excerpt_range = excerpt_range.clone();
                    let snapshot = snapshot.clone();
                    async move { Self::parse_edits(output_excerpt, excerpt_range, &snapshot) }
                })
                .await?
                .into();

            let Some((edits, snapshot, edit_preview)) = buffer.read_with(&cx, {
                let edits = edits.clone();
                |buffer, cx| {
                    let new_snapshot = buffer.snapshot();
                    let edits: Arc<[(Range<Anchor>, String)]> =
                        interpolate(&snapshot, &new_snapshot, edits)?.into();
                    Some((edits.clone(), new_snapshot, buffer.preview_edits(edits, cx)))
                }
            })?
            else {
                return anyhow::Ok(None);
            };

            let edit_preview = edit_preview.await;

            Ok(Some(InlineCompletion {
                id: InlineCompletionId::new(),
                path,
                excerpt_range,
                cursor_offset,
                edits,
                edit_preview,
                snapshot,
                input_outline: input_outline.into(),
                input_events: input_events.into(),
                input_excerpt: input_excerpt.into(),
                output_excerpt,
                request_sent_at,
                response_received_at: Instant::now(),
            }))
        })
    }

    fn parse_edits(
        output_excerpt: Arc<str>,
        excerpt_range: Range<usize>,
        snapshot: &BufferSnapshot,
    ) -> Result<Vec<(Range<Anchor>, String)>> {
        let content = output_excerpt.replace(CURSOR_MARKER, "");

        let start_markers = content
            .match_indices(EDITABLE_REGION_START_MARKER)
            .collect::<Vec<_>>();
        anyhow::ensure!(
            start_markers.len() == 1,
            "expected exactly one start marker, found {}",
            start_markers.len()
        );

        let end_markers = content
            .match_indices(EDITABLE_REGION_END_MARKER)
            .collect::<Vec<_>>();
        anyhow::ensure!(
            end_markers.len() == 1,
            "expected exactly one end marker, found {}",
            end_markers.len()
        );

        let sof_markers = content
            .match_indices(START_OF_FILE_MARKER)
            .collect::<Vec<_>>();
        anyhow::ensure!(
            sof_markers.len() <= 1,
            "expected at most one start-of-file marker, found {}",
            sof_markers.len()
        );

        let codefence_start = start_markers[0].0;
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

        Ok(Self::compute_edits(
            old_text,
            new_text,
            excerpt_range.start,
            &snapshot,
        ))
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
                let range = if old_range.is_empty() {
                    let anchor = snapshot.anchor_after(old_range.start);
                    anchor..anchor
                } else {
                    snapshot.anchor_after(old_range.start)..snapshot.anchor_before(old_range.end)
                };
                (range, new_text)
            })
            .collect()
    }

    pub fn is_completion_rated(&self, completion_id: InlineCompletionId) -> bool {
        self.rated_completions.contains(&completion_id)
    }

    pub fn completion_shown(&mut self, completion: &InlineCompletion, cx: &mut Context<Self>) {
        self.shown_completions.push_front(completion.clone());
        if self.shown_completions.len() > 50 {
            let completion = self.shown_completions.pop_back().unwrap();
            self.rated_completions.remove(&completion.id);
        }
        cx.notify();
    }

    pub fn rate_completion(
        &mut self,
        completion: &InlineCompletion,
        rating: InlineCompletionRating,
        feedback: String,
        cx: &mut Context<Self>,
    ) {
        self.rated_completions.insert(completion.id);
        telemetry::event!(
            "Inline Completion Rated",
            rating,
            input_events = completion.input_events,
            input_excerpt = completion.input_excerpt,
            input_outline = completion.input_outline,
            output_excerpt = completion.output_excerpt,
            feedback
        );
        self.client.telemetry().flush_events();
        cx.notify();
    }

    pub fn shown_completions(&self) -> impl DoubleEndedIterator<Item = &InlineCompletion> {
        self.shown_completions.iter()
    }

    pub fn shown_completions_len(&self) -> usize {
        self.shown_completions.len()
    }

    fn report_changes_for_buffer(
        &mut self,
        buffer: &Entity<Buffer>,
        cx: &mut Context<Self>,
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

    /// Creates a `Entity<DataCollectionChoice>` for each unique worktree abs path it sees.
    pub fn data_collection_choice_at(
        &mut self,
        worktree_abs_path: PathBuf,
        cx: &mut Context<Self>,
    ) -> Entity<DataCollectionChoice> {
        match self
            .data_collection_preferences
            .per_worktree
            .entry(worktree_abs_path)
        {
            Entry::Vacant(entry) => {
                let choice = cx.new(|_| DataCollectionChoice::NotAnswered);
                entry.insert(choice.clone());
                choice
            }
            Entry::Occupied(entry) => entry.get().clone(),
        }
    }

    fn set_never_ask_again_for_data_collection(&mut self, cx: &mut Context<Self>) {
        self.data_collection_preferences.never_ask_again = true;

        // persist choice
        db::write_and_log(cx, move || {
            KEY_VALUE_STORE.write_kvp(
                ZED_PREDICT_DATA_COLLECTION_NEVER_ASK_AGAIN_KEY.into(),
                "true".to_string(),
            )
        });
    }

    fn load_data_collection_preferences(cx: &mut Context<Self>) -> DataCollectionPreferences {
        if env::var("ZED_PREDICT_CLEAR_DATA_COLLECTION_PREFERENCES").is_ok() {
            db::write_and_log(cx, move || async move {
                KEY_VALUE_STORE
                    .delete_kvp(ZED_PREDICT_DATA_COLLECTION_NEVER_ASK_AGAIN_KEY.into())
                    .await
                    .log_err();

                persistence::DB.clear_all_zeta_preferences().await
            });
            return DataCollectionPreferences::default();
        }

        let never_ask_again = KEY_VALUE_STORE
            .read_kvp(ZED_PREDICT_DATA_COLLECTION_NEVER_ASK_AGAIN_KEY)
            .log_err()
            .flatten()
            .map(|value| value == "true")
            .unwrap_or(false);

        let preferences_per_worktree = persistence::DB
            .get_all_data_collection_preferences()
            .log_err()
            .into_iter()
            .flatten()
            .map(|(path, choice)| {
                let choice = cx.new(|_| DataCollectionChoice::from(choice));
                (path, choice)
            })
            .collect();

        DataCollectionPreferences {
            never_ask_again,
            per_worktree: preferences_per_worktree,
        }
    }
}

#[derive(Default, Debug)]
struct DataCollectionPreferences {
    /// Set when a user clicks on "Never Ask Again", can never be unset.
    never_ask_again: bool,
    /// The choices for each worktree.
    ///
    /// This is filled when loading from database, or when querying if no matching path is found.
    per_worktree: HashMap<PathBuf, Entity<DataCollectionChoice>>,
}

fn common_prefix<T1: Iterator<Item = char>, T2: Iterator<Item = char>>(a: T1, b: T2) -> usize {
    a.zip(b)
        .take_while(|(a, b)| a == b)
        .map(|(a, _)| a.len_utf8())
        .sum()
}

fn prompt_for_outline(snapshot: &BufferSnapshot) -> String {
    let mut input_outline = String::new();

    writeln!(
        input_outline,
        "```{}",
        snapshot
            .file()
            .map_or(Cow::Borrowed("untitled"), |file| file
                .path()
                .to_string_lossy())
    )
    .unwrap();

    if let Some(outline) = snapshot.outline(None) {
        let guess_size = outline.items.len() * 15;
        input_outline.reserve(guess_size);
        for item in outline.items.iter() {
            let spacing = " ".repeat(item.depth);
            writeln!(input_outline, "{}{}", spacing, item.text).unwrap();
        }
    }

    writeln!(input_outline, "```").unwrap();

    input_outline
}

fn prompt_for_excerpt(
    offset: usize,
    excerpt_range: &Range<usize>,
    mut len_guess: usize,
    path: &str,
    snapshot: &BufferSnapshot,
) -> String {
    let point_range = excerpt_range.to_point(snapshot);

    // Include one line of extra context before and after editable range, if those lines are non-empty.
    let extra_context_before_range =
        if point_range.start.row > 0 && !snapshot.is_line_blank(point_range.start.row - 1) {
            let range =
                (Point::new(point_range.start.row - 1, 0)..point_range.start).to_offset(snapshot);
            len_guess += range.end - range.start;
            Some(range)
        } else {
            None
        };
    let extra_context_after_range = if point_range.end.row < snapshot.max_point().row
        && !snapshot.is_line_blank(point_range.end.row + 1)
    {
        let range = (point_range.end
            ..Point::new(
                point_range.end.row + 1,
                snapshot.line_len(point_range.end.row + 1),
            ))
            .to_offset(snapshot);
        len_guess += range.end - range.start;
        Some(range)
    } else {
        None
    };

    let mut prompt_excerpt = String::with_capacity(len_guess);
    writeln!(prompt_excerpt, "```{}", path).unwrap();

    if excerpt_range.start == 0 {
        writeln!(prompt_excerpt, "{START_OF_FILE_MARKER}").unwrap();
    }

    if let Some(extra_context_before_range) = extra_context_before_range {
        for chunk in snapshot.text_for_range(extra_context_before_range) {
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

    if let Some(extra_context_after_range) = extra_context_after_range {
        for chunk in snapshot.text_for_range(extra_context_after_range) {
            prompt_excerpt.push_str(chunk);
        }
    }

    write!(prompt_excerpt, "\n```").unwrap();
    debug_assert!(
        prompt_excerpt.len() <= len_guess,
        "Excerpt length {} exceeds estimated length {}",
        prompt_excerpt.len(),
        len_guess
    );
    prompt_excerpt
}

fn excerpt_range_for_position(
    cursor_point: Point,
    byte_limit: usize,
    line_limit: u32,
    path: &str,
    snapshot: &BufferSnapshot,
) -> Result<(Range<usize>, usize)> {
    let cursor_row = cursor_point.row;
    let last_buffer_row = snapshot.max_point().row;

    // This is an overestimate because it includes parts of prompt_for_excerpt which are
    // conditionally skipped.
    let mut len_guess = 0;
    len_guess += "```".len() + path.len() + 1;
    len_guess += START_OF_FILE_MARKER.len() + 1;
    len_guess += EDITABLE_REGION_START_MARKER.len() + 1;
    len_guess += CURSOR_MARKER.len();
    len_guess += EDITABLE_REGION_END_MARKER.len() + 1;
    len_guess += "```".len() + 1;

    len_guess += usize::try_from(snapshot.line_len(cursor_row) + 1).unwrap();

    if len_guess > byte_limit {
        return Err(anyhow!("Current line too long to send to model."));
    }

    let mut excerpt_start_row = cursor_row;
    let mut excerpt_end_row = cursor_row;
    let mut no_more_before = cursor_row == 0;
    let mut no_more_after = cursor_row >= last_buffer_row;
    let mut row_delta = 1;
    loop {
        if !no_more_before {
            let row = cursor_point.row - row_delta;
            let line_len: usize = usize::try_from(snapshot.line_len(row) + 1).unwrap();
            let mut new_len_guess = len_guess + line_len;
            if row == 0 {
                new_len_guess += START_OF_FILE_MARKER.len() + 1;
            }
            if new_len_guess <= byte_limit {
                len_guess = new_len_guess;
                excerpt_start_row = row;
                if row == 0 {
                    no_more_before = true;
                }
            } else {
                no_more_before = true;
            }
        }
        if excerpt_end_row - excerpt_start_row >= line_limit {
            break;
        }
        if !no_more_after {
            let row = cursor_point.row + row_delta;
            let line_len: usize = usize::try_from(snapshot.line_len(row) + 1).unwrap();
            let new_len_guess = len_guess + line_len;
            if new_len_guess <= byte_limit {
                len_guess = new_len_guess;
                excerpt_end_row = row;
                if row >= last_buffer_row {
                    no_more_after = true;
                }
            } else {
                no_more_after = true;
            }
        }
        if excerpt_end_row - excerpt_start_row >= line_limit {
            break;
        }
        if no_more_before && no_more_after {
            break;
        }
        row_delta += 1;
    }

    let excerpt_start = Point::new(excerpt_start_row, 0);
    let excerpt_end = Point::new(excerpt_end_row, snapshot.line_len(excerpt_end_row));
    Ok((
        excerpt_start.to_offset(snapshot)..excerpt_end.to_offset(snapshot),
        len_guess,
    ))
}

fn prompt_for_events<'a>(
    events: impl Iterator<Item = &'a Event>,
    mut bytes_remaining: usize,
) -> String {
    let mut result = String::new();
    for event in events {
        if !result.is_empty() {
            result.push('\n');
            result.push('\n');
        }
        let event_string = event.to_prompt();
        let len = event_string.len();
        if len > PER_EVENT_BYTE_LIMIT {
            continue;
        }
        if len > bytes_remaining {
            break;
        }
        bytes_remaining -= len;
        result.push_str(&event_string);
    }
    result
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

#[derive(Debug, Clone)]
struct CurrentInlineCompletion {
    buffer_id: EntityId,
    completion: InlineCompletion,
}

impl CurrentInlineCompletion {
    fn should_replace_completion(&self, old_completion: &Self, snapshot: &BufferSnapshot) -> bool {
        if self.buffer_id != old_completion.buffer_id {
            return true;
        }

        let Some(old_edits) = old_completion.completion.interpolate(&snapshot) else {
            return true;
        };
        let Some(new_edits) = self.completion.interpolate(&snapshot) else {
            return false;
        };

        if old_edits.len() == 1 && new_edits.len() == 1 {
            let (old_range, old_text) = &old_edits[0];
            let (new_range, new_text) = &new_edits[0];
            new_range == old_range && new_text.starts_with(old_text)
        } else {
            true
        }
    }
}

struct PendingCompletion {
    id: usize,
    _task: Task<()>,
}

#[derive(Debug, Clone, Copy)]
pub enum DataCollectionChoice {
    NotAnswered,
    Enabled,
    Disabled,
}

impl DataCollectionChoice {
    pub fn is_enabled(self) -> bool {
        match self {
            Self::Enabled => true,
            Self::NotAnswered | Self::Disabled => false,
        }
    }

    pub fn is_answered(self) -> bool {
        match self {
            Self::Enabled | Self::Disabled => true,
            Self::NotAnswered => false,
        }
    }

    pub fn toggle(self) -> DataCollectionChoice {
        match self {
            Self::Enabled => Self::Disabled,
            Self::Disabled => Self::Enabled,
            Self::NotAnswered => Self::Enabled,
        }
    }
}

impl From<bool> for DataCollectionChoice {
    fn from(value: bool) -> Self {
        match value {
            true => DataCollectionChoice::Enabled,
            false => DataCollectionChoice::Disabled,
        }
    }
}

pub struct ZetaInlineCompletionProvider {
    zeta: Entity<Zeta>,
    pending_completions: ArrayVec<PendingCompletion, 2>,
    next_pending_completion_id: usize,
    current_completion: Option<CurrentInlineCompletion>,
    data_collection: Option<ProviderDataCollection>,
}

pub struct ProviderDataCollection {
    workspace: WeakEntity<Workspace>,
    worktree_root_path: PathBuf,
    choice: Entity<DataCollectionChoice>,
}

impl ProviderDataCollection {
    pub fn new(
        zeta: Entity<Zeta>,
        workspace: Option<Entity<Workspace>>,
        buffer: Option<Entity<Buffer>>,
        cx: &mut App,
    ) -> Option<ProviderDataCollection> {
        let workspace = workspace?;

        let worktree_root_path = buffer?.update(cx, |buffer, cx| {
            let file = buffer.file()?;

            if !file.is_local() || file.is_private() {
                return None;
            }

            workspace.update(cx, |workspace, cx| {
                Some(
                    workspace
                        .absolute_path_of_worktree(file.worktree_id(cx), cx)?
                        .to_path_buf(),
                )
            })
        })?;

        let choice = zeta.update(cx, |zeta, cx| {
            zeta.data_collection_choice_at(worktree_root_path.clone(), cx)
        });

        Some(ProviderDataCollection {
            workspace: workspace.downgrade(),
            worktree_root_path,
            choice,
        })
    }

    fn set_choice(&mut self, choice: DataCollectionChoice, cx: &mut App) {
        self.choice.update(cx, |this, _| *this = choice);

        let worktree_root_path = self.worktree_root_path.clone();

        db::write_and_log(cx, move || {
            persistence::DB.save_data_collection_choice(worktree_root_path, choice.is_enabled())
        });
    }

    fn toggle_choice(&mut self, cx: &mut App) {
        self.set_choice(self.choice.read(cx).toggle(), cx);
    }
}

impl ZetaInlineCompletionProvider {
    pub const DEBOUNCE_TIMEOUT: Duration = Duration::from_millis(8);

    pub fn new(zeta: Entity<Zeta>, data_collection: Option<ProviderDataCollection>) -> Self {
        Self {
            zeta,
            pending_completions: ArrayVec::new(),
            next_pending_completion_id: 0,
            current_completion: None,
            data_collection,
        }
    }

    fn set_data_collection_choice(&mut self, choice: DataCollectionChoice, cx: &mut App) {
        if let Some(data_collection) = self.data_collection.as_mut() {
            data_collection.set_choice(choice, cx);
        }
    }
}

impl inline_completion::InlineCompletionProvider for ZetaInlineCompletionProvider {
    fn name() -> &'static str {
        "zed-predict"
    }

    fn display_name() -> &'static str {
        "Zed's Edit Predictions"
    }

    fn show_completions_in_menu() -> bool {
        true
    }

    fn show_completions_in_normal_mode() -> bool {
        true
    }

    fn show_tab_accept_marker() -> bool {
        true
    }

    fn data_collection_state(&self, cx: &App) -> DataCollectionState {
        let Some(data_collection) = self.data_collection.as_ref() else {
            return DataCollectionState::Unknown;
        };

        if data_collection.choice.read(cx).is_enabled() {
            DataCollectionState::Enabled
        } else {
            DataCollectionState::Disabled
        }
    }

    fn toggle_data_collection(&mut self, cx: &mut App) {
        if let Some(data_collection) = self.data_collection.as_mut() {
            data_collection.toggle_choice(cx);
        }
    }

    fn is_enabled(
        &self,
        buffer: &Entity<Buffer>,
        cursor_position: language::Anchor,
        cx: &App,
    ) -> bool {
        let buffer = buffer.read(cx);
        let file = buffer.file();
        let language = buffer.language_at(cursor_position);
        let settings = all_language_settings(file, cx);
        settings.inline_completions_enabled(language.as_ref(), file.map(|f| f.path().as_ref()), cx)
    }

    fn needs_terms_acceptance(&self, cx: &App) -> bool {
        !self.zeta.read(cx).tos_accepted
    }

    fn is_refreshing(&self) -> bool {
        !self.pending_completions.is_empty()
    }

    fn refresh(
        &mut self,
        buffer: Entity<Buffer>,
        position: language::Anchor,
        debounce: bool,
        cx: &mut Context<Self>,
    ) {
        if !self.zeta.read(cx).tos_accepted {
            return;
        }

        if let Some(current_completion) = self.current_completion.as_ref() {
            let snapshot = buffer.read(cx).snapshot();
            if current_completion
                .completion
                .interpolate(&snapshot)
                .is_some()
            {
                return;
            }
        }

        let pending_completion_id = self.next_pending_completion_id;
        self.next_pending_completion_id += 1;
        let can_collect_data = self
            .data_collection
            .as_ref()
            .map_or(false, |data_collection| {
                data_collection.choice.read(cx).is_enabled()
            });

        let task = cx.spawn(|this, mut cx| async move {
            if debounce {
                cx.background_executor().timer(Self::DEBOUNCE_TIMEOUT).await;
            }

            let completion_request = this.update(&mut cx, |this, cx| {
                this.zeta.update(cx, |zeta, cx| {
                    zeta.request_completion(&buffer, position, can_collect_data, cx)
                })
            });

            let completion = match completion_request {
                Ok(completion_request) => {
                    let completion_request = completion_request.await;
                    completion_request.map(|c| {
                        c.map(|completion| CurrentInlineCompletion {
                            buffer_id: buffer.entity_id(),
                            completion,
                        })
                    })
                }
                Err(error) => Err(error),
            };
            let Some(new_completion) = completion
                .context("edit prediction failed")
                .log_err()
                .flatten()
            else {
                return;
            };

            this.update(&mut cx, |this, cx| {
                if this.pending_completions[0].id == pending_completion_id {
                    this.pending_completions.remove(0);
                } else {
                    this.pending_completions.clear();
                }

                if let Some(old_completion) = this.current_completion.as_ref() {
                    let snapshot = buffer.read(cx).snapshot();
                    if new_completion.should_replace_completion(&old_completion, &snapshot) {
                        this.zeta.update(cx, |zeta, cx| {
                            zeta.completion_shown(&new_completion.completion, cx);
                        });
                        this.current_completion = Some(new_completion);
                    }
                } else {
                    this.zeta.update(cx, |zeta, cx| {
                        zeta.completion_shown(&new_completion.completion, cx);
                    });
                    this.current_completion = Some(new_completion);
                }

                cx.notify();
            })
            .ok();
        });

        // We always maintain at most two pending completions. When we already
        // have two, we replace the newest one.
        if self.pending_completions.len() <= 1 {
            self.pending_completions.push(PendingCompletion {
                id: pending_completion_id,
                _task: task,
            });
        } else if self.pending_completions.len() == 2 {
            self.pending_completions.pop();
            self.pending_completions.push(PendingCompletion {
                id: pending_completion_id,
                _task: task,
            });
        }
    }

    fn cycle(
        &mut self,
        _buffer: Entity<Buffer>,
        _cursor_position: language::Anchor,
        _direction: inline_completion::Direction,
        _cx: &mut Context<Self>,
    ) {
        // Right now we don't support cycling.
    }

    fn accept(&mut self, cx: &mut Context<Self>) {
        self.pending_completions.clear();

        let Some(data_collection) = self.data_collection.as_mut() else {
            return;
        };

        if data_collection.choice.read(cx).is_answered()
            || self
                .zeta
                .read(cx)
                .data_collection_preferences
                .never_ask_again
        {
            return;
        }

        struct ZetaDataCollectionNotification;
        let notification_id = NotificationId::unique::<ZetaDataCollectionNotification>();

        const DATA_COLLECTION_INFO_URL: &str = "https://zed.dev/terms-of-service"; // TODO: Replace for a link that's dedicated to Edit Predictions data collection

        let this = cx.entity();
        data_collection
            .workspace
            .update(cx, |workspace, cx| {
                workspace.show_notification(notification_id, cx, |cx| {
                    let zeta = self.zeta.clone();

                    cx.new(move |_cx| {
                        let message =
                            "To allow Zed to suggest better edits, turn on data collection. You \
                            can turn off at any time via the status bar menu.";
                        MessageNotification::new(message)
                            .with_title("Per-Project Data Collection Program")
                            .show_close_button(false)
                            .with_click_message("Turn On")
                            .on_click({
                                let this = this.clone();
                                move |_window, cx| {
                                    this.update(cx, |this, cx| {
                                        this.set_data_collection_choice(
                                            DataCollectionChoice::Enabled,
                                            cx,
                                        )
                                    });
                                }
                            })
                            .with_secondary_click_message("Turn Off")
                            .on_secondary_click({
                                move |_window, cx| {
                                    this.update(cx, |this, cx| {
                                        this.set_data_collection_choice(
                                            DataCollectionChoice::Disabled,
                                            cx,
                                        )
                                    });
                                }
                            })
                            .with_tertiary_click_message("Never Ask Again")
                            .on_tertiary_click({
                                move |_window, cx| {
                                    zeta.update(cx, |zeta, cx| {
                                        zeta.set_never_ask_again_for_data_collection(cx);
                                    });
                                }
                            })
                            .more_info_message("Learn More")
                            .more_info_url(DATA_COLLECTION_INFO_URL)
                    })
                });
            })
            .log_err();
    }

    fn discard(&mut self, _cx: &mut Context<Self>) {
        self.pending_completions.clear();
        self.current_completion.take();
    }

    fn suggest(
        &mut self,
        buffer: &Entity<Buffer>,
        cursor_position: language::Anchor,
        cx: &mut Context<Self>,
    ) -> Option<inline_completion::InlineCompletion> {
        let CurrentInlineCompletion {
            buffer_id,
            completion,
            ..
        } = self.current_completion.as_mut()?;

        // Invalidate previous completion if it was generated for a different buffer.
        if *buffer_id != buffer.entity_id() {
            self.current_completion.take();
            return None;
        }

        let buffer = buffer.read(cx);
        let Some(edits) = completion.interpolate(&buffer.snapshot()) else {
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
            edit_preview: Some(completion.edit_preview.clone()),
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
    async fn test_inline_completion_basic_interpolation(cx: &mut TestAppContext) {
        let buffer = cx.new(|cx| Buffer::local("Lorem ipsum dolor", cx));
        let edits: Arc<[(Range<Anchor>, String)]> = cx.update(|cx| {
            to_completion_edits(
                [(2..5, "REM".to_string()), (9..11, "".to_string())],
                &buffer,
                cx,
            )
            .into()
        });

        let edit_preview = cx
            .read(|cx| buffer.read(cx).preview_edits(edits.clone(), cx))
            .await;

        let completion = InlineCompletion {
            edits,
            edit_preview,
            path: Path::new("").into(),
            snapshot: cx.read(|cx| buffer.read(cx).snapshot()),
            id: InlineCompletionId::new(),
            excerpt_range: 0..0,
            cursor_offset: 0,
            input_outline: "".into(),
            input_events: "".into(),
            input_excerpt: "".into(),
            output_excerpt: "".into(),
            request_sent_at: Instant::now(),
            response_received_at: Instant::now(),
        };

        cx.update(|cx| {
            assert_eq!(
                from_completion_edits(
                    &completion.interpolate(&buffer.read(cx).snapshot()).unwrap(),
                    &buffer,
                    cx
                ),
                vec![(2..5, "REM".to_string()), (9..11, "".to_string())]
            );

            buffer.update(cx, |buffer, cx| buffer.edit([(2..5, "")], None, cx));
            assert_eq!(
                from_completion_edits(
                    &completion.interpolate(&buffer.read(cx).snapshot()).unwrap(),
                    &buffer,
                    cx
                ),
                vec![(2..2, "REM".to_string()), (6..8, "".to_string())]
            );

            buffer.update(cx, |buffer, cx| buffer.undo(cx));
            assert_eq!(
                from_completion_edits(
                    &completion.interpolate(&buffer.read(cx).snapshot()).unwrap(),
                    &buffer,
                    cx
                ),
                vec![(2..5, "REM".to_string()), (9..11, "".to_string())]
            );

            buffer.update(cx, |buffer, cx| buffer.edit([(2..5, "R")], None, cx));
            assert_eq!(
                from_completion_edits(
                    &completion.interpolate(&buffer.read(cx).snapshot()).unwrap(),
                    &buffer,
                    cx
                ),
                vec![(3..3, "EM".to_string()), (7..9, "".to_string())]
            );

            buffer.update(cx, |buffer, cx| buffer.edit([(3..3, "E")], None, cx));
            assert_eq!(
                from_completion_edits(
                    &completion.interpolate(&buffer.read(cx).snapshot()).unwrap(),
                    &buffer,
                    cx
                ),
                vec![(4..4, "M".to_string()), (8..10, "".to_string())]
            );

            buffer.update(cx, |buffer, cx| buffer.edit([(4..4, "M")], None, cx));
            assert_eq!(
                from_completion_edits(
                    &completion.interpolate(&buffer.read(cx).snapshot()).unwrap(),
                    &buffer,
                    cx
                ),
                vec![(9..11, "".to_string())]
            );

            buffer.update(cx, |buffer, cx| buffer.edit([(4..5, "")], None, cx));
            assert_eq!(
                from_completion_edits(
                    &completion.interpolate(&buffer.read(cx).snapshot()).unwrap(),
                    &buffer,
                    cx
                ),
                vec![(4..4, "M".to_string()), (8..10, "".to_string())]
            );

            buffer.update(cx, |buffer, cx| buffer.edit([(8..10, "")], None, cx));
            assert_eq!(
                from_completion_edits(
                    &completion.interpolate(&buffer.read(cx).snapshot()).unwrap(),
                    &buffer,
                    cx
                ),
                vec![(4..4, "M".to_string())]
            );

            buffer.update(cx, |buffer, cx| buffer.edit([(4..6, "")], None, cx));
            assert_eq!(completion.interpolate(&buffer.read(cx).snapshot()), None);
        })
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
        let user_store = cx.new(|cx| UserStore::new(client.clone(), cx));
        let zeta = cx.new(|cx| Zeta::new(client, user_store, cx));

        let buffer = cx.new(|cx| Buffer::local(buffer_content, cx));
        let cursor = buffer.read_with(cx, |buffer, _| buffer.anchor_before(Point::new(1, 0)));
        let completion_task = zeta.update(cx, |zeta, cx| {
            zeta.request_completion(&buffer, cursor, false, cx)
        });

        let token_request = server.receive::<proto::GetLlmToken>().await.unwrap();
        server.respond(
            token_request.receipt(),
            proto::GetLlmTokenResponse { token: "".into() },
        );

        let completion = completion_task.await.unwrap().unwrap();
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
        buffer: &Entity<Buffer>,
        cx: &App,
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
        buffer: &Entity<Buffer>,
        cx: &App,
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
