use gpui::{
    AnyElement, App, BoxShadow, IntoElement, ParentElement, RenderOnce, Styled, Window, div, hsla,
    point, prelude::FluentBuilder, px,
};
use theme::ActiveTheme;

use crate::{ElevationIndex, IconButton, h_flex};

use super::ButtonLike;

#[derive(Clone, Copy, PartialEq)]
pub enum SplitButtonStyle {
    Filled,
    Outlined,
    Transparent,
}

pub enum SplitButtonKind {
    ButtonLike(ButtonLike),
    IconButton(IconButton),
}

impl From<IconButton> for SplitButtonKind {
    fn from(icon_button: IconButton) -> Self {
        Self::IconButton(icon_button)
    }
}

impl From<ButtonLike> for SplitButtonKind {
    fn from(button_like: ButtonLike) -> Self {
        Self::ButtonLike(button_like)
    }
}

/// /// A button with two parts: a primary action on the left and a secondary action on the right.
///
/// The left side is a [`ButtonLike`] with the main action, while the right side can contain
/// any element (typically a dropdown trigger or similar).
///
/// The two sections are visually separated by a divider, but presented as a unified control.
#[derive(IntoElement)]
pub struct SplitButton {
    left: SplitButtonKind,
    right: AnyElement,
    style: SplitButtonStyle,
}

impl SplitButton {
    pub fn new(left: impl Into<SplitButtonKind>, right: AnyElement) -> Self {
        Self {
            left: left.into(),
            right,
            style: SplitButtonStyle::Filled,
        }
    }

    pub fn style(mut self, style: SplitButtonStyle) -> Self {
        self.style = style;
        self
    }
}

impl RenderOnce for SplitButton {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let is_filled_or_outlined = matches!(
            self.style,
            SplitButtonStyle::Filled | SplitButtonStyle::Outlined
        );

        h_flex()
            .rounded_sm()
            .when(is_filled_or_outlined, |this| {
                this.border_1()
                    .border_color(cx.theme().colors().border.opacity(0.8))
            })
            .child(div().flex_grow().child(match self.left {
                SplitButtonKind::ButtonLike(button) => button.into_any_element(),
                SplitButtonKind::IconButton(icon) => icon.into_any_element(),
            }))
            .child(
                div()
                    .h_full()
                    .w_px()
                    .bg(cx.theme().colors().border.opacity(0.5)),
            )
            .child(self.right)
            .when(self.style == SplitButtonStyle::Filled, |this| {
                this.bg(ElevationIndex::Surface.on_elevation_bg(cx))
                    .shadow(vec![BoxShadow {
                        color: hsla(0.0, 0.0, 0.0, 0.16),
                        offset: point(px(0.), px(1.)),
                        blur_radius: px(0.),
                        spread_radius: px(0.),
                    }])
            })
    }
}
