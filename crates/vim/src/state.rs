use std::{fmt::Display, ops::Range, sync::Arc};

use crate::normal::repeat::Replayer;
use crate::surrounds::SurroundsType;
use crate::{motion::Motion, object::Object};
use collections::HashMap;
use editor::{Anchor, ClipboardSelection};
use gpui::{Action, ClipboardEntry, ClipboardItem, KeyContext};
use language::{CursorShape, Selection, TransactionId};
use serde::{Deserialize, Serialize};
use ui::SharedString;
use workspace::searchable::Direction;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum Mode {
    Normal,
    Insert,
    Replace,
    Visual,
    VisualLine,
    VisualBlock,
}

impl Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Mode::Normal => write!(f, "NORMAL"),
            Mode::Insert => write!(f, "INSERT"),
            Mode::Replace => write!(f, "REPLACE"),
            Mode::Visual => write!(f, "VISUAL"),
            Mode::VisualLine => write!(f, "VISUAL LINE"),
            Mode::VisualBlock => write!(f, "VISUAL BLOCK"),
        }
    }
}

impl Mode {
    pub fn is_visual(&self) -> bool {
        match self {
            Mode::Normal | Mode::Insert | Mode::Replace => false,
            Mode::Visual | Mode::VisualLine | Mode::VisualBlock => true,
        }
    }
}

