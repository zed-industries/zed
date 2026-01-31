use crate::{
    CurrentEditPrediction, DebugEvent, EditPrediction, EditPredictionFinishedDebugEvent,
    EditPredictionId, EditPredictionModelInput, EditPredictionStartedDebugEvent,
    EditPredictionStore, UserActionRecord, UserActionType, prediction::EditPredictionResult,
};
use anyhow::{Result, bail};
use client::Client;
use edit_prediction_types::SuggestionDisplayType;
use futures::{AsyncReadExt as _, channel::mpsc};
use gpui::{
    App, AppContext as _, Entity, Global, SharedString, Task,
    http_client::{self, AsyncBody, Method},
};
use language::{Anchor, Buffer, BufferSnapshot, Point, ToOffset as _};
use language_model::{ApiKeyState, EnvVar, env_var};
use lsp::DiagnosticSeverity;
use serde::{Deserialize, Serialize};
use std::{
    fmt::{self, Write as _},
    ops::Range,
    path::Path,
    sync::Arc,
    time::Instant,
};

const SWEEP_API_URL: &str = "https://autocomplete.sweep.dev/backend/next_edit_autocomplete";
const SWEEP_METRICS_URL: &str = "https://backend.app.sweep.dev/backend/track_autocomplete_metrics";

pub struct SweepAi {
    pub api_token: Entity<ApiKeyState>,
    pub debug_info: Arc<str>,
}

impl SweepAi {
    pub fn new(cx: &mut App) -> Self {
        SweepAi {
            api_token: sweep_api_token(cx),
            debug_info: debug_info(cx),
        }
    }

