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
use collections::{CommandPaletteFilter, HashMap};
use command_palette::CommandPaletteInterceptor;
use editor::{movement, Editor, EditorMode, Event};
use gpui::{
    actions, impl_actions, keymap_matcher::KeymapContext, keymap_matcher::MatchResult, Action,
    AppContext, Subscription, ViewContext, ViewHandle, WeakViewHandle, WindowContext,
};
use language::{CursorShape, Point, Selection, SelectionGoal};
pub use mode_indicator::ModeIndicator;
use motion::Motion;
use normal::normal_replace;
use serde::Deserialize;
use settings::{Setting, SettingsStore};
use state::{EditorState, Mode, Operator, RecordedSelection, WorkspaceState};
use std::{ops::Range, sync::Arc};
use visual::{visual_block_motion, visual_replace};
use workspace::{self, Workspace};

use crate::state::ReplayableAction;

struct VimModeSetting(bool);

#[derive(Clone, Deserialize, PartialEq)]
pub struct SwitchMode(pub Mode);

#[derive(Clone, Deserialize, PartialEq)]
pub struct PushOperator(pub Operator);

#[derive(Clone, Deserialize, PartialEq)]
struct Number(usize);

actions!(
    vim,
    [Tab, Enter, Object, InnerObject, FindForward, FindBackward]
);
impl_actions!(vim, [Number, SwitchMode, PushOperator]);

#[derive(Copy, Clone, Debug)]
enum VimEvent {
    ModeChanged { mode: Mode },
}

pub fn init(cx: &mut AppContext) {
    cx.set_global(Vim::default());
    settings::register::<VimModeSetting>(cx);

    editor_events::init(cx);
    normal::init(cx);
    visual::init(cx);
    insert::init(cx);
    object::init(cx);
    motion::init(cx);
    command::init(cx);

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
        Vim::update(cx, |vim, cx| vim.push_count_digit(n.0, cx));
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
    cx.update_global(|vim: &mut Vim, cx: &mut AppContext| {
        vim.set_enabled(settings::get::<VimModeSetting>(cx).0, cx)
    });
    cx.observe_global::<SettingsStore, _>(|cx| {
        cx.update_global(|vim: &mut Vim, cx: &mut AppContext| {
            vim.set_enabled(settings::get::<VimModeSetting>(cx).0, cx)
        });
    })
    .detach();
}

pub fn observe_keystrokes(cx: &mut WindowContext) {
    cx.observe_keystrokes(|_keystroke, result, handled_by, cx| {
        if result == &MatchResult::Pending {
            return true;
        }
        if let Some(handled_by) = handled_by {
            Vim::update(cx, |vim, _| {
                if vim.workspace_state.recording {
                    vim.workspace_state
                        .recorded_actions
                        .push(ReplayableAction::Action(handled_by.boxed_clone()));

                    if vim.workspace_state.stop_recording_after_next_action {
                        vim.workspace_state.recording = false;
                        vim.workspace_state.stop_recording_after_next_action = false;
                    }
                }
            });

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
    enabled: bool,
    editor_states: HashMap<usize, EditorState>,
    workspace_state: WorkspaceState,
    default_state: EditorState,
}

impl Vim {
    fn read(cx: &mut AppContext) -> &Self {
        cx.default_global()
    }

    fn update<F, S>(cx: &mut WindowContext, update: F) -> S
    where
        F: FnOnce(&mut Self, &mut WindowContext) -> S,
    {
        cx.update_global(update)
    }

    fn set_active_editor(&mut self, editor: ViewHandle<Editor>, cx: &mut WindowContext) {
        self.active_editor = Some(editor.clone().downgrade());
        self.editor_subscription = Some(cx.subscribe(&editor, |editor, event, cx| match event {
            Event::SelectionsChanged { local: true } => {
                let editor = editor.read(cx);
                if editor.leader_replica_id().is_none() {
                    let newest = editor.selections.newest::<usize>(cx);
                    local_selections_changed(newest, cx);
                }
            }
            Event::InputIgnored { text } => {
                Vim::active_editor_input_ignored(text.clone(), cx);
                Vim::record_insertion(text, None, cx)
            }
            Event::InputHandled {
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
                // if leader_replica_id is set, then you're following someone else's cursor
                // don't switch vim mode.
                && editor.leader_replica_id().is_none()
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
        let editor = self.active_editor.clone()?.upgrade(cx)?;
        Some(editor.update(cx, update))
    }

    pub fn start_recording(&mut self, cx: &mut WindowContext) {
        if !self.workspace_state.replaying {
            self.workspace_state.recording = true;
            self.workspace_state.recorded_actions = Default::default();
            self.workspace_state.recorded_count = None;

            let selections = self
                .active_editor
                .and_then(|editor| editor.upgrade(cx))
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

    pub fn stop_recording(&mut self) {
        if self.workspace_state.recording {
            self.workspace_state.stop_recording_after_next_action = true;
        }
    }

    pub fn stop_recording_immediately(&mut self, action: Box<dyn Action>) {
        if self.workspace_state.recording {
            self.workspace_state
                .recorded_actions
                .push(ReplayableAction::Action(action.boxed_clone()));
            self.workspace_state.recording = false;
            self.workspace_state.stop_recording_after_next_action = false;
        }
    }

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

        cx.emit_global(VimEvent::ModeChanged { mode });

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

            cx.update_default_global::<CommandPaletteFilter, _, _>(|filter, _| {
                if self.enabled {
                    filter.filtered_namespaces.remove("vim");
                } else {
                    filter.filtered_namespaces.insert("vim");
                }
            });

            if self.enabled {
                cx.set_global::<CommandPaletteInterceptor>(Box::new(command::command_interceptor));
            } else if cx.has_global::<CommandPaletteInterceptor>() {
                let _ = cx.remove_global::<CommandPaletteInterceptor>();
            }

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

    pub fn state(&self) -> &EditorState {
        if let Some(active_editor) = self.active_editor.as_ref() {
            if let Some(state) = self.editor_states.get(&active_editor.id()) {
                return state;
            }
        }

        &self.default_state
    }

    pub fn update_state<T>(&mut self, func: impl FnOnce(&mut EditorState) -> T) -> T {
        let mut state = self.state().clone();
        let ret = func(&mut state);

        if let Some(active_editor) = self.active_editor.as_ref() {
            self.editor_states.insert(active_editor.id(), state);
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

fn local_selections_changed(newest: Selection<usize>, cx: &mut WindowContext) {
    Vim::update(cx, |vim, cx| {
        if vim.enabled && vim.state().mode == Mode::Normal && !newest.is_empty() {
            if matches!(newest.goal, SelectionGoal::ColumnRange { .. }) {
                vim.switch_mode(Mode::VisualBlock, false, cx);
            } else {
                vim.switch_mode(Mode::Visual, false, cx)
            }
        }
    })
}
