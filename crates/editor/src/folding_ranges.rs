use futures::future::join_all;
use itertools::Itertools;
use language::language_settings::language_settings;
use text::BufferId;
use ui::{Context, Window};

use crate::{Editor, LSP_REQUEST_DEBOUNCE_TIMEOUT};

impl Editor {
    pub(super) fn refresh_folding_ranges(
        &mut self,
        for_buffer: Option<BufferId>,
        _window: &Window,
        cx: &mut Context<Self>,
    ) {
        if !self.mode().is_full() || !self.use_document_folding_ranges {
            return;
        }
        let Some(project) = self.project.clone() else {
            return;
        };

        let buffers_to_query = self
            .visible_excerpts(true, cx)
            .into_values()
            .map(|(buffer, ..)| buffer)
            .chain(for_buffer.and_then(|id| self.buffer.read(cx).buffer(id)))
            .filter(|buffer| {
                let id = buffer.read(cx).remote_id();
                (for_buffer.is_none_or(|target| target == id))
                    && self.registered_buffers.contains_key(&id)
                    && language_settings(
                        buffer.read(cx).language().map(|l| l.name()),
                        buffer.read(cx).file(),
                        cx,
                    )
                    .document_folding_ranges
                    .enabled()
            })
            .unique_by(|buffer| buffer.read(cx).remote_id())
            .collect::<Vec<_>>();

        self.refresh_folding_ranges_task = cx.spawn(async move |editor, cx| {
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
                                let task = lsp_store.fetch_folding_ranges(&buffer, cx);
                                async move { (buffer_id, task.await) }
                            })
                            .collect::<Vec<_>>()
                    })
                })
                .ok()
            else {
                return;
            };

            let results = join_all(tasks).await;
            if results.is_empty() {
                return;
            }

            editor
                .update(cx, |editor, cx| {
                    editor.display_map.update(cx, |display_map, cx| {
                        for (buffer_id, ranges) in results {
                            display_map.set_lsp_folding_ranges(buffer_id, ranges, cx);
                        }
                    });
                    cx.notify();
                })
                .ok();
        });
    }

    pub fn document_folding_ranges_enabled(&self, cx: &ui::App) -> bool {
        self.use_document_folding_ranges && self.display_map.read(cx).has_lsp_folding_ranges()
    }

    /// Removes LSP folding creases for buffers whose `lsp_folding_ranges`
    /// setting has been turned off, and triggers a refresh so newly-enabled
    /// buffers get their ranges fetched.
    pub(super) fn clear_disabled_lsp_folding_ranges(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if !self.use_document_folding_ranges {
            return;
        }

        let buffers_to_clear = self
            .buffer
            .read(cx)
            .all_buffers()
            .into_iter()
            .filter(|buffer| {
                let buffer = buffer.read(cx);
                !language_settings(buffer.language().map(|l| l.name()), buffer.file(), cx)
                    .document_folding_ranges
                    .enabled()
            })
            .map(|buffer| buffer.read(cx).remote_id())
            .collect::<Vec<_>>();

        if !buffers_to_clear.is_empty() {
            self.display_map.update(cx, |display_map, cx| {
                for buffer_id in buffers_to_clear {
                    display_map.clear_lsp_folding_ranges(buffer_id, cx);
                }
            });
            cx.notify();
        }

        self.refresh_folding_ranges(None, window, cx);
    }
}

#[cfg(test)]
mod tests {
    use futures::StreamExt as _;
    use gpui::TestAppContext;
    use lsp::FoldingRange;
    use multi_buffer::MultiBufferRow;
    use settings::DocumentFoldingRanges;

    use crate::{
        editor_tests::{init_test, update_test_language_settings},
        test::editor_lsp_test_context::EditorLspTestContext,
    };

