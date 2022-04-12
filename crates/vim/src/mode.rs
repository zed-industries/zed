use editor::CursorShape;
use gpui::keymap::Context;
use serde::Deserialize;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize)]
pub enum Mode {
    Normal(NormalState),
    Insert,
}

impl Mode {
    pub fn cursor_shape(&self) -> CursorShape {
        match self {
            Mode::Normal(_) => CursorShape::Block,
            Mode::Insert => CursorShape::Bar,
        }
    }

    pub fn keymap_context_layer(&self) -> Context {
        let mut context = Context::default();
        context.map.insert(
            "vim_mode".to_string(),
            match self {
                Self::Normal(_) => "normal",
                Self::Insert => "insert",
            }
            .to_string(),
        );

        match self {
            Self::Normal(normal_state) => normal_state.set_context(&mut context),
            _ => {}
        }
        context
    }

    pub fn normal() -> Mode {
        Mode::Normal(Default::default())
    }
}

impl Default for Mode {
    fn default() -> Self {
        Self::Normal(Default::default())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize)]
pub enum NormalState {
    None,
    GPrefix,
}

impl NormalState {
    pub fn set_context(&self, context: &mut Context) {
        let submode = match self {
            Self::GPrefix => Some("g"),
            _ => None,
        };

        if let Some(submode) = submode {
            context
                .map
                .insert("vim_submode".to_string(), submode.to_string());
        }
    }
}

impl Default for NormalState {
    fn default() -> Self {
        NormalState::None
    }
}
