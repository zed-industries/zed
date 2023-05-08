use crate::{platform::MouseButton, window::WindowContext, EventContext, View, ViewContext};
use collections::HashMap;
use pathfinder_geometry::rect::RectF;
use smallvec::SmallVec;
use std::{
    any::{Any, TypeId},
    fmt::Debug,
    mem::Discriminant,
    rc::Rc,
};

use super::{
    mouse_event::{
        MouseClick, MouseDown, MouseDownOut, MouseDrag, MouseEvent, MouseHover, MouseMove, MouseUp,
        MouseUpOut,
    },
    MouseMoveOut, MouseScrollWheel,
};

#[derive(Clone)]
pub struct MouseRegion {
    pub id: MouseRegionId,
    pub bounds: RectF,
    pub handlers: HandlerSet,
    pub hoverable: bool,
    pub notify_on_hover: bool,
    pub notify_on_click: bool,
}

impl MouseRegion {
    /// Region ID is used to track semantically equivalent mouse regions across render passes.
    /// e.g. if you have mouse handlers attached to a list item type, then each item of the list
    /// should pass a different (consistent) region_id. If you have one big region that covers your
    /// whole component, just pass the view_id again.
    pub fn new<Tag: 'static>(view_id: usize, region_id: usize, bounds: RectF) -> Self {
        Self::from_handlers::<Tag>(view_id, region_id, bounds, Default::default())
    }

    pub fn handle_all<Tag: 'static>(view_id: usize, region_id: usize, bounds: RectF) -> Self {
        Self::from_handlers::<Tag>(view_id, region_id, bounds, HandlerSet::capture_all())
    }

    pub fn from_handlers<Tag: 'static>(
        view_id: usize,
        region_id: usize,
        bounds: RectF,
        handlers: HandlerSet,
    ) -> Self {
        Self {
            id: MouseRegionId {
                view_id,
                tag: TypeId::of::<Tag>(),
                region_id,
                #[cfg(debug_assertions)]
                tag_type_name: std::any::type_name::<Tag>(),
            },
            bounds,
            handlers,
            hoverable: true,
            notify_on_hover: false,
            notify_on_click: false,
        }
    }

    pub fn on_down<V, F>(mut self, button: MouseButton, handler: F) -> Self
    where
        V: View,
        F: Fn(MouseDown, &mut V, &mut EventContext<V>) + 'static,
    {
        self.handlers = self.handlers.on_down(button, handler);
        self
    }

    pub fn on_up<V, F>(mut self, button: MouseButton, handler: F) -> Self
    where
        V: View,
        F: Fn(MouseUp, &mut V, &mut EventContext<V>) + 'static,
    {
        self.handlers = self.handlers.on_up(button, handler);
        self
    }

    pub fn on_click<V, F>(mut self, button: MouseButton, handler: F) -> Self
    where
        V: View,
        F: Fn(MouseClick, &mut V, &mut EventContext<V>) + 'static,
    {
        self.handlers = self.handlers.on_click(button, handler);
        self
    }

    pub fn on_down_out<V, F>(mut self, button: MouseButton, handler: F) -> Self
    where
        V: View,
        F: Fn(MouseDownOut, &mut V, &mut EventContext<V>) + 'static,
    {
        self.handlers = self.handlers.on_down_out(button, handler);
        self
    }

    pub fn on_up_out<V, F>(mut self, button: MouseButton, handler: F) -> Self
    where
        V: View,
        F: Fn(MouseUpOut, &mut V, &mut EventContext<V>) + 'static,
    {
        self.handlers = self.handlers.on_up_out(button, handler);
        self
    }

    pub fn on_drag<V, F>(mut self, button: MouseButton, handler: F) -> Self
    where
        V: View,
        F: Fn(MouseDrag, &mut V, &mut EventContext<V>) + 'static,
    {
        self.handlers = self.handlers.on_drag(button, handler);
        self
    }

    pub fn on_hover<V, F>(mut self, handler: F) -> Self
    where
        V: View,
        F: Fn(MouseHover, &mut V, &mut EventContext<V>) + 'static,
    {
        self.handlers = self.handlers.on_hover(handler);
        self
    }

    pub fn on_move<V, F>(mut self, handler: F) -> Self
    where
        V: View,
        F: Fn(MouseMove, &mut V, &mut EventContext<V>) + 'static,
    {
        self.handlers = self.handlers.on_move(handler);
        self
    }

    pub fn on_move_out<V, F>(mut self, handler: F) -> Self
    where
        V: View,
        F: Fn(MouseMoveOut, &mut V, &mut EventContext<V>) + 'static,
    {
        self.handlers = self.handlers.on_move_out(handler);
        self
    }

    pub fn on_scroll<V, F>(mut self, handler: F) -> Self
    where
        V: View,
        F: Fn(MouseScrollWheel, &mut V, &mut EventContext<V>) + 'static,
    {
        self.handlers = self.handlers.on_scroll(handler);
        self
    }

    pub fn with_hoverable(mut self, is_hoverable: bool) -> Self {
        self.hoverable = is_hoverable;
        self
    }

    pub fn with_notify_on_hover(mut self, notify: bool) -> Self {
        self.notify_on_hover = notify;
        self
    }

    pub fn with_notify_on_click(mut self, notify: bool) -> Self {
        self.notify_on_click = notify;
        self
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct MouseRegionId {
    view_id: usize,
    tag: TypeId,
    region_id: usize,
    #[cfg(debug_assertions)]
    tag_type_name: &'static str,
}

impl MouseRegionId {
    pub(crate) fn new<Tag: 'static>(view_id: usize, region_id: usize) -> Self {
        MouseRegionId {
            view_id,
            region_id,
            tag: TypeId::of::<Tag>(),
            #[cfg(debug_assertions)]
            tag_type_name: std::any::type_name::<Tag>(),
        }
    }

    pub fn view_id(&self) -> usize {
        self.view_id
    }

    #[cfg(debug_assertions)]
    pub fn tag_type_name(&self) -> &'static str {
        self.tag_type_name
    }
}