    #[gpui::test]
    async fn test_lsp_folding_ranges_populates_creases(cx: &mut TestAppContext) {
        init_test(cx, |_| {});

        update_test_language_settings(cx, |settings| {
            settings.defaults.document_folding_ranges = Some(DocumentFoldingRanges::On);
        });

        let mut cx = EditorLspTestContext::new_rust(
            lsp::ServerCapabilities {
                folding_range_provider: Some(lsp::FoldingRangeProviderCapability::Simple(true)),
                ..lsp::ServerCapabilities::default()
            },
            cx,
        )
        .await;

        let mut folding_request = cx
            .set_request_handler::<lsp::request::FoldingRangeRequest, _, _>(
                move |_, _, _| async move {
                    Ok(Some(vec![
                        FoldingRange {
                            start_line: 0,
                            start_character: Some(10),
                            end_line: 4,
                            end_character: Some(1),
                            kind: None,
                            collapsed_text: None,
                        },
                        FoldingRange {
                            start_line: 1,
                            start_character: Some(13),
                            end_line: 3,
                            end_character: Some(5),
                            kind: None,
                            collapsed_text: None,
                        },
                        FoldingRange {
                            start_line: 6,
                            start_character: Some(11),
                            end_line: 8,
                            end_character: Some(1),
                            kind: None,
                            collapsed_text: None,
                        },
                    ]))
                },
            );

        cx.set_state(
            "ˇfn main() {\n    if true {\n        println!(\"hello\");\n    }\n}\n\nfn other() {\n    let x = 1;\n}\n",
        );
        assert!(folding_request.next().await.is_some());
        cx.run_until_parked();

        cx.editor.read_with(&cx.cx.cx, |editor, cx| {
            assert!(
                editor.document_folding_ranges_enabled(cx),
                "Expected LSP folding ranges to be populated"
            );
        });

        cx.update_editor(|editor, _window, cx| {
            let snapshot = editor.display_snapshot(cx);
            assert!(
                !snapshot.is_line_folded(MultiBufferRow(0)),
                "Line 0 should not be folded before any fold action"
            );
            assert!(
                !snapshot.is_line_folded(MultiBufferRow(6)),
                "Line 6 should not be folded before any fold action"
            );
        });

        cx.update_editor(|editor, window, cx| {
            editor.fold_at(MultiBufferRow(0), window, cx);
        });

        cx.update_editor(|editor, _window, cx| {
            let snapshot = editor.display_snapshot(cx);
            assert!(
                snapshot.is_line_folded(MultiBufferRow(0)),
                "Line 0 should be folded after fold_at on an LSP crease"
            );
            assert_eq!(
                editor.display_text(cx),
                "fn main() ⋯\n\nfn other() {\n    let x = 1;\n}\n",
            );
        });

        cx.update_editor(|editor, window, cx| {
            editor.fold_at(MultiBufferRow(6), window, cx);
        });

        cx.update_editor(|editor, _window, cx| {
            let snapshot = editor.display_snapshot(cx);
            assert!(
                snapshot.is_line_folded(MultiBufferRow(6)),
                "Line 6 should be folded after fold_at on the second LSP crease"
            );
            assert_eq!(editor.display_text(cx), "fn main() ⋯\n\nfn other() ⋯\n",);
        });
    }

    #[gpui::test]
    async fn test_lsp_folding_ranges_disabled_by_default(cx: &mut TestAppContext) {
        init_test(cx, |_| {});

        let mut cx = EditorLspTestContext::new_rust(
            lsp::ServerCapabilities {
                folding_range_provider: Some(lsp::FoldingRangeProviderCapability::Simple(true)),
                ..lsp::ServerCapabilities::default()
            },
            cx,
        )
        .await;

        cx.set_state("ˇfn main() {\n    let x = 1;\n}\n");
        cx.run_until_parked();

        cx.editor.read_with(&cx.cx.cx, |editor, cx| {
            assert!(
                !editor.document_folding_ranges_enabled(cx),
                "LSP folding ranges should not be enabled by default"
            );
        });
    }

