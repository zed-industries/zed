use ::serde::{Deserialize, Serialize};
use anyhow::Context as _;
use gpui::{App, Entity, PromptLevel, Task, WeakEntity};
use lsp::LanguageServer;
use rpc::proto;
use text::BufferId;

use crate::{LanguageServerPromptRequest, LspStore, LspStoreEvent, Project, lsp_store};

pub const RUST_ANALYZER_NAME: &str = "rust-analyzer";
pub const CARGO_DIAGNOSTICS_SOURCE_NAME: &str = "rustc";

/// Experimental: Informs the end user about the state of the server
///
/// [Rust Analyzer Specification](https://rust-analyzer.github.io/book/contributing/lsp-extensions.html#server-status)
#[derive(Debug)]
enum ServerStatus {}

/// Other(String) variant to handle unknown values due to this still being experimental
#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
enum ServerHealthStatus {
    Ok,
    Warning,
    Error,
    Other(String),
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
struct ServerStatusParams {
    pub health: ServerHealthStatus,
    pub message: Option<String>,
}

impl lsp::notification::Notification for ServerStatus {
    type Params = ServerStatusParams;
    const METHOD: &'static str = "experimental/serverStatus";
}

pub fn register_notifications(lsp_store: WeakEntity<LspStore>, language_server: &LanguageServer) {
    let name = language_server.name();
    let server_id = language_server.server_id();

    language_server
        .on_notification::<ServerStatus, _>({
            let name = name.to_string();
            move |params, cx| {
                let name = name.to_string();
                if let Some(ref message) = params.message {
                    let message = message.trim();
                    if !message.is_empty() {
                        let formatted_message = format!(
                            "Language server {name} (id {server_id}) status update: {message}"
                        );
                        match params.health {
                            ServerHealthStatus::Ok => log::info!("{formatted_message}"),
                            ServerHealthStatus::Warning => log::warn!("{formatted_message}"),
                            ServerHealthStatus::Error => {
                                log::error!("{formatted_message}");
                                let (tx, _rx) = smol::channel::bounded(1);
                                let request = LanguageServerPromptRequest {
                                    level: PromptLevel::Critical,
                                    message: params.message.unwrap_or_default(),
                                    actions: Vec::new(),
                                    response_channel: tx,
                                    lsp_name: name.clone(),
                                };
                                lsp_store
                                    .update(cx, |_, cx| {
                                        cx.emit(LspStoreEvent::LanguageServerPrompt(request));
                                    })
                                    .ok();
                            }
                            ServerHealthStatus::Other(status) => {
                                log::info!("Unknown server health: {status}\n{formatted_message}")
                            }
                        }
                    }
                }
            }
        })
        .detach();
}

pub fn cancel_flycheck(
    project: &Entity<Project>,
    buffer_id: BufferId,
    cx: &App,
) -> Task<anyhow::Result<()>> {
    let rust_analyzer_server = project
        .read(cx)
        .language_server_with_name(RUST_ANALYZER_NAME, cx);
    let upstream_client = project.read(cx).lsp_store().read(cx).upstream_client();
    let lsp_store = project.read(cx).lsp_store();

    cx.spawn(async move |cx| {
        let Some(rust_analyzer_server) = rust_analyzer_server.await else {
            return Ok(());
        };

        if let Some((client, project_id)) = upstream_client {
            let request = proto::LspExtCancelFlycheck {
                project_id,
                buffer_id: buffer_id.to_proto(),
                language_server_id: rust_analyzer_server.to_proto(),
            };
            client
                .request(request)
                .await
                .context("lsp ext cancel flycheck proto request")?;
        } else {
            lsp_store
                .update(cx, |lsp_store, _| {
                    if let Some(server) = lsp_store.language_server_for_id(rust_analyzer_server) {
                        server.notify::<lsp_store::lsp_ext_command::LspExtCancelFlycheck>(&())?;
                    }
                    anyhow::Ok(())
                })?
                .context("lsp ext cancel flycheck")?;
        };
        anyhow::Ok(())
    })
}

pub fn run_flycheck(
    project: &Entity<Project>,
    buffer_id: BufferId,
    cx: &App,
) -> Task<anyhow::Result<()>> {
    let rust_analyzer_server = project
        .read(cx)
        .language_server_with_name(RUST_ANALYZER_NAME, cx);
    let upstream_client = project.read(cx).lsp_store().read(cx).upstream_client();
    let lsp_store = project.read(cx).lsp_store();

    cx.spawn(async move |cx| {
        let Some(rust_analyzer_server) = rust_analyzer_server.await else {
            return Ok(());
        };

        if let Some((client, project_id)) = upstream_client {
            let request = proto::LspExtRunFlycheck {
                project_id,
                buffer_id: buffer_id.to_proto(),
                language_server_id: rust_analyzer_server.to_proto(),
                current_file_only: false,
            };
            client
                .request(request)
                .await
                .context("lsp ext run flycheck proto request")?;
        } else {
            lsp_store
                .update(cx, |lsp_store, _| {
                    if let Some(server) = lsp_store.language_server_for_id(rust_analyzer_server) {
                        server.notify::<lsp_store::lsp_ext_command::LspExtRunFlycheck>(
                            &lsp_store::lsp_ext_command::RunFlycheckParams {
                                text_document: None,
                            },
                        )?;
                    }
                    anyhow::Ok(())
                })?
                .context("lsp ext run flycheck")?;
        };
        anyhow::Ok(())
    })
}

pub fn clear_flycheck(
    project: &Entity<Project>,
    buffer_id: BufferId,
    cx: &App,
) -> Task<anyhow::Result<()>> {
    let rust_analyzer_server = project
        .read(cx)
        .language_server_with_name(RUST_ANALYZER_NAME, cx);
    let upstream_client = project.read(cx).lsp_store().read(cx).upstream_client();
    let lsp_store = project.read(cx).lsp_store();

    cx.spawn(async move |cx| {
        let Some(rust_analyzer_server) = rust_analyzer_server.await else {
            return Ok(());
        };

        if let Some((client, project_id)) = upstream_client {
            let request = proto::LspExtClearFlycheck {
                project_id,
                buffer_id: buffer_id.to_proto(),
                language_server_id: rust_analyzer_server.to_proto(),
            };
            client
                .request(request)
                .await
                .context("lsp ext clear flycheck proto request")?;
        } else {
            lsp_store
                .update(cx, |lsp_store, _| {
                    if let Some(server) = lsp_store.language_server_for_id(rust_analyzer_server) {
                        server.notify::<lsp_store::lsp_ext_command::LspExtClearFlycheck>(&())?;
                    }
                    anyhow::Ok(())
                })?
                .context("lsp ext clear flycheck")?;
        };
        anyhow::Ok(())
    })
}
