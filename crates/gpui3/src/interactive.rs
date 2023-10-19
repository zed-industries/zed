use std::{any::TypeId, sync::Arc};

use crate::{
    DispatchPhase, Element, EventListeners, KeyDownEvent, KeyUpEvent, MouseButton, MouseClickEvent,
    MouseDownEvent, MouseMoveEvent, MouseUpEvent, ScrollWheelEvent, ViewContext,
};

pub trait Interactive: Element {
    fn listeners(&mut self) -> &mut EventListeners<Self::ViewState>;

    fn on_mouse_down(
        mut self,
        button: MouseButton,
        handler: impl Fn(&mut Self::ViewState, &MouseDownEvent, &mut ViewContext<Self::ViewState>)
            + Send
            + Sync
            + 'static,
    ) -> Self
    where
        Self: Sized,
    {
        self.listeners()
            .mouse_down
            .push(Arc::new(move |view, event, bounds, phase, cx| {
                if phase == DispatchPhase::Bubble
                    && event.button == button
                    && bounds.contains_point(&event.position)
                {
                    handler(view, event, cx)
                }
            }));
        self
    }

    fn on_mouse_up(
        mut self,
        button: MouseButton,
        handler: impl Fn(&mut Self::ViewState, &MouseUpEvent, &mut ViewContext<Self::ViewState>)
            + Send
            + Sync
            + 'static,
    ) -> Self
    where
        Self: Sized,
    {
        self.listeners()
            .mouse_up
            .push(Arc::new(move |view, event, bounds, phase, cx| {
                if phase == DispatchPhase::Bubble
                    && event.button == button
                    && bounds.contains_point(&event.position)
                {
                    handler(view, event, cx)
                }
            }));
        self
    }

    fn on_mouse_down_out(
        mut self,
        button: MouseButton,
        handler: impl Fn(&mut Self::ViewState, &MouseDownEvent, &mut ViewContext<Self::ViewState>)
            + Send
            + Sync
            + 'static,
    ) -> Self
    where
        Self: Sized,
    {
        self.listeners()
            .mouse_down
            .push(Arc::new(move |view, event, bounds, phase, cx| {
                if phase == DispatchPhase::Capture
                    && event.button == button
                    && !bounds.contains_point(&event.position)
                {
                    handler(view, event, cx)
                }
            }));
        self
    }

    fn on_mouse_up_out(
        mut self,
        button: MouseButton,
        handler: impl Fn(&mut Self::ViewState, &MouseUpEvent, &mut ViewContext<Self::ViewState>)
            + Send
            + Sync
            + 'static,
    ) -> Self
    where
        Self: Sized,
    {
        self.listeners()
            .mouse_up
            .push(Arc::new(move |view, event, bounds, phase, cx| {
                if phase == DispatchPhase::Capture
                    && event.button == button
                    && !bounds.contains_point(&event.position)
                {
                    handler(view, event, cx);
                }
            }));
        self
    }

    fn on_mouse_move(
        mut self,
        handler: impl Fn(&mut Self::ViewState, &MouseMoveEvent, &mut ViewContext<Self::ViewState>)
            + Send
            + Sync
            + 'static,
    ) -> Self
    where
        Self: Sized,
    {
        self.listeners()
            .mouse_move
            .push(Arc::new(move |view, event, bounds, phase, cx| {
                if phase == DispatchPhase::Bubble && bounds.contains_point(&event.position) {
                    handler(view, event, cx);
                }
            }));
        self
    }

    fn on_scroll_wheel(
        mut self,
        handler: impl Fn(&mut Self::ViewState, &ScrollWheelEvent, &mut ViewContext<Self::ViewState>)
            + Send
            + Sync
            + 'static,
    ) -> Self
    where
        Self: Sized,
    {
        self.listeners()
            .scroll_wheel
            .push(Arc::new(move |view, event, bounds, phase, cx| {
                if phase == DispatchPhase::Bubble && bounds.contains_point(&event.position) {
                    handler(view, event, cx);
                }
            }));
        self
    }

    fn on_key_down(
        mut self,
        listener: impl Fn(
                &mut Self::ViewState,
                &KeyDownEvent,
                DispatchPhase,
                &mut ViewContext<Self::ViewState>,
            ) + Send
            + Sync
            + 'static,
    ) -> Self
    where
        Self: Sized,
    {
        self.listeners().key.push((
            TypeId::of::<KeyDownEvent>(),
            Arc::new(move |view, event, _, phase, cx| {
                let event = event.downcast_ref().unwrap();
                listener(view, event, phase, cx);
                None
            }),
        ));
        self
    }

    fn on_key_up(
        mut self,
        listener: impl Fn(&mut Self::ViewState, &KeyUpEvent, DispatchPhase, &mut ViewContext<Self::ViewState>)
            + Send
            + Sync
            + 'static,
    ) -> Self
    where
        Self: Sized,
    {
        self.listeners().key.push((
            TypeId::of::<KeyUpEvent>(),
            Arc::new(move |view, event, _, phase, cx| {
                let event = event.downcast_ref().unwrap();
                listener(view, event, phase, cx);
                None
            }),
        ));
        self
    }

    fn on_action<A: 'static>(
        mut self,
        listener: impl Fn(&mut Self::ViewState, &A, DispatchPhase, &mut ViewContext<Self::ViewState>)
            + Send
            + Sync
            + 'static,
    ) -> Self
    where
        Self: Sized,
    {
        self.listeners().key.push((
            TypeId::of::<A>(),
            Arc::new(move |view, event, _, phase, cx| {
                let event = event.downcast_ref().unwrap();
                listener(view, event, phase, cx);
                None
            }),
        ));
        self
    }
}

pub trait Click: Interactive {
    fn on_click(
        mut self,
        handler: impl Fn(&mut Self::ViewState, &MouseClickEvent, &mut ViewContext<Self::ViewState>)
            + Send
            + Sync
            + 'static,
    ) -> Self
    where
        Self: Sized,
    {
        self.listeners()
            .mouse_click
            .push(Arc::new(move |view, event, cx| handler(view, event, cx)));
        self
    }
}