    #[gpui::test]
    async fn test_lsp_folding_ranges_toggling_off_removes_creases(cx: &mut TestAppContext) {
        init_test(cx, |_| {});

        update_test_language_settings(cx, |settings| {
            settings.defaults.document_folding_ranges = Some(DocumentFoldingRanges::On);
        });

        let mut cx = EditorLspTestContext::new_rust(
            lsp::ServerCapabilities {
                folding_range_provider: Some(lsp::FoldingRangeProviderCapability::Simple(true)),
                ..lsp::ServerCapabilities::default()
            },
            cx,
        )
        .await;

        let mut folding_request = cx
            .set_request_handler::<lsp::request::FoldingRangeRequest, _, _>(
                move |_, _, _| async move {
                    Ok(Some(vec![FoldingRange {
                        start_line: 0,
                        start_character: Some(10),
                        end_line: 4,
                        end_character: Some(1),
                        kind: None,
                        collapsed_text: None,
                    }]))
                },
            );

        cx.set_state("ˇfn main() {\n    if true {\n        println!(\"hello\");\n    }\n}\n");
        assert!(folding_request.next().await.is_some());
        cx.run_until_parked();

        cx.editor.read_with(&cx.cx.cx, |editor, cx| {
            assert!(
                editor.document_folding_ranges_enabled(cx),
                "Expected LSP folding ranges to be active before toggling off"
            );
        });

        cx.update_editor(|editor, window, cx| {
            editor.fold_at(MultiBufferRow(0), window, cx);
        });
        cx.update_editor(|editor, _window, cx| {
            let snapshot = editor.display_snapshot(cx);
            assert!(
                snapshot.is_line_folded(MultiBufferRow(0)),
                "Line 0 should be folded via LSP crease before toggling off"
            );
            assert_eq!(editor.display_text(cx), "fn main() ⋯\n",);
        });

        update_test_language_settings(&mut cx.cx.cx, |settings| {
            settings.defaults.document_folding_ranges = Some(DocumentFoldingRanges::Off);
        });
        cx.run_until_parked();

        cx.editor.read_with(&cx.cx.cx, |editor, cx| {
            assert!(
                !editor.document_folding_ranges_enabled(cx),
                "LSP folding ranges should be cleared after toggling off"
            );
        });
    }

    #[gpui::test]
    async fn test_lsp_folding_ranges_nested_folds(cx: &mut TestAppContext) {
        init_test(cx, |_| {});

        update_test_language_settings(cx, |settings| {
            settings.defaults.document_folding_ranges = Some(DocumentFoldingRanges::On);
        });

        let mut cx = EditorLspTestContext::new_rust(
            lsp::ServerCapabilities {
                folding_range_provider: Some(lsp::FoldingRangeProviderCapability::Simple(true)),
                ..lsp::ServerCapabilities::default()
            },
            cx,
        )
        .await;

        let mut folding_request = cx
            .set_request_handler::<lsp::request::FoldingRangeRequest, _, _>(
                move |_, _, _| async move {
                    Ok(Some(vec![
                        FoldingRange {
                            start_line: 0,
                            start_character: Some(10),
                            end_line: 7,
                            end_character: Some(1),
                            kind: None,
                            collapsed_text: None,
                        },
                        FoldingRange {
                            start_line: 1,
                            start_character: Some(12),
                            end_line: 3,
                            end_character: Some(5),
                            kind: None,
                            collapsed_text: None,
                        },
                        FoldingRange {
                            start_line: 4,
                            start_character: Some(13),
                            end_line: 6,
                            end_character: Some(5),
                            kind: None,
                            collapsed_text: None,
                        },
                    ]))
                },
            );

        cx.set_state(
            "ˇfn main() {\n    if true {\n        a();\n    }\n    if false {\n        b();\n    }\n}\n",
        );
        assert!(folding_request.next().await.is_some());
        cx.run_until_parked();

        cx.update_editor(|editor, window, cx| {
            editor.fold_at(MultiBufferRow(1), window, cx);
        });
        cx.update_editor(|editor, _window, cx| {
            let snapshot = editor.display_snapshot(cx);
            assert!(snapshot.is_line_folded(MultiBufferRow(1)));
            assert!(!snapshot.is_line_folded(MultiBufferRow(0)));
            assert_eq!(
                editor.display_text(cx),
                "fn main() {\n    if true ⋯\n    if false {\n        b();\n    }\n}\n",
            );
        });

        cx.update_editor(|editor, window, cx| {
            editor.fold_at(MultiBufferRow(4), window, cx);
        });
        cx.update_editor(|editor, _window, cx| {
            let snapshot = editor.display_snapshot(cx);
            assert!(snapshot.is_line_folded(MultiBufferRow(4)));
            assert_eq!(
                editor.display_text(cx),
                "fn main() {\n    if true ⋯\n    if false ⋯\n}\n",
            );
        });

        cx.update_editor(|editor, window, cx| {
            editor.fold_at(MultiBufferRow(0), window, cx);
        });
        cx.update_editor(|editor, _window, cx| {
            let snapshot = editor.display_snapshot(cx);
            assert!(snapshot.is_line_folded(MultiBufferRow(0)));
            assert_eq!(editor.display_text(cx), "fn main() ⋯\n",);
        });
    }

