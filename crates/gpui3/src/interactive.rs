use crate::{
    Bounds, DispatchPhase, MouseButton, MouseDownEvent, MouseMoveEvent, MouseUpEvent, Pixels,
    ScrollWheelEvent, ViewContext,
};
use smallvec::SmallVec;
use std::sync::Arc;

pub trait Interactive<S: 'static + Send + Sync> {
    fn listeners(&mut self) -> &mut MouseEventListeners<S>;

    fn on_mouse_down(
        mut self,
        button: MouseButton,
        handler: impl Fn(&mut S, &MouseDownEvent, &mut ViewContext<S>) + Send + Sync + 'static,
    ) -> Self
    where
        Self: Sized,
    {
        self.listeners()
            .mouse_down
            .push(Arc::new(move |view, event, bounds, phase, cx| {
                if phase == DispatchPhase::Bubble
                    && event.button == button
                    && bounds.contains_point(event.position)
                {
                    handler(view, event, cx)
                }
            }));
        self
    }

    fn on_mouse_up(
        mut self,
        button: MouseButton,
        handler: impl Fn(&mut S, &MouseUpEvent, &mut ViewContext<S>) + Send + Sync + 'static,
    ) -> Self
    where
        Self: Sized,
    {
        self.listeners()
            .mouse_up
            .push(Arc::new(move |view, event, bounds, phase, cx| {
                if phase == DispatchPhase::Bubble
                    && event.button == button
                    && bounds.contains_point(event.position)
                {
                    handler(view, event, cx)
                }
            }));
        self
    }

    fn on_mouse_down_out(
        mut self,
        button: MouseButton,
        handler: impl Fn(&mut S, &MouseDownEvent, &mut ViewContext<S>) + Send + Sync + 'static,
    ) -> Self
    where
        Self: Sized,
    {
        self.listeners()
            .mouse_down
            .push(Arc::new(move |view, event, bounds, phase, cx| {
                if phase == DispatchPhase::Capture
                    && event.button == button
                    && !bounds.contains_point(event.position)
                {
                    handler(view, event, cx)
                }
            }));
        self
    }

    fn on_mouse_up_out(
        mut self,
        button: MouseButton,
        handler: impl Fn(&mut S, &MouseUpEvent, &mut ViewContext<S>) + Send + Sync + 'static,
    ) -> Self
    where
        Self: Sized,
    {
        self.listeners()
            .mouse_up
            .push(Arc::new(move |view, event, bounds, phase, cx| {
                if phase == DispatchPhase::Capture
                    && event.button == button
                    && !bounds.contains_point(event.position)
                {
                    handler(view, event, cx);
                }
            }));
        self
    }

    fn on_mouse_move(
        mut self,
        handler: impl Fn(&mut S, &MouseMoveEvent, &mut ViewContext<S>) + Send + Sync + 'static,
    ) -> Self
    where
        Self: Sized,
    {
        self.listeners()
            .mouse_move
            .push(Arc::new(move |view, event, bounds, phase, cx| {
                if phase == DispatchPhase::Bubble && bounds.contains_point(event.position) {
                    handler(view, event, cx);
                }
            }));
        self
    }

    fn on_scroll_wheel(
        mut self,
        handler: impl Fn(&mut S, &ScrollWheelEvent, &mut ViewContext<S>) + Send + Sync + 'static,
    ) -> Self
    where
        Self: Sized,
    {
        self.listeners()
            .scroll_wheel
            .push(Arc::new(move |view, event, bounds, phase, cx| {
                if phase == DispatchPhase::Bubble && bounds.contains_point(event.position) {
                    handler(view, event, cx);
                }
            }));
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
    mouse_down: SmallVec<[MouseDownHandler<V>; 2]>,
    mouse_up: SmallVec<[MouseUpHandler<V>; 2]>,
    mouse_move: SmallVec<[MouseMoveHandler<V>; 2]>,
    scroll_wheel: SmallVec<[ScrollWheelHandler<V>; 2]>,
}

impl<S: Send + Sync + 'static> MouseEventListeners<S> {
    pub fn paint(&self, bounds: Bounds<Pixels>, cx: &mut ViewContext<S>) {
        for handler in self.mouse_down.iter().cloned() {
            cx.on_mouse_event(move |view, event: &MouseDownEvent, phase, cx| {
                handler(view, event, &bounds, phase, cx);
            })
        }
        for handler in self.mouse_up.iter().cloned() {
            cx.on_mouse_event(move |view, event: &MouseUpEvent, phase, cx| {
                handler(view, event, &bounds, phase, cx);
            })
        }
        for handler in self.mouse_move.iter().cloned() {
            cx.on_mouse_event(move |view, event: &MouseMoveEvent, phase, cx| {
                handler(view, event, &bounds, phase, cx);
            })
        }

        for handler in self.scroll_wheel.iter().cloned() {
            cx.on_mouse_event(move |view, event: &ScrollWheelEvent, phase, cx| {
                handler(view, event, &bounds, phase, cx);
            })
        }
    }
}

impl<V> Default for MouseEventListeners<V> {
    fn default() -> Self {
        Self {
            mouse_down: Default::default(),
            mouse_up: Default::default(),
            mouse_move: Default::default(),
            scroll_wheel: Default::default(),
        }
    }
}
