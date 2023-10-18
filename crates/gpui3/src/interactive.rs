use smallvec::SmallVec;

use crate::{
    Bounds, DispatchPhase, Element, KeyDownEvent, KeyUpEvent, MouseButton, MouseDownEvent,
    MouseMoveEvent, MouseUpEvent, Pixels, ScrollWheelEvent, ViewContext,
};
use std::sync::Arc;

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

type MouseDownListener<V> = Arc<
    dyn Fn(&mut V, &MouseDownEvent, &Bounds<Pixels>, DispatchPhase, &mut ViewContext<V>)
        + Send
        + Sync
        + 'static,
>;
type MouseUpListener<V> = Arc<
    dyn Fn(&mut V, &MouseUpEvent, &Bounds<Pixels>, DispatchPhase, &mut ViewContext<V>)
        + Send
        + Sync
        + 'static,
>;
type MouseClickListener<V> =
    Arc<dyn Fn(&mut V, &MouseClickEvent, &mut ViewContext<V>) + Send + Sync + 'static>;

type MouseMoveListener<V> = Arc<
    dyn Fn(&mut V, &MouseMoveEvent, &Bounds<Pixels>, DispatchPhase, &mut ViewContext<V>)
        + Send
        + Sync
        + 'static,
>;

type ScrollWheelListener<V> = Arc<
    dyn Fn(&mut V, &ScrollWheelEvent, &Bounds<Pixels>, DispatchPhase, &mut ViewContext<V>)
        + Send
        + Sync
        + 'static,
>;

pub type KeyDownListener<V> =
    Arc<dyn Fn(&mut V, &KeyDownEvent, DispatchPhase, &mut ViewContext<V>) + Send + Sync + 'static>;

pub type KeyUpListener<V> =
    Arc<dyn Fn(&mut V, &KeyUpEvent, DispatchPhase, &mut ViewContext<V>) + Send + Sync + 'static>;

pub struct EventListeners<V: 'static> {
    pub mouse_down: SmallVec<[MouseDownListener<V>; 2]>,
    pub mouse_up: SmallVec<[MouseUpListener<V>; 2]>,
    pub mouse_click: SmallVec<[MouseClickListener<V>; 2]>,
    pub mouse_move: SmallVec<[MouseMoveListener<V>; 2]>,
    pub scroll_wheel: SmallVec<[ScrollWheelListener<V>; 2]>,
    pub key_down: SmallVec<[KeyDownListener<V>; 2]>,
    pub key_up: SmallVec<[KeyUpListener<V>; 2]>,
}

impl<V> Default for EventListeners<V> {
    fn default() -> Self {
        Self {
            mouse_down: SmallVec::new(),
            mouse_up: SmallVec::new(),
            mouse_click: SmallVec::new(),
            mouse_move: SmallVec::new(),
            scroll_wheel: SmallVec::new(),
            key_down: SmallVec::new(),
            key_up: SmallVec::new(),
        }
    }
}

pub struct MouseClickEvent {
    pub down: MouseDownEvent,
    pub up: MouseUpEvent,
}
