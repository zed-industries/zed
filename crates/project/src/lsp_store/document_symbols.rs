use std::sync::Arc;
use std::time::Duration;

use anyhow::Context as _;
use clock::Global;
use collections::HashMap;
use futures::FutureExt as _;
use futures::future::{Shared, join_all};
use gpui::{AppContext as _, Context, Entity, HighlightStyle, Task};
use itertools::Itertools;
use language::{Buffer, BufferSnapshot, OutlineItem};
use lsp::LanguageServerId;
use settings::Settings as _;
use text::{Anchor, PointUtf16, ToOffset as _};
use theme::{ActiveTheme as _, SyntaxTheme};

use crate::DocumentSymbol;
use crate::lsp_command::{GetDocumentSymbols, LspCommand as _};
use crate::lsp_store::LspStore;
use crate::project_settings::ProjectSettings;

pub(super) type DocumentSymbolsTask =
    Shared<Task<std::result::Result<Vec<OutlineItem<Anchor>>, Arc<anyhow::Error>>>>;

#[derive(Debug, Default)]
pub(super) struct DocumentSymbolsData {
    pub(super) symbols: HashMap<LanguageServerId, Vec<OutlineItem<Anchor>>>,
    symbols_update: Option<(Global, DocumentSymbolsTask)>,
}

impl DocumentSymbolsData {
    pub(super) fn remove_server_data(&mut self, for_server: LanguageServerId) {
        self.symbols.remove(&for_server);
    }
}

fn flatten_document_symbols(
    symbols: &[DocumentSymbol],
    snapshot: &BufferSnapshot,
    syntax_theme: Option<&SyntaxTheme>,
    depth: usize,
    output: &mut Vec<OutlineItem<Anchor>>,
) {
    for symbol in symbols {
        let start = snapshot.clip_point_utf16(symbol.range.start, text::Bias::Left);
        let end = snapshot.clip_point_utf16(symbol.range.end, text::Bias::Right);
        let selection_start =
            snapshot.clip_point_utf16(symbol.selection_range.start, text::Bias::Left);
        let selection_end =
            snapshot.clip_point_utf16(symbol.selection_range.end, text::Bias::Right);

        let range = snapshot.anchor_before(start)..snapshot.anchor_after(end);
        let selection_range =
            snapshot.anchor_before(selection_start)..snapshot.anchor_after(selection_end);

        let (text, highlight_ranges, name_ranges) = build_symbol_text_and_highlights(
            &symbol.name,
            snapshot,
            start..end,
            selection_start..selection_end,
            syntax_theme,
        );

        output.push(OutlineItem {
            depth,
            range,
            source_range_for_text: selection_range,
            text,
            highlight_ranges,
            name_ranges,
            body_range: None,
            annotation_range: None,
        });

        if !symbol.children.is_empty() {
            flatten_document_symbols(&symbol.children, snapshot, syntax_theme, depth + 1, output);
        }
    }
}

/// Builds the display text and highlight ranges for an LSP document symbol
/// by reading tree-sitter chunks directly from the buffer, mirroring how
/// tree-sitter outline items get their highlights in `BufferSnapshot::next_outline_item`.
///
/// Strategy:
/// 1. Try to find the symbol name verbatim in the buffer near the selection range.
///    If found, read tree-sitter chunks from that exact location — this handles
///    complex names like `impl Trait for Type` that span wider than the selection range.
/// 2. If the name is not found verbatim (e.g. the LSP rewrote generics), fall back to
///    reading chunks from the selection range, which always points at the identifier.
/// 3. Prepend the kind label (e.g. "struct", "fn") with keyword highlighting.
fn build_symbol_text_and_highlights(
    name: &str,
    snapshot: &BufferSnapshot,
    symbol_range: std::ops::Range<PointUtf16>,
    selection_range: std::ops::Range<PointUtf16>,
    syntax_theme: Option<&SyntaxTheme>,
) -> (
    String,
    Vec<(std::ops::Range<usize>, HighlightStyle)>,
    Vec<std::ops::Range<usize>>,
) {
    let mut text = String::new();
    let mut highlights = Vec::new();
    let name_start_in_text = text.len();

    if let Some(name_highlights) = syntax_theme.and_then(|theme| {
        highlights_from_buffer(
            name,
            name_start_in_text,
            snapshot,
            &symbol_range,
            &selection_range,
            theme,
        )
    }) {
        text.push_str(name);
        highlights.extend(name_highlights);
    } else {
        text.push_str(name);
    }

    let name_end_in_text = text.len();
    let name_ranges = vec![name_start_in_text..name_end_in_text];

    (text, highlights, name_ranges)
}

