use std::ops::Range;

use crate::{
    geometry::{rect::RectF, vector::Vector2F},
    json::{self, json, ToJson},
    presenter::MeasurementContext,
    DebugContext, Element, ElementBox, LayoutContext, PaintContext, SizeConstraint,
};

/// Element which renders it's children in a stack on top of each other.
/// The first child determines the size of the others.
#[derive(Default)]
pub struct Stack {
    children: Vec<ElementBox>,
}

impl Stack {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Element for Stack {
    type LayoutState = ();
    type PaintState = ();

    fn layout(
        &mut self,
        mut constraint: SizeConstraint,
        cx: &mut LayoutContext,
    ) -> (Vector2F, Self::LayoutState) {
        let mut size = constraint.min;
        let mut children = self.children.iter_mut();
        if let Some(bottom_child) = children.next() {
            size = bottom_child.layout(constraint, cx);
            constraint = SizeConstraint::strict(size);
        }

        for child in children {
            child.layout(constraint, cx);
        }

        (size, ())
    }

    fn paint(
        &mut self,
        bounds: RectF,
        visible_bounds: RectF,
        _: &mut Self::LayoutState,
        cx: &mut PaintContext,
    ) -> Self::PaintState {
        for child in &mut self.children {
            cx.paint_layer(None, |cx| {
                child.paint(bounds.origin(), visible_bounds, cx);
            });
        }
    }

    fn rect_for_text_range(
        &self,
        range_utf16: Range<usize>,
        _: RectF,
        _: RectF,
        _: &Self::LayoutState,
        _: &Self::PaintState,
        cx: &MeasurementContext,
    ) -> Option<RectF> {
        self.children
            .iter()
            .rev()
            .find_map(|child| child.rect_for_text_range(range_utf16.clone(), cx))
    }

    fn debug(
        &self,
        bounds: RectF,
        _: &Self::LayoutState,
        _: &Self::PaintState,
        cx: &DebugContext,
    ) -> json::Value {
        json!({
            "type": "Stack",
            "bounds": bounds.to_json(),
            "children": self.children.iter().map(|child| child.debug(cx)).collect::<Vec<json::Value>>()
        })
    }
}

impl Extend<ElementBox> for Stack {
    fn extend<T: IntoIterator<Item = ElementBox>>(&mut self, children: T) {
        self.children.extend(children)
    }
}
