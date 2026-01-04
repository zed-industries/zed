use anyhow::{Context as _, Result};
use collections::VecDeque;
use edit_prediction_types::{EditPrediction, EditPredictionDelegate};
use futures::{AsyncBufReadExt, StreamExt, io::BufReader};
use gpui::{App, AppContext as _, Context, Entity, Global, SharedString, Subscription, Task};
use http_client::{AsyncBody, HttpClient, Method, Request as HttpRequest};
use icons::IconName;
use language::{Anchor, Buffer, BufferSnapshot, EditPreview, Point, ToPoint, text_diff};
use language_model::{ApiKeyState, EnvVar, env_var};
use lsp::DiagnosticSeverity;
use project::Project;
use serde::{Deserialize, Serialize};
use std::{fmt::Write as _, mem, ops::Range, path::Path, sync::Arc, time::Duration};
use text::ToOffset;
use uuid::Uuid;
use zeta_prompt::{Event, RelatedFile};

use crate::{EditPredictionExcerpt, EditPredictionExcerptOptions, RelatedExcerptStore};
pub const DEBOUNCE_TIMEOUT: Duration = Duration::from_millis(150);

pub const AMP_TAB_CREDENTIALS_URL: SharedString =
    SharedString::new_static("https://ampcode.com/api/tab/llm-proxy");
pub const AMP_TAB_CREDENTIALS_USERNAME: &str = "amp-tab-api-token";
pub static AMP_TAB_TOKEN_ENV_VAR: std::sync::LazyLock<EnvVar> = env_var!("AMP_TAB_API_KEY");

const AMP_TAB_API_URL: &str = "https://ampcode.com/api/tab/llm-proxy";
const AMP_TAB_MODEL: &str = "amp-tab-long-suggestion-model-instruct";

const EXCERPT_OPTIONS: EditPredictionExcerptOptions = EditPredictionExcerptOptions {
    max_bytes: 4000,
    min_bytes: 500,
    target_before_cursor_over_total_bytes: 0.66,
};

const DIAGNOSTIC_LINES_RANGE: u32 = 20;

struct GlobalAmpTabApiKey(Entity<ApiKeyState>);

impl Global for GlobalAmpTabApiKey {}

pub fn amp_tab_api_token(cx: &mut App) -> Entity<ApiKeyState> {
    if let Some(global) = cx.try_global::<GlobalAmpTabApiKey>() {
        return global.0.clone();
    }
    let entity =
        cx.new(|_| ApiKeyState::new(AMP_TAB_CREDENTIALS_URL, AMP_TAB_TOKEN_ENV_VAR.clone()));
    cx.set_global(GlobalAmpTabApiKey(entity.clone()));
    entity
}

pub fn try_amp_tab_api_token(cx: &App) -> Option<Entity<ApiKeyState>> {
    cx.try_global::<GlobalAmpTabApiKey>().map(|g| g.0.clone())
}

pub fn load_amp_tab_api_token(
    cx: &mut App,
) -> gpui::Task<Result<(), language_model::AuthenticateError>> {
    amp_tab_api_token(cx).update(cx, |key_state, cx| {
        key_state.load_if_needed(AMP_TAB_CREDENTIALS_URL, |s| s, cx)
    })
}

#[derive(Clone)]
struct CurrentCompletion {
    snapshot: BufferSnapshot,
    edits: Arc<[(Range<Anchor>, Arc<str>)]>,
    edit_preview: EditPreview,
}

impl CurrentCompletion {
    fn interpolate(&self, new_snapshot: &BufferSnapshot) -> Option<Vec<(Range<Anchor>, Arc<str>)>> {
        edit_prediction_types::interpolate_edits(&self.snapshot, new_snapshot, &self.edits)
    }
}

const EVENT_COUNT_MAX: usize = 20;

pub struct AmpTabEditPredictionDelegate {
    http_client: Arc<dyn HttpClient>,
    pending_request: Option<Task<Result<()>>>,
    current_completion: Option<CurrentCompletion>,
    queued_refresh: Option<QueuedRefresh>,
    related_excerpt_store: Option<Entity<RelatedExcerptStore>>,
    events: VecDeque<Arc<Event>>,
    registered_buffer: Option<RegisteredBuffer>,
    _subscriptions: Vec<Subscription>,
}

struct RegisteredBuffer {
    buffer_id: gpui::EntityId,
    snapshot: text::BufferSnapshot,
    file_path: Option<Arc<Path>>,
}

struct QueuedRefresh {
    buffer: Entity<Buffer>,
    cursor_position: Anchor,
    debounce: bool,
}