/// Reads tree-sitter highlights for the symbol name from the buffer.
///
/// First tries to find the name verbatim near the selection range so that
/// complex names (`impl Trait for Type`) get full highlighting. Falls back
/// to the selection range itself, which always points at the identifier.
fn highlights_from_buffer(
    name: &str,
    name_offset_in_text: usize,
    snapshot: &BufferSnapshot,
    symbol_range: &std::ops::Range<PointUtf16>,
    selection_range: &std::ops::Range<PointUtf16>,
    syntax_theme: &SyntaxTheme,
) -> Option<Vec<(std::ops::Range<usize>, HighlightStyle)>> {
    if name.is_empty() {
        return None;
    }

    let range_start_offset = symbol_range.start.to_offset(snapshot);
    let range_end_offset = symbol_range.end.to_offset(snapshot);
    let selection_start_offset = selection_range.start.to_offset(snapshot);

    // Try to find the name verbatim in the buffer near the selection range.
    let search_start = selection_start_offset
        .saturating_sub(name.len())
        .max(range_start_offset);
    let search_end = (selection_start_offset + name.len() * 2).min(range_end_offset);

    if search_start < search_end {
        let buffer_text: String = snapshot.text_for_range(search_start..search_end).collect();
        if let Some(found_at) = buffer_text.find(name) {
            let name_start_offset = search_start + found_at;
            let name_end_offset = name_start_offset + name.len();
            let result = highlights_for_range(
                name_offset_in_text,
                name_start_offset,
                name_end_offset,
                snapshot,
                syntax_theme,
            );
            if result.is_some() {
                return result;
            }
        }
    }

    // Fallback: match word-by-word. Split the name on whitespace and find
    // each word sequentially in the buffer's symbol range. This handles
    // cases like `impl<T> Trait<T> for Type` where the LSP name
    // (`impl Trait<T> for Type`) doesn't appear verbatim in the buffer
    // but each word does.
    let mut highlights = Vec::new();
    let mut got_any = false;
    let buffer_text: String = snapshot
        .text_for_range(range_start_offset..range_end_offset)
        .collect();
    let mut buf_search_from = 0usize;
    let mut name_search_from = 0usize;
    for word in name.split_whitespace() {
        let name_word_start = name[name_search_from..]
            .find(word)
            .map(|pos| name_search_from + pos)
            .unwrap_or(name_search_from);
        if let Some(found_in_buf) = buffer_text[buf_search_from..].find(word) {
            let buf_word_start = range_start_offset + buf_search_from + found_in_buf;
            let buf_word_end = buf_word_start + word.len();
            let text_cursor = name_offset_in_text + name_word_start;
            if let Some(mut word_highlights) = highlights_for_range(
                text_cursor,
                buf_word_start,
                buf_word_end,
                snapshot,
                syntax_theme,
            ) {
                got_any = true;
                highlights.append(&mut word_highlights);
            }
            buf_search_from = buf_search_from + found_in_buf + word.len();
        }
        name_search_from = name_word_start + word.len();
    }

    got_any.then_some(highlights)
}

/// Extracts tree-sitter highlight styles from buffer chunks for the given
/// byte range, mapping them onto the outline item text starting at
/// `text_cursor_start`.
fn highlights_for_range(
    text_cursor_start: usize,
    buffer_start: usize,
    buffer_end: usize,
    snapshot: &BufferSnapshot,
    syntax_theme: &SyntaxTheme,
) -> Option<Vec<(std::ops::Range<usize>, HighlightStyle)>> {
    let mut highlights = Vec::new();
    let mut got_any = false;
    let mut text_cursor = text_cursor_start;
    let mut offset = buffer_start;
    for chunk in snapshot.chunks(buffer_start..buffer_end, true) {
        let chunk_len = chunk.text.len().min(buffer_end - offset);
        if let Some(style) = chunk
            .syntax_highlight_id
            .and_then(|id| id.style(syntax_theme))
        {
            highlights.push((text_cursor..text_cursor + chunk_len, style));
            got_any = true;
        }
        text_cursor += chunk_len;
        offset += chunk_len;
        if offset >= buffer_end {
            break;
        }
    }

    got_any.then_some(highlights)
}

