use crate::{
    element::{Element, Layout},
    layout_context::LayoutContext,
    paint_context::PaintContext,
    style::{Style, StyleHelpers, StyleRefinement, Styleable},
};
use anyhow::Result;
use gpui::platform::MouseMovedEvent;
use refineable::Refineable;
use std::cell::Cell;

pub struct Hoverable<E: Styleable> {
    hovered: Cell<bool>,
    child_style: StyleRefinement,
    hovered_style: StyleRefinement,
    child: E,
}

pub fn hoverable<E: Styleable>(mut child: E) -> Hoverable<E> {
    Hoverable {
        hovered: Cell::new(false),
        child_style: child.declared_style().clone(),
        hovered_style: Default::default(),
        child,
    }
}

impl<E: Styleable> Styleable for Hoverable<E> {
    type Style = E::Style;

    fn declared_style(&mut self) -> &mut crate::style::StyleRefinement {
        &mut self.hovered_style
    }
}

impl<V: 'static, E: Element<V> + Styleable> Element<V> for Hoverable<E> {
    type Layout = E::Layout;

    fn layout(&mut self, view: &mut V, cx: &mut LayoutContext<V>) -> Result<Layout<V, Self::Layout>>
    where
        Self: Sized,
    {
        self.child.layout(view, cx)
    }

    fn paint(
        &mut self,
        view: &mut V,
        layout: &mut Layout<V, Self::Layout>,
        cx: &mut PaintContext<V>,
    ) where
        Self: Sized,
    {
        let bounds = layout.bounds(cx);
        let order = layout.order(cx);

        self.hovered.set(bounds.contains_point(cx.mouse_position()));
        if self.hovered.get() {
            // If hovered, refine the child's style with this element's style.
            self.child.declared_style().refine(&self.hovered_style);
        } else {
            // Otherwise, set the child's style back to its original style.
            *self.child.declared_style() = self.child_style.clone();
        }

        let hovered = self.hovered.clone();
        cx.on_event(order, move |view, event: &MouseMovedEvent, cx| {
            if bounds.contains_point(event.position) != hovered.get() {
                cx.repaint();
            }
        });

        self.child.paint(view, layout, cx);
    }
}

impl<E: Styleable<Style = Style>> StyleHelpers for Hoverable<E> {}