impl AmpTabEditPredictionDelegate {
    pub fn new(http_client: Arc<dyn HttpClient>) -> Self {
        Self {
            http_client,
            pending_request: None,
            current_completion: None,
            queued_refresh: None,
            related_excerpt_store: None,
            events: VecDeque::new(),
            registered_buffer: None,
            _subscriptions: Vec::new(),
        }
    }

    pub fn new_with_project(
        http_client: Arc<dyn HttpClient>,
        project: &Entity<Project>,
        cx: &mut Context<Self>,
    ) -> Self {
        let related_excerpt_store = cx.new(|cx| RelatedExcerptStore::new(project, cx));
        Self {
            http_client,
            pending_request: None,
            current_completion: None,
            queued_refresh: None,
            related_excerpt_store: Some(related_excerpt_store),
            events: VecDeque::new(),
            registered_buffer: None,
            _subscriptions: Vec::new(),
        }
    }

    fn register_buffer(&mut self, buffer: &Entity<Buffer>, cx: &mut Context<Self>) {
        let buf = buffer.read(cx);
        let buffer_id = buffer.entity_id();

        if self
            .registered_buffer
            .as_ref()
            .is_some_and(|rb| rb.buffer_id == buffer_id)
        {
            return;
        }

        let file_path = buf
            .file()
            .map(|f| Arc::from(f.path().as_ref().as_std_path()));
        let snapshot = buf.text_snapshot();

        self.registered_buffer = Some(RegisteredBuffer {
            buffer_id,
            snapshot,
            file_path,
        });

        let subscription = cx.subscribe(buffer, |this, buffer, event, cx| {
            if let language::BufferEvent::Edited = event {
                this.report_changes_for_buffer(&buffer, cx);
            }
        });
        self._subscriptions.push(subscription);
    }

    fn report_changes_for_buffer(&mut self, buffer: &Entity<Buffer>, cx: &Context<Self>) {
        let Some(registered) = self.registered_buffer.as_mut() else {
            return;
        };

        let buf = buffer.read(cx);
        let new_snapshot = buf.text_snapshot();

        if new_snapshot.version == registered.snapshot.version {
            return;
        }

        let new_file_path = buf
            .file()
            .map(|f| Arc::from(f.path().as_ref().as_std_path()));
        let old_file_path = mem::replace(&mut registered.file_path, new_file_path.clone());
        let old_snapshot = mem::replace(&mut registered.snapshot, new_snapshot.clone());

        let old_path = old_file_path.unwrap_or_else(|| Arc::from(Path::new("untitled")));
        let new_path = new_file_path.unwrap_or_else(|| Arc::from(Path::new("untitled")));

        let diff = compute_diff_for_event(&old_snapshot, &new_snapshot);
        let Some(diff) = diff else {
            return;
        };

        let event = Arc::new(Event::BufferChange {
            path: new_path,
            old_path,
            diff,
            predicted: false,
            in_open_source_repo: false,
        });

        if self.events.len() >= EVENT_COUNT_MAX {
            self.events.pop_front();
        }
        self.events.push_back(event);
    }

    pub fn has_api_key(cx: &App) -> bool {
        try_amp_tab_api_token(cx).is_some_and(|token| token.read(cx).has_key())
    }

    pub fn ensure_api_key_loaded(_http_client: Arc<dyn HttpClient>, cx: &mut App) {
        amp_tab_api_token(cx).update(cx, |key_state, cx| {
            _ = key_state.load_if_needed(AMP_TAB_CREDENTIALS_URL, |s| s, cx);
        });
    }

    fn api_key(cx: &App) -> Option<Arc<str>> {
        try_amp_tab_api_token(cx)?
            .read(cx)
            .key(&AMP_TAB_CREDENTIALS_URL)
    }

