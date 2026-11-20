use std::any::TypeId;
use std::ops::Range;
use std::sync::Arc;
use std::time::Duration;

use anyhow::Context as _;
use collections::{HashMap, HashSet};
use futures::FutureExt as _;
use futures::future::{Shared, join_all};
use gpui::{AppContext as _, AsyncApp, Context, Entity, Task};
use itertools::Itertools;
use language::{Buffer, BufferSnapshot, OutlineItem};
use lsp::LanguageServerId;
use rpc::{TypedEnvelope, proto, proto::LspRequestMessage as _};
use settings::Settings as _;
use text::{Anchor, Bias, PointUtf16};
use util::ResultExt;

use crate::DocumentSymbol;
use crate::lsp_command::{GetDocumentSymbols, LspCommand as _};
use crate::lsp_store::{
    LspKey, LspRequestId, LspStore, LspStoreEvent, RunningFetch, missing_servers_to_query,
    next_lsp_fetch_id,
};
use crate::project_settings::ProjectSettings;

pub(super) type DocumentSymbolsTask =
    Shared<Task<std::result::Result<Option<Vec<OutlineItem<Anchor>>>, Arc<anyhow::Error>>>>;

#[derive(Debug, Default)]
pub(super) struct DocumentSymbolsData {
    symbols: HashMap<LanguageServerId, Vec<OutlineItem<Anchor>>>,
    // Includes inapplicable remote candidates, but never failed requests.
    fetched_servers: HashSet<LanguageServerId>,
    symbols_update: Option<RunningFetch<DocumentSymbolsTask>>,
}

#[derive(Default)]
struct FetchedDocumentSymbols {
    symbols: HashMap<LanguageServerId, Vec<DocumentSymbol>>,
    inapplicable_servers: HashSet<LanguageServerId>,
}

impl DocumentSymbolsData {
    pub(super) fn remove_server_data(&mut self, for_server: LanguageServerId) {
        self.symbols.remove(&for_server);
        self.fetched_servers.remove(&for_server);
        RunningFetch::discard_if_queried(&mut self.symbols_update, for_server);
    }

    fn evict(&mut self, for_server: Option<LanguageServerId>) {
        match for_server {
            Some(server_id) => self.remove_server_data(server_id),
            None => {
                self.symbols.clear();
                self.fetched_servers.clear();
                self.symbols_update = None;
            }
        }
    }
}

impl LspStore {
    pub(super) fn refresh_document_symbols(
        &mut self,
        for_server: Option<LanguageServerId>,
        cx: &mut Context<Self>,
    ) {
        for lsp_data in self.lsp_data.values_mut() {
            if let Some(document_symbols) = &mut lsp_data.document_symbols {
                document_symbols.evict(for_server);
            }
        }

        cx.emit(LspStoreEvent::RefreshDocumentSymbols {
            server_id: for_server,
        });
        if let Some((downstream_client, project_id)) = self.downstream_client.as_ref() {
            downstream_client
                .send(proto::RefreshDocumentSymbols {
                    project_id: *project_id,
                    server_id: for_server.map(|server_id| server_id.to_proto()),
                })
                .context("sending refresh document symbols downstream")
                .log_err();
        }
    }

    pub(super) async fn handle_refresh_document_symbols(
        lsp_store: Entity<Self>,
        envelope: TypedEnvelope<proto::RefreshDocumentSymbols>,
        mut cx: AsyncApp,
    ) -> anyhow::Result<proto::Ack> {
        lsp_store.update(&mut cx, |lsp_store, cx| {
            let server_id = envelope.payload.server_id.map(LanguageServerId::from_proto);
            lsp_store.refresh_document_symbols(server_id, cx);
        });
        Ok(proto::Ack {})
    }

