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
use language::{Anchor, Buffer};
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

pub type CodeLensResolveTask = Shared<Task<Option<CodeAction>>>;

#[derive(Debug, Default)]
pub(super) struct CodeLensData {
    pub(super) lens: HashMap<LanguageServerId, Vec<CodeAction>>,
    pub(super) update: Option<(Global, CodeLensTask)>,
    /// In-flight `codeLens/resolve` requests, keyed by `(server, action
    /// range)` so multiple callers awaiting the same lens (e.g. concurrent
    /// scroll-driven resolves overlapping a quick-action menu fetch) share
    /// a single LSP request. Entries are cleared on completion.
    pub(super) resolving: HashMap<(LanguageServerId, Range<Anchor>), CodeLensResolveTask>,
}

impl CodeLensData {
    pub(super) fn remove_server_data(&mut self, server_id: LanguageServerId) {
        self.lens.remove(&server_id);
        self.resolving.retain(|(s, _), _| *s != server_id);
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

    /// Resolves a single code lens action via `codeLens/resolve`. The
    /// returned task is shared and cached on [`CodeLensData::resolving`]
    /// keyed by `(server, range)`, so concurrent callers awaiting the same
    /// lens only drive a single LSP request.
    ///
    /// `None` is yielded when the lens cannot be resolved (no cached
    /// action, server gone, server has no `resolveProvider`, request
    /// failure, etc.). On success, the cached `lsp_data` entry is updated
    /// in place before the resolved [`CodeAction`] is returned.
    ///
    /// All visibility / batching policy lives in the caller. Remote (proto)
    /// resolves are not yet supported and currently yield `None` so callers
    /// can fall through to the unresolved title.
    pub fn resolve_code_lens(
        &mut self,
        buffer: &Entity<Buffer>,
        action: &CodeAction,
        cx: &mut Context<Self>,
    ) -> CodeLensResolveTask {
        let buffer_id = buffer.read(cx).remote_id();
        let server_id = action.server_id;
        let key_range = action.range.clone();

        let Some(code_lens) = self
            .lsp_data
            .get_mut(&buffer_id)
            .and_then(|data| data.code_lens.as_mut())
        else {
            return Task::ready(None).shared();
        };
        if let Some(existing) = code_lens.resolving.get(&(server_id, key_range.clone())) {
            return existing.clone();
        }
        let Some(actions) = code_lens.lens.get(&server_id) else {
            return Task::ready(None).shared();
        };
        let Some(cached) = actions.iter().find(|a| a.range == key_range) else {
            return Task::ready(None).shared();
        };
        if cached.resolved {
            return Task::ready(Some(cached.clone())).shared();
        }
        let LspAction::CodeLens(lens) = &cached.lsp_action else {
            return Task::ready(None).shared();
        };
        let lens = lens.clone();

        let Some(server) = self.language_server_for_id(server_id) else {
            return Task::ready(None).shared();
        };
        if !GetCodeLens::can_resolve_lens(&server.capabilities()) {
            return Task::ready(None).shared();
        }
        let request_timeout = ProjectSettings::get_global(cx)
            .global_lsp_settings
            .get_request_timeout();

        let key = (server_id, key_range.clone());
        let task = cx
            .spawn({
                let key = key.clone();
                async move |lsp_store, cx| {
                    let response = server
                        .request::<lsp::request::CodeLensResolve>(lens, request_timeout)
                        .await
                        .into_response();
                    lsp_store
                        .update(cx, |lsp_store, _| {
                            let code_lens = lsp_store
                                .lsp_data
                                .get_mut(&buffer_id)
                                .and_then(|data| data.code_lens.as_mut())?;
                            code_lens.resolving.remove(&key);
                            let resolved_lens = match response {
                                Ok(resolved_lens) => resolved_lens,
                                Err(e) => {
                                    log::warn!("Failed to resolve code lens: {e:#}");
                                    return None;
                                }
                            };
                            let actions = code_lens.lens.get_mut(&key.0)?;
                            let action = actions.iter_mut().find(|a| a.range == key.1)?;
                            action.resolved = true;
                            action.lsp_action = LspAction::CodeLens(resolved_lens);
                            Some(action.clone())
                        })
                        .ok()
                        .flatten()
                }
            })
            .shared();

        if let Some(code_lens) = self
            .lsp_data
            .get_mut(&buffer_id)
            .and_then(|data| data.code_lens.as_mut())
        {
            code_lens.resolving.insert(key, task.clone());
        }
        task
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
                let snapshot = buffer.read_with(cx, |buffer, _| buffer.snapshot());
                actions.retain(|action| {
                    range.start.cmp(&action.range.start, &snapshot).is_ge()
                        && range.end.cmp(&action.range.end, &snapshot).is_le()
                });
                let resolve_tasks = lsp_store.update(cx, |lsp_store, cx| {
                    actions
                        .iter()
                        .filter(|action| !action.resolved)
                        .map(|action| lsp_store.resolve_code_lens(&buffer, action, cx))
                        .collect::<Vec<_>>()
                });
                for resolved in join_all(resolve_tasks).await.into_iter().flatten() {
                    if let Some(action) = actions
                        .iter_mut()
                        .find(|a| a.server_id == resolved.server_id && a.range == resolved.range)
                    {
                        *action = resolved;
                    }
                }
            }
            Ok(actions)
        })
    }
}