    async fn fetch_completion(
        http_client: Arc<dyn HttpClient>,
        api_key: Arc<str>,
        prompt: String,
        prediction_content: String,
    ) -> Result<String> {
        let request_id = Uuid::new_v4().to_string();

        log::debug!(
            "Amp Tab: Requesting completion (request_id: {})",
            request_id
        );

        let request_body = AmpTabRequest {
            stream: true,
            model: AMP_TAB_MODEL.to_string(),
            temperature: 0.1,
            max_tokens: 2000,
            response_format: ResponseFormat {
                r#type: "text".to_string(),
            },
            prediction: Prediction {
                r#type: "content".to_string(),
                content: prediction_content,
            },
            stop: vec!["<|editable_region_end|>".to_string()],
            prompt,
        };

        let request_json = serde_json::to_string(&request_body)?;

        let http_request = HttpRequest::builder()
            .method(Method::POST)
            .uri(AMP_TAB_API_URL)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", &*api_key))
            .header("x-amp-feature", "amp.tab")
            .header("x-amp-tab-request-id", &request_id)
            .body(AsyncBody::from(request_json))?;

        let mut response = http_client.send(http_request).await?;
        let status = response.status();

        log::debug!("Amp Tab: Response status: {}", status);

        if !status.is_success() {
            let mut body = String::new();
            futures::AsyncReadExt::read_to_string(response.body_mut(), &mut body).await?;
            return Err(anyhow::anyhow!("Amp Tab API error: {} - {}", status, body));
        }

        let reader = BufReader::new(response.into_body());
        let mut lines = reader.lines();
        let mut accumulated_content = String::new();

        while let Some(line_result) = lines.next().await {
            let line = line_result?;
            let line = line.trim();

            if line.is_empty() {
                continue;
            }

            let data = line
                .strip_prefix("data: ")
                .or_else(|| line.strip_prefix("data:"));

            let Some(data) = data else {
                continue;
            };

            if data == "[DONE]" {
                break;
            }

            match serde_json::from_str::<AmpTabStreamChunk>(data) {
                Ok(chunk) => {
                    if let Some(choice) = chunk.choices.first() {
                        if let Some(content) = &choice.delta.content {
                            accumulated_content.push_str(content);
                        }
                        if choice.finish_reason.is_some() {
                            break;
                        }
                    }
                }
                Err(e) => {
                    log::warn!("Amp Tab: Failed to parse SSE chunk: {} - data: {}", e, data);
                }
            }
        }

        log::debug!(
            "Amp Tab: Completion received ({} chars)",
            accumulated_content.len()
        );

        Ok(accumulated_content)
    }

    fn start_completion_request(
        &mut self,
        buffer: Entity<Buffer>,
        cursor_position: Anchor,
        debounce: bool,
        api_key: Arc<str>,
        cx: &mut Context<Self>,
    ) {
        let snapshot = buffer.read(cx).snapshot();
        let http_client = self.http_client.clone();
        let recent_copy = cx
            .read_from_clipboard()
            .and_then(|item| item.text())
            .unwrap_or_default();

        let related_files: Arc<[RelatedFile]> = self
            .related_excerpt_store
            .as_ref()
            .map(|store| store.read(cx).related_files())
            .unwrap_or_else(|| Arc::from([]));

        let events: Vec<Arc<Event>> = self.events.iter().cloned().collect();

        log::debug!(
            "Amp Tab: Starting new completion request (related_files: {}, events: {})",
            related_files.len(),
            events.len()
        );

        self.pending_request = Some(cx.spawn(async move |this, cx| {
            if debounce {
                log::debug!("Amp Tab: Debouncing for {:?}", DEBOUNCE_TIMEOUT);
                smol::Timer::after(DEBOUNCE_TIMEOUT).await;
            }

            let cursor_offset = cursor_position.to_offset(&snapshot);
            let cursor_point = cursor_offset.to_point(&snapshot);
            let excerpt = EditPredictionExcerpt::select_from_buffer(
                cursor_point,
                &snapshot,
                &EXCERPT_OPTIONS,
            )
            .context("Line containing cursor doesn't fit in excerpt max bytes")?;

            let excerpt_text = excerpt.text(&snapshot);
            let cursor_within_excerpt = cursor_offset
                .saturating_sub(excerpt.range.start)
                .min(excerpt_text.body.len());

            let before_cursor = &excerpt_text.body[..cursor_within_excerpt];
            let after_cursor = &excerpt_text.body[cursor_within_excerpt..];

            let file_path = snapshot.file().map(|f| f.path().as_unix_str());
            let lint_errors = format_lint_errors_for_prompt(&snapshot, cursor_point);
            let recently_viewed_snippets = format_recently_viewed_snippets(&related_files);
            let diff_history = format_diff_history(&events);

            let prompt_ctx = AmpTabPromptContext {
                file_path,
                before_cursor,
                after_cursor,
                lint_errors: &lint_errors,
                recently_viewed_snippets: &recently_viewed_snippets,
                diff_history: &diff_history,
                recent_copy: &recent_copy,
            };
            let prompt = build_amp_tab_prompt(&prompt_ctx);
            let prediction_content = excerpt_text.body.clone();

            let completion_text = match Self::fetch_completion(
                http_client,
                api_key,
                prompt,
                prediction_content,
            )
            .await
            {
                Ok(completion) => completion,
                Err(e) => {
                    log::error!("Amp Tab: Failed to fetch completion: {}", e);
                    this.update(cx, |this, cx| {
                        this.finish_request_and_process_queue(cx);
                    })?;
                    return Err(e);
                }
            };

            let new_excerpt_text = extract_edited_region_from_response(&completion_text);

            if new_excerpt_text.is_none() {
                log::debug!("Amp Tab: Could not extract edited region from response; ignoring");
                this.update(cx, |this, cx| {
                    this.finish_request_and_process_queue(cx);
                })?;
                return Ok(());
            }
            let new_excerpt_text = new_excerpt_text.unwrap();

            let old_excerpt_text = &excerpt_text.body;
            let excerpt_start_offset = excerpt.range.start;

            let edits = compute_edits_from_diff(
                old_excerpt_text,
                &new_excerpt_text,
                excerpt_start_offset,
                &snapshot,
            );

            if edits.is_empty() {
                log::debug!("Amp Tab: No changes detected in completion; ignoring");
                this.update(cx, |this, cx| {
                    this.finish_request_and_process_queue(cx);
                })?;
                return Ok(());
            }

            let whitespace_only = edits.iter().all(|(_, text)| text.trim().is_empty());
            if whitespace_only {
                log::debug!("Amp Tab: Rejecting prediction - whitespace-only change");
                this.update(cx, |this, cx| {
                    this.finish_request_and_process_queue(cx);
                })?;
                return Ok(());
            }

            let total_edit_len: usize = edits.iter().map(|(_, text)| text.len()).sum();
            const MAX_EDIT_CHARS: usize = 8000;
            if total_edit_len > MAX_EDIT_CHARS {
                log::debug!(
                    "Amp Tab: Rejecting prediction - big modification ({} chars exceeds {} limit)",
                    total_edit_len,
                    MAX_EDIT_CHARS
                );
                this.update(cx, |this, cx| {
                    this.finish_request_and_process_queue(cx);
                })?;
                return Ok(());
            }

            log::debug!(
                "Amp Tab: Computed {} edit(s) from diff ({} chars)",
                edits.len(),
                total_edit_len
            );

            let edits: Arc<[(Range<Anchor>, Arc<str>)]> = edits.into();
            let edit_preview = buffer
                .read_with(cx, |buffer, cx| buffer.preview_edits(edits.clone(), cx))?
                .await;

            this.update(cx, |this, cx| {
                log::debug!("Amp Tab: Completion stored and ready for suggestion");
                this.current_completion = Some(CurrentCompletion {
                    snapshot,
                    edits,
                    edit_preview,
                });
                this.finish_request_and_process_queue(cx);
            })?;

            Ok(())
        }));
    }