    /// Returns a task that resolves to the document symbol outline items for
    /// the given buffer.
    ///
    /// Returns `None` when no successful server result is available, and `Some`
    /// even when a successful result contains no symbols. Failed refreshes discard
    /// the queried servers' previous results; other servers' current results remain valid.
    ///
    /// Caches results per buffer version so repeated calls for the same version
    /// return immediately. Deduplicates concurrent in-flight requests.
    ///
    /// The returned items contain text and ranges but no syntax highlights.
    /// Callers (e.g. the editor) are responsible for applying highlights
    /// via the buffer's tree-sitter data and the active theme.
    pub fn fetch_document_symbols(
        &mut self,
        buffer: &Entity<Buffer>,
        cx: &mut Context<Self>,
    ) -> Task<Option<Vec<OutlineItem<Anchor>>>> {
        let version_queried_for = buffer.read(cx).version();
        let buffer_id = buffer.read(cx).remote_id();

        let current_servers = self.language_server_ids_for_request(buffer, &GetDocumentSymbols, cx);
        if current_servers.is_empty() {
            if let Some(lsp_data) = self.current_lsp_data(buffer_id)
                && let Some(document_symbols) = &mut lsp_data.document_symbols
            {
                document_symbols.evict(None);
            }
            return Task::ready(None);
        }

        let mut servers_to_query = None;
        if let Some(lsp_data) = self.current_lsp_data(buffer_id) {
            if !version_queried_for.changed_since(&lsp_data.buffer_version)
                && let Some(cached) = &mut lsp_data.document_symbols
            {
                match missing_servers_to_query(
                    &mut cached.symbols,
                    &mut cached.fetched_servers,
                    &current_servers,
                ) {
                    Some(missing_servers) => servers_to_query = Some(missing_servers),
                    None => {
                        let snapshot = buffer.read(cx).snapshot();
                        return Task::ready((!cached.symbols.is_empty()).then(|| {
                            cached
                                .symbols
                                .values()
                                .flatten()
                                .unique()
                                .cloned()
                                .sorted_by(|a, b| a.range.start.cmp(&b.range.start, &snapshot))
                                .collect()
                        }));
                    }
                }
            }
            if let Some(document_symbols) = &lsp_data.document_symbols
                && let Some(running) = &document_symbols.symbols_update
                && !version_queried_for.changed_since(&running.version)
                && servers_to_query
                    .as_ref()
                    .is_none_or(|missing| missing.is_subset(&running.servers))
            {
                let running = running.task.clone();
                return cx
                    .background_spawn(async move { running.await.log_err().unwrap_or_default() });
            }
        }

        let doc_symbols_data = self
            .latest_lsp_data(buffer, cx)
            .document_symbols
            .get_or_insert_default();
        let fetch_id = next_lsp_fetch_id();
        let queried_servers = servers_to_query
            .clone()
            .unwrap_or_else(|| current_servers.clone());
        let buffer = buffer.clone();
        let query_version = version_queried_for.clone();
        let servers_being_refreshed = queried_servers.clone();
        let new_task = cx
            .spawn(async move |lsp_store, cx| {
                cx.background_executor()
                    .timer(Duration::from_millis(30))
                    .await;

                let fetched = lsp_store
                    .update(cx, |lsp_store, cx| {
                        lsp_store.fetch_document_symbols_for_buffer(&buffer, servers_to_query, cx)
                    })
                    .map_err(Arc::new)?
                    .await
                    .context("fetching document symbols")
                    .log_err()
                    .flatten()
                    .unwrap_or_default();

                lsp_store
                    .update(cx, |lsp_store, cx| {
                        let current_servers = lsp_store.language_server_ids_for_request(
                            &buffer,
                            &GetDocumentSymbols,
                            cx,
                        );
                        let snapshot = buffer.read(cx).snapshot();
                        let lsp_data = lsp_store.latest_lsp_data(&buffer, cx);
                        let doc_symbols = lsp_data.document_symbols.get_or_insert_default();

                        if RunningFetch::take_finished(&mut doc_symbols.symbols_update, fetch_id) {
                            let converted = fetched
                                .symbols
                                .iter()
                                .filter(|(server_id, _)| current_servers.contains(*server_id))
                                .map(|(&server_id, symbols)| {
                                    let mut items = Vec::new();
                                    flatten_document_symbols(symbols, &snapshot, 0, &mut items);
                                    (server_id, items)
                                })
                                .collect();
                            let completed_servers = fetched
                                .symbols
                                .keys()
                                .chain(&fetched.inapplicable_servers)
                                .copied()
                                .filter(|server_id| current_servers.contains(server_id));
                            if lsp_data.buffer_version == query_version {
                                // A failed server must not regain authority through an older cached result.
                                doc_symbols.symbols.retain(|server_id, _| {
                                    !servers_being_refreshed.contains(server_id)
                                });
                                doc_symbols.fetched_servers.retain(|server_id| {
                                    !servers_being_refreshed.contains(server_id)
                                });
                                doc_symbols.symbols.extend(converted);
                                doc_symbols.fetched_servers.extend(completed_servers);
                            } else if !lsp_data.buffer_version.changed_since(&query_version) {
                                lsp_data.buffer_version = query_version;
                                doc_symbols.symbols = converted;
                                doc_symbols.fetched_servers = completed_servers.collect();
                            }
                        }
                        doc_symbols
                            .symbols
                            .retain(|server_id, _| current_servers.contains(server_id));
                        doc_symbols
                            .fetched_servers
                            .retain(|server_id| current_servers.contains(server_id));
                        (!doc_symbols.symbols.is_empty()).then(|| {
                            doc_symbols
                                .symbols
                                .values()
                                .flatten()
                                .unique()
                                .cloned()
                                .sorted_by(|a, b| a.range.start.cmp(&b.range.start, &snapshot))
                                .collect()
                        })
                    })
                    .map_err(Arc::new)
            })
            .shared();

        doc_symbols_data.symbols_update = Some(RunningFetch {
            id: fetch_id,
            version: version_queried_for,
            servers: queried_servers,
            task: new_task.clone(),
        });

        cx.background_spawn(async move { new_task.await.log_err().unwrap_or_default() })
    }

