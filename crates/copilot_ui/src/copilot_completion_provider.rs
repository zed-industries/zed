use anyhow::Result;
use client::telemetry::Telemetry;
use copilot::Copilot;
use editor::{Direction, InlineCompletionProvider};
use gpui::{AppContext, EntityId, Model, ModelContext, Task};
use language::language_settings::AllLanguageSettings;
use language::{language_settings::all_language_settings, Buffer, OffsetRangeExt, ToOffset};
use settings::Settings;
use std::{path::Path, sync::Arc, time::Duration};

pub const COPILOT_DEBOUNCE_TIMEOUT: Duration = Duration::from_millis(75);

pub struct CopilotCompletionProvider {
    cycled: bool,
    buffer_id: Option<EntityId>,
    completions: Vec<copilot::Completion>,
    active_completion_index: usize,
    file_extension: Option<String>,
    pending_refresh: Task<Result<()>>,
    pending_cycling_refresh: Task<Result<()>>,
    copilot: Model<Copilot>,
    telemetry: Option<Arc<Telemetry>>,
}

impl CopilotCompletionProvider {
    pub fn new(copilot: Model<Copilot>) -> Self {
        Self {
            cycled: false,
            buffer_id: None,
            completions: Vec::new(),
            active_completion_index: 0,
            file_extension: None,
            pending_refresh: Task::ready(Ok(())),
            pending_cycling_refresh: Task::ready(Ok(())),
            copilot,
            telemetry: None,
        }
    }

    pub fn with_telemetry(mut self, telemetry: Arc<Telemetry>) -> Self {
        self.telemetry = Some(telemetry);
        self
    }

    fn active_completion(&self) -> Option<&copilot::Completion> {
        self.completions.get(self.active_completion_index)
    }

    fn push_completion(&mut self, new_completion: copilot::Completion) {
        for completion in &self.completions {
            if completion.text == new_completion.text && completion.range == new_completion.range {
                return;
            }
        }
        self.completions.push(new_completion);
    }
}

impl InlineCompletionProvider for CopilotCompletionProvider {
    fn is_enabled(
        &self,
        buffer: &Model<Buffer>,
        cursor_position: language::Anchor,
        cx: &AppContext,
    ) -> bool {
        if !self.copilot.read(cx).status().is_authorized() {
            return false;
        }

        let buffer = buffer.read(cx);
        let file = buffer.file();
        let language = buffer.language_at(cursor_position);
        let settings = all_language_settings(file, cx);
        settings.inline_completions_enabled(language.as_ref(), file.map(|f| f.path().as_ref()))
    }

    fn refresh(
        &mut self,
        buffer: Model<Buffer>,
        cursor_position: language::Anchor,
        debounce: bool,
        cx: &mut ModelContext<Self>,
    ) {
        let copilot = self.copilot.clone();
        self.pending_refresh = cx.spawn(|this, mut cx| async move {
            if debounce {
                cx.background_executor()
                    .timer(COPILOT_DEBOUNCE_TIMEOUT)
                    .await;
            }

            let completions = copilot
                .update(&mut cx, |copilot, cx| {
                    copilot.completions(&buffer, cursor_position, cx)
                })?
                .await?;

            this.update(&mut cx, |this, cx| {
                if !completions.is_empty() {
                    this.cycled = false;
                    this.pending_cycling_refresh = Task::ready(Ok(()));
                    this.completions.clear();
                    this.active_completion_index = 0;
                    this.buffer_id = Some(buffer.entity_id());
                    this.file_extension = buffer.read(cx).file().and_then(|file| {
                        Some(
                            Path::new(file.file_name(cx))
                                .extension()?
                                .to_str()?
                                .to_string(),
                        )
                    });

                    for completion in completions {
                        this.push_completion(completion);
                    }
                    cx.notify();
                }
            })?;

            Ok(())
        });
    }

