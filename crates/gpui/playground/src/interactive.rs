use gpui::{platform::MouseMovedEvent, EventContext};
use smallvec::SmallVec;
use std::rc::Rc;

pub trait Interactive<V: 'static> {
    fn interaction_handlers(&mut self) -> &mut InteractionHandlers<V>;

    fn on_mouse_move<H>(mut self, handler: H) -> Self
    where
        H: 'static + Fn(&mut V, &MouseMovedEvent, bool, &mut EventContext<V>),
        Self: Sized,
    {
        self.interaction_handlers()
            .mouse_moved
            .push(Rc::new(move |view, event, hit_test, cx| {
                handler(view, event, hit_test, cx);
                cx.bubble
            }));
        self
    }
}

pub struct InteractionHandlers<V: 'static> {
    mouse_moved:
        SmallVec<[Rc<dyn Fn(&mut V, &MouseMovedEvent, bool, &mut EventContext<V>) -> bool>; 2]>,
}

impl<V> Default for InteractionHandlers<V> {
    fn default() -> Self {
        Self {
            mouse_moved: Default::default(),
        }
    }
}