    pub(super) async fn query_document_symbols_locally(
        lsp_store: Entity<Self>,
        for_server: Option<LanguageServerId>,
        sender_id: proto::PeerId,
        request_id: LspRequestId,
        request: proto::GetDocumentSymbols,
        cx: &mut AsyncApp,
    ) -> anyhow::Result<()> {
        let (buffer_version, buffer) =
            Self::wait_for_buffer_version::<GetDocumentSymbols>(&lsp_store, &request, cx).await?;
        lsp_store.update(cx, |lsp_store, cx| {
            let requested_servers = if request.server_ids.is_empty() {
                for_server.map(|server_id| HashSet::from_iter([server_id]))
            } else {
                Some(
                    request
                        .server_ids
                        .into_iter()
                        .map(LanguageServerId::from_proto)
                        .collect(),
                )
            };
            let symbols_task =
                lsp_store.fetch_document_symbols_for_buffer(&buffer, requested_servers, cx);
            let key = LspKey {
                request_type: TypeId::of::<GetDocumentSymbols>(),
                server_queried: for_server,
                sender_id,
            };
            let lsp_data = lsp_store.latest_lsp_data(&buffer, cx);
            let requests = lsp_data.lsp_requests.entry(key).or_default();
            if proto::GetDocumentSymbols::stop_previous_requests() {
                requests.clear();
            }
            requests.insert(
                request_id,
                cx.spawn(async move |lsp_store, cx| {
                    let fetched = symbols_task.await.log_err().flatten().unwrap_or_default();
                    lsp_store
                        .update(cx, |lsp_store, cx| {
                            let Some((client, project_id)) = lsp_store.downstream_client.clone()
                            else {
                                return;
                            };
                            let mut responses = fetched
                                .symbols
                                .into_iter()
                                .map(|(server_id, symbols)| {
                                    (
                                        server_id.to_proto(),
                                        GetDocumentSymbols::response_to_proto(
                                            symbols,
                                            lsp_store,
                                            sender_id,
                                            &buffer_version,
                                            cx,
                                        ),
                                    )
                                })
                                .collect::<HashMap<_, _>>();
                            responses.extend(fetched.inapplicable_servers.into_iter().map(
                                |server_id| {
                                    (
                                        server_id.to_proto(),
                                        proto::GetDocumentSymbolsResponse {
                                            symbols: Vec::new(),
                                            inapplicable: true,
                                        },
                                    )
                                },
                            ));
                            client
                                .send_lsp_response::<proto::GetDocumentSymbols>(
                                    project_id, sender_id, request_id, responses,
                                )
                                .context("sending document symbols downstream")
                                .log_err();
                        })
                        .log_err();
                }),
            );
        });
        Ok(())
    }