fn document_symbols_to_outline_items(
    symbols_by_server: &HashMap<LanguageServerId, Vec<DocumentSymbol>>,
    snapshot: &BufferSnapshot,
    syntax_theme: Option<&SyntaxTheme>,
) -> HashMap<LanguageServerId, Vec<OutlineItem<Anchor>>> {
    symbols_by_server
        .iter()
        .map(|(&server_id, symbols)| {
            let mut items = Vec::new();
            flatten_document_symbols(symbols, snapshot, syntax_theme, 0, &mut items);
            (server_id, items)
        })
        .collect()
}

impl LspStore {
    /// Returns a task that resolves to the document symbol outline items for
    /// the given buffer.
    ///
    /// Caches results per buffer version so repeated calls for the same version
    /// return immediately. Deduplicates concurrent in-flight requests.
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
                return cx.background_spawn(async move { running.await.unwrap_or_default() });
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
                            let syntax_theme = cx.theme().syntax();
                            let converted = document_symbols_to_outline_items(
                                &fetched_symbols,
                                &snapshot,
                                Some(&syntax_theme),
                            );
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

        cx.background_spawn(async move { new_task.await.unwrap_or_default() })
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

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{Hsla, TestAppContext};
    use text::Unclipped;
    use theme::SyntaxTheme;

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

    fn test_syntax_theme() -> SyntaxTheme {
        SyntaxTheme::new_test([
            (
                "keyword",
                Hsla {
                    h: 0.0,
                    s: 1.0,
                    l: 0.5,
                    a: 1.0,
                },
            ),
            (
                "type",
                Hsla {
                    h: 0.1,
                    s: 1.0,
                    l: 0.5,
                    a: 1.0,
                },
            ),
            (
                "function",
                Hsla {
                    h: 0.2,
                    s: 1.0,
                    l: 0.5,
                    a: 1.0,
                },
            ),
            (
                "property",
                Hsla {
                    h: 0.3,
                    s: 1.0,
                    l: 0.5,
                    a: 1.0,
                },
            ),
            (
                "punctuation.bracket",
                Hsla {
                    h: 0.4,
                    s: 1.0,
                    l: 0.5,
                    a: 1.0,
                },
            ),
            (
                "lifetime",
                Hsla {
                    h: 0.5,
                    s: 1.0,
                    l: 0.5,
                    a: 1.0,
                },
            ),
        ])
    }

    fn highlighted_texts(item: &OutlineItem<Anchor>) -> Vec<&str> {
        item.highlight_ranges
            .iter()
            .map(|(range, _)| &item.text[range.clone()])
            .collect()
    }

    fn make_rust_buffer_and_theme(
        source: &str,
        cx: &mut TestAppContext,
    ) -> (gpui::Entity<Buffer>, SyntaxTheme) {
        let syntax_theme = test_syntax_theme();
        let lang = language::rust_lang();
        lang.set_theme(&syntax_theme);

        let buffer = cx.new(|cx| {
            let mut buffer = Buffer::local(source, cx);
            buffer.set_language(Some(lang), cx);
            buffer
        });
        (buffer, syntax_theme)
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
        flatten_document_symbols(&symbols, &snapshot, None, 0, &mut items);

        assert_eq!(items.len(), 5);

        assert_eq!(items[0].depth, 0);
        assert_eq!(items[0].text, "struct Foo");
        assert_eq!(items[0].name_ranges, vec![7..10]);

        assert_eq!(items[1].depth, 1);
        assert_eq!(items[1].text, "bar");
        assert_eq!(items[1].name_ranges, vec![0..3]);

        assert_eq!(items[2].depth, 1);
        assert_eq!(items[2].text, "baz");
        assert_eq!(items[2].name_ranges, vec![0..3]);

        assert_eq!(items[3].depth, 0);
        assert_eq!(items[3].text, "struct Foo");
        assert_eq!(items[3].name_ranges, vec![7..10]);

        assert_eq!(items[4].depth, 1);
        assert_eq!(items[4].text, "fn new");
        assert_eq!(items[4].name_ranges, vec![3..6]);
    }

    #[gpui::test]
    async fn test_symbol_kind_labels(cx: &mut TestAppContext) {
        let buffer = cx.new(|cx| Buffer::local("fn main() {}\n", cx));
        let snapshot = buffer.read_with(cx, |buffer, _| buffer.snapshot());

        let symbols = vec![make_symbol(
            "main",
            lsp::SymbolKind::FUNCTION,
            (0, 0)..(0, 13),
            (0, 3)..(0, 7),
            vec![],
        )];

        let mut items = Vec::new();
        flatten_document_symbols(&symbols, &snapshot, None, 0, &mut items);

        assert_eq!(items.len(), 1);
        assert_eq!(items[0].text, "fn main");
        assert_eq!(items[0].name_ranges, vec![3..7]);
    }

