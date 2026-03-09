use std::sync::Arc;

use anyhow::{Context as _, Result};
use clock::Global;
use collections::HashMap;
use futures::{
    FutureExt as _,
    future::{Shared, join_all},
};
use gpui::{AppContext as _, AsyncApp, Context, Entity, Task};
use language::Buffer;
use lsp::LanguageServerId;
use rpc::{TypedEnvelope, proto};
use settings::Settings as _;
use std::time::Duration;

use crate::{
    CodeAction, LspStore, LspStoreEvent,
    lsp_command::{GetCodeLens, LspCommand as _},
    project_settings::ProjectSettings,
};

pub(super) type CodeLensTask =
    Shared<Task<std::result::Result<Option<Vec<CodeAction>>, Arc<anyhow::Error>>>>;

#[derive(Debug, Default)]
pub(super) struct CodeLensData {
    pub(super) lens: HashMap<LanguageServerId, Vec<CodeAction>>,
    pub(super) update: Option<(Global, CodeLensTask)>,
}

impl CodeLensData {
    pub(super) fn remove_server_data(&mut self, server_id: LanguageServerId) {
        self.lens.remove(&server_id);
    }
}

impl LspStore {
    pub fn code_lens_actions(
        &mut self,
        buffer: &Entity<Buffer>,
        cx: &mut Context<Self>,
    ) -> CodeLensTask {
        let version_queried_for = buffer.read(cx).version();
        let buffer_id = buffer.read(cx).remote_id();
        let existing_servers = self.as_local().map(|local| {
            local
                .buffers_opened_in_servers
                .get(&buffer_id)
                .cloned()
                .unwrap_or_default()
        });

        if let Some(lsp_data) = self.current_lsp_data(buffer_id) {
            if let Some(cached_lens) = &lsp_data.code_lens {
                if !version_queried_for.changed_since(&lsp_data.buffer_version) {
                    let has_different_servers = existing_servers.is_some_and(|existing_servers| {
                        existing_servers != cached_lens.lens.keys().copied().collect()
                    });
                    if !has_different_servers {
                        return Task::ready(Ok(Some(
                            cached_lens.lens.values().flatten().cloned().collect(),
                        )))
                        .shared();
                    }
                } else if let Some((updating_for, running_update)) = cached_lens.update.as_ref() {
                    if !version_queried_for.changed_since(updating_for) {
                        return running_update.clone();
                    }
                }
            }
        }

        let lens_lsp_data = self
            .latest_lsp_data(buffer, cx)
            .code_lens
            .get_or_insert_default();
        let buffer = buffer.clone();
        let query_version_queried_for = version_queried_for.clone();
        let new_task = cx
            .spawn(async move |lsp_store, cx| {
                cx.background_executor()
                    .timer(Duration::from_millis(30))
                    .await;
                let fetched_lens = lsp_store
                    .update(cx, |lsp_store, cx| lsp_store.fetch_code_lens(&buffer, cx))
                    .map_err(Arc::new)?
                    .await
                    .context("fetching code lens")
                    .map_err(Arc::new);
                let fetched_lens = match fetched_lens {
                    Ok(fetched_lens) => fetched_lens,
                    Err(e) => {
                        lsp_store
                            .update(cx, |lsp_store, _| {
                                if let Some(lens_lsp_data) = lsp_store
                                    .lsp_data
                                    .get_mut(&buffer_id)
                                    .and_then(|lsp_data| lsp_data.code_lens.as_mut())
                                {
                                    lens_lsp_data.update = None;
                                }
                            })
                            .ok();
                        return Err(e);
                    }
                };

                lsp_store
                    .update(cx, |lsp_store, _| {
                        let lsp_data = lsp_store.current_lsp_data(buffer_id)?;
                        let code_lens = lsp_data.code_lens.as_mut()?;
                        if let Some(fetched_lens) = fetched_lens {
                            if lsp_data.buffer_version == query_version_queried_for {
                                code_lens.lens.extend(fetched_lens);
                            } else if !lsp_data
                                .buffer_version
                                .changed_since(&query_version_queried_for)
                            {
                                lsp_data.buffer_version = query_version_queried_for;
                                code_lens.lens = fetched_lens;
                            }
                        }
                        code_lens.update = None;
                        Some(code_lens.lens.values().flatten().cloned().collect())
                    })
                    .map_err(Arc::new)
            })
            .shared();
        lens_lsp_data.update = Some((version_queried_for, new_task.clone()));
        new_task
    }

    pub(super) fn fetch_code_lens(
        &mut self,
        buffer: &Entity<Buffer>,
        cx: &mut Context<Self>,
    ) -> Task<Result<Option<HashMap<LanguageServerId, Vec<CodeAction>>>>> {
        if let Some((upstream_client, project_id)) = self.upstream_client() {
            let request = GetCodeLens;
            if !self.is_capable_for_proto_request(buffer, &request, cx) {
                return Task::ready(Ok(None));
            }
            let request_timeout = ProjectSettings::get_global(cx)
                .global_lsp_settings
                .get_request_timeout();
            let request_task = upstream_client.request_lsp(
                project_id,
                None,
                request_timeout,
                cx.background_executor().clone(),
                request.to_proto(project_id, buffer.read(cx)),
            );
            let buffer = buffer.clone();
            cx.spawn(async move |weak_lsp_store, cx| {
                let Some(lsp_store) = weak_lsp_store.upgrade() else {
                    return Ok(None);
                };
                let Some(responses) = request_task.await? else {
                    return Ok(None);
                };

                let code_lens_actions = join_all(responses.payload.into_iter().map(|response| {
                    let lsp_store = lsp_store.clone();
                    let buffer = buffer.clone();
                    let cx = cx.clone();
                    async move {
                        (
                            LanguageServerId::from_proto(response.server_id),
                            GetCodeLens
                                .response_from_proto(response.response, lsp_store, buffer, cx)
                                .await,
                        )
                    }
                }))
                .await;

                let mut has_errors = false;
                let code_lens_actions = code_lens_actions
                    .into_iter()
                    .filter_map(|(server_id, code_lens)| match code_lens {
                        Ok(code_lens) => Some((server_id, code_lens)),
                        Err(e) => {
                            has_errors = true;
                            log::error!("{e:#}");
                            None
                        }
                    })
                    .collect::<HashMap<_, _>>();
                anyhow::ensure!(
                    !has_errors || !code_lens_actions.is_empty(),
                    "Failed to fetch code lens"
                );
                Ok(Some(code_lens_actions))
            })
        } else {
            let code_lens_actions_task =
                self.request_multiple_lsp_locally(buffer, None::<usize>, GetCodeLens, cx);
            cx.background_spawn(async move {
                Ok(Some(code_lens_actions_task.await.into_iter().collect()))
            })
        }
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn forget_code_lens_task(&mut self, buffer_id: text::BufferId) -> Option<CodeLensTask> {
        Some(
            self.lsp_data
                .get_mut(&buffer_id)?
                .code_lens
                .take()?
                .update
                .take()?
                .1,
        )
    }

    pub(super) async fn handle_refresh_code_lens(
        this: Entity<Self>,
        _: TypedEnvelope<proto::RefreshCodeLens>,
        mut cx: AsyncApp,
    ) -> Result<proto::Ack> {
        this.update(&mut cx, |_, cx| {
            cx.emit(LspStoreEvent::RefreshCodeLens);
        });
        Ok(proto::Ack {})
    }
}
