use ::serde::{Deserialize, Serialize};
use anyhow::Context as _;
use gpui::{App, AsyncApp, Entity, Task, WeakEntity};
use language::{Buffer, ServerHealth};
use lsp::{LanguageServer, LanguageServerId, LanguageServerName};
use rpc::proto;

use crate::{LspStore, LspStoreEvent, Project, ProjectPath, lsp_store};

pub const RUST_ANALYZER_NAME: LanguageServerName = LanguageServerName::new_static("rust-analyzer");
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
            move |params, cx| {
                let message = params.message;
                let log_message = message.as_ref().map(|message| {
                    format!("Language server {name} (id {server_id}) status update: {message}")
                });
                let status = match &params.health {
                    ServerHealth::Ok => {
                        if let Some(log_message) = log_message {
                            log::info!("{log_message}");
                        }
                        proto::ServerHealth::Ok
                    }
                    ServerHealth::Warning => {
                        if let Some(log_message) = log_message {
                            log::warn!("{log_message}");
                        }
                        proto::ServerHealth::Warning
                    }
                    ServerHealth::Error => {
                        if let Some(log_message) = log_message {
                            log::error!("{log_message}");
                        }
                        proto::ServerHealth::Error
                    }
                };

                lsp_store
                    .update(cx, |_, cx| {
                        cx.emit(LspStoreEvent::LanguageServerUpdate {
                            language_server_id: server_id,
                            name: Some(name.clone()),
                            message: proto::update_language_server::Variant::StatusUpdate(
                                proto::StatusUpdate {
                                    message,
                                    status: Some(proto::status_update::Status::Health(
                                        status as i32,
                                    )),
                                },
                            ),
                        });
                    })
                    .ok();
            }
        })
        .detach();
}

pub fn cancel_flycheck(
    project: Entity<Project>,
    buffer_path: Option<ProjectPath>,
    cx: &mut App,
) -> Task<anyhow::Result<()>> {
    let upstream_client = project.read(cx).lsp_store().read(cx).upstream_client();
    let lsp_store = project.read(cx).lsp_store();
    let buffer = buffer_path.map(|buffer_path| {
        project.update(cx, |project, cx| {
            project.buffer_store().update(cx, |buffer_store, cx| {
                buffer_store.open_buffer(buffer_path, cx)
            })
        })
    });

    cx.spawn(async move |cx| {
        let buffer = match buffer {
            Some(buffer) => Some(buffer.await?),
            None => None,
        };
        let Some(rust_analyzer_server) = find_rust_analyzer_server(&project, buffer.as_ref(), cx)
        else {
            return Ok(());
        };

        if let Some((client, project_id)) = upstream_client {
            let request = proto::LspExtCancelFlycheck {
                project_id,
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
                        server.notify::<lsp_store::lsp_ext_command::LspExtCancelFlycheck>(())
                    } else {
                        Ok(())
                    }
                })
                .context("lsp ext cancel flycheck")??;
        };
        anyhow::Ok(())
    })
}

pub fn run_flycheck(
    project: Entity<Project>,
    buffer_path: Option<ProjectPath>,
    cx: &mut App,
) -> Task<anyhow::Result<()>> {
    let upstream_client = project.read(cx).lsp_store().read(cx).upstream_client();
    let lsp_store = project.read(cx).lsp_store();
    let buffer = buffer_path.map(|buffer_path| {
        project.update(cx, |project, cx| {
            project.buffer_store().update(cx, |buffer_store, cx| {
                buffer_store.open_buffer(buffer_path, cx)
            })
        })
    });

    cx.spawn(async move |cx| {
        let buffer = match buffer {
            Some(buffer) => Some(buffer.await?),
            None => None,
        };
        let Some(rust_analyzer_server) = find_rust_analyzer_server(&project, buffer.as_ref(), cx)
        else {
            return Ok(());
        };

        if let Some((client, project_id)) = upstream_client {
            let buffer_id = buffer
                .map(|buffer| buffer.read_with(cx, |buffer, _| buffer.remote_id().to_proto()))
                .transpose()?;
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
                            lsp_store::lsp_ext_command::RunFlycheckParams {
                                text_document: None,
                            },
                        )
                    } else {
                        Ok(())
                    }
                })
                .context("lsp ext run flycheck")??;
        };
        anyhow::Ok(())
    })
}

pub fn clear_flycheck(
    project: Entity<Project>,
    buffer_path: Option<ProjectPath>,
    cx: &mut App,
) -> Task<anyhow::Result<()>> {
    let upstream_client = project.read(cx).lsp_store().read(cx).upstream_client();
    let lsp_store = project.read(cx).lsp_store();
    let buffer = buffer_path.map(|buffer_path| {
        project.update(cx, |project, cx| {
            project.buffer_store().update(cx, |buffer_store, cx| {
                buffer_store.open_buffer(buffer_path, cx)
            })
        })
    });

    cx.spawn(async move |cx| {
        let buffer = match buffer {
            Some(buffer) => Some(buffer.await?),
            None => None,
        };
        let Some(rust_analyzer_server) = find_rust_analyzer_server(&project, buffer.as_ref(), cx)
        else {
            return Ok(());
        };

        if let Some((client, project_id)) = upstream_client {
            let request = proto::LspExtClearFlycheck {
                project_id,
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
                        server.notify::<lsp_store::lsp_ext_command::LspExtClearFlycheck>(())
                    } else {
                        Ok(())
                    }
                })
                .context("lsp ext clear flycheck")??;
        };
        anyhow::Ok(())
    })
}

fn find_rust_analyzer_server(
    project: &Entity<Project>,
    buffer: Option<&Entity<Buffer>>,
    cx: &mut AsyncApp,
) -> Option<LanguageServerId> {
    project
        .read_with(cx, |project, cx| {
            buffer
                .and_then(|buffer| {
                    project.language_server_id_for_name(buffer.read(cx), &RUST_ANALYZER_NAME, cx)
                })
                // If no rust-analyzer found for the current buffer (e.g. `settings.json`), fall back to the project lookup
                // and use project's rust-analyzer if it's the only one.
                .or_else(|| {
                    let rust_analyzer_servers = project
                        .lsp_store()
                        .read(cx)
                        .language_server_statuses
                        .iter()
                        .filter_map(|(server_id, server_status)| {
                            if server_status.name == RUST_ANALYZER_NAME {
                                Some(*server_id)
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<_>>();
                    if rust_analyzer_servers.len() == 1 {
                        rust_analyzer_servers.first().copied()
                    } else {
                        None
                    }
                })
        })
        .ok()?
}