    fn finish_request_and_process_queue(&mut self, cx: &mut Context<Self>) {
        self.pending_request = None;

        if let Some(queued) = self.queued_refresh.take() {
            log::debug!("Amp Tab: Processing queued refresh request");
            self.refresh(queued.buffer, queued.cursor_position, queued.debounce, cx);
        } else {
            cx.notify();
        }
    }
}

#[derive(Debug, Serialize)]
struct AmpTabRequest {
    stream: bool,
    model: String,
    temperature: f32,
    max_tokens: u32,
    response_format: ResponseFormat,
    prediction: Prediction,
    stop: Vec<String>,
    prompt: String,
}

#[derive(Debug, Serialize)]
struct ResponseFormat {
    r#type: String,
}

#[derive(Debug, Serialize)]
struct Prediction {
    r#type: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct AmpTabStreamChunk {
    choices: Vec<StreamChoice>,
}

#[derive(Debug, Deserialize)]
struct StreamChoice {
    delta: StreamDelta,
    finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct StreamDelta {
    content: Option<String>,
}

impl EditPredictionDelegate for AmpTabEditPredictionDelegate {
    fn name() -> &'static str {
        "amp_tab"
    }

    fn display_name() -> &'static str {
        "Amp Tab"
    }

    fn show_predictions_in_menu() -> bool {
        true
    }

    fn icon(&self, _cx: &App) -> IconName {
        IconName::AmpTab
    }

    fn is_enabled(&self, _buffer: &Entity<Buffer>, _cursor_position: Anchor, cx: &App) -> bool {
        Self::api_key(cx).is_some()
    }

    fn is_refreshing(&self, _cx: &App) -> bool {
        self.pending_request.is_some()
    }

    fn refresh(
        &mut self,
        buffer: Entity<Buffer>,
        cursor_position: Anchor,
        debounce: bool,
        cx: &mut Context<Self>,
    ) {
        let snapshot = buffer.read(cx).snapshot();
        let cursor_offset = cursor_position.to_offset(&snapshot);
        let cursor_point = cursor_offset.to_point(&snapshot);

        log::debug!(
            "Amp Tab: Refresh requested (debounce: {}, cursor: line {} col {}, file: {:?})",
            debounce,
            cursor_point.row + 1,
            cursor_point.column,
            snapshot.file().map(|f| f.path().as_unix_str())
        );

        self.register_buffer(&buffer, cx);

        if let Some(related_excerpt_store) = &self.related_excerpt_store {
            related_excerpt_store.update(cx, |store, cx| {
                store.refresh(buffer.clone(), cursor_position, cx);
            });
        }

        let Some(api_key) = Self::api_key(cx) else {
            log::warn!("Amp Tab: No API key configured, skipping refresh");
            return;
        };

        if let Some(current_completion) = self.current_completion.as_ref() {
            if current_completion.interpolate(&snapshot).is_some() {
                log::debug!(
                    "Amp Tab: Existing completion still valid after interpolation, skipping new request"
                );
                return;
            }
        }

        if self.pending_request.is_some() {
            log::debug!("Amp Tab: Request already in flight, queueing refresh for later");
            self.queued_refresh = Some(QueuedRefresh {
                buffer,
                cursor_position,
                debounce,
            });
            return;
        }

        self.start_completion_request(buffer, cursor_position, debounce, api_key, cx);
    }

    fn accept(&mut self, _cx: &mut Context<Self>) {
        log::debug!("Amp Tab: Completion accepted");
        self.pending_request = None;
        self.current_completion = None;
        self.queued_refresh = None;
    }

    fn discard(&mut self, _cx: &mut Context<Self>) {
        log::debug!("Amp Tab: Completion discarded");
        self.pending_request = None;
        self.current_completion = None;
        self.queued_refresh = None;
    }

    fn suggest(
        &mut self,
        buffer: &Entity<Buffer>,
        cursor_position: Anchor,
        cx: &mut Context<Self>,
    ) -> Option<EditPrediction> {
        let current_completion = self.current_completion.as_ref();
        if current_completion.is_none() {
            log::debug!("Amp Tab: suggest() called but no current completion available");
            return None;
        }
        let current_completion = current_completion?;

        let buffer = buffer.read(cx);
        let snapshot = buffer.snapshot();
        let cursor_point = cursor_position.to_offset(&snapshot).to_point(&snapshot);

        let edits = current_completion.interpolate(&snapshot);
        if edits.is_none() {
            log::debug!(
                "Amp Tab: suggest() - interpolation failed (cursor: line {} col {})",
                cursor_point.row + 1,
                cursor_point.column
            );
            return None;
        }
        let edits = edits?;

        if edits.is_empty() {
            log::debug!("Amp Tab: suggest() - interpolated edits are empty");
            return None;
        }

        let total_edit_len: usize = edits.iter().map(|(_, text)| text.len()).sum();
        log::debug!(
            "Amp Tab: suggest() returning {} edit(s) totaling {} chars (cursor: line {} col {})",
            edits.len(),
            total_edit_len,
            cursor_point.row + 1,
            cursor_point.column
        );

        Some(EditPrediction::Local {
            id: None,
            edits,
            edit_preview: Some(current_completion.edit_preview.clone()),
        })
    }
}

