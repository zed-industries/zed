use std::sync::Arc;
use std::time::Duration;

use anyhow::Context as _;
use clock::Global;
use collections::HashMap;
use futures::FutureExt as _;
use futures::future::{Shared, join_all};
use gpui::{AppContext as _, Context, Entity, Task};
use itertools::Itertools;
use language::{Buffer, BufferSnapshot, OutlineItem};
use lsp::LanguageServerId;
use settings::Settings as _;
use text::{Anchor, Bias};
use util::ResultExt;

use crate::DocumentSymbol;
use crate::lsp_command::{GetDocumentSymbols, LspCommand as _};
use crate::lsp_store::LspStore;
use crate::project_settings::ProjectSettings;

pub(super) type DocumentSymbolsTask =
    Shared<Task<std::result::Result<Vec<OutlineItem<Anchor>>, Arc<anyhow::Error>>>>;

#[derive(Debug, Default)]
pub(super) struct DocumentSymbolsData {
    symbols: HashMap<LanguageServerId, Vec<OutlineItem<Anchor>>>,
    symbols_update: Option<(Global, DocumentSymbolsTask)>,
}

impl DocumentSymbolsData {
    pub(super) fn remove_server_data(&mut self, for_server: LanguageServerId) {
        self.symbols.remove(&for_server);
    }
}

impl LspStore {
    /// Returns a task that resolves to the document symbol outline items for
    /// the given buffer.
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
    ) -> Task<Vec<OutlineItem<Anchor>>> {
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
            if let Some(cached) = &lsp_data.document_symbols {
                if !version_queried_for.changed_since(&lsp_data.buffer_version) {
                    let has_different_servers =
                        current_language_servers.is_some_and(|current_language_servers| {
                            current_language_servers != cached.symbols.keys().copied().collect()
                        });
                    if !has_different_servers {
                        let snapshot = buffer.read(cx).snapshot();
                        return Task::ready(
                            cached
                                .symbols
                                .values()
                                .flatten()
                                .cloned()
                                .sorted_by(|a, b| a.range.start.cmp(&b.range.start, &snapshot))
                                .collect(),
                        );
                    }
                }
            }
        }

        let doc_symbols_data = self
            .latest_lsp_data(buffer, cx)
            .document_symbols
            .get_or_insert_default();
        if let Some((updating_for, running_update)) = &doc_symbols_data.symbols_update {
            if !version_queried_for.changed_since(updating_for) {
                let running = running_update.clone();
                return cx
                    .background_spawn(async move { running.await.log_err().unwrap_or_default() });
            }
        }

        let buffer = buffer.clone();
        let query_version = version_queried_for.clone();
        let new_task = cx
            .spawn(async move |lsp_store, cx| {
                cx.background_executor()
                    .timer(Duration::from_millis(30))
                    .await;

                let fetched = lsp_store
                    .update(cx, |lsp_store, cx| {
                        lsp_store.fetch_document_symbols_for_buffer(&buffer, cx)
                    })
                    .map_err(Arc::new)?
                    .await
                    .context("fetching document symbols")
                    .map_err(Arc::new);

                let fetched = match fetched {
                    Ok(fetched) => fetched,
                    Err(e) => {
                        lsp_store
                            .update(cx, |lsp_store, _| {
                                if let Some(lsp_data) = lsp_store.lsp_data.get_mut(&buffer_id) {
                                    if let Some(document_symbols) = &mut lsp_data.document_symbols {
                                        document_symbols.symbols_update = None;
                                    }
                                }
                            })
                            .ok();
                        return Err(e);
                    }
                };

                lsp_store
                    .update(cx, |lsp_store, cx| {
                        let snapshot = buffer.read(cx).snapshot();
                        let lsp_data = lsp_store.latest_lsp_data(&buffer, cx);
                        let doc_symbols = lsp_data.document_symbols.get_or_insert_default();

                        if let Some(fetched_symbols) = fetched {
                            let converted = fetched_symbols
                                .iter()
                                .map(|(&server_id, symbols)| {
                                    let mut items = Vec::new();
                                    flatten_document_symbols(symbols, &snapshot, 0, &mut items);
                                    (server_id, items)
                                })
                                .collect();
                            if lsp_data.buffer_version == query_version {
                                doc_symbols.symbols.extend(converted);
                            } else if !lsp_data.buffer_version.changed_since(&query_version) {
                                lsp_data.buffer_version = query_version;
                                doc_symbols.symbols = converted;
                            }
                        }
                        doc_symbols.symbols_update = None;
                        doc_symbols
                            .symbols
                            .values()
                            .flatten()
                            .cloned()
                            .sorted_by(|a, b| a.range.start.cmp(&b.range.start, &snapshot))
                            .collect()
                    })
                    .map_err(Arc::new)
            })
            .shared();

        doc_symbols_data.symbols_update = Some((version_queried_for, new_task.clone()));

        cx.background_spawn(async move { new_task.await.log_err().unwrap_or_default() })
    }

    fn fetch_document_symbols_for_buffer(
        &mut self,
        buffer: &Entity<Buffer>,
        cx: &mut Context<Self>,
    ) -> Task<anyhow::Result<Option<HashMap<LanguageServerId, Vec<DocumentSymbol>>>>> {
        if let Some((client, project_id)) = self.upstream_client() {
            let request = GetDocumentSymbols;
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

                let document_symbols = join_all(responses.payload.into_iter().map(|response| {
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
                Ok(Some(result))
            })
        } else {
            let symbols_task =
                self.request_multiple_lsp_locally(buffer, None::<usize>, GetDocumentSymbols, cx);
            cx.background_spawn(async move { Ok(Some(symbols_task.await.into_iter().collect())) })
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
        let start = snapshot.clip_point_utf16(symbol.range.start, Bias::Right);
        let end = snapshot.clip_point_utf16(symbol.range.end, Bias::Left);
        let selection_start = snapshot.clip_point_utf16(symbol.selection_range.start, Bias::Right);
        let selection_end = snapshot.clip_point_utf16(symbol.selection_range.end, Bias::Left);

        let range = snapshot.anchor_after(start)..snapshot.anchor_before(end);
        let selection_range =
            snapshot.anchor_after(selection_start)..snapshot.anchor_before(selection_end);

        let text = symbol.name.clone();
        let name_ranges = vec![0..text.len()];

        output.push(OutlineItem {
            depth,
            range,
            source_range_for_text: selection_range,
            text,
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

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::TestAppContext;
    use text::Unclipped;

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
            kind,
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
        assert_eq!(items[0].text, "Foo");
        assert_eq!(items[0].name_ranges, vec![0..3]);

        assert_eq!(items[1].depth, 1);
        assert_eq!(items[1].text, "bar");
        assert_eq!(items[1].name_ranges, vec![0..3]);

        assert_eq!(items[2].depth, 1);
        assert_eq!(items[2].text, "baz");
        assert_eq!(items[2].name_ranges, vec![0..3]);

        assert_eq!(items[3].depth, 0);
        assert_eq!(items[3].text, "Foo");
        assert_eq!(items[3].name_ranges, vec![0..3]);

        assert_eq!(items[4].depth, 1);
        assert_eq!(items[4].text, "new");
        assert_eq!(items[4].name_ranges, vec![0..3]);
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
}