    fn cycle(
        &mut self,
        buffer: Model<Buffer>,
        cursor_position: language::Anchor,
        direction: Direction,
        cx: &mut ModelContext<Self>,
    ) {
        if self.cycled {
            match direction {
                Direction::Prev => {
                    self.active_completion_index = if self.active_completion_index == 0 {
                        self.completions.len().saturating_sub(1)
                    } else {
                        self.active_completion_index - 1
                    };
                }
                Direction::Next => {
                    if self.completions.len() == 0 {
                        self.active_completion_index = 0
                    } else {
                        self.active_completion_index =
                            (self.active_completion_index + 1) % self.completions.len();
                    }
                }
            }

            cx.notify();
        } else {
            let copilot = self.copilot.clone();
            self.pending_cycling_refresh = cx.spawn(|this, mut cx| async move {
                let completions = copilot
                    .update(&mut cx, |copilot, cx| {
                        copilot.completions_cycling(&buffer, cursor_position, cx)
                    })?
                    .await?;

                this.update(&mut cx, |this, cx| {
                    this.cycled = true;
                    this.file_extension = buffer.read(cx).file().and_then(|file| {
                        Some(
                            Path::new(file.file_name(cx))
                                .extension()?
                                .to_str()?
                                .to_string(),
                        )
                    });
                    for completion in completions {
                        this.push_completion(completion);
                    }
                    this.cycle(buffer, cursor_position, direction, cx);
                })?;

                Ok(())
            });
        }
    }

    fn accept(&mut self, cx: &mut ModelContext<Self>) {
        if let Some(completion) = self.active_completion() {
            self.copilot
                .update(cx, |copilot, cx| copilot.accept_completion(completion, cx))
                .detach_and_log_err(cx);
            if let Some(telemetry) = self.telemetry.as_ref() {
                telemetry.report_copilot_event(
                    Some(completion.uuid.clone()),
                    true,
                    self.file_extension.clone(),
                );
            }
        }
    }

    fn discard(&mut self, cx: &mut ModelContext<Self>) {
        let settings = AllLanguageSettings::get_global(cx);
        if !settings.copilot.feature_enabled {
            return;
        }

        self.copilot
            .update(cx, |copilot, cx| {
                copilot.discard_completions(&self.completions, cx)
            })
            .detach_and_log_err(cx);
        if let Some(telemetry) = self.telemetry.as_ref() {
            telemetry.report_copilot_event(None, false, self.file_extension.clone());
        }
    }

