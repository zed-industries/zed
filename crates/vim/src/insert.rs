use crate::{state::Mode, Vim};
use editor::{scroll::autoscroll::Autoscroll, Bias};
use gpui::{actions, AppContext, ViewContext};
use language::SelectionGoal;
use workspace::Workspace;

actions!(vim, [NormalBefore]);

pub fn init(cx: &mut AppContext) {
    cx.add_action(normal_before);
}

fn normal_before(_: &mut Workspace, _: &NormalBefore, cx: &mut ViewContext<Workspace>) {
    Vim::update(cx, |vim, cx| {
        vim.stop_recording();
        vim.update_active_editor(cx, |editor, cx| {
            editor.change_selections(Some(Autoscroll::fit()), cx, |s| {
                s.move_cursors_with(|map, mut cursor, _| {
                    *cursor.column_mut() = cursor.column().saturating_sub(1);
                    (map.clip_point(cursor, Bias::Left), SelectionGoal::None)
                });
            });
        });
        vim.switch_mode(Mode::Normal, false, cx);
    })
}

#[cfg(test)]
mod test {
    use crate::{state::Mode, test::VimTestContext};

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
}
