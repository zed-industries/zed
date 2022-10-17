use gpui::keymap::Context;
use language::CursorShape;
use serde::{Deserialize, Serialize};

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
pub enum Namespace {
    G,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Deserialize)]
pub enum Operator {
    Number(usize),
    Namespace(Namespace),
    Change,
    Delete,
    Yank,
    Object { around: bool },
}

#[derive(Default)]
pub struct VimState {
    pub mode: Mode,
    pub operator_stack: Vec<Operator>,
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
    }

    pub fn clip_at_line_end(&self) -> bool {
        !matches!(self.mode, Mode::Insert | Mode::Visual { .. })
    }

    pub fn empty_selections_only(&self) -> bool {
        !matches!(self.mode, Mode::Visual { .. })
    }

    pub fn update_global_keymap_context(&self, context: &mut Context, enabled: bool) {
        if enabled {
            context.map.insert(
                "vim_mode".to_string(),
                match self.mode {
                    Mode::Normal => "normal",
                    Mode::Visual { .. } => "visual",
                    Mode::Insert => "insert",
                }
                .to_string(),
            );

            if self.vim_controlled() {
                context.set.insert("VimControl".to_string());
            } else {
                context.set.remove("VimControl");
            }

            let active_operator = self.operator_stack.last();
            if matches!(active_operator, Some(Operator::Object { .. })) {
                context.set.insert("VimObject".to_string());
            } else {
                context.set.remove("VimObject");
            }

            let operator_context = match active_operator {
                Some(Operator::Number(_)) => "n",
                Some(Operator::Namespace(Namespace::G)) => "g",
                Some(Operator::Object { around: false }) => "i",
                Some(Operator::Object { around: true }) => "a",
                Some(Operator::Change) => "c",
                Some(Operator::Delete) => "d",
                Some(Operator::Yank) => "y",

                None => "none",
            }
            .to_owned();

            context
                .map
                .insert("vim_operator".to_string(), operator_context);
        } else {
            context.set.remove("VimControl");
            context.set.remove("VimObject");
            context.map.remove("vim_mode");
            context.map.remove("vim_operator");
        }
    }
}