struct AmpTabPromptContext<'a> {
    file_path: Option<&'a str>,
    before_cursor: &'a str,
    after_cursor: &'a str,
    lint_errors: &'a str,
    recently_viewed_snippets: &'a str,
    diff_history: &'a str,
    recent_copy: &'a str,
}

fn build_amp_tab_prompt(ctx: &AmpTabPromptContext<'_>) -> String {
    let mut prompt = String::new();

    prompt.push_str(
        "You are an intelligent programmer and an expert at coding. \
Your goal is to help a colleague finish a code change.\n\n",
    );

    prompt.push_str(
        "Help me finish a coding change. You will see snippets from current open files in my \
editor, files I have recently viewed, the file I am editing, then a history of my recent \
codebase changes, then current compiler and linter errors. You will then rewrite the code \
between the <|editable_region_start|> and <|editable_region_end|> tags, to match what you \
think I would do next in the codebase. <|user_cursor_is_here|> indicates the position of \
the cursor in the current file. Note: I might have stopped in the middle of typing.\n\n",
    );

    if !ctx.recently_viewed_snippets.is_empty() {
        writeln!(
            &mut prompt,
            "Code snippets I have recently viewed, roughly from oldest to newest. \
Some may be irrelevant to the change:\n"
        )
        .ok();
        writeln!(
            &mut prompt,
            "<recently_viewed_snippets>\n{}</recently_viewed_snippets>\n",
            ctx.recently_viewed_snippets,
        )
        .ok();
    }

    if !ctx.diff_history.is_empty() {
        writeln!(&mut prompt, "My recent edits, from oldest to newest:\n").ok();
        writeln!(
            &mut prompt,
            "<diff_history>\n{}</diff_history>\n",
            ctx.diff_history,
        )
        .ok();
    }

    if !ctx.lint_errors.is_empty() {
        writeln!(
            &mut prompt,
            "Linter errors from the code that you will rewrite:\n"
        )
        .ok();
        writeln!(
            &mut prompt,
            "<lint_errors>\n{}</lint_errors>\n",
            ctx.lint_errors,
        )
        .ok();
    }

    if !ctx.recent_copy.is_empty() {
        writeln!(&mut prompt, "Code I recently copied to clipboard:\n").ok();
        writeln!(
            &mut prompt,
            "<recent_copy>\n{}</recent_copy>\n",
            ctx.recent_copy,
        )
        .ok();
    }

    writeln!(&mut prompt, "The file currently open:\n").ok();
    writeln!(&mut prompt, "<file>").ok();
    if let Some(path) = ctx.file_path {
        writeln!(&mut prompt, "file_path: {}", path).ok();
    }
    writeln!(
        &mut prompt,
        "<|editable_region_start|>\n{}<|user_cursor_is_here|>{}\n<|editable_region_end|>",
        ctx.before_cursor, ctx.after_cursor
    )
    .ok();
    writeln!(&mut prompt, "</file>\n").ok();

    prompt.push_str(
        "Continue where I left off and finish my change by rewriting the code between the \
<|editable_region_start|> and <|editable_region_end|> tags:",
    );

    if !ctx.lint_errors.is_empty() {
        prompt.push_str(" Fix any relevant linter errors in the code you rewrite.");
    }

    prompt.push('\n');
    prompt
}

