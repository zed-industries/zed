use crate::prelude::*;
use gpui::{AnyElement, Axis};
use smallvec::SmallVec;

/// A component preview with a label and children.
#[derive(IntoElement)]
pub struct ComponentPreview {
    label: Option<SharedString>,
    children: SmallVec<[AnyElement; 2]>,
}

impl ComponentPreview {
    /// Creates a new ComponentPreview with the specified label side.
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: Some(label.into()),
            children: SmallVec::new(),
        }
    }

    /// Creates a new ComponentPreview with no label.
    pub fn no_label() -> Self {
        Self {
            label: None,
            children: SmallVec::new(),
        }
    }

    /// Sets the label for the ComponentPreview.
    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }
}

impl RenderOnce for ComponentPreview {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        v_flex()
            .flex_none()
            .gap_3()
            .when_some(self.label, |this, label| {
                this.child(Label::new(label).color(Color::Muted))
            })
            .child(
                h_flex()
                    .gap_1()
                    .w_full()
                    .flex_none()
                    .children(self.children),
            )
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
    direction: Axis,
    children: SmallVec<[AnyElement; 2]>,
}

impl ComponentPreviewGroup {
    /// Creates a new ComponentPreviewGroup.
    pub fn new() -> Self {
        Self {
            direction: Axis::Horizontal,
            children: SmallVec::new(),
        }
    }

    /// Lay out the previews horizontally.
    pub fn horizontal(mut self) -> Self {
        self.direction = Axis::Horizontal;
        self
    }

    /// Lay out the previews vertically.
    pub fn vertical(mut self) -> Self {
        self.direction = Axis::Vertical;
        self
    }
}

impl ParentElement for ComponentPreviewGroup {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements)
    }
}

impl RenderOnce for ComponentPreviewGroup {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        let group = match self.direction {
            Axis::Horizontal => h_group(),
            Axis::Vertical => v_group(),
        };

        group
            .size_full()
            .items_start()
            .outset()
            .gap_3()
            .children(self.children)
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
