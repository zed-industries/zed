use std::{any::TypeId, fmt::Debug, mem::Discriminant, rc::Rc};

use collections::HashMap;

use pathfinder_geometry::rect::RectF;

use crate::{EventContext, MouseButton};

use super::{
    mouse_event::{
        MouseClick, MouseDown, MouseDownOut, MouseDrag, MouseEvent, MouseHover, MouseMove, MouseUp,
        MouseUpOut,
    },
    MouseScrollWheel,
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

    pub fn on_down(
        mut self,
        button: MouseButton,
        handler: impl Fn(MouseDown, &mut EventContext) + 'static,
    ) -> Self {
        self.handlers = self.handlers.on_down(button, handler);
        self
    }

    pub fn on_up(
        mut self,
        button: MouseButton,
        handler: impl Fn(MouseUp, &mut EventContext) + 'static,
    ) -> Self {
        self.handlers = self.handlers.on_up(button, handler);
        self
    }

    pub fn on_click(
        mut self,
        button: MouseButton,
        handler: impl Fn(MouseClick, &mut EventContext) + 'static,
    ) -> Self {
        self.handlers = self.handlers.on_click(button, handler);
        self
    }

    pub fn on_down_out(
        mut self,
        button: MouseButton,
        handler: impl Fn(MouseDownOut, &mut EventContext) + 'static,
    ) -> Self {
        self.handlers = self.handlers.on_down_out(button, handler);
        self
    }

    pub fn on_up_out(
        mut self,
        button: MouseButton,
        handler: impl Fn(MouseUpOut, &mut EventContext) + 'static,
    ) -> Self {
        self.handlers = self.handlers.on_up_out(button, handler);
        self
    }

    pub fn on_drag(
        mut self,
        button: MouseButton,
        handler: impl Fn(MouseDrag, &mut EventContext) + 'static,
    ) -> Self {
        self.handlers = self.handlers.on_drag(button, handler);
        self
    }

    pub fn on_hover(mut self, handler: impl Fn(MouseHover, &mut EventContext) + 'static) -> Self {
        self.handlers = self.handlers.on_hover(handler);
        self
    }

    pub fn on_move(mut self, handler: impl Fn(MouseMove, &mut EventContext) + 'static) -> Self {
        self.handlers = self.handlers.on_move(handler);
        self
    }

    pub fn on_scroll(
        mut self,
        handler: impl Fn(MouseScrollWheel, &mut EventContext) + 'static,
    ) -> Self {
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

#[derive(Clone, Default)]
pub struct HandlerSet {
    #[allow(clippy::type_complexity)]
    pub set: HashMap<
        (Discriminant<MouseEvent>, Option<MouseButton>),
        Rc<dyn Fn(MouseEvent, &mut EventContext)>,
    >,
}

impl HandlerSet {
    pub fn capture_all() -> Self {
        #[allow(clippy::type_complexity)]
        let mut set: HashMap<
            (Discriminant<MouseEvent>, Option<MouseButton>),
            Rc<dyn Fn(MouseEvent, &mut EventContext)>,
        > = Default::default();

        set.insert((MouseEvent::move_disc(), None), Rc::new(|_, _| {}));
        set.insert((MouseEvent::hover_disc(), None), Rc::new(|_, _| {}));
        for button in MouseButton::all() {
            set.insert((MouseEvent::drag_disc(), Some(button)), Rc::new(|_, _| {}));
            set.insert((MouseEvent::down_disc(), Some(button)), Rc::new(|_, _| {}));
            set.insert((MouseEvent::up_disc(), Some(button)), Rc::new(|_, _| {}));
            set.insert((MouseEvent::click_disc(), Some(button)), Rc::new(|_, _| {}));
            set.insert(
                (MouseEvent::down_out_disc(), Some(button)),
                Rc::new(|_, _| {}),
            );
            set.insert(
                (MouseEvent::up_out_disc(), Some(button)),
                Rc::new(|_, _| {}),
            );
        }
        set.insert((MouseEvent::scroll_wheel_disc(), None), Rc::new(|_, _| {}));

        HandlerSet { set }
    }

    pub fn get(
        &self,
        key: &(Discriminant<MouseEvent>, Option<MouseButton>),
    ) -> Option<Rc<dyn Fn(MouseEvent, &mut EventContext)>> {
        self.set.get(key).cloned()
    }

    pub fn contains_handler(
        &self,
        event: Discriminant<MouseEvent>,
        button: Option<MouseButton>,
    ) -> bool {
        self.set.contains_key(&(event, button))
    }

    pub fn on_move(mut self, handler: impl Fn(MouseMove, &mut EventContext) + 'static) -> Self {
        self.set.insert((MouseEvent::move_disc(), None),
            Rc::new(move |region_event, cx| {
                if let MouseEvent::Move(e) = region_event {
                    handler(e, cx);
                } else {
                    panic!(
                        "Mouse Region Event incorrectly called with mismatched event type. Expected MouseRegionEvent::Move, found {:?}", 
                        region_event);
                }
            }));
        self
    }

    pub fn on_down(
        mut self,
        button: MouseButton,
        handler: impl Fn(MouseDown, &mut EventContext) + 'static,
    ) -> Self {
        self.set.insert((MouseEvent::down_disc(), Some(button)),
            Rc::new(move |region_event, cx| {
                if let MouseEvent::Down(e) = region_event {
                    handler(e, cx);
                } else {
                    panic!(
                        "Mouse Region Event incorrectly called with mismatched event type. Expected MouseRegionEvent::Down, found {:?}", 
                        region_event);
                }
            }));
        self
    }

    pub fn on_up(
        mut self,
        button: MouseButton,
        handler: impl Fn(MouseUp, &mut EventContext) + 'static,
    ) -> Self {
        self.set.insert((MouseEvent::up_disc(), Some(button)),
            Rc::new(move |region_event, cx| {
                if let MouseEvent::Up(e) = region_event {
                    handler(e, cx);
                } else {
                    panic!(
                        "Mouse Region Event incorrectly called with mismatched event type. Expected MouseRegionEvent::Up, found {:?}", 
                        region_event);
                }
            }));
        self
    }

    pub fn on_click(
        mut self,
        button: MouseButton,
        handler: impl Fn(MouseClick, &mut EventContext) + 'static,
    ) -> Self {
        self.set.insert((MouseEvent::click_disc(), Some(button)),
            Rc::new(move |region_event, cx| {
                if let MouseEvent::Click(e) = region_event {
                    handler(e, cx);
                } else {
                    panic!(
                        "Mouse Region Event incorrectly called with mismatched event type. Expected MouseRegionEvent::Click, found {:?}", 
                        region_event);
                }
            }));
        self
    }

    pub fn on_down_out(
        mut self,
        button: MouseButton,
        handler: impl Fn(MouseDownOut, &mut EventContext) + 'static,
    ) -> Self {
        self.set.insert((MouseEvent::down_out_disc(), Some(button)),
            Rc::new(move |region_event, cx| {
                if let MouseEvent::DownOut(e) = region_event {
                    handler(e, cx);
                } else {
                    panic!(
                        "Mouse Region Event incorrectly called with mismatched event type. Expected MouseRegionEvent::DownOut, found {:?}", 
                        region_event);
                }
            }));
        self
    }

    pub fn on_up_out(
        mut self,
        button: MouseButton,
        handler: impl Fn(MouseUpOut, &mut EventContext) + 'static,
    ) -> Self {
        self.set.insert((MouseEvent::up_out_disc(), Some(button)),
            Rc::new(move |region_event, cx| {
                if let MouseEvent::UpOut(e) = region_event {
                    handler(e, cx);
                } else {
                    panic!(
                        "Mouse Region Event incorrectly called with mismatched event type. Expected MouseRegionEvent::UpOut, found {:?}", 
                        region_event);
                }
            }));
        self
    }

    pub fn on_drag(
        mut self,
        button: MouseButton,
        handler: impl Fn(MouseDrag, &mut EventContext) + 'static,
    ) -> Self {
        self.set.insert((MouseEvent::drag_disc(), Some(button)),
            Rc::new(move |region_event, cx| {
                if let MouseEvent::Drag(e) = region_event {
                    handler(e, cx);
                } else {
                    panic!(
                        "Mouse Region Event incorrectly called with mismatched event type. Expected MouseRegionEvent::Drag, found {:?}", 
                        region_event);
                }
            }));
        self
    }

    pub fn on_hover(mut self, handler: impl Fn(MouseHover, &mut EventContext) + 'static) -> Self {
        self.set.insert((MouseEvent::hover_disc(), None),
            Rc::new(move |region_event, cx| {
                if let MouseEvent::Hover(e) = region_event {
                    handler(e, cx);
                } else {
                    panic!(
                        "Mouse Region Event incorrectly called with mismatched event type. Expected MouseRegionEvent::Hover, found {:?}", 
                        region_event);
                }
            }));
        self
    }

    pub fn on_scroll(
        mut self,
        handler: impl Fn(MouseScrollWheel, &mut EventContext) + 'static,
    ) -> Self {
        self.set.insert((MouseEvent::scroll_wheel_disc(), None),
            Rc::new(move |region_event, cx| {
                if let MouseEvent::ScrollWheel(e) = region_event {
                    handler(e, cx);
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