    fn fetch_document_symbols_for_buffer(
        &mut self,
        buffer: &Entity<Buffer>,
        for_servers: Option<HashSet<LanguageServerId>>,
        cx: &mut Context<Self>,
    ) -> Task<anyhow::Result<Option<FetchedDocumentSymbols>>> {
        if let Some((client, project_id)) = self.upstream_client() {
            let request = GetDocumentSymbols;
            if !self.is_capable_for_proto_request(buffer, &request, cx) {
                return Task::ready(Ok(None));
            }

            let request_timeout = ProjectSettings::get_global(cx)
                .global_lsp_settings
                .get_request_timeout();
            let mut proto_request = request.to_proto(project_id, buffer.read(cx));
            let requested_servers = for_servers
                .unwrap_or_else(|| self.language_server_ids_for_request(buffer, &request, cx));
            proto_request.server_ids = requested_servers
                .iter()
                .map(|server_id| server_id.to_proto())
                .collect();
            // Older hosts ignore `server_ids`. Never use their single-server path, which can
            // fabricate an empty success when the selected server does not serve this buffer.
            let request_task = client.request_lsp(
                project_id,
                None,
                request_timeout,
                cx.background_executor().clone(),
                proto_request,
            );
            let buffer = buffer.clone();
            cx.spawn(async move |weak_lsp_store, cx| {
                let Some(lsp_store) = weak_lsp_store.upgrade() else {
                    return Ok(None);
                };
                let Some(responses) = request_task.await? else {
                    return Ok(None);
                };

                let mut inapplicable_servers = HashSet::default();
                let responses = responses.payload.into_iter().filter(|response| {
                    // An older host may return servers outside the scope.
                    // Their results are not protected by this fetch's invalidation guard.
                    // May need improvement.
                    if !requested_servers
                        .contains(&LanguageServerId::from_proto(response.server_id))
                    {
                        return false;
                    }
                    if response.response.inapplicable {
                        inapplicable_servers
                            .insert(LanguageServerId::from_proto(response.server_id));
                        false
                    } else {
                        true
                    }
                });
                let document_symbols = join_all(responses.map(|response| {
                    let lsp_store = lsp_store.clone();
                    let buffer = buffer.clone();
                    let cx = cx.clone();
                    async move {
                        (
                            LanguageServerId::from_proto(response.server_id),
                            GetDocumentSymbols
                                .response_from_proto(response.response, lsp_store, buffer, cx)
                                .await,
                        )
                    }
                }))
                .await;

                let mut has_errors = false;
                let result = document_symbols
                    .into_iter()
                    .filter_map(|(server_id, symbols)| match symbols {
                        Ok(symbols) => Some((server_id, symbols)),
                        Err(e) => {
                            has_errors = true;
                            log::error!("Failed to fetch document symbols: {e:#}");
                            None
                        }
                    })
                    .collect::<HashMap<_, _>>();
                anyhow::ensure!(
                    !has_errors || !result.is_empty(),
                    "Failed to fetch document symbols"
                );
                Ok(Some(FetchedDocumentSymbols {
                    symbols: result,
                    inapplicable_servers,
                }))
            })
        } else {
            let serving_servers =
                self.language_server_ids_for_request(buffer, &GetDocumentSymbols, cx);
            let inapplicable_servers = for_servers
                .iter()
                .flatten()
                .filter(|server_id| !serving_servers.contains(*server_id))
                .copied()
                .collect();
            let symbols_task = self.request_filtered_lsp_locally(
                buffer,
                None::<usize>,
                GetDocumentSymbols,
                for_servers.as_ref(),
                cx,
            );
            cx.background_spawn(async move {
                Ok(Some(FetchedDocumentSymbols {
                    symbols: symbols_task.await.into_iter().collect(),
                    inapplicable_servers,
                }))
            })
        }
    }
}

