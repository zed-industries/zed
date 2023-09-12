use crate::{
    motion::Motion,
    state::{Mode, RecordedSelection, ReplayableAction},
    visual::visual_motion,
    Vim,
};
use gpui::{actions, Action, AppContext};
use workspace::Workspace;

actions!(vim, [Repeat, EndRepeat,]);

fn should_replay(action: &Box<dyn Action>) -> bool {
    // skip so that we don't leave the character palette open
    if editor::ShowCharacterPalette.id() == action.id() {
        return false;
    }
    true
}

pub(crate) fn init(cx: &mut AppContext) {
    cx.add_action(|_: &mut Workspace, _: &EndRepeat, cx| {
        Vim::update(cx, |vim, cx| {
            vim.workspace_state.replaying = false;
            vim.update_active_editor(cx, |editor, _| {
                editor.show_local_selections = true;
            });
            vim.switch_mode(Mode::Normal, false, cx)
        });
    });

    cx.add_action(|_: &mut Workspace, _: &Repeat, cx| {
        let Some((actions, editor, selection)) = Vim::update(cx, |vim, cx| {
            let actions = vim.workspace_state.recorded_actions.clone();
            let Some(editor) = vim.active_editor.clone() else {
                return None;
            };
            let count = vim.pop_number_operator(cx);

            vim.workspace_state.replaying = true;

            let selection = vim.workspace_state.recorded_selection.clone();
            match selection {
                RecordedSelection::SingleLine { .. } | RecordedSelection::Visual { .. } => {
                    vim.workspace_state.recorded_count = None;
                    vim.switch_mode(Mode::Visual, false, cx)
                }
                RecordedSelection::VisualLine { .. } => {
                    vim.workspace_state.recorded_count = None;
                    vim.switch_mode(Mode::VisualLine, false, cx)
                }
                RecordedSelection::VisualBlock { .. } => {
                    vim.workspace_state.recorded_count = None;
                    vim.switch_mode(Mode::VisualBlock, false, cx)
                }
                RecordedSelection::None => {
                    if let Some(count) = count {
                        vim.workspace_state.recorded_count = Some(count);
                    }
                }
            }

            if let Some(editor) = editor.upgrade(cx) {
                editor.update(cx, |editor, _| {
                    editor.show_local_selections = false;
                })
            } else {
                return None;
            }

            Some((actions, editor, selection))
        }) else {
            return;
        };

        match selection {
            RecordedSelection::SingleLine { cols } => {
                if cols > 1 {
                    visual_motion(Motion::Right, Some(cols as usize - 1), cx)
                }
            }
            RecordedSelection::Visual { rows, cols } => {
                visual_motion(
                    Motion::Down {
                        display_lines: false,
                    },
                    Some(rows as usize),
                    cx,
                );
                visual_motion(
                    Motion::StartOfLine {
                        display_lines: false,
                    },
                    None,
                    cx,
                );
                if cols > 1 {
                    visual_motion(Motion::Right, Some(cols as usize - 1), cx)
                }
            }
            RecordedSelection::VisualBlock { rows, cols } => {
                visual_motion(
                    Motion::Down {
                        display_lines: false,
                    },
                    Some(rows as usize),
                    cx,
                );
                if cols > 1 {
                    visual_motion(Motion::Right, Some(cols as usize - 1), cx);
                }
            }
            RecordedSelection::VisualLine { rows } => {
                visual_motion(
                    Motion::Down {
                        display_lines: false,
                    },
                    Some(rows as usize),
                    cx,
                );
            }
            RecordedSelection::None => {}
        }

        let window = cx.window();
        cx.app_context()
            .spawn(move |mut cx| async move {
                for action in actions {
                    match action {
                        ReplayableAction::Action(action) => {
                            if should_replay(&action) {
                                window
                                    .dispatch_action(editor.id(), action.as_ref(), &mut cx)
                                    .ok_or_else(|| anyhow::anyhow!("window was closed"))
                            } else {
                                Ok(())
                            }
                        }
                        ReplayableAction::Insertion {
                            text,
                            utf16_range_to_replace,
                        } => editor.update(&mut cx, |editor, cx| {
                            editor.replay_insert_event(&text, utf16_range_to_replace.clone(), cx)
                        }),
                    }?
                }
                window
                    .dispatch_action(editor.id(), &EndRepeat, &mut cx)
                    .ok_or_else(|| anyhow::anyhow!("window was closed"))
            })
            .detach_and_log_err(cx);
    });
}

