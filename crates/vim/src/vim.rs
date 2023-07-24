#[cfg(test)]
mod test;

mod editor_events;
mod insert;
mod mode_indicator;
mod motion;
mod normal;
mod object;
mod state;
mod utils;
mod visual;

use anyhow::Result;
use collections::CommandPaletteFilter;
use editor::{Bias, Editor, EditorMode, Event};
use gpui::{
    actions, impl_actions, keymap_matcher::KeymapContext, AppContext, Subscription, ViewContext,
    ViewHandle, WeakViewHandle, WindowContext,
};
use language::CursorShape;
pub use mode_indicator::ModeIndicator;
use motion::Motion;
use normal::normal_replace;
use serde::Deserialize;
use settings::{Setting, SettingsStore};
use state::{Mode, Operator, VimState};
use std::sync::Arc;
use visual::visual_replace;
use workspace::{self, Workspace};

struct VimModeSetting(bool);

#[derive(Clone, Deserialize, PartialEq)]
pub struct SwitchMode(pub Mode);

#[derive(Clone, Deserialize, PartialEq)]
pub struct PushOperator(pub Operator);

#[derive(Clone, Deserialize, PartialEq)]
struct Number(u8);

actions!(vim, [Tab, Enter]);
impl_actions!(vim, [Number, SwitchMode, PushOperator]);

pub fn init(cx: &mut AppContext) {
    settings::register::<VimModeSetting>(cx);

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
        vim.set_enabled(settings::get::<VimModeSetting>(cx).0, cx)
    });
    cx.observe_global::<SettingsStore, _>(|cx| {
        cx.update_default_global(|vim: &mut Vim, cx: &mut AppContext| {
            vim.set_enabled(settings::get::<VimModeSetting>(cx).0, cx)
        });
    })
    .detach();
}

pub fn observe_keystrokes(cx: &mut WindowContext) {
    cx.observe_keystrokes(|_keystroke, _result, handled_by, cx| {
        if let Some(handled_by) = handled_by {
            // Keystroke is handled by the vim system, so continue forward
            if handled_by.namespace() == "vim" {
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
    mode_indicator: Option<ViewHandle<ModeIndicator>>,

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

        if let Some(mode_indicator) = &self.mode_indicator {
            mode_indicator.update(cx, |mode_indicator, cx| mode_indicator.set_mode(mode, cx))
        }

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

    fn pop_number_operator(&mut self, cx: &mut WindowContext) -> Option<usize> {
        if let Some(Operator::Number(number)) = self.active_operator() {
            self.pop_operator(cx);
            return Some(number);
        }
        None
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

    fn sync_mode_indicator(cx: &mut AppContext) {
        cx.spawn(|mut cx| async move {
            let workspace = match cx.update(|cx| {
                cx.update_active_window(|cx| {
                    cx.root_view()
                        .downcast_ref::<Workspace>()
                        .map(|workspace| workspace.downgrade())
                })
            }) {
                Some(Some(workspace)) => workspace,
                _ => {
                    return Ok(());
                }
            };

            workspace.update(&mut cx, |workspace, cx| {
                Vim::update(cx, |vim, cx| {
                    workspace.status_bar().update(cx, |status_bar, cx| {
                        let current_position = status_bar.position_of_item::<ModeIndicator>();
                        if vim.enabled && current_position.is_none() {
                            if vim.mode_indicator.is_none() {
                                vim.mode_indicator =
                                    Some(cx.add_view(|_| ModeIndicator::new(vim.state.mode)));
                            };
                            let mode_indicator = vim.mode_indicator.as_ref().unwrap();
                            // TODO: would it be better to depend on the diagnostics crate
                            // so we can pass the type directly?
                            let position = status_bar.position_of_named_item("DiagnosticIndicator");
                            if let Some(position) = position {
                                status_bar.insert_item_after(position, mode_indicator.clone(), cx)
                            } else {
                                status_bar.add_left_item(mode_indicator.clone(), cx)
                            }
                        } else if !vim.enabled {
                            if let Some(position) = current_position {
                                status_bar.remove_item_at(position, cx)
                            }
                        }
                    })
                })
            })
        })
        .detach_and_log_err(cx);
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
                editor.set_collapse_matches(true);
                editor.set_input_enabled(!state.vim_controlled());
                editor.selections.line_mode = matches!(state.mode, Mode::Visual { line: true });
                let context_layer = state.keymap_context_layer();
                editor.set_keymap_context_layer::<Self>(context_layer, cx);
            } else {
                // Note: set_collapse_matches is not in unhook_vim_settings, as that method is called on blur,
                // but we need collapse_matches to persist when the search bar is focused.
                editor.set_collapse_matches(false);
                self.unhook_vim_settings(editor, cx);
            }
        });

        Vim::sync_mode_indicator(cx);
    }

    fn unhook_vim_settings(&self, editor: &mut Editor, cx: &mut ViewContext<Editor>) {
        editor.set_cursor_shape(CursorShape::Bar, cx);
        editor.set_clip_at_line_ends(false, cx);
        editor.set_input_enabled(true);
        editor.selections.line_mode = false;

        // we set the VimEnabled context on all editors so that we
        // can distinguish between vim mode and non-vim mode in the BufferSearchBar.
        // This is a bit of a hack, but currently the search crate does not depend on vim,
        // and it seems nice to keep it that way.
        if self.enabled {
            let mut context = KeymapContext::default();
            context.add_identifier("VimEnabled");
            editor.set_keymap_context_layer::<Self>(context, cx)
        } else {
            editor.remove_keymap_context_layer::<Self>(cx);
        }
    }
}

impl Setting for VimModeSetting {
    const KEY: Option<&'static str> = Some("vim_mode");

    type FileContent = Option<bool>;

    fn load(
        default_value: &Self::FileContent,
        user_values: &[&Self::FileContent],
        _: &AppContext,
    ) -> Result<Self> {
        Ok(Self(user_values.iter().rev().find_map(|v| **v).unwrap_or(
            default_value.ok_or_else(Self::missing_default)?,
        )))
    }
}

fn local_selections_changed(newest_empty: bool, cx: &mut WindowContext) {
    Vim::update(cx, |vim, cx| {
        if vim.enabled && vim.state.mode == Mode::Normal && !newest_empty {
            vim.switch_mode(Mode::Visual { line: false }, false, cx)
        }
    })
}
