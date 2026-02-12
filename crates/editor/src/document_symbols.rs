use std::ops::Range;

use collections::HashMap;
use futures::FutureExt;
use futures::future::join_all;
use gpui::{App, Context, HighlightStyle, Task};
use itertools::Itertools as _;
use language::language_settings::language_settings;
use language::{Buffer, BufferSnapshot, OutlineItem};
use multi_buffer::{Anchor, MultiBufferSnapshot};
use text::{BufferId, OffsetRangeExt as _, ToOffset as _};
use theme::{ActiveTheme as _, SyntaxTheme};

use crate::display_map::DisplaySnapshot;
use crate::{Editor, LSP_REQUEST_DEBOUNCE_TIMEOUT};

impl Editor {
    /// Returns all document outline items for a buffer, using LSP or
    /// tree-sitter based on the `document_symbols` setting.
    /// External consumers (outline modal, outline panel, breadcrumbs) should use this.
    pub fn buffer_outline_items(
        &self,
        buffer_id: BufferId,
        cx: &mut Context<Self>,
    ) -> Task<Vec<OutlineItem<text::Anchor>>> {
        let Some(buffer) = self.buffer.read(cx).buffer(buffer_id) else {
            return Task::ready(Vec::new());
        };

        if lsp_symbols_enabled(buffer.read(cx), cx) {
            let refresh_task = self.refresh_document_symbols_task.clone();
            cx.spawn(async move |editor, cx| {
                refresh_task.await;
                editor
                    .read_with(cx, |editor, _| {
                        editor
                            .lsp_document_symbols
                            .get(&buffer_id)
                            .cloned()
                            .unwrap_or_default()
                    })
                    .ok()
                    .unwrap_or_default()
            })
        } else {
            let buffer_snapshot = buffer.read(cx).snapshot();
            let syntax = cx.theme().syntax().clone();
            cx.background_executor()
                .spawn(async move { buffer_snapshot.outline(Some(&syntax)).items })
        }
    }

    /// Whether the buffer at `cursor` has LSP document symbols enabled.
    pub(super) fn uses_lsp_document_symbols(
        &self,
        cursor: Anchor,
        multi_buffer_snapshot: &MultiBufferSnapshot,
        cx: &Context<Self>,
    ) -> bool {
        let Some(excerpt) = multi_buffer_snapshot.excerpt_containing(cursor..cursor) else {
            return false;
        };
        let Some(buffer) = self.buffer.read(cx).buffer(excerpt.buffer_id()) else {
            return false;
        };
        lsp_symbols_enabled(buffer.read(cx), cx)
    }

    /// Filters editor-local LSP document symbols to the ancestor chain
    /// containing `cursor`. Never triggers an LSP request.
    pub(super) fn lsp_symbols_at_cursor(
        &self,
        cursor: Anchor,
        multi_buffer_snapshot: &MultiBufferSnapshot,
        cx: &Context<Self>,
    ) -> Option<(BufferId, Vec<OutlineItem<Anchor>>)> {
        let excerpt = multi_buffer_snapshot.excerpt_containing(cursor..cursor)?;
        let excerpt_id = excerpt.id();
        let buffer_id = excerpt.buffer_id();
        let buffer = self.buffer.read(cx).buffer(buffer_id)?;
        let buffer_snapshot = buffer.read(cx).snapshot();
        let cursor_text_anchor = cursor.text_anchor;

        let all_items = self.lsp_document_symbols.get(&buffer_id)?;
        if all_items.is_empty() {
            return None;
        }

        let mut symbols = all_items
            .iter()
            .filter(|item| {
                item.range
                    .start
                    .cmp(&cursor_text_anchor, &buffer_snapshot)
                    .is_le()
                    && item
                        .range
                        .end
                        .cmp(&cursor_text_anchor, &buffer_snapshot)
                        .is_ge()
            })
            .map(|item| OutlineItem {
                depth: item.depth,
                range: Anchor::range_in_buffer(excerpt_id, item.range.clone()),
                source_range_for_text: Anchor::range_in_buffer(
                    excerpt_id,
                    item.source_range_for_text.clone(),
                ),
                text: item.text.clone(),
                highlight_ranges: item.highlight_ranges.clone(),
                name_ranges: item.name_ranges.clone(),
                body_range: item
                    .body_range
                    .as_ref()
                    .map(|r| Anchor::range_in_buffer(excerpt_id, r.clone())),
                annotation_range: item
                    .annotation_range
                    .as_ref()
                    .map(|r| Anchor::range_in_buffer(excerpt_id, r.clone())),
            })
            .collect::<Vec<_>>();

        let mut prev_depth = None;
        symbols.retain(|item| {
            let retain = prev_depth.is_none_or(|prev_depth| item.depth > prev_depth);
            prev_depth = Some(item.depth);
            retain
        });

        Some((buffer_id, symbols))
    }