fn flatten_document_symbols(
    symbols: &[DocumentSymbol],
    snapshot: &BufferSnapshot,
    depth: usize,
    output: &mut Vec<OutlineItem<Anchor>>,
) {
    for symbol in symbols {
        let name = super::collapse_newlines(&symbol.name, " ");

        let start = snapshot.clip_point_utf16(symbol.range.start, Bias::Right);
        let end = snapshot.clip_point_utf16(symbol.range.end, Bias::Left);
        let selection_start = snapshot.clip_point_utf16(symbol.selection_range.start, Bias::Right);
        let selection_end = snapshot.clip_point_utf16(symbol.selection_range.end, Bias::Left);

        let range = snapshot.anchor_after(start)..snapshot.anchor_before(end);
        let selection_range =
            snapshot.anchor_after(selection_start)..snapshot.anchor_before(selection_end);

        let (text, name_ranges, source_range_for_text) =
            enriched_symbol_text(&name, start, selection_start, selection_end, snapshot)
                .unwrap_or_else(|| {
                    let name_len = name.len();
                    (name.clone(), vec![0..name_len], selection_range.clone())
                });

        output.push(OutlineItem {
            depth,
            range,
            selection_range,
            source_range_for_text,
            text: text.into(),
            highlight_ranges: Vec::new(),
            name_ranges,
            body_range: None,
            annotation_range: None,
        });

        if !symbol.children.is_empty() {
            flatten_document_symbols(&symbol.children, snapshot, depth + 1, output);
        }
    }
}