fn format_recently_viewed_snippets(related_files: &[RelatedFile]) -> String {
    let mut out = String::new();
    for related_file in related_files {
        for excerpt in &related_file.excerpts {
            writeln!(&mut out, "<snippet>").ok();
            writeln!(
                &mut out,
                "file_path: {}",
                related_file.path.to_string_lossy()
            )
            .ok();
            out.push_str(&excerpt.text);
            if !out.ends_with('\n') {
                out.push('\n');
            }
            writeln!(&mut out, "</snippet>").ok();
        }
    }
    out
}

fn format_diff_history(events: &[Arc<Event>]) -> String {
    let mut out = String::new();
    for event in events {
        zeta_prompt::write_event(&mut out, event);
    }
    out
}

fn compute_diff_for_event(
    old_snapshot: &text::BufferSnapshot,
    new_snapshot: &text::BufferSnapshot,
) -> Option<String> {
    use text::Edit;

    let edits: Vec<Edit<usize>> = new_snapshot
        .edits_since::<usize>(&old_snapshot.version)
        .collect();

    let (first_edit, last_edit) = edits.first().zip(edits.last())?;

    let old_start_point = old_snapshot.offset_to_point(first_edit.old.start);
    let old_end_point = old_snapshot.offset_to_point(last_edit.old.end);
    let new_start_point = new_snapshot.offset_to_point(first_edit.new.start);
    let new_end_point = new_snapshot.offset_to_point(last_edit.new.end);

    const CONTEXT_LINES: u32 = 3;

    let old_context_start_row = old_start_point.row.saturating_sub(CONTEXT_LINES);
    let new_context_start_row = new_start_point.row.saturating_sub(CONTEXT_LINES);
    let old_context_end_row =
        (old_end_point.row + 1 + CONTEXT_LINES).min(old_snapshot.max_point().row);
    let new_context_end_row =
        (new_end_point.row + 1 + CONTEXT_LINES).min(new_snapshot.max_point().row);

    let old_start_line_offset = old_snapshot.point_to_offset(Point::new(old_context_start_row, 0));
    let new_start_line_offset = new_snapshot.point_to_offset(Point::new(new_context_start_row, 0));
    let old_end_line_offset = old_snapshot
        .point_to_offset(Point::new(old_context_end_row + 1, 0).min(old_snapshot.max_point()));
    let new_end_line_offset = new_snapshot
        .point_to_offset(Point::new(new_context_end_row + 1, 0).min(new_snapshot.max_point()));
    let old_edit_range = old_start_line_offset..old_end_line_offset;
    let new_edit_range = new_start_line_offset..new_end_line_offset;

    let old_region_text: String = old_snapshot.text_for_range(old_edit_range).collect();
    let new_region_text: String = new_snapshot.text_for_range(new_edit_range).collect();

    let diff = language::unified_diff_with_offsets(
        &old_region_text,
        &new_region_text,
        old_context_start_row,
        new_context_start_row,
    );

    if diff.is_empty() { None } else { Some(diff) }
}

/// Extracts the edited region content from the model's response.
/// Returns None if the start marker is not found.
/// If the end marker is missing (likely truncated by the stop sequence), uses the rest of the response.
fn extract_edited_region_from_response(response: &str) -> Option<String> {
    const START_MARKER: &str = "<|editable_region_start|>";
    const END_MARKER: &str = "<|editable_region_end|>";
    const CURSOR_MARKER: &str = "<|user_cursor_is_here|>";

    // Find the start marker
    let start_pos = response.find(START_MARKER)?;
    let after_start = &response[start_pos + START_MARKER.len()..];

    // Find the end marker if present, otherwise use the rest of the response.
    // The end marker is often missing because it's configured as a stop sequence,
    // so the model's response gets truncated before it appears.
    let content = match after_start.find(END_MARKER) {
        Some(end_pos) => &after_start[..end_pos],
        None => after_start,
    };

    // Remove leading newline (model often adds one after the start marker)
    let content = content.strip_prefix('\n').unwrap_or(content);

    // Remove the cursor marker if present
    let content = content.replace(CURSOR_MARKER, "");

    Some(content)
}

