use std::{ops::Range, sync::Arc};

use collections::HashMap;
use gpui::{Action, KeyContext};
use language::CursorShape;
use serde::{Deserialize, Serialize};
use workspace::searchable::Direction;

use crate::motion::Motion;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum Mode {
    Normal,
    Insert,
    Visual,
    VisualLine,
    VisualBlock,
}

impl Mode {
    pub fn is_visual(&self) -> bool {
        match self {
            Mode::Normal | Mode::Insert => false,
            Mode::Visual | Mode::VisualLine | Mode::VisualBlock => true,
        }
    }
}

impl Default for Mode {
    fn default() -> Self {
        Self::Normal
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Deserialize)]
pub enum Operator {
    Change,
    Delete,
    Yank,
    Replace,
    Object { around: bool },
    FindForward { before: bool },
    FindBackward { after: bool },
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

#[derive(Default, Clone)]
pub struct WorkspaceState {
    pub search: SearchState,
    pub last_find: Option<Motion>,

    pub recording: bool,
    pub stop_recording_after_next_action: bool,
    pub replaying: bool,
    pub recorded_count: Option<usize>,
    pub recorded_actions: Vec<ReplayableAction>,
    pub recorded_selection: RecordedSelection,

    pub registers: HashMap<String, String>,
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

#[derive(Clone)]
pub struct SearchState {
    pub direction: Direction,
    pub count: usize,
    pub initial_query: String,
}

impl Default for SearchState {
    fn default() -> Self {
        Self {
            direction: Direction::Next,
            count: 1,
            initial_query: "".to_string(),
        }
    }
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
            Mode::Visual | Mode::VisualLine | Mode::VisualBlock => CursorShape::Block,
            Mode::Insert => CursorShape::Bar,
        }
    }

    pub fn vim_controlled(&self) -> bool {
        !matches!(self.mode, Mode::Insert)
            || matches!(
                self.operator_stack.last(),
                Some(Operator::FindForward { .. }) | Some(Operator::FindBackward { .. })
            )
    }

    pub fn should_autoindent(&self) -> bool {
        !(self.mode == Mode::Insert && self.last_mode == Mode::VisualBlock)
    }

    pub fn clip_at_line_ends(&self) -> bool {
        match self.mode {
            Mode::Insert | Mode::Visual | Mode::VisualLine | Mode::VisualBlock => false,
            Mode::Normal => true,
        }
    }

    pub fn active_operator(&self) -> Option<Operator> {
        self.operator_stack.last().copied()
    }

    pub fn keymap_context_layer(&self) -> KeyContext {
        let mut context = KeyContext::default();
        context.set(
            "vim_mode",
            match self.mode {
                Mode::Normal => "normal",
                Mode::Visual | Mode::VisualLine | Mode::VisualBlock => "visual",
                Mode::Insert => "insert",
            },
        );

        if self.vim_controlled() {
            context.add("VimControl");
        }

        if self.active_operator().is_none() && self.pre_count.is_some()
            || self.active_operator().is_some() && self.post_count.is_some()
        {
            context.add("VimCount");
        }

        let active_operator = self.active_operator();

        if let Some(active_operator) = active_operator {
            for context_flag in active_operator.context_flags().into_iter() {
                context.add(*context_flag);
            }
        }

        context.set(
            "vim_operator",
            active_operator.map(|op| op.id()).unwrap_or_else(|| "none"),
        );

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
            Operator::FindForward { before: false } => "f",
            Operator::FindForward { before: true } => "t",
            Operator::FindBackward { after: false } => "F",
            Operator::FindBackward { after: true } => "T",
        }
    }

    pub fn context_flags(&self) -> &'static [&'static str] {
        match self {
            Operator::Object { .. } => &["VimObject"],
            Operator::FindForward { .. } | Operator::FindBackward { .. } | Operator::Replace => {
                &["VimWaiting"]
            }
            _ => &[],
        }
    }
}
