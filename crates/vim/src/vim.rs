//! Vim support for Zed.

#[cfg(test)]
mod test;

mod change_list;
mod command;
mod digraph;
mod insert;
mod mode_indicator;
mod motion;
mod normal;
mod object;
mod replace;
mod state;
mod surrounds;
mod visual;

use anyhow::Result;
use collections::HashMap;
use editor::{
    movement::{self, FindRange},
    Anchor, Bias, Editor, EditorEvent, EditorMode, ToPoint,
};
use gpui::{
    actions, impl_actions, Action, AppContext, Entity, EventEmitter, KeyContext, KeystrokeEvent,
    Render, View, ViewContext, WeakView,
};
use insert::NormalBefore;
use language::{CursorShape, Point, Selection, SelectionGoal, TransactionId};
pub use mode_indicator::ModeIndicator;
use motion::Motion;
use normal::search::SearchSubmit;
use schemars::JsonSchema;
use serde::Deserialize;
use serde_derive::Serialize;
use settings::{update_settings_file, Settings, SettingsSources, SettingsStore};
use state::{Mode, Operator, RecordedSelection, SearchState, VimGlobals};
use std::{ops::Range, sync::Arc};
use surrounds::SurroundsType;
use ui::{IntoElement, VisualContext};
use workspace::{self, Pane, Workspace};

use crate::state::ReplayableAction;

/// Whether or not to enable Vim mode.
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

#[derive(Clone, Deserialize, PartialEq)]
struct SelectRegister(String);