    /// Fetches document symbols from the LSP for buffers that have the setting
    /// enabled. Called from `update_lsp_data` on edits, server events, etc.
    /// When the fetch completes, stores results in `self.lsp_document_symbols`
    /// and triggers `refresh_outline_symbols_at_cursor` so breadcrumbs pick up the new data.
    pub(super) fn refresh_document_symbols(
        &mut self,
        for_buffer: Option<BufferId>,
        cx: &mut Context<Self>,
    ) {
        if !self.mode().is_full() {
            return;
        }
        let Some(project) = self.project.clone() else {
            return;
        };

        let buffers_to_query = self
            .visible_excerpts(true, cx)
            .into_iter()
            .filter_map(|(_, (buffer, _, _))| {
                let id = buffer.read(cx).remote_id();
                if for_buffer.is_none_or(|target| target == id)
                    && lsp_symbols_enabled(buffer.read(cx), cx)
                {
                    Some(buffer)
                } else {
                    None
                }
            })
            .unique_by(|buffer| buffer.read(cx).remote_id())
            .collect::<Vec<_>>();

        let mut symbols_altered = false;
        let multi_buffer = self.buffer().clone();
        self.lsp_document_symbols.retain(|buffer_id, _| {
            let Some(buffer) = multi_buffer.read(cx).buffer(*buffer_id) else {
                symbols_altered = true;
                return false;
            };
            let retain = lsp_symbols_enabled(buffer.read(cx), cx);
            symbols_altered |= !retain;
            retain
        });
        if symbols_altered {
            self.refresh_outline_symbols_at_cursor(cx);
        }

        if buffers_to_query.is_empty() {
            return;
        }

        self.refresh_document_symbols_task = cx
            .spawn(async move |editor, cx| {
                cx.background_executor()
                    .timer(LSP_REQUEST_DEBOUNCE_TIMEOUT)
                    .await;

                let Some(tasks) = editor
                    .update(cx, |_, cx| {
                        project.read(cx).lsp_store().update(cx, |lsp_store, cx| {
                            buffers_to_query
                                .into_iter()
                                .map(|buffer| {
                                    let buffer_id = buffer.read(cx).remote_id();
                                    let task = lsp_store.fetch_document_symbols(&buffer, cx);
                                    async move { (buffer_id, task.await) }
                                })
                                .collect::<Vec<_>>()
                        })
                    })
                    .ok()
                else {
                    return;
                };

                let results = join_all(tasks).await.into_iter().collect::<HashMap<_, _>>();
                editor
                    .update(cx, |editor, cx| {
                        let syntax = cx.theme().syntax().clone();
                        let display_snapshot =
                            editor.display_map.update(cx, |map, cx| map.snapshot(cx));
                        let mut highlighted_results = results;
                        for (buffer_id, items) in &mut highlighted_results {
                            if let Some(buffer) = editor.buffer.read(cx).buffer(*buffer_id) {
                                let snapshot = buffer.read(cx).snapshot();
                                apply_highlights(
                                    items,
                                    *buffer_id,
                                    &snapshot,
                                    &display_snapshot,
                                    &syntax,
                                );
                            }
                        }
                        editor.lsp_document_symbols.extend(highlighted_results);
                        editor.refresh_outline_symbols_at_cursor(cx);
                    })
                    .ok();
            })
            .shared();
    }
}