pub type HandlerCallback = Rc<dyn Fn(MouseEvent, &mut dyn Any, &mut WindowContext, usize) -> bool>;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct HandlerKey {
    event_kind: Discriminant<MouseEvent>,
    button: Option<MouseButton>,
}

impl HandlerKey {
    pub fn new(event_kind: Discriminant<MouseEvent>, button: Option<MouseButton>) -> HandlerKey {
        HandlerKey { event_kind, button }
    }
}

#[derive(Clone, Default)]
pub struct HandlerSet {
    set: HashMap<HandlerKey, SmallVec<[HandlerCallback; 1]>>,
}

impl HandlerSet {
    pub fn capture_all() -> Self {
        let mut set: HashMap<HandlerKey, SmallVec<[HandlerCallback; 1]>> = HashMap::default();

        set.insert(
            HandlerKey::new(MouseEvent::move_disc(), None),
            SmallVec::from_buf([Rc::new(|_, _, _, _| true)]),
        );
        set.insert(
            HandlerKey::new(MouseEvent::hover_disc(), None),
            SmallVec::from_buf([Rc::new(|_, _, _, _| true)]),
        );
        for button in MouseButton::all() {
            set.insert(
                HandlerKey::new(MouseEvent::drag_disc(), Some(button)),
                SmallVec::from_buf([Rc::new(|_, _, _, _| true)]),
            );
            set.insert(
                HandlerKey::new(MouseEvent::down_disc(), Some(button)),
                SmallVec::from_buf([Rc::new(|_, _, _, _| true)]),
            );
            set.insert(
                HandlerKey::new(MouseEvent::up_disc(), Some(button)),
                SmallVec::from_buf([Rc::new(|_, _, _, _| true)]),
            );
            set.insert(
                HandlerKey::new(MouseEvent::click_disc(), Some(button)),
                SmallVec::from_buf([Rc::new(|_, _, _, _| true)]),
            );
            set.insert(
                HandlerKey::new(MouseEvent::down_out_disc(), Some(button)),
                SmallVec::from_buf([Rc::new(|_, _, _, _| true)]),
            );
            set.insert(
                HandlerKey::new(MouseEvent::up_out_disc(), Some(button)),
                SmallVec::from_buf([Rc::new(|_, _, _, _| true)]),
            );
        }
        set.insert(
            HandlerKey::new(MouseEvent::scroll_wheel_disc(), None),
            SmallVec::from_buf([Rc::new(|_, _, _, _| true)]),
        );

        HandlerSet { set }
    }

    pub fn get(&self, key: &HandlerKey) -> Option<&[HandlerCallback]> {
        self.set.get(key).map(|vec| vec.as_slice())
    }