actions!(
    vim,
    [
        ClearOperators,
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

impl_actions!(vim, [SwitchMode, PushOperator, Number, SelectRegister]);

/// Initializes the `vim` crate.
pub fn init(cx: &mut AppContext) {
    VimModeSetting::register(cx);
    VimSettings::register(cx);
    VimGlobals::register(cx);

    cx.observe_new_views(|editor: &mut Editor, cx| Vim::register(editor, cx))
        .detach();

    cx.observe_new_views(|workspace: &mut Workspace, _| {
        workspace.register_action(|workspace, _: &ToggleVimMode, cx| {
            let fs = workspace.app_state().fs.clone();
            let currently_enabled = Vim::enabled(cx);
            update_settings_file::<VimModeSetting>(fs, cx, move |setting, _| {
                *setting = Some(!currently_enabled)
            })
        });

        workspace.register_action(|_, _: &OpenDefaultKeymap, cx| {
            cx.emit(workspace::Event::OpenBundledFile {
                text: settings::vim_keymap(),
                title: "Default Vim Bindings",
                language: "JSON",
            });
        });

        workspace.register_action(|workspace, _: &SearchSubmit, cx| {
            let Some(vim) = workspace
                .active_item_as::<Editor>(cx)
                .and_then(|editor| editor.read(cx).addon::<VimAddon>().cloned())
            else {
                return;
            };
            vim.view
                .update(cx, |_, cx| cx.defer(|vim, cx| vim.search_submit(cx)))
        });
    })
    .detach();
}

#[derive(Clone)]
pub(crate) struct VimAddon {
    pub(crate) view: View<Vim>,
}

impl editor::Addon for VimAddon {
    fn extend_key_context(&self, key_context: &mut KeyContext, cx: &AppContext) {
        self.view.read(cx).extend_key_context(key_context)
    }

    fn to_any(&self) -> &dyn std::any::Any {
        self
    }
}

/// The state pertaining to Vim mode.
pub(crate) struct Vim {
    pub(crate) mode: Mode,
    pub last_mode: Mode,

    /// pre_count is the number before an operator is specified (3 in 3d2d)
    pre_count: Option<usize>,
    /// post_count is the number after an operator is specified (2 in 3d2d)
    post_count: Option<usize>,

    operator_stack: Vec<Operator>,
    pub(crate) replacements: Vec<(Range<editor::Anchor>, String)>,

    pub(crate) marks: HashMap<String, Vec<Anchor>>,
    pub(crate) stored_visual_mode: Option<(Mode, Vec<bool>)>,
    pub(crate) change_list: Vec<Vec<Anchor>>,
    pub(crate) change_list_position: Option<usize>,

    pub(crate) current_tx: Option<TransactionId>,
    pub(crate) current_anchor: Option<Selection<Anchor>>,
    pub(crate) undo_modes: HashMap<TransactionId, Mode>,

    selected_register: Option<char>,
    pub search: SearchState,

    editor: WeakView<Editor>,
}

// Hack: Vim intercepts events dispatched to a window and updates the view in response.
// This means it needs a VisualContext. The easiest way to satisfy that constraint is
// to make Vim a "View" that is just never actually rendered.
impl Render for Vim {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        gpui::Empty
    }
}

enum VimEvent {
    Focused,
}
impl EventEmitter<VimEvent> for Vim {}

impl Vim {
    /// The namespace for Vim actions.
    const NAMESPACE: &'static str = "vim";

    pub fn new(cx: &mut ViewContext<Editor>) -> View<Self> {
        let editor = cx.view().clone();

        cx.new_view(|cx: &mut ViewContext<Vim>| {
            cx.subscribe(&editor, |vim, _, event, cx| {
                vim.handle_editor_event(event, cx)
            })
            .detach();

            let listener = cx.listener(Vim::observe_keystrokes);
            cx.observe_keystrokes(listener).detach();

            Vim {
                mode: Mode::Normal,
                last_mode: Mode::Normal,
                pre_count: None,
                post_count: None,
                operator_stack: Vec::new(),
                replacements: Vec::new(),

                marks: HashMap::default(),
                stored_visual_mode: None,
                change_list: Vec::new(),
                change_list_position: None,
                current_tx: None,
                current_anchor: None,
                undo_modes: HashMap::default(),

                selected_register: None,
                search: SearchState::default(),

                editor: editor.downgrade(),
            }
        })
    }

    fn register(editor: &mut Editor, cx: &mut ViewContext<Editor>) {
        if !editor.use_modal_editing() {
            return;
        }

        let mut was_enabled = Vim::enabled(cx);
        let mut was_toggle = VimSettings::get_global(cx).toggle_relative_line_numbers;
        cx.observe_global::<SettingsStore>(move |editor, cx| {
            let enabled = Vim::enabled(cx);
            let toggle = VimSettings::get_global(cx).toggle_relative_line_numbers;
            if enabled && was_enabled && (toggle != was_toggle) {
                if toggle {
                    let is_relative = editor
                        .addon::<VimAddon>()
                        .map(|vim| vim.view.read(cx).mode != Mode::Insert);
                    editor.set_relative_line_number(is_relative, cx)
                } else {
                    editor.set_relative_line_number(None, cx)
                }
            }
            was_toggle = VimSettings::get_global(cx).toggle_relative_line_numbers;
            if was_enabled == enabled {
                return;
            }
            was_enabled = enabled;
            if enabled {
                Self::activate(editor, cx)
            } else {
                Self::deactivate(editor, cx)
            }
        })
        .detach();
        if was_enabled {
            Self::activate(editor, cx)
        }
    }

    fn activate(editor: &mut Editor, cx: &mut ViewContext<Editor>) {
        let vim = Vim::new(cx);

        editor.register_addon(VimAddon { view: vim.clone() });

        vim.update(cx, |_, cx| {
            Vim::action(editor, cx, |vim, action: &SwitchMode, cx| {
                vim.switch_mode(action.0, false, cx)
            });

            Vim::action(editor, cx, |vim, action: &PushOperator, cx| {
                vim.push_operator(action.0.clone(), cx)
            });

            Vim::action(editor, cx, |vim, _: &ClearOperators, cx| {
                vim.clear_operator(cx)
            });
            Vim::action(editor, cx, |vim, n: &Number, cx| {
                vim.push_count_digit(n.0, cx);
            });
            Vim::action(editor, cx, |vim, _: &Tab, cx| {
                vim.input_ignored(" ".into(), cx)
            });
            Vim::action(editor, cx, |vim, _: &Enter, cx| {
                vim.input_ignored("\n".into(), cx)
            });

            normal::register(editor, cx);
            insert::register(editor, cx);
            motion::register(editor, cx);
            command::register(editor, cx);
            replace::register(editor, cx);
            object::register(editor, cx);
            visual::register(editor, cx);
            change_list::register(editor, cx);

            cx.defer(|vim, cx| {
                vim.focused(false, cx);
            })
        })
    }

    fn deactivate(editor: &mut Editor, cx: &mut ViewContext<Editor>) {
        editor.set_cursor_shape(CursorShape::Bar, cx);
        editor.set_clip_at_line_ends(false, cx);
        editor.set_collapse_matches(false);
        editor.set_input_enabled(true);
        editor.set_autoindent(true);
        editor.selections.line_mode = false;
        editor.unregister_addon::<VimAddon>();
        editor.set_relative_line_number(None, cx);
        if let Some(vim) = Vim::globals(cx).focused_vim() {
            if vim.entity_id() == cx.view().entity_id() {
                Vim::globals(cx).focused_vim = None;
            }
        }
    }

    /// Register an action on the editor.
    pub fn action<A: Action>(
        editor: &mut Editor,
        cx: &mut ViewContext<Vim>,
        f: impl Fn(&mut Vim, &A, &mut ViewContext<Vim>) + 'static,
    ) {
        let subscription = editor.register_action(cx.listener(f));
        cx.on_release(|_, _, _| drop(subscription)).detach();
    }

    pub fn editor(&self) -> Option<View<Editor>> {
        self.editor.upgrade()
    }

    pub fn workspace(&self, cx: &ViewContext<Self>) -> Option<View<Workspace>> {
        self.editor().and_then(|editor| editor.read(cx).workspace())
    }

    pub fn pane(&self, cx: &ViewContext<Self>) -> Option<View<Pane>> {
        self.workspace(cx)
            .and_then(|workspace| workspace.read(cx).pane_for(&self.editor()?))
    }

    pub fn enabled(cx: &mut AppContext) -> bool {
        VimModeSetting::get_global(cx).0
    }

    /// Called whenever an keystroke is typed so vim can observe all actions
    /// and keystrokes accordingly.
    fn observe_keystrokes(&mut self, keystroke_event: &KeystrokeEvent, cx: &mut ViewContext<Self>) {
        if let Some(action) = keystroke_event.action.as_ref() {
            // Keystroke is handled by the vim system, so continue forward
            if action.name().starts_with("vim::") {
                return;
            }
        } else if cx.has_pending_keystrokes() || keystroke_event.keystroke.is_ime_in_progress() {
            return;
        }

        if let Some(operator) = self.active_operator() {
            if !operator.is_waiting(self.mode) {
                self.clear_operator(cx);
                self.stop_recording_immediately(Box::new(ClearOperators), cx)
            }
        }
    }

    fn handle_editor_event(&mut self, event: &EditorEvent, cx: &mut ViewContext<Self>) {
        match event {
            EditorEvent::Focused => self.focused(true, cx),
            EditorEvent::Blurred => self.blurred(cx),
            EditorEvent::SelectionsChanged { local: true } => {
                self.local_selections_changed(cx);
            }
            EditorEvent::InputIgnored { text } => {
                self.input_ignored(text.clone(), cx);
                Vim::globals(cx).observe_insertion(text, None)
            }
            EditorEvent::InputHandled {
                text,
                utf16_range_to_replace: range_to_replace,
            } => Vim::globals(cx).observe_insertion(text, range_to_replace.clone()),
            EditorEvent::TransactionBegun { transaction_id } => {
                self.transaction_begun(*transaction_id, cx)
            }
            EditorEvent::TransactionUndone { transaction_id } => {
                self.transaction_undone(transaction_id, cx)
            }
            EditorEvent::Edited { .. } => self.push_to_change_list(cx),
            EditorEvent::FocusedIn => self.sync_vim_settings(cx),
            _ => {}
        }
    }

    fn push_operator(&mut self, operator: Operator, cx: &mut ViewContext<Self>) {
        if matches!(
            operator,
            Operator::Change
                | Operator::Delete
                | Operator::Replace
                | Operator::Indent
                | Operator::Outdent
                | Operator::Lowercase
                | Operator::Uppercase
                | Operator::OppositeCase
                | Operator::ToggleComments
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
            self.operator_stack.clear();
            if let Operator::AddSurrounds { target: None } = operator {
                self.start_recording(cx);
            }
        };
        self.operator_stack.push(operator);
        self.sync_vim_settings(cx);
    }

    pub fn switch_mode(&mut self, mode: Mode, leave_selections: bool, cx: &mut ViewContext<Self>) {
        let last_mode = self.mode;
        let prior_mode = self.last_mode;
        let prior_tx = self.current_tx;
        self.last_mode = last_mode;
        self.mode = mode;
        self.operator_stack.clear();
        self.selected_register.take();
        if mode == Mode::Normal || mode != last_mode {
            self.current_tx.take();
            self.current_anchor.take();
        }
        if mode != Mode::Insert && mode != Mode::Replace {
            self.take_count(cx);
        }

        // Sync editor settings like clip mode
        self.sync_vim_settings(cx);

        if VimSettings::get_global(cx).toggle_relative_line_numbers {
            if self.mode != self.last_mode {
                if self.mode == Mode::Insert || self.last_mode == Mode::Insert {
                    self.update_editor(cx, |vim, editor, cx| {
                        let is_relative = vim.mode != Mode::Insert;
                        editor.set_relative_line_number(Some(is_relative), cx)
                    });
                }
            }
        }

        if leave_selections {
            return;
        }

        if !mode.is_visual() && last_mode.is_visual() {
            self.create_visual_marks(last_mode, cx);
        }

        // Adjust selections
        self.update_editor(cx, |vim, editor, cx| {
            if last_mode != Mode::VisualBlock && last_mode.is_visual() && mode == Mode::VisualBlock
            {
                vim.visual_block_motion(true, editor, cx, |_, point, goal| Some((point, goal)))
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
                    }
                });
            })
        });
    }

    fn take_count(&mut self, cx: &mut ViewContext<Self>) -> Option<usize> {
        let global_state = cx.global_mut::<VimGlobals>();
        if global_state.dot_replaying {
            return global_state.recorded_count;
        }

        let count = if self.post_count == None && self.pre_count == None {
            return None;
        } else {
            Some(self.post_count.take().unwrap_or(1) * self.pre_count.take().unwrap_or(1))
        };

        if global_state.dot_recording {
            global_state.recorded_count = count;
        }
        self.sync_vim_settings(cx);
        count
    }

    pub fn cursor_shape(&self) -> CursorShape {
        match self.mode {
            Mode::Normal => {
                if self.operator_stack.is_empty() {
                    CursorShape::Block
                } else {
                    CursorShape::Underscore
                }
            }
            Mode::Replace => CursorShape::Underscore,
            Mode::Visual | Mode::VisualLine | Mode::VisualBlock => CursorShape::Block,
            Mode::Insert => CursorShape::Bar,
        }
    }

    pub fn editor_input_enabled(&self) -> bool {
        match self.mode {
            Mode::Insert => {
                if let Some(operator) = self.operator_stack.last() {
                    !operator.is_waiting(self.mode)
                } else {
                    true
                }
            }
            Mode::Normal | Mode::Replace | Mode::Visual | Mode::VisualLine | Mode::VisualBlock => {
                false
            }
        }
    }

    pub fn should_autoindent(&self) -> bool {
        !(self.mode == Mode::Insert && self.last_mode == Mode::VisualBlock)
    }

    pub fn clip_at_line_ends(&self) -> bool {
        match self.mode {
            Mode::Insert | Mode::Visual | Mode::VisualLine | Mode::VisualBlock | Mode::Replace => {
                false
            }
            Mode::Normal => true,
        }
    }

    pub fn extend_key_context(&self, context: &mut KeyContext) {
        let mut mode = match self.mode {
            Mode::Normal => "normal",
            Mode::Visual | Mode::VisualLine | Mode::VisualBlock => "visual",
            Mode::Insert => "insert",
            Mode::Replace => "replace",
        }
        .to_string();

        let mut operator_id = "none";

        let active_operator = self.active_operator();
        if active_operator.is_none() && self.pre_count.is_some()
            || active_operator.is_some() && self.post_count.is_some()
        {
            context.add("VimCount");
        }

        if let Some(active_operator) = active_operator {
            if active_operator.is_waiting(self.mode) {
                mode = "waiting".to_string();
            } else {
                mode = "operator".to_string();
                operator_id = active_operator.id();
            }
        }

        if mode != "waiting" && mode != "insert" && mode != "replace" {
            context.add("VimControl");
        }
        context.set("vim_mode", mode);
        context.set("vim_operator", operator_id);
    }

    fn focused(&mut self, preserve_selection: bool, cx: &mut ViewContext<Self>) {
        let Some(editor) = self.editor() else {
            return;
        };
        let editor = editor.read(cx);
        let editor_mode = editor.mode();
        let newest_selection_empty = editor.selections.newest::<usize>(cx).is_empty();

        if editor_mode == EditorMode::Full
                && !newest_selection_empty
                && self.mode == Mode::Normal
                // When following someone, don't switch vim mode.
                && editor.leader_peer_id().is_none()
        {
            if preserve_selection {
                self.switch_mode(Mode::Visual, true, cx);
            } else {
                self.update_editor(cx, |_, editor, cx| {
                    editor.set_clip_at_line_ends(false, cx);
                    editor.change_selections(None, cx, |s| {
                        s.move_with(|_, selection| {
                            selection.collapse_to(selection.start, selection.goal)
                        })
                    });
                });
            }
        }

        cx.emit(VimEvent::Focused);
        self.sync_vim_settings(cx);

        if VimSettings::get_global(cx).toggle_relative_line_numbers {
            if let Some(old_vim) = Vim::globals(cx).focused_vim() {
                if old_vim.entity_id() != cx.view().entity_id() {
                    old_vim.update(cx, |vim, cx| {
                        vim.update_editor(cx, |_, editor, cx| {
                            editor.set_relative_line_number(None, cx)
                        });
                    });

                    self.update_editor(cx, |vim, editor, cx| {
                        let is_relative = vim.mode != Mode::Insert;
                        editor.set_relative_line_number(Some(is_relative), cx)
                    });
                }
            } else {
                self.update_editor(cx, |vim, editor, cx| {
                    let is_relative = vim.mode != Mode::Insert;
                    editor.set_relative_line_number(Some(is_relative), cx)
                });
            }
        }
        Vim::globals(cx).focused_vim = Some(cx.view().downgrade());
    }

    fn blurred(&mut self, cx: &mut ViewContext<Self>) {
        self.stop_recording_immediately(NormalBefore.boxed_clone(), cx);
        self.store_visual_marks(cx);
        self.clear_operator(cx);
        self.update_editor(cx, |_, editor, cx| {
            editor.set_cursor_shape(language::CursorShape::Hollow, cx);
        });
    }

    fn update_editor<S>(
        &mut self,
        cx: &mut ViewContext<Self>,
        update: impl FnOnce(&mut Self, &mut Editor, &mut ViewContext<Editor>) -> S,
    ) -> Option<S> {
        let editor = self.editor.upgrade()?;
        Some(editor.update(cx, |editor, cx| update(self, editor, cx)))
    }

    fn editor_selections(&mut self, cx: &mut ViewContext<Self>) -> Vec<Range<Anchor>> {
        self.update_editor(cx, |_, editor, _| {
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
    pub fn start_recording(&mut self, cx: &mut ViewContext<Self>) {
        Vim::update_globals(cx, |globals, cx| {
            if !globals.dot_replaying {
                globals.dot_recording = true;
                globals.recorded_actions = Default::default();
                globals.recorded_count = None;

                let selections = self.editor().map(|editor| {
                    let editor = editor.read(cx);
                    (
                        editor.selections.oldest::<Point>(cx),
                        editor.selections.newest::<Point>(cx),
                    )
                });

                if let Some((oldest, newest)) = selections {
                    globals.recorded_selection = match self.mode {
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
                    globals.recorded_selection = RecordedSelection::None;
                }
            }
        })
    }

    pub fn stop_replaying(&mut self, cx: &mut ViewContext<Self>) {
        let globals = Vim::globals(cx);
        globals.dot_replaying = false;
        if let Some(replayer) = globals.replayer.take() {
            replayer.stop();
        }
    }

    /// When finishing an action that modifies the buffer, stop recording.
    /// as you usually call this within a keystroke handler we also ensure that
    /// the current action is recorded.
    pub fn stop_recording(&mut self, cx: &mut ViewContext<Self>) {
        let globals = Vim::globals(cx);
        if globals.dot_recording {
            globals.stop_recording_after_next_action = true;
        }
    }

    /// Stops recording actions immediately rather than waiting until after the
    /// next action to stop recording.
    ///
    /// This doesn't include the current action.
    pub fn stop_recording_immediately(
        &mut self,
        action: Box<dyn Action>,
        cx: &mut ViewContext<Self>,
    ) {
        let globals = Vim::globals(cx);
        if globals.dot_recording {
            globals
                .recorded_actions
                .push(ReplayableAction::Action(action.boxed_clone()));
            globals.dot_recording = false;
            globals.stop_recording_after_next_action = false;
        }
    }

    /// Explicitly record one action (equivalents to start_recording and stop_recording)
    pub fn record_current_action(&mut self, cx: &mut ViewContext<Self>) {
        self.start_recording(cx);
        self.stop_recording(cx);
    }

    fn push_count_digit(&mut self, number: usize, cx: &mut ViewContext<Self>) {
        if self.active_operator().is_some() {
            let post_count = self.post_count.unwrap_or(0);

            self.post_count = Some(
                post_count
                    .checked_mul(10)
                    .and_then(|post_count| post_count.checked_add(number))
                    .unwrap_or(post_count),
            )
        } else {
            let pre_count = self.pre_count.unwrap_or(0);

            self.pre_count = Some(
                pre_count
                    .checked_mul(10)
                    .and_then(|pre_count| pre_count.checked_add(number))
                    .unwrap_or(pre_count),
            )
        }
        // update the keymap so that 0 works
        self.sync_vim_settings(cx)
    }

    fn select_register(&mut self, register: Arc<str>, cx: &mut ViewContext<Self>) {
        if register.chars().count() == 1 {
            self.selected_register
                .replace(register.chars().next().unwrap());
        }
        self.operator_stack.clear();
        self.sync_vim_settings(cx);
    }

    fn maybe_pop_operator(&mut self) -> Option<Operator> {
        self.operator_stack.pop()
    }

    fn pop_operator(&mut self, cx: &mut ViewContext<Self>) -> Operator {
        let popped_operator = self.operator_stack.pop()
            .expect("Operator popped when no operator was on the stack. This likely means there is an invalid keymap config");
        self.sync_vim_settings(cx);
        popped_operator
    }

    fn clear_operator(&mut self, cx: &mut ViewContext<Self>) {
        self.take_count(cx);
        self.selected_register.take();
        self.operator_stack.clear();
        self.sync_vim_settings(cx);
    }

    fn active_operator(&self) -> Option<Operator> {
        self.operator_stack.last().cloned()
    }

    fn transaction_begun(&mut self, transaction_id: TransactionId, _: &mut ViewContext<Self>) {
        let mode = if (self.mode == Mode::Insert
            || self.mode == Mode::Replace
            || self.mode == Mode::Normal)
            && self.current_tx.is_none()
        {
            self.current_tx = Some(transaction_id);
            self.last_mode
        } else {
            self.mode
        };
        if mode == Mode::VisualLine || mode == Mode::VisualBlock {
            self.undo_modes.insert(transaction_id, mode);
        }
    }

    fn transaction_undone(&mut self, transaction_id: &TransactionId, cx: &mut ViewContext<Self>) {
        match self.mode {
            Mode::VisualLine | Mode::VisualBlock | Mode::Visual => {
                self.update_editor(cx, |vim, editor, cx| {
                    let original_mode = vim.undo_modes.get(transaction_id);
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
                            s.move_with(|map, selection| {
                                selection.collapse_to(
                                    map.clip_at_line_end(selection.start),
                                    selection.goal,
                                );
                            });
                        }
                    });
                });
                self.switch_mode(Mode::Normal, true, cx)
            }
            Mode::Normal => {
                self.update_editor(cx, |_, editor, cx| {
                    editor.change_selections(None, cx, |s| {
                        s.move_with(|map, selection| {
                            selection
                                .collapse_to(map.clip_at_line_end(selection.end), selection.goal)
                        })
                    })
                });
            }
            Mode::Insert | Mode::Replace => {}
        }
    }

    fn local_selections_changed(&mut self, cx: &mut ViewContext<Self>) {
        let Some(editor) = self.editor() else { return };

        if editor.read(cx).leader_peer_id().is_some() {
            return;
        }

        let newest = editor.read(cx).selections.newest_anchor().clone();
        let is_multicursor = editor.read(cx).selections.count() > 1;
        if self.mode == Mode::Insert && self.current_tx.is_some() {
            if self.current_anchor.is_none() {
                self.current_anchor = Some(newest);
            } else if self.current_anchor.as_ref().unwrap() != &newest {
                if let Some(tx_id) = self.current_tx.take() {
                    self.update_editor(cx, |_, editor, cx| {
                        editor.group_until_transaction(tx_id, cx)
                    });
                }
            }
        } else if self.mode == Mode::Normal && newest.start != newest.end {
            if matches!(newest.goal, SelectionGoal::HorizontalRange { .. }) {
                self.switch_mode(Mode::VisualBlock, false, cx);
            } else {
                self.switch_mode(Mode::Visual, false, cx)
            }
        } else if newest.start == newest.end
            && !is_multicursor
            && [Mode::Visual, Mode::VisualLine, Mode::VisualBlock].contains(&self.mode)
        {
            self.switch_mode(Mode::Normal, true, cx);
        }
    }

    fn input_ignored(&mut self, text: Arc<str>, cx: &mut ViewContext<Self>) {
        if text.is_empty() {
            return;
        }

        match self.active_operator() {
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
                Vim::globals(cx).last_find = Some(find.clone());
                self.motion(find, cx)
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
                Vim::globals(cx).last_find = Some(find.clone());
                self.motion(find, cx)
            }
            Some(Operator::Replace) => match self.mode {
                Mode::Normal => self.normal_replace(text, cx),
                Mode::Visual | Mode::VisualLine | Mode::VisualBlock => {
                    self.visual_replace(text, cx)
                }
                _ => self.clear_operator(cx),
            },
            Some(Operator::Digraph { first_char }) => {
                if let Some(first_char) = first_char {
                    if let Some(second_char) = text.chars().next() {
                        self.insert_digraph(first_char, second_char, cx);
                    }
                } else {
                    let first_char = text.chars().next();
                    self.pop_operator(cx);
                    self.push_operator(Operator::Digraph { first_char }, cx);
                }
            }
            Some(Operator::AddSurrounds { target }) => match self.mode {
                Mode::Normal => {
                    if let Some(target) = target {
                        self.add_surrounds(text, target, cx);
                        self.clear_operator(cx);
                    }
                }
                Mode::Visual | Mode::VisualLine | Mode::VisualBlock => {
                    self.add_surrounds(text, SurroundsType::Selection, cx);
                    self.clear_operator(cx);
                }
                _ => self.clear_operator(cx),
            },
            Some(Operator::ChangeSurrounds { target }) => match self.mode {
                Mode::Normal => {
                    if let Some(target) = target {
                        self.change_surrounds(text, target, cx);
                        self.clear_operator(cx);
                    }
                }
                _ => self.clear_operator(cx),
            },
            Some(Operator::DeleteSurrounds) => match self.mode {
                Mode::Normal => {
                    self.delete_surrounds(text, cx);
                    self.clear_operator(cx);
                }
                _ => self.clear_operator(cx),
            },
            Some(Operator::Mark) => self.create_mark(text, false, cx),
            Some(Operator::RecordRegister) => {
                self.record_register(text.chars().next().unwrap(), cx)
            }
            Some(Operator::ReplayRegister) => {
                self.replay_register(text.chars().next().unwrap(), cx)
            }
            Some(Operator::Register) => match self.mode {
                Mode::Insert => {
                    self.update_editor(cx, |_, editor, cx| {
                        if let Some(register) = Vim::update_globals(cx, |globals, cx| {
                            globals.read_register(text.chars().next(), Some(editor), cx)
                        }) {
                            editor.do_paste(
                                &register.text.to_string(),
                                register.clipboard_selections.clone(),
                                false,
                                cx,
                            )
                        }
                    });
                    self.clear_operator(cx);
                }
                _ => {
                    self.select_register(text, cx);
                }
            },
            Some(Operator::Jump { line }) => self.jump(text, line, cx),
            _ => match self.mode {
                Mode::Replace => self.multi_replace(text, cx),
                _ => {}
            },
        }
    }

    fn sync_vim_settings(&mut self, cx: &mut ViewContext<Self>) {
        self.update_editor(cx, |vim, editor, cx| {
            editor.set_cursor_shape(vim.cursor_shape(), cx);
            editor.set_clip_at_line_ends(vim.clip_at_line_ends(), cx);
            editor.set_collapse_matches(true);
            editor.set_input_enabled(vim.editor_input_enabled());
            editor.set_autoindent(vim.should_autoindent());
            editor.selections.line_mode = matches!(vim.mode, Mode::VisualLine);
            editor.set_inline_completions_enabled(matches!(vim.mode, Mode::Insert | Mode::Replace));
        });
        cx.notify()
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
    pub toggle_relative_line_numbers: bool,
    pub use_system_clipboard: UseSystemClipboard,
    pub use_multiline_find: bool,
    pub use_smartcase_find: bool,
    pub custom_digraphs: HashMap<String, Arc<str>>,
}

#[derive(Clone, Default, Serialize, Deserialize, JsonSchema)]
struct VimSettingsContent {
    pub toggle_relative_line_numbers: Option<bool>,
    pub use_system_clipboard: Option<UseSystemClipboard>,
    pub use_multiline_find: Option<bool>,
    pub use_smartcase_find: Option<bool>,
    pub custom_digraphs: Option<HashMap<String, Arc<str>>>,
}

impl Settings for VimSettings {
    const KEY: Option<&'static str> = Some("vim");

    type FileContent = VimSettingsContent;

    fn load(sources: SettingsSources<Self::FileContent>, _: &mut AppContext) -> Result<Self> {
        sources.json_merge()
    }
}