#[cfg(test)]
mod test {
    use std::sync::Arc;

    use editor::test::editor_lsp_test_context::EditorLspTestContext;
    use futures::StreamExt;
    use indoc::indoc;

    use gpui::{executor::Deterministic, View};

    use crate::{
        state::Mode,
        test::{NeovimBackedTestContext, VimTestContext},
    };

    #[gpui::test]
    async fn test_dot_repeat(deterministic: Arc<Deterministic>, cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        // "o"
        cx.set_shared_state("ˇhello").await;
        cx.simulate_shared_keystrokes(["o", "w", "o", "r", "l", "d", "escape"])
            .await;
        cx.assert_shared_state("hello\nworlˇd").await;
        cx.simulate_shared_keystrokes(["."]).await;
        deterministic.run_until_parked();
        cx.assert_shared_state("hello\nworld\nworlˇd").await;

        // "d"
        cx.simulate_shared_keystrokes(["^", "d", "f", "o"]).await;
        cx.simulate_shared_keystrokes(["g", "g", "."]).await;
        deterministic.run_until_parked();
        cx.assert_shared_state("ˇ\nworld\nrld").await;

        // "p" (note that it pastes the current clipboard)
        cx.simulate_shared_keystrokes(["j", "y", "y", "p"]).await;
        cx.simulate_shared_keystrokes(["shift-g", "y", "y", "."])
            .await;
        deterministic.run_until_parked();
        cx.assert_shared_state("\nworld\nworld\nrld\nˇrld").await;

        // "~" (note that counts apply to the action taken, not . itself)
        cx.set_shared_state("ˇthe quick brown fox").await;
        cx.simulate_shared_keystrokes(["2", "~", "."]).await;
        deterministic.run_until_parked();
        cx.set_shared_state("THE ˇquick brown fox").await;
        cx.simulate_shared_keystrokes(["3", "."]).await;
        deterministic.run_until_parked();
        cx.set_shared_state("THE QUIˇck brown fox").await;
        deterministic.run_until_parked();
        cx.simulate_shared_keystrokes(["."]).await;
        deterministic.run_until_parked();
        cx.set_shared_state("THE QUICK ˇbrown fox").await;
    }

    #[gpui::test]
    async fn test_repeat_ime(deterministic: Arc<Deterministic>, cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;

        cx.set_state("hˇllo", Mode::Normal);
        cx.simulate_keystrokes(["i"]);

        // simulate brazilian input for ä.
        cx.update_editor(|editor, cx| {
            editor.replace_and_mark_text_in_range(None, "\"", Some(1..1), cx);
            editor.replace_text_in_range(None, "ä", cx);
        });
        cx.simulate_keystrokes(["escape"]);
        cx.assert_state("hˇällo", Mode::Normal);
        cx.simulate_keystrokes(["."]);
        deterministic.run_until_parked();
        cx.assert_state("hˇäällo", Mode::Normal);
    }