    #[gpui::test]
    async fn test_lsp_folding_ranges_unsorted_from_server(cx: &mut TestAppContext) {
        init_test(cx, |_| {});

        update_test_language_settings(cx, |settings| {
            settings.defaults.document_folding_ranges = Some(DocumentFoldingRanges::On);
        });

        let mut cx = EditorLspTestContext::new_rust(
            lsp::ServerCapabilities {
                folding_range_provider: Some(lsp::FoldingRangeProviderCapability::Simple(true)),
                ..lsp::ServerCapabilities::default()
            },
            cx,
        )
        .await;

        let mut folding_request = cx
            .set_request_handler::<lsp::request::FoldingRangeRequest, _, _>(
                move |_, _, _| async move {
                    Ok(Some(vec![
                        FoldingRange {
                            start_line: 6,
                            start_character: Some(11),
                            end_line: 8,
                            end_character: Some(1),
                            kind: None,
                            collapsed_text: None,
                        },
                        FoldingRange {
                            start_line: 0,
                            start_character: Some(10),
                            end_line: 4,
                            end_character: Some(1),
                            kind: None,
                            collapsed_text: None,
                        },
                        FoldingRange {
                            start_line: 1,
                            start_character: Some(13),
                            end_line: 3,
                            end_character: Some(5),
                            kind: None,
                            collapsed_text: None,
                        },
                    ]))
                },
            );

        cx.set_state(
            "ˇfn main() {\n    if true {\n        println!(\"hello\");\n    }\n}\n\nfn other() {\n    let x = 1;\n}\n",
        );
        assert!(folding_request.next().await.is_some());
        cx.run_until_parked();

        cx.editor.read_with(&cx.cx.cx, |editor, cx| {
            assert!(
                editor.document_folding_ranges_enabled(cx),
                "Expected LSP folding ranges to be populated despite unsorted server response"
            );
        });

        cx.update_editor(|editor, window, cx| {
            editor.fold_at(MultiBufferRow(0), window, cx);
        });
        cx.update_editor(|editor, _window, cx| {
            assert_eq!(
                editor.display_text(cx),
                "fn main() ⋯\n\nfn other() {\n    let x = 1;\n}\n",
            );
        });

        cx.update_editor(|editor, window, cx| {
            editor.fold_at(MultiBufferRow(6), window, cx);
        });
        cx.update_editor(|editor, _window, cx| {
            assert_eq!(editor.display_text(cx), "fn main() ⋯\n\nfn other() ⋯\n",);
        });
    }