/// Tries to build an enriched label by including buffer text from the symbol
/// range start to the selection range end (e.g., "struct Foo" instead of just "Foo").
/// Only uses same-line prefix to avoid pulling in attributes/decorators.
fn enriched_symbol_text(
    name: &str,
    range_start: PointUtf16,
    selection_start: PointUtf16,
    selection_end: PointUtf16,
    snapshot: &BufferSnapshot,
) -> Option<(String, Vec<Range<usize>>, Range<Anchor>)> {
    let text_start = if range_start.row == selection_start.row {
        range_start
    } else {
        PointUtf16::new(selection_start.row, 0)
    };

    let start_offset = snapshot.point_utf16_to_offset(text_start);
    let end_offset = snapshot.point_utf16_to_offset(selection_end);
    if start_offset >= end_offset {
        return None;
    }

    let raw: String = snapshot.text_for_range(start_offset..end_offset).collect();
    let trimmed = raw.trim_start();
    if trimmed.len() <= name.len() || !trimmed.ends_with(name) {
        return None;
    }

    let name_start = trimmed.len() - name.len();
    let leading_ws = raw.len() - trimmed.len();
    let adjusted_start = start_offset + leading_ws;

    Some((
        trimmed.to_string(),
        vec![name_start..trimmed.len()],
        snapshot.anchor_after(adjusted_start)..snapshot.anchor_before(end_offset),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::TestAppContext;
    use language::lsp_to_symbol_kind;
    use text::{OffsetRangeExt, Point, Unclipped};

    fn make_symbol(
        name: &str,
        kind: lsp::SymbolKind,
        range: std::ops::Range<(u32, u32)>,
        selection_range: std::ops::Range<(u32, u32)>,
        children: Vec<DocumentSymbol>,
    ) -> DocumentSymbol {
        use text::PointUtf16;
        DocumentSymbol {
            name: name.to_string(),
            kind: lsp_to_symbol_kind(kind),
            range: Unclipped(PointUtf16::new(range.start.0, range.start.1))
                ..Unclipped(PointUtf16::new(range.end.0, range.end.1)),
            selection_range: Unclipped(PointUtf16::new(
                selection_range.start.0,
                selection_range.start.1,
            ))
                ..Unclipped(PointUtf16::new(
                    selection_range.end.0,
                    selection_range.end.1,
                )),
            children,
        }
    }

    #[gpui::test]
    async fn test_flatten_document_symbols(cx: &mut TestAppContext) {
        let buffer = cx.new(|cx| {
            Buffer::local(
                concat!(
                    "struct Foo {\n",
                    "    bar: u32,\n",
                    "    baz: String,\n",
                    "}\n",
                    "\n",
                    "impl Foo {\n",
                    "    fn new() -> Self {\n",
                    "        Foo { bar: 0, baz: String::new() }\n",
                    "    }\n",
                    "}\n",
                ),
                cx,
            )
        });

        let symbols = vec![
            make_symbol(
                "Foo",
                lsp::SymbolKind::STRUCT,
                (0, 0)..(3, 1),
                (0, 7)..(0, 10),
                vec![
                    make_symbol(
                        "bar",
                        lsp::SymbolKind::FIELD,
                        (1, 4)..(1, 13),
                        (1, 4)..(1, 7),
                        vec![],
                    ),
                    make_symbol(
                        "baz",
                        lsp::SymbolKind::FIELD,
                        (2, 4)..(2, 15),
                        (2, 4)..(2, 7),
                        vec![],
                    ),
                ],
            ),
            make_symbol(
                "Foo",
                lsp::SymbolKind::STRUCT,
                (5, 0)..(9, 1),
                (5, 5)..(5, 8),
                vec![make_symbol(
                    "new",
                    lsp::SymbolKind::FUNCTION,
                    (6, 4)..(8, 5),
                    (6, 7)..(6, 10),
                    vec![],
                )],
            ),
        ];

        let snapshot = buffer.read_with(cx, |buffer, _| buffer.snapshot());

        let mut items = Vec::new();
        flatten_document_symbols(&symbols, &snapshot, 0, &mut items);

        assert_eq!(items.len(), 5);

        assert_eq!(items[0].depth, 0);
        assert_eq!(items[0].text, "struct Foo");
        assert_eq!(items[0].name_ranges, vec![7..10]);
        assert_eq!(
            items[0].selection_range.to_point(&snapshot),
            Point::new(0, 7)..Point::new(0, 10)
        );

        assert_eq!(items[1].depth, 1);
        assert_eq!(items[1].text, "bar");
        assert_eq!(items[1].name_ranges, vec![0..3]);
        assert_eq!(
            items[1].selection_range.to_point(&snapshot),
            Point::new(1, 4)..Point::new(1, 7)
        );

        assert_eq!(items[2].depth, 1);
        assert_eq!(items[2].text, "baz");
        assert_eq!(items[2].name_ranges, vec![0..3]);
        assert_eq!(
            items[2].selection_range.to_point(&snapshot),
            Point::new(2, 4)..Point::new(2, 7)
        );

        assert_eq!(items[3].depth, 0);
        assert_eq!(items[3].text, "impl Foo");
        assert_eq!(items[3].name_ranges, vec![5..8]);
        assert_eq!(
            items[3].selection_range.to_point(&snapshot),
            Point::new(5, 5)..Point::new(5, 8)
        );

        assert_eq!(items[4].depth, 1);
        assert_eq!(items[4].text, "fn new");
        assert_eq!(items[4].name_ranges, vec![3..6]);
        assert_eq!(
            items[4].selection_range.to_point(&snapshot),
            Point::new(6, 7)..Point::new(6, 10)
        );
    }

    #[gpui::test]
    async fn test_empty_symbols(cx: &mut TestAppContext) {
        let buffer = cx.new(|cx| Buffer::local("", cx));
        let snapshot = buffer.read_with(cx, |buffer, _| buffer.snapshot());

        let symbols: Vec<DocumentSymbol> = Vec::new();
        let mut items = Vec::new();
        flatten_document_symbols(&symbols, &snapshot, 0, &mut items);
        assert!(items.is_empty());
    }

    #[gpui::test]
    async fn test_newlines_collapsed_in_name(cx: &mut TestAppContext) {
        let buffer = cx.new(|cx| Buffer::local("x = 1\ny = 2\n", cx));

        let symbols = vec![
            make_symbol(
                "line1\nline2",
                lsp::SymbolKind::VARIABLE,
                (0, 0)..(0, 5),
                (0, 0)..(0, 1),
                vec![],
            ),
            make_symbol(
                "  a  \n  b  ",
                lsp::SymbolKind::VARIABLE,
                (1, 0)..(1, 5),
                (1, 0)..(1, 1),
                vec![],
            ),
            make_symbol(
                "a\r\nb",
                lsp::SymbolKind::VARIABLE,
                (0, 0)..(1, 5),
                (0, 0)..(0, 1),
                vec![],
            ),
            make_symbol(
                "a\n\nb",
                lsp::SymbolKind::VARIABLE,
                (0, 0)..(1, 5),
                (0, 0)..(0, 1),
                vec![],
            ),
        ];

        let snapshot = buffer.read_with(cx, |buffer, _| buffer.snapshot());
        let mut items = Vec::new();
        flatten_document_symbols(&symbols, &snapshot, 0, &mut items);

        assert_eq!(items.len(), 4);
        assert_eq!(items[0].text, "line1 line2");
        assert_eq!(items[1].text, "a b");
        assert_eq!(items[2].text, "a b");
        assert_eq!(items[3].text, "a b");
    }
}
