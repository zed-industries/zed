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
        if vim_state.enabled {
            VimState::update_cursor_shapes(cx);
        }
    })
}

fn editor_focused(EditorFocused(editor): &EditorFocused, cx: &mut MutableAppContext) {
    let mode = if matches!(editor.read(cx).mode(), EditorMode::SingleLine) {
        Mode::Insert
    } else {
        Mode::Normal
    };

    cx.update_default_global(|vim_state: &mut VimState, _| {
        vim_state.active_editor = Some(editor.downgrade());
    });
    VimState::switch_mode(&SwitchMode(mode), cx);
}

fn editor_blurred(EditorBlurred(editor): &EditorBlurred, cx: &mut MutableAppContext) {
    cx.update_default_global(|vim_state: &mut VimState, _| {
        if let Some(previous_editor) = vim_state.active_editor.clone() {
            if previous_editor == editor.clone() {
                vim_state.active_editor = None;
            }
        }
    });
    editor.update(cx, |editor, _| {
        editor.remove_keymap_context_layer::<VimState>();
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
