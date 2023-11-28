use gpui::{Hsla, WindowContext};
use theme::ActiveTheme;

#[derive(Default, PartialEq, Copy, Clone)]
pub enum Color {
    #[default]
    Default,
    Accent,
    Created,
    Deleted,
    Disabled,
    Error,
    Hidden,
    Info,
    Modified,
    Muted,
    Placeholder,
    Player(u32),
    Selected,
    Success,
    Warning,
}

impl Color {
    pub fn color(&self, cx: &WindowContext) -> Hsla {
        match self {
            Color::Default => cx.theme().colors().text,
            Color::Muted => cx.theme().colors().text_muted,
            Color::Created => cx.theme().status().created,
            Color::Modified => cx.theme().status().modified,
            Color::Deleted => cx.theme().status().deleted,
            Color::Disabled => cx.theme().colors().text_disabled,
            Color::Hidden => cx.theme().status().hidden,
            Color::Info => cx.theme().status().info,
            Color::Placeholder => cx.theme().colors().text_placeholder,
            Color::Accent => cx.theme().colors().text_accent,
            Color::Player(i) => cx.theme().styles.player.0[i.clone() as usize].cursor,
            Color::Error => cx.theme().status().error,
            Color::Selected => cx.theme().colors().text_accent,
            Color::Success => cx.theme().status().success,
            Color::Warning => cx.theme().status().warning,
        }
    }
}