    #[gpui::test]
    async fn test_empty_symbols(cx: &mut TestAppContext) {
        let buffer = cx.new(|cx| Buffer::local("", cx));
        let snapshot = buffer.read_with(cx, |buffer, _| buffer.snapshot());

        let symbols: Vec<DocumentSymbol> = vec![];
        let mut items = Vec::new();
        flatten_document_symbols(&symbols, &snapshot, None, 0, &mut items);
        assert!(items.is_empty());
    }

    #[gpui::test]
    async fn test_highlight_ranges_from_buffer(cx: &mut TestAppContext) {
        let (buffer, syntax_theme) = make_rust_buffer_and_theme(
            concat!(
                "struct Foo {\n",                 // line 0
                "    bar: u32,\n",                // line 1
                "}\n",                            // line 2
                "\n",                             // line 3
                "impl<T> MyTrait<T> for Foo {\n", // line 4
                "    fn do_thing(&self) {}\n",    // line 5
                "}\n",                            // line 6
                "\n",                             // line 7
                "impl Foo {\n",                   // line 8
                "    fn simple(&self) {}\n",      // line 9
                "}\n",                            // line 10
            ),
            cx,
        );

        let snapshot = buffer.read_with(cx, |buffer, _| buffer.snapshot());

        let symbols = vec![
            make_symbol(
                "Foo",
                lsp::SymbolKind::STRUCT,
                (0, 0)..(2, 1),
                (0, 7)..(0, 10),
                vec![make_symbol(
                    "bar",
                    lsp::SymbolKind::FIELD,
                    (1, 4)..(1, 13),
                    (1, 4)..(1, 7),
                    vec![],
                )],
            ),
            // "impl MyTrait<T> for Foo" — name differs from buffer text
            // ("impl<T> MyTrait<T> for Foo"), but the selection range
            // covers "MyTrait<T> for Foo" which IS in the name, so the
            // fallback path should produce highlights for that part.
            make_symbol(
                "impl MyTrait<T> for Foo",
                lsp::SymbolKind::OBJECT,
                (4, 0)..(6, 1),
                (4, 8)..(4, 27),
                vec![make_symbol(
                    "do_thing",
                    lsp::SymbolKind::METHOD,
                    (5, 4)..(5, 26),
                    (5, 7)..(5, 15),
                    vec![],
                )],
            ),
            make_symbol(
                "impl Foo",
                lsp::SymbolKind::OBJECT,
                (8, 0)..(10, 1),
                (8, 5)..(8, 8),
                vec![make_symbol(
                    "simple",
                    lsp::SymbolKind::METHOD,
                    (9, 4)..(9, 26),
                    (9, 7)..(9, 13),
                    vec![],
                )],
            ),
        ];

        let mut items = Vec::new();
        flatten_document_symbols(&symbols, &snapshot, Some(&syntax_theme), 0, &mut items);

        assert_eq!(items.len(), 6);

        // "struct Foo": kind_label "struct" gets keyword highlight,
        // name "Foo" gets tree-sitter type highlight from buffer.
        let item = &items[0];
        assert_eq!(item.text, "struct Foo");
        let texts = highlighted_texts(item);
        assert!(
            texts.contains(&"struct"),
            "kind label 'struct' should be highlighted, got: {texts:?}"
        );
        assert!(
            texts.iter().any(|t| t.contains("Foo")),
            "'Foo' should be highlighted from buffer, got: {texts:?}"
        );

        // "bar": field with no kind_label, name highlighted from buffer
        let item = &items[1];
        assert_eq!(item.text, "bar");

        // "impl MyTrait<T> for Foo": name doesn't appear verbatim in buffer
        // ("impl<T> MyTrait<T> for Foo"), but word-by-word matching should
        // find each word (`impl`, `MyTrait<T>`, `for`, `Foo`) in the buffer
        // and produce highlights for all of them.
        let item = &items[2];
        assert_eq!(item.text, "impl MyTrait<T> for Foo");
        let texts = highlighted_texts(item);
        assert!(
            !texts.is_empty(),
            "Word-by-word fallback should produce highlights for 'impl MyTrait<T> for Foo', got none"
        );
        assert!(
            texts.iter().any(|t| t.contains("impl")),
            "'impl' keyword should be highlighted via word-by-word fallback, got: {texts:?}"
        );
        assert!(
            texts.iter().any(|t| t.contains("for")),
            "'for' keyword should be highlighted via word-by-word fallback, got: {texts:?}"
        );

        // "fn do_thing": kind_label "fn" + name from buffer
        let item = &items[3];
        assert_eq!(item.text, "fn do_thing");
        let texts = highlighted_texts(item);
        assert!(
            texts.contains(&"fn"),
            "kind label 'fn' should be highlighted, got: {texts:?}"
        );

        // "impl Foo": exact match in buffer, highlights from tree-sitter
        let item = &items[4];
        assert_eq!(item.text, "impl Foo");
        let texts = highlighted_texts(item);
        assert!(
            texts.contains(&"impl"),
            "'impl' should be highlighted, got: {texts:?}"
        );
        assert!(
            texts.iter().any(|t| t.contains("Foo")),
            "'Foo' should be highlighted, got: {texts:?}"
        );

        // "fn simple": kind_label "fn" + name from buffer
        let item = &items[5];
        assert_eq!(item.text, "fn simple");
        assert!(!item.highlight_ranges.is_empty());

        // Verify keyword colors are consistent across kind_label and
        // buffer highlights: "struct" from kind_label must use the same
        // style as "impl" pulled from tree-sitter chunks.
        let struct_keyword_style = items[0]
            .highlight_ranges
            .iter()
            .find(|(r, _)| &items[0].text[r.clone()] == "struct")
            .map(|(_, s)| s);
        let impl_item = &items[4];
        let impl_keyword_style = impl_item
            .highlight_ranges
            .iter()
            .find(|(r, _)| &impl_item.text[r.clone()] == "impl")
            .map(|(_, s)| s);
        assert_eq!(
            struct_keyword_style.and_then(|s| s.color),
            impl_keyword_style.and_then(|s| s.color),
            "Both 'struct' kind_label and 'impl' from buffer should use the keyword color"
        );
    }

