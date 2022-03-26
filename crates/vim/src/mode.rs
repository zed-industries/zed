use editor::CursorShape;
use gpui::keymap::Context;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    Normal,
    Insert,
}

impl Mode {
    pub fn cursor_shape(&self) -> CursorShape {
        match self {
            Mode::Normal => CursorShape::Block,
            Mode::Insert => CursorShape::Bar,
        }
    }

    pub fn keymap_context_layer(&self) -> Context {
        let mut context = Context::default();
        context.map.insert(
            "vim_mode".to_string(),
            match self {
                Self::Normal => "normal",
                Self::Insert => "insert",
            }
            .to_string(),
        );
        context
    }
}

impl Default for Mode {
    fn default() -> Self {
        Self::Normal
    }
}
