use crate::Vim;
use editor::{EditorBlurred, EditorFocused, EditorReleased};
use gpui::AppContext;

pub fn init(cx: &mut AppContext) {
    cx.subscribe_global(focused).detach();
    cx.subscribe_global(blurred).detach();
    cx.subscribe_global(released).detach();
}

fn focused(EditorFocused(editor): &EditorFocused, cx: &mut AppContext) {
    if let Some(previously_active_editor) = Vim::read(cx).active_editor.clone() {
        cx.update_window(previously_active_editor.window_id(), |cx| {
            Vim::update(cx, |vim, cx| {
                vim.update_active_editor(cx, |previously_active_editor, cx| {
                    Vim::unhook_vim_settings(previously_active_editor, cx);
                });
            });
        });
    }

    cx.update_window(editor.window_id(), |cx| {
        Vim::update(cx, |vim, cx| {
            vim.set_active_editor(editor.clone(), cx);
        });
    });
}

fn blurred(EditorBlurred(editor): &EditorBlurred, cx: &mut AppContext) {
    cx.update_window(editor.window_id(), |cx| {
        Vim::update(cx, |vim, cx| {
            if let Some(previous_editor) = vim.active_editor.clone() {
                if previous_editor == editor.clone() {
                    vim.active_editor = None;
                }
            }

            cx.update_window(editor.window_id(), |cx| {
                editor.update(cx, |editor, cx| Vim::unhook_vim_settings(editor, cx))
            });
        });
    });
}

fn released(EditorReleased(editor): &EditorReleased, cx: &mut AppContext) {
    cx.update_window(editor.window_id(), |cx| {
        cx.update_default_global(|vim: &mut Vim, _| {
            if let Some(previous_editor) = vim.active_editor.clone() {
                if previous_editor == editor.clone() {
                    vim.active_editor = None;
                }
            }
        });
    });
}
