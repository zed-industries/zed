use gpui::{Hsla, IntoElement, Point, svg};
use strum::{EnumIter, EnumString, IntoStaticStr};
use ui_macros::DerivePathStr;

use crate::prelude::*;

const ICON_DECORATION_SIZE: Pixels = px(11.);

/// An icon silhouette used to knockout the background of an element for an icon
/// to sit on top of it, emulating a stroke/border.
#[derive(Debug, PartialEq, Eq, Copy, Clone, EnumIter, EnumString, IntoStaticStr, DerivePathStr)]
#[strum(serialize_all = "snake_case")]
#[path_str(prefix = "icons/knockouts", suffix = ".svg")]
pub enum KnockoutIconName {
    XFg,
    XBg,
    DotFg,
    DotBg,
    TriangleFg,
    TriangleBg,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, EnumIter, EnumString)]
pub enum IconDecorationKind {
    X,
    Dot,
    Triangle,
}

impl IconDecorationKind {
    fn fg(&self) -> KnockoutIconName {
        match self {
            Self::X => KnockoutIconName::XFg,
            Self::Dot => KnockoutIconName::DotFg,
            Self::Triangle => KnockoutIconName::TriangleFg,
        }
    }

    fn bg(&self) -> KnockoutIconName {
        match self {
            Self::X => KnockoutIconName::XBg,
            Self::Dot => KnockoutIconName::DotBg,
            Self::Triangle => KnockoutIconName::TriangleBg,
        }
    }
}

/// The decoration for an icon.
///
/// For example, this can show an indicator, an "x", or a diagonal strikethrough
/// to indicate something is disabled.
#[derive(IntoElement)]
pub struct IconDecoration {
    kind: IconDecorationKind,
    color: Hsla,
    knockout_color: Hsla,
    knockout_hover_color: Hsla,
    position: Point<Pixels>,
    group_name: Option<SharedString>,
}

impl IconDecoration {
    /// Creates a new [`IconDecoration`].
    pub fn new(kind: IconDecorationKind, knockout_color: Hsla, cx: &App) -> Self {
        let color = cx.theme().colors().icon;
        let position = Point::default();

        Self {
            kind,
            color,
            knockout_color,
            knockout_hover_color: knockout_color,
            position,
            group_name: None,
        }
    }

    /// Sets the kind of decoration.
    pub fn kind(mut self, kind: IconDecorationKind) -> Self {
        self.kind = kind;
        self
    }

    /// Sets the color of the decoration.
    pub fn color(mut self, color: Hsla) -> Self {
        self.color = color;
        self
    }

    /// Sets the color of the decoration's knockout
    ///
    /// Match this to the background of the element the icon will be rendered
    /// on.
    pub fn knockout_color(mut self, color: Hsla) -> Self {
        self.knockout_color = color;
        self
    }

    /// Sets the color of the decoration that is used on hover.
    pub fn knockout_hover_color(mut self, color: Hsla) -> Self {
        self.knockout_hover_color = color;
        self
    }

    /// Sets the position of the decoration.
    pub fn position(mut self, position: Point<Pixels>) -> Self {
        self.position = position;
        self
    }

    /// Sets the name of the group the decoration belongs to
    pub fn group_name(mut self, name: Option<SharedString>) -> Self {
        self.group_name = name;
        self
    }
}

impl RenderOnce for IconDecoration {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let foreground = svg()
            .absolute()
            .bottom_0()
            .right_0()
            .size(ICON_DECORATION_SIZE)
            .path(self.kind.fg().path())
            .text_color(self.color);

        let background = svg()
            .absolute()
            .bottom_0()
            .right_0()
            .size(ICON_DECORATION_SIZE)
            .path(self.kind.bg().path())
            .text_color(self.knockout_color)
            .map(|this| match self.group_name {
                Some(group_name) => this.group_hover(group_name, |style| {
                    style.text_color(self.knockout_hover_color)
                }),
                None => this.hover(|style| style.text_color(self.knockout_hover_color)),
            });

        div()
            .size(ICON_DECORATION_SIZE)
            .flex_none()
            .absolute()
            .bottom(self.position.y)
            .right(self.position.x)
            .child(foreground)
            .child(background)
    }
}
