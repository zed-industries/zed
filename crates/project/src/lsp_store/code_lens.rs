use std::ops::Range;
use std::sync::Arc;

use anyhow::{Context as _, Result};
use clock::Global;
use collections::HashMap;
use futures::{
    FutureExt as _,
    future::{Shared, join_all},
};
use gpui::{AppContext as _, AsyncApp, Context, Entity, Task};
use language::{Anchor, Buffer, ToOffset as _};
use lsp::LanguageServerId;
use rpc::{TypedEnvelope, proto};
use settings::Settings as _;
use std::time::Duration;
use text::OffsetRangeExt as _;

use crate::{
    CodeAction, LspAction, LspStore, LspStoreEvent, Project,
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
    pub(super) fn invalidate_code_lens(&mut self) {
        for lsp_data in self.lsp_data.values_mut() {
            lsp_data.code_lens = None;
        }
    }

    /// Fetches and returns all code lenses for the buffer.
    ///
    /// Resolution of individual lenses is the caller's responsibility; see
    /// [`LspStore::resolve_visible_code_lenses`].
    pub fn code_lens_actions(
        &mut self,
        buffer: &Entity<Buffer>,
        cx: &mut Context<Self>,
    ) -> Task<Result<Option<Vec<CodeAction>>>> {
        let buffer_id = buffer.read(cx).remote_id();
        let fetch_task = self.fetch_code_lenses(buffer, cx);

        cx.spawn(async move |lsp_store, cx| {
            fetch_task
                .await
                .map_err(|e| anyhow::anyhow!("code lens fetch failed: {e:#}"))?;

            let actions = lsp_store.read_with(cx, |lsp_store, _| {
                lsp_store
                    .lsp_data
                    .get(&buffer_id)
                    .and_then(|data| data.code_lens.as_ref())
                    .map(|code_lens| code_lens.lens.values().flatten().cloned().collect())
            })?;
            Ok(actions)
        })
    }

    fn fetch_code_lenses(
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
                    .update(cx, |lsp_store, cx| {
                        lsp_store.fetch_code_lens_for_buffer(&buffer, cx)
                    })
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
                    .update(cx, |lsp_store, cx| {
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
                            let snapshot = buffer.read(cx).snapshot();
                            for actions in code_lens.lens.values_mut() {
                                actions
                                    .sort_by(|a, b| a.range.start.cmp(&b.range.start, &snapshot));
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

    fn fetch_code_lens_for_buffer(
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

    pub fn resolve_visible_code_lenses(
        &mut self,
        buffer: &Entity<Buffer>,
        visible_range: Range<Anchor>,
        cx: &mut Context<Self>,
    ) -> Task<Vec<CodeAction>> {
        let buffer_id = buffer.read(cx).remote_id();
        let snapshot = buffer.read(cx).snapshot();
        let visible_start = visible_range.start.to_offset(&snapshot);
        let visible_end = visible_range.end.to_offset(&snapshot);

        let Some(code_lens) = self
            .lsp_data
            .get(&buffer_id)
            .and_then(|data| data.code_lens.as_ref())
        else {
            return Task::ready(Vec::new());
        };

        let capable_servers = code_lens
            .lens
            .keys()
            .filter_map(|server_id| {
                let server = self.language_server_for_id(*server_id)?;
                GetCodeLens::can_resolve_lens(&server.capabilities())
                    .then_some((*server_id, server))
            })
            .collect::<HashMap<_, _>>();
        if capable_servers.is_empty() {
            return Task::ready(Vec::new());
        }

        let to_resolve = code_lens
            .lens
            .iter()
            .flat_map(|(server_id, actions)| {
                let start_idx =
                    actions.partition_point(|a| a.range.start.to_offset(&snapshot) < visible_start);
                let end_idx = start_idx
                    + actions[start_idx..]
                        .partition_point(|a| a.range.start.to_offset(&snapshot) <= visible_end);
                actions[start_idx..end_idx].iter().enumerate().filter_map(
                    move |(local_idx, action)| {
                        let LspAction::CodeLens(lens) = &action.lsp_action else {
                            return None;
                        };
                        if lens.command.is_some() {
                            return None;
                        }
                        Some((*server_id, start_idx + local_idx, lens.clone()))
                    },
                )
            })
            .collect::<Vec<_>>();
        if to_resolve.is_empty() {
            return Task::ready(Vec::new());
        }

        let request_timeout = ProjectSettings::get_global(cx)
            .global_lsp_settings
            .get_request_timeout();

        cx.spawn(async move |lsp_store, cx| {
            let mut resolved = Vec::new();
            for (server_id, index, lens) in to_resolve {
                let Some(server) = capable_servers.get(&server_id) else {
                    continue;
                };
                match server
                    .request::<lsp::request::CodeLensResolve>(lens, request_timeout)
                    .await
                    .into_response()
                {
                    Ok(resolved_lens) => resolved.push((server_id, index, resolved_lens)),
                    Err(e) => log::warn!("Failed to resolve code lens: {e:#}"),
                }
            }
            if resolved.is_empty() {
                return Vec::new();
            }

            lsp_store
                .update(cx, |lsp_store, _| {
                    let Some(code_lens) = lsp_store
                        .lsp_data
                        .get_mut(&buffer_id)
                        .and_then(|data| data.code_lens.as_mut())
                    else {
                        return Vec::new();
                    };
                    let mut newly_resolved = Vec::new();
                    for (server_id, index, resolved_lens) in resolved {
                        if let Some(actions) = code_lens.lens.get_mut(&server_id) {
                            if let Some(action) = actions.get_mut(index) {
                                action.resolved = true;
                                action.lsp_action = LspAction::CodeLens(resolved_lens);
                                newly_resolved.push(action.clone());
                            }
                        }
                    }
                    newly_resolved
                })
                .unwrap_or_default()
        })
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
        lsp_store: Entity<Self>,
        _: TypedEnvelope<proto::RefreshCodeLens>,
        mut cx: AsyncApp,
    ) -> Result<proto::Ack> {
        lsp_store.update(&mut cx, |lsp_store, cx| {
            lsp_store.invalidate_code_lens();
            cx.emit(LspStoreEvent::RefreshCodeLens);
        });
        Ok(proto::Ack {})
    }
}

impl Project {
    pub fn code_lens_actions(
        &mut self,
        buffer: &Entity<Buffer>,
        range: Range<Anchor>,
        cx: &mut Context<Self>,
    ) -> Task<Result<Option<Vec<CodeAction>>>> {
        let snapshot = buffer.read(cx).snapshot();
        let range = range.to_point(&snapshot);
        let range_start = snapshot.anchor_before(range.start);
        let range_end = if range.start == range.end {
            range_start
        } else {
            snapshot.anchor_after(range.end)
        };
        let range = range_start..range_end;
        let lsp_store = self.lsp_store();
        let fetch_task =
            lsp_store.update(cx, |lsp_store, cx| lsp_store.code_lens_actions(buffer, cx));
        let buffer = buffer.clone();
        cx.spawn(async move |_, cx| {
            let mut actions = fetch_task.await?;
            if let Some(actions) = &mut actions {
                let resolve_task = lsp_store.update(cx, |lsp_store, cx| {
                    lsp_store.resolve_visible_code_lenses(&buffer, range.clone(), cx)
                });
                let resolved = resolve_task.await;
                for resolved_action in resolved {
                    if let Some(action) = actions.iter_mut().find(|a| {
                        a.server_id == resolved_action.server_id && a.range == resolved_action.range
                    }) {
                        *action = resolved_action;
                    }
                }
                let snapshot = buffer.read_with(cx, |buffer, _| buffer.snapshot());
                actions.retain(|action| {
                    range.start.cmp(&action.range.start, &snapshot).is_ge()
                        && range.end.cmp(&action.range.end, &snapshot).is_le()
                });
            }
            Ok(actions)
        })
    }
}
