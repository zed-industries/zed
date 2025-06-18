use ::serde::{Deserialize, Serialize};
use anyhow::Context as _;
use gpui::{App, Entity, SharedString, Task, WeakEntity};
use language::{LanguageServerStatusUpdate, ServerHealth};
use lsp::LanguageServer;
use rpc::proto;

use crate::{LspStore, Project, ProjectPath, lsp_store};

pub const RUST_ANALYZER_NAME: &str = "rust-analyzer";
pub const CARGO_DIAGNOSTICS_SOURCE_NAME: &str = "rustc";

/// Experimental: Informs the end user about the state of the server
///
/// [Rust Analyzer Specification](https://rust-analyzer.github.io/book/contributing/lsp-extensions.html#server-status)
#[derive(Debug)]
enum ServerStatus {}

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
struct ServerStatusParams {
    pub health: ServerHealth,
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
            let name = name.clone();
            move |params, cx| {
                let status = params.message;
                let log_message =
                    format!("Language server {name} (id {server_id}) status update: {status:?}");
                match &params.health {
                    ServerHealth::Ok => log::info!("{log_message}"),
                    ServerHealth::Warning => log::warn!("{log_message}"),
                    ServerHealth::Error => log::error!("{log_message}"),
                }

                lsp_store
                    .update(cx, |lsp_store, _| {
                        lsp_store.languages.update_lsp_status(
                            name.clone(),
                            LanguageServerStatusUpdate::Health(
                                params.health,
                                status.map(SharedString::from),
                            ),
                        );
                    })
                    .ok();
            }
        })
        .detach();
}

pub fn cancel_flycheck(
    project: Entity<Project>,
    buffer_path: ProjectPath,
    cx: &mut App,
) -> Task<anyhow::Result<()>> {
    let upstream_client = project.read(cx).lsp_store().read(cx).upstream_client();
    let lsp_store = project.read(cx).lsp_store();
    let buffer = project.update(cx, |project, cx| {
        project.buffer_store().update(cx, |buffer_store, cx| {
            buffer_store.open_buffer(buffer_path, cx)
        })
    });

    cx.spawn(async move |cx| {
        let buffer = buffer.await?;
        let Some(rust_analyzer_server) = project
            .update(cx, |project, cx| {
                buffer.update(cx, |buffer, cx| {
                    project.language_server_id_for_name(buffer, RUST_ANALYZER_NAME, cx)
                })
            })?
            .await
        else {
            return Ok(());
        };
        let buffer_id = buffer.read_with(cx, |buffer, _| buffer.remote_id().to_proto())?;

        if let Some((client, project_id)) = upstream_client {
            let request = proto::LspExtCancelFlycheck {
                project_id,
                buffer_id,
                language_server_id: rust_analyzer_server.to_proto(),
            };
            client
                .request(request)
                .await
                .context("lsp ext cancel flycheck proto request")?;
        } else {
            lsp_store
                .read_with(cx, |lsp_store, _| {
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
    project: Entity<Project>,
    buffer_path: ProjectPath,
    cx: &mut App,
) -> Task<anyhow::Result<()>> {
    let upstream_client = project.read(cx).lsp_store().read(cx).upstream_client();
    let lsp_store = project.read(cx).lsp_store();
    let buffer = project.update(cx, |project, cx| {
        project.buffer_store().update(cx, |buffer_store, cx| {
            buffer_store.open_buffer(buffer_path, cx)
        })
    });

    cx.spawn(async move |cx| {
        let buffer = buffer.await?;
        let Some(rust_analyzer_server) = project
            .update(cx, |project, cx| {
                buffer.update(cx, |buffer, cx| {
                    project.language_server_id_for_name(buffer, RUST_ANALYZER_NAME, cx)
                })
            })?
            .await
        else {
            return Ok(());
        };
        let buffer_id = buffer.read_with(cx, |buffer, _| buffer.remote_id().to_proto())?;

        if let Some((client, project_id)) = upstream_client {
            let request = proto::LspExtRunFlycheck {
                project_id,
                buffer_id,
                language_server_id: rust_analyzer_server.to_proto(),
                current_file_only: false,
            };
            client
                .request(request)
                .await
                .context("lsp ext run flycheck proto request")?;
        } else {
            lsp_store
                .read_with(cx, |lsp_store, _| {
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
    project: Entity<Project>,
    buffer_path: ProjectPath,
    cx: &mut App,
) -> Task<anyhow::Result<()>> {
    let upstream_client = project.read(cx).lsp_store().read(cx).upstream_client();
    let lsp_store = project.read(cx).lsp_store();
    let buffer = project.update(cx, |project, cx| {
        project.buffer_store().update(cx, |buffer_store, cx| {
            buffer_store.open_buffer(buffer_path, cx)
        })
    });

    cx.spawn(async move |cx| {
        let buffer = buffer.await?;
        let Some(rust_analyzer_server) = project
            .update(cx, |project, cx| {
                buffer.update(cx, |buffer, cx| {
                    project.language_server_id_for_name(buffer, RUST_ANALYZER_NAME, cx)
                })
            })?
            .await
        else {
            return Ok(());
        };
        let buffer_id = buffer.read_with(cx, |buffer, _| buffer.remote_id().to_proto())?;

        if let Some((client, project_id)) = upstream_client {
            let request = proto::LspExtClearFlycheck {
                project_id,
                buffer_id,
                language_server_id: rust_analyzer_server.to_proto(),
            };
            client
                .request(request)
                .await
                .context("lsp ext clear flycheck proto request")?;
        } else {
            lsp_store
                .read_with(cx, |lsp_store, _| {
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
