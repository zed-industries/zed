use crate::{normal::repeat, state::Mode, Vim};
use editor::{scroll::Autoscroll, Bias};
use gpui::{actions, Action, ViewContext};
use language::SelectionGoal;
use workspace::Workspace;

actions!(vim, [NormalBefore]);

pub fn register(workspace: &mut Workspace, _: &mut ViewContext<Workspace>) {
    workspace.register_action(normal_before);
}

fn normal_before(_: &mut Workspace, action: &NormalBefore, cx: &mut ViewContext<Workspace>) {
    let should_repeat = Vim::update(cx, |vim, cx| {
        let count = vim.take_count(cx).unwrap_or(1);
        vim.stop_recording_immediately(action.boxed_clone());
        if count <= 1 || vim.workspace_state.replaying {
            vim.update_active_editor(cx, |_, editor, cx| {
                editor.dismiss_menus_and_popups(cx);
                editor.change_selections(Some(Autoscroll::fit()), cx, |s| {
                    s.move_cursors_with(|map, mut cursor, _| {
                        *cursor.column_mut() = cursor.column().saturating_sub(1);
                        (map.clip_point(cursor, Bias::Left), SelectionGoal::None)
                    });
                });
            });
            vim.switch_mode(Mode::Normal, false, cx);
            false
        } else {
            true
        }
    });

    if should_repeat {
        repeat::repeat(cx, true)
    }
}

#[cfg(test)]
mod test {
    use crate::{
        state::Mode,
        test::{NeovimBackedTestContext, VimTestContext},
    };

    #[gpui::test]
    async fn test_enter_and_exit_insert_mode(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        cx.simulate_keystroke("i");
        assert_eq!(cx.mode(), Mode::Insert);
        cx.simulate_keystrokes(["T", "e", "s", "t"]);
        cx.assert_editor_state("Testˇ");
        cx.simulate_keystroke("escape");
        assert_eq!(cx.mode(), Mode::Normal);
        cx.assert_editor_state("Tesˇt");
    }

    #[gpui::test]
    async fn test_insert_with_counts(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        cx.set_shared_state("ˇhello\n").await;
        cx.simulate_shared_keystrokes(["5", "i", "-", "escape"])
            .await;
        cx.run_until_parked();
        cx.assert_shared_state("----ˇ-hello\n").await;

        cx.set_shared_state("ˇhello\n").await;
        cx.simulate_shared_keystrokes(["5", "a", "-", "escape"])
            .await;
        cx.run_until_parked();
        cx.assert_shared_state("h----ˇ-ello\n").await;

        cx.simulate_shared_keystrokes(["4", "shift-i", "-", "escape"])
            .await;
        cx.run_until_parked();
        cx.assert_shared_state("---ˇ-h-----ello\n").await;

        cx.simulate_shared_keystrokes(["3", "shift-a", "-", "escape"])
            .await;
        cx.run_until_parked();
        cx.assert_shared_state("----h-----ello--ˇ-\n").await;

        cx.set_shared_state("ˇhello\n").await;
        cx.simulate_shared_keystrokes(["3", "o", "o", "i", "escape"])
            .await;
        cx.run_until_parked();
        cx.assert_shared_state("hello\noi\noi\noˇi\n").await;

        cx.set_shared_state("ˇhello\n").await;
        cx.simulate_shared_keystrokes(["3", "shift-o", "o", "i", "escape"])
            .await;
        cx.run_until_parked();
        cx.assert_shared_state("oi\noi\noˇi\nhello\n").await;
    }

    #[gpui::test]
    async fn test_insert_with_repeat(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        cx.set_shared_state("ˇhello\n").await;
        cx.simulate_shared_keystrokes(["3", "i", "-", "escape"])
            .await;
        cx.run_until_parked();
        cx.assert_shared_state("--ˇ-hello\n").await;
        cx.simulate_shared_keystrokes(["."]).await;
        cx.run_until_parked();
        cx.assert_shared_state("----ˇ--hello\n").await;
        cx.simulate_shared_keystrokes(["2", "."]).await;
        cx.run_until_parked();
        cx.assert_shared_state("-----ˇ---hello\n").await;

        cx.set_shared_state("ˇhello\n").await;
        cx.simulate_shared_keystrokes(["2", "o", "k", "k", "escape"])
            .await;
        cx.run_until_parked();
        cx.assert_shared_state("hello\nkk\nkˇk\n").await;
        cx.simulate_shared_keystrokes(["."]).await;
        cx.run_until_parked();
        cx.assert_shared_state("hello\nkk\nkk\nkk\nkˇk\n").await;
        cx.simulate_shared_keystrokes(["1", "."]).await;
        cx.run_until_parked();
        cx.assert_shared_state("hello\nkk\nkk\nkk\nkk\nkˇk\n").await;
    }
}