    #[gpui::test]
    async fn test_highlights_match_tree_sitter_outline(cx: &mut TestAppContext) {
        let source = concat!(
            "struct Foo {\n",
            "    bar: u32,\n",
            "}\n",
            "\n",
            "impl Foo {\n",
            "    fn new() -> Self {\n",
            "        Foo { bar: 0 }\n",
            "    }\n",
            "}\n",
        );
        let (buffer, syntax_theme) = make_rust_buffer_and_theme(source, cx);
        let snapshot = buffer.read_with(cx, |buffer, _| buffer.snapshot());

        // Tree-sitter outline — the ground truth for editor highlights.
        let ts_outline = snapshot.outline(Some(&syntax_theme));
        let ts_items: Vec<_> = ts_outline
            .items
            .iter()
            .map(|item| (item.text.as_str(), &item.highlight_ranges))
            .collect();

        // LSP document symbols for the same code.
        let symbols = vec![
            make_symbol(
                "Foo",
                lsp::SymbolKind::STRUCT,
                (0, 0)..(2, 1),
                (0, 7)..(0, 10),
                vec![make_symbol(
                    "bar",
                    lsp::SymbolKind::FIELD,
                    (1, 4)..(1, 13),
                    (1, 4)..(1, 7),
                    vec![],
                )],
            ),
            make_symbol(
                "Foo",
                lsp::SymbolKind::STRUCT,
                (4, 0)..(8, 1),
                (4, 5)..(4, 8),
                vec![make_symbol(
                    "new",
                    lsp::SymbolKind::FUNCTION,
                    (5, 4)..(7, 5),
                    (5, 7)..(5, 10),
                    vec![],
                )],
            ),
        ];

        let mut lsp_items = Vec::new();
        flatten_document_symbols(&symbols, &snapshot, Some(&syntax_theme), 0, &mut lsp_items);

        // Both should produce text with kind prefix.
        let lsp_texts: Vec<&str> = lsp_items.iter().map(|i| i.text.as_str()).collect();
        assert!(
            lsp_texts.contains(&"struct Foo"),
            "LSP items should contain 'struct Foo', got: {lsp_texts:?}"
        );
        assert!(
            lsp_texts.contains(&"fn new"),
            "LSP items should contain 'fn new', got: {lsp_texts:?}"
        );

        // For items that appear in both outlines, the keyword highlight
        // color should match: the kind_label "struct" should use the same
        // color tree-sitter assigns to "struct" in the buffer.
        for lsp_item in &lsp_items {
            if let Some(ts_item) = ts_items.iter().find(|(text, _)| *text == lsp_item.text) {
                let lsp_keyword_color =
                    lsp_item.highlight_ranges.first().and_then(|(_, s)| s.color);
                let ts_keyword_color = ts_item.1.first().and_then(|(_, s)| s.color);
                assert_eq!(
                    lsp_keyword_color, ts_keyword_color,
                    "Keyword color mismatch for '{}': LSP={:?} vs TS={:?}",
                    lsp_item.text, lsp_keyword_color, ts_keyword_color,
                );
            }
        }
    }

