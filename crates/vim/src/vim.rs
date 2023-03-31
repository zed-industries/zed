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

use std::sync::Arc;

use collections::CommandPaletteFilter;
use editor::{Bias, Cancel, Editor, EditorMode};
use gpui::{
    actions, impl_actions, MutableAppContext, Subscription, ViewContext, ViewHandle, WeakViewHandle,
};
use language::CursorShape;
use motion::Motion;
use normal::normal_replace;
use serde::Deserialize;
use settings::Settings;
use state::{Mode, Operator, VimState};
use visual::visual_replace;
use workspace::{self, Workspace};

#[derive(Clone, Deserialize, PartialEq)]
pub struct SwitchMode(pub Mode);

#[derive(Clone, Deserialize, PartialEq)]
pub struct PushOperator(pub Operator);

#[derive(Clone, Deserialize, PartialEq)]
struct Number(u8);

actions!(vim, [Tab, Enter]);
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
        // If we are in aren't in normal mode or have an active operator, swap to normal mode
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

    cx.add_action(|_: &mut Workspace, _: &Tab, cx| {
        Vim::active_editor_input_ignored(" ".into(), cx)
    });

    cx.add_action(|_: &mut Workspace, _: &Enter, cx| {
        Vim::active_editor_input_ignored("\n".into(), cx)
    });

    // Sync initial settings with the rest of the app
    Vim::update(cx, |vim, cx| vim.sync_vim_settings(cx));

    // Any time settings change, update vim mode to match
    cx.observe_global::<Settings, _>(|cx| {
        Vim::update(cx, |state, cx| {
            state.set_enabled(cx.global::<Settings>().vim_mode, cx)
        })
    })
    .detach();
}

pub fn observe_keystrokes(window_id: usize, cx: &mut MutableAppContext) {
    cx.observe_keystrokes(window_id, |_keystroke, _result, handled_by, cx| {
        if let Some(handled_by) = handled_by {
            // Keystroke is handled by the vim system, so continue forward
            // Also short circuit if it is the special cancel action
            if handled_by.namespace() == "vim"
                || (handled_by.namespace() == "editor" && handled_by.name() == "Cancel")
            {
                return true;
            }
        }

        Vim::update(cx, |vim, cx| match vim.active_operator() {
            Some(
                Operator::FindForward { .. } | Operator::FindBackward { .. } | Operator::Replace,
            ) => {}
            Some(_) => {
                vim.clear_operator(cx);
            }
            _ => {}
        });
        true
    })
    .detach()
}

#[derive(Default)]
pub struct Vim {
    active_editor: Option<WeakViewHandle<Editor>>,
    editor_subscription: Option<Subscription>,

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
        if let Some(editor) = self
            .active_editor
            .as_ref()
            .and_then(|editor| editor.upgrade(cx))
        {
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

    fn active_editor_input_ignored(text: Arc<str>, cx: &mut MutableAppContext) {
        if text.is_empty() {
            return;
        }

        match Vim::read(cx).active_operator() {
            Some(Operator::FindForward { before }) => {
                motion::motion(Motion::FindForward { before, text }, cx)
            }
            Some(Operator::FindBackward { after }) => {
                motion::motion(Motion::FindBackward { after, text }, cx)
            }
            Some(Operator::Replace) => match Vim::read(cx).state.mode {
                Mode::Normal => normal_replace(text, cx),
                Mode::Visual { line } => visual_replace(text, line, cx),
                _ => Vim::update(cx, |vim, cx| vim.clear_operator(cx)),
            },
            _ => {}
        }
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

        if let Some(editor) = self
            .active_editor
            .as_ref()
            .and_then(|editor| editor.upgrade(cx))
        {
            if self.enabled && editor.read(cx).mode() == EditorMode::Full {
                editor.update(cx, |editor, cx| {
                    editor.set_cursor_shape(cursor_shape, cx);
                    editor.set_clip_at_line_ends(state.clip_at_line_end(), cx);
                    editor.set_input_enabled(!state.vim_controlled());
                    editor.selections.line_mode = matches!(state.mode, Mode::Visual { line: true });
                    let context_layer = state.keymap_context_layer();
                    editor.set_keymap_context_layer::<Self>(context_layer);
                });
            } else {
                self.unhook_vim_settings(editor, cx);
            }
        }
    }

    fn unhook_vim_settings(&self, editor: ViewHandle<Editor>, cx: &mut MutableAppContext) {
        editor.update(cx, |editor, cx| {
            editor.set_cursor_shape(CursorShape::Bar, cx);
            editor.set_clip_at_line_ends(false, cx);
            editor.set_input_enabled(true);
            editor.selections.line_mode = false;
            editor.remove_keymap_context_layer::<Self>();
        });
    }
}
