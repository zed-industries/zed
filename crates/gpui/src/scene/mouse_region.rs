use std::{any::TypeId, mem::Discriminant, rc::Rc};

use collections::HashMap;

use pathfinder_geometry::rect::RectF;

use crate::{EventContext, MouseButton};

use super::mouse_region_event::{
    ClickRegionEvent, DownOutRegionEvent, DownRegionEvent, DragRegionEvent, HoverRegionEvent,
    MouseRegionEvent, MoveRegionEvent, UpOutRegionEvent, UpRegionEvent,
};

#[derive(Clone, Default)]
pub struct MouseRegion {
    pub view_id: usize,
    pub discriminant: Option<(TypeId, usize)>,
    pub bounds: RectF,
    pub handlers: HandlerSet,
}

impl MouseRegion {
    pub fn new(view_id: usize, discriminant: Option<(TypeId, usize)>, bounds: RectF) -> Self {
        Self::from_handlers(view_id, discriminant, bounds, Default::default())
    }

    pub fn from_handlers(
        view_id: usize,
        discriminant: Option<(TypeId, usize)>,
        bounds: RectF,
        handlers: HandlerSet,
    ) -> Self {
        Self {
            view_id,
            discriminant,
            bounds,
            handlers,
        }
    }

    pub fn handle_all(
        view_id: usize,
        discriminant: Option<(TypeId, usize)>,
        bounds: RectF,
    ) -> Self {
        Self {
            view_id,
            discriminant,
            bounds,
            handlers: HandlerSet::capture_all(),
        }
    }

    pub fn on_down(
        mut self,
        button: MouseButton,
        handler: impl Fn(DownRegionEvent, &mut EventContext) + 'static,
    ) -> Self {
        self.handlers = self.handlers.on_down(button, handler);
        self
    }

    pub fn on_up(
        mut self,
        button: MouseButton,
        handler: impl Fn(UpRegionEvent, &mut EventContext) + 'static,
    ) -> Self {
        self.handlers = self.handlers.on_up(button, handler);
        self
    }

    pub fn on_click(
        mut self,
        button: MouseButton,
        handler: impl Fn(ClickRegionEvent, &mut EventContext) + 'static,
    ) -> Self {
        self.handlers = self.handlers.on_click(button, handler);
        self
    }

    pub fn on_down_out(
        mut self,
        button: MouseButton,
        handler: impl Fn(DownOutRegionEvent, &mut EventContext) + 'static,
    ) -> Self {
        self.handlers = self.handlers.on_down_out(button, handler);
        self
    }

    pub fn on_up_out(
        mut self,
        button: MouseButton,
        handler: impl Fn(UpOutRegionEvent, &mut EventContext) + 'static,
    ) -> Self {
        self.handlers = self.handlers.on_up_out(button, handler);
        self
    }

    pub fn on_drag(
        mut self,
        button: MouseButton,
        handler: impl Fn(DragRegionEvent, &mut EventContext) + 'static,
    ) -> Self {
        self.handlers = self.handlers.on_drag(button, handler);
        self
    }

    pub fn on_hover(
        mut self,
        handler: impl Fn(HoverRegionEvent, &mut EventContext) + 'static,
    ) -> Self {
        self.handlers = self.handlers.on_hover(handler);
        self
    }

