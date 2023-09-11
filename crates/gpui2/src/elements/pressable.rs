use crate::{
    element::{AnyElement, Element, IntoElement, Layout, ParentElement},
    interactive::{InteractionHandlers, Interactive},
    style::{Style, StyleHelpers, Styleable},
    ViewContext,
};
use anyhow::Result;
use gpui::{geometry::vector::Vector2F, platform::MouseButtonEvent, LayoutId};
use refineable::{CascadeSlot, Refineable, RefinementCascade};
use smallvec::SmallVec;
use std::{cell::Cell, rc::Rc};

pub struct Pressable<E: Styleable> {
    pressed: Rc<Cell<bool>>,
    pressed_style: <E::Style as Refineable>::Refinement,
    cascade_slot: CascadeSlot,
    child: E,
}

pub fn pressable<E: Styleable>(mut child: E) -> Pressable<E> {
    Pressable {
        pressed: Rc::new(Cell::new(false)),
        pressed_style: Default::default(),
        cascade_slot: child.style_cascade().reserve(),
        child,
    }
}

impl<E: Styleable> Styleable for Pressable<E> {
    type Style = E::Style;

    fn declared_style(&mut self) -> &mut <Self::Style as Refineable>::Refinement {
        &mut self.pressed_style
    }

    fn style_cascade(&mut self) -> &mut RefinementCascade<E::Style> {
        self.child.style_cascade()
    }
}

impl<V: 'static, E: Element<V> + Styleable> Element<V> for Pressable<E> {
    type PaintState = E::PaintState;

    fn layout(
        &mut self,
        view: &mut V,
        cx: &mut ViewContext<V>,
    ) -> Result<(LayoutId, Self::PaintState)>
    where
        Self: Sized,
    {
        self.child.layout(view, cx)
    }

    fn paint(
        &mut self,
        view: &mut V,
        parent_origin: Vector2F,
        layout: &Layout,
        paint_state: &mut Self::PaintState,
        cx: &mut ViewContext<V>,
    ) where
        Self: Sized,
    {
        let slot = self.cascade_slot;
        let style = self.pressed.get().then_some(self.pressed_style.clone());
        self.style_cascade().set(slot, style);

        let pressed = self.pressed.clone();
        let bounds = layout.bounds + parent_origin;
        cx.on_event(layout.order, move |_view, event: &MouseButtonEvent, cx| {
            if event.is_down {
                if bounds.contains_point(event.position) {
                    pressed.set(true);
                    cx.repaint();
                }
            } else if pressed.get() {
                pressed.set(false);
                cx.repaint();
            }
        });

        self.child
            .paint(view, parent_origin, layout, paint_state, cx);
    }
}

impl<E: Styleable<Style = Style>> StyleHelpers for Pressable<E> {}

impl<V: 'static, E: Interactive<V> + Styleable> Interactive<V> for Pressable<E> {
    fn interaction_handlers(&mut self) -> &mut InteractionHandlers<V> {
        self.child.interaction_handlers()
    }
}

impl<V: 'static, E: ParentElement<V> + Styleable> ParentElement<V> for Pressable<E> {
    fn children_mut(&mut self) -> &mut SmallVec<[AnyElement<V>; 2]> {
        self.child.children_mut()
    }
}

impl<V: 'static, E: Element<V> + Styleable> IntoElement<V> for Pressable<E> {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}
