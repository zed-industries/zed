use editor::Bias;
use gpui::{action, keymap::Binding, MutableAppContext, ViewContext};
use language::SelectionGoal;
use workspace::Workspace;

use crate::{editor_utils::VimEditorExt, mode::Mode, SwitchMode, VimState};

action!(NormalBefore);

pub fn init(cx: &mut MutableAppContext) {
    let context = Some("Editor && vim_mode == insert");
    cx.add_bindings(vec![
        Binding::new("escape", NormalBefore, context),
        Binding::new("ctrl-c", NormalBefore, context),
    ]);

    cx.add_action(normal_before);
}

fn normal_before(_: &mut Workspace, _: &NormalBefore, cx: &mut ViewContext<Workspace>) {
    VimState::switch_mode(&SwitchMode(Mode::Normal), cx);
    VimState::update_active_editor(cx, |editor, cx| {
        editor.clipped_move_cursors(cx, |map, mut cursor, _| {
            *cursor.column_mut() = cursor.column().saturating_sub(1);
            (map.clip_point(cursor, Bias::Left), SelectionGoal::None)
        });
    });
}
