use gpui3::{hsla, rems, AbsoluteLength, Hsla};

#[derive(Clone, Copy)]
pub struct Token {
    pub list_indent_depth: AbsoluteLength,
    pub default_panel_size: AbsoluteLength,
    pub state_hover_background: Hsla,
    pub state_active_background: Hsla,
}

impl Default for Token {
    fn default() -> Self {
        Self {
            list_indent_depth: AbsoluteLength::Rems(rems(0.5)),
            default_panel_size: AbsoluteLength::Rems(rems(16.)),
            state_hover_background: hsla(0.0, 0.0, 0.0, 0.08),
            state_active_background: hsla(0.0, 0.0, 0.0, 0.16),
        }
    }
}

pub fn token() -> Token {
    Token::default()
}