    fn active_completion_text(
        &self,
        buffer: &Model<Buffer>,
        cursor_position: language::Anchor,
        cx: &AppContext,
    ) -> Option<&str> {
        let buffer_id = buffer.entity_id();
        let buffer = buffer.read(cx);
        let completion = self.active_completion()?;
        if Some(buffer_id) != self.buffer_id
            || !completion.range.start.is_valid(buffer)
            || !completion.range.end.is_valid(buffer)
        {
            return None;
        }

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

        if completion_range.is_empty()
            && completion_range.start == cursor_position.to_offset(buffer)
        {
            let completion_text = &completion.text[prefix_len..completion.text.len() - suffix_len];
            if completion_text.trim().is_empty() {
                None
            } else {
                Some(completion_text)
            }
        } else {
            None
        }
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
    use editor::{
        test::editor_lsp_test_context::EditorLspTestContext, Editor, ExcerptRange, MultiBuffer,
    };
    use fs::FakeFs;
    use futures::StreamExt;
    use gpui::{BackgroundExecutor, BorrowAppContext, Context, TestAppContext};
    use indoc::indoc;
    use language::{
        language_settings::{AllLanguageSettings, AllLanguageSettingsContent},
        Point,
    };
    use project::Project;
    use serde_json::json;
    use settings::SettingsStore;
    use std::future::Future;
    use util::test::{marked_text_ranges_by, TextRangeMarker};

    #[gpui::test(iterations = 10)]
    async fn test_copilot(executor: BackgroundExecutor, cx: &mut TestAppContext) {
        // flaky
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
        let copilot_provider = cx.new_model(|_| CopilotCompletionProvider::new(copilot));
        cx.update_editor(|editor, cx| {
            editor.set_inline_completion_provider(Some(copilot_provider), cx)
        });

        // When inserting, ensure autocompletion is favored over Copilot suggestions.
        cx.set_state(indoc! {"
            oneˇ
            two
            three
        "});
        cx.simulate_keystroke(".");
        let _ = handle_completion_request(
            &mut cx,
            indoc! {"
                one.|<>
                two
                three
            "},
            vec!["completion_a", "completion_b"],
        );
        handle_copilot_completion_request(
            &copilot_lsp,
            vec![copilot::request::Completion {
                text: "one.copilot1".into(),
                range: lsp::Range::new(lsp::Position::new(0, 0), lsp::Position::new(0, 4)),
                ..Default::default()
            }],
            vec![],
        );
        executor.advance_clock(COPILOT_DEBOUNCE_TIMEOUT);
        cx.update_editor(|editor, cx| {
            assert!(editor.context_menu_visible());
            assert!(!editor.has_active_inline_completion(cx));

            // Confirming a completion inserts it and hides the context menu, without showing
            // the copilot suggestion afterwards.
            editor
                .confirm_completion(&Default::default(), cx)
                .unwrap()
                .detach();
            assert!(!editor.context_menu_visible());
            assert!(!editor.has_active_inline_completion(cx));
            assert_eq!(editor.text(cx), "one.completion_a\ntwo\nthree\n");
            assert_eq!(editor.display_text(cx), "one.completion_a\ntwo\nthree\n");
        });

        // Ensure Copilot suggestions are shown right away if no autocompletion is available.
        cx.set_state(indoc! {"
            oneˇ
            two
            three
        "});
        cx.simulate_keystroke(".");
        let _ = handle_completion_request(
            &mut cx,
            indoc! {"
                one.|<>
                two
                three
            "},
            vec![],
        );
        handle_copilot_completion_request(
            &copilot_lsp,
            vec![copilot::request::Completion {
                text: "one.copilot1".into(),
                range: lsp::Range::new(lsp::Position::new(0, 0), lsp::Position::new(0, 4)),
                ..Default::default()
            }],
            vec![],
        );
        executor.advance_clock(COPILOT_DEBOUNCE_TIMEOUT);
        cx.update_editor(|editor, cx| {
            assert!(!editor.context_menu_visible());
            assert!(editor.has_active_inline_completion(cx));
            assert_eq!(editor.display_text(cx), "one.copilot1\ntwo\nthree\n");
            assert_eq!(editor.text(cx), "one.\ntwo\nthree\n");
        });

        // Reset editor, and ensure autocompletion is still favored over Copilot suggestions.
        cx.set_state(indoc! {"
            oneˇ
            two
            three
        "});
        cx.simulate_keystroke(".");
        let _ = handle_completion_request(
            &mut cx,
            indoc! {"
                one.|<>
                two
                three
            "},
            vec!["completion_a", "completion_b"],
        );
        handle_copilot_completion_request(
            &copilot_lsp,
            vec![copilot::request::Completion {
                text: "one.copilot1".into(),
                range: lsp::Range::new(lsp::Position::new(0, 0), lsp::Position::new(0, 4)),
                ..Default::default()
            }],
            vec![],
        );
        executor.advance_clock(COPILOT_DEBOUNCE_TIMEOUT);
        cx.update_editor(|editor, cx| {
            assert!(editor.context_menu_visible());
            assert!(!editor.has_active_inline_completion(cx));

            // When hiding the context menu, the Copilot suggestion becomes visible.
            editor.cancel(&Default::default(), cx);
            assert!(!editor.context_menu_visible());
            assert!(editor.has_active_inline_completion(cx));
            assert_eq!(editor.display_text(cx), "one.copilot1\ntwo\nthree\n");
            assert_eq!(editor.text(cx), "one.\ntwo\nthree\n");
        });

        // Ensure existing completion is interpolated when inserting again.
        cx.simulate_keystroke("c");
        executor.run_until_parked();
        cx.update_editor(|editor, cx| {
            assert!(!editor.context_menu_visible());
            assert!(editor.has_active_inline_completion(cx));
            assert_eq!(editor.display_text(cx), "one.copilot1\ntwo\nthree\n");
            assert_eq!(editor.text(cx), "one.c\ntwo\nthree\n");
        });

        // After debouncing, new Copilot completions should be requested.
        handle_copilot_completion_request(
            &copilot_lsp,
            vec![copilot::request::Completion {
                text: "one.copilot2".into(),
                range: lsp::Range::new(lsp::Position::new(0, 0), lsp::Position::new(0, 5)),
                ..Default::default()
            }],
            vec![],
        );
        executor.advance_clock(COPILOT_DEBOUNCE_TIMEOUT);
        cx.update_editor(|editor, cx| {
            assert!(!editor.context_menu_visible());
            assert!(editor.has_active_inline_completion(cx));
            assert_eq!(editor.display_text(cx), "one.copilot2\ntwo\nthree\n");
            assert_eq!(editor.text(cx), "one.c\ntwo\nthree\n");

            // Canceling should remove the active Copilot suggestion.
            editor.cancel(&Default::default(), cx);
            assert!(!editor.has_active_inline_completion(cx));
            assert_eq!(editor.display_text(cx), "one.c\ntwo\nthree\n");
            assert_eq!(editor.text(cx), "one.c\ntwo\nthree\n");

            // After canceling, tabbing shouldn't insert the previously shown suggestion.
            editor.tab(&Default::default(), cx);
            assert!(!editor.has_active_inline_completion(cx));
            assert_eq!(editor.display_text(cx), "one.c   \ntwo\nthree\n");
            assert_eq!(editor.text(cx), "one.c   \ntwo\nthree\n");

            // When undoing the previously active suggestion is shown again.
            editor.undo(&Default::default(), cx);
            assert!(editor.has_active_inline_completion(cx));
            assert_eq!(editor.display_text(cx), "one.copilot2\ntwo\nthree\n");
            assert_eq!(editor.text(cx), "one.c\ntwo\nthree\n");
        });

        // If an edit occurs outside of this editor, the suggestion is still correctly interpolated.
        cx.update_buffer(|buffer, cx| buffer.edit([(5..5, "o")], None, cx));
        cx.update_editor(|editor, cx| {
            assert!(editor.has_active_inline_completion(cx));
            assert_eq!(editor.display_text(cx), "one.copilot2\ntwo\nthree\n");
            assert_eq!(editor.text(cx), "one.co\ntwo\nthree\n");

            // Tabbing when there is an active suggestion inserts it.
            editor.tab(&Default::default(), cx);
            assert!(!editor.has_active_inline_completion(cx));
            assert_eq!(editor.display_text(cx), "one.copilot2\ntwo\nthree\n");
            assert_eq!(editor.text(cx), "one.copilot2\ntwo\nthree\n");

            // When undoing the previously active suggestion is shown again.
            editor.undo(&Default::default(), cx);
            assert!(editor.has_active_inline_completion(cx));
            assert_eq!(editor.display_text(cx), "one.copilot2\ntwo\nthree\n");
            assert_eq!(editor.text(cx), "one.co\ntwo\nthree\n");

            // Hide suggestion.
            editor.cancel(&Default::default(), cx);
            assert!(!editor.has_active_inline_completion(cx));
            assert_eq!(editor.display_text(cx), "one.co\ntwo\nthree\n");
            assert_eq!(editor.text(cx), "one.co\ntwo\nthree\n");
        });

        // If an edit occurs outside of this editor but no suggestion is being shown,
        // we won't make it visible.
        cx.update_buffer(|buffer, cx| buffer.edit([(6..6, "p")], None, cx));
        cx.update_editor(|editor, cx| {
            assert!(!editor.has_active_inline_completion(cx));
            assert_eq!(editor.display_text(cx), "one.cop\ntwo\nthree\n");
            assert_eq!(editor.text(cx), "one.cop\ntwo\nthree\n");
        });

        // Reset the editor to verify how suggestions behave when tabbing on leading indentation.
        cx.update_editor(|editor, cx| {
            editor.set_text("fn foo() {\n  \n}", cx);
            editor.change_selections(None, cx, |s| {
                s.select_ranges([Point::new(1, 2)..Point::new(1, 2)])
            });
        });
        handle_copilot_completion_request(
            &copilot_lsp,
            vec![copilot::request::Completion {
                text: "    let x = 4;".into(),
                range: lsp::Range::new(lsp::Position::new(1, 0), lsp::Position::new(1, 2)),
                ..Default::default()
            }],
            vec![],
        );

        cx.update_editor(|editor, cx| editor.next_inline_completion(&Default::default(), cx));
        executor.advance_clock(COPILOT_DEBOUNCE_TIMEOUT);
        cx.update_editor(|editor, cx| {
            assert!(editor.has_active_inline_completion(cx));
            assert_eq!(editor.display_text(cx), "fn foo() {\n    let x = 4;\n}");
            assert_eq!(editor.text(cx), "fn foo() {\n  \n}");

            // Tabbing inside of leading whitespace inserts indentation without accepting the suggestion.
            editor.tab(&Default::default(), cx);
            assert!(editor.has_active_inline_completion(cx));
            assert_eq!(editor.text(cx), "fn foo() {\n    \n}");
            assert_eq!(editor.display_text(cx), "fn foo() {\n    let x = 4;\n}");

            // Tabbing again accepts the suggestion.
            editor.tab(&Default::default(), cx);
            assert!(!editor.has_active_inline_completion(cx));
            assert_eq!(editor.text(cx), "fn foo() {\n    let x = 4;\n}");
            assert_eq!(editor.display_text(cx), "fn foo() {\n    let x = 4;\n}");
        });
    }

    #[gpui::test(iterations = 10)]
    async fn test_accept_partial_copilot_suggestion(
        executor: BackgroundExecutor,
        cx: &mut TestAppContext,
    ) {
        // flaky
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
        let copilot_provider = cx.new_model(|_| CopilotCompletionProvider::new(copilot));
        cx.update_editor(|editor, cx| {
            editor.set_inline_completion_provider(Some(copilot_provider), cx)
        });

        // Setup the editor with a completion request.
        cx.set_state(indoc! {"
            oneˇ
            two
            three
        "});
        cx.simulate_keystroke(".");
        let _ = handle_completion_request(
            &mut cx,
            indoc! {"
                one.|<>
                two
                three
            "},
            vec![],
        );
        handle_copilot_completion_request(
            &copilot_lsp,
            vec![copilot::request::Completion {
                text: "one.copilot1".into(),
                range: lsp::Range::new(lsp::Position::new(0, 0), lsp::Position::new(0, 4)),
                ..Default::default()
            }],
            vec![],
        );
        executor.advance_clock(COPILOT_DEBOUNCE_TIMEOUT);
        cx.update_editor(|editor, cx| {
            assert!(editor.has_active_inline_completion(cx));

            // Accepting the first word of the suggestion should only accept the first word and still show the rest.
            editor.accept_partial_inline_completion(&Default::default(), cx);
            assert!(editor.has_active_inline_completion(cx));
            assert_eq!(editor.text(cx), "one.copilot\ntwo\nthree\n");
            assert_eq!(editor.display_text(cx), "one.copilot1\ntwo\nthree\n");

            // Accepting next word should accept the non-word and copilot suggestion should be gone
            editor.accept_partial_inline_completion(&Default::default(), cx);
            assert!(!editor.has_active_inline_completion(cx));
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
        let _ = handle_completion_request(
            &mut cx,
            indoc! {"
                one.|<>
                two
                three
            "},
            vec![],
        );
        handle_copilot_completion_request(
            &copilot_lsp,
            vec![copilot::request::Completion {
                text: "one.123. copilot\n 456".into(),
                range: lsp::Range::new(lsp::Position::new(0, 0), lsp::Position::new(0, 4)),
                ..Default::default()
            }],
            vec![],
        );
        executor.advance_clock(COPILOT_DEBOUNCE_TIMEOUT);
        cx.update_editor(|editor, cx| {
            assert!(editor.has_active_inline_completion(cx));

            // Accepting the first word (non-word) of the suggestion should only accept the first word and still show the rest.
            editor.accept_partial_inline_completion(&Default::default(), cx);
            assert!(editor.has_active_inline_completion(cx));
            assert_eq!(editor.text(cx), "one.123. \ntwo\nthree\n");
            assert_eq!(
                editor.display_text(cx),
                "one.123. copilot\n 456\ntwo\nthree\n"
            );

            // Accepting next word should accept the next word and copilot suggestion should still exist
            editor.accept_partial_inline_completion(&Default::default(), cx);
            assert!(editor.has_active_inline_completion(cx));
            assert_eq!(editor.text(cx), "one.123. copilot\ntwo\nthree\n");
            assert_eq!(
                editor.display_text(cx),
                "one.123. copilot\n 456\ntwo\nthree\n"
            );

            // Accepting the whitespace should accept the non-word/whitespaces with newline and copilot suggestion should be gone
            editor.accept_partial_inline_completion(&Default::default(), cx);
            assert!(!editor.has_active_inline_completion(cx));
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
        let copilot_provider = cx.new_model(|_| CopilotCompletionProvider::new(copilot));
        cx.update_editor(|editor, cx| {
            editor.set_inline_completion_provider(Some(copilot_provider), cx)
        });

        cx.set_state(indoc! {"
            one
            twˇ
            three
        "});

        handle_copilot_completion_request(
            &copilot_lsp,
            vec![copilot::request::Completion {
                text: "two.foo()".into(),
                range: lsp::Range::new(lsp::Position::new(1, 0), lsp::Position::new(1, 2)),
                ..Default::default()
            }],
            vec![],
        );
        cx.update_editor(|editor, cx| editor.next_inline_completion(&Default::default(), cx));
        executor.advance_clock(COPILOT_DEBOUNCE_TIMEOUT);
        cx.update_editor(|editor, cx| {
            assert!(editor.has_active_inline_completion(cx));
            assert_eq!(editor.display_text(cx), "one\ntwo.foo()\nthree\n");
            assert_eq!(editor.text(cx), "one\ntw\nthree\n");

            editor.backspace(&Default::default(), cx);
            assert!(editor.has_active_inline_completion(cx));
            assert_eq!(editor.display_text(cx), "one\ntwo.foo()\nthree\n");
            assert_eq!(editor.text(cx), "one\nt\nthree\n");

            editor.backspace(&Default::default(), cx);
            assert!(editor.has_active_inline_completion(cx));
            assert_eq!(editor.display_text(cx), "one\ntwo.foo()\nthree\n");
            assert_eq!(editor.text(cx), "one\n\nthree\n");

            // Deleting across the original suggestion range invalidates it.
            editor.backspace(&Default::default(), cx);
            assert!(!editor.has_active_inline_completion(cx));
            assert_eq!(editor.display_text(cx), "one\nthree\n");
            assert_eq!(editor.text(cx), "one\nthree\n");

            // Undoing the deletion restores the suggestion.
            editor.undo(&Default::default(), cx);
            assert!(editor.has_active_inline_completion(cx));
            assert_eq!(editor.display_text(cx), "one\ntwo.foo()\nthree\n");
            assert_eq!(editor.text(cx), "one\n\nthree\n");
        });
    }

    #[gpui::test]
    async fn test_copilot_multibuffer(executor: BackgroundExecutor, cx: &mut TestAppContext) {
        init_test(cx, |_| {});

        let (copilot, copilot_lsp) = Copilot::fake(cx);

        let buffer_1 = cx.new_model(|cx| Buffer::local("a = 1\nb = 2\n", cx));
        let buffer_2 = cx.new_model(|cx| Buffer::local("c = 3\nd = 4\n", cx));
        let multibuffer = cx.new_model(|cx| {
            let mut multibuffer = MultiBuffer::new(0, language::Capability::ReadWrite);
            multibuffer.push_excerpts(
                buffer_1.clone(),
                [ExcerptRange {
                    context: Point::new(0, 0)..Point::new(2, 0),
                    primary: None,
                }],
                cx,
            );
            multibuffer.push_excerpts(
                buffer_2.clone(),
                [ExcerptRange {
                    context: Point::new(0, 0)..Point::new(2, 0),
                    primary: None,
                }],
                cx,
            );
            multibuffer
        });
        let editor = cx.add_window(|cx| Editor::for_multibuffer(multibuffer, None, cx));
        editor.update(cx, |editor, cx| editor.focus(cx)).unwrap();
        let copilot_provider = cx.new_model(|_| CopilotCompletionProvider::new(copilot));
        editor
            .update(cx, |editor, cx| {
                editor.set_inline_completion_provider(Some(copilot_provider), cx)
            })
            .unwrap();

        handle_copilot_completion_request(
            &copilot_lsp,
            vec![copilot::request::Completion {
                text: "b = 2 + a".into(),
                range: lsp::Range::new(lsp::Position::new(1, 0), lsp::Position::new(1, 5)),
                ..Default::default()
            }],
            vec![],
        );
        _ = editor.update(cx, |editor, cx| {
            // Ensure copilot suggestions are shown for the first excerpt.
            editor.change_selections(None, cx, |s| {
                s.select_ranges([Point::new(1, 5)..Point::new(1, 5)])
            });
            editor.next_inline_completion(&Default::default(), cx);
        });
        executor.advance_clock(COPILOT_DEBOUNCE_TIMEOUT);
        _ = editor.update(cx, |editor, cx| {
            assert!(editor.has_active_inline_completion(cx));
            assert_eq!(
                editor.display_text(cx),
                "\n\na = 1\nb = 2 + a\n\n\n\nc = 3\nd = 4\n"
            );
            assert_eq!(editor.text(cx), "a = 1\nb = 2\n\nc = 3\nd = 4\n");
        });

        handle_copilot_completion_request(
            &copilot_lsp,
            vec![copilot::request::Completion {
                text: "d = 4 + c".into(),
                range: lsp::Range::new(lsp::Position::new(1, 0), lsp::Position::new(1, 6)),
                ..Default::default()
            }],
            vec![],
        );
        _ = editor.update(cx, |editor, cx| {
            // Move to another excerpt, ensuring the suggestion gets cleared.
            editor.change_selections(None, cx, |s| {
                s.select_ranges([Point::new(4, 5)..Point::new(4, 5)])
            });
            assert!(!editor.has_active_inline_completion(cx));
            assert_eq!(
                editor.display_text(cx),
                "\n\na = 1\nb = 2\n\n\n\nc = 3\nd = 4\n"
            );
            assert_eq!(editor.text(cx), "a = 1\nb = 2\n\nc = 3\nd = 4\n");

            // Type a character, ensuring we don't even try to interpolate the previous suggestion.
            editor.handle_input(" ", cx);
            assert!(!editor.has_active_inline_completion(cx));
            assert_eq!(
                editor.display_text(cx),
                "\n\na = 1\nb = 2\n\n\n\nc = 3\nd = 4 \n"
            );
            assert_eq!(editor.text(cx), "a = 1\nb = 2\n\nc = 3\nd = 4 \n");
        });

        // Ensure the new suggestion is displayed when the debounce timeout expires.
        executor.advance_clock(COPILOT_DEBOUNCE_TIMEOUT);
        _ = editor.update(cx, |editor, cx| {
            assert!(editor.has_active_inline_completion(cx));
            assert_eq!(
                editor.display_text(cx),
                "\n\na = 1\nb = 2\n\n\n\nc = 3\nd = 4 + c\n"
            );
            assert_eq!(editor.text(cx), "a = 1\nb = 2\n\nc = 3\nd = 4 \n");
        });
    }

    #[gpui::test]
    async fn test_copilot_disabled_globs(executor: BackgroundExecutor, cx: &mut TestAppContext) {
        init_test(cx, |settings| {
            settings
                .inline_completions
                .get_or_insert(Default::default())
                .disabled_globs = Some(vec![".env*".to_string()]);
        });

        let (copilot, copilot_lsp) = Copilot::fake(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            "/test",
            json!({
                ".env": "SECRET=something\n",
                "README.md": "hello\n"
            }),
        )
        .await;
        let project = Project::test(fs, ["/test".as_ref()], cx).await;

        let private_buffer = project
            .update(cx, |project, cx| {
                project.open_local_buffer("/test/.env", cx)
            })
            .await
            .unwrap();
        let public_buffer = project
            .update(cx, |project, cx| {
                project.open_local_buffer("/test/README.md", cx)
            })
            .await
            .unwrap();

        let multibuffer = cx.new_model(|cx| {
            let mut multibuffer = MultiBuffer::new(0, language::Capability::ReadWrite);
            multibuffer.push_excerpts(
                private_buffer.clone(),
                [ExcerptRange {
                    context: Point::new(0, 0)..Point::new(1, 0),
                    primary: None,
                }],
                cx,
            );
            multibuffer.push_excerpts(
                public_buffer.clone(),
                [ExcerptRange {
                    context: Point::new(0, 0)..Point::new(1, 0),
                    primary: None,
                }],
                cx,
            );
            multibuffer
        });
        let editor = cx.add_window(|cx| Editor::for_multibuffer(multibuffer, None, cx));
        let copilot_provider = cx.new_model(|_| CopilotCompletionProvider::new(copilot));
        editor
            .update(cx, |editor, cx| {
                editor.set_inline_completion_provider(Some(copilot_provider), cx)
            })
            .unwrap();

        let mut copilot_requests = copilot_lsp
            .handle_request::<copilot::request::GetCompletions, _, _>(
                move |_params, _cx| async move {
                    Ok(copilot::request::GetCompletionsResult {
                        completions: vec![copilot::request::Completion {
                            text: "next line".into(),
                            range: lsp::Range::new(
                                lsp::Position::new(1, 0),
                                lsp::Position::new(1, 0),
                            ),
                            ..Default::default()
                        }],
                    })
                },
            );

        _ = editor.update(cx, |editor, cx| {
            editor.change_selections(None, cx, |selections| {
                selections.select_ranges([Point::new(0, 0)..Point::new(0, 0)])
            });
            editor.next_inline_completion(&Default::default(), cx);
        });

        executor.advance_clock(COPILOT_DEBOUNCE_TIMEOUT);
        assert!(copilot_requests.try_next().is_err());

        _ = editor.update(cx, |editor, cx| {
            editor.change_selections(None, cx, |s| {
                s.select_ranges([Point::new(2, 0)..Point::new(2, 0)])
            });
            editor.next_inline_completion(&Default::default(), cx);
        });

        executor.advance_clock(COPILOT_DEBOUNCE_TIMEOUT);
        assert!(copilot_requests.try_next().is_ok());
    }

    fn handle_copilot_completion_request(
        lsp: &lsp::FakeLanguageServer,
        completions: Vec<copilot::request::Completion>,
        completions_cycling: Vec<copilot::request::Completion>,
    ) {
        lsp.handle_request::<copilot::request::GetCompletions, _, _>(move |_params, _cx| {
            let completions = completions.clone();
            async move {
                Ok(copilot::request::GetCompletionsResult {
                    completions: completions.clone(),
                })
            }
        });
        lsp.handle_request::<copilot::request::GetCompletionsCycling, _, _>(move |_params, _cx| {
            let completions_cycling = completions_cycling.clone();
            async move {
                Ok(copilot::request::GetCompletionsResult {
                    completions: completions_cycling.clone(),
                })
            }
        });
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
            vec![complete_from_marker.clone(), replace_range_marker.clone()],
        );

        let complete_from_position =
            cx.to_lsp(marked_ranges.remove(&complete_from_marker).unwrap()[0].start);
        let replace_range =
            cx.to_lsp_range(marked_ranges.remove(&replace_range_marker).unwrap()[0].clone());

        let mut request =
            cx.handle_request::<lsp::request::Completion, _, _>(move |url, params, _| {
                let completions = completions.clone();
                async move {
                    assert_eq!(params.text_document_position.text_document.uri, url.clone());
                    assert_eq!(
                        params.text_document_position.position,
                        complete_from_position
                    );
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
        _ = cx.update(|cx| {
            let store = SettingsStore::test(cx);
            cx.set_global(store);
            theme::init(theme::LoadThemes::JustBase, cx);
            client::init_settings(cx);
            language::init(cx);
            editor::init_settings(cx);
            Project::init_settings(cx);
            workspace::init_settings(cx);
            cx.update_global(|store: &mut SettingsStore, cx| {
                store.update_user_settings::<AllLanguageSettings>(cx, f);
            });
        });
    }
}
