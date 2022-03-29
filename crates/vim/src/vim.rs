mod editor_events;
mod insert;
mod mode;
mod normal;
#[cfg(test)]
mod vim_tests;

use collections::HashMap;
use editor::{CursorShape, Editor};
use gpui::{action, MutableAppContext, ViewContext, WeakViewHandle};

use mode::Mode;
use workspace::{self, Settings, Workspace};

action!(SwitchMode, Mode);

pub fn init(cx: &mut MutableAppContext) {
    editor_events::init(cx);
    insert::init(cx);
    normal::init(cx);

    cx.add_action(|_: &mut Workspace, action: &SwitchMode, cx| {
        VimState::update_global(cx, |state, cx| state.switch_mode(action, cx))
    });

    cx.observe_global::<Settings, _>(|settings, cx| {
        VimState::update_global(cx, |state, cx| state.set_enabled(settings.vim_mode, cx))
    })
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
    fn update_global<F, S>(cx: &mut MutableAppContext, update: F) -> S
    where
        F: FnOnce(&mut Self, &mut MutableAppContext) -> S,
    {
        cx.update_default_global(update)
    }

    fn update_active_editor<S>(
        &self,
        cx: &mut MutableAppContext,
        update: impl FnOnce(&mut Editor, &mut ViewContext<Editor>) -> S,
    ) -> Option<S> {
        self.active_editor
            .clone()
            .and_then(|ae| ae.upgrade(cx))
            .map(|ae| ae.update(cx, update))
    }

    fn switch_mode(&mut self, SwitchMode(mode): &SwitchMode, cx: &mut MutableAppContext) {
        self.mode = *mode;
        self.sync_editor_options(cx);
    }

    fn set_enabled(&mut self, enabled: bool, cx: &mut MutableAppContext) {
        if self.enabled != enabled {
            self.enabled = enabled;
            if enabled {
                self.mode = Mode::Normal;
            }
            self.sync_editor_options(cx);
        }
    }

    fn sync_editor_options(&self, cx: &mut MutableAppContext) {
        let mode = self.mode;
        let cursor_shape = mode.cursor_shape();
        for editor in self.editors.values() {
            if let Some(editor) = editor.upgrade(cx) {
                editor.update(cx, |editor, cx| {
                    if self.enabled {
                        editor.set_cursor_shape(cursor_shape, cx);
                        editor.set_clip_at_line_ends(cursor_shape == CursorShape::Block, cx);
                        editor.set_input_enabled(mode == Mode::Insert);
                        let context_layer = mode.keymap_context_layer();
                        editor.set_keymap_context_layer::<Self>(context_layer);
                    } else {
                        editor.set_cursor_shape(CursorShape::Bar, cx);
                        editor.set_clip_at_line_ends(false, cx);
                        editor.set_input_enabled(true);
                        editor.remove_keymap_context_layer::<Self>();
                    }
                });
            }
        }
    }
}
