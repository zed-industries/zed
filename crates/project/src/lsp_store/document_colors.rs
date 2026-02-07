use std::{sync::Arc, time::Duration};

use anyhow::{Context as _, Result};
use clock::Global;
use collections::{HashMap, HashSet};
use futures::{
    FutureExt as _,
    future::{Shared, join_all},
};
use gpui::{AppContext as _, AsyncApp, Context, Entity, SharedString, Task};
use language::{
    Buffer, LocalFile as _, PointUtf16, point_to_lsp,
    proto::{deserialize_lsp_edit, serialize_lsp_edit},
};
use lsp::LanguageServerId;
use rpc::{TypedEnvelope, proto};
use settings::Settings as _;
use text::BufferId;
use util::ResultExt as _;
use worktree::File;

use crate::{
    ColorPresentation, DocumentColor, LspStore,
    lsp_command::{GetDocumentColor, LspCommand as _, make_text_document_identifier},
    project_settings::ProjectSettings,
};

#[derive(Debug, Default, Clone)]
pub struct DocumentColors {
    pub colors: HashSet<DocumentColor>,
}

pub(super) type DocumentColorTask =
    Shared<Task<std::result::Result<DocumentColors, Arc<anyhow::Error>>>>;

#[derive(Debug, Default)]
pub(super) struct DocumentColorData {
    pub(super) colors: HashMap<LanguageServerId, HashSet<DocumentColor>>,
    pub(super) colors_update: Option<(Global, DocumentColorTask)>,
}

impl DocumentColorData {
    pub(super) fn remove_server_data(&mut self, server_id: LanguageServerId) {
        self.colors.remove(&server_id);
    }
}

impl LspStore {
    pub fn document_colors(
        &mut self,
        buffer: Entity<Buffer>,
        cx: &mut Context<Self>,
    ) -> Option<DocumentColorTask> {
        let version_queried_for = buffer.read(cx).version();
        let buffer_id = buffer.read(cx).remote_id();

        let current_language_servers = self.as_local().map(|local| {
            local
                .buffers_opened_in_servers
                .get(&buffer_id)
                .cloned()
                .unwrap_or_default()
        });

        if let Some(lsp_data) = self.current_lsp_data(buffer_id) {
            if let Some(cached_colors) = &lsp_data.document_colors {
                if !version_queried_for.changed_since(&lsp_data.buffer_version) {
                    let has_different_servers =
                        current_language_servers.is_some_and(|current_language_servers| {
                            current_language_servers
                                != cached_colors.colors.keys().copied().collect()
                        });
                    if !has_different_servers {
                        return Some(
                            Task::ready(Ok(DocumentColors {
                                colors: cached_colors.colors.values().flatten().cloned().collect(),
                            }))
                            .shared(),
                        );
                    }
                }
            }
        }

        let color_lsp_data = self
            .latest_lsp_data(&buffer, cx)
            .document_colors
            .get_or_insert_default();
        if let Some((updating_for, running_update)) = &color_lsp_data.colors_update
            && !version_queried_for.changed_since(updating_for)
        {
            return Some(running_update.clone());
        }
        let buffer_version_queried_for = version_queried_for.clone();
        let new_task = cx
            .spawn(async move |lsp_store, cx| {
                cx.background_executor()
                    .timer(Duration::from_millis(30))
                    .await;
                let fetched_colors = lsp_store
                    .update(cx, |lsp_store, cx| {
                        lsp_store.fetch_document_colors_for_buffer(&buffer, cx)
                    })?
                    .await
                    .context("fetching document colors")
                    .map_err(Arc::new);
                let fetched_colors = match fetched_colors {
                    Ok(fetched_colors) => {
                        if buffer.update(cx, |buffer, _| {
                            buffer.version() != buffer_version_queried_for
                        }) {
                            return Ok(DocumentColors::default());
                        }
                        fetched_colors
                    }
                    Err(e) => {
                        lsp_store
                            .update(cx, |lsp_store, _| {
                                if let Some(lsp_data) = lsp_store.lsp_data.get_mut(&buffer_id) {
                                    if let Some(document_colors) = &mut lsp_data.document_colors {
                                        document_colors.colors_update = None;
                                    }
                                }
                            })
                            .ok();
                        return Err(e);
                    }
                };

                lsp_store
                    .update(cx, |lsp_store, cx| {
                        let lsp_data = lsp_store.latest_lsp_data(&buffer, cx);
                        let lsp_colors = lsp_data.document_colors.get_or_insert_default();

                        if let Some(fetched_colors) = fetched_colors {
                            if lsp_data.buffer_version == buffer_version_queried_for {
                                lsp_colors.colors.extend(fetched_colors);
                            } else if !lsp_data
                                .buffer_version
                                .changed_since(&buffer_version_queried_for)
                            {
                                lsp_data.buffer_version = buffer_version_queried_for;
                                lsp_colors.colors = fetched_colors;
                            }
                        }
                        lsp_colors.colors_update = None;
                        let colors = lsp_colors
                            .colors
                            .values()
                            .flatten()
                            .cloned()
                            .collect::<HashSet<_>>();
                        DocumentColors { colors }
                    })
                    .map_err(Arc::new)
            })
            .shared();
        color_lsp_data.colors_update = Some((version_queried_for, new_task.clone()));
        Some(new_task)
    }

