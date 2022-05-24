#[cfg(test)]
mod vim_test_context;

mod editor_events;
mod insert;
mod motion;
mod normal;
mod state;
mod utils;
mod visual;

use collections::HashMap;
use editor::{Bias, CursorShape, Editor, Input};
use gpui::{impl_actions, MutableAppContext, Subscription, ViewContext, WeakViewHandle};
use serde::Deserialize;

use settings::Settings;
use state::{Mode, Operator, VimState};
use workspace::{self, Workspace};

#[derive(Clone, Deserialize)]
pub struct SwitchMode(pub Mode);

#[derive(Clone, Deserialize)]
pub struct PushOperator(pub Operator);

impl_actions!(vim, [SwitchMode, PushOperator]);

pub fn init(cx: &mut MutableAppContext) {
    editor_events::init(cx);
    normal::init(cx);
    visual::init(cx);
    insert::init(cx);
    motion::init(cx);

    cx.add_action(|_: &mut Workspace, &SwitchMode(mode): &SwitchMode, cx| {
        Vim::update(cx, |vim, cx| vim.switch_mode(mode, cx))
    });
    cx.add_action(
        |_: &mut Workspace, &PushOperator(operator): &PushOperator, cx| {
            Vim::update(cx, |vim, cx| vim.push_operator(operator, cx))
        },
    );
    cx.add_action(|_: &mut Editor, _: &Input, cx| {
        if Vim::read(cx).active_operator().is_some() {
            // Defer without updating editor
            MutableAppContext::defer(cx, |cx| Vim::update(cx, |vim, cx| vim.clear_operator(cx)))
        } else {
            cx.propagate_action()
        }
    });

    cx.observe_global::<Settings, _>(|cx| {
        Vim::update(cx, |state, cx| {
            state.set_enabled(cx.global::<Settings>().vim_mode, cx)
        })
    })
    .detach();
}

#[derive(Default)]
pub struct Vim {
    editors: HashMap<usize, WeakViewHandle<Editor>>,
    active_editor: Option<WeakViewHandle<Editor>>,
    selection_subscription: Option<Subscription>,

    enabled: bool,
    state: VimState,
}

impl Vim {
    fn read(cx: &mut MutableAppContext) -> &Self {
        cx.default_global()
    }

    fn update<F, S>(cx: &mut MutableAppContext, update: F) -> S
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

    fn switch_mode(&mut self, mode: Mode, cx: &mut MutableAppContext) {
        self.state.mode = mode;
        self.state.operator_stack.clear();
        self.sync_editor_options(cx);
    }

    fn push_operator(&mut self, operator: Operator, cx: &mut MutableAppContext) {
        self.state.operator_stack.push(operator);
        self.sync_editor_options(cx);
    }

    fn pop_operator(&mut self, cx: &mut MutableAppContext) -> Operator {
        let popped_operator = self.state.operator_stack.pop().expect("Operator popped when no operator was on the stack. This likely means there is an invalid keymap config");
        self.sync_editor_options(cx);
        popped_operator
    }

    fn clear_operator(&mut self, cx: &mut MutableAppContext) {
        self.state.operator_stack.clear();
        self.sync_editor_options(cx);
    }

    fn active_operator(&self) -> Option<Operator> {
        self.state.operator_stack.last().copied()
    }

    fn set_enabled(&mut self, enabled: bool, cx: &mut MutableAppContext) {
        if self.enabled != enabled {
            self.enabled = enabled;
            self.state = Default::default();
            if enabled {
                self.state.mode = Mode::Normal;
            }
            self.sync_editor_options(cx);
        }
    }

    fn sync_editor_options(&self, cx: &mut MutableAppContext) {
        let state = &self.state;
        let cursor_shape = state.cursor_shape();

        for editor in self.editors.values() {
            if let Some(editor) = editor.upgrade(cx) {
                editor.update(cx, |editor, cx| {
                    if self.enabled {
                        editor.set_cursor_shape(cursor_shape, cx);
                        editor.set_clip_at_line_ends(state.clip_at_line_end(), cx);
                        editor.set_input_enabled(!state.vim_controlled());
                        editor.selections.line_mode =
                            matches!(state.mode, Mode::Visual { line: true });
                        let context_layer = state.keymap_context_layer();
                        editor.set_keymap_context_layer::<Self>(context_layer);
                    } else {
                        editor.set_cursor_shape(CursorShape::Bar, cx);
                        editor.set_clip_at_line_ends(false, cx);
                        editor.set_input_enabled(true);
                        editor.selections.line_mode = false;
                        editor.remove_keymap_context_layer::<Self>();
                    }

                    editor.change_selections(None, cx, |s| {
                        s.move_with(|map, selection| {
                            selection.set_head(
                                map.clip_point(selection.head(), Bias::Left),
                                selection.goal,
                            );
                            if state.empty_selections_only() {
                                selection.collapse_to(selection.head(), selection.goal)
                            }
                        });
                    })
                });
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{state::Mode, vim_test_context::VimTestContext};

    #[gpui::test]
    async fn test_initially_disabled(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, false).await;
        cx.simulate_keystrokes(["h", "j", "k", "l"]);
        cx.assert_editor_state("hjkl|");
    }

    #[gpui::test]
    async fn test_toggle_through_settings(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;

        cx.simulate_keystroke("i");
        assert_eq!(cx.mode(), Mode::Insert);

        // Editor acts as though vim is disabled
        cx.disable_vim();
        cx.simulate_keystrokes(["h", "j", "k", "l"]);
        cx.assert_editor_state("hjkl|");

        // Enabling dynamically sets vim mode again and restores normal mode
        cx.enable_vim();
        assert_eq!(cx.mode(), Mode::Normal);
        cx.simulate_keystrokes(["h", "h", "h", "l"]);
        assert_eq!(cx.editor_text(), "hjkl".to_owned());
        cx.assert_editor_state("h|jkl");
        cx.simulate_keystrokes(["i", "T", "e", "s", "t"]);
        cx.assert_editor_state("hTest|jkl");

        // Disabling and enabling resets to normal mode
        assert_eq!(cx.mode(), Mode::Insert);
        cx.disable_vim();
        cx.enable_vim();
        assert_eq!(cx.mode(), Mode::Normal);
    }
}