    pub fn contains(
        &self,
        discriminant: Discriminant<MouseEvent>,
        button: Option<MouseButton>,
    ) -> bool {
        self.set
            .contains_key(&HandlerKey::new(discriminant, button))
    }

    fn insert(
        &mut self,
        event_kind: Discriminant<MouseEvent>,
        button: Option<MouseButton>,
        callback: HandlerCallback,
    ) {
        use std::collections::hash_map::Entry;

        match self.set.entry(HandlerKey::new(event_kind, button)) {
            Entry::Occupied(mut vec) => {
                vec.get_mut().push(callback);
            }

            Entry::Vacant(entry) => {
                entry.insert(SmallVec::from_buf([callback]));
            }
        }
    }

    pub fn on_move<V, F>(mut self, handler: F) -> Self
    where
        V: View,
        F: Fn(MouseMove, &mut V, &mut EventContext<V>) + 'static,
    {
        self.insert(MouseEvent::move_disc(), None,
            Rc::new(move |region_event, view, cx, view_id| {
                if let MouseEvent::Move(e) = region_event {
                    let view = view.downcast_mut().unwrap();
                    let mut cx = ViewContext::mutable(cx, view_id);
                    let mut cx = EventContext::new(&mut cx);
                    handler(e, view, &mut cx);
                    cx.handled
                } else {
                    panic!(
                        "Mouse Region Event incorrectly called with mismatched event type. Expected MouseRegionEvent::Move, found {:?}",
                        region_event);
                }
            }));
        self
    }

    pub fn on_move_out<V, F>(mut self, handler: F) -> Self
    where
        V: View,
        F: Fn(MouseMoveOut, &mut V, &mut EventContext<V>) + 'static,
    {
        self.insert(MouseEvent::move_out_disc(), None,
            Rc::new(move |region_event, view, cx, view_id| {
                if let MouseEvent::MoveOut(e) = region_event {
                    let view = view.downcast_mut().unwrap();
                    let mut cx = ViewContext::<V>::mutable(cx, view_id);
                    let mut cx = EventContext::new(&mut cx);
                    handler(e, view, &mut cx);
                    cx.handled
                } else {
                    panic!(
                        "Mouse Region Event incorrectly called with mismatched event type. Expected MouseRegionEvent::MoveOut, found {:?}",
                        region_event);
                }
            }));
        self
    }

    pub fn on_down<V, F>(mut self, button: MouseButton, handler: F) -> Self
    where
        V: View,
        F: Fn(MouseDown, &mut V, &mut EventContext<V>) + 'static,
    {
        self.insert(MouseEvent::down_disc(), Some(button),
            Rc::new(move |region_event, view, cx, view_id| {
                if let MouseEvent::Down(e) = region_event {
                    let view = view.downcast_mut().unwrap();
                    let mut cx = ViewContext::mutable(cx, view_id);
                    let mut cx = EventContext::new(&mut cx);
                    handler(e, view, &mut cx);
                    cx.handled
                } else {
                    panic!(
                        "Mouse Region Event incorrectly called with mismatched event type. Expected MouseRegionEvent::Down, found {:?}",
                        region_event);
                }
            }));
        self
    }

    pub fn on_up<V, F>(mut self, button: MouseButton, handler: F) -> Self
    where
        V: View,
        F: Fn(MouseUp, &mut V, &mut EventContext<V>) + 'static,
    {
        self.insert(MouseEvent::up_disc(), Some(button),
            Rc::new(move |region_event, view, cx, view_id| {
                if let MouseEvent::Up(e) = region_event {
                    let view = view.downcast_mut().unwrap();
                    let mut cx = ViewContext::mutable(cx, view_id);
                    let mut cx = EventContext::new(&mut cx);
                    handler(e, view, &mut cx);
                    cx.handled
                } else {
                    panic!(
                        "Mouse Region Event incorrectly called with mismatched event type. Expected MouseRegionEvent::Up, found {:?}",
                        region_event);
                }
            }));
        self
    }

    pub fn on_click<V, F>(mut self, button: MouseButton, handler: F) -> Self
    where
        V: View,
        F: Fn(MouseClick, &mut V, &mut EventContext<V>) + 'static,
    {
        self.insert(MouseEvent::click_disc(), Some(button),
            Rc::new(move |region_event, view, cx, view_id| {
                if let MouseEvent::Click(e) = region_event {
                    let view = view.downcast_mut().unwrap();
                    let mut cx = ViewContext::mutable(cx, view_id);
                    let mut cx = EventContext::new(&mut cx);
                    handler(e, view, &mut cx);
                    cx.handled
                } else {
                    panic!(
                        "Mouse Region Event incorrectly called with mismatched event type. Expected MouseRegionEvent::Click, found {:?}",
                        region_event);
                }
            }));
        self
    }