    pub fn on_move(
        mut self,
        handler: impl Fn(MoveRegionEvent, &mut EventContext) + 'static,
    ) -> Self {
        self.handlers = self.handlers.on_move(handler);
        self
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct MouseRegionId {
    pub view_id: usize,
    pub discriminant: (TypeId, usize),
}

#[derive(Clone, Default)]
pub struct HandlerSet {
    #[allow(clippy::type_complexity)]
    pub set: HashMap<
        (Discriminant<MouseRegionEvent>, Option<MouseButton>),
        Rc<dyn Fn(MouseRegionEvent, &mut EventContext)>,
    >,
}

impl HandlerSet {
    pub fn capture_all() -> Self {
        #[allow(clippy::type_complexity)]
        let mut set: HashMap<
            (Discriminant<MouseRegionEvent>, Option<MouseButton>),
            Rc<dyn Fn(MouseRegionEvent, &mut EventContext)>,
        > = Default::default();

        set.insert((MouseRegionEvent::move_disc(), None), Rc::new(|_, _| {}));
        set.insert((MouseRegionEvent::hover_disc(), None), Rc::new(|_, _| {}));
        for button in MouseButton::all() {
            set.insert(
                (MouseRegionEvent::drag_disc(), Some(button)),
                Rc::new(|_, _| {}),
            );
            set.insert(
                (MouseRegionEvent::down_disc(), Some(button)),
                Rc::new(|_, _| {}),
            );
            set.insert(
                (MouseRegionEvent::up_disc(), Some(button)),
                Rc::new(|_, _| {}),
            );
            set.insert(
                (MouseRegionEvent::click_disc(), Some(button)),
                Rc::new(|_, _| {}),
            );
            set.insert(
                (MouseRegionEvent::down_out_disc(), Some(button)),
                Rc::new(|_, _| {}),
            );
            set.insert(
                (MouseRegionEvent::up_out_disc(), Some(button)),
                Rc::new(|_, _| {}),
            );
        }
        set.insert(
            (MouseRegionEvent::scroll_wheel_disc(), None),
            Rc::new(|_, _| {}),
        );

        HandlerSet { set }
    }

    pub fn get(
        &self,
        key: &(Discriminant<MouseRegionEvent>, Option<MouseButton>),
    ) -> Option<Rc<dyn Fn(MouseRegionEvent, &mut EventContext)>> {
        self.set.get(key).cloned()
    }

    pub fn on_move(
        mut self,
        handler: impl Fn(MoveRegionEvent, &mut EventContext) + 'static,
    ) -> Self {
        self.set.insert((MouseRegionEvent::move_disc(), None),
            Rc::new(move |region_event, cx| {
                if let MouseRegionEvent::Move(e) = region_event {
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
        handler: impl Fn(DownRegionEvent, &mut EventContext) + 'static,
    ) -> Self {
        self.set.insert((MouseRegionEvent::down_disc(), Some(button)),
            Rc::new(move |region_event, cx| {
                if let MouseRegionEvent::Down(e) = region_event {
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
        handler: impl Fn(UpRegionEvent, &mut EventContext) + 'static,
    ) -> Self {
        self.set.insert((MouseRegionEvent::up_disc(), Some(button)),
            Rc::new(move |region_event, cx| {
                if let MouseRegionEvent::Up(e) = region_event {
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
        handler: impl Fn(ClickRegionEvent, &mut EventContext) + 'static,
    ) -> Self {
        self.set.insert((MouseRegionEvent::click_disc(), Some(button)),
            Rc::new(move |region_event, cx| {
                if let MouseRegionEvent::Click(e) = region_event {
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
        handler: impl Fn(DownOutRegionEvent, &mut EventContext) + 'static,
    ) -> Self {
        self.set.insert((MouseRegionEvent::down_out_disc(), Some(button)),
            Rc::new(move |region_event, cx| {
                if let MouseRegionEvent::DownOut(e) = region_event {
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
        handler: impl Fn(UpOutRegionEvent, &mut EventContext) + 'static,
    ) -> Self {
        self.set.insert((MouseRegionEvent::up_out_disc(), Some(button)),
            Rc::new(move |region_event, cx| {
                if let MouseRegionEvent::UpOut(e) = region_event {
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
        handler: impl Fn(DragRegionEvent, &mut EventContext) + 'static,
    ) -> Self {
        self.set.insert((MouseRegionEvent::drag_disc(), Some(button)),
            Rc::new(move |region_event, cx| {
                if let MouseRegionEvent::Drag(e) = region_event {
                    handler(e, cx);
                } else {
                    panic!(
                        "Mouse Region Event incorrectly called with mismatched event type. Expected MouseRegionEvent::Drag, found {:?}", 
                        region_event);
                }
            }));
        self
    }

    pub fn on_hover(
        mut self,
        handler: impl Fn(HoverRegionEvent, &mut EventContext) + 'static,
    ) -> Self {
        self.set.insert((MouseRegionEvent::hover_disc(), None),
            Rc::new(move |region_event, cx| {
                if let MouseRegionEvent::Hover(e) = region_event {
                    handler(e, cx);
                } else {
                    panic!(
                        "Mouse Region Event incorrectly called with mismatched event type. Expected MouseRegionEvent::Hover, found {:?}", 
                        region_event);
                }
            }));
        self
    }
}
