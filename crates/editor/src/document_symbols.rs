use gpui::{AppContext as _, Context, Task};
use language::OutlineItem;
use multi_buffer::{Anchor, MultiBufferSnapshot};
use text::BufferId;

use crate::Editor;

impl Editor {
    pub(super) fn lsp_document_symbols_for_cursor(
        &self,
        cursor: Anchor,
        multibuffer_snapshot: &MultiBufferSnapshot,
        cx: &mut Context<Self>,
    ) -> Option<Task<Option<(BufferId, Vec<OutlineItem<Anchor>>)>>> {
        let provider = self.semantics_provider.as_ref()?;
        let excerpt = multibuffer_snapshot.excerpt_containing(cursor..cursor)?;
        let excerpt_id = excerpt.id();
        let buffer_id = excerpt.buffer_id();
        let buffer = self.buffer.read(cx).buffer(buffer_id)?;
        let buffer_snapshot = buffer.read(cx).snapshot();
        let task = provider.document_symbols(&buffer, cx)?;
        let cursor_text_anchor = cursor.text_anchor;

        Some(cx.background_spawn(async move {
            let lsp_items = task.await;
            if lsp_items.is_empty() {
                return None;
            }

            let mut symbols: Vec<OutlineItem<Anchor>> = lsp_items
                .into_iter()
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
                    range: Anchor::range_in_buffer(excerpt_id, item.range),
                    source_range_for_text: Anchor::range_in_buffer(
                        excerpt_id,
                        item.source_range_for_text,
                    ),
                    text: item.text,
                    highlight_ranges: item.highlight_ranges,
                    name_ranges: item.name_ranges,
                    body_range: item
                        .body_range
                        .map(|r| Anchor::range_in_buffer(excerpt_id, r)),
                    annotation_range: item
                        .annotation_range
                        .map(|r| Anchor::range_in_buffer(excerpt_id, r)),
                })
                .collect();

            let mut prev_depth = None;
            symbols.retain(|item| {
                let result = prev_depth.is_none_or(|prev_depth| item.depth > prev_depth);
                prev_depth = Some(item.depth);
                result
            });

            Some((buffer_id, symbols))
        }))
    }
}

#[cfg(test)]
mod tests {
    use futures::StreamExt as _;
    use gpui::TestAppContext;
    use settings::DocumentSymbols;
    use zed_actions::editor::{MoveDown, MoveUp};

