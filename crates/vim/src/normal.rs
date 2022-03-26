use editor::{movement, Bias};
use gpui::{action, keymap::Binding, MutableAppContext, ViewContext};
use language::SelectionGoal;
use workspace::Workspace;

use crate::{Mode, SwitchMode, VimState};

action!(InsertBefore);
action!(MoveLeft);
action!(MoveDown);
action!(MoveUp);
action!(MoveRight);

pub fn init(cx: &mut MutableAppContext) {
    let context = Some("Editor && vim_mode == normal");
    cx.add_bindings(vec![
        Binding::new("i", SwitchMode(Mode::Insert), context),
        Binding::new("h", MoveLeft, context),
        Binding::new("j", MoveDown, context),
        Binding::new("k", MoveUp, context),
        Binding::new("l", MoveRight, context),
    ]);

    cx.add_action(move_left);
    cx.add_action(move_down);
    cx.add_action(move_up);
    cx.add_action(move_right);
}

fn move_left(_: &mut Workspace, _: &MoveLeft, cx: &mut ViewContext<Workspace>) {
    VimState::update_active_editor(cx, |editor, cx| {
        editor.move_cursors(cx, |map, mut cursor, _| {
            *cursor.column_mut() = cursor.column().saturating_sub(1);
            (map.clip_point(cursor, Bias::Left), SelectionGoal::None)
        });
    });
}

fn move_down(_: &mut Workspace, _: &MoveDown, cx: &mut ViewContext<Workspace>) {
    VimState::update_active_editor(cx, |editor, cx| {
        editor.move_cursors(cx, movement::down);
    });
}

fn move_up(_: &mut Workspace, _: &MoveUp, cx: &mut ViewContext<Workspace>) {
    VimState::update_active_editor(cx, |editor, cx| {
        editor.move_cursors(cx, movement::up);
    });
}

fn move_right(_: &mut Workspace, _: &MoveRight, cx: &mut ViewContext<Workspace>) {
    VimState::update_active_editor(cx, |editor, cx| {
        editor.move_cursors(cx, |map, mut cursor, _| {
            *cursor.column_mut() += 1;
            (map.clip_point(cursor, Bias::Right), SelectionGoal::None)
        });
    });
}
