use smallvec::SmallVec;

use crate::{
    Bounds, DispatchPhase, Element, MouseButton, MouseDownEvent, MouseMoveEvent, MouseUpEvent,
    Pixels, ScrollWheelEvent, ViewContext,
};
use std::sync::Arc;

pub trait Interactive: Element {
    fn listeners(&mut self) -> &mut MouseEventListeners<Self::ViewState>;

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

type MouseDownHandler<V> = Arc<
    dyn Fn(&mut V, &MouseDownEvent, &Bounds<Pixels>, DispatchPhase, &mut ViewContext<V>)
        + Send
        + Sync
        + 'static,
>;
type MouseUpHandler<V> = Arc<
    dyn Fn(&mut V, &MouseUpEvent, &Bounds<Pixels>, DispatchPhase, &mut ViewContext<V>)
        + Send
        + Sync
        + 'static,
>;
type MouseClickHandler<V> =
    Arc<dyn Fn(&mut V, &MouseClickEvent, &mut ViewContext<V>) + Send + Sync + 'static>;

type MouseMoveHandler<V> = Arc<
    dyn Fn(&mut V, &MouseMoveEvent, &Bounds<Pixels>, DispatchPhase, &mut ViewContext<V>)
        + Send
        + Sync
        + 'static,
>;
type ScrollWheelHandler<V> = Arc<
    dyn Fn(&mut V, &ScrollWheelEvent, &Bounds<Pixels>, DispatchPhase, &mut ViewContext<V>)
        + Send
        + Sync
        + 'static,
>;

pub struct MouseEventListeners<V: 'static> {
    pub mouse_down: SmallVec<[MouseDownHandler<V>; 2]>,
    pub mouse_up: SmallVec<[MouseUpHandler<V>; 2]>,
    pub mouse_click: SmallVec<[MouseClickHandler<V>; 2]>,
    pub mouse_move: SmallVec<[MouseMoveHandler<V>; 2]>,
    pub scroll_wheel: SmallVec<[ScrollWheelHandler<V>; 2]>,
}

impl<V> Default for MouseEventListeners<V> {
    fn default() -> Self {
        Self {
            mouse_down: SmallVec::new(),
            mouse_up: SmallVec::new(),
            mouse_click: SmallVec::new(),
            mouse_move: SmallVec::new(),
            scroll_wheel: SmallVec::new(),
        }
    }
}

pub struct MouseClickEvent {
    pub down: MouseDownEvent,
    pub up: MouseUpEvent,
}