    use crate::{
        editor_tests::{init_test, update_test_language_settings},
        test::editor_lsp_test_context::EditorLspTestContext,
    };

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
                            vec![],
                        ),
                    ])))
                },
            );

        cx.set_state("fn maˇin() {\n    let x = 1;\n}\n");
        cx.run_until_parked();

        // Trigger breadcrumb refresh by notifying
        cx.update_editor(|editor, _window, cx| {
            let _symbols = &editor.outline_symbols;
            cx.notify();
        });

        // The selection change triggers refresh_outline_symbols
        assert!(symbol_request.next().await.is_some());
        cx.run_until_parked();

        cx.update_editor(|editor, _window, _cx| {
            let symbols = editor
                .outline_symbols
                .as_ref()
                .expect("Should have outline symbols after LSP response");
            let names: Vec<&str> = symbols.1.iter().map(|s| s.text.as_str()).collect();
            assert_eq!(names, vec!["main"]);
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
                                    vec![],
                                ),
                                nested_symbol(
                                    "baz",
                                    lsp::SymbolKind::FIELD,
                                    lsp_range(2, 4, 2, 15),
                                    lsp_range(2, 4, 2, 7),
                                    vec![],
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
            let symbols = editor
                .outline_symbols
                .as_ref()
                .expect("Should have outline symbols");
            let names: Vec<&str> = symbols.1.iter().map(|s| s.text.as_str()).collect();
            // cursor is inside Foo > bar, so we expect the containing chain
            assert_eq!(names, vec!["Foo", "bar"]);
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
                            vec![],
                        ),
                    ])))
                },
            );

        cx.set_state("fn maˇin() {\n    let x = 1;\n}\n");
        cx.run_until_parked();

        // Step 1: With tree-sitter (default), breadcrumbs use tree-sitter outline
        cx.update_editor(|editor, _window, _cx| {
            let symbols = editor
                .outline_symbols
                .as_ref()
                .expect("Should have tree-sitter outline symbols");
            let names: Vec<&str> = symbols.1.iter().map(|s| s.text.as_str()).collect();
            assert_eq!(
                names,
                vec!["fn main"],
                "Tree-sitter should produce 'fn main'"
            );
        });

        // Step 2: Switch to LSP
        update_test_language_settings(&mut cx.cx.cx, |settings| {
            settings.defaults.document_symbols = Some(DocumentSymbols::On);
        });
        cx.run_until_parked();

        // Force a selection change to trigger refresh_outline_symbols
        cx.update_editor(|editor, window, cx| {
            editor.move_down(&MoveDown, window, cx);
        });
        assert!(symbol_request.next().await.is_some());
        cx.run_until_parked();

        cx.update_editor(|editor, _window, _cx| {
            let symbols = editor
                .outline_symbols
                .as_ref()
                .expect("Should have LSP outline symbols after switching to LSP");
            let names: Vec<&str> = symbols.1.iter().map(|s| s.text.as_str()).collect();
            assert_eq!(
                names,
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
            let symbols = editor
                .outline_symbols
                .as_ref()
                .expect("Should have tree-sitter symbols after switching back");
            let names: Vec<&str> = symbols.1.iter().map(|s| s.text.as_str()).collect();
            assert_eq!(
                names,
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

        let request_count = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
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
                request_count_clone.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                async move {
                    Ok(Some(lsp::DocumentSymbolResponse::Nested(vec![
                        nested_symbol(
                            "main",
                            lsp::SymbolKind::FUNCTION,
                            lsp_range(0, 0, 2, 1),
                            lsp_range(0, 3, 0, 7),
                            vec![],
                        ),
                    ])))
                }
            });

        cx.set_state("fn maˇin() {\n    let x = 1;\n}\n");
        assert!(symbol_request.next().await.is_some());
        cx.run_until_parked();

        let first_count = request_count.load(std::sync::atomic::Ordering::SeqCst);
        assert!(first_count > 0, "Should have made at least one request");

        // Move cursor within the same buffer version — should use cache
        cx.update_editor(|editor, window, cx| {
            editor.move_down(&MoveDown, window, cx);
        });
        cx.run_until_parked();

        let second_count = request_count.load(std::sync::atomic::Ordering::SeqCst);
        assert_eq!(
            first_count, second_count,
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
                                uri: lsp::Uri::from_file_path("/a/main.rs").unwrap(),
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
            let symbols = editor
                .outline_symbols
                .as_ref()
                .expect("Should have outline symbols from flat response");
            let names: Vec<&str> = symbols.1.iter().map(|s| s.text.as_str()).collect();
            assert_eq!(names, vec!["main"]);
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
                            "Foo",
                            lsp::SymbolKind::STRUCT,
                            lsp_range(0, 0, 3, 1),
                            lsp_range(0, 7, 0, 10),
                            vec![nested_symbol(
                                "bar",
                                lsp::SymbolKind::FIELD,
                                lsp_range(1, 4, 1, 13),
                                lsp_range(1, 4, 1, 7),
                                vec![],
                            )],
                        ),
                    ])))
                },
            );

        cx.set_state("struct Foo {\n    baˇr: u32,\n    baz: String,\n}\n");
        assert!(symbol_request.next().await.is_some());
        cx.run_until_parked();

        cx.update_editor(|editor, _window, _cx| {
            let (_buffer_id, symbols) = editor
                .outline_symbols
                .as_ref()
                .expect("Should have breadcrumb symbols");
            let names: Vec<&str> = symbols.iter().map(|s| s.text.as_str()).collect();
            assert_eq!(
                names,
                vec!["Foo", "bar"],
                "Breadcrumbs should show the LSP symbol chain from root to cursor"
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
                move |_, _, _| async move { Ok(Some(lsp::DocumentSymbolResponse::Nested(vec![]))) },
            );

        cx.set_state("fn maˇin() {\n    let x = 1;\n}\n");
        assert!(symbol_request.next().await.is_some());
        cx.run_until_parked();

        cx.update_editor(|editor, _window, _cx| {
            // Empty LSP response means no outline symbols for breadcrumbs
            assert!(
                editor.outline_symbols.is_none(),
                "Empty LSP response should result in no outline symbols"
            );
        });
    }

    #[gpui::test]
    async fn test_lsp_document_symbols_disabled_by_default(cx: &mut TestAppContext) {
        init_test(cx, |_| {});

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
                            vec![],
                        ),
                    ])))
                },
            );

        cx.set_state("fn maˇin() {\n    let x = 1;\n}\n");
        cx.run_until_parked();

        // LSP document symbols are off by default — the tree-sitter path is used,
        // so no LSP request should be made.
        assert!(
            symbol_request.try_next().is_err(),
            "No LSP documentSymbol request should be sent when setting is tree_sitter"
        );

        // But we should still have tree-sitter based outline symbols
        cx.update_editor(|editor, _window, _cx| {
            let symbols = editor
                .outline_symbols
                .as_ref()
                .expect("Should have tree-sitter outline symbols even when LSP is off");
            let names: Vec<&str> = symbols.1.iter().map(|s| s.text.as_str()).collect();
            assert_eq!(names, vec!["fn main"]);
        });

        // Now enable LSP
        update_test_language_settings(&mut cx.cx.cx, |settings| {
            settings.defaults.document_symbols = Some(DocumentSymbols::On);
        });
        cx.run_until_parked();

        // Move cursor to trigger a refresh
        cx.update_editor(|editor, window, cx| {
            editor.move_down(&MoveDown, window, cx);
        });
        assert!(symbol_request.next().await.is_some());
        cx.run_until_parked();

        cx.update_editor(|editor, _window, _cx| {
            let symbols = editor
                .outline_symbols
                .as_ref()
                .expect("Should have LSP outline symbols after enabling LSP");
            let names: Vec<&str> = symbols.1.iter().map(|s| s.text.as_str()).collect();
            assert_eq!(
                names,
                vec!["main"],
                "After enabling LSP, should see LSP symbols"
            );
        });
    }
}