    #[gpui::test]
    async fn test_lsp_folding_ranges_switch_between_treesitter_and_lsp(cx: &mut TestAppContext) {
        init_test(cx, |_| {});

        let mut cx = EditorLspTestContext::new_rust(
            lsp::ServerCapabilities {
                folding_range_provider: Some(lsp::FoldingRangeProviderCapability::Simple(true)),
                ..lsp::ServerCapabilities::default()
            },
            cx,
        )
        .await;

        let source =
            "fn main() {\n    let a = 1;\n    let b = 2;\n    let c = 3;\n    let d = 4;\n}\n";
        cx.set_state(&format!("ˇ{source}"));
        cx.run_until_parked();

        // Phase 1: tree-sitter / indentation-based folding (LSP folding OFF by default).
        cx.editor.read_with(&cx.cx.cx, |editor, cx| {
            assert!(
                !editor.document_folding_ranges_enabled(cx),
                "LSP folding ranges should be off by default"
            );
        });

        cx.update_editor(|editor, window, cx| {
            editor.fold_at(MultiBufferRow(0), window, cx);
        });
        cx.update_editor(|editor, _window, cx| {
            let snapshot = editor.display_snapshot(cx);
            assert!(
                snapshot.is_line_folded(MultiBufferRow(0)),
                "Indentation-based fold should work on the function"
            );
            assert_eq!(editor.display_text(cx), "fn main() {⋯\n}\n",);
        });

        cx.update_editor(|editor, window, cx| {
            editor.unfold_at(MultiBufferRow(0), window, cx);
        });
        cx.update_editor(|editor, _window, cx| {
            assert!(
                !editor
                    .display_snapshot(cx)
                    .is_line_folded(MultiBufferRow(0)),
                "Function should be unfolded"
            );
        });

        // Phase 2: switch to LSP folding with non-syntactic ("odd") ranges.
        // The LSP returns two ranges that each cover a pair of let-bindings,
        // which is not something tree-sitter / indentation folding would produce.
        let mut folding_request = cx
            .set_request_handler::<lsp::request::FoldingRangeRequest, _, _>(
                move |_, _, _| async move {
                    Ok(Some(vec![
                        FoldingRange {
                            start_line: 1,
                            start_character: Some(14),
                            end_line: 2,
                            end_character: Some(14),
                            kind: None,
                            collapsed_text: None,
                        },
                        FoldingRange {
                            start_line: 3,
                            start_character: Some(14),
                            end_line: 4,
                            end_character: Some(14),
                            kind: None,
                            collapsed_text: None,
                        },
                    ]))
                },
            );

        update_test_language_settings(&mut cx.cx.cx, |settings| {
            settings.defaults.document_folding_ranges = Some(DocumentFoldingRanges::On);
        });
        assert!(folding_request.next().await.is_some());
        cx.run_until_parked();

        cx.editor.read_with(&cx.cx.cx, |editor, cx| {
            assert!(
                editor.document_folding_ranges_enabled(cx),
                "LSP folding ranges should now be active"
            );
        });

        // The indentation fold at row 0 should no longer be available;
        // only the LSP ranges exist.
        cx.update_editor(|editor, window, cx| {
            editor.fold_at(MultiBufferRow(0), window, cx);
        });
        cx.update_editor(|editor, _window, cx| {
            assert!(
                !editor
                    .display_snapshot(cx)
                    .is_line_folded(MultiBufferRow(0)),
                "Row 0 has no LSP crease, so fold_at should be a no-op"
            );
        });

        cx.update_editor(|editor, window, cx| {
            editor.fold_at(MultiBufferRow(1), window, cx);
        });
        cx.update_editor(|editor, _window, cx| {
            assert!(
                editor
                    .display_snapshot(cx)
                    .is_line_folded(MultiBufferRow(1)),
                "First odd LSP range should fold"
            );
            assert_eq!(
                editor.display_text(cx),
                "fn main() {\n    let a = 1;⋯\n    let c = 3;\n    let d = 4;\n}\n",
            );
        });

        cx.update_editor(|editor, window, cx| {
            editor.fold_at(MultiBufferRow(3), window, cx);
        });
        cx.update_editor(|editor, _window, cx| {
            assert!(
                editor
                    .display_snapshot(cx)
                    .is_line_folded(MultiBufferRow(3)),
                "Second odd LSP range should fold"
            );
            assert_eq!(
                editor.display_text(cx),
                "fn main() {\n    let a = 1;⋯\n    let c = 3;⋯\n}\n",
            );
        });

        cx.update_editor(|editor, window, cx| {
            editor.unfold_at(MultiBufferRow(1), window, cx);
            editor.unfold_at(MultiBufferRow(3), window, cx);
        });

        // Phase 3: switch back to tree-sitter by disabling LSP folding ranges.
        update_test_language_settings(&mut cx.cx.cx, |settings| {
            settings.defaults.document_folding_ranges = Some(DocumentFoldingRanges::Off);
        });
        cx.run_until_parked();

        cx.editor.read_with(&cx.cx.cx, |editor, cx| {
            assert!(
                !editor.document_folding_ranges_enabled(cx),
                "LSP folding ranges should be cleared after switching back"
            );
        });

        cx.update_editor(|editor, window, cx| {
            editor.fold_at(MultiBufferRow(0), window, cx);
        });
        cx.update_editor(|editor, _window, cx| {
            let snapshot = editor.display_snapshot(cx);
            assert!(
                snapshot.is_line_folded(MultiBufferRow(0)),
                "Indentation-based fold should work again after switching back"
            );
            assert_eq!(editor.display_text(cx), "fn main() {⋯\n}\n",);
        });
    }
}
