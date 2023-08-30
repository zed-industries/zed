use gpui::{
    geometry::rect::RectF,
    platform::{MouseButton, MouseButtonEvent},
    EventContext,
};
use smallvec::SmallVec;
use std::{cell::Cell, rc::Rc};

use crate::element::PaintContext;

pub trait Interactive<V: 'static> {
    fn interaction_handlers(&mut self) -> &mut InteractionHandlers<V>;

    fn on_mouse_down(
        mut self,
        button: MouseButton,
        handler: impl Fn(&mut V, &MouseButtonEvent, &mut EventContext<V>) + 'static,
    ) -> Self
    where
        Self: Sized,
    {
        self.interaction_handlers()
            .mouse_down
            .push(Rc::new(move |view, event, cx| {
                if event.button == button {
                    handler(view, event, cx)
                }
            }));
        self
    }

    fn on_mouse_up(
        mut self,
        button: MouseButton,
        handler: impl Fn(&mut V, &MouseButtonEvent, &mut EventContext<V>) + 'static,
    ) -> Self
    where
        Self: Sized,
    {
        self.interaction_handlers()
            .mouse_up
            .push(Rc::new(move |view, event, cx| {
                if event.button == button {
                    handler(view, event, cx)
                }
            }));
        self
    }

    fn on_mouse_down_out(
        mut self,
        button: MouseButton,
        handler: impl Fn(&mut V, &MouseButtonEvent, &mut EventContext<V>) + 'static,
    ) -> Self
    where
        Self: Sized,
    {
        self.interaction_handlers()
            .mouse_down_out
            .push(Rc::new(move |view, event, cx| {
                if event.button == button {
                    handler(view, event, cx)
                }
            }));
        self
    }

    fn on_mouse_up_out(
        mut self,
        button: MouseButton,
        handler: impl Fn(&mut V, &MouseButtonEvent, &mut EventContext<V>) + 'static,
    ) -> Self
    where
        Self: Sized,
    {
        self.interaction_handlers()
            .mouse_up_out
            .push(Rc::new(move |view, event, cx| {
                if event.button == button {
                    handler(view, event, cx)
                }
            }));
        self
    }

    fn on_click(
        self,
        button: MouseButton,
        handler: impl Fn(&mut V, &MouseButtonEvent, &mut EventContext<V>) + 'static,
    ) -> Self
    where
        Self: Sized,
    {
        let pressed = Rc::new(Cell::new(false));
        self.on_mouse_down(button, {
            let pressed = pressed.clone();
            move |_, _, _| {
                pressed.set(true);
            }
        })
        .on_mouse_up_out(button, {
            let pressed = pressed.clone();
            move |_, _, _| {
                pressed.set(false);
            }
        })
        .on_mouse_up(button, move |view, event, cx| {
            if pressed.get() {
                pressed.set(false);
                handler(view, event, cx);
            }
        })
    }
}

pub struct InteractionHandlers<V: 'static> {
    mouse_down: SmallVec<[Rc<dyn Fn(&mut V, &MouseButtonEvent, &mut EventContext<V>)>; 2]>,
    mouse_down_out: SmallVec<[Rc<dyn Fn(&mut V, &MouseButtonEvent, &mut EventContext<V>)>; 2]>,
    mouse_up: SmallVec<[Rc<dyn Fn(&mut V, &MouseButtonEvent, &mut EventContext<V>)>; 2]>,
    mouse_up_out: SmallVec<[Rc<dyn Fn(&mut V, &MouseButtonEvent, &mut EventContext<V>)>; 2]>,
}

impl<V: 'static> InteractionHandlers<V> {
    pub fn paint(&self, order: u32, bounds: RectF, cx: &mut PaintContext<V>) {
        for handler in self.mouse_down.iter().cloned() {
            cx.on_event(order, move |view, event: &MouseButtonEvent, cx| {
                if event.is_down && bounds.contains_point(event.position) {
                    handler(view, event, cx);
                }
            })
        }
        for handler in self.mouse_up.iter().cloned() {
            cx.on_event(order, move |view, event: &MouseButtonEvent, cx| {
                if !event.is_down && bounds.contains_point(event.position) {
                    handler(view, event, cx);
                }
            })
        }
        for handler in self.mouse_down_out.iter().cloned() {
            cx.on_event(order, move |view, event: &MouseButtonEvent, cx| {
                if event.is_down && !bounds.contains_point(event.position) {
                    handler(view, event, cx);
                }
            })
        }
        for handler in self.mouse_up_out.iter().cloned() {
            cx.on_event(order, move |view, event: &MouseButtonEvent, cx| {
                if !event.is_down && !bounds.contains_point(event.position) {
                    handler(view, event, cx);
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
            mouse_down_out: Default::default(),
            mouse_up_out: Default::default(),
        }
    }
}
