use gpui2::geometry::AbsoluteLength;

#[derive(Clone, Copy)]
pub struct Token {
    pub list_indent_depth: AbsoluteLength,
}

impl Default for Token {
    fn default() -> Self {
        Self {
            list_indent_depth: AbsoluteLength::Rems(0.5),
        }
    }
}

pub fn token() -> Token {
    Token::default()
}
