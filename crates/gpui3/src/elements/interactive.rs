use crate::{
    Bounds, DispatchPhase, MouseButton, MouseDownEvent, MouseUpEvent, Pixels, ViewContext,
};
use parking_lot::Mutex;
use smallvec::SmallVec;
use std::sync::Arc;

pub trait Interactive<S: 'static + Send + Sync> {
    fn interaction_listeners(&mut self) -> &mut InteractionHandlers<S>;

    fn on_mouse_down(
        mut self,
        button: MouseButton,
        handler: impl Fn(&mut S, &MouseDownEvent, &mut ViewContext<S>) + Send + Sync + 'static,
    ) -> Self
    where
        Self: Sized,
    {
        self.interaction_listeners()
            .mouse_down
            .push(Arc::new(move |view, event, phase, cx| {
                if phase == DispatchPhase::Bubble && event.button == button {
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
        self.interaction_listeners()
            .mouse_up
            .push(Arc::new(move |view, event, phase, cx| {
                if phase == DispatchPhase::Bubble && event.button == button {
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
        self.interaction_listeners()
            .mouse_down
            .push(Arc::new(move |view, event, phase, cx| {
                if phase == DispatchPhase::Capture && event.button == button {
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
        self.interaction_listeners()
            .mouse_up
            .push(Arc::new(move |view, event, phase, cx| {
                if event.button == button && phase == DispatchPhase::Capture {
                    handler(view, event, cx);
                }
            }));
        self
    }

    fn on_click(
        self,
        button: MouseButton,
        handler: impl Fn(&mut S, &MouseDownEvent, &MouseUpEvent, &mut ViewContext<S>)
            + Send
            + Sync
            + 'static,
    ) -> Self
    where
        Self: Sized,
    {
        let down_event = Arc::new(Mutex::new(None));
        self.on_mouse_down(button, {
            let down_event = down_event.clone();
            move |_, event, _| {
                down_event.lock().replace(event.clone());
            }
        })
        .on_mouse_up_out(button, {
            let down_event = down_event.clone();
            move |_, _, _| {
                down_event.lock().take();
            }
        })
        .on_mouse_up(button, move |view, event, cx| {
            if let Some(down_event) = down_event.lock().take() {
                handler(view, &down_event, event, cx);
            }
        })
    }
}

type MouseDownHandler<V> = Arc<
    dyn Fn(&mut V, &MouseDownEvent, DispatchPhase, &mut ViewContext<V>) + Send + Sync + 'static,
>;
type MouseUpHandler<V> =
    Arc<dyn Fn(&mut V, &MouseUpEvent, DispatchPhase, &mut ViewContext<V>) + Send + Sync + 'static>;

pub struct InteractionHandlers<V: 'static> {
    mouse_down: SmallVec<[MouseDownHandler<V>; 2]>,
    mouse_up: SmallVec<[MouseUpHandler<V>; 2]>,
}

impl<S: Send + Sync + 'static> InteractionHandlers<S> {
    pub fn paint(&self, bounds: Bounds<Pixels>, cx: &mut ViewContext<S>) {
        for handler in self.mouse_down.iter().cloned() {
            cx.on_mouse_event(move |view, event: &MouseDownEvent, phase, cx| {
                if bounds.contains_point(event.position) {
                    handler(view, event, phase, cx);
                }
            })
        }
        for handler in self.mouse_up.iter().cloned() {
            cx.on_mouse_event(move |view, event: &MouseUpEvent, phase, cx| {
                if bounds.contains_point(event.position) {
                    handler(view, event, phase, cx);
                }
            })
        }
    }
}

impl<V> Default for InteractionHandlers<V> {
    fn default() -> Self {
        Self {
            mouse_down: Default::default(),
            mouse_up: Default::default(),
        }
    }
}