    #[gpui::test]
    async fn test_repeat_completion(
        deterministic: Arc<Deterministic>,
        cx: &mut gpui::TestAppContext,
    ) {
        let cx = EditorLspTestContext::new_rust(
            lsp::ServerCapabilities {
                completion_provider: Some(lsp::CompletionOptions {
                    trigger_characters: Some(vec![".".to_string(), ":".to_string()]),
                    resolve_provider: Some(true),
                    ..Default::default()
                }),
                ..Default::default()
            },
            cx,
        )
        .await;
        let mut cx = VimTestContext::new_with_lsp(cx, true);

        cx.set_state(
            indoc! {"
            onˇe
            two
            three
        "},
            Mode::Normal,
        );

        let mut request =
            cx.handle_request::<lsp::request::Completion, _, _>(move |_, params, _| async move {
                let position = params.text_document_position.position;
                Ok(Some(lsp::CompletionResponse::Array(vec![
                    lsp::CompletionItem {
                        label: "first".to_string(),
                        text_edit: Some(lsp::CompletionTextEdit::Edit(lsp::TextEdit {
                            range: lsp::Range::new(position.clone(), position.clone()),
                            new_text: "first".to_string(),
                        })),
                        ..Default::default()
                    },
                    lsp::CompletionItem {
                        label: "second".to_string(),
                        text_edit: Some(lsp::CompletionTextEdit::Edit(lsp::TextEdit {
                            range: lsp::Range::new(position.clone(), position.clone()),
                            new_text: "second".to_string(),
                        })),
                        ..Default::default()
                    },
                ])))
            });
        cx.simulate_keystrokes(["a", "."]);
        request.next().await;
        cx.condition(|editor, _| editor.context_menu_visible())
            .await;
        cx.simulate_keystrokes(["down", "enter", "!", "escape"]);

        cx.assert_state(
            indoc! {"
                one.secondˇ!
                two
                three
            "},
            Mode::Normal,
        );
        cx.simulate_keystrokes(["j", "."]);
        deterministic.run_until_parked();
        cx.assert_state(
            indoc! {"
                one.second!
                two.secondˇ!
                three
            "},
            Mode::Normal,
        );
    }

    #[gpui::test]
    async fn test_repeat_visual(deterministic: Arc<Deterministic>, cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        // single-line (3 columns)
        cx.set_shared_state(indoc! {
            "ˇthe quick brown
            fox jumps over
            the lazy dog"
        })
        .await;
        cx.simulate_shared_keystrokes(["v", "i", "w", "s", "o", "escape"])
            .await;
        cx.assert_shared_state(indoc! {
            "ˇo quick brown
            fox jumps over
            the lazy dog"
        })
        .await;
        cx.simulate_shared_keystrokes(["j", "w", "."]).await;
        deterministic.run_until_parked();
        cx.assert_shared_state(indoc! {
            "o quick brown
            fox ˇops over
            the lazy dog"
        })
        .await;
        cx.simulate_shared_keystrokes(["f", "r", "."]).await;
        deterministic.run_until_parked();
        cx.assert_shared_state(indoc! {
            "o quick brown
            fox ops oveˇothe lazy dog"
        })
        .await;

        // visual
        cx.set_shared_state(indoc! {
            "the ˇquick brown
            fox jumps over
            fox jumps over
            fox jumps over
            the lazy dog"
        })
        .await;
        cx.simulate_shared_keystrokes(["v", "j", "x"]).await;
        cx.assert_shared_state(indoc! {
            "the ˇumps over
            fox jumps over
            fox jumps over
            the lazy dog"
        })
        .await;
        cx.simulate_shared_keystrokes(["."]).await;
        deterministic.run_until_parked();
        cx.assert_shared_state(indoc! {
            "the ˇumps over
            fox jumps over
            the lazy dog"
        })
        .await;
        cx.simulate_shared_keystrokes(["w", "."]).await;
        deterministic.run_until_parked();
        cx.assert_shared_state(indoc! {
            "the umps ˇumps over
            the lazy dog"
        })
        .await;
        cx.simulate_shared_keystrokes(["j", "."]).await;
        deterministic.run_until_parked();
        cx.assert_shared_state(indoc! {
            "the umps umps over
            the ˇog"
        })
        .await;

        // block mode (3 rows)
        cx.set_shared_state(indoc! {
            "ˇthe quick brown
            fox jumps over
            the lazy dog"
        })
        .await;
        cx.simulate_shared_keystrokes(["ctrl-v", "j", "j", "shift-i", "o", "escape"])
            .await;
        cx.assert_shared_state(indoc! {
            "ˇothe quick brown
            ofox jumps over
            othe lazy dog"
        })
        .await;
        cx.simulate_shared_keystrokes(["j", "4", "l", "."]).await;
        deterministic.run_until_parked();
        cx.assert_shared_state(indoc! {
            "othe quick brown
            ofoxˇo jumps over
            otheo lazy dog"
        })
        .await;

        // line mode
        cx.set_shared_state(indoc! {
            "ˇthe quick brown
            fox jumps over
            the lazy dog"
        })
        .await;
        cx.simulate_shared_keystrokes(["shift-v", "shift-r", "o", "escape"])
            .await;
        cx.assert_shared_state(indoc! {
            "ˇo
            fox jumps over
            the lazy dog"
        })
        .await;
        cx.simulate_shared_keystrokes(["j", "."]).await;
        deterministic.run_until_parked();
        cx.assert_shared_state(indoc! {
            "o
            ˇo
            the lazy dog"
        })
        .await;
    }
}
