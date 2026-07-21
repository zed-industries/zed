use std::ops::Range;
use std::sync::Arc;
use std::time::Duration;

use anyhow::Context as _;
use clock::Global;
use collections::{HashMap, HashSet};
use futures::FutureExt as _;
use futures::future::{Shared, join_all};
use gpui::{AppContext as _, AsyncApp, Context, Entity, SharedString, Task};
use itertools::Itertools;
use language::Buffer;
use lsp::LanguageServerId;
use rpc::{TypedEnvelope, proto};
use settings::Settings as _;
use text::Anchor;
use util::ResultExt as _;

use crate::lsp_command::{GetFoldingRanges, LspCommand as _};
use crate::lsp_store::{LspStore, LspStoreEvent, missing_servers_to_query};
use crate::project_settings::ProjectSettings;

#[derive(Clone, Debug)]
pub struct LspFoldingRange {
    pub range: Range<Anchor>,
    pub collapsed_text: Option<SharedString>,
}

pub(super) type FoldingRangeTask =
    Shared<Task<std::result::Result<Vec<LspFoldingRange>, Arc<anyhow::Error>>>>;

#[derive(Debug, Default)]
pub(super) struct FoldingRangeData {
    pub(super) ranges: HashMap<LanguageServerId, Vec<LspFoldingRange>>,
    ranges_update: Option<(Global, FoldingRangeTask)>,
    generation: u64,
}

impl FoldingRangeData {
    pub(super) fn remove_server_data(&mut self, server_id: LanguageServerId) {
        self.ranges.remove(&server_id);
    }

    fn evict(&mut self, for_server: Option<LanguageServerId>) {
        match for_server {
            Some(server_id) => self.remove_server_data(server_id),
            None => self.ranges.clear(),
        }
        self.ranges_update = None;
        self.generation += 1;
    }
}

impl LspStore {
    pub(super) fn refresh_folding_ranges(
        &mut self,
        for_server: Option<LanguageServerId>,
        cx: &mut Context<Self>,
    ) {
        for lsp_data in self.lsp_data.values_mut() {
            if let Some(folding_ranges) = &mut lsp_data.folding_ranges {
                folding_ranges.evict(for_server);
            }
        }

        cx.emit(LspStoreEvent::RefreshFoldingRanges {
            server_id: for_server,
        });
        if let Some((downstream_client, project_id)) = self.downstream_client.as_ref() {
            downstream_client
                .send(proto::RefreshFoldingRanges {
                    project_id: *project_id,
                    server_id: for_server.map(|server_id| server_id.to_proto()),
                })
                .context("sending refresh folding ranges downstream")
                .log_err();
        }
    }

    pub(super) async fn handle_refresh_folding_ranges(
        lsp_store: Entity<Self>,
        envelope: TypedEnvelope<proto::RefreshFoldingRanges>,
        mut cx: AsyncApp,
    ) -> anyhow::Result<proto::Ack> {
        lsp_store.update(&mut cx, |lsp_store, cx| {
            let server_id = envelope.payload.server_id.map(LanguageServerId::from_proto);
            lsp_store.refresh_folding_ranges(server_id, cx);
        });
        Ok(proto::Ack {})
    }

