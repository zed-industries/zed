use gpui::{relative, AnyElement, Styled};
use smallvec::SmallVec;

use crate::prelude::*;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Default)]
pub enum LabelSize {
    #[default]
    Default,
    Large,
    Small,
    XSmall,
}

#[derive(Default, PartialEq, Copy, Clone)]
pub enum LineHeightStyle {
    #[default]
    TextLabel,
    /// Sets the line height to 1.
    UiLabel,
}

/// A common set of traits all labels must implement.
pub trait LabelCommon {
    /// Sets the size of the label using a [`LabelSize`].
    fn size(self, size: LabelSize) -> Self;

    /// Sets the line height style of the label using a [`LineHeightStyle`].
    fn line_height_style(self, line_height_style: LineHeightStyle) -> Self;

    /// Sets the color of the label using a [`Color`].
    fn color(self, color: Color) -> Self;

    /// Sets the strikethrough property of the label.
    fn strikethrough(self, strikethrough: bool) -> Self;
}

#[derive(IntoElement)]
pub struct LabelLike {
    size: LabelSize,
    line_height_style: LineHeightStyle,
    pub(crate) color: Color,
    strikethrough: bool,
    children: SmallVec<[AnyElement; 2]>,
}

impl LabelLike {
    pub fn new() -> Self {
        Self {
            size: LabelSize::Default,
            line_height_style: LineHeightStyle::default(),
            color: Color::Default,
            strikethrough: false,
            children: SmallVec::new(),
        }
    }
}

impl LabelCommon for LabelLike {
    fn size(mut self, size: LabelSize) -> Self {
        self.size = size;
        self
    }

    fn line_height_style(mut self, line_height_style: LineHeightStyle) -> Self {
        self.line_height_style = line_height_style;
        self
    }

    fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    fn strikethrough(mut self, strikethrough: bool) -> Self {
        self.strikethrough = strikethrough;
        self
    }
}

impl ParentElement for LabelLike {
    fn extend(&mut self, elements: impl Iterator<Item = AnyElement>) {
        self.children.extend(elements)
    }
}

impl RenderOnce for LabelLike {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        div()
            .when(self.strikethrough, |this| {
                this.relative().child(
                    div()
                        .absolute()
                        .top_1_2()
                        .w_full()
                        .h_px()
                        .bg(Color::Hidden.color(cx)),
                )
            })
            .map(|this| match self.size {
                LabelSize::Large => this.text_ui_lg(),
                LabelSize::Default => this.text_ui(),
                LabelSize::Small => this.text_ui_sm(),
                LabelSize::XSmall => this.text_ui_xs(),
            })
            .when(self.line_height_style == LineHeightStyle::UiLabel, |this| {
                this.line_height(relative(1.))
            })
            .text_color(self.color.color(cx))
            .children(self.children)
    }
}