    pub fn request_prediction_with_sweep(
        &self,
        inputs: EditPredictionModelInput,
        cx: &mut App,
    ) -> Task<Result<Option<EditPredictionResult>>> {
        let debug_info = self.debug_info.clone();
        self.api_token.update(cx, |key_state, cx| {
            _ = key_state.load_if_needed(SWEEP_CREDENTIALS_URL, |s| s, cx);
        });

        let buffer = inputs.buffer.clone();
        let debug_tx = inputs.debug_tx.clone();

        let Some(api_token) = self.api_token.read(cx).key(&SWEEP_CREDENTIALS_URL) else {
            return Task::ready(Ok(None));
        };
        let full_path: Arc<Path> = inputs
            .snapshot
            .file()
            .map(|file| file.full_path(cx))
            .unwrap_or_else(|| "untitled".into())
            .into();

        let project_file = project::File::from_dyn(inputs.snapshot.file());
        let repo_name = project_file
            .map(|file| file.worktree.read(cx).root_name_str())
            .unwrap_or("untitled")
            .into();
        let offset = inputs.position.to_offset(&inputs.snapshot);
        let buffer_entity_id = inputs.buffer.entity_id();

        let recent_buffers = inputs.recent_paths.iter().cloned();
        let http_client = cx.http_client();

        let recent_buffer_snapshots = recent_buffers
            .filter_map(|project_path| {
                let buffer = inputs.project.read(cx).get_open_buffer(&project_path, cx)?;
                if inputs.buffer == buffer {
                    None
                } else {
                    Some(buffer.read(cx).snapshot())
                }
            })
            .take(3)
            .collect::<Vec<_>>();

        let buffer_snapshotted_at = Instant::now();

        let result = cx.background_spawn(async move {
            let text = inputs.snapshot.text();

            let mut recent_changes = String::new();
            for event in &inputs.events {
                write_event(event.as_ref(), &mut recent_changes).unwrap();
            }

            let file_chunks = recent_buffer_snapshots
                .into_iter()
                .map(|snapshot| {
                    let end_point = Point::new(30, 0).min(snapshot.max_point());
                    FileChunk {
                        content: snapshot.text_for_range(Point::zero()..end_point).collect(),
                        file_path: snapshot
                            .file()
                            .map(|f| f.path().as_unix_str())
                            .unwrap_or("untitled")
                            .to_string(),
                        start_line: 0,
                        end_line: end_point.row as usize,
                        timestamp: snapshot.file().and_then(|file| {
                            Some(
                                file.disk_state()
                                    .mtime()?
                                    .to_seconds_and_nanos_for_persistence()?
                                    .0,
                            )
                        }),
                    }
                })
                .collect::<Vec<_>>();

            let mut retrieval_chunks: Vec<FileChunk> = inputs
                .related_files
                .iter()
                .flat_map(|related_file| {
                    related_file.excerpts.iter().map(|excerpt| FileChunk {
                        file_path: related_file.path.to_string_lossy().to_string(),
                        start_line: excerpt.row_range.start as usize,
                        end_line: excerpt.row_range.end as usize,
                        content: excerpt.text.to_string(),
                        timestamp: None,
                    })
                })
                .collect();

            let diagnostic_entries = inputs
                .snapshot
                .diagnostics_in_range(inputs.diagnostic_search_range, false);
            let mut diagnostic_content = String::new();
            let mut diagnostic_count = 0;

            for entry in diagnostic_entries {
                let start_point: Point = entry.range.start;

                let severity = match entry.diagnostic.severity {
                    DiagnosticSeverity::ERROR => "error",
                    DiagnosticSeverity::WARNING => "warning",
                    DiagnosticSeverity::INFORMATION => "info",
                    DiagnosticSeverity::HINT => "hint",
                    _ => continue,
                };

                diagnostic_count += 1;

                writeln!(
                    &mut diagnostic_content,
                    "{}:{}:{}: {}: {}",
                    full_path.display(),
                    start_point.row + 1,
                    start_point.column + 1,
                    severity,
                    entry.diagnostic.message
                )?;
            }

            if !diagnostic_content.is_empty() {
                retrieval_chunks.push(FileChunk {
                    file_path: "diagnostics".to_string(),
                    start_line: 1,
                    end_line: diagnostic_count,
                    content: diagnostic_content,
                    timestamp: None,
                });
            }

            let file_path_str = full_path.display().to_string();
            let recent_user_actions = inputs
                .user_actions
                .iter()
                .filter(|r| r.buffer_id == buffer_entity_id)
                .map(|r| to_sweep_user_action(r, &file_path_str))
                .collect();

            let request_body = AutocompleteRequest {
                debug_info,
                repo_name,
                file_path: full_path.clone(),
                file_contents: text.clone(),
                original_file_contents: text,
                cursor_position: offset,
                recent_changes: recent_changes.clone(),
                changes_above_cursor: true,
                multiple_suggestions: false,
                branch: None,
                file_chunks,
                retrieval_chunks,
                recent_user_actions,
                use_bytes: true,
                // TODO
                privacy_mode_enabled: false,
            };

            let mut buf: Vec<u8> = Vec::new();
            let writer = brotli::CompressorWriter::new(&mut buf, 4096, 1, 22);
            serde_json::to_writer(writer, &request_body)?;
            let body: AsyncBody = buf.into();

            let ep_inputs = zeta_prompt::ZetaPromptInput {
                events: inputs.events,
                related_files: inputs.related_files.clone(),
                cursor_path: full_path.clone(),
                cursor_excerpt: request_body.file_contents.clone().into(),
                // we actually don't know
                editable_range_in_excerpt: 0..inputs.snapshot.len(),
                cursor_offset_in_excerpt: request_body.cursor_position,
                excerpt_start_row: Some(0),
            };

            send_started_event(
                &debug_tx,
                &buffer,
                inputs.position,
                serde_json::to_string(&request_body).unwrap_or_default(),
            );

            let request = http_client::Request::builder()
                .uri(SWEEP_API_URL)
                .header("Content-Type", "application/json")
                .header("Authorization", format!("Bearer {}", api_token))
                .header("Connection", "keep-alive")
                .header("Content-Encoding", "br")
                .method(Method::POST)
                .body(body)?;

            let mut response = http_client.send(request).await?;

            let mut body = String::new();
            response.body_mut().read_to_string(&mut body).await?;

            let response_received_at = Instant::now();
            if !response.status().is_success() {
                let message = format!(
                    "Request failed with status: {:?}\nBody: {}",
                    response.status(),
                    body,
                );
                send_finished_event(&debug_tx, &buffer, inputs.position, message.clone());
                bail!(message);
            };

            let response: AutocompleteResponse = serde_json::from_str(&body)?;

            send_finished_event(&debug_tx, &buffer, inputs.position, body);

            let old_text = inputs
                .snapshot
                .text_for_range(response.start_index..response.end_index)
                .collect::<String>();
            let edits = language::text_diff(&old_text, &response.completion)
                .into_iter()
                .map(|(range, text)| {
                    (
                        inputs
                            .snapshot
                            .anchor_after(response.start_index + range.start)
                            ..inputs
                                .snapshot
                                .anchor_before(response.start_index + range.end),
                        text,
                    )
                })
                .collect::<Vec<_>>();

            anyhow::Ok((
                response.autocomplete_id,
                edits,
                inputs.snapshot,
                response_received_at,
                ep_inputs,
            ))
        });

        let buffer = inputs.buffer.clone();

        cx.spawn(async move |cx| {
            let (id, edits, old_snapshot, response_received_at, inputs) = result.await?;
            anyhow::Ok(Some(
                EditPredictionResult::new(
                    EditPredictionId(id.into()),
                    &buffer,
                    &old_snapshot,
                    edits.into(),
                    buffer_snapshotted_at,
                    response_received_at,
                    inputs,
                    cx,
                )
                .await,
            ))
        })
    }
}

