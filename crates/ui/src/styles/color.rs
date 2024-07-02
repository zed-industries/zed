use gpui::{Hsla, WindowContext};
use theme::ActiveTheme;

/// Sets a color that has a consistent meaning across all themes.
#[derive(Debug, Default, PartialEq, Copy, Clone)]
pub enum Color {
    #[default]
    Default,
    Accent,
    Created,
    Deleted,
    Disabled,
    Error,
    Hidden,
    Hint,
    Info,
    Modified,
    Conflict,
    Ignored,
    Muted,
    Placeholder,
    Player(u32),
    Selected,
    Success,
    Warning,
    Custom(Hsla),
}

impl Color {
    pub fn color(&self, cx: &WindowContext) -> Hsla {
        match self {
            Color::Default => cx.theme().colors().text,
            Color::Muted => cx.theme().colors().text_muted,
            Color::Created => cx.theme().status().created,
            Color::Modified => cx.theme().status().modified,
            Color::Conflict => cx.theme().status().conflict,
            Color::Ignored => cx.theme().status().ignored,
            Color::Deleted => cx.theme().status().deleted,
            Color::Disabled => cx.theme().colors().text_disabled,
            Color::Hidden => cx.theme().status().hidden,
            Color::Hint => cx.theme().status().hint,
            Color::Info => cx.theme().status().info,
            Color::Placeholder => cx.theme().colors().text_placeholder,
            Color::Accent => cx.theme().colors().text_accent,
            Color::Player(i) => cx.theme().styles.player.color_for_participant(*i).cursor,
            Color::Error => cx.theme().status().error,
            Color::Selected => cx.theme().colors().text_accent,
            Color::Success => cx.theme().status().success,
            Color::Warning => cx.theme().status().warning,
            Color::Custom(color) => *color,
        }
    }
}