    pub fn resolve_color_presentation(
        &mut self,
        mut color: DocumentColor,
        buffer: Entity<Buffer>,
        server_id: LanguageServerId,
        cx: &mut Context<Self>,
    ) -> Task<Result<DocumentColor>> {
        if color.resolved {
            return Task::ready(Ok(color));
        }

        if let Some((upstream_client, project_id)) = self.upstream_client() {
            let start = color.lsp_range.start;
            let end = color.lsp_range.end;
            let request = proto::GetColorPresentation {
                project_id,
                server_id: server_id.to_proto(),
                buffer_id: buffer.read(cx).remote_id().into(),
                color: Some(proto::ColorInformation {
                    red: color.color.red,
                    green: color.color.green,
                    blue: color.color.blue,
                    alpha: color.color.alpha,
                    lsp_range_start: Some(proto::PointUtf16 {
                        row: start.line,
                        column: start.character,
                    }),
                    lsp_range_end: Some(proto::PointUtf16 {
                        row: end.line,
                        column: end.character,
                    }),
                }),
            };
            cx.background_spawn(async move {
                let response = upstream_client
                    .request(request)
                    .await
                    .context("color presentation proto request")?;
                color.resolved = true;
                color.color_presentations = response
                    .presentations
                    .into_iter()
                    .map(|presentation| ColorPresentation {
                        label: SharedString::from(presentation.label),
                        text_edit: presentation.text_edit.and_then(deserialize_lsp_edit),
                        additional_text_edits: presentation
                            .additional_text_edits
                            .into_iter()
                            .filter_map(deserialize_lsp_edit)
                            .collect(),
                    })
                    .collect();
                Ok(color)
            })
        } else {
            let path = match buffer
                .update(cx, |buffer, cx| {
                    Some(File::from_dyn(buffer.file())?.abs_path(cx))
                })
                .context("buffer with the missing path")
            {
                Ok(path) => path,
                Err(e) => return Task::ready(Err(e)),
            };
            let Some(lang_server) = buffer.update(cx, |buffer, cx| {
                self.language_server_for_local_buffer(buffer, server_id, cx)
                    .map(|(_, server)| server.clone())
            }) else {
                return Task::ready(Ok(color));
            };

            let request_timeout = ProjectSettings::get_global(cx)
                .global_lsp_settings
                .get_request_timeout();
            cx.background_spawn(async move {
                let resolve_task = lang_server.request::<lsp::request::ColorPresentationRequest>(
                    lsp::ColorPresentationParams {
                        text_document: make_text_document_identifier(&path)?,
                        color: color.color,
                        range: color.lsp_range,
                        work_done_progress_params: Default::default(),
                        partial_result_params: Default::default(),
                    },
                    request_timeout,
                );
                color.color_presentations = resolve_task
                    .await
                    .into_response()
                    .context("color presentation resolve LSP request")?
                    .into_iter()
                    .map(|presentation| ColorPresentation {
                        label: SharedString::from(presentation.label),
                        text_edit: presentation.text_edit,
                        additional_text_edits: presentation
                            .additional_text_edits
                            .unwrap_or_default(),
                    })
                    .collect();
                color.resolved = true;
                Ok(color)
            })
        }
    }

