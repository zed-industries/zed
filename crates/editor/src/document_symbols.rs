use std::ops::Range;

use collections::HashMap;
use futures::FutureExt;
use futures::future::join_all;
use gpui::{App, Context, HighlightStyle, Task};
use itertools::Itertools as _;
use language::language_settings::language_settings;
use language::{Buffer, OutlineItem};
use multi_buffer::{
    Anchor, AnchorRangeExt as _, MultiBufferOffset, MultiBufferRow, MultiBufferSnapshot,
    ToOffset as _,
};
use text::BufferId;
use theme::{ActiveTheme as _, SyntaxTheme};
use unicode_segmentation::UnicodeSegmentation as _;
use util::maybe;

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
        if Some(buffer_id) != cursor.text_anchor.buffer_id {
            return None;
        }
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
        if !self.lsp_data_enabled() {
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
                        for items in highlighted_results.values_mut() {
                            for item in items {
                                if let Some(highlights) =
                                    highlights_from_buffer(&display_snapshot, &item, &syntax)
                                {
                                    item.highlight_ranges = highlights;
                                }
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

/// Finds where the symbol name appears in the buffer and returns combined
/// (tree-sitter + semantic token) highlights for those positions.
///
/// First tries to find the name verbatim near the selection range so that
/// complex names (`impl Trait for Type`) get full highlighting. Falls back
/// to word-by-word matching for cases like `impl<T> Trait<T> for Type`
/// where the LSP name doesn't appear verbatim in the buffer.
fn highlights_from_buffer(
    display_snapshot: &DisplaySnapshot,
    item: &OutlineItem<text::Anchor>,
    syntax_theme: &SyntaxTheme,
) -> Option<Vec<(Range<usize>, HighlightStyle)>> {
    let outline_text = &item.text;
    if outline_text.is_empty() {
        return None;
    }

    let multi_buffer_snapshot = display_snapshot.buffer();
    let multi_buffer_source_range_anchors =
        multi_buffer_snapshot.text_anchors_to_visible_anchors([
            item.source_range_for_text.start,
            item.source_range_for_text.end,
        ]);
    let Some(anchor_range) = maybe!({
        Some(
            (*multi_buffer_source_range_anchors.get(0)?)?
                ..(*multi_buffer_source_range_anchors.get(1)?)?,
        )
    }) else {
        return None;
    };

    let selection_point_range = anchor_range.to_point(multi_buffer_snapshot);
    let mut search_start = selection_point_range.start;
    search_start.column = 0;
    let search_start_offset = search_start.to_offset(&multi_buffer_snapshot);
    let mut search_end = selection_point_range.end;
    search_end.column = multi_buffer_snapshot.line_len(MultiBufferRow(search_end.row));

    let search_text = multi_buffer_snapshot
        .text_for_range(search_start..search_end)
        .collect::<String>();

    let mut outline_text_highlights = Vec::new();
    match search_text.find(outline_text) {
        Some(start_index) => {
            let multibuffer_start = search_start_offset + MultiBufferOffset(start_index);
            let multibuffer_end = multibuffer_start + MultiBufferOffset(outline_text.len());
            outline_text_highlights.extend(
                display_snapshot
                    .combined_highlights(multibuffer_start..multibuffer_end, syntax_theme),
            );
        }
        None => {
            for (outline_text_word_start, outline_word) in outline_text.split_word_bound_indices() {
                if let Some(start_index) = search_text.find(outline_word) {
                    let multibuffer_start = search_start_offset + MultiBufferOffset(start_index);
                    let multibuffer_end = multibuffer_start + MultiBufferOffset(outline_word.len());
                    outline_text_highlights.extend(
                        display_snapshot
                            .combined_highlights(multibuffer_start..multibuffer_end, syntax_theme)
                            .into_iter()
                            .map(|(range_in_word, style)| {
                                (
                                    outline_text_word_start + range_in_word.start
                                        ..outline_text_word_start + range_in_word.end,
                                    style,
                                )
                            }),
                    );
                }
            }
        }
    }

    if outline_text_highlights.is_empty() {
        None
    } else {
        Some(outline_text_highlights)
    }
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

        update_test_language_settings(cx, &|settings| {
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
            assert_eq!(outline_symbol_names(editor), vec!["fn main"]);
        });
    }

    #[gpui::test]
    async fn test_lsp_document_symbols_nested(cx: &mut TestAppContext) {
        init_test(cx, |_| {});

        update_test_language_settings(cx, &|settings| {
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
                vec!["struct Foo", "bar"],
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
        update_test_language_settings(&mut cx.cx.cx, &|settings| {
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
        update_test_language_settings(&mut cx.cx.cx, &|settings| {
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

        update_test_language_settings(cx, &|settings| {
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

        update_test_language_settings(cx, &|settings| {
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

        update_test_language_settings(cx, &|settings| {
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
                vec!["mod MyModule", "fn my_function"]
            );
        });
    }

    #[gpui::test]
    async fn test_lsp_document_symbols_multibyte_highlights(cx: &mut TestAppContext) {
        init_test(cx, |_| {});

        update_test_language_settings(cx, &|settings| {
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
                    // Buffer: "/// αyzabc\nfn test() {}\n"
                    // Bytes 0-3: "/// ", bytes 4-5: α (2-byte UTF-8), bytes 6-11: "yzabc\n"
                    // Line 1 starts at byte 12: "fn test() {}"
                    //
                    // Symbol range includes doc comment (line 0-1).
                    // Selection points to "test" on line 1.
                    // enriched_symbol_text extracts "fn test" with source_range_for_text.start at byte 12.
                    // search_start = max(12 - 7, 0) = 5, which is INSIDE the 2-byte 'α' char.
                    Ok(Some(lsp::DocumentSymbolResponse::Nested(vec![
                        nested_symbol(
                            "test",
                            lsp::SymbolKind::FUNCTION,
                            lsp_range(0, 0, 1, 13), // includes doc comment
                            lsp_range(1, 3, 1, 7),  // "test"
                            Vec::new(),
                        ),
                    ])))
                },
            );

        // "/// αyzabc\n" = 12 bytes, then "fn test() {}\n"
        // search_start = 12 - 7 = 5, which is byte 5 = second byte of 'α' (not a char boundary)
        cx.set_state("/// αyzabc\nfn teˇst() {}\n");
        assert!(symbol_request.next().await.is_some());
        cx.run_until_parked();

        cx.update_editor(|editor, _window, _cx| {
            let (_, symbols) = editor
                .outline_symbols_at_cursor
                .as_ref()
                .expect("Should have outline symbols");
            assert_eq!(symbols.len(), 1);

            let symbol = &symbols[0];
            assert_eq!(symbol.text, "fn test");

            // Verify all highlight ranges are valid byte boundaries in the text
            for (range, _style) in &symbol.highlight_ranges {
                assert!(
                    symbol.text.is_char_boundary(range.start),
                    "highlight range start {} is not a char boundary in {:?}",
                    range.start,
                    symbol.text
                );
                assert!(
                    symbol.text.is_char_boundary(range.end),
                    "highlight range end {} is not a char boundary in {:?}",
                    range.end,
                    symbol.text
                );
                assert!(
                    range.end <= symbol.text.len(),
                    "highlight range end {} exceeds text length {} for {:?}",
                    range.end,
                    symbol.text.len(),
                    symbol.text
                );
            }
        });
    }

    #[gpui::test]
    async fn test_lsp_document_symbols_empty_response(cx: &mut TestAppContext) {
        init_test(cx, |_| {});

        update_test_language_settings(cx, &|settings| {
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
