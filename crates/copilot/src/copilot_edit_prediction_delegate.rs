use crate::{Copilot, CopilotEditPrediction};
use anyhow::Result;
use edit_prediction_types::{EditPrediction, EditPredictionDelegate, interpolate_edits};
use gpui::{App, Context, Entity, Task};
use language::{Anchor, Buffer, BufferSnapshot, EditPreview, OffsetRangeExt};
use std::{ops::Range, sync::Arc, time::Duration};

pub const COPILOT_DEBOUNCE_TIMEOUT: Duration = Duration::from_millis(75);

pub struct CopilotEditPredictionDelegate {
    completion: Option<(CopilotEditPrediction, EditPreview)>,
    pending_refresh: Option<Task<Result<()>>>,
    copilot: Entity<Copilot>,
}

impl CopilotEditPredictionDelegate {
    pub fn new(copilot: Entity<Copilot>) -> Self {
        Self {
            completion: None,
            pending_refresh: None,
            copilot,
        }
    }

    fn active_completion(&self) -> Option<&(CopilotEditPrediction, EditPreview)> {
        self.completion.as_ref()
    }
}

impl EditPredictionDelegate for CopilotEditPredictionDelegate {
    fn name() -> &'static str {
        "copilot"
    }

    fn display_name() -> &'static str {
        "Copilot"
    }

    fn show_predictions_in_menu() -> bool {
        true
    }

    fn show_tab_accept_marker() -> bool {
        true
    }

    fn is_refreshing(&self, _cx: &App) -> bool {
        self.pending_refresh.is_some() && self.completion.is_none()
    }

    fn is_enabled(
        &self,
        _buffer: &Entity<Buffer>,
        _cursor_position: language::Anchor,
        cx: &App,
    ) -> bool {
        self.copilot.read(cx).status().is_authorized()
    }

    fn refresh(
        &mut self,
        buffer: Entity<Buffer>,
        cursor_position: language::Anchor,
        debounce: bool,
        cx: &mut Context<Self>,
    ) {
        let copilot = self.copilot.clone();
        self.pending_refresh = Some(cx.spawn(async move |this, cx| {
            if debounce {
                cx.background_executor()
                    .timer(COPILOT_DEBOUNCE_TIMEOUT)
                    .await;
            }

            let completions = copilot
                .update(cx, |copilot, cx| {
                    copilot.completions(&buffer, cursor_position, cx)
                })?
                .await?;

            if let Some(mut completion) = completions.into_iter().next()
                && let Some((trimmed_range, trimmed_text, snapshot)) = cx
                    .update(|cx| trim_completion(&completion, cx))
                    .ok()
                    .flatten()
            {
                let preview = buffer
                    .update(cx, |this, cx| {
                        this.preview_edits(
                            Arc::from([(trimmed_range.clone(), trimmed_text.clone())].as_slice()),
                            cx,
                        )
                    })?
                    .await;
                this.update(cx, |this, cx| {
                    this.pending_refresh = None;
                    completion.range = trimmed_range;
                    completion.text = trimmed_text.to_string();
                    completion.snapshot = snapshot;
                    this.completion = Some((completion, preview));

                    cx.notify();
                })?;
            }

            Ok(())
        }));
    }

    fn accept(&mut self, cx: &mut Context<Self>) {
        if let Some((completion, _)) = self.active_completion() {
            self.copilot
                .update(cx, |copilot, cx| copilot.accept_completion(completion, cx))
                .detach_and_log_err(cx);
        }
    }

    fn discard(&mut self, _: &mut Context<Self>) {}

    fn suggest(
        &mut self,
        buffer: &Entity<Buffer>,
        _: language::Anchor,
        cx: &mut Context<Self>,
    ) -> Option<EditPrediction> {
        let buffer_id = buffer.entity_id();
        let buffer = buffer.read(cx);
        let (completion, edit_preview) = self.active_completion()?;

        if Some(buffer_id) != Some(completion.buffer.entity_id())
            || !completion.range.start.is_valid(buffer)
            || !completion.range.end.is_valid(buffer)
        {
            return None;
        }
        let edits = vec![(
            completion.range.clone(),
            Arc::from(completion.text.as_ref()),
        )];
        let edits = interpolate_edits(&completion.snapshot, &buffer.snapshot(), &edits)
            .filter(|edits| !edits.is_empty())?;

        Some(EditPrediction::Local {
            id: None,
            edits,
            edit_preview: Some(edit_preview.clone()),
        })
    }
}

