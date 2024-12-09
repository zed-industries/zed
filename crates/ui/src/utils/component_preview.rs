use crate::{content_group, prelude::*};
use gpui::AnyElement;
use smallvec::SmallVec;

/// Specifies the side on which the preview label should be displayed.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum PreviewLabelSide {
    /// Left side
    Left,
    /// Right side
    Right,
    /// Top side
    Top,
    #[default]
    /// Bottom side
    Bottom,
}

/// A component preview with a label and children.
#[derive(IntoElement)]
pub struct ComponentPreview {
    label: Option<SharedString>,
    label_side: PreviewLabelSide,
    children: SmallVec<[AnyElement; 2]>,
}

impl ComponentPreview {
    /// Creates a new ComponentPreview with the specified label side.
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: Some(label.into()),
            label_side: PreviewLabelSide::default(),
            children: SmallVec::new(),
        }
    }

    /// Creates a new ComponentPreview with no label.
    pub fn no_label() -> Self {
        Self {
            label: None,
            label_side: PreviewLabelSide::default(),
            children: SmallVec::new(),
        }
    }

    /// Sets the label for the ComponentPreview.
    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    fn base(&self) -> Div {
        div().flex().map(|this| match self.label_side {
            PreviewLabelSide::Left => this.flex_row(),
            PreviewLabelSide::Right => this.flex_row_reverse(),
            PreviewLabelSide::Top => this.flex_col(),
            PreviewLabelSide::Bottom => this.flex_col_reverse(),
        })
    }
}

impl RenderOnce for ComponentPreview {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let el_base = self.base();
        let children_base = self.base();

        el_base
            .gap_1()
            .when_some(self.label, |this, label| {
                this.child(Label::new(label).color(Color::Muted))
            })
            .child(children_base.children(self.children))
    }
}

impl ParentElement for ComponentPreview {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements)
    }
}

/// A group of component previews.
#[derive(IntoElement)]
pub struct ComponentPreviewGroup {
    children: SmallVec<[AnyElement; 2]>,
}

impl ComponentPreviewGroup {
    /// Creates a new ComponentPreviewGroup.
    pub fn new() -> Self {
        Self {
            children: SmallVec::new(),
        }
    }
}

impl ParentElement for ComponentPreviewGroup {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements)
    }
}

impl RenderOnce for ComponentPreviewGroup {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        content_group().size_full().children(self.children)
    }
}

/// Creates a new [ComponentPreview]
pub fn component_preview(label: impl Into<SharedString>) -> ComponentPreview {
    ComponentPreview::new(label)
}

/// Creates a new [ComponentPreviewGroup]
pub fn component_preview_group() -> ComponentPreviewGroup {
    ComponentPreviewGroup::new()
}