impl Default for Mode {
    fn default() -> Self {
        Self::Normal
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
pub enum Operator {
    Change,
    Delete,
    Yank,
    Replace,
    Object { around: bool },
    FindForward { before: bool },
    FindBackward { after: bool },
    AddSurrounds { target: Option<SurroundsType> },
    ChangeSurrounds { target: Option<Object> },
    DeleteSurrounds,
    Mark,
    Jump { line: bool },
    Indent,
    Outdent,
    Lowercase,
    Uppercase,
    OppositeCase,
    Digraph { first_char: Option<char> },
    Register,
    RecordRegister,
    ReplayRegister,
    ToggleComments,
}

#[derive(Default, Clone)]
pub struct EditorState {
    pub mode: Mode,
    pub last_mode: Mode,

    /// pre_count is the number before an operator is specified (3 in 3d2d)
    pub pre_count: Option<usize>,
    /// post_count is the number after an operator is specified (2 in 3d2d)
    pub post_count: Option<usize>,

    pub operator_stack: Vec<Operator>,
    pub replacements: Vec<(Range<editor::Anchor>, String)>,

    pub marks: HashMap<String, Vec<Anchor>>,
    pub stored_visual_mode: Option<(Mode, Vec<bool>)>,
    pub change_list: Vec<Vec<Anchor>>,
    pub change_list_position: Option<usize>,

    pub current_tx: Option<TransactionId>,
    pub current_anchor: Option<Selection<Anchor>>,
    pub undo_modes: HashMap<TransactionId, Mode>,

    pub selected_register: Option<char>,
    pub search: SearchState,
}

#[derive(Default, Clone, Debug)]
pub enum RecordedSelection {
    #[default]
    None,
    Visual {
        rows: u32,
        cols: u32,
    },
    SingleLine {
        cols: u32,
    },
    VisualBlock {
        rows: u32,
        cols: u32,
    },
    VisualLine {
        rows: u32,
    },
}

#[derive(Default, Clone, Debug)]
pub struct Register {
    pub(crate) text: SharedString,
    pub(crate) clipboard_selections: Option<Vec<ClipboardSelection>>,
}

impl From<Register> for ClipboardItem {
    fn from(register: Register) -> Self {
        if let Some(clipboard_selections) = register.clipboard_selections {
            ClipboardItem::new_string_with_json_metadata(register.text.into(), clipboard_selections)
        } else {
            ClipboardItem::new_string(register.text.into())
        }
    }
}

impl From<ClipboardItem> for Register {
    fn from(item: ClipboardItem) -> Self {
        // For now, we don't store metadata for multiple entries.
        match item.entries().first() {
            Some(ClipboardEntry::String(value)) if item.entries().len() == 1 => Register {
                text: value.text().to_owned().into(),
                clipboard_selections: value.metadata_json::<Vec<ClipboardSelection>>(),
            },
            // For now, registers can't store images. This could change in the future.
            _ => Register::default(),
        }
    }
}

impl From<String> for Register {
    fn from(text: String) -> Self {
        Register {
            text: text.into(),
            clipboard_selections: None,
        }
    }
}

#[derive(Default, Clone)]
pub struct WorkspaceState {
    pub last_find: Option<Motion>,

    pub dot_recording: bool,
    pub dot_replaying: bool,

    pub stop_recording_after_next_action: bool,
    pub ignore_current_insertion: bool,
    pub recorded_count: Option<usize>,
    pub recorded_actions: Vec<ReplayableAction>,
    pub recorded_selection: RecordedSelection,

    pub recording_register: Option<char>,
    pub last_recorded_register: Option<char>,
    pub last_replayed_register: Option<char>,
    pub replayer: Option<Replayer>,

    pub last_yank: Option<SharedString>,
    pub registers: HashMap<char, Register>,
    pub recordings: HashMap<char, Vec<ReplayableAction>>,
}

#[derive(Debug)]
pub enum ReplayableAction {
    Action(Box<dyn Action>),
    Insertion {
        text: Arc<str>,
        utf16_range_to_replace: Option<Range<isize>>,
    },
}

impl Clone for ReplayableAction {
    fn clone(&self) -> Self {
        match self {
            Self::Action(action) => Self::Action(action.boxed_clone()),
            Self::Insertion {
                text,
                utf16_range_to_replace,
            } => Self::Insertion {
                text: text.clone(),
                utf16_range_to_replace: utf16_range_to_replace.clone(),
            },
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct SearchState {
    pub direction: Direction,
    pub count: usize,
    pub initial_query: String,

    pub prior_selections: Vec<Range<Anchor>>,
    pub prior_operator: Option<Operator>,
    pub prior_mode: Mode,
}

impl EditorState {
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

    pub fn active_operator(&self) -> Option<Operator> {
        self.operator_stack.last().cloned()
    }

    pub fn keymap_context_layer(&self) -> KeyContext {
        let mut context = KeyContext::new_with_defaults();

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

        context
    }
}

impl Operator {
    pub fn id(&self) -> &'static str {
        match self {
            Operator::Object { around: false } => "i",
            Operator::Object { around: true } => "a",
            Operator::Change => "c",
            Operator::Delete => "d",
            Operator::Yank => "y",
            Operator::Replace => "r",
            Operator::Digraph { .. } => "^K",
            Operator::FindForward { before: false } => "f",
            Operator::FindForward { before: true } => "t",
            Operator::FindBackward { after: false } => "F",
            Operator::FindBackward { after: true } => "T",
            Operator::AddSurrounds { .. } => "ys",
            Operator::ChangeSurrounds { .. } => "cs",
            Operator::DeleteSurrounds => "ds",
            Operator::Mark => "m",
            Operator::Jump { line: true } => "'",
            Operator::Jump { line: false } => "`",
            Operator::Indent => ">",
            Operator::Outdent => "<",
            Operator::Uppercase => "gU",
            Operator::Lowercase => "gu",
            Operator::OppositeCase => "g~",
            Operator::Register => "\"",
            Operator::RecordRegister => "q",
            Operator::ReplayRegister => "@",
            Operator::ToggleComments => "gc",
        }
    }

    pub fn is_waiting(&self, mode: Mode) -> bool {
        match self {
            Operator::AddSurrounds { target } => target.is_some() || mode.is_visual(),
            Operator::FindForward { .. }
            | Operator::Mark
            | Operator::Jump { .. }
            | Operator::FindBackward { .. }
            | Operator::Register
            | Operator::RecordRegister
            | Operator::ReplayRegister
            | Operator::Replace
            | Operator::Digraph { .. }
            | Operator::ChangeSurrounds { target: Some(_) }
            | Operator::DeleteSurrounds => true,
            Operator::Change
            | Operator::Delete
            | Operator::Yank
            | Operator::Indent
            | Operator::Outdent
            | Operator::Lowercase
            | Operator::Uppercase
            | Operator::Object { .. }
            | Operator::ChangeSurrounds { target: None }
            | Operator::OppositeCase
            | Operator::ToggleComments => false,
        }
    }
}
