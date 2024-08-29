use crate::{state::Mode, Vim};
use editor::{scroll::Autoscroll, Bias, Editor};
use gpui::{actions, Action, ViewContext};
use language::SelectionGoal;

actions!(vim, [NormalBefore]);

pub fn register(editor: &mut Editor, cx: &mut ViewContext<Vim>) {
    Vim::action(editor, cx, Vim::normal_before);
}

impl Vim {
    fn normal_before(&mut self, action: &NormalBefore, cx: &mut ViewContext<Self>) {
        if self.active_operator().is_some() {
            self.operator_stack.clear();
            self.sync_vim_settings(cx);
            return;
        }
        let count = self.take_count(cx).unwrap_or(1);
        self.stop_recording_immediately(action.boxed_clone(), cx);
        if count <= 1 || Vim::globals(cx).dot_replaying {
            self.create_mark("^".into(), false, cx);
            self.update_editor(cx, |_, editor, cx| {
                editor.dismiss_menus_and_popups(false, cx);
                editor.change_selections(Some(Autoscroll::fit()), cx, |s| {
                    s.move_cursors_with(|map, mut cursor, _| {
                        *cursor.column_mut() = cursor.column().saturating_sub(1);
                        (map.clip_point(cursor, Bias::Left), SelectionGoal::None)
                    });
                });
            });
            self.switch_mode(Mode::Normal, false, cx);
            return;
        }

        self.repeat(true, cx)
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
