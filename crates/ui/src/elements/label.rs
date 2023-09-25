use crate::theme::theme;
use gpui2::elements::div;
use gpui2::style::StyleHelpers;
use gpui2::{Element, ViewContext};
use gpui2::{IntoElement, ParentElement};

#[derive(Default, PartialEq, Copy, Clone)]
pub enum LabelColor {
    #[default]
    Default,
    Muted,
    Created,
    Modified,
    Deleted,
    Disabled,
    Hidden,
    Placeholder,
    Accent,
}

#[derive(Default, PartialEq, Copy, Clone)]
pub enum LabelSize {
    #[default]
    Default,
    Small,
}

#[derive(Element, Clone)]
pub struct Label {
    label: &'static str,
    color: LabelColor,
    size: LabelSize,
}

pub fn label(label: &'static str) -> Label {
    Label {
        label,
        color: LabelColor::Default,
        size: LabelSize::Default,
    }
}

impl Label {
    pub fn color(mut self, color: LabelColor) -> Self {
        self.color = color;
        self
    }

    pub fn size(mut self, size: LabelSize) -> Self {
        self.size = size;
        self
    }

    fn render<V: 'static>(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        let theme = theme(cx);

        let color = match self.color {
            LabelColor::Default => theme.lowest.base.default.foreground,
            LabelColor::Muted => theme.lowest.variant.default.foreground,
            LabelColor::Created => theme.lowest.positive.default.foreground,
            LabelColor::Modified => theme.lowest.warning.default.foreground,
            LabelColor::Deleted => theme.lowest.negative.default.foreground,
            LabelColor::Disabled => theme.lowest.base.disabled.foreground,
            LabelColor::Hidden => theme.lowest.variant.default.foreground,
            LabelColor::Placeholder => theme.lowest.base.disabled.foreground,
            LabelColor::Accent => theme.lowest.accent.default.foreground,
        };

        let mut div = div();

        if self.size == LabelSize::Small {
            div = div.text_xs();
        } else {
            div = div.text_sm();
        }

        div.text_color(color).child(self.label.clone())
    }
}
