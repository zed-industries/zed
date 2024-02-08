//! Vim support for Zed.

#[cfg(test)]
mod test;

mod command;
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
use collections::HashMap;
use command_palette::CommandPaletteInterceptor;
use copilot::CommandPaletteFilter;
use editor::{movement, Editor, EditorEvent, EditorMode};
use gpui::{
    actions, impl_actions, Action, AppContext, EntityId, Global, KeyContext, Subscription, View,
    ViewContext, WeakView, WindowContext,
};
use language::{CursorShape, Point, Selection, SelectionGoal};
pub use mode_indicator::ModeIndicator;
use motion::Motion;
use normal::normal_replace;
use serde::Deserialize;
use settings::{update_settings_file, Settings, SettingsStore};
use state::{EditorState, Mode, Operator, RecordedSelection, WorkspaceState};
use std::{ops::Range, sync::Arc};
use visual::{visual_block_motion, visual_replace};
use workspace::{self, Workspace};

use crate::state::ReplayableAction;

/// Whether or not to enable Vim mode (work in progress).
///
/// Default: false
pub struct VimModeSetting(pub bool);

/// An Action to Switch between modes
#[derive(Clone, Deserialize, PartialEq)]
pub struct SwitchMode(pub Mode);

/// PushOperator is used to put vim into a "minor" mode,
/// where it's waiting for a specific next set of keystrokes.
/// For example 'd' needs a motion to complete.
#[derive(Clone, Deserialize, PartialEq)]
pub struct PushOperator(pub Operator);

/// Number is used to manage vim's count. Pushing a digit
/// multiplis the current value by 10 and adds the digit.
#[derive(Clone, Deserialize, PartialEq)]
struct Number(usize);

actions!(
    vim,
    [Tab, Enter, Object, InnerObject, FindForward, FindBackward]
);

// in the workspace namespace so it's not filtered out when vim is disabled.
actions!(workspace, [ToggleVimMode]);

impl_actions!(vim, [SwitchMode, PushOperator, Number]);

/// Initializes the `vim` crate.
pub fn init(cx: &mut AppContext) {
    cx.set_global(Vim::default());
    VimModeSetting::register(cx);

    editor_events::init(cx);

    cx.observe_new_views(|workspace: &mut Workspace, cx| register(workspace, cx))
        .detach();

    // Any time settings change, update vim mode to match. The Vim struct
    // will be initialized as disabled by default, so we filter its commands
    // out when starting up.
    cx.update_global::<CommandPaletteFilter, _>(|filter, _| {
        filter.hidden_namespaces.insert("vim");
    });
    cx.update_global(|vim: &mut Vim, cx: &mut AppContext| {
        vim.set_enabled(VimModeSetting::get_global(cx).0, cx)
    });
    cx.observe_global::<SettingsStore>(|cx| {
        cx.update_global(|vim: &mut Vim, cx: &mut AppContext| {
            vim.set_enabled(VimModeSetting::get_global(cx).0, cx)
        });
    })
    .detach();
}

fn register(workspace: &mut Workspace, cx: &mut ViewContext<Workspace>) {
    workspace.register_action(|_: &mut Workspace, &SwitchMode(mode): &SwitchMode, cx| {
        Vim::update(cx, |vim, cx| vim.switch_mode(mode, false, cx))
    });
    workspace.register_action(
        |_: &mut Workspace, &PushOperator(operator): &PushOperator, cx| {
            Vim::update(cx, |vim, cx| vim.push_operator(operator, cx))
        },
    );
    workspace.register_action(|_: &mut Workspace, n: &Number, cx: _| {
        Vim::update(cx, |vim, cx| vim.push_count_digit(n.0, cx));
    });

    workspace.register_action(|_: &mut Workspace, _: &Tab, cx| {
        Vim::active_editor_input_ignored(" ".into(), cx)
    });

    workspace.register_action(|_: &mut Workspace, _: &Enter, cx| {
        Vim::active_editor_input_ignored("\n".into(), cx)
    });

    workspace.register_action(|workspace: &mut Workspace, _: &ToggleVimMode, cx| {
        let fs = workspace.app_state().fs.clone();
        let currently_enabled = VimModeSetting::get_global(cx).0;
        update_settings_file::<VimModeSetting>(fs, cx, move |setting| {
            *setting = Some(!currently_enabled)
        })
    });

    normal::register(workspace, cx);
    insert::register(workspace, cx);
    motion::register(workspace, cx);
    command::register(workspace, cx);
    object::register(workspace, cx);
    visual::register(workspace, cx);
}

