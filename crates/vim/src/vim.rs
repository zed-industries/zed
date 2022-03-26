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
        cx.update_default_global(|this: &mut Self, _| {
            this.mode = *mode;
        });

        VimState::sync_editor_options(cx);
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
                Self::sync_editor_options(cx);
            }
        });
    }

    fn sync_editor_options(cx: &mut MutableAppContext) {
        cx.defer(move |cx| {
            cx.update_default_global(|this: &mut VimState, cx| {
                let mode = this.mode;
                let cursor_shape = mode.cursor_shape();
                let keymap_layer_active = this.enabled;
                for editor in this.editors.values() {
                    if let Some(editor) = editor.upgrade(cx) {
                        editor.update(cx, |editor, cx| {
                            editor.set_cursor_shape(cursor_shape, cx);
                            editor.set_clip_at_line_ends(cursor_shape == CursorShape::Block, cx);
                            editor.set_input_enabled(mode == Mode::Insert);
                            if keymap_layer_active {
                                let context_layer = mode.keymap_context_layer();
                                editor.set_keymap_context_layer::<Self>(context_layer);
                            } else {
                                editor.remove_keymap_context_layer::<Self>();
                            }
                        });
                    }
                }
            });
        });
    }
}