    /// Returns a task that resolves to the folding ranges for the given buffer.
    ///
    /// Caches results per buffer version so repeated calls for the same version
    /// return immediately. Deduplicates concurrent in-flight requests.
    pub fn fetch_folding_ranges(
        &mut self,
        buffer: &Entity<Buffer>,
        cx: &mut Context<Self>,
    ) -> Task<Vec<LspFoldingRange>> {
        let version_queried_for = buffer.read(cx).version();
        let buffer_id = buffer.read(cx).remote_id();

        let current_servers = self
            .relevant_server_ids_for_capability_check(buffer, cx)
            .into_iter()
            .collect::<HashSet<_>>();

        let mut servers_to_query = None;
        if let Some(lsp_data) = self.current_lsp_data(buffer_id)
            && !version_queried_for.changed_since(&lsp_data.buffer_version)
            && let Some(cached) = &mut lsp_data.folding_ranges
        {
            match missing_servers_to_query(&mut cached.ranges, &current_servers) {
                Some(missing_servers) => servers_to_query = Some(missing_servers),
                None => {
                    let snapshot = buffer.read(cx).snapshot();
                    return Task::ready(
                        cached
                            .ranges
                            .values()
                            .flatten()
                            .cloned()
                            .sorted_by(|a, b| a.range.start.cmp(&b.range.start, &snapshot))
                            .collect(),
                    );
                }
            }
        }

        let folding_lsp_data = self
            .latest_lsp_data(buffer, cx)
            .folding_ranges
            .get_or_insert_default();
        if let Some((updating_for, running_update)) = &folding_lsp_data.ranges_update {
            if !version_queried_for.changed_since(updating_for) {
                let running = running_update.clone();
                return cx.background_spawn(async move { running.await.unwrap_or_default() });
            }
        }

        let query_generation = folding_lsp_data.generation;
        let buffer = buffer.clone();
        let query_version = version_queried_for.clone();
        let new_task = cx
            .spawn(async move |lsp_store, cx| {
                cx.background_executor()
                    .timer(Duration::from_millis(30))
                    .await;

                let fetched = lsp_store
                    .update(cx, |lsp_store, cx| {
                        lsp_store.fetch_folding_ranges_for_buffer(&buffer, servers_to_query, cx)
                    })
                    .map_err(Arc::new)?
                    .await
                    .context("fetching folding ranges")
                    .map_err(Arc::new);

                let fetched = match fetched {
                    Ok(fetched) => fetched,
                    Err(e) => {
                        lsp_store
                            .update(cx, |lsp_store, _| {
                                if let Some(lsp_data) = lsp_store.lsp_data.get_mut(&buffer_id)
                                    && let Some(folding_ranges) = &mut lsp_data.folding_ranges
                                    && folding_ranges.generation == query_generation
                                {
                                    folding_ranges.ranges_update = None;
                                }
                            })
                            .ok();
                        return Err(e);
                    }
                };

                lsp_store
                    .update(cx, |lsp_store, cx| {
                        let lsp_data = lsp_store.latest_lsp_data(&buffer, cx);
                        let folding = lsp_data.folding_ranges.get_or_insert_default();

                        if folding.generation == query_generation {
                            if let Some(fetched_ranges) = fetched {
                                if lsp_data.buffer_version == query_version {
                                    folding.ranges.extend(fetched_ranges);
                                } else if !lsp_data.buffer_version.changed_since(&query_version) {
                                    lsp_data.buffer_version = query_version;
                                    folding.ranges = fetched_ranges;
                                }
                            }
                            folding.ranges_update = None;
                        }
                        let snapshot = buffer.read(cx).snapshot();
                        folding
                            .ranges
                            .values()
                            .flatten()
                            .cloned()
                            .sorted_by(|a, b| a.range.start.cmp(&b.range.start, &snapshot))
                            .collect()
                    })
                    .map_err(Arc::new)
            })
            .shared();

        folding_lsp_data.ranges_update = Some((version_queried_for, new_task.clone()));

        cx.background_spawn(async move { new_task.await.unwrap_or_default() })
    }

    fn fetch_folding_ranges_for_buffer(
        &mut self,
        buffer: &Entity<Buffer>,
        for_servers: Option<HashSet<LanguageServerId>>,
        cx: &mut Context<Self>,
    ) -> Task<anyhow::Result<Option<HashMap<LanguageServerId, Vec<LspFoldingRange>>>>> {
        if let Some((client, project_id)) = self.upstream_client() {
            let request = GetFoldingRanges;
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
            cx.spawn(async move |weak_lsp_store, cx| {
                let Some(lsp_store) = weak_lsp_store.upgrade() else {
                    return Ok(None);
                };
                let Some(responses) = request_task.await? else {
                    return Ok(None);
                };

                let folding_ranges = join_all(responses.payload.into_iter().map(|response| {
                    let lsp_store = lsp_store.clone();
                    let buffer = buffer.clone();
                    let cx = cx.clone();
                    async move {
                        (
                            LanguageServerId::from_proto(response.server_id),
                            GetFoldingRanges
                                .response_from_proto(response.response, lsp_store, buffer, cx)
                                .await,
                        )
                    }
                }))
                .await;

                let mut has_errors = false;
                let result = folding_ranges
                    .into_iter()
                    .filter_map(|(server_id, ranges)| match ranges {
                        Ok(ranges) => Some((server_id, ranges)),
                        Err(e) => {
                            has_errors = true;
                            log::error!("Failed to fetch folding ranges: {e:#}");
                            None
                        }
                    })
                    .collect::<HashMap<_, _>>();
                anyhow::ensure!(
                    !has_errors || !result.is_empty(),
                    "Failed to fetch folding ranges"
                );
                Ok(Some(result))
            })
        } else {
            let folding_task = self.request_filtered_lsp_locally(
                buffer,
                None::<usize>,
                GetFoldingRanges,
                for_servers.as_ref(),
                cx,
            );
            cx.background_spawn(async move { Ok(Some(folding_task.await.into_iter().collect())) })
        }
    }
}
