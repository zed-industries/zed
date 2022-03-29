use editor::{EditorBlurred, EditorCreated, EditorFocused, EditorMode, EditorReleased};
use gpui::MutableAppContext;

use crate::{mode::Mode, SwitchMode, VimState};

pub fn init(cx: &mut MutableAppContext) {
    cx.subscribe_global(editor_created).detach();
    cx.subscribe_global(editor_focused).detach();
    cx.subscribe_global(editor_blurred).detach();
    cx.subscribe_global(editor_released).detach();
}

fn editor_created(EditorCreated(editor): &EditorCreated, cx: &mut MutableAppContext) {
    cx.update_default_global(|vim_state: &mut VimState, cx| {
        vim_state.editors.insert(editor.id(), editor.downgrade());
        vim_state.sync_editor_options(cx);
    })
}

fn editor_focused(EditorFocused(editor): &EditorFocused, cx: &mut MutableAppContext) {
    let mode = if matches!(editor.read(cx).mode(), EditorMode::SingleLine) {
        Mode::Insert
    } else {
        Mode::Normal
    };

    VimState::update_global(cx, |state, cx| {
        state.active_editor = Some(editor.downgrade());
        state.switch_mode(&SwitchMode(mode), cx);
    });
}

fn editor_blurred(EditorBlurred(editor): &EditorBlurred, cx: &mut MutableAppContext) {
    VimState::update_global(cx, |state, cx| {
        if let Some(previous_editor) = state.active_editor.clone() {
            if previous_editor == editor.clone() {
                state.active_editor = None;
            }
        }
        state.sync_editor_options(cx);
    })
}

fn editor_released(EditorReleased(editor): &EditorReleased, cx: &mut MutableAppContext) {
    cx.update_default_global(|vim_state: &mut VimState, _| {
        vim_state.editors.remove(&editor.id());
        if let Some(previous_editor) = vim_state.active_editor.clone() {
            if previous_editor == editor.clone() {
                vim_state.active_editor = None;
            }
        }
    });
}
