use gpui::keymap_matcher::KeymapContext;
use language::CursorShape;
use serde::{Deserialize, Serialize};
use workspace::searchable::Direction;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum Mode {
    Normal,
    Insert,
    Visual { line: bool },
}

impl Default for Mode {
    fn default() -> Self {
        Self::Normal
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Deserialize)]
pub enum Operator {
    Number(usize),
    Change,
    Delete,
    Yank,
    Replace,
    Object { around: bool },
    FindForward { before: bool },
    FindBackward { after: bool },
}

#[derive(Default)]
pub struct VimState {
    pub mode: Mode,
    pub operator_stack: Vec<Operator>,
    pub search: SearchState,
}

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

impl VimState {
    pub fn cursor_shape(&self) -> CursorShape {
        match self.mode {
            Mode::Normal => {
                if self.operator_stack.is_empty() {
                    CursorShape::Block
                } else {
                    CursorShape::Underscore
                }
            }
            Mode::Visual { .. } => CursorShape::Block,
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

    pub fn clip_at_line_end(&self) -> bool {
        !matches!(self.mode, Mode::Insert | Mode::Visual { .. })
    }

    pub fn empty_selections_only(&self) -> bool {
        !matches!(self.mode, Mode::Visual { .. })
    }

    pub fn keymap_context_layer(&self) -> KeymapContext {
        let mut context = KeymapContext::default();
        context.add_identifier("VimEnabled");
        context.add_key(
            "vim_mode",
            match self.mode {
                Mode::Normal => "normal",
                Mode::Visual { .. } => "visual",
                Mode::Insert => "insert",
            },
        );

        if self.vim_controlled() {
            context.add_identifier("VimControl");
        }

        let active_operator = self.operator_stack.last();

        if let Some(active_operator) = active_operator {
            for context_flag in active_operator.context_flags().into_iter() {
                context.add_identifier(*context_flag);
            }
        }

        context.add_key(
            "vim_operator",
            active_operator.map(|op| op.id()).unwrap_or_else(|| "none"),
        );

        context
    }
}

impl Operator {
    pub fn id(&self) -> &'static str {
        match self {
            Operator::Number(_) => "n",
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
