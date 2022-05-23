use editor::CursorShape;
use gpui::keymap::Context;
use serde::Deserialize;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize)]
pub enum Mode {
    Normal,
    Insert,
    Visual,
    VisualLine,
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
    Namespace(Namespace),
    Change,
    Delete,
    Yank,
}

#[derive(Default)]
pub struct VimState {
    pub mode: Mode,
    pub operator_stack: Vec<Operator>,
}

impl VimState {
    pub fn cursor_shape(&self) -> CursorShape {
        match self.mode {
            Mode::Normal | Mode::Visual | Mode::VisualLine => CursorShape::Block,
            Mode::Insert => CursorShape::Bar,
        }
    }

    pub fn vim_controlled(&self) -> bool {
        !matches!(self.mode, Mode::Insert)
    }

    pub fn empty_selections_only(&self) -> bool {
        self.mode != Mode::Visual && self.mode != Mode::VisualLine
    }

    pub fn keymap_context_layer(&self) -> Context {
        let mut context = Context::default();
        context.map.insert(
            "vim_mode".to_string(),
            match self.mode {
                Mode::Normal => "normal",
                Mode::Visual => "visual",
                Mode::VisualLine => "visual_line",
                Mode::Insert => "insert",
            }
            .to_string(),
        );

        if self.vim_controlled() {
            context.set.insert("VimControl".to_string());
        }

        if let Some(operator) = &self.operator_stack.last() {
            operator.set_context(&mut context);
        }
        context
    }
}

impl Operator {
    pub fn set_context(&self, context: &mut Context) {
        let operator_context = match self {
            Operator::Namespace(Namespace::G) => "g",
            Operator::Change => "c",
            Operator::Delete => "d",
            Operator::Yank => "y",
        }
        .to_owned();

        context
            .map
            .insert("vim_operator".to_string(), operator_context.to_string());
    }
}