    #[gpui::test]
    async fn test_impl_with_generics_highlights(cx: &mut TestAppContext) {
        let source = concat!(
            "pub enum LazyProperty<T> {\n",                   // line 0
            "    Computed(T),\n",                             // line 1
            "    Lazy,\n",                                    // line 2
            "}\n",                                            // line 3
            "\n",                                             // line 4
            "impl<T> LazyProperty<T> {\n",                    // line 5
            "    pub fn computed(self) {}\n",                 // line 6
            "}\n",                                            // line 7
            "\n",                                             // line 8
            "impl std::hash::Hash for LazyProperty<u32> {\n", // line 9
            "    fn hash(&self) {}\n",                        // line 10
            "}\n",                                            // line 11
        );
        let (buffer, syntax_theme) = make_rust_buffer_and_theme(source, cx);
        let snapshot = buffer.read_with(cx, |buffer, _| buffer.snapshot());

        // rust-analyzer drops `<T>` from `impl<T>` in the symbol name
        let symbols = vec![
            make_symbol(
                "LazyProperty",
                lsp::SymbolKind::ENUM,
                (0, 0)..(3, 1),
                (0, 9)..(0, 21),
                vec![],
            ),
            make_symbol(
                "impl LazyProperty<T>",
                lsp::SymbolKind::OBJECT,
                (5, 0)..(7, 1),
                (5, 8)..(5, 24),
                vec![make_symbol(
                    "computed",
                    lsp::SymbolKind::METHOD,
                    (6, 4)..(6, 28),
                    (6, 11)..(6, 19),
                    vec![],
                )],
            ),
            make_symbol(
                "impl Hash for LazyProperty<u32>",
                lsp::SymbolKind::OBJECT,
                (9, 0)..(11, 1),
                (9, 5)..(9, 20),
                vec![make_symbol(
                    "hash",
                    lsp::SymbolKind::METHOD,
                    (10, 4)..(10, 22),
                    (10, 7)..(10, 11),
                    vec![],
                )],
            ),
        ];

        let mut items = Vec::new();
        flatten_document_symbols(&symbols, &snapshot, Some(&syntax_theme), 0, &mut items);

        let texts: Vec<&str> = items.iter().map(|i| i.text.as_str()).collect();
        assert_eq!(
            texts,
            vec![
                "enum LazyProperty",
                "impl LazyProperty<T>",
                "fn computed",
                "impl Hash for LazyProperty<u32>",
                "fn hash",
            ]
        );

        // `impl LazyProperty<T>`: buffer has `impl<T> LazyProperty<T>`,
        // name has `impl LazyProperty<T>`. Verbatim match fails because
        // of the extra `<T>` after `impl`. Word-by-word fallback should
        // still highlight both `impl` (keyword) and `LazyProperty` (type).
        let impl_item = &items[1];
        let impl_texts = highlighted_texts(impl_item);
        assert!(
            impl_texts.iter().any(|t| t.contains("impl")),
            "'impl' should be highlighted for 'impl LazyProperty<T>', got: {impl_texts:?}"
        );

        // `impl Hash for LazyProperty<u32>`: buffer has
        // `impl std::hash::Hash for LazyProperty<u32>`, LSP name drops
        // the path prefix. Word-by-word should still find `impl`, `Hash`,
        // `for` keywords in the buffer.
        let trait_impl_item = &items[3];
        let trait_impl_texts = highlighted_texts(trait_impl_item);
        assert!(
            trait_impl_texts.iter().any(|t| t.contains("impl")),
            "'impl' should be highlighted for trait impl, got: {trait_impl_texts:?}"
        );
        assert!(
            trait_impl_texts.iter().any(|t| t.contains("for")),
            "'for' should be highlighted for trait impl, got: {trait_impl_texts:?}"
        );
    }
}
