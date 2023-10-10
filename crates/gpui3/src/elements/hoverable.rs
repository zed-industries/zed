use crate::{
    Bounds, Element, Interactive, MouseEventListeners, Pixels, Style, Styled, ViewContext,
};
use anyhow::Result;
use refineable::{CascadeSlot, RefinementCascade};
use std::{cell::Cell, rc::Rc};

pub fn hoverable<E: Styled>(mut child: E) -> Hoverable<E> {
    Hoverable {
        hovered: Rc::new(Cell::new(false)),
        cascade_slot: child.style_cascade().reserve(),
        hovered_style: Default::default(),
        child,
    }
}

pub struct Hoverable<E: Styled> {
    hovered: Rc<Cell<bool>>,
    cascade_slot: CascadeSlot,
    hovered_style: RefinementCascade<E::Style>,
    child: E,
}

impl<E> Styled for Hoverable<E>
where
    E: Styled,
{
    type Style = E::Style;

    fn style_cascade(&mut self) -> &mut RefinementCascade<E::Style> {
        self.child.style_cascade()
    }

    fn declared_style(&mut self) -> &mut <Self::Style as refineable::Refineable>::Refinement {
        &mut self.hovered_style
    }
}

impl<S: 'static + Send + Sync, E: Interactive<S> + Styled> Interactive<S> for Hoverable<E> {
    fn listeners(&mut self) -> &mut MouseEventListeners<S> {
        self.child.listeners()
    }
}

impl<E: Element + Styled> Element for Hoverable<E> {
    type State = E::State;
    type FrameState = E::FrameState;

    fn layout(
        &mut self,
        state: &mut Self::State,
        cx: &mut ViewContext<Self::State>,
    ) -> Result<(crate::LayoutId, Self::FrameState)> {
        Ok(self.child.layout(state, cx)?)
    }

    fn paint(
        &mut self,
        bounds: Bounds<Pixels>,
        state: &mut Self::State,
        frame_state: &mut Self::FrameState,
        cx: &mut ViewContext<Self::State>,
    ) -> Result<()> {
        todo!()
        // self.hovered.set(bounds.contains_point(cx.mouse_position()));

        // let slot = self.cascade_slot;
        // let style = self.hovered.get().then_some(self.hovered_style.clone());
        // self.style_cascade().set(slot, style);

        // let hovered = self.hovered.clone();
        // cx.on_event(layout.order, move |_view, _: &MouseMovedEvent, cx| {
        //     cx.bubble_event();
        //     if bounds.contains_point(cx.mouse_position()) != hovered.get() {
        //         cx.repaint();
        //     }
        // });

        // self.child
        //     .paint(view, parent_origin, layout, paint_state, cx);
    }
}
