use anyhow::Result;
use futures::AsyncReadExt as _;
use gpui::{
    App, AppContext as _, Entity, SharedString, Task,
    http_client::{self, AsyncBody, Method},
};
use language::{Point, ToOffset as _};
use language_model::{ApiKeyState, EnvVar, env_var};
use lsp::DiagnosticSeverity;
use serde::{Deserialize, Serialize};
use std::{
    fmt::{self, Write as _},
    path::Path,
    sync::Arc,
    time::Instant,
};

use crate::{EditPredictionId, EditPredictionModelInput, prediction::EditPredictionResult};

const SWEEP_API_URL: &str = "https://autocomplete.sweep.dev/backend/next_edit_autocomplete";

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

            let mut file_chunks = recent_buffer_snapshots
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

            let retrieval_chunks = inputs
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
                    "{} at line {}: {}",
                    severity,
                    start_point.row + 1,
                    entry.diagnostic.message
                )?;
            }

            if !diagnostic_content.is_empty() {
                file_chunks.push(FileChunk {
                    file_path: format!("Diagnostics for {}", full_path.display()),
                    start_line: 0,
                    end_line: diagnostic_count,
                    content: diagnostic_content,
                    timestamp: None,
                });
            }

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
                recent_user_actions: vec![],
                use_bytes: true,
                // TODO
                privacy_mode_enabled: false,
            };

            let mut buf: Vec<u8> = Vec::new();
            let writer = brotli::CompressorWriter::new(&mut buf, 4096, 11, 22);
            serde_json::to_writer(writer, &request_body)?;
            let body: AsyncBody = buf.into();

            let ep_inputs = zeta_prompt::ZetaPromptInput {
                events: inputs.events,
                related_files: inputs.related_files.clone(),
                cursor_path: full_path.clone(),
                cursor_excerpt: request_body.file_contents.into(),
                // we actually don't know
                editable_range_in_excerpt: 0..inputs.snapshot.len(),
                cursor_offset_in_excerpt: request_body.cursor_position,
            };

            let request = http_client::Request::builder()
                .uri(SWEEP_API_URL)
                .header("Content-Type", "application/json")
                .header("Authorization", format!("Bearer {}", api_token))
                .header("Connection", "keep-alive")
                .header("Content-Encoding", "br")
                .method(Method::POST)
                .body(body)?;

            let mut response = http_client.send(request).await?;

            let mut body: Vec<u8> = Vec::new();
            response.body_mut().read_to_end(&mut body).await?;

            let response_received_at = Instant::now();
            if !response.status().is_success() {
                anyhow::bail!(
                    "Request failed with status: {:?}\nBody: {}",
                    response.status(),
                    String::from_utf8_lossy(&body),
                );
            };

            let response: AutocompleteResponse = serde_json::from_slice(&body)?;

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

pub const SWEEP_CREDENTIALS_URL: SharedString =
    SharedString::new_static("https://autocomplete.sweep.dev");
pub const SWEEP_CREDENTIALS_USERNAME: &str = "sweep-api-token";
pub static SWEEP_AI_TOKEN_ENV_VAR: std::sync::LazyLock<EnvVar> = env_var!("SWEEP_AI_TOKEN");
pub static SWEEP_API_KEY: std::sync::OnceLock<Entity<ApiKeyState>> = std::sync::OnceLock::new();

pub fn sweep_api_token(cx: &mut App) -> Entity<ApiKeyState> {
    SWEEP_API_KEY
        .get_or_init(|| {
            cx.new(|_| ApiKeyState::new(SWEEP_CREDENTIALS_URL, SWEEP_AI_TOKEN_ENV_VAR.clone()))
        })
        .clone()
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

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
enum ActionType {
    CursorMovement,
    InsertChar,
    DeleteChar,
    InsertSelection,
    DeleteSelection,
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
