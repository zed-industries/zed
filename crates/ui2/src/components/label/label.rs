use gpui::WindowContext;

use crate::{prelude::*, LabelCommon, LabelLike, LabelSize, LineHeightStyle};

#[derive(IntoElement)]
pub struct Label {
    base: LabelLike,
    label: SharedString,
}

impl Label {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            base: LabelLike::new(),
            label: label.into(),
        }
    }
}

impl LabelCommon for Label {
    fn size(mut self, size: LabelSize) -> Self {
        self.base = self.base.size(size);
        self
    }

    fn line_height_style(mut self, line_height_style: LineHeightStyle) -> Self {
        self.base = self.base.line_height_style(line_height_style);
        self
    }

    fn color(mut self, color: Color) -> Self {
        self.base = self.base.color(color);
        self
    }

    fn strikethrough(mut self, strikethrough: bool) -> Self {
        self.base = self.base.strikethrough(strikethrough);
        self
    }
}

impl RenderOnce for Label {
    type Rendered = LabelLike;

    fn render(self, _cx: &mut WindowContext) -> Self::Rendered {
        self.base.child(self.label)
    }
}
