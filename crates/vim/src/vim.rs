mod editor_events;
mod editor_utils;
mod insert;
mod mode;
mod normal;
#[cfg(test)]
mod vim_tests;

use collections::HashMap;
use editor::Editor;
use editor_utils::VimEditorExt;
use gpui::{action, MutableAppContext, ViewContext, WeakViewHandle};

use mode::Mode;
use workspace::{self, Settings, Workspace};

action!(SwitchMode, Mode);

pub fn init(cx: &mut MutableAppContext) {
    editor_events::init(cx);
    insert::init(cx);
    normal::init(cx);

    cx.add_action(|_: &mut Workspace, action: &SwitchMode, cx| VimState::switch_mode(action, cx));

    cx.observe_global::<Settings, _>(VimState::settings_changed)
        .detach();
}

#[derive(Default)]
pub struct VimState {
    editors: HashMap<usize, WeakViewHandle<Editor>>,
    active_editor: Option<WeakViewHandle<Editor>>,

    enabled: bool,
    mode: Mode,
}

impl VimState {
    fn update_active_editor<S>(
        cx: &mut MutableAppContext,
        update: impl FnOnce(&mut Editor, &mut ViewContext<Editor>) -> S,
    ) -> Option<S> {
        cx.global::<Self>()
            .active_editor
            .clone()
            .and_then(|ae| ae.upgrade(cx))
            .map(|ae| ae.update(cx, update))
    }

    fn switch_mode(SwitchMode(mode): &SwitchMode, cx: &mut MutableAppContext) {
        let active_editor = cx.update_default_global(|this: &mut Self, _| {
            this.mode = *mode;
            this.active_editor.clone()
        });

        if let Some(active_editor) = active_editor.and_then(|e| e.upgrade(cx)) {
            active_editor.update(cx, |active_editor, cx| {
                active_editor.set_keymap_context_layer::<Self>(mode.keymap_context_layer());
                active_editor.set_input_enabled(*mode == Mode::Insert);
                if *mode != Mode::Insert {
                    active_editor.adjust_selections(cx);
                }
            });
        }
        VimState::update_cursor_shapes(cx);
    }

    fn settings_changed(cx: &mut MutableAppContext) {
        cx.update_default_global(|this: &mut Self, cx| {
            let settings = cx.global::<Settings>();
            if this.enabled != settings.vim_mode {
                this.enabled = settings.vim_mode;
                this.mode = if settings.vim_mode {
                    Mode::Normal
                } else {
                    Mode::Insert
                };
                Self::update_cursor_shapes(cx);
            }
        });
    }

    fn update_cursor_shapes(cx: &mut MutableAppContext) {
        cx.defer(move |cx| {
            cx.update_default_global(|this: &mut VimState, cx| {
                let cursor_shape = this.mode.cursor_shape();
                for editor in this.editors.values() {
                    if let Some(editor) = editor.upgrade(cx) {
                        editor.update(cx, |editor, cx| {
                            editor.set_cursor_shape(cursor_shape, cx);
                        });
                    }
                }
            });
        });
    }
}
