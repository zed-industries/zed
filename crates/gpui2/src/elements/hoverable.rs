use crate::{
    element::{AnyElement, Element, IntoElement, Layout, ParentElement},
    interactive::{InteractionHandlers, Interactive},
    paint_context::PaintContext,
    style::{Style, StyleHelpers, Styleable},
    ViewContext,
};
use anyhow::Result;
use gpui::{geometry::vector::Vector2F, platform::MouseMovedEvent, LayoutId};
use refineable::{CascadeSlot, Refineable, RefinementCascade};
use smallvec::SmallVec;
use std::{cell::Cell, rc::Rc};

pub struct Hoverable<E: Styleable> {
    hovered: Rc<Cell<bool>>,
    cascade_slot: CascadeSlot,
    hovered_style: <E::Style as Refineable>::Refinement,
    child: E,
}

pub fn hoverable<E: Styleable>(mut child: E) -> Hoverable<E> {
    Hoverable {
        hovered: Rc::new(Cell::new(false)),
        cascade_slot: child.style_cascade().reserve(),
        hovered_style: Default::default(),
        child,
    }
}

impl<E: Styleable> Styleable for Hoverable<E> {
    type Style = E::Style;

    fn style_cascade(&mut self) -> &mut RefinementCascade<Self::Style> {
        self.child.style_cascade()
    }

    fn declared_style(&mut self) -> &mut <Self::Style as Refineable>::Refinement {
        &mut self.hovered_style
    }
}

impl<V: 'static, E: Element<V> + Styleable> Element<V> for Hoverable<E> {
    type PaintState = E::PaintState;

    fn layout(
        &mut self,
        view: &mut V,
        cx: &mut ViewContext<V>,
    ) -> Result<(LayoutId, Self::PaintState)>
    where
        Self: Sized,
    {
        Ok(self.child.layout(view, cx)?)
    }

    fn paint(
        &mut self,
        view: &mut V,
        parent_origin: Vector2F,
        layout: &Layout,
        paint_state: &mut Self::PaintState,
        cx: &mut PaintContext<V>,
    ) where
        Self: Sized,
    {
        let bounds = layout.bounds + parent_origin;
        self.hovered.set(bounds.contains_point(cx.mouse_position()));

        let slot = self.cascade_slot;
        let style = self.hovered.get().then_some(self.hovered_style.clone());
        self.style_cascade().set(slot, style);

        let hovered = self.hovered.clone();
        cx.on_event(layout.order, move |_view, _: &MouseMovedEvent, cx| {
            cx.bubble_event();
            if bounds.contains_point(cx.mouse_position()) != hovered.get() {
                cx.repaint();
            }
        });

        self.child
            .paint(view, parent_origin, layout, paint_state, cx);
    }
}

impl<E: Styleable<Style = Style>> StyleHelpers for Hoverable<E> {}

impl<V: 'static, E: Interactive<V> + Styleable> Interactive<V> for Hoverable<E> {
    fn interaction_handlers(&mut self) -> &mut InteractionHandlers<V> {
        self.child.interaction_handlers()
    }
}

impl<V: 'static, E: ParentElement<V> + Styleable> ParentElement<V> for Hoverable<E> {
    fn children_mut(&mut self) -> &mut SmallVec<[AnyElement<V>; 2]> {
        self.child.children_mut()
    }
}

impl<V: 'static, E: Element<V> + Styleable> IntoElement<V> for Hoverable<E> {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}
