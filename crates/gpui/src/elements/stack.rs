use std::ops::Range;

use crate::{
    geometry::{rect::RectF, vector::Vector2F},
    json::{self, json, ToJson},
    AnyElement, Element, SceneBuilder, SizeConstraint, View, ViewContext,
};

/// Element which renders it's children in a stack on top of each other.
/// The first child determines the size of the others.
pub struct Stack<V: View> {
    children: Vec<AnyElement<V>>,
}

impl<V: View> Default for Stack<V> {
    fn default() -> Self {
        Self {
            children: Vec::new(),
        }
    }
}

impl<V: View> Stack<V> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<V: View> Element<V> for Stack<V> {
    type LayoutState = ();
    type PaintState = ();

    fn layout(
        &mut self,
        mut constraint: SizeConstraint,
        view: &mut V,
        cx: &mut ViewContext<V>,
    ) -> (Vector2F, Self::LayoutState) {
        let mut size = constraint.min;
        let mut children = self.children.iter_mut();
        if let Some(bottom_child) = children.next() {
            size = bottom_child.layout(constraint, view, cx);
            constraint = SizeConstraint::strict(size);
        }

        for child in children {
            child.layout(constraint, view, cx);
        }

        (size, ())
    }

    fn paint(
        &mut self,
        scene: &mut SceneBuilder,
        bounds: RectF,
        visible_bounds: RectF,
        _: &mut Self::LayoutState,
        view: &mut V,
        cx: &mut ViewContext<V>,
    ) -> Self::PaintState {
        for child in &mut self.children {
            scene.paint_layer(None, |scene| {
                child.paint(scene, bounds.origin(), visible_bounds, view, cx);
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
        view: &V,
        cx: &ViewContext<V>,
    ) -> Option<RectF> {
        self.children
            .iter()
            .rev()
            .find_map(|child| child.rect_for_text_range(range_utf16.clone(), view, cx))
    }

    fn debug(
        &self,
        bounds: RectF,
        _: &Self::LayoutState,
        _: &Self::PaintState,
        view: &V,
        cx: &ViewContext<V>,
    ) -> json::Value {
        json!({
            "type": "Stack",
            "bounds": bounds.to_json(),
            "children": self.children.iter().map(|child| child.debug(view, cx)).collect::<Vec<json::Value>>()
        })
    }
}

impl<V: View> Extend<AnyElement<V>> for Stack<V> {
    fn extend<T: IntoIterator<Item = AnyElement<V>>>(&mut self, children: T) {
        self.children.extend(children)
    }
}