    pub fn on_down_out<V, F>(mut self, button: MouseButton, handler: F) -> Self
    where
        V: View,
        F: Fn(MouseDownOut, &mut V, &mut EventContext<V>) + 'static,
    {
        self.insert(MouseEvent::down_out_disc(), Some(button),
            Rc::new(move |region_event, view, cx, view_id| {
                if let MouseEvent::DownOut(e) = region_event {
                    let view = view.downcast_mut().unwrap();
                    let mut cx = ViewContext::mutable(cx, view_id);
                    let mut cx = EventContext::new(&mut cx);
                    handler(e, view, &mut cx);
                    cx.handled
                } else {
                    panic!(
                        "Mouse Region Event incorrectly called with mismatched event type. Expected MouseRegionEvent::DownOut, found {:?}",
                        region_event);
                }
            }));
        self
    }

    pub fn on_up_out<V, F>(mut self, button: MouseButton, handler: F) -> Self
    where
        V: View,
        F: Fn(MouseUpOut, &mut V, &mut EventContext<V>) + 'static,
    {
        self.insert(MouseEvent::up_out_disc(), Some(button),
            Rc::new(move |region_event, view, cx, view_id| {
                if let MouseEvent::UpOut(e) = region_event {
                    let view = view.downcast_mut().unwrap();
                    let mut cx = ViewContext::mutable(cx, view_id);
                    let mut cx = EventContext::new(&mut cx);
                    handler(e, view, &mut cx);
                    cx.handled
                } else {
                    panic!(
                        "Mouse Region Event incorrectly called with mismatched event type. Expected MouseRegionEvent::UpOut, found {:?}",
                        region_event);
                }
            }));
        self
    }

    pub fn on_drag<V, F>(mut self, button: MouseButton, handler: F) -> Self
    where
        V: View,
        F: Fn(MouseDrag, &mut V, &mut EventContext<V>) + 'static,
    {
        self.insert(MouseEvent::drag_disc(), Some(button),
            Rc::new(move |region_event, view, cx, view_id| {
                if let MouseEvent::Drag(e) = region_event {
                    let view = view.downcast_mut().unwrap();
                    let mut cx = ViewContext::mutable(cx, view_id);
                    let mut cx = EventContext::new(&mut cx);
                    handler(e, view, &mut cx);
                    cx.handled
                } else {
                    panic!(
                        "Mouse Region Event incorrectly called with mismatched event type. Expected MouseRegionEvent::Drag, found {:?}",
                        region_event);
                }
            }));
        self
    }

    pub fn on_hover<V, F>(mut self, handler: F) -> Self
    where
        V: View,
        F: Fn(MouseHover, &mut V, &mut EventContext<V>) + 'static,
    {
        self.insert(MouseEvent::hover_disc(), None,
            Rc::new(move |region_event, view, cx, view_id| {
                if let MouseEvent::Hover(e) = region_event {
                    let view = view.downcast_mut().unwrap();
                    let mut cx = ViewContext::mutable(cx, view_id);
                    let mut cx = EventContext::new(&mut cx);
                    handler(e, view, &mut cx);
                    cx.handled
                } else {
                    panic!(
                        "Mouse Region Event incorrectly called with mismatched event type. Expected MouseRegionEvent::Hover, found {:?}",
                        region_event);
                }
            }));
        self
    }

    pub fn on_scroll<V, F>(mut self, handler: F) -> Self
    where
        V: View,
        F: Fn(MouseScrollWheel, &mut V, &mut EventContext<V>) + 'static,
    {
        self.insert(MouseEvent::scroll_wheel_disc(), None,
            Rc::new(move |region_event, view, cx, view_id| {
                if let MouseEvent::ScrollWheel(e) = region_event {
                    let view = view.downcast_mut().unwrap();
                    let mut cx = ViewContext::mutable(cx, view_id);
                    let mut cx = EventContext::new(&mut cx);
                    handler(e, view, &mut cx);
                    cx.handled
                } else {
                    panic!(
                        "Mouse Region Event incorrectly called with mismatched event type. Expected MouseRegionEvent::ScrollWheel, found {:?}",
                        region_event
                    );
                }
            }));
        self
    }
}