fn send_started_event(
    debug_tx: &Option<mpsc::UnboundedSender<DebugEvent>>,
    buffer: &Entity<Buffer>,
    position: Anchor,
    prompt: String,
) {
    if let Some(debug_tx) = debug_tx {
        _ = debug_tx.unbounded_send(DebugEvent::EditPredictionStarted(
            EditPredictionStartedDebugEvent {
                buffer: buffer.downgrade(),
                position,
                prompt: Some(prompt),
            },
        ));
    }
}

fn send_finished_event(
    debug_tx: &Option<mpsc::UnboundedSender<DebugEvent>>,
    buffer: &Entity<Buffer>,
    position: Anchor,
    model_output: String,
) {
    if let Some(debug_tx) = debug_tx {
        _ = debug_tx.unbounded_send(DebugEvent::EditPredictionFinished(
            EditPredictionFinishedDebugEvent {
                buffer: buffer.downgrade(),
                position,
                model_output: Some(model_output),
            },
        ));
    }
}

pub const SWEEP_CREDENTIALS_URL: SharedString =
    SharedString::new_static("https://autocomplete.sweep.dev");
pub const SWEEP_CREDENTIALS_USERNAME: &str = "sweep-api-token";
pub static SWEEP_AI_TOKEN_ENV_VAR: std::sync::LazyLock<EnvVar> = env_var!("SWEEP_AI_TOKEN");

struct GlobalSweepApiKey(Entity<ApiKeyState>);

impl Global for GlobalSweepApiKey {}

pub fn sweep_api_token(cx: &mut App) -> Entity<ApiKeyState> {
    if let Some(global) = cx.try_global::<GlobalSweepApiKey>() {
        return global.0.clone();
    }
    let entity =
        cx.new(|_| ApiKeyState::new(SWEEP_CREDENTIALS_URL, SWEEP_AI_TOKEN_ENV_VAR.clone()));
    cx.set_global(GlobalSweepApiKey(entity.clone()));
    entity
}

pub fn load_sweep_api_token(cx: &mut App) -> Task<Result<(), language_model::AuthenticateError>> {
    sweep_api_token(cx).update(cx, |key_state, cx| {
        key_state.load_if_needed(SWEEP_CREDENTIALS_URL, |s| s, cx)
    })
}

#[derive(Debug, Clone, Serialize)]
struct AutocompleteRequest {
    pub debug_info: Arc<str>,
    pub repo_name: String,
    pub branch: Option<String>,
    pub file_path: Arc<Path>,
    pub file_contents: String,
    pub recent_changes: String,
    pub cursor_position: usize,
    pub original_file_contents: String,
    pub file_chunks: Vec<FileChunk>,
    pub retrieval_chunks: Vec<FileChunk>,
    pub recent_user_actions: Vec<UserAction>,
    pub multiple_suggestions: bool,
    pub privacy_mode_enabled: bool,
    pub changes_above_cursor: bool,
    pub use_bytes: bool,
}