fn lsp_symbols_enabled(buffer: &Buffer, cx: &App) -> bool {
    language_settings(buffer.language().map(|l| l.name()), buffer.file(), cx)
        .document_symbols
        .lsp_enabled()
}

/// Applies combined syntax + semantic token highlights to LSP document symbol
/// outline items that were built without highlights by the project layer.
fn apply_highlights(
    items: &mut [OutlineItem<text::Anchor>],
    buffer_id: BufferId,
    buffer_snapshot: &BufferSnapshot,
    display_snapshot: &DisplaySnapshot,
    syntax_theme: &SyntaxTheme,
) {
    for item in items {
        let symbol_range = item.range.to_offset(buffer_snapshot);
        let selection_start = item.source_range_for_text.start.to_offset(buffer_snapshot);

        if let Some(highlights) = highlights_from_buffer(
            &item.text,
            0,
            buffer_id,
            buffer_snapshot,
            display_snapshot,
            symbol_range,
            selection_start,
            syntax_theme,
        ) {
            item.highlight_ranges = highlights;
        }
    }
}

/// Finds where the symbol name appears in the buffer and returns combined
/// (tree-sitter + semantic token) highlights for those positions.
///
/// First tries to find the name verbatim near the selection range so that
/// complex names (`impl Trait for Type`) get full highlighting. Falls back
/// to word-by-word matching for cases like `impl<T> Trait<T> for Type`
/// where the LSP name doesn't appear verbatim in the buffer.
fn highlights_from_buffer(
    name: &str,
    name_offset_in_text: usize,
    buffer_id: BufferId,
    buffer_snapshot: &BufferSnapshot,
    display_snapshot: &DisplaySnapshot,
    symbol_range: Range<usize>,
    selection_start_offset: usize,
    syntax_theme: &SyntaxTheme,
) -> Option<Vec<(Range<usize>, HighlightStyle)>> {
    if name.is_empty() {
        return None;
    }

    let range_start_offset = symbol_range.start;
    let range_end_offset = symbol_range.end;

    // Try to find the name verbatim in the buffer near the selection range.
    let search_start = selection_start_offset
        .saturating_sub(name.len())
        .max(range_start_offset);
    let search_end = (selection_start_offset + name.len() * 2).min(range_end_offset);

    if search_start < search_end {
        let buffer_text: String = buffer_snapshot
            .text_for_range(search_start..search_end)
            .collect();
        if let Some(found_at) = buffer_text.find(name) {
            let name_start_offset = search_start + found_at;
            let name_end_offset = name_start_offset + name.len();
            let result = highlights_for_buffer_range(
                name_offset_in_text,
                name_start_offset..name_end_offset,
                buffer_id,
                display_snapshot,
                syntax_theme,
            );
            if result.is_some() {
                return result;
            }
        }
    }

    // Fallback: match word-by-word. Split the name on whitespace and find
    // each word sequentially in the buffer's symbol range.
    let mut highlights = Vec::new();
    let mut got_any = false;
    let buffer_text: String = buffer_snapshot
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
            if let Some(mut word_highlights) = highlights_for_buffer_range(
                text_cursor,
                buf_word_start..buf_word_end,
                buffer_id,
                display_snapshot,
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

/// Gets combined (tree-sitter + semantic token) highlights for a buffer byte
/// range via the editor's display snapshot, then shifts the returned ranges
/// so they start at `text_cursor_start` (the position in the outline item text).
fn highlights_for_buffer_range(
    text_cursor_start: usize,
    buffer_range: Range<usize>,
    buffer_id: BufferId,
    display_snapshot: &DisplaySnapshot,
    syntax_theme: &SyntaxTheme,
) -> Option<Vec<(Range<usize>, HighlightStyle)>> {
    let raw = display_snapshot.combined_highlights(buffer_id, buffer_range, syntax_theme);
    if raw.is_empty() {
        return None;
    }
    Some(
        raw.into_iter()
            .map(|(range, style)| {
                (
                    range.start + text_cursor_start..range.end + text_cursor_start,
                    style,
                )
            })
            .collect(),
    )
}

#[cfg(test)]
mod tests {
    use std::{
        sync::{Arc, atomic},
        time::Duration,
    };

    use futures::StreamExt as _;
    use gpui::TestAppContext;
    use settings::DocumentSymbols;
    use util::path;
    use zed_actions::editor::{MoveDown, MoveUp};

    use crate::{
        Editor, LSP_REQUEST_DEBOUNCE_TIMEOUT,
        editor_tests::{init_test, update_test_language_settings},
        test::editor_lsp_test_context::EditorLspTestContext,
    };

    fn outline_symbol_names(editor: &Editor) -> Vec<&str> {
        editor
            .outline_symbols_at_cursor
            .as_ref()
            .expect("Should have outline symbols")
            .1
            .iter()
            .map(|s| s.text.as_str())
            .collect()
    }

    fn lsp_range(start_line: u32, start_char: u32, end_line: u32, end_char: u32) -> lsp::Range {
        lsp::Range {
            start: lsp::Position::new(start_line, start_char),
            end: lsp::Position::new(end_line, end_char),
        }
    }

    fn nested_symbol(
        name: &str,
        kind: lsp::SymbolKind,
        range: lsp::Range,
        selection_range: lsp::Range,
        children: Vec<lsp::DocumentSymbol>,
    ) -> lsp::DocumentSymbol {
        #[allow(deprecated)]
        lsp::DocumentSymbol {
            name: name.to_string(),
            detail: None,
            kind,
            tags: None,
            deprecated: None,
            range,
            selection_range,
            children: if children.is_empty() {
                None
            } else {
                Some(children)
            },
        }
    }

    #[gpui::test]
    async fn test_lsp_document_symbols_fetches_when_enabled(cx: &mut TestAppContext) {
        init_test(cx, |_| {});

        update_test_language_settings(cx, |settings| {
            settings.defaults.document_symbols = Some(DocumentSymbols::On);
        });

        let mut cx = EditorLspTestContext::new_rust(
            lsp::ServerCapabilities {
                document_symbol_provider: Some(lsp::OneOf::Left(true)),
                ..lsp::ServerCapabilities::default()
            },
            cx,
        )
        .await;
        let mut symbol_request = cx
            .set_request_handler::<lsp::request::DocumentSymbolRequest, _, _>(
                move |_, _, _| async move {
                    Ok(Some(lsp::DocumentSymbolResponse::Nested(vec![
                        nested_symbol(
                            "main",
                            lsp::SymbolKind::FUNCTION,
                            lsp_range(0, 0, 2, 1),
                            lsp_range(0, 3, 0, 7),
                            Vec::new(),
                        ),
                    ])))
                },
            );

        cx.set_state("fn maˇin() {\n    let x = 1;\n}\n");
        assert!(symbol_request.next().await.is_some());
        cx.run_until_parked();

        cx.update_editor(|editor, _window, _cx| {
            assert_eq!(outline_symbol_names(editor), vec!["main"]);
        });
    }

    #[gpui::test]
    async fn test_lsp_document_symbols_nested(cx: &mut TestAppContext) {
        init_test(cx, |_| {});

        update_test_language_settings(cx, |settings| {
            settings.defaults.document_symbols = Some(DocumentSymbols::On);
        });

        let mut cx = EditorLspTestContext::new_rust(
            lsp::ServerCapabilities {
                document_symbol_provider: Some(lsp::OneOf::Left(true)),
                ..lsp::ServerCapabilities::default()
            },
            cx,
        )
        .await;
        let mut symbol_request = cx
            .set_request_handler::<lsp::request::DocumentSymbolRequest, _, _>(
                move |_, _, _| async move {
                    Ok(Some(lsp::DocumentSymbolResponse::Nested(vec![
                        nested_symbol(
                            "Foo",
                            lsp::SymbolKind::STRUCT,
                            lsp_range(0, 0, 3, 1),
                            lsp_range(0, 7, 0, 10),
                            vec![
                                nested_symbol(
                                    "bar",
                                    lsp::SymbolKind::FIELD,
                                    lsp_range(1, 4, 1, 13),
                                    lsp_range(1, 4, 1, 7),
                                    Vec::new(),
                                ),
                                nested_symbol(
                                    "baz",
                                    lsp::SymbolKind::FIELD,
                                    lsp_range(2, 4, 2, 15),
                                    lsp_range(2, 4, 2, 7),
                                    Vec::new(),
                                ),
                            ],
                        ),
                    ])))
                },
            );

        cx.set_state("struct Foo {\n    baˇr: u32,\n    baz: String,\n}\n");
        assert!(symbol_request.next().await.is_some());
        cx.run_until_parked();

        cx.update_editor(|editor, _window, _cx| {
            assert_eq!(
                outline_symbol_names(editor),
                vec!["Foo", "bar"],
                "cursor is inside Foo > bar, so we expect the containing chain"
            );
        });
    }

    #[gpui::test]
    async fn test_lsp_document_symbols_switch_tree_sitter_to_lsp_and_back(cx: &mut TestAppContext) {
        init_test(cx, |_| {});

        // Start with tree-sitter (default)
        let mut cx = EditorLspTestContext::new_rust(
            lsp::ServerCapabilities {
                document_symbol_provider: Some(lsp::OneOf::Left(true)),
                ..lsp::ServerCapabilities::default()
            },
            cx,
        )
        .await;
        let mut symbol_request = cx
            .set_request_handler::<lsp::request::DocumentSymbolRequest, _, _>(
                move |_, _, _| async move {
                    Ok(Some(lsp::DocumentSymbolResponse::Nested(vec![
                        nested_symbol(
                            "lsp_main_symbol",
                            lsp::SymbolKind::FUNCTION,
                            lsp_range(0, 0, 2, 1),
                            lsp_range(0, 3, 0, 7),
                            Vec::new(),
                        ),
                    ])))
                },
            );

        cx.set_state("fn maˇin() {\n    let x = 1;\n}\n");
        cx.run_until_parked();

        // Step 1: With tree-sitter (default), breadcrumbs use tree-sitter outline
        cx.update_editor(|editor, _window, _cx| {
            assert_eq!(
                outline_symbol_names(editor),
                vec!["fn main"],
                "Tree-sitter should produce 'fn main'"
            );
        });

        // Step 2: Switch to LSP
        update_test_language_settings(&mut cx.cx.cx, |settings| {
            settings.defaults.document_symbols = Some(DocumentSymbols::On);
        });
        assert!(symbol_request.next().await.is_some());
        cx.run_until_parked();

        cx.update_editor(|editor, _window, _cx| {
            assert_eq!(
                outline_symbol_names(editor),
                vec!["lsp_main_symbol"],
                "After switching to LSP, should see LSP symbols"
            );
        });

        // Step 3: Switch back to tree-sitter
        update_test_language_settings(&mut cx.cx.cx, |settings| {
            settings.defaults.document_symbols = Some(DocumentSymbols::Off);
        });
        cx.run_until_parked();

        // Force another selection change
        cx.update_editor(|editor, window, cx| {
            editor.move_up(&MoveUp, window, cx);
        });
        cx.run_until_parked();

        cx.update_editor(|editor, _window, _cx| {
            assert_eq!(
                outline_symbol_names(editor),
                vec!["fn main"],
                "After switching back to tree-sitter, should see tree-sitter symbols again"
            );
        });
    }

    #[gpui::test]
    async fn test_lsp_document_symbols_caches_results(cx: &mut TestAppContext) {
        init_test(cx, |_| {});

        update_test_language_settings(cx, |settings| {
            settings.defaults.document_symbols = Some(DocumentSymbols::On);
        });

        let request_count = Arc::new(atomic::AtomicUsize::new(0));
        let request_count_clone = request_count.clone();

        let mut cx = EditorLspTestContext::new_rust(
            lsp::ServerCapabilities {
                document_symbol_provider: Some(lsp::OneOf::Left(true)),
                ..lsp::ServerCapabilities::default()
            },
            cx,
        )
        .await;

        let mut symbol_request = cx
            .set_request_handler::<lsp::request::DocumentSymbolRequest, _, _>(move |_, _, _| {
                request_count_clone.fetch_add(1, atomic::Ordering::AcqRel);
                async move {
                    Ok(Some(lsp::DocumentSymbolResponse::Nested(vec![
                        nested_symbol(
                            "main",
                            lsp::SymbolKind::FUNCTION,
                            lsp_range(0, 0, 2, 1),
                            lsp_range(0, 3, 0, 7),
                            Vec::new(),
                        ),
                    ])))
                }
            });

        cx.set_state("fn maˇin() {\n    let x = 1;\n}\n");
        assert!(symbol_request.next().await.is_some());
        cx.run_until_parked();

        let first_count = request_count.load(atomic::Ordering::Acquire);
        assert_eq!(first_count, 1, "Should have made exactly one request");

        // Move cursor within the same buffer version — should use cache
        cx.update_editor(|editor, window, cx| {
            editor.move_down(&MoveDown, window, cx);
        });
        cx.background_executor
            .advance_clock(LSP_REQUEST_DEBOUNCE_TIMEOUT + Duration::from_millis(100));
        cx.run_until_parked();

        assert_eq!(
            first_count,
            request_count.load(atomic::Ordering::Acquire),
            "Moving cursor without editing should use cached symbols"
        );
    }

    #[gpui::test]
    async fn test_lsp_document_symbols_flat_response(cx: &mut TestAppContext) {
        init_test(cx, |_| {});

        update_test_language_settings(cx, |settings| {
            settings.defaults.document_symbols = Some(DocumentSymbols::On);
        });

        let mut cx = EditorLspTestContext::new_rust(
            lsp::ServerCapabilities {
                document_symbol_provider: Some(lsp::OneOf::Left(true)),
                ..lsp::ServerCapabilities::default()
            },
            cx,
        )
        .await;
        let mut symbol_request = cx
            .set_request_handler::<lsp::request::DocumentSymbolRequest, _, _>(
                move |_, _, _| async move {
                    #[allow(deprecated)]
                    Ok(Some(lsp::DocumentSymbolResponse::Flat(vec![
                        lsp::SymbolInformation {
                            name: "main".to_string(),
                            kind: lsp::SymbolKind::FUNCTION,
                            tags: None,
                            deprecated: None,
                            location: lsp::Location {
                                uri: lsp::Uri::from_file_path(path!("/a/main.rs")).unwrap(),
                                range: lsp_range(0, 0, 2, 1),
                            },
                            container_name: None,
                        },
                    ])))
                },
            );

        cx.set_state("fn maˇin() {\n    let x = 1;\n}\n");
        assert!(symbol_request.next().await.is_some());
        cx.run_until_parked();

        cx.update_editor(|editor, _window, _cx| {
            assert_eq!(outline_symbol_names(editor), vec!["main"]);
        });
    }

    #[gpui::test]
    async fn test_breadcrumbs_use_lsp_symbols(cx: &mut TestAppContext) {
        init_test(cx, |_| {});

        update_test_language_settings(cx, |settings| {
            settings.defaults.document_symbols = Some(DocumentSymbols::On);
        });

        let mut cx = EditorLspTestContext::new_rust(
            lsp::ServerCapabilities {
                document_symbol_provider: Some(lsp::OneOf::Left(true)),
                ..lsp::ServerCapabilities::default()
            },
            cx,
        )
        .await;
        let mut symbol_request = cx
            .set_request_handler::<lsp::request::DocumentSymbolRequest, _, _>(
                move |_, _, _| async move {
                    Ok(Some(lsp::DocumentSymbolResponse::Nested(vec![
                        nested_symbol(
                            "MyModule",
                            lsp::SymbolKind::MODULE,
                            lsp_range(0, 0, 4, 1),
                            lsp_range(0, 4, 0, 12),
                            vec![nested_symbol(
                                "my_function",
                                lsp::SymbolKind::FUNCTION,
                                lsp_range(1, 4, 3, 5),
                                lsp_range(1, 7, 1, 18),
                                Vec::new(),
                            )],
                        ),
                    ])))
                },
            );

        cx.set_state("mod MyModule {\n    fn my_fuˇnction() {\n        let x = 1;\n    }\n}\n");
        assert!(symbol_request.next().await.is_some());
        cx.run_until_parked();

        cx.update_editor(|editor, _window, _cx| {
            assert_eq!(
                outline_symbol_names(editor),
                vec!["MyModule", "my_function"]
            );
        });
    }

    #[gpui::test]
    async fn test_lsp_document_symbols_empty_response(cx: &mut TestAppContext) {
        init_test(cx, |_| {});

        update_test_language_settings(cx, |settings| {
            settings.defaults.document_symbols = Some(DocumentSymbols::On);
        });

        let mut cx = EditorLspTestContext::new_rust(
            lsp::ServerCapabilities {
                document_symbol_provider: Some(lsp::OneOf::Left(true)),
                ..lsp::ServerCapabilities::default()
            },
            cx,
        )
        .await;
        let mut symbol_request = cx
            .set_request_handler::<lsp::request::DocumentSymbolRequest, _, _>(
                move |_, _, _| async move {
                    Ok(Some(lsp::DocumentSymbolResponse::Nested(Vec::new())))
                },
            );

        cx.set_state("fn maˇin() {\n    let x = 1;\n}\n");
        assert!(symbol_request.next().await.is_some());
        cx.run_until_parked();
        cx.update_editor(|editor, _window, _cx| {
            // With LSP enabled but empty response, outline_symbols_at_cursor should be None
            // (no symbols to show in breadcrumbs)
            assert!(
                editor.outline_symbols_at_cursor.is_none(),
                "Empty LSP response should result in no outline symbols"
            );
        });
    }

    #[gpui::test]
    async fn test_lsp_document_symbols_disabled_by_default(cx: &mut TestAppContext) {
        init_test(cx, |_| {});

        let request_count = Arc::new(atomic::AtomicUsize::new(0));
        // Do NOT enable document_symbols — defaults to Off
        let mut cx = EditorLspTestContext::new_rust(
            lsp::ServerCapabilities {
                document_symbol_provider: Some(lsp::OneOf::Left(true)),
                ..lsp::ServerCapabilities::default()
            },
            cx,
        )
        .await;
        let request_count_clone = request_count.clone();
        let _symbol_request =
            cx.set_request_handler::<lsp::request::DocumentSymbolRequest, _, _>(move |_, _, _| {
                request_count_clone.fetch_add(1, atomic::Ordering::AcqRel);
                async move {
                    Ok(Some(lsp::DocumentSymbolResponse::Nested(vec![
                        nested_symbol(
                            "should_not_appear",
                            lsp::SymbolKind::FUNCTION,
                            lsp_range(0, 0, 2, 1),
                            lsp_range(0, 3, 0, 7),
                            Vec::new(),
                        ),
                    ])))
                }
            });

        cx.set_state("fn maˇin() {\n    let x = 1;\n}\n");
        cx.run_until_parked();

        // Tree-sitter should be used instead
        cx.update_editor(|editor, _window, _cx| {
            assert_eq!(
                outline_symbol_names(editor),
                vec!["fn main"],
                "With document_symbols off, should use tree-sitter"
            );
        });

        assert_eq!(
            request_count.load(atomic::Ordering::Acquire),
            0,
            "Should not have made any LSP document symbol requests when setting is off"
        );
    }
}