/// Computes edits by diffing the old excerpt text against the new excerpt text.
/// The resulting edits are anchored to positions in the buffer.
fn compute_edits_from_diff(
    old_text: &str,
    new_text: &str,
    excerpt_start_offset: usize,
    snapshot: &BufferSnapshot,
) -> Vec<(Range<Anchor>, Arc<str>)> {
    text_diff(old_text, new_text)
        .into_iter()
        .map(|(range_in_excerpt, new_text)| {
            let buffer_start = excerpt_start_offset + range_in_excerpt.start;
            let buffer_end = excerpt_start_offset + range_in_excerpt.end;

            let range = if buffer_start == buffer_end {
                let anchor = snapshot.anchor_after(buffer_start);
                anchor..anchor
            } else {
                snapshot.anchor_after(buffer_start)..snapshot.anchor_before(buffer_end)
            };

            (range, new_text)
        })
        .collect()
}

fn format_lint_errors_for_prompt(snapshot: &BufferSnapshot, cursor_point: Point) -> String {
    let diagnostic_search_start = cursor_point.row.saturating_sub(DIAGNOSTIC_LINES_RANGE);
    let diagnostic_search_end = cursor_point.row + DIAGNOSTIC_LINES_RANGE;
    let diagnostic_search_range =
        Point::new(diagnostic_search_start, 0)..Point::new(diagnostic_search_end, 0);

    let diagnostic_entries = snapshot.diagnostics_in_range(diagnostic_search_range, false);
    let mut diagnostic_content = String::new();

    for entry in diagnostic_entries {
        let start_point: Point = entry.range.start;

        let severity = match entry.diagnostic.severity {
            DiagnosticSeverity::ERROR => "error",
            DiagnosticSeverity::WARNING => "warning",
            DiagnosticSeverity::INFORMATION => "info",
            DiagnosticSeverity::HINT => "hint",
            _ => continue,
        };

        writeln!(
            &mut diagnostic_content,
            "{} at line {}: {}",
            severity,
            start_point.row + 1,
            entry.diagnostic.message
        )
        .ok();
    }

    diagnostic_content
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_extract_edited_region_basic() {
        let response = "<|editable_region_start|>\nfunction add(a: number, b: number): number {\n  return a + b;\n}\n<|editable_region_end|>";
        let result = extract_edited_region_from_response(response);
        assert_eq!(
            result,
            Some("function add(a: number, b: number): number {\n  return a + b;\n}\n".to_string())
        );
    }

    #[test]
    fn test_extract_edited_region_with_cursor_marker() {
        let response = "<|editable_region_start|>\nfunction add(a: number, b: number): number {\n  return <|user_cursor_is_here|>a + b;\n}\n<|editable_region_end|>";
        let result = extract_edited_region_from_response(response);
        assert_eq!(
            result,
            Some("function add(a: number, b: number): number {\n  return a + b;\n}\n".to_string())
        );
    }

    #[test]
    fn test_extract_edited_region_no_markers() {
        let response = "hello world";
        let result = extract_edited_region_from_response(response);
        assert_eq!(result, None);
    }

    #[test]
    fn test_extract_edited_region_missing_end_marker() {
        // When the end marker is missing (truncated by stop sequence), use the rest of the response
        let response = "<|editable_region_start|>\nsome content";
        let result = extract_edited_region_from_response(response);
        assert_eq!(result, Some("some content".to_string()));
    }

    #[test]
    fn test_extract_edited_region_strips_leading_newline() {
        let response = "<|editable_region_start|>\nconst x = 42;\n<|editable_region_end|>";
        let result = extract_edited_region_from_response(response);
        assert_eq!(result, Some("const x = 42;\n".to_string()));
    }

    #[test]
    fn test_extract_edited_region_no_leading_newline() {
        let response = "<|editable_region_start|>const x = 42;\n<|editable_region_end|>";
        let result = extract_edited_region_from_response(response);
        assert_eq!(result, Some("const x = 42;\n".to_string()));
    }

    #[test]
    fn test_build_prompt_without_diagnostics() {
        let ctx = AmpTabPromptContext {
            file_path: None,
            before_cursor: "let x = ",
            after_cursor: ";",
            lint_errors: "",
            recently_viewed_snippets: "",
            diff_history: "",
            recent_copy: "",
        };
        let prompt = build_amp_tab_prompt(&ctx);

        assert!(prompt.contains("You are an intelligent programmer"));
        assert!(prompt.contains("let x = <|user_cursor_is_here|>;"));
        assert!(!prompt.contains("<lint_errors>"));
        assert!(!prompt.contains("<recent_copy>"));
    }

    #[test]
    fn test_build_prompt_with_lint_errors() {
        let ctx = AmpTabPromptContext {
            file_path: Some("src/main.rs"),
            before_cursor: "let x = ",
            after_cursor: ";",
            lint_errors: "error at line 5: cannot find value `foo`\nwarning at line 7: unused variable\n",
            recently_viewed_snippets: "",
            diff_history: "",
            recent_copy: "",
        };
        let prompt = build_amp_tab_prompt(&ctx);

        assert!(prompt.contains("You are an intelligent programmer"));
        assert!(prompt.contains("let x = <|user_cursor_is_here|>;"));
        assert!(prompt.contains("<lint_errors>"));
        assert!(prompt.contains("error at line 5: cannot find value `foo`"));
        assert!(prompt.contains("warning at line 7: unused variable"));
        assert!(prompt.contains("</lint_errors>"));
        assert!(prompt.contains("file_path: src/main.rs"));
        assert!(prompt.contains("Fix any relevant linter errors"));
    }

    #[test]
    fn test_build_prompt_with_recently_viewed_snippets() {
        let ctx = AmpTabPromptContext {
            file_path: None,
            before_cursor: "fn main() {",
            after_cursor: "}",
            lint_errors: "",
            recently_viewed_snippets: "<snippet>\nfile_path: src/lib.rs\nfn helper() {}\n</snippet>\n",
            diff_history: "",
            recent_copy: "",
        };
        let prompt = build_amp_tab_prompt(&ctx);

        assert!(prompt.contains("<recently_viewed_snippets>"));
        assert!(prompt.contains("file_path: src/lib.rs"));
        assert!(prompt.contains("fn helper() {}"));
        assert!(prompt.contains("</recently_viewed_snippets>"));
    }

    #[test]
    fn test_build_prompt_with_diff_history() {
        let ctx = AmpTabPromptContext {
            file_path: None,
            before_cursor: "let y = ",
            after_cursor: ";",
            lint_errors: "",
            recently_viewed_snippets: "",
            diff_history: "--- a/src/main.rs\n+++ b/src/main.rs\n@@ -1 +1 @@\n-old\n+new\n",
            recent_copy: "",
        };
        let prompt = build_amp_tab_prompt(&ctx);

        assert!(prompt.contains("<diff_history>"));
        assert!(prompt.contains("--- a/src/main.rs"));
        assert!(prompt.contains("+++ b/src/main.rs"));
        assert!(prompt.contains("</diff_history>"));
    }

    #[test]
    fn test_build_prompt_with_recent_copy() {
        let ctx = AmpTabPromptContext {
            file_path: None,
            before_cursor: "let z = ",
            after_cursor: ";",
            lint_errors: "",
            recently_viewed_snippets: "",
            diff_history: "",
            recent_copy: "some_function(arg1, arg2)",
        };
        let prompt = build_amp_tab_prompt(&ctx);

        assert!(prompt.contains("<recent_copy>"));
        assert!(prompt.contains("some_function(arg1, arg2)"));
        assert!(prompt.contains("</recent_copy>"));
        assert!(prompt.contains("Code I recently copied to clipboard"));
    }

    #[test]
    fn test_format_recently_viewed_snippets() {
        let related_files = vec![RelatedFile {
            path: Arc::from(Path::new("src/utils.rs")),
            max_row: 10,
            excerpts: vec![zeta_prompt::RelatedExcerpt {
                row_range: 0..5,
                text: "fn helper() {\n    println!(\"hello\");\n}".to_string(),
            }],
        }];

        let output = format_recently_viewed_snippets(&related_files);
        assert!(output.contains("<snippet>"));
        assert!(output.contains("file_path: src/utils.rs"));
        assert!(output.contains("fn helper()"));
        assert!(output.contains("</snippet>"));
    }

    #[test]
    fn test_format_diff_history() {
        let events = vec![Arc::new(Event::BufferChange {
            path: Arc::from(Path::new("src/main.rs")),
            old_path: Arc::from(Path::new("src/main.rs")),
            diff: "@@ -1 +1 @@\n-old\n+new\n".to_string(),
            predicted: false,
            in_open_source_repo: false,
        })];

        let output = format_diff_history(&events);
        assert!(output.contains("--- a/src/main.rs"));
        assert!(output.contains("+++ b/src/main.rs"));
        assert!(output.contains("-old"));
        assert!(output.contains("+new"));
    }
}