#[derive(Debug, Clone, Serialize)]
struct FileChunk {
    pub file_path: String,
    pub start_line: usize,
    pub end_line: usize,
    pub content: String,
    pub timestamp: Option<u64>,
}

#[derive(Debug, Clone, Serialize)]
struct UserAction {
    pub action_type: ActionType,
    pub line_number: usize,
    pub offset: usize,
    pub file_path: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
enum ActionType {
    CursorMovement,
    InsertChar,
    DeleteChar,
    InsertSelection,
    DeleteSelection,
}

fn to_sweep_user_action(record: &UserActionRecord, file_path: &str) -> UserAction {
    UserAction {
        action_type: match record.action_type {
            UserActionType::InsertChar => ActionType::InsertChar,
            UserActionType::InsertSelection => ActionType::InsertSelection,
            UserActionType::DeleteChar => ActionType::DeleteChar,
            UserActionType::DeleteSelection => ActionType::DeleteSelection,
            UserActionType::CursorMovement => ActionType::CursorMovement,
        },
        line_number: record.line_number as usize,
        offset: record.offset,
        file_path: file_path.to_string(),
        timestamp: record.timestamp_epoch_ms,
    }
}

#[derive(Debug, Clone, Deserialize)]
struct AutocompleteResponse {
    pub autocomplete_id: String,
    pub start_index: usize,
    pub end_index: usize,
    pub completion: String,
    #[allow(dead_code)]
    pub confidence: f64,
    #[allow(dead_code)]
    pub logprobs: Option<serde_json::Value>,
    #[allow(dead_code)]
    pub finish_reason: Option<String>,
    #[allow(dead_code)]
    pub elapsed_time_ms: u64,
    #[allow(dead_code)]
    #[serde(default, rename = "completions")]
    pub additional_completions: Vec<AdditionalCompletion>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
struct AdditionalCompletion {
    pub start_index: usize,
    pub end_index: usize,
    pub completion: String,
    pub confidence: f64,
    pub autocomplete_id: String,
    pub logprobs: Option<serde_json::Value>,
    pub finish_reason: Option<String>,
}

fn write_event(event: &zeta_prompt::Event, f: &mut impl fmt::Write) -> fmt::Result {
    match event {
        zeta_prompt::Event::BufferChange {
            old_path,
            path,
            diff,
            ..
        } => {
            if old_path != path {
                // TODO confirm how to do this for sweep
                // writeln!(f, "User renamed {:?} to {:?}\n", old_path, new_path)?;
            }

            if !diff.is_empty() {
                write!(f, "File: {}:\n{}\n", path.display(), diff)?
            }

            fmt::Result::Ok(())
        }
    }
}

fn debug_info(cx: &gpui::App) -> Arc<str> {
    format!(
        "Zed v{version} ({sha}) - OS: {os} - Zed v{version}",
        version = release_channel::AppVersion::global(cx),
        sha = release_channel::AppCommitSha::try_global(cx)
            .map_or("unknown".to_string(), |sha| sha.full()),
        os = client::telemetry::os_name(),
    )
    .into()
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SweepEventType {
    AutocompleteSuggestionShown,
    AutocompleteSuggestionAccepted,
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SweepSuggestionType {
    GhostText,
    Popup,
    JumpToEdit,
}

#[derive(Debug, Clone, Serialize)]
struct AutocompleteMetricsRequest {
    event_type: SweepEventType,
    suggestion_type: SweepSuggestionType,
    additions: u32,
    deletions: u32,
    autocomplete_id: String,
    edit_tracking: String,
    edit_tracking_line: Option<u32>,
    lifespan: Option<u64>,
    debug_info: Arc<str>,
    device_id: String,
    privacy_mode_enabled: bool,
}

fn send_autocomplete_metrics_request(
    cx: &App,
    client: Arc<Client>,
    api_token: Arc<str>,
    request_body: AutocompleteMetricsRequest,
) {
    let http_client = client.http_client();
    cx.background_spawn(async move {
        let body: AsyncBody = serde_json::to_string(&request_body)?.into();

        let request = http_client::Request::builder()
            .uri(SWEEP_METRICS_URL)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", api_token))
            .method(Method::POST)
            .body(body)?;

        let mut response = http_client.send(request).await?;

        if !response.status().is_success() {
            let mut body = String::new();
            response.body_mut().read_to_string(&mut body).await?;
            anyhow::bail!(
                "Failed to send autocomplete metrics for sweep_ai: {:?}\nBody: {}",
                response.status(),
                body,
            );
        }

        Ok(())
    })
    .detach_and_log_err(cx);
}

pub(crate) fn edit_prediction_accepted(
    store: &EditPredictionStore,
    current_prediction: CurrentEditPrediction,
    cx: &App,
) {
    let Some(api_token) = store
        .sweep_ai
        .api_token
        .read(cx)
        .key(&SWEEP_CREDENTIALS_URL)
    else {
        return;
    };
    let debug_info = store.sweep_ai.debug_info.clone();

    let prediction = current_prediction.prediction;

    let (additions, deletions) = compute_edit_metrics(&prediction.edits, &prediction.snapshot);
    let autocomplete_id = prediction.id.to_string();

    let device_id = store
        .client
        .user_id()
        .as_ref()
        .map(ToString::to_string)
        .unwrap_or_default();

    let suggestion_type = match current_prediction.shown_with {
        Some(SuggestionDisplayType::DiffPopover) => SweepSuggestionType::Popup,
        Some(SuggestionDisplayType::Jump) => return, // should'nt happen
        Some(SuggestionDisplayType::GhostText) | None => SweepSuggestionType::GhostText,
    };

    let request_body = AutocompleteMetricsRequest {
        event_type: SweepEventType::AutocompleteSuggestionAccepted,
        suggestion_type,
        additions,
        deletions,
        autocomplete_id,
        edit_tracking: String::new(),
        edit_tracking_line: None,
        lifespan: None,
        debug_info,
        device_id,
        privacy_mode_enabled: false,
    };

    send_autocomplete_metrics_request(cx, store.client.clone(), api_token, request_body);
}

pub fn edit_prediction_shown(
    sweep_ai: &SweepAi,
    client: Arc<Client>,
    prediction: &EditPrediction,
    display_type: SuggestionDisplayType,
    cx: &App,
) {
    let Some(api_token) = sweep_ai.api_token.read(cx).key(&SWEEP_CREDENTIALS_URL) else {
        return;
    };
    let debug_info = sweep_ai.debug_info.clone();

    let (additions, deletions) = compute_edit_metrics(&prediction.edits, &prediction.snapshot);
    let autocomplete_id = prediction.id.to_string();

    let suggestion_type = match display_type {
        SuggestionDisplayType::GhostText => SweepSuggestionType::GhostText,
        SuggestionDisplayType::DiffPopover => SweepSuggestionType::Popup,
        SuggestionDisplayType::Jump => SweepSuggestionType::JumpToEdit,
    };

    let request_body = AutocompleteMetricsRequest {
        event_type: SweepEventType::AutocompleteSuggestionShown,
        suggestion_type,
        additions,
        deletions,
        autocomplete_id,
        edit_tracking: String::new(),
        edit_tracking_line: None,
        lifespan: None,
        debug_info,
        device_id: String::new(),
        privacy_mode_enabled: false,
    };

    send_autocomplete_metrics_request(cx, client, api_token, request_body);
}

fn compute_edit_metrics(
    edits: &[(Range<Anchor>, Arc<str>)],
    snapshot: &BufferSnapshot,
) -> (u32, u32) {
    let mut additions = 0u32;
    let mut deletions = 0u32;

    for (range, new_text) in edits {
        let old_text = snapshot.text_for_range(range.clone());
        deletions += old_text
            .map(|chunk| chunk.lines().count())
            .sum::<usize>()
            .max(1) as u32;
        additions += new_text.lines().count().max(1) as u32;
    }

    (additions, deletions)
}