/// Registers a keystroke observer to observe keystrokes for the Vim integration.
pub fn observe_keystrokes(cx: &mut WindowContext) {
    cx.observe_keystrokes(|keystroke_event, cx| {
        if let Some(action) = keystroke_event
            .action
            .as_ref()
            .map(|action| action.boxed_clone())
        {
            Vim::update(cx, |vim, _| {
                if vim.workspace_state.recording {
                    vim.workspace_state
                        .recorded_actions
                        .push(ReplayableAction::Action(action.boxed_clone()));

                    if vim.workspace_state.stop_recording_after_next_action {
                        vim.workspace_state.recording = false;
                        vim.workspace_state.stop_recording_after_next_action = false;
                    }
                }
            });

            // Keystroke is handled by the vim system, so continue forward
            if action.name().starts_with("vim::") {
                return;
            }
        } else if cx.has_pending_keystrokes() {
            return;
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
    })
    .detach()
}

/// The state pertaining to Vim mode.
#[derive(Default)]
struct Vim {
    active_editor: Option<WeakView<Editor>>,
    editor_subscription: Option<Subscription>,
    enabled: bool,
    editor_states: HashMap<EntityId, EditorState>,
    workspace_state: WorkspaceState,
    default_state: EditorState,
}

impl Global for Vim {}

impl Vim {
    fn read(cx: &mut AppContext) -> &Self {
        cx.global::<Self>()
    }

    fn update<F, S>(cx: &mut WindowContext, update: F) -> S
    where
        F: FnOnce(&mut Self, &mut WindowContext) -> S,
    {
        cx.update_global(update)
    }

    fn set_active_editor(&mut self, editor: View<Editor>, cx: &mut WindowContext) {
        self.active_editor = Some(editor.clone().downgrade());
        self.editor_subscription = Some(cx.subscribe(&editor, |editor, event, cx| match event {
            EditorEvent::SelectionsChanged { local: true } => {
                let editor = editor.read(cx);
                if editor.leader_peer_id().is_none() {
                    let newest = editor.selections.newest::<usize>(cx);
                    let is_multicursor = editor.selections.count() > 1;
                    local_selections_changed(newest, is_multicursor, cx);
                }
            }
            EditorEvent::InputIgnored { text } => {
                Vim::active_editor_input_ignored(text.clone(), cx);
                Vim::record_insertion(text, None, cx)
            }
            EditorEvent::InputHandled {
                text,
                utf16_range_to_replace: range_to_replace,
            } => Vim::record_insertion(text, range_to_replace.clone(), cx),
            _ => {}
        }));

        if self.enabled {
            let editor = editor.read(cx);
            let editor_mode = editor.mode();
            let newest_selection_empty = editor.selections.newest::<usize>(cx).is_empty();

            if editor_mode == EditorMode::Full
                && !newest_selection_empty
                && self.state().mode == Mode::Normal
                // When following someone, don't switch vim mode.
                && editor.leader_peer_id().is_none()
            {
                self.switch_mode(Mode::Visual, true, cx);
            }
        }

        self.sync_vim_settings(cx);
    }

    fn record_insertion(
        text: &Arc<str>,
        range_to_replace: Option<Range<isize>>,
        cx: &mut WindowContext,
    ) {
        Vim::update(cx, |vim, _| {
            if vim.workspace_state.recording {
                vim.workspace_state
                    .recorded_actions
                    .push(ReplayableAction::Insertion {
                        text: text.clone(),
                        utf16_range_to_replace: range_to_replace,
                    });
                if vim.workspace_state.stop_recording_after_next_action {
                    vim.workspace_state.recording = false;
                    vim.workspace_state.stop_recording_after_next_action = false;
                }
            }
        });
    }

    fn update_active_editor<S>(
        &self,
        cx: &mut WindowContext,
        update: impl FnOnce(&mut Editor, &mut ViewContext<Editor>) -> S,
    ) -> Option<S> {
        let editor = self.active_editor.clone()?.upgrade()?;
        Some(editor.update(cx, update))
    }

    /// When doing an action that modifies the buffer, we start recording so that `.`
    /// will replay the action.
    pub fn start_recording(&mut self, cx: &mut WindowContext) {
        if !self.workspace_state.replaying {
            self.workspace_state.recording = true;
            self.workspace_state.recorded_actions = Default::default();
            self.workspace_state.recorded_count = None;

            let selections = self
                .active_editor
                .as_ref()
                .and_then(|editor| editor.upgrade())
                .map(|editor| {
                    let editor = editor.read(cx);
                    (
                        editor.selections.oldest::<Point>(cx),
                        editor.selections.newest::<Point>(cx),
                    )
                });

            if let Some((oldest, newest)) = selections {
                self.workspace_state.recorded_selection = match self.state().mode {
                    Mode::Visual if newest.end.row == newest.start.row => {
                        RecordedSelection::SingleLine {
                            cols: newest.end.column - newest.start.column,
                        }
                    }
                    Mode::Visual => RecordedSelection::Visual {
                        rows: newest.end.row - newest.start.row,
                        cols: newest.end.column,
                    },
                    Mode::VisualLine => RecordedSelection::VisualLine {
                        rows: newest.end.row - newest.start.row,
                    },
                    Mode::VisualBlock => RecordedSelection::VisualBlock {
                        rows: newest.end.row.abs_diff(oldest.start.row),
                        cols: newest.end.column.abs_diff(oldest.start.column),
                    },
                    _ => RecordedSelection::None,
                }
            } else {
                self.workspace_state.recorded_selection = RecordedSelection::None;
            }
        }
    }

    /// When finishing an action that modifies the buffer, stop recording.
    /// as you usually call this within a keystroke handler we also ensure that
    /// the current action is recorded.
    pub fn stop_recording(&mut self) {
        if self.workspace_state.recording {
            self.workspace_state.stop_recording_after_next_action = true;
        }
    }

    /// Stops recording actions immediately rather than waiting until after the
    /// next action to stop recording.
    ///
    /// This doesn't include the current action.
    pub fn stop_recording_immediately(&mut self, action: Box<dyn Action>) {
        if self.workspace_state.recording {
            self.workspace_state
                .recorded_actions
                .push(ReplayableAction::Action(action.boxed_clone()));
            self.workspace_state.recording = false;
            self.workspace_state.stop_recording_after_next_action = false;
        }
    }

    /// Explicitly record one action (equivalents to start_recording and stop_recording)
    pub fn record_current_action(&mut self, cx: &mut WindowContext) {
        self.start_recording(cx);
        self.stop_recording();
    }

    fn switch_mode(&mut self, mode: Mode, leave_selections: bool, cx: &mut WindowContext) {
        let state = self.state();
        let last_mode = state.mode;
        let prior_mode = state.last_mode;
        self.update_state(|state| {
            state.last_mode = last_mode;
            state.mode = mode;
            state.operator_stack.clear();
        });
        if mode != Mode::Insert {
            self.take_count(cx);
        }

        // Sync editor settings like clip mode
        self.sync_vim_settings(cx);

        if leave_selections {
            return;
        }

        // Adjust selections
        self.update_active_editor(cx, |editor, cx| {
            if last_mode != Mode::VisualBlock && last_mode.is_visual() && mode == Mode::VisualBlock
            {
                visual_block_motion(true, editor, cx, |_, point, goal| Some((point, goal)))
            }

            editor.change_selections(None, cx, |s| {
                // we cheat with visual block mode and use multiple cursors.
                // the cost of this cheat is we need to convert back to a single
                // cursor whenever vim would.
                if last_mode == Mode::VisualBlock
                    && (mode != Mode::VisualBlock && mode != Mode::Insert)
                {
                    let tail = s.oldest_anchor().tail();
                    let head = s.newest_anchor().head();
                    s.select_anchor_ranges(vec![tail..head]);
                } else if last_mode == Mode::Insert
                    && prior_mode == Mode::VisualBlock
                    && mode != Mode::VisualBlock
                {
                    let pos = s.first_anchor().head();
                    s.select_anchor_ranges(vec![pos..pos])
                }

                s.move_with(|map, selection| {
                    if last_mode.is_visual() && !mode.is_visual() {
                        let mut point = selection.head();
                        if !selection.reversed && !selection.is_empty() {
                            point = movement::left(map, selection.head());
                        }
                        selection.collapse_to(point, selection.goal)
                    } else if !last_mode.is_visual() && mode.is_visual() {
                        if selection.is_empty() {
                            selection.end = movement::right(map, selection.start);
                        }
                    }
                });
            })
        });
    }

    fn push_count_digit(&mut self, number: usize, cx: &mut WindowContext) {
        if self.active_operator().is_some() {
            self.update_state(|state| {
                state.post_count = Some(state.post_count.unwrap_or(0) * 10 + number)
            })
        } else {
            self.update_state(|state| {
                state.pre_count = Some(state.pre_count.unwrap_or(0) * 10 + number)
            })
        }
        // update the keymap so that 0 works
        self.sync_vim_settings(cx)
    }

    fn take_count(&mut self, cx: &mut WindowContext) -> Option<usize> {
        if self.workspace_state.replaying {
            return self.workspace_state.recorded_count;
        }

        let count = if self.state().post_count == None && self.state().pre_count == None {
            return None;
        } else {
            Some(self.update_state(|state| {
                state.post_count.take().unwrap_or(1) * state.pre_count.take().unwrap_or(1)
            }))
        };
        if self.workspace_state.recording {
            self.workspace_state.recorded_count = count;
        }
        self.sync_vim_settings(cx);
        count
    }

    fn push_operator(&mut self, operator: Operator, cx: &mut WindowContext) {
        if matches!(
            operator,
            Operator::Change | Operator::Delete | Operator::Replace
        ) {
            self.start_recording(cx)
        };
        self.update_state(|state| state.operator_stack.push(operator));
        self.sync_vim_settings(cx);
    }

    fn maybe_pop_operator(&mut self) -> Option<Operator> {
        self.update_state(|state| state.operator_stack.pop())
    }

    fn pop_operator(&mut self, cx: &mut WindowContext) -> Operator {
        let popped_operator = self.update_state( |state| state.operator_stack.pop()
        )            .expect("Operator popped when no operator was on the stack. This likely means there is an invalid keymap config");
        self.sync_vim_settings(cx);
        popped_operator
    }
    fn clear_operator(&mut self, cx: &mut WindowContext) {
        self.take_count(cx);
        self.update_state(|state| state.operator_stack.clear());
        self.sync_vim_settings(cx);
    }

    fn active_operator(&self) -> Option<Operator> {
        self.state().operator_stack.last().copied()
    }

    fn active_editor_input_ignored(text: Arc<str>, cx: &mut WindowContext) {
        if text.is_empty() {
            return;
        }

        match Vim::read(cx).active_operator() {
            Some(Operator::FindForward { before }) => {
                let find = Motion::FindForward {
                    before,
                    char: text.chars().next().unwrap(),
                };
                Vim::update(cx, |vim, _| {
                    vim.workspace_state.last_find = Some(find.clone())
                });
                motion::motion(find, cx)
            }
            Some(Operator::FindBackward { after }) => {
                let find = Motion::FindBackward {
                    after,
                    char: text.chars().next().unwrap(),
                };
                Vim::update(cx, |vim, _| {
                    vim.workspace_state.last_find = Some(find.clone())
                });
                motion::motion(find, cx)
            }
            Some(Operator::Replace) => match Vim::read(cx).state().mode {
                Mode::Normal => normal_replace(text, cx),
                Mode::Visual | Mode::VisualLine | Mode::VisualBlock => visual_replace(text, cx),
                _ => Vim::update(cx, |vim, cx| vim.clear_operator(cx)),
            },
            _ => {}
        }
    }

    fn set_enabled(&mut self, enabled: bool, cx: &mut AppContext) {
        if self.enabled != enabled {
            self.enabled = enabled;

            cx.update_global::<CommandPaletteFilter, _>(|filter, _| {
                if self.enabled {
                    filter.hidden_namespaces.remove("vim");
                } else {
                    filter.hidden_namespaces.insert("vim");
                }
            });

            if self.enabled {
                cx.set_global::<CommandPaletteInterceptor>(CommandPaletteInterceptor(Box::new(
                    command::command_interceptor,
                )));
            } else if cx.has_global::<CommandPaletteInterceptor>() {
                let _ = cx.remove_global::<CommandPaletteInterceptor>();
            }

            if let Some(active_window) = cx.active_window() {
                active_window
                    .update(cx, |root_view, cx| {
                        if self.enabled {
                            let active_editor = root_view
                                .downcast::<Workspace>()
                                .ok()
                                .and_then(|workspace| workspace.read(cx).active_item(cx))
                                .and_then(|item| item.downcast::<Editor>());
                            if let Some(active_editor) = active_editor {
                                self.set_active_editor(active_editor, cx);
                            }
                            self.switch_mode(Mode::Normal, false, cx);
                        }
                        self.sync_vim_settings(cx);
                    })
                    .ok();
            }
        }
    }

    /// Returns the state of the active editor.
    pub fn state(&self) -> &EditorState {
        if let Some(active_editor) = self.active_editor.as_ref() {
            if let Some(state) = self.editor_states.get(&active_editor.entity_id()) {
                return state;
            }
        }

        &self.default_state
    }

    /// Updates the state of the active editor.
    pub fn update_state<T>(&mut self, func: impl FnOnce(&mut EditorState) -> T) -> T {
        let mut state = self.state().clone();
        let ret = func(&mut state);

        if let Some(active_editor) = self.active_editor.as_ref() {
            self.editor_states.insert(active_editor.entity_id(), state);
        }

        ret
    }

    fn sync_vim_settings(&self, cx: &mut WindowContext) {
        let state = self.state();
        let cursor_shape = state.cursor_shape();

        self.update_active_editor(cx, |editor, cx| {
            if self.enabled && editor.mode() == EditorMode::Full {
                editor.set_cursor_shape(cursor_shape, cx);
                editor.set_clip_at_line_ends(state.clip_at_line_ends(), cx);
                editor.set_collapse_matches(true);
                editor.set_input_enabled(!state.vim_controlled());
                editor.set_autoindent(state.should_autoindent());
                editor.selections.line_mode = matches!(state.mode, Mode::VisualLine);
                let context_layer = state.keymap_context_layer();
                editor.set_keymap_context_layer::<Self>(context_layer, cx);
            } else {
                // Note: set_collapse_matches is not in unhook_vim_settings, as that method is called on blur,
                // but we need collapse_matches to persist when the search bar is focused.
                editor.set_collapse_matches(false);
                self.unhook_vim_settings(editor, cx);
            }
        });
    }

    fn unhook_vim_settings(&self, editor: &mut Editor, cx: &mut ViewContext<Editor>) {
        editor.set_cursor_shape(CursorShape::Bar, cx);
        editor.set_clip_at_line_ends(false, cx);
        editor.set_input_enabled(true);
        editor.set_autoindent(true);
        editor.selections.line_mode = false;

        // we set the VimEnabled context on all editors so that we
        // can distinguish between vim mode and non-vim mode in the BufferSearchBar.
        // This is a bit of a hack, but currently the search crate does not depend on vim,
        // and it seems nice to keep it that way.
        if self.enabled {
            let mut context = KeyContext::default();
            context.add("VimEnabled");
            editor.set_keymap_context_layer::<Self>(context, cx)
        } else {
            editor.remove_keymap_context_layer::<Self>(cx);
        }
    }
}

impl Settings for VimModeSetting {
    const KEY: Option<&'static str> = Some("vim_mode");

    type FileContent = Option<bool>;

    fn load(
        default_value: &Self::FileContent,
        user_values: &[&Self::FileContent],
        _: &mut AppContext,
    ) -> Result<Self> {
        Ok(Self(user_values.iter().rev().find_map(|v| **v).unwrap_or(
            default_value.ok_or_else(Self::missing_default)?,
        )))
    }
}

fn local_selections_changed(
    newest: Selection<usize>,
    is_multicursor: bool,
    cx: &mut WindowContext,
) {
    Vim::update(cx, |vim, cx| {
        if vim.enabled {
            if vim.state().mode == Mode::Normal && !newest.is_empty() {
                if matches!(newest.goal, SelectionGoal::HorizontalRange { .. }) {
                    vim.switch_mode(Mode::VisualBlock, false, cx);
                } else {
                    vim.switch_mode(Mode::Visual, false, cx)
                }
            } else if newest.is_empty()
                && !is_multicursor
                && [Mode::Visual, Mode::VisualLine, Mode::VisualBlock].contains(&vim.state().mode)
            {
                vim.switch_mode(Mode::Normal, true, cx)
            }
        }
    })
}
