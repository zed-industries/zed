use crate::{
    element::{Element, Layout},
    layout_context::LayoutContext,
    paint_context::PaintContext,
    style::{Style, StyleHelpers, Styleable},
};
use anyhow::Result;
use gpui::platform::MouseButtonEvent;
use refineable::{CascadeSlot, Refineable, RefinementCascade};
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
        let slot = self.cascade_slot;
        let style = self.pressed.get().then_some(self.pressed_style.clone());
        self.style_cascade().set(slot, style);

        let bounds = layout.bounds(cx);
        let order = layout.order(cx);
        let pressed = self.pressed.clone();
        cx.on_event(order, move |view, event: &MouseButtonEvent, cx| {
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

        self.child.paint(view, layout, cx);
    }
}

impl<E: Styleable<Style = Style>> StyleHelpers for Pressable<E> {}
