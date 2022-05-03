use editor::{EditorBlurred, EditorCreated, EditorFocused, EditorMode, EditorReleased};
use gpui::MutableAppContext;

use crate::{state::Mode, Vim};

pub fn init(cx: &mut MutableAppContext) {
    cx.subscribe_global(editor_created).detach();
    cx.subscribe_global(editor_focused).detach();
    cx.subscribe_global(editor_blurred).detach();
    cx.subscribe_global(editor_released).detach();
}

fn editor_created(EditorCreated(editor): &EditorCreated, cx: &mut MutableAppContext) {
    cx.update_default_global(|vim: &mut Vim, cx| {
        vim.editors.insert(editor.id(), editor.downgrade());
        vim.sync_editor_options(cx);
    })
}

fn editor_focused(EditorFocused(editor): &EditorFocused, cx: &mut MutableAppContext) {
    Vim::update(cx, |state, cx| {
        state.active_editor = Some(editor.downgrade());
        if editor.read(cx).mode() != EditorMode::Full {
            state.switch_mode(Mode::Insert, cx);
        }
    });
}

fn editor_blurred(EditorBlurred(editor): &EditorBlurred, cx: &mut MutableAppContext) {
    Vim::update(cx, |state, cx| {
        if let Some(previous_editor) = state.active_editor.clone() {
            if previous_editor == editor.clone() {
                state.active_editor = None;
            }
        }
        state.sync_editor_options(cx);
    })
}

fn editor_released(EditorReleased(editor): &EditorReleased, cx: &mut MutableAppContext) {
    cx.update_default_global(|vim: &mut Vim, _| {
        vim.editors.remove(&editor.id());
        if let Some(previous_editor) = vim.active_editor.clone() {
            if previous_editor == editor.clone() {
                vim.active_editor = None;
            }
        }
    });
}
