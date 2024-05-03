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
mod replace;
mod state;
mod surrounds;
mod utils;
mod visual;

use anyhow::Result;
use collections::HashMap;
use command_palette_hooks::{CommandPaletteFilter, CommandPaletteInterceptor};
use editor::{
    movement::{self, FindRange},
    Anchor, Bias, Editor, EditorEvent, EditorMode, ToPoint,
};
use gpui::{
    actions, impl_actions, Action, AppContext, EntityId, FocusableView, Global, KeystrokeEvent,
    Subscription, View, ViewContext, WeakView, WindowContext,
};
use language::{CursorShape, Point, SelectionGoal, TransactionId};
pub use mode_indicator::ModeIndicator;
use motion::Motion;
use normal::normal_replace;
use replace::multi_replace;
use schemars::JsonSchema;
use serde::Deserialize;
use serde_derive::Serialize;
use settings::{update_settings_file, Settings, SettingsSources, SettingsStore};
use state::{EditorState, Mode, Operator, RecordedSelection, WorkspaceState};
use std::{ops::Range, sync::Arc};
use surrounds::{add_surrounds, change_surrounds, delete_surrounds};
use ui::BorrowAppContext;
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
    [
        Tab,
        Enter,
        Object,
        InnerObject,
        FindForward,
        FindBackward,
        OpenDefaultKeymap
    ]
);

// in the workspace namespace so it's not filtered out when vim is disabled.
actions!(workspace, [ToggleVimMode]);

impl_actions!(vim, [SwitchMode, PushOperator, Number]);