    pub(super) fn fetch_document_colors_for_buffer(
        &mut self,
        buffer: &Entity<Buffer>,
        cx: &mut Context<Self>,
    ) -> Task<anyhow::Result<Option<HashMap<LanguageServerId, HashSet<DocumentColor>>>>> {
        if let Some((client, project_id)) = self.upstream_client() {
            let request = GetDocumentColor {};
            if !self.is_capable_for_proto_request(buffer, &request, cx) {
                return Task::ready(Ok(None));
            }

            let request_timeout = ProjectSettings::get_global(cx)
                .global_lsp_settings
                .get_request_timeout();
            let request_task = client.request_lsp(
                project_id,
                None,
                request_timeout,
                cx.background_executor().clone(),
                request.to_proto(project_id, buffer.read(cx)),
            );
            let buffer = buffer.clone();
            cx.spawn(async move |lsp_store, cx| {
                let Some(lsp_store) = lsp_store.upgrade() else {
                    return Ok(None);
                };
                let colors: HashMap<LanguageServerId, HashSet<DocumentColor>> = join_all(
                    request_task
                        .await
                        .log_err()
                        .flatten()
                        .map(|response| response.payload)
                        .unwrap_or_default()
                        .into_iter()
                        .map(|color_response| {
                            let response = request.response_from_proto(
                                color_response.response,
                                lsp_store.clone(),
                                buffer.clone(),
                                cx.clone(),
                            );
                            async move {
                                let colors: Vec<DocumentColor> =
                                    response.await.log_err().unwrap_or_default();
                                (
                                    LanguageServerId::from_proto(color_response.server_id),
                                    colors,
                                )
                            }
                        }),
                )
                .await
                .into_iter()
                .fold(HashMap::default(), |mut acc, (server_id, colors)| {
                    acc.entry(server_id)
                        .or_insert_with(HashSet::default)
                        .extend(colors);
                    acc
                });
                Ok(Some(colors))
            })
        } else {
            let document_colors_task =
                self.request_multiple_lsp_locally(buffer, None::<usize>, GetDocumentColor, cx);
            cx.background_spawn(async move {
                Ok(Some(
                    document_colors_task
                        .await
                        .into_iter()
                        .fold(HashMap::default(), |mut acc, (server_id, colors)| {
                            acc.entry(server_id)
                                .or_insert_with(HashSet::default)
                                .extend(colors);
                            acc
                        })
                        .into_iter()
                        .collect(),
                ))
            })
        }
    }

    pub(super) async fn handle_get_color_presentation(
        lsp_store: Entity<Self>,
        envelope: TypedEnvelope<proto::GetColorPresentation>,
        mut cx: AsyncApp,
    ) -> Result<proto::GetColorPresentationResponse> {
        let buffer_id = BufferId::new(envelope.payload.buffer_id)?;
        let buffer = lsp_store.update(&mut cx, |lsp_store, cx| {
            lsp_store.buffer_store.read(cx).get_existing(buffer_id)
        })?;

        let color = envelope
            .payload
            .color
            .context("invalid color resolve request")?;
        let start = color
            .lsp_range_start
            .context("invalid color resolve request")?;
        let end = color
            .lsp_range_end
            .context("invalid color resolve request")?;

        let color = DocumentColor {
            lsp_range: lsp::Range {
                start: point_to_lsp(PointUtf16::new(start.row, start.column)),
                end: point_to_lsp(PointUtf16::new(end.row, end.column)),
            },
            color: lsp::Color {
                red: color.red,
                green: color.green,
                blue: color.blue,
                alpha: color.alpha,
            },
            resolved: false,
            color_presentations: Vec::new(),
        };
        let resolved_color = lsp_store
            .update(&mut cx, |lsp_store, cx| {
                lsp_store.resolve_color_presentation(
                    color,
                    buffer.clone(),
                    LanguageServerId(envelope.payload.server_id as usize),
                    cx,
                )
            })
            .await
            .context("resolving color presentation")?;

        Ok(proto::GetColorPresentationResponse {
            presentations: resolved_color
                .color_presentations
                .into_iter()
                .map(|presentation| proto::ColorPresentation {
                    label: presentation.label.to_string(),
                    text_edit: presentation.text_edit.map(serialize_lsp_edit),
                    additional_text_edits: presentation
                        .additional_text_edits
                        .into_iter()
                        .map(serialize_lsp_edit)
                        .collect(),
                })
                .collect(),
        })
    }
}
