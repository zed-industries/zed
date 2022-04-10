use crate::{mode::Mode, SwitchMode, VimState};
use editor::Bias;
use gpui::{actions, MutableAppContext, ViewContext};
use language::SelectionGoal;
use workspace::Workspace;

actions!(vim, [NormalBefore]);

pub fn init(cx: &mut MutableAppContext) {
    cx.add_action(normal_before);
}

fn normal_before(_: &mut Workspace, _: &NormalBefore, cx: &mut ViewContext<Workspace>) {
    VimState::update_global(cx, |state, cx| {
        state.update_active_editor(cx, |editor, cx| {
            editor.move_cursors(cx, |map, mut cursor, _| {
                *cursor.column_mut() = cursor.column().saturating_sub(1);
                (map.clip_point(cursor, Bias::Left), SelectionGoal::None)
            });
        });
        state.switch_mode(&SwitchMode(Mode::normal()), cx);
    })
}

#[cfg(test)]
mod test {
    use crate::{mode::Mode, vim_test_context::VimTestContext};

    #[gpui::test]
    async fn test_enter_and_exit_insert_mode(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true, "").await;
        cx.simulate_keystroke("i");
        assert_eq!(cx.mode(), Mode::Insert);
        cx.simulate_keystrokes(&["T", "e", "s", "t"]);
        cx.assert_editor_state("Test|");
        cx.simulate_keystroke("escape");
        assert_eq!(cx.mode(), Mode::normal());
        cx.assert_editor_state("Tes|t");
    }
}
