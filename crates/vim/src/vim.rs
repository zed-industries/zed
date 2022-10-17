#[cfg(test)]
mod test;

mod editor_events;
mod insert;
mod motion;
mod normal;
mod object;
mod state;
mod utils;
mod visual;

use collections::HashMap;
use command_palette::CommandPaletteFilter;
use editor::{Bias, Cancel, Editor};
use gpui::{impl_actions, MutableAppContext, Subscription, ViewContext, WeakViewHandle};
use language::CursorShape;
use serde::Deserialize;

use settings::Settings;
use state::{Mode, Operator, VimState};
use workspace::{self, Workspace};

#[derive(Clone, Deserialize, PartialEq)]
pub struct SwitchMode(pub Mode);

#[derive(Clone, Deserialize, PartialEq)]
pub struct PushOperator(pub Operator);

#[derive(Clone, Deserialize, PartialEq)]
struct Number(u8);

impl_actions!(vim, [Number, SwitchMode, PushOperator]);

pub fn init(cx: &mut MutableAppContext) {
    editor_events::init(cx);
    normal::init(cx);
    visual::init(cx);
    insert::init(cx);
    object::init(cx);
    motion::init(cx);

    // Vim Actions
    cx.add_action(|_: &mut Workspace, &SwitchMode(mode): &SwitchMode, cx| {
        Vim::update(cx, |vim, cx| vim.switch_mode(mode, false, cx))
    });
    cx.add_action(
        |_: &mut Workspace, &PushOperator(operator): &PushOperator, cx| {
            Vim::update(cx, |vim, cx| vim.push_operator(operator, cx))
        },
    );
    cx.add_action(|_: &mut Workspace, n: &Number, cx: _| {
        Vim::update(cx, |vim, cx| vim.push_number(n, cx));
    });

    // Editor Actions
    cx.add_action(|_: &mut Editor, _: &Cancel, cx| {
        // If we are in a non normal mode or have an active operator, swap to normal mode
        // Otherwise forward cancel on to the editor
        let vim = Vim::read(cx);
        if vim.state.mode != Mode::Normal || vim.active_operator().is_some() {
            MutableAppContext::defer(cx, |cx| {
                Vim::update(cx, |state, cx| {
                    state.switch_mode(Mode::Normal, false, cx);
                });
            });
        } else {
            cx.propagate_action();
        }
    });

    // Sync initial settings with the rest of the app
    Vim::update(cx, |state, cx| state.sync_vim_settings(cx));

    // Any time settings change, update vim mode to match
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

    fn switch_mode(&mut self, mode: Mode, leave_selections: bool, cx: &mut MutableAppContext) {
        self.state.mode = mode;
        self.state.operator_stack.clear();

        // Sync editor settings like clip mode
        self.sync_vim_settings(cx);

        if leave_selections {
            return;
        }

        // Adjust selections
        for editor in self.editors.values() {
            if let Some(editor) = editor.upgrade(cx) {
                editor.update(cx, |editor, cx| {
                    editor.change_selections(None, cx, |s| {
                        s.move_with(|map, selection| {
                            if self.state.empty_selections_only() {
                                let new_head = map.clip_point(selection.head(), Bias::Left);
                                selection.collapse_to(new_head, selection.goal)
                            } else {
                                selection.set_head(
                                    map.clip_point(selection.head(), Bias::Left),
                                    selection.goal,
                                );
                            }
                        });
                    })
                })
            }
        }
    }

    fn push_operator(&mut self, operator: Operator, cx: &mut MutableAppContext) {
        self.state.operator_stack.push(operator);
        self.sync_vim_settings(cx);
    }

    fn push_number(&mut self, Number(number): &Number, cx: &mut MutableAppContext) {
        if let Some(Operator::Number(current_number)) = self.active_operator() {
            self.pop_operator(cx);
            self.push_operator(Operator::Number(current_number * 10 + *number as usize), cx);
        } else {
            self.push_operator(Operator::Number(*number as usize), cx);
        }
    }

    fn pop_operator(&mut self, cx: &mut MutableAppContext) -> Operator {
        let popped_operator = self.state.operator_stack.pop()
            .expect("Operator popped when no operator was on the stack. This likely means there is an invalid keymap config");
        self.sync_vim_settings(cx);
        popped_operator
    }

    fn pop_number_operator(&mut self, cx: &mut MutableAppContext) -> usize {
        let mut times = 1;
        if let Some(Operator::Number(number)) = self.active_operator() {
            times = number;
            self.pop_operator(cx);
        }
        times
    }

    fn clear_operator(&mut self, cx: &mut MutableAppContext) {
        self.state.operator_stack.clear();
        self.sync_vim_settings(cx);
    }

    fn active_operator(&self) -> Option<Operator> {
        self.state.operator_stack.last().copied()
    }

    fn set_enabled(&mut self, enabled: bool, cx: &mut MutableAppContext) {
        if self.enabled != enabled {
            self.enabled = enabled;
            self.state = Default::default();
            if enabled {
                self.switch_mode(Mode::Normal, false, cx);
            }
            self.sync_vim_settings(cx);
        }
    }

    fn sync_vim_settings(&self, cx: &mut MutableAppContext) {
        let state = &self.state;
        let cursor_shape = state.cursor_shape();

        cx.update_default_global::<CommandPaletteFilter, _, _>(|filter, _| {
            if self.enabled {
                filter.filtered_namespaces.remove("vim");
            } else {
                filter.filtered_namespaces.insert("vim");
            }
        });

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
                });
            }
        }
    }
}
