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
    Vim::update(cx, |vim, cx| {
        vim.active_editor = Some(editor.downgrade());
        vim.selection_subscription = Some(cx.subscribe(editor, |editor, event, cx| {
            if let editor::Event::SelectionsChanged { local: true } = event {
                let newest_empty = !editor.read(cx).selections.newest::<usize>(cx).is_empty();
                editor_local_selections_changed(newest_empty, cx);
            }
        }));

        if editor.read(cx).mode() != EditorMode::Full {
            vim.switch_mode(Mode::Insert, cx);
        }
    });
}

fn editor_blurred(EditorBlurred(editor): &EditorBlurred, cx: &mut MutableAppContext) {
    Vim::update(cx, |vim, cx| {
        if let Some(previous_editor) = vim.active_editor.clone() {
            if previous_editor == editor.clone() {
                vim.active_editor = None;
            }
        }
        vim.sync_editor_options(cx);
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

fn editor_local_selections_changed(newest_empty: bool, cx: &mut MutableAppContext) {
    Vim::update(cx, |vim, cx| {
        if vim.state.mode == Mode::Normal && !newest_empty {
            vim.switch_mode(Mode::Visual, cx)
        }
    })
}
