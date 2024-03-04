use crate::{
    insert::NormalBefore,
    motion::Motion,
    state::{Mode, RecordedSelection, ReplayableAction},
    visual::visual_motion,
    Vim,
};
use gpui::{actions, Action, ViewContext, WindowContext};
use workspace::Workspace;

actions!(vim, [Repeat, EndRepeat]);

fn should_replay(action: &Box<dyn Action>) -> bool {
    // skip so that we don't leave the character palette open
    if editor::actions::ShowCharacterPalette.partial_eq(&**action) {
        return false;
    }
    true
}

fn repeatable_insert(action: &ReplayableAction) -> Option<Box<dyn Action>> {
    match action {
        ReplayableAction::Action(action) => {
            if super::InsertBefore.partial_eq(&**action)
                || super::InsertAfter.partial_eq(&**action)
                || super::InsertFirstNonWhitespace.partial_eq(&**action)
                || super::InsertEndOfLine.partial_eq(&**action)
            {
                Some(super::InsertBefore.boxed_clone())
            } else if super::InsertLineAbove.partial_eq(&**action)
                || super::InsertLineBelow.partial_eq(&**action)
            {
                Some(super::InsertLineBelow.boxed_clone())
            } else {
                None
            }
        }
        ReplayableAction::Insertion { .. } => None,
    }
}

pub(crate) fn register(workspace: &mut Workspace, _: &mut ViewContext<Workspace>) {
    workspace.register_action(|_: &mut Workspace, _: &EndRepeat, cx| {
        Vim::update(cx, |vim, cx| {
            vim.workspace_state.replaying = false;
            vim.switch_mode(Mode::Normal, false, cx)
        });
    });

    workspace.register_action(|_: &mut Workspace, _: &Repeat, cx| repeat(cx, false));
}