fn trim_completion(
    completion: &CopilotEditPrediction,
    cx: &mut App,
) -> Option<(Range<Anchor>, Arc<str>, BufferSnapshot)> {
    let buffer = completion.buffer.read(cx);
    let mut completion_range = completion.range.to_offset(buffer);
    let prefix_len = common_prefix(
        buffer.chars_for_range(completion_range.clone()),
        completion.text.chars(),
    );
    completion_range.start += prefix_len;
    let suffix_len = common_prefix(
        buffer.reversed_chars_for_range(completion_range.clone()),
        completion.text[prefix_len..].chars().rev(),
    );
    completion_range.end = completion_range.end.saturating_sub(suffix_len);
    let completion_text = &completion.text[prefix_len..completion.text.len() - suffix_len];
    if completion_text.trim().is_empty() {
        None
    } else {
        let snapshot = buffer.snapshot();
        let completion_range = snapshot.anchor_after(completion_range.start)
            ..snapshot.anchor_after(completion_range.end);

        Some((completion_range, Arc::from(completion_text), snapshot))
    }
}

fn common_prefix<T1: Iterator<Item = char>, T2: Iterator<Item = char>>(a: T1, b: T2) -> usize {
    a.zip(b)
        .take_while(|(a, b)| a == b)
        .map(|(a, _)| a.len_utf8())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use edit_prediction_types::EditPredictionGranularity;
    use editor::{
        Editor, ExcerptRange, MultiBuffer, MultiBufferOffset, SelectionEffects,
        test::editor_lsp_test_context::EditorLspTestContext,
    };
    use fs::FakeFs;
    use futures::StreamExt;
    use gpui::{AppContext as _, BackgroundExecutor, TestAppContext, UpdateGlobal};
    use indoc::indoc;
    use language::{
        Point,
        language_settings::{CompletionSettingsContent, LspInsertMode, WordsCompletionMode},
    };
    use lsp::Uri;
    use project::Project;
    use serde_json::json;
    use settings::{AllLanguageSettingsContent, SettingsStore};
    use std::future::Future;
    use util::{
        path,
        test::{TextRangeMarker, marked_text_ranges_by},
    };

    #[gpui::test(iterations = 10)]
    async fn test_copilot(executor: BackgroundExecutor, cx: &mut TestAppContext) {
        // flaky
        init_test(cx, |settings| {
            settings.defaults.completions = Some(CompletionSettingsContent {
                words: Some(WordsCompletionMode::Disabled),
                words_min_length: Some(0),
                lsp_insert_mode: Some(LspInsertMode::Insert),
                ..Default::default()
            });
        });

        let (copilot, copilot_lsp) = Copilot::fake(cx);
        let mut cx = EditorLspTestContext::new_rust(
            lsp::ServerCapabilities {
                completion_provider: Some(lsp::CompletionOptions {
                    trigger_characters: Some(vec![".".to_string(), ":".to_string()]),
                    ..Default::default()
                }),
                ..Default::default()
            },
            cx,
        )
        .await;
        let copilot_provider = cx.new(|_| CopilotEditPredictionDelegate::new(copilot));
        cx.update_editor(|editor, window, cx| {
            editor.set_edit_prediction_provider(Some(copilot_provider), window, cx)
        });

        cx.set_state(indoc! {"
            oneˇ
            two
            three
        "});
        cx.simulate_keystroke(".");
        drop(handle_completion_request(
            &mut cx,
            indoc! {"
                one.|<>
                two
                three
            "},
            vec!["completion_a", "completion_b"],
        ));
        handle_copilot_completion_request(
            &copilot_lsp,
            vec![crate::request::NextEditSuggestion {
                text: "one.copilot1".into(),
                range: lsp::Range::new(lsp::Position::new(0, 0), lsp::Position::new(0, 4)),
                command: None,
                text_document: lsp::VersionedTextDocumentIdentifier {
                    uri: Uri::from_file_path(path!("/root/dir/file.rs")).unwrap(),
                    version: 0,
                },
            }],
        );
        executor.advance_clock(COPILOT_DEBOUNCE_TIMEOUT);
        cx.update_editor(|editor, window, cx| {
            assert!(editor.context_menu_visible());
            assert!(editor.has_active_edit_prediction());
            // Since we have both, the copilot suggestion is existing but does not show up as ghost text
            assert_eq!(editor.text(cx), "one.\ntwo\nthree\n");
            assert_eq!(editor.display_text(cx), "one.\ntwo\nthree\n");

            // Confirming a non-copilot completion inserts it and hides the context menu, without showing
            // the copilot suggestion afterwards.
            editor
                .confirm_completion(&Default::default(), window, cx)
                .unwrap()
                .detach();
            assert!(!editor.context_menu_visible());
            assert!(!editor.has_active_edit_prediction());
            assert_eq!(editor.text(cx), "one.completion_a\ntwo\nthree\n");
            assert_eq!(editor.display_text(cx), "one.completion_a\ntwo\nthree\n");
        });

        // Reset editor and only return copilot suggestions
        cx.set_state(indoc! {"
            oneˇ
            two
            three
        "});
        cx.simulate_keystroke(".");

        drop(handle_completion_request(
            &mut cx,
            indoc! {"
                one.|<>
                two
                three
            "},
            vec![],
        ));
        handle_copilot_completion_request(
            &copilot_lsp,
            vec![crate::request::NextEditSuggestion {
                text: "one.copilot1".into(),
                range: lsp::Range::new(lsp::Position::new(0, 0), lsp::Position::new(0, 4)),
                command: None,
                text_document: lsp::VersionedTextDocumentIdentifier {
                    uri: Uri::from_file_path(path!("/root/dir/file.rs")).unwrap(),
                    version: 0,
                },
            }],
        );
        executor.advance_clock(COPILOT_DEBOUNCE_TIMEOUT);
        cx.update_editor(|editor, _, cx| {
            assert!(!editor.context_menu_visible());
            assert!(editor.has_active_edit_prediction());
            // Since only the copilot is available, it's shown inline
            assert_eq!(editor.display_text(cx), "one.copilot1\ntwo\nthree\n");
            assert_eq!(editor.text(cx), "one.\ntwo\nthree\n");
        });

        // Ensure existing edit prediction is interpolated when inserting again.
        cx.simulate_keystroke("c");
        executor.run_until_parked();
        cx.update_editor(|editor, _, cx| {
            assert!(!editor.context_menu_visible());
            assert!(editor.has_active_edit_prediction());
            assert_eq!(editor.display_text(cx), "one.copilot1\ntwo\nthree\n");
            assert_eq!(editor.text(cx), "one.c\ntwo\nthree\n");
        });

        // After debouncing, new Copilot completions should be requested.
        handle_copilot_completion_request(
            &copilot_lsp,
            vec![crate::request::NextEditSuggestion {
                text: "one.copilot2".into(),
                range: lsp::Range::new(lsp::Position::new(0, 0), lsp::Position::new(0, 5)),
                command: None,
                text_document: lsp::VersionedTextDocumentIdentifier {
                    uri: Uri::from_file_path(path!("/root/dir/file.rs")).unwrap(),
                    version: 0,
                },
            }],
        );
        executor.advance_clock(COPILOT_DEBOUNCE_TIMEOUT);
        cx.update_editor(|editor, window, cx| {
            assert!(!editor.context_menu_visible());
            assert!(editor.has_active_edit_prediction());
            assert_eq!(editor.display_text(cx), "one.copilot2\ntwo\nthree\n");
            assert_eq!(editor.text(cx), "one.c\ntwo\nthree\n");

            // Canceling should remove the active Copilot suggestion.
            editor.cancel(&Default::default(), window, cx);
            assert!(!editor.has_active_edit_prediction());
            assert_eq!(editor.display_text(cx), "one.c\ntwo\nthree\n");
            assert_eq!(editor.text(cx), "one.c\ntwo\nthree\n");

            // After canceling, tabbing shouldn't insert the previously shown suggestion.
            editor.tab(&Default::default(), window, cx);
            assert!(!editor.has_active_edit_prediction());
            assert_eq!(editor.display_text(cx), "one.c   \ntwo\nthree\n");
            assert_eq!(editor.text(cx), "one.c   \ntwo\nthree\n");

            // When undoing the previously active suggestion is shown again.
            editor.undo(&Default::default(), window, cx);
            assert!(editor.has_active_edit_prediction());
            assert_eq!(editor.display_text(cx), "one.copilot2\ntwo\nthree\n");
            assert_eq!(editor.text(cx), "one.c\ntwo\nthree\n");
        });

        // If an edit occurs outside of this editor, the suggestion is still correctly interpolated.
        cx.update_buffer(|buffer, cx| buffer.edit([(5..5, "o")], None, cx));
        cx.update_editor(|editor, window, cx| {
            assert!(editor.has_active_edit_prediction());
            assert_eq!(editor.display_text(cx), "one.copilot2\ntwo\nthree\n");
            assert_eq!(editor.text(cx), "one.co\ntwo\nthree\n");

            // AcceptEditPrediction when there is an active suggestion inserts it.
            editor.accept_edit_prediction(&Default::default(), window, cx);
            assert!(!editor.has_active_edit_prediction());
            assert_eq!(editor.display_text(cx), "one.copilot2\ntwo\nthree\n");
            assert_eq!(editor.text(cx), "one.copilot2\ntwo\nthree\n");

            // When undoing the previously active suggestion is shown again.
            editor.undo(&Default::default(), window, cx);
            assert!(editor.has_active_edit_prediction());
            assert_eq!(editor.display_text(cx), "one.copilot2\ntwo\nthree\n");
            assert_eq!(editor.text(cx), "one.co\ntwo\nthree\n");

            // Hide suggestion.
            editor.cancel(&Default::default(), window, cx);
            assert!(!editor.has_active_edit_prediction());
            assert_eq!(editor.display_text(cx), "one.co\ntwo\nthree\n");
            assert_eq!(editor.text(cx), "one.co\ntwo\nthree\n");
        });

        // If an edit occurs outside of this editor but no suggestion is being shown,
        // we won't make it visible.
        cx.update_buffer(|buffer, cx| buffer.edit([(6..6, "p")], None, cx));
        cx.update_editor(|editor, _, cx| {
            assert!(!editor.has_active_edit_prediction());
            assert_eq!(editor.display_text(cx), "one.cop\ntwo\nthree\n");
            assert_eq!(editor.text(cx), "one.cop\ntwo\nthree\n");
        });
    }

    #[gpui::test(iterations = 10)]
    async fn test_accept_partial_copilot_suggestion(
        executor: BackgroundExecutor,
        cx: &mut TestAppContext,
    ) {
        // flaky
        init_test(cx, |settings| {
            settings.defaults.completions = Some(CompletionSettingsContent {
                words: Some(WordsCompletionMode::Disabled),
                words_min_length: Some(0),
                lsp_insert_mode: Some(LspInsertMode::Insert),
                ..Default::default()
            });
        });

        let (copilot, copilot_lsp) = Copilot::fake(cx);
        let mut cx = EditorLspTestContext::new_rust(
            lsp::ServerCapabilities {
                completion_provider: Some(lsp::CompletionOptions {
                    trigger_characters: Some(vec![".".to_string(), ":".to_string()]),
                    ..Default::default()
                }),
                ..Default::default()
            },
            cx,
        )
        .await;
        let copilot_provider = cx.new(|_| CopilotEditPredictionDelegate::new(copilot));
        cx.update_editor(|editor, window, cx| {
            editor.set_edit_prediction_provider(Some(copilot_provider), window, cx)
        });

        // Setup the editor with a completion request.
        cx.set_state(indoc! {"
            oneˇ
            two
            three
        "});
        cx.simulate_keystroke(".");
        drop(handle_completion_request(
            &mut cx,
            indoc! {"
                one.|<>
                two
                three
            "},
            vec![],
        ));
        handle_copilot_completion_request(
            &copilot_lsp,
            vec![crate::request::NextEditSuggestion {
                text: "one.copilot1".into(),
                range: lsp::Range::new(lsp::Position::new(0, 0), lsp::Position::new(0, 4)),
                command: None,
                text_document: lsp::VersionedTextDocumentIdentifier {
                    uri: Uri::from_file_path(path!("/root/dir/file.rs")).unwrap(),
                    version: 0,
                },
            }],
        );
        executor.advance_clock(COPILOT_DEBOUNCE_TIMEOUT);
        cx.update_editor(|editor, window, cx| {
            assert!(editor.has_active_edit_prediction());

            // Accepting the first word of the suggestion should only accept the first word and still show the rest.
            editor.accept_partial_edit_prediction(EditPredictionGranularity::Word, window, cx);

            assert!(editor.has_active_edit_prediction());
            assert_eq!(editor.text(cx), "one.copilot\ntwo\nthree\n");
            assert_eq!(editor.display_text(cx), "one.copilot1\ntwo\nthree\n");

            // Accepting next word should accept the non-word and copilot suggestion should be gone
            editor.accept_partial_edit_prediction(EditPredictionGranularity::Word, window, cx);

            assert!(!editor.has_active_edit_prediction());
            assert_eq!(editor.text(cx), "one.copilot1\ntwo\nthree\n");
            assert_eq!(editor.display_text(cx), "one.copilot1\ntwo\nthree\n");
        });

        // Reset the editor and check non-word and whitespace completion
        cx.set_state(indoc! {"
            oneˇ
            two
            three
        "});
        cx.simulate_keystroke(".");
        drop(handle_completion_request(
            &mut cx,
            indoc! {"
                one.|<>
                two
                three
            "},
            vec![],
        ));
        handle_copilot_completion_request(
            &copilot_lsp,
            vec![crate::request::NextEditSuggestion {
                text: "one.123. copilot\n 456".into(),
                range: lsp::Range::new(lsp::Position::new(0, 0), lsp::Position::new(0, 4)),
                command: None,
                text_document: lsp::VersionedTextDocumentIdentifier {
                    uri: Uri::from_file_path(path!("/root/dir/file.rs")).unwrap(),
                    version: 0,
                },
            }],
        );
        executor.advance_clock(COPILOT_DEBOUNCE_TIMEOUT);
        cx.update_editor(|editor, window, cx| {
            assert!(editor.has_active_edit_prediction());

            // Accepting the first word (non-word) of the suggestion should only accept the first word and still show the rest.
            editor.accept_partial_edit_prediction(EditPredictionGranularity::Word, window, cx);
            assert!(editor.has_active_edit_prediction());
            assert_eq!(editor.text(cx), "one.123. \ntwo\nthree\n");
            assert_eq!(
                editor.display_text(cx),
                "one.123. copilot\n 456\ntwo\nthree\n"
            );

            // Accepting next word should accept the next word and copilot suggestion should still exist
            editor.accept_partial_edit_prediction(EditPredictionGranularity::Word, window, cx);
            assert!(editor.has_active_edit_prediction());
            assert_eq!(editor.text(cx), "one.123. copilot\ntwo\nthree\n");
            assert_eq!(
                editor.display_text(cx),
                "one.123. copilot\n 456\ntwo\nthree\n"
            );

            // Accepting the whitespace should accept the non-word/whitespaces with newline and copilot suggestion should be gone
            editor.accept_partial_edit_prediction(EditPredictionGranularity::Word, window, cx);
            assert!(!editor.has_active_edit_prediction());
            assert_eq!(editor.text(cx), "one.123. copilot\n 456\ntwo\nthree\n");
            assert_eq!(
                editor.display_text(cx),
                "one.123. copilot\n 456\ntwo\nthree\n"
            );
        });
    }

    #[gpui::test]
    async fn test_copilot_completion_invalidation(
        executor: BackgroundExecutor,
        cx: &mut TestAppContext,
    ) {
        init_test(cx, |_| {});

        let (copilot, copilot_lsp) = Copilot::fake(cx);
        let mut cx = EditorLspTestContext::new_rust(
            lsp::ServerCapabilities {
                completion_provider: Some(lsp::CompletionOptions {
                    trigger_characters: Some(vec![".".to_string(), ":".to_string()]),
                    ..Default::default()
                }),
                ..Default::default()
            },
            cx,
        )
        .await;
        let copilot_provider = cx.new(|_| CopilotEditPredictionDelegate::new(copilot));
        cx.update_editor(|editor, window, cx| {
            editor.set_edit_prediction_provider(Some(copilot_provider), window, cx)
        });

        cx.set_state(indoc! {"
            one
            twˇ
            three
        "});

        handle_copilot_completion_request(
            &copilot_lsp,
            vec![crate::request::NextEditSuggestion {
                text: "two.foo()".into(),
                range: lsp::Range::new(lsp::Position::new(1, 0), lsp::Position::new(1, 2)),
                command: None,
                text_document: lsp::VersionedTextDocumentIdentifier {
                    uri: Uri::from_file_path(path!("/root/dir/file.rs")).unwrap(),
                    version: 0,
                },
            }],
        );
        cx.update_editor(|editor, window, cx| {
            editor.show_edit_prediction(&Default::default(), window, cx)
        });
        executor.advance_clock(COPILOT_DEBOUNCE_TIMEOUT);
        cx.update_editor(|editor, window, cx| {
            assert!(editor.has_active_edit_prediction());
            assert_eq!(editor.display_text(cx), "one\ntwo.foo()\nthree\n");
            assert_eq!(editor.text(cx), "one\ntw\nthree\n");

            editor.backspace(&Default::default(), window, cx);
        });
        executor.advance_clock(COPILOT_DEBOUNCE_TIMEOUT);
        cx.run_until_parked();
        cx.update_editor(|editor, window, cx| {
            assert!(editor.has_active_edit_prediction());
            assert_eq!(editor.display_text(cx), "one\ntwo.foo()\nthree\n");
            assert_eq!(editor.text(cx), "one\nt\nthree\n");

            editor.backspace(&Default::default(), window, cx);
        });
        executor.advance_clock(COPILOT_DEBOUNCE_TIMEOUT);
        cx.run_until_parked();
        cx.update_editor(|editor, window, cx| {
            assert!(editor.has_active_edit_prediction());
            assert_eq!(editor.display_text(cx), "one\ntwo.foo()\nthree\n");
            assert_eq!(editor.text(cx), "one\n\nthree\n");
            // Deleting across the original suggestion range invalidates it.
            editor.backspace(&Default::default(), window, cx);
            assert!(!editor.has_active_edit_prediction());
            assert_eq!(editor.display_text(cx), "one\nthree\n");
            assert_eq!(editor.text(cx), "one\nthree\n");

            // Undoing the deletion restores the suggestion.
            editor.undo(&Default::default(), window, cx);
            assert!(editor.has_active_edit_prediction());
            assert_eq!(editor.display_text(cx), "one\ntwo.foo()\nthree\n");
            assert_eq!(editor.text(cx), "one\n\nthree\n");
        });
    }

    #[gpui::test]
    async fn test_copilot_multibuffer(executor: BackgroundExecutor, cx: &mut TestAppContext) {
        init_test(cx, |_| {});

        let (copilot, copilot_lsp) = Copilot::fake(cx);

        let buffer_1 = cx.new(|cx| Buffer::local("a = 1\nb = 2\n", cx));
        let buffer_2 = cx.new(|cx| Buffer::local("c = 3\nd = 4\n", cx));
        let multibuffer = cx.new(|cx| {
            let mut multibuffer = MultiBuffer::new(language::Capability::ReadWrite);
            multibuffer.push_excerpts(
                buffer_1.clone(),
                [ExcerptRange::new(Point::new(0, 0)..Point::new(2, 0))],
                cx,
            );
            multibuffer.push_excerpts(
                buffer_2.clone(),
                [ExcerptRange::new(Point::new(0, 0)..Point::new(2, 0))],
                cx,
            );
            multibuffer
        });
        let editor =
            cx.add_window(|window, cx| Editor::for_multibuffer(multibuffer, None, window, cx));
        editor
            .update(cx, |editor, window, cx| {
                use gpui::Focusable;
                window.focus(&editor.focus_handle(cx), cx);
            })
            .unwrap();
        let copilot_provider = cx.new(|_| CopilotEditPredictionDelegate::new(copilot));
        editor
            .update(cx, |editor, window, cx| {
                editor.set_edit_prediction_provider(Some(copilot_provider), window, cx)
            })
            .unwrap();

        handle_copilot_completion_request(
            &copilot_lsp,
            vec![crate::request::NextEditSuggestion {
                text: "b = 2 + a".into(),
                range: lsp::Range::new(lsp::Position::new(1, 0), lsp::Position::new(1, 5)),
                command: None,
                text_document: lsp::VersionedTextDocumentIdentifier {
                    uri: Uri::from_file_path(path!("/root/dir/file.rs")).unwrap(),
                    version: 0,
                },
            }],
        );
        _ = editor.update(cx, |editor, window, cx| {
            // Ensure copilot suggestions are shown for the first excerpt.
            editor.change_selections(SelectionEffects::no_scroll(), window, cx, |s| {
                s.select_ranges([Point::new(1, 5)..Point::new(1, 5)])
            });
            editor.show_edit_prediction(&Default::default(), window, cx);
        });
        executor.advance_clock(COPILOT_DEBOUNCE_TIMEOUT);
        _ = editor.update(cx, |editor, _, cx| {
            assert!(editor.has_active_edit_prediction());
            assert_eq!(
                editor.display_text(cx),
                "\n\na = 1\nb = 2 + a\n\n\n\nc = 3\nd = 4\n"
            );
            assert_eq!(editor.text(cx), "a = 1\nb = 2\n\nc = 3\nd = 4\n");
        });

        handle_copilot_completion_request(
            &copilot_lsp,
            vec![crate::request::NextEditSuggestion {
                text: "d = 4 + c".into(),
                range: lsp::Range::new(lsp::Position::new(1, 0), lsp::Position::new(1, 6)),
                command: None,
                text_document: lsp::VersionedTextDocumentIdentifier {
                    uri: Uri::from_file_path(path!("/root/dir/file.rs")).unwrap(),
                    version: 0,
                },
            }],
        );
        _ = editor.update(cx, |editor, window, cx| {
            // Move to another excerpt, ensuring the suggestion gets cleared.
            editor.change_selections(SelectionEffects::no_scroll(), window, cx, |s| {
                s.select_ranges([Point::new(4, 5)..Point::new(4, 5)])
            });
            assert!(!editor.has_active_edit_prediction());
            assert_eq!(
                editor.display_text(cx),
                "\n\na = 1\nb = 2\n\n\n\nc = 3\nd = 4\n"
            );
            assert_eq!(editor.text(cx), "a = 1\nb = 2\n\nc = 3\nd = 4\n");

            // Type a character, ensuring we don't even try to interpolate the previous suggestion.
            editor.handle_input(" ", window, cx);
            assert!(!editor.has_active_edit_prediction());
            assert_eq!(
                editor.display_text(cx),
                "\n\na = 1\nb = 2\n\n\n\nc = 3\nd = 4 \n"
            );
            assert_eq!(editor.text(cx), "a = 1\nb = 2\n\nc = 3\nd = 4 \n");
        });

        // Ensure the new suggestion is displayed when the debounce timeout expires.
        executor.advance_clock(COPILOT_DEBOUNCE_TIMEOUT);
        _ = editor.update(cx, |editor, _, cx| {
            assert!(editor.has_active_edit_prediction());
            assert_eq!(
                editor.display_text(cx),
                "\n\na = 1\nb = 2\n\n\n\nc = 3\nd = 4 + c\n"
            );
            assert_eq!(editor.text(cx), "a = 1\nb = 2\n\nc = 3\nd = 4 \n");
        });
    }

    #[gpui::test]
    async fn test_copilot_does_not_prevent_completion_triggers(
        executor: BackgroundExecutor,
        cx: &mut TestAppContext,
    ) {
        init_test(cx, |_| {});

        let (copilot, copilot_lsp) = Copilot::fake(cx);
        let mut cx = EditorLspTestContext::new_rust(
            lsp::ServerCapabilities {
                completion_provider: Some(lsp::CompletionOptions {
                    trigger_characters: Some(vec![".".to_string(), ":".to_string()]),
                    ..lsp::CompletionOptions::default()
                }),
                ..lsp::ServerCapabilities::default()
            },
            cx,
        )
        .await;
        let copilot_provider = cx.new(|_| CopilotEditPredictionDelegate::new(copilot));
        cx.update_editor(|editor, window, cx| {
            editor.set_edit_prediction_provider(Some(copilot_provider), window, cx)
        });

        cx.set_state(indoc! {"
                one
                twˇ
                three
            "});

        drop(handle_completion_request(
            &mut cx,
            indoc! {"
                one
                tw|<>
                three
            "},
            vec!["completion_a", "completion_b"],
        ));
        handle_copilot_completion_request(
            &copilot_lsp,
            vec![crate::request::NextEditSuggestion {
                text: "two.foo()".into(),
                range: lsp::Range::new(lsp::Position::new(1, 0), lsp::Position::new(1, 2)),
                command: None,
                text_document: lsp::VersionedTextDocumentIdentifier {
                    uri: Uri::from_file_path(path!("/root/dir/file.rs")).unwrap(),
                    version: 0,
                },
            }],
        );
        cx.update_editor(|editor, window, cx| {
            editor.show_edit_prediction(&Default::default(), window, cx)
        });
        executor.advance_clock(COPILOT_DEBOUNCE_TIMEOUT);
        cx.update_editor(|editor, _, cx| {
            assert!(!editor.context_menu_visible());
            assert!(editor.has_active_edit_prediction());
            assert_eq!(editor.display_text(cx), "one\ntwo.foo()\nthree\n");
            assert_eq!(editor.text(cx), "one\ntw\nthree\n");
        });

        cx.simulate_keystroke("o");
        drop(handle_completion_request(
            &mut cx,
            indoc! {"
                one
                two|<>
                three
            "},
            vec!["completion_a_2", "completion_b_2"],
        ));
        handle_copilot_completion_request(
            &copilot_lsp,
            vec![crate::request::NextEditSuggestion {
                text: "two.foo()".into(),
                range: lsp::Range::new(lsp::Position::new(1, 0), lsp::Position::new(1, 3)),
                command: None,
                text_document: lsp::VersionedTextDocumentIdentifier {
                    uri: Uri::from_file_path(path!("/root/dir/file.rs")).unwrap(),
                    version: 0,
                },
            }],
        );
        executor.advance_clock(COPILOT_DEBOUNCE_TIMEOUT);
        cx.update_editor(|editor, _, cx| {
            assert!(!editor.context_menu_visible());
            assert!(editor.has_active_edit_prediction());
            assert_eq!(editor.display_text(cx), "one\ntwo.foo()\nthree\n");
            assert_eq!(editor.text(cx), "one\ntwo\nthree\n");
        });

        cx.simulate_keystroke(".");
        drop(handle_completion_request(
            &mut cx,
            indoc! {"
                one
                two.|<>
                three
            "},
            vec!["something_else()"],
        ));
        handle_copilot_completion_request(
            &copilot_lsp,
            vec![crate::request::NextEditSuggestion {
                text: "two.foo()".into(),
                range: lsp::Range::new(lsp::Position::new(1, 0), lsp::Position::new(1, 4)),
                command: None,
                text_document: lsp::VersionedTextDocumentIdentifier {
                    uri: Uri::from_file_path(path!("/root/dir/file.rs")).unwrap(),
                    version: 0,
                },
            }],
        );
        executor.advance_clock(COPILOT_DEBOUNCE_TIMEOUT);
        cx.update_editor(|editor, _, cx| {
            assert!(editor.context_menu_visible());
            assert!(editor.has_active_edit_prediction());
            assert_eq!(editor.text(cx), "one\ntwo.\nthree\n");
            assert_eq!(editor.display_text(cx), "one\ntwo.\nthree\n");
        });
    }

    #[gpui::test]
    async fn test_copilot_disabled_globs(executor: BackgroundExecutor, cx: &mut TestAppContext) {
        init_test(cx, |settings| {
            settings
                .edit_predictions
                .get_or_insert(Default::default())
                .disabled_globs = Some(vec![".env*".to_string()]);
        });

        let (copilot, copilot_lsp) = Copilot::fake(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/test"),
            json!({
                ".env": "SECRET=something\n",
                "README.md": "hello\nworld\nhow\nare\nyou\ntoday"
            }),
        )
        .await;
        let project = Project::test(fs, [path!("/test").as_ref()], cx).await;

        let private_buffer = project
            .update(cx, |project, cx| {
                project.open_local_buffer(path!("/test/.env"), cx)
            })
            .await
            .unwrap();
        let public_buffer = project
            .update(cx, |project, cx| {
                project.open_local_buffer(path!("/test/README.md"), cx)
            })
            .await
            .unwrap();

        let multibuffer = cx.new(|cx| {
            let mut multibuffer = MultiBuffer::new(language::Capability::ReadWrite);
            multibuffer.push_excerpts(
                private_buffer.clone(),
                [ExcerptRange::new(Point::new(0, 0)..Point::new(1, 0))],
                cx,
            );
            multibuffer.push_excerpts(
                public_buffer.clone(),
                [ExcerptRange::new(Point::new(0, 0)..Point::new(6, 0))],
                cx,
            );
            multibuffer
        });
        let editor =
            cx.add_window(|window, cx| Editor::for_multibuffer(multibuffer, None, window, cx));
        editor
            .update(cx, |editor, window, cx| {
                use gpui::Focusable;
                window.focus(&editor.focus_handle(cx), cx)
            })
            .unwrap();
        let copilot_provider = cx.new(|_| CopilotEditPredictionDelegate::new(copilot));
        editor
            .update(cx, |editor, window, cx| {
                editor.set_edit_prediction_provider(Some(copilot_provider), window, cx)
            })
            .unwrap();

        let mut copilot_requests = copilot_lsp
            .set_request_handler::<crate::request::NextEditSuggestions, _, _>(
                move |_params, _cx| async move {
                    Ok(crate::request::NextEditSuggestionsResult {
                        edits: vec![crate::request::NextEditSuggestion {
                            text: "next line".into(),
                            range: lsp::Range::new(
                                lsp::Position::new(1, 0),
                                lsp::Position::new(1, 0),
                            ),
                            command: None,
                            text_document: lsp::VersionedTextDocumentIdentifier {
                                uri: Uri::from_file_path(path!("/root/dir/file.rs")).unwrap(),
                                version: 0,
                            },
                        }],
                    })
                },
            );

        _ = editor.update(cx, |editor, window, cx| {
            editor.change_selections(SelectionEffects::no_scroll(), window, cx, |selections| {
                selections.select_ranges([Point::new(0, 0)..Point::new(0, 0)])
            });
            editor.refresh_edit_prediction(true, false, window, cx);
        });

        executor.advance_clock(COPILOT_DEBOUNCE_TIMEOUT);
        assert!(copilot_requests.try_next().is_err());

        _ = editor.update(cx, |editor, window, cx| {
            editor.change_selections(SelectionEffects::no_scroll(), window, cx, |s| {
                s.select_ranges([Point::new(5, 0)..Point::new(5, 0)])
            });
            editor.refresh_edit_prediction(true, false, window, cx);
        });

        executor.advance_clock(COPILOT_DEBOUNCE_TIMEOUT);
        assert!(copilot_requests.try_next().is_ok());
    }

    fn handle_copilot_completion_request(
        lsp: &lsp::FakeLanguageServer,
        completions: Vec<crate::request::NextEditSuggestion>,
    ) {
        lsp.set_request_handler::<crate::request::NextEditSuggestions, _, _>(
            move |_params, _cx| {
                let completions = completions.clone();
                async move {
                    Ok(crate::request::NextEditSuggestionsResult {
                        edits: completions.clone(),
                    })
                }
            },
        );
    }

    fn handle_completion_request(
        cx: &mut EditorLspTestContext,
        marked_string: &str,
        completions: Vec<&'static str>,
    ) -> impl Future<Output = ()> {
        let complete_from_marker: TextRangeMarker = '|'.into();
        let replace_range_marker: TextRangeMarker = ('<', '>').into();
        let (_, mut marked_ranges) = marked_text_ranges_by(
            marked_string,
            vec![complete_from_marker, replace_range_marker.clone()],
        );

        let range = marked_ranges.remove(&replace_range_marker).unwrap()[0].clone();
        let replace_range =
            cx.to_lsp_range(MultiBufferOffset(range.start)..MultiBufferOffset(range.end));

        let mut request =
            cx.set_request_handler::<lsp::request::Completion, _, _>(move |url, params, _| {
                let completions = completions.clone();
                async move {
                    assert_eq!(params.text_document_position.text_document.uri, url.clone());
                    Ok(Some(lsp::CompletionResponse::Array(
                        completions
                            .iter()
                            .map(|completion_text| lsp::CompletionItem {
                                label: completion_text.to_string(),
                                text_edit: Some(lsp::CompletionTextEdit::Edit(lsp::TextEdit {
                                    range: replace_range,
                                    new_text: completion_text.to_string(),
                                })),
                                ..Default::default()
                            })
                            .collect(),
                    )))
                }
            });

        async move {
            request.next().await;
        }
    }

    fn init_test(cx: &mut TestAppContext, f: fn(&mut AllLanguageSettingsContent)) {
        cx.update(|cx| {
            let store = SettingsStore::test(cx);
            cx.set_global(store);
            theme::init(theme::LoadThemes::JustBase, cx);
            SettingsStore::update_global(cx, |store: &mut SettingsStore, cx| {
                store.update_user_settings(cx, |settings| f(&mut settings.project.all_languages));
            });
        });
    }
}
