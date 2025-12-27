use anyhow::{Context as _, Result};
use edit_prediction_types::{EditPrediction, EditPredictionDelegate};
use futures::{AsyncBufReadExt, StreamExt, io::BufReader};
use gpui::{App, AppContext as _, Context, Entity, Global, SharedString, Task};
use http_client::{AsyncBody, HttpClient, Method, Request as HttpRequest};
use icons::IconName;
use language::{Anchor, Buffer, BufferSnapshot, EditPreview, Point, ToPoint, text_diff};
use language_model::{ApiKeyState, EnvVar, env_var};
use lsp::DiagnosticSeverity;
use serde::{Deserialize, Serialize};
use std::{fmt::Write as _, ops::Range, sync::Arc, time::Duration};
use text::ToOffset;
use uuid::Uuid;

use crate::{EditPredictionExcerpt, EditPredictionExcerptOptions};

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

pub struct AmpTabEditPredictionDelegate {
    http_client: Arc<dyn HttpClient>,
    pending_request: Option<Task<Result<()>>>,
    current_completion: Option<CurrentCompletion>,
    queued_refresh: Option<QueuedRefresh>,
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
        }
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

        log::debug!("Amp Tab: Starting new completion request");

        self.pending_request =
            Some(cx.spawn(async move |this, cx| {
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

                let diagnostics = format_diagnostics_for_prompt(&snapshot, cursor_point);
                let prompt = build_amp_tab_prompt(before_cursor, after_cursor, &diagnostics);
                let prediction_content = excerpt_text.body.clone();

                let completion_text =
                    match Self::fetch_completion(http_client, api_key, prompt, prediction_content)
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

                log::debug!("Amp Tab: Computed {} edit(s) from diff", edits.len());

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

fn build_amp_tab_prompt(before_cursor: &str, after_cursor: &str, diagnostics: &str) -> String {
    let (diagnostics_section, diagnostics_instruction) = if diagnostics.is_empty() {
        (String::new(), String::new())
    } else {
        (
            format!(
                r#"
Diagnostics near the cursor:

<diagnostics>
{diagnostics}</diagnostics>
"#
            ),
            " If there are any errors or warnings in the diagnostics, fix them.".to_string(),
        )
    };

    format!(
        r#"Help me finish a coding change. You will see the file I am editing. You will then rewrite the code between the <|editable_region_start|> and <|editable_region_end|> tags, to match what you think I would do next in the codebase. <|user_cursor_is_here|> indicates the position of the cursor in the current file. Note: I might have stopped in the middle of typing.
{diagnostics_section}
The file currently open:

<file>
<|editable_region_start|>
{before_cursor}<|user_cursor_is_here|>{after_cursor}
<|editable_region_end|>
</file>

Continue where I left off and finish my change by rewriting the code between the <|editable_region_start|> and <|editable_region_end|> tags:{diagnostics_instruction}
"#
    )
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

fn format_diagnostics_for_prompt(snapshot: &BufferSnapshot, cursor_point: Point) -> String {
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

        let _ = writeln!(
            &mut diagnostic_content,
            "{} at line {}: {}",
            severity,
            start_point.row + 1,
            entry.diagnostic.message
        );
    }

    diagnostic_content
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let prompt = build_amp_tab_prompt("let x = ", ";", "");
        assert!(prompt.contains("let x = <|user_cursor_is_here|>;"));
        assert!(!prompt.contains("<diagnostics>"));
    }

    #[test]
    fn test_build_prompt_with_diagnostics() {
        let diagnostics =
            "error at line 5: cannot find value `foo`\nwarning at line 7: unused variable\n";
        let prompt = build_amp_tab_prompt("let x = ", ";", diagnostics);
        assert!(prompt.contains("let x = <|user_cursor_is_here|>;"));
        assert!(prompt.contains("<diagnostics>"));
        assert!(prompt.contains("error at line 5: cannot find value `foo`"));
        assert!(prompt.contains("warning at line 7: unused variable"));
        assert!(prompt.contains("</diagnostics>"));
    }
}
