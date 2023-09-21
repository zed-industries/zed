use crate::theme::theme;
use gpui2::elements::div;
use gpui2::style::StyleHelpers;
use gpui2::{Element, ViewContext};
use gpui2::{IntoElement, ParentElement};

#[derive(Default, PartialEq, Copy, Clone)]
pub enum LabelColor {
    #[default]
    Default,
    Created,
    Modified,
    Deleted,
    Hidden,
}

#[derive(Element, Clone)]
pub struct Label {
    label: &'static str,
    color: LabelColor,
}

pub fn label(label: &'static str) -> Label {
    Label {
        label,
        color: LabelColor::Default,
    }
}

impl Label {
    pub fn color(mut self, color: LabelColor) -> Self {
        self.color = color;
        self
    }

    fn render<V: 'static>(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        let theme = theme(cx);

        let color = match self.color {
            LabelColor::Default => theme.lowest.base.default.foreground,
            LabelColor::Created => theme.lowest.positive.default.foreground,
            LabelColor::Modified => theme.lowest.warning.default.foreground,
            LabelColor::Deleted => theme.lowest.negative.default.foreground,
            LabelColor::Hidden => theme.lowest.variant.default.foreground,
        };

        div().text_sm().text_color(color).child(self.label.clone())
    }
}
