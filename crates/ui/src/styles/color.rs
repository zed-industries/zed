use gpui::{Hsla, WindowContext};
use theme::ActiveTheme;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum StateColor {
    Active,
    Inactive,
    Disabled,
}

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
    State(StateColor),
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
            Color::State(state) => {
                let (active, inactive, disabled) = build_state_colors(cx);

                match state {
                    StateColor::Active => active,
                    StateColor::Inactive => inactive,
                    StateColor::Disabled => disabled,
                }
            }
            Color::Custom(color) => *color,
        }
    }
}

fn most_intense_color(cx: &WindowContext, colors: &Vec<Hsla>) -> Hsla {
    let is_light = cx.theme().appearance().is_light();
    colors
        .iter()
        .max_by(|a, b| {
            if is_light {
                b.l.partial_cmp(&a.l).unwrap()
            } else {
                a.l.partial_cmp(&b.l).unwrap()
            }
        })
        .cloned()
        .unwrap_or_default()
}

fn build_state_colors(cx: &WindowContext) -> (Hsla, Hsla, Hsla) {
    let is_light = cx.theme().appearance().is_light();
    let color_canidates = vec![
        cx.theme().colors().text,
        cx.theme().colors().editor_foreground,
    ];
    let base_color = most_intense_color(cx, &color_canidates);

    let intensity_reduction = if is_light { 0.24 } else { 0.2 };

    let active = base_color;

    let inactive = if is_light {
        Hsla {
            h: base_color.h,
            s: (base_color.s / 2.0).max(0.0),
            l: (base_color.l + intensity_reduction).min(1.0),
            a: base_color.a,
        }
    } else {
        Hsla {
            h: base_color.h,
            s: (base_color.s / 3.0).max(0.0),
            l: (base_color.l - intensity_reduction).max(0.0),
            a: base_color.a,
        }
    };

    let disabled = if is_light {
        Hsla {
            h: base_color.h,
            s: (base_color.s / 3.0).max(0.0),
            l: (base_color.l + 2.0 * intensity_reduction).min(1.0),
            a: base_color.a,
        }
    } else {
        Hsla {
            h: base_color.h,
            s: (base_color.s / 4.0).max(0.0),
            l: (base_color.l - 2.0 * intensity_reduction).max(0.0),
            a: base_color.a,
        }
    };

    (active, inactive, disabled)
}
