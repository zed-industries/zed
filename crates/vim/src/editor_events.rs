use crate::{Vim, VimEvent};
use editor::{EditorBlurred, EditorFocused, EditorReleased};
use gpui::AppContext;

pub fn init(cx: &mut AppContext) {
    cx.subscribe_global(focused).detach();
    cx.subscribe_global(blurred).detach();
    cx.subscribe_global(released).detach();
}

fn focused(EditorFocused(editor): &EditorFocused, cx: &mut AppContext) {
    if let Some(previously_active_editor) = Vim::read(cx).active_editor.clone() {
        previously_active_editor.window().update(cx, |cx| {
            Vim::update(cx, |vim, cx| {
                vim.update_active_editor(cx, |previously_active_editor, cx| {
                    vim.unhook_vim_settings(previously_active_editor, cx)
                });
            });
        });
    }

    editor.window().update(cx, |cx| {
        Vim::update(cx, |vim, cx| {
            vim.set_active_editor(editor.clone(), cx);
            if vim.enabled {
                cx.emit_global(VimEvent::ModeChanged {
                    mode: vim.state().mode,
                });
            }
        });
    });
}

fn blurred(EditorBlurred(editor): &EditorBlurred, cx: &mut AppContext) {
    editor.window().update(cx, |cx| {
        Vim::update(cx, |vim, cx| {
            vim.clear_operator(cx);
            vim.workspace_state.recording = false;
            vim.workspace_state.recorded_actions.clear();
            if let Some(previous_editor) = vim.active_editor.clone() {
                if previous_editor == editor.clone() {
                    vim.active_editor = None;
                }
            }

            editor.update(cx, |editor, cx| vim.unhook_vim_settings(editor, cx))
        });
    });
}

fn released(EditorReleased(editor): &EditorReleased, cx: &mut AppContext) {
    editor.window().update(cx, |cx| {
        cx.update_default_global(|vim: &mut Vim, _| {
            if let Some(previous_editor) = vim.active_editor.clone() {
                if previous_editor == editor.clone() {
                    vim.active_editor = None;
                }
            }
            vim.editor_states.remove(&editor.id())
        });
    });
}
