pub mod bench_preview;
pub mod svg_stream;
pub mod typst_viewer_view;

use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use anyhow::{Context as _, Result};
use gpui::App;
use language::Buffer;
use lsp::{LanguageServer, LanguageServerId, LanguageServerName, Subscription};
use project::Project;
use serde::Deserialize;
use workspace::Workspace;

/// Track which LSP server IDs already have notification handlers registered,
/// so we don't panic on double-registration when opening multiple previews.
static REGISTERED_SERVERS: Mutex<Option<HashSet<LanguageServerId>>> = Mutex::new(None);

pub use zed_actions::preview::typst::{OpenPreview, OpenPreviewToTheSide};

pub const TINYMIST_SERVER_NAME: LanguageServerName = LanguageServerName::new_static("tinymist");

/// Response from tinymist's `doStartPreview` / `startPreview` command.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StartPreviewResponse {
    pub static_server_port: Option<u16>,
    pub static_server_addr: Option<String>,
    pub data_plane_port: Option<u16>,
    pub is_primary: bool,
}

/// tinymist sends document outline notifications that Zed doesn't consume.
/// Register a no-op handler to suppress "unhandled notification" log spam.
#[derive(Debug)]
enum DocumentOutline {}

impl lsp::notification::Notification for DocumentOutline {
    type Params = serde_json::Value;
    const METHOD: &'static str = "tinymist/documentOutline";
}

/// Register handlers for tinymist-specific notifications.
/// Returns a subscription that must be kept alive (stored or detached).
/// Safe to call multiple times for the same server — subsequent calls
/// return None instead of panicking on double-registration.
pub fn register_tinymist_notifications(server: &LanguageServer) -> Option<Subscription> {
    let server_id = server.server_id();
    let mut guard = REGISTERED_SERVERS.lock().unwrap_or_else(|e| e.into_inner());
    let set = guard.get_or_insert_with(HashSet::new);
    if !set.insert(server_id) {
        // Already registered for this server.
        return None;
    }
    Some(server.on_notification::<DocumentOutline, _>(|_params, _cx| {}))
}

pub fn init(cx: &mut App) {
    cx.observe_new(|workspace: &mut Workspace, window, cx| {
        let Some(window) = window else {
            return;
        };
        crate::typst_viewer_view::TypstPreviewView::register(workspace, window, cx);
    })
    .detach();
}

/// Find the tinymist language server for a given buffer.
///
/// Searches by buffer association first, then falls back to scanning all
/// running language servers for one named "tinymist" (mirrors the
/// rust-analyzer pattern).
pub fn find_tinymist_server(
    project: &Project,
    buffer: Option<&Buffer>,
    cx: &App,
) -> Option<LanguageServerId> {
    buffer
        .and_then(|buffer| project.language_server_id_for_name(buffer, &TINYMIST_SERVER_NAME, cx))
        .or_else(|| {
            let servers: Vec<_> = project
                .language_server_statuses(cx)
                .filter_map(|(server_id, status)| {
                    if status.name == TINYMIST_SERVER_NAME {
                        Some(server_id)
                    } else {
                        None
                    }
                })
                .collect();
            if servers.len() == 1 {
                servers.first().copied()
            } else {
                None
            }
        })
}

/// Send `tinymist.doStartPreview` to the language server and return the
/// WebSocket URL for the data plane.
///
/// The command accepts CLI-style arguments as a JSON array of strings.
/// We pass `["--server-svg", "--strip-svg-glyph-defs", "--data-plane-host=127.0.0.1:0"]` so that:
/// - tinymist renders complete SVG server-side
/// - unchanged glyph defs are stripped after the first frame (~200KB vs ~2MB)
/// - the data plane binds to an OS-assigned port
pub async fn start_preview_via_lsp(
    server: Arc<LanguageServer>,
    entry_path: &std::path::Path,
    request_timeout: Duration,
) -> Result<String> {
    let entry_str = entry_path
        .to_str()
        .context("entry file path is not valid UTF-8")?;

    // Use the file path as the task ID so each file gets its own preview
    // and we can kill/restart a specific one without affecting others.
    let task_id = entry_str.to_string();

    // Kill any existing preview for this file — tinymist only allows one
    // preview per task ID, and the previous one may not have been cleaned
    // up (e.g. the pane was closed without sending doKillPreview).
    let kill_params = lsp::ExecuteCommandParams {
        command: "tinymist.doKillPreview".into(),
        arguments: vec![serde_json::json!(task_id)],
        ..Default::default()
    };
    log::info!("typst_viewer: sending tinymist.doKillPreview for {task_id}");
    let _ = server
        .request::<lsp::request::ExecuteCommand>(kill_params, request_timeout)
        .await;

    let args: Vec<String> = vec![
        "--server-svg".into(),
        "--strip-svg-glyph-defs".into(),
        "--data-plane-host=127.0.0.1:0".into(),
        format!("--task-id={task_id}"),
        entry_str.into(),
    ];

    let params = lsp::ExecuteCommandParams {
        command: "tinymist.doStartPreview".into(),
        arguments: vec![serde_json::to_value(args)?],
        ..Default::default()
    };

    log::info!("typst_viewer: sending tinymist.doStartPreview");

    let result = server
        .request::<lsp::request::ExecuteCommand>(params, request_timeout)
        .await
        .into_response()
        .context("tinymist.doStartPreview request failed")?;

    let response: StartPreviewResponse = serde_json::from_value(
        result.context("tinymist.doStartPreview returned null")?,
    )
    .context("failed to parse StartPreviewResponse")?;

    log::info!("typst_viewer: StartPreviewResponse: {response:?}");

    let port = response
        .data_plane_port
        .context("StartPreviewResponse missing data_plane_port")?;

    Ok(format!("ws://127.0.0.1:{port}"))
}