/// Initializes the `vim` crate.
pub fn init(cx: &mut AppContext) {
    cx.set_global(Vim::default());
    VimModeSetting::register(cx);
    VimSettings::register(cx);

    cx.observe_keystrokes(observe_keystrokes).detach();
    editor_events::init(cx);

    cx.observe_new_views(|workspace: &mut Workspace, cx| register(workspace, cx))
        .detach();

    // Any time settings change, update vim mode to match. The Vim struct
    // will be initialized as disabled by default, so we filter its commands
    // out when starting up.
    CommandPaletteFilter::update_global(cx, |filter, _| {
        filter.hide_namespace(Vim::NAMESPACE);
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
        |_: &mut Workspace, PushOperator(operator): &PushOperator, cx| {
            Vim::update(cx, |vim, cx| vim.push_operator(operator.clone(), cx))
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

    workspace.register_action(|_: &mut Workspace, _: &OpenDefaultKeymap, cx| {
        cx.emit(workspace::Event::OpenBundledFile {
            text: settings::vim_keymap(),
            title: "Default Vim Bindings",
            language: "JSON",
        });
    });

    normal::register(workspace, cx);
    insert::register(workspace, cx);
    motion::register(workspace, cx);
    command::register(workspace, cx);
    replace::register(workspace, cx);
    object::register(workspace, cx);
    visual::register(workspace, cx);
}

/// Called whenever an keystroke is typed so vim can observe all actions
/// and keystrokes accordingly.
fn observe_keystrokes(keystroke_event: &KeystrokeEvent, cx: &mut WindowContext) {
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
            Operator::FindForward { .. }
            | Operator::FindBackward { .. }
            | Operator::Replace
            | Operator::AddSurrounds { .. }
            | Operator::ChangeSurrounds { .. }
            | Operator::DeleteSurrounds,
        ) => {}
        Some(_) => {
            vim.clear_operator(cx);
        }
        _ => {}
    });
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
    /// The namespace for Vim actions.
    const NAMESPACE: &'static str = "vim";

    fn read(cx: &mut AppContext) -> &Self {
        cx.global::<Self>()
    }

    fn update<F, S>(cx: &mut WindowContext, update: F) -> S
    where
        F: FnOnce(&mut Self, &mut WindowContext) -> S,
    {
        cx.update_global(update)
    }

    fn activate_editor(&mut self, editor: View<Editor>, cx: &mut WindowContext) {
        if !editor.read(cx).use_modal_editing() {
            return;
        }

        self.active_editor = Some(editor.clone().downgrade());
        self.editor_subscription = Some(cx.subscribe(&editor, |editor, event, cx| match event {
            EditorEvent::SelectionsChanged { local: true } => {
                if editor.read(cx).leader_peer_id().is_none() {
                    Vim::update(cx, |vim, cx| {
                        vim.local_selections_changed(editor, cx);
                    })
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
            EditorEvent::TransactionBegun { transaction_id } => Vim::update(cx, |vim, cx| {
                vim.transaction_begun(*transaction_id, cx);
            }),
            EditorEvent::TransactionUndone { transaction_id } => Vim::update(cx, |vim, cx| {
                vim.transaction_undone(transaction_id, cx);
            }),
            _ => {}
        }));

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
        &mut self,
        cx: &mut WindowContext,
        update: impl FnOnce(&mut Vim, &mut Editor, &mut ViewContext<Editor>) -> S,
    ) -> Option<S> {
        let editor = self.active_editor.clone()?.upgrade()?;
        Some(editor.update(cx, |editor, cx| update(self, editor, cx)))
    }

    fn editor_selections(&mut self, cx: &mut WindowContext) -> Vec<Range<Anchor>> {
        self.update_active_editor(cx, |_, editor, _| {
            editor
                .selections
                .disjoint_anchors()
                .iter()
                .map(|selection| selection.tail()..selection.head())
                .collect()
        })
        .unwrap_or_default()
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

    pub fn stop_replaying(&mut self) {
        self.workspace_state.replaying = false;
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
        let prior_tx = state.current_tx;
        self.update_state(|state| {
            state.last_mode = last_mode;
            state.mode = mode;
            state.operator_stack.clear();
            state.current_tx.take();
            state.current_anchor.take();
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
        self.update_active_editor(cx, |_, editor, cx| {
            if last_mode != Mode::VisualBlock && last_mode.is_visual() && mode == Mode::VisualBlock
            {
                visual_block_motion(true, editor, cx, |_, point, goal| Some((point, goal)))
            }
            if last_mode == Mode::Insert || last_mode == Mode::Replace {
                if let Some(prior_tx) = prior_tx {
                    editor.group_until_transaction(prior_tx, cx)
                }
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

                let snapshot = s.display_map();
                if let Some(pending) = s.pending.as_mut() {
                    if pending.selection.reversed && mode.is_visual() && !last_mode.is_visual() {
                        let mut end = pending.selection.end.to_point(&snapshot.buffer_snapshot);
                        end = snapshot
                            .buffer_snapshot
                            .clip_point(end + Point::new(0, 1), Bias::Right);
                        pending.selection.end = snapshot.buffer_snapshot.anchor_before(end);
                    }
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
                    } else if last_mode == Mode::Replace {
                        if selection.head().column() != 0 {
                            let point = movement::left(map, selection.head());
                            selection.collapse_to(point, selection.goal)
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
        // Since these operations can only be entered with pre-operators,
        // we need to clear the previous operators when pushing,
        // so that the current stack is the most correct
        if matches!(
            operator,
            Operator::AddSurrounds { .. }
                | Operator::ChangeSurrounds { .. }
                | Operator::DeleteSurrounds
        ) {
            self.update_state(|state| state.operator_stack.clear());
        };
        self.update_state(|state| state.operator_stack.push(operator));
        self.sync_vim_settings(cx);
    }

    fn maybe_pop_operator(&mut self) -> Option<Operator> {
        self.update_state(|state| state.operator_stack.pop())
    }

    fn pop_operator(&mut self, cx: &mut WindowContext) -> Operator {
        let popped_operator = self.update_state(|state| state.operator_stack.pop())
            .expect("Operator popped when no operator was on the stack. This likely means there is an invalid keymap config");
        self.sync_vim_settings(cx);
        popped_operator
    }

    fn clear_operator(&mut self, cx: &mut WindowContext) {
        self.take_count(cx);
        self.update_state(|state| state.operator_stack.clear());
        self.sync_vim_settings(cx);
    }

    fn active_operator(&self) -> Option<Operator> {
        self.state().operator_stack.last().cloned()
    }

    fn transaction_begun(&mut self, transaction_id: TransactionId, _: &mut WindowContext) {
        self.update_state(|state| {
            let mode = if (state.mode == Mode::Insert
                || state.mode == Mode::Replace
                || state.mode == Mode::Normal)
                && state.current_tx.is_none()
            {
                state.current_tx = Some(transaction_id);
                state.last_mode
            } else {
                state.mode
            };
            if mode == Mode::VisualLine || mode == Mode::VisualBlock {
                state.undo_modes.insert(transaction_id, mode);
            }
        });
    }

    fn transaction_undone(&mut self, transaction_id: &TransactionId, cx: &mut WindowContext) {
        if !self.state().mode.is_visual() {
            return;
        };
        self.update_active_editor(cx, |vim, editor, cx| {
            let original_mode = vim.state().undo_modes.get(transaction_id);
            editor.change_selections(None, cx, |s| match original_mode {
                Some(Mode::VisualLine) => {
                    s.move_with(|map, selection| {
                        selection.collapse_to(
                            map.prev_line_boundary(selection.start.to_point(map)).1,
                            SelectionGoal::None,
                        )
                    });
                }
                Some(Mode::VisualBlock) => {
                    let mut first = s.first_anchor();
                    first.collapse_to(first.start, first.goal);
                    s.select_anchors(vec![first]);
                }
                _ => {
                    s.move_with(|_, selection| {
                        selection.collapse_to(selection.start, selection.goal);
                    });
                }
            });
        });
        self.switch_mode(Mode::Normal, true, cx)
    }

    fn local_selections_changed(&mut self, editor: View<Editor>, cx: &mut WindowContext) {
        let newest = editor.read(cx).selections.newest_anchor().clone();
        let is_multicursor = editor.read(cx).selections.count() > 1;

        let state = self.state();
        if state.mode == Mode::Insert && state.current_tx.is_some() {
            if state.current_anchor.is_none() {
                self.update_state(|state| state.current_anchor = Some(newest));
            } else if state.current_anchor.as_ref().unwrap() != &newest {
                if let Some(tx_id) = self.update_state(|state| state.current_tx.take()) {
                    self.update_active_editor(cx, |_, editor, cx| {
                        editor.group_until_transaction(tx_id, cx)
                    });
                }
            }
        } else if state.mode == Mode::Normal && newest.start != newest.end {
            if matches!(newest.goal, SelectionGoal::HorizontalRange { .. }) {
                self.switch_mode(Mode::VisualBlock, false, cx);
            } else {
                self.switch_mode(Mode::Visual, false, cx)
            }
        } else if newest.start == newest.end
            && !is_multicursor
            && [Mode::Visual, Mode::VisualLine, Mode::VisualBlock].contains(&state.mode)
        {
            self.switch_mode(Mode::Normal, true, cx)
        }
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
                    mode: if VimSettings::get_global(cx).use_multiline_find {
                        FindRange::MultiLine
                    } else {
                        FindRange::SingleLine
                    },
                    smartcase: VimSettings::get_global(cx).use_smartcase_find,
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
                    mode: if VimSettings::get_global(cx).use_multiline_find {
                        FindRange::MultiLine
                    } else {
                        FindRange::SingleLine
                    },
                    smartcase: VimSettings::get_global(cx).use_smartcase_find,
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
            Some(Operator::AddSurrounds { target }) => match Vim::read(cx).state().mode {
                Mode::Normal => {
                    if let Some(target) = target {
                        add_surrounds(text, target, cx);
                        Vim::update(cx, |vim, cx| vim.clear_operator(cx));
                    }
                }
                _ => Vim::update(cx, |vim, cx| vim.clear_operator(cx)),
            },
            Some(Operator::ChangeSurrounds { target }) => match Vim::read(cx).state().mode {
                Mode::Normal => {
                    if let Some(target) = target {
                        change_surrounds(text, target, cx);
                        Vim::update(cx, |vim, cx| vim.clear_operator(cx));
                    }
                }
                _ => Vim::update(cx, |vim, cx| vim.clear_operator(cx)),
            },
            Some(Operator::DeleteSurrounds) => match Vim::read(cx).state().mode {
                Mode::Normal => {
                    delete_surrounds(text, cx);
                    Vim::update(cx, |vim, cx| vim.clear_operator(cx));
                }
                _ => Vim::update(cx, |vim, cx| vim.clear_operator(cx)),
            },
            _ => match Vim::read(cx).state().mode {
                Mode::Replace => multi_replace(text, cx),
                _ => {}
            },
        }
    }

    fn set_enabled(&mut self, enabled: bool, cx: &mut AppContext) {
        if self.enabled == enabled {
            return;
        }
        if !enabled {
            CommandPaletteInterceptor::update_global(cx, |interceptor, _| {
                interceptor.clear();
            });
            CommandPaletteFilter::update_global(cx, |filter, _| {
                filter.hide_namespace(Self::NAMESPACE);
            });
            *self = Default::default();
            return;
        }

        self.enabled = true;
        CommandPaletteFilter::update_global(cx, |filter, _| {
            filter.show_namespace(Self::NAMESPACE);
        });
        CommandPaletteInterceptor::update_global(cx, |interceptor, _| {
            interceptor.set(Box::new(command::command_interceptor));
        });

        if let Some(active_window) = cx
            .active_window()
            .and_then(|window| window.downcast::<Workspace>())
        {
            active_window
                .update(cx, |workspace, cx| {
                    let active_editor = workspace.active_item_as::<Editor>(cx);
                    if let Some(active_editor) = active_editor {
                        self.activate_editor(active_editor, cx);
                        self.switch_mode(Mode::Normal, false, cx);
                    }
                })
                .ok();
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

    fn sync_vim_settings(&mut self, cx: &mut WindowContext) {
        self.update_active_editor(cx, |vim, editor, cx| {
            let state = vim.state();
            editor.set_cursor_shape(state.cursor_shape(), cx);
            editor.set_clip_at_line_ends(state.clip_at_line_ends(), cx);
            editor.set_collapse_matches(true);
            editor.set_input_enabled(!state.vim_controlled());
            editor.set_autoindent(state.should_autoindent());
            editor.selections.line_mode = matches!(state.mode, Mode::VisualLine);
            if editor.is_focused(cx) {
                editor.set_keymap_context_layer::<Self>(state.keymap_context_layer(), cx);
            // disables vim if the rename editor is focused,
            // but not if the command palette is open.
            } else if editor.focus_handle(cx).contains_focused(cx) {
                editor.remove_keymap_context_layer::<Self>(cx)
            }
        });
    }

    fn unhook_vim_settings(editor: &mut Editor, cx: &mut ViewContext<Editor>) {
        if editor.mode() == EditorMode::Full {
            editor.set_cursor_shape(CursorShape::Bar, cx);
            editor.set_clip_at_line_ends(false, cx);
            editor.set_collapse_matches(false);
            editor.set_input_enabled(true);
            editor.set_autoindent(true);
            editor.selections.line_mode = false;
        }
        editor.remove_keymap_context_layer::<Self>(cx)
    }
}

impl Settings for VimModeSetting {
    const KEY: Option<&'static str> = Some("vim_mode");

    type FileContent = Option<bool>;

    fn load(sources: SettingsSources<Self::FileContent>, _: &mut AppContext) -> Result<Self> {
        Ok(Self(sources.user.copied().flatten().unwrap_or(
            sources.default.ok_or_else(Self::missing_default)?,
        )))
    }
}

/// Controls when to use system clipboard.
#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum UseSystemClipboard {
    /// Don't use system clipboard.
    Never,
    /// Use system clipboard.
    Always,
    /// Use system clipboard for yank operations.
    OnYank,
}

#[derive(Deserialize)]
struct VimSettings {
    // all vim uses vim clipboard
    // vim always uses system cliupbaord
    // some magic where yy is system and dd is not.
    pub use_system_clipboard: UseSystemClipboard,
    pub use_multiline_find: bool,
    pub use_smartcase_find: bool,
}

#[derive(Clone, Default, Serialize, Deserialize, JsonSchema)]
struct VimSettingsContent {
    pub use_system_clipboard: Option<UseSystemClipboard>,
    pub use_multiline_find: Option<bool>,
    pub use_smartcase_find: Option<bool>,
}

impl Settings for VimSettings {
    const KEY: Option<&'static str> = Some("vim");

    type FileContent = VimSettingsContent;

    fn load(sources: SettingsSources<Self::FileContent>, _: &mut AppContext) -> Result<Self> {
        sources.json_merge()
    }
}
