use crate::{
    normal::{mark::create_mark, repeat},
    state::Mode,
    Vim,
};
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
        if vim.state().active_operator().is_some() {
            vim.update_state(|state| state.operator_stack.clear());
            vim.sync_vim_settings(cx);
            return false;
        }
        let count = vim.take_count(cx).unwrap_or(1);
        vim.stop_recording_immediately(action.boxed_clone());
        if count <= 1 || vim.workspace_state.replaying {
            create_mark(vim, "^".into(), false, cx);
            vim.update_active_editor(cx, |_, editor, cx| {
                editor.dismiss_menus_and_popups(false, cx);
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
        cx.simulate_keystrokes("i");
        assert_eq!(cx.mode(), Mode::Insert);
        cx.simulate_keystrokes("T e s t");
        cx.assert_editor_state("Testˇ");
        cx.simulate_keystrokes("escape");
        assert_eq!(cx.mode(), Mode::Normal);
        cx.assert_editor_state("Tesˇt");
    }

    #[gpui::test]
    async fn test_insert_with_counts(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        cx.set_shared_state("ˇhello\n").await;
        cx.simulate_shared_keystrokes("5 i - escape").await;
        cx.shared_state().await.assert_eq("----ˇ-hello\n");

        cx.set_shared_state("ˇhello\n").await;
        cx.simulate_shared_keystrokes("5 a - escape").await;
        cx.shared_state().await.assert_eq("h----ˇ-ello\n");

        cx.simulate_shared_keystrokes("4 shift-i - escape").await;
        cx.shared_state().await.assert_eq("---ˇ-h-----ello\n");

        cx.simulate_shared_keystrokes("3 shift-a - escape").await;
        cx.shared_state().await.assert_eq("----h-----ello--ˇ-\n");

        cx.set_shared_state("ˇhello\n").await;
        cx.simulate_shared_keystrokes("3 o o i escape").await;
        cx.shared_state().await.assert_eq("hello\noi\noi\noˇi\n");

        cx.set_shared_state("ˇhello\n").await;
        cx.simulate_shared_keystrokes("3 shift-o o i escape").await;
        cx.shared_state().await.assert_eq("oi\noi\noˇi\nhello\n");
    }

    #[gpui::test]
    async fn test_insert_with_repeat(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        cx.set_shared_state("ˇhello\n").await;
        cx.simulate_shared_keystrokes("3 i - escape").await;
        cx.shared_state().await.assert_eq("--ˇ-hello\n");
        cx.simulate_shared_keystrokes(".").await;
        cx.shared_state().await.assert_eq("----ˇ--hello\n");
        cx.simulate_shared_keystrokes("2 .").await;
        cx.shared_state().await.assert_eq("-----ˇ---hello\n");

        cx.set_shared_state("ˇhello\n").await;
        cx.simulate_shared_keystrokes("2 o k k escape").await;
        cx.shared_state().await.assert_eq("hello\nkk\nkˇk\n");
        cx.simulate_shared_keystrokes(".").await;
        cx.shared_state()
            .await
            .assert_eq("hello\nkk\nkk\nkk\nkˇk\n");
        cx.simulate_shared_keystrokes("1 .").await;
        cx.shared_state()
            .await
            .assert_eq("hello\nkk\nkk\nkk\nkk\nkˇk\n");
    }

    #[gpui::test]
    async fn test_insert_ctrl_r(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        cx.set_shared_state("heˇllo\n").await;
        cx.simulate_shared_keystrokes("y y i ctrl-r \"").await;
        cx.shared_state().await.assert_eq("hehello\nˇllo\n");

        cx.simulate_shared_keystrokes("ctrl-r x ctrl-r escape")
            .await;
        cx.shared_state().await.assert_eq("hehello\nˇllo\n");
    }
}