pub(crate) fn repeat(cx: &mut WindowContext, from_insert_mode: bool) {
    let Some((mut actions, editor, selection)) = Vim::update(cx, |vim, cx| {
        let actions = vim.workspace_state.recorded_actions.clone();
        if actions.is_empty() {
            return None;
        }

        let Some(editor) = vim.active_editor.clone() else {
            return None;
        };
        let count = vim.take_count(cx);

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

    // insert internally uses repeat to handle counts
    // vim doesn't treat 3a1 as though you literally repeated a1
    // 3 times, instead it inserts the content thrice at the insert position.
    if let Some(to_repeat) = repeatable_insert(&actions[0]) {
        if let Some(ReplayableAction::Action(action)) = actions.last() {
            if NormalBefore.partial_eq(&**action) {
                actions.pop();
            }
        }

        let mut new_actions = actions.clone();
        actions[0] = ReplayableAction::Action(to_repeat.boxed_clone());

        let mut count = Vim::read(cx).workspace_state.recorded_count.unwrap_or(1);

        // if we came from insert mode we're just doing repetitions 2 onwards.
        if from_insert_mode {
            count -= 1;
            new_actions[0] = actions[0].clone();
        }

        for _ in 1..count {
            new_actions.append(actions.clone().as_mut());
        }
        new_actions.push(ReplayableAction::Action(NormalBefore.boxed_clone()));
        actions = new_actions;
    }

    Vim::update(cx, |vim, _| vim.workspace_state.replaying = true);
    let window = cx.window_handle();
    cx.spawn(move |mut cx| async move {
        editor.update(&mut cx, |editor, _| {
            editor.show_local_selections = false;
        })?;
        for action in actions {
            match action {
                ReplayableAction::Action(action) => {
                    if should_replay(&action) {
                        window.update(&mut cx, |_, cx| cx.dispatch_action(action))
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
        editor.update(&mut cx, |editor, _| {
            editor.show_local_selections = true;
        })?;
        window.update(&mut cx, |_, cx| cx.dispatch_action(EndRepeat.boxed_clone()))
    })
    .detach_and_log_err(cx);
}

#[cfg(test)]
mod test {
    use editor::test::editor_lsp_test_context::EditorLspTestContext;
    use futures::StreamExt;
    use indoc::indoc;

    use gpui::ViewInputHandler;

    use crate::{
        state::Mode,
        test::{NeovimBackedTestContext, VimTestContext},
    };

    #[gpui::test]
    async fn test_dot_repeat(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        // "o"
        cx.set_shared_state("ˇhello").await;
        cx.simulate_shared_keystrokes(["o", "w", "o", "r", "l", "d", "escape"])
            .await;
        cx.assert_shared_state("hello\nworlˇd").await;
        cx.simulate_shared_keystrokes(["."]).await;
        cx.assert_shared_state("hello\nworld\nworlˇd").await;

        // "d"
        cx.simulate_shared_keystrokes(["^", "d", "f", "o"]).await;
        cx.simulate_shared_keystrokes(["g", "g", "."]).await;
        cx.assert_shared_state("ˇ\nworld\nrld").await;

        // "p" (note that it pastes the current clipboard)
        cx.simulate_shared_keystrokes(["j", "y", "y", "p"]).await;
        cx.simulate_shared_keystrokes(["shift-g", "y", "y", "."])
            .await;
        cx.assert_shared_state("\nworld\nworld\nrld\nˇrld").await;

        // "~" (note that counts apply to the action taken, not . itself)
        cx.set_shared_state("ˇthe quick brown fox").await;
        cx.simulate_shared_keystrokes(["2", "~", "."]).await;
        cx.set_shared_state("THE ˇquick brown fox").await;
        cx.simulate_shared_keystrokes(["3", "."]).await;
        cx.set_shared_state("THE QUIˇck brown fox").await;
        cx.run_until_parked();
        cx.simulate_shared_keystrokes(["."]).await;
        cx.assert_shared_state("THE QUICK ˇbrown fox").await;
    }

    #[gpui::test]
    async fn test_repeat_ime(cx: &mut gpui::TestAppContext) {
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
        cx.assert_state("hˇäällo", Mode::Normal);
    }

    #[gpui::test]
    async fn test_repeat_completion(cx: &mut gpui::TestAppContext) {
        VimTestContext::init(cx);
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
                            range: lsp::Range::new(position, position),
                            new_text: "first".to_string(),
                        })),
                        ..Default::default()
                    },
                    lsp::CompletionItem {
                        label: "second".to_string(),
                        text_edit: Some(lsp::CompletionTextEdit::Edit(lsp::TextEdit {
                            range: lsp::Range::new(position, position),
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
    async fn test_repeat_visual(cx: &mut gpui::TestAppContext) {
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
        cx.assert_shared_state(indoc! {
            "o quick brown
            fox ˇops over
            the lazy dog"
        })
        .await;
        cx.simulate_shared_keystrokes(["f", "r", "."]).await;
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
        cx.assert_shared_state(indoc! {
            "the ˇumps over
            fox jumps over
            the lazy dog"
        })
        .await;
        cx.simulate_shared_keystrokes(["w", "."]).await;
        cx.assert_shared_state(indoc! {
            "the umps ˇumps over
            the lazy dog"
        })
        .await;
        cx.simulate_shared_keystrokes(["j", "."]).await;
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
        cx.assert_shared_state(indoc! {
            "o
            ˇo
            the lazy dog"
        })
        .await;
    }

    #[gpui::test]
    async fn test_repeat_motion_counts(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        cx.set_shared_state(indoc! {
            "ˇthe quick brown
            fox jumps over
            the lazy dog"
        })
        .await;
        cx.simulate_shared_keystrokes(["3", "d", "3", "l"]).await;
        cx.assert_shared_state(indoc! {
            "ˇ brown
            fox jumps over
            the lazy dog"
        })
        .await;
        cx.simulate_shared_keystrokes(["j", "."]).await;
        cx.assert_shared_state(indoc! {
            " brown
            ˇ over
            the lazy dog"
        })
        .await;
        cx.simulate_shared_keystrokes(["j", "2", "."]).await;
        cx.assert_shared_state(indoc! {
            " brown
             over
            ˇe lazy dog"
        })
        .await;
    }

    #[gpui::test]
    async fn test_record_interrupted(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;

        cx.set_state("ˇhello\n", Mode::Normal);
        cx.simulate_keystrokes(["4", "i", "j", "cmd-shift-p", "escape"]);
        cx.simulate_keystrokes(["escape"]);
        cx.assert_state("ˇjhello\n", Mode::Normal);
    }

    #[gpui::test]
    async fn test_repeat_over_blur(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        cx.set_shared_state("ˇhello hello hello\n").await;
        cx.simulate_shared_keystrokes(["c", "f", "o", "x", "escape"])
            .await;
        cx.assert_shared_state("ˇx hello hello\n").await;
        cx.simulate_shared_keystrokes([":", "escape"]).await;
        cx.simulate_shared_keystrokes(["."]).await;
        cx.assert_shared_state("ˇx hello\n").await;
    }
}
