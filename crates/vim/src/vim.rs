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
use editor::{Bias, Cancel, Editor, EditorMode, Event};
use gpui::{
    actions, impl_actions, AppContext, Subscription, ViewContext, ViewHandle, WeakViewHandle,
    WindowContext,
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

pub fn init(cx: &mut AppContext) {
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
            WindowContext::defer(cx, |cx| {
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

    // Any time settings change, update vim mode to match. The Vim struct
    // will be initialized as disabled by default, so we filter its commands
    // out when starting up.
    cx.update_default_global::<CommandPaletteFilter, _, _>(|filter, _| {
        filter.filtered_namespaces.insert("vim");
    });
    cx.update_default_global(|vim: &mut Vim, cx: &mut AppContext| {
        vim.set_enabled(cx.global::<Settings>().vim_mode, cx)
    });
    cx.observe_global::<Settings, _>(|cx| {
        cx.update_default_global(|vim: &mut Vim, cx: &mut AppContext| {
            vim.set_enabled(cx.global::<Settings>().vim_mode, cx)
        });
    })
    .detach();
}

pub fn observe_keystrokes(cx: &mut WindowContext) {
    cx.observe_keystrokes(|_keystroke, _result, handled_by, cx| {
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
    fn read(cx: &mut AppContext) -> &Self {
        cx.default_global()
    }

    fn update<F, S>(cx: &mut WindowContext, update: F) -> S
    where
        F: FnOnce(&mut Self, &mut WindowContext) -> S,
    {
        cx.update_default_global(update)
    }

    fn set_active_editor(&mut self, editor: ViewHandle<Editor>, cx: &mut WindowContext) {
        self.active_editor = Some(editor.downgrade());
        self.editor_subscription = Some(cx.subscribe(&editor, |editor, event, cx| match event {
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

        if self.enabled {
            let editor = editor.read(cx);
            let editor_mode = editor.mode();
            let newest_selection_empty = editor.selections.newest::<usize>(cx).is_empty();

            if editor_mode == EditorMode::Full && !newest_selection_empty {
                self.switch_mode(Mode::Visual { line: false }, true, cx);
            }
        }

        self.sync_vim_settings(cx);
    }

    fn update_active_editor<S>(
        &self,
        cx: &mut WindowContext,
        update: impl FnOnce(&mut Editor, &mut ViewContext<Editor>) -> S,
    ) -> Option<S> {
        let editor = self.active_editor.clone()?.upgrade(cx)?;
        Some(editor.update(cx, update))
    }

    fn switch_mode(&mut self, mode: Mode, leave_selections: bool, cx: &mut WindowContext) {
        self.state.mode = mode;
        self.state.operator_stack.clear();

        // Sync editor settings like clip mode
        self.sync_vim_settings(cx);

        if leave_selections {
            return;
        }

        // Adjust selections
        self.update_active_editor(cx, |editor, cx| {
            editor.change_selections(None, cx, |s| {
                s.move_with(|map, selection| {
                    if self.state.empty_selections_only() {
                        let new_head = map.clip_point(selection.head(), Bias::Left);
                        selection.collapse_to(new_head, selection.goal)
                    } else {
                        selection
                            .set_head(map.clip_point(selection.head(), Bias::Left), selection.goal);
                    }
                });
            })
        });
    }

    fn push_operator(&mut self, operator: Operator, cx: &mut WindowContext) {
        self.state.operator_stack.push(operator);
        self.sync_vim_settings(cx);
    }

    fn push_number(&mut self, Number(number): &Number, cx: &mut WindowContext) {
        if let Some(Operator::Number(current_number)) = self.active_operator() {
            self.pop_operator(cx);
            self.push_operator(Operator::Number(current_number * 10 + *number as usize), cx);
        } else {
            self.push_operator(Operator::Number(*number as usize), cx);
        }
    }

    fn pop_operator(&mut self, cx: &mut WindowContext) -> Operator {
        let popped_operator = self.state.operator_stack.pop()
            .expect("Operator popped when no operator was on the stack. This likely means there is an invalid keymap config");
        self.sync_vim_settings(cx);
        popped_operator
    }

    fn pop_number_operator(&mut self, cx: &mut WindowContext) -> usize {
        let mut times = 1;
        if let Some(Operator::Number(number)) = self.active_operator() {
            times = number;
            self.pop_operator(cx);
        }
        times
    }

    fn clear_operator(&mut self, cx: &mut WindowContext) {
        self.state.operator_stack.clear();
        self.sync_vim_settings(cx);
    }

    fn active_operator(&self) -> Option<Operator> {
        self.state.operator_stack.last().copied()
    }

    fn active_editor_input_ignored(text: Arc<str>, cx: &mut WindowContext) {
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

    fn set_enabled(&mut self, enabled: bool, cx: &mut AppContext) {
        if self.enabled != enabled {
            self.enabled = enabled;
            self.state = Default::default();

            cx.update_default_global::<CommandPaletteFilter, _, _>(|filter, _| {
                if self.enabled {
                    filter.filtered_namespaces.remove("vim");
                } else {
                    filter.filtered_namespaces.insert("vim");
                }
            });

            cx.update_active_window(|cx| {
                if self.enabled {
                    let active_editor = cx
                        .root_view()
                        .downcast_ref::<Workspace>()
                        .and_then(|workspace| workspace.read(cx).active_item(cx))
                        .and_then(|item| item.downcast::<Editor>());
                    if let Some(active_editor) = active_editor {
                        self.set_active_editor(active_editor, cx);
                    }
                    self.switch_mode(Mode::Normal, false, cx);
                }
                self.sync_vim_settings(cx);
            });
        }
    }

    fn sync_vim_settings(&self, cx: &mut WindowContext) {
        let state = &self.state;
        let cursor_shape = state.cursor_shape();

        self.update_active_editor(cx, |editor, cx| {
            if self.enabled && editor.mode() == EditorMode::Full {
                editor.set_cursor_shape(cursor_shape, cx);
                editor.set_clip_at_line_ends(state.clip_at_line_end(), cx);
                editor.set_input_enabled(!state.vim_controlled());
                editor.selections.line_mode = matches!(state.mode, Mode::Visual { line: true });
                let context_layer = state.keymap_context_layer();
                editor.set_keymap_context_layer::<Self>(context_layer, cx);
            } else {
                Self::unhook_vim_settings(editor, cx);
            }
        });
    }

    fn unhook_vim_settings(editor: &mut Editor, cx: &mut ViewContext<Editor>) {
        editor.set_cursor_shape(CursorShape::Bar, cx);
        editor.set_clip_at_line_ends(false, cx);
        editor.set_input_enabled(true);
        editor.selections.line_mode = false;
        editor.remove_keymap_context_layer::<Self>(cx);
    }
}

fn local_selections_changed(newest_empty: bool, cx: &mut WindowContext) {
    Vim::update(cx, |vim, cx| {
        if vim.enabled && vim.state.mode == Mode::Normal && !newest_empty {
            vim.switch_mode(Mode::Visual { line: false }, false, cx)
        }
    })
}
