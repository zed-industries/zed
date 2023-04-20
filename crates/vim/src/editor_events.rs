use editor::{EditorBlurred, EditorFocused, EditorMode, EditorReleased, Event};
use gpui::{AppContext, WindowContext};

use crate::{state::Mode, Vim};

pub fn init(cx: &mut AppContext) {
    cx.subscribe_global(focused).detach();
    cx.subscribe_global(blurred).detach();
    cx.subscribe_global(released).detach();
}

fn focused(EditorFocused(editor): &EditorFocused, cx: &mut AppContext) {
    cx.update_window(editor.window_id(), |cx| {
        Vim::update(cx, |vim, cx| {
            vim.update_active_editor(cx, |previously_active_editor, cx| {
                Vim::unhook_vim_settings(previously_active_editor, cx);
            });

            vim.active_editor = Some(editor.downgrade());
            vim.editor_subscription = Some(cx.subscribe(editor, |editor, event, cx| match event {
                Event::SelectionsChanged { local: true } => {
                    let editor = editor.read(cx);
                    if editor.leader_replica_id().is_none() {
                        let newest_empty = editor.selections.newest::<usize>(cx).is_empty();
                        local_selections_changed(newest_empty, cx);
                    }
                }
                Event::InputIgnored { text } => {
                    Vim::active_editor_input_ignored(text.clone(), cx);
                }
                _ => {}
            }));

            if vim.enabled {
                let editor = editor.read(cx);
                let editor_mode = editor.mode();
                let newest_selection_empty = editor.selections.newest::<usize>(cx).is_empty();

                if editor_mode == EditorMode::Full && !newest_selection_empty {
                    vim.switch_mode(Mode::Visual { line: false }, true, cx);
                }
            }

            vim.sync_vim_settings(cx);
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

fn local_selections_changed(newest_empty: bool, cx: &mut WindowContext) {
    Vim::update(cx, |vim, cx| {
        if vim.enabled && vim.state.mode == Mode::Normal && !newest_empty {
            vim.switch_mode(Mode::Visual { line: false }, false, cx)
        }
    })
}
