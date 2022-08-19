use std::{any::TypeId, mem::Discriminant, rc::Rc};

use collections::HashMap;

use pathfinder_geometry::{rect::RectF, vector::Vector2F};

use crate::{EventContext, MouseButton, MouseButtonEvent, MouseMovedEvent, ScrollWheelEvent};

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
            handlers: HandlerSet::handle_all(),
        }
    }

    pub fn on_down(
        mut self,
        button: MouseButton,
        handler: impl Fn(MouseButtonEvent, &mut EventContext) + 'static,
    ) -> Self {
        self.handlers = self.handlers.on_down(button, handler);
        self
    }

    pub fn on_up(
        mut self,
        button: MouseButton,
        handler: impl Fn(MouseButtonEvent, &mut EventContext) + 'static,
    ) -> Self {
        self.handlers = self.handlers.on_up(button, handler);
        self
    }

    pub fn on_click(
        mut self,
        button: MouseButton,
        handler: impl Fn(MouseButtonEvent, &mut EventContext) + 'static,
    ) -> Self {
        self.handlers = self.handlers.on_click(button, handler);
        self
    }

    pub fn on_down_out(
        mut self,
        button: MouseButton,
        handler: impl Fn(MouseButtonEvent, &mut EventContext) + 'static,
    ) -> Self {
        self.handlers = self.handlers.on_down_out(button, handler);
        self
    }

    pub fn on_drag(
        mut self,
        button: MouseButton,
        handler: impl Fn(Vector2F, MouseMovedEvent, &mut EventContext) + 'static,
    ) -> Self {
        self.handlers = self.handlers.on_drag(button, handler);
        self
    }

    pub fn on_hover(
        mut self,
        handler: impl Fn(bool, MouseMovedEvent, &mut EventContext) + 'static,
    ) -> Self {
        self.handlers = self.handlers.on_hover(handler);
        self
    }

    pub fn on_move(
        mut self,
        handler: impl Fn(MouseMovedEvent, &mut EventContext) + 'static,
    ) -> Self {
        self.handlers = self.handlers.on_move(handler);
        self
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
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
    pub fn handle_all() -> Self {
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

    pub fn on_down(
        mut self,
        button: MouseButton,
        handler: impl Fn(MouseButtonEvent, &mut EventContext) + 'static,
    ) -> Self {
        self.set.insert((MouseRegionEvent::down_disc(), Some(button)),
            Rc::new(move |region_event, cx| {
                if let MouseRegionEvent::Down(mouse_button_event) = region_event {
                    handler(mouse_button_event, cx);
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
        handler: impl Fn(MouseButtonEvent, &mut EventContext) + 'static,
    ) -> Self {
        self.set.insert((MouseRegionEvent::up_disc(), Some(button)),
            Rc::new(move |region_event, cx| {
                if let MouseRegionEvent::Up(mouse_button_event) = region_event {
                    handler(mouse_button_event, cx);
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
        handler: impl Fn(MouseButtonEvent, &mut EventContext) + 'static,
    ) -> Self {
        self.set.insert((MouseRegionEvent::click_disc(), Some(button)),
            Rc::new(move |region_event, cx| {
                if let MouseRegionEvent::Click(mouse_button_event) = region_event {
                    handler(mouse_button_event, cx);
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
        handler: impl Fn(MouseButtonEvent, &mut EventContext) + 'static,
    ) -> Self {
        self.set.insert((MouseRegionEvent::down_out_disc(), Some(button)),
            Rc::new(move |region_event, cx| {
                if let MouseRegionEvent::DownOut(mouse_button_event) = region_event {
                    handler(mouse_button_event, cx);
                } else {
                    panic!(
                        "Mouse Region Event incorrectly called with mismatched event type. Expected MouseRegionEvent::DownOut, found {:?}", 
                        region_event);
                }
            }));
        self
    }

    pub fn on_drag(
        mut self,
        button: MouseButton,
        handler: impl Fn(Vector2F, MouseMovedEvent, &mut EventContext) + 'static,
    ) -> Self {
        self.set.insert((MouseRegionEvent::drag_disc(), Some(button)),
            Rc::new(move |region_event, cx| {
                if let MouseRegionEvent::Drag(prev_drag_position, mouse_moved_event) = region_event {
                    handler(prev_drag_position, mouse_moved_event, cx);
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
        handler: impl Fn(bool, MouseMovedEvent, &mut EventContext) + 'static,
    ) -> Self {
        self.set.insert((MouseRegionEvent::hover_disc(), None),
            Rc::new(move |region_event, cx| {
                if let MouseRegionEvent::Hover(hover, mouse_moved_event) = region_event {
                    handler(hover, mouse_moved_event, cx);
                } else {
                    panic!(
                        "Mouse Region Event incorrectly called with mismatched event type. Expected MouseRegionEvent::Hover, found {:?}", 
                        region_event);
                }
            }));
        self
    }

    pub fn on_move(
        mut self,
        handler: impl Fn(MouseMovedEvent, &mut EventContext) + 'static,
    ) -> Self {
        self.set.insert((MouseRegionEvent::move_disc(), None),
            Rc::new(move |region_event, cx| {
                if let MouseRegionEvent::Move(move_event)= region_event {
                    handler(move_event, cx);
                }  else {
                    panic!(
                        "Mouse Region Event incorrectly called with mismatched event type. Expected MouseRegionEvent::Move, found {:?}", 
                        region_event);
                }
            }));
        self
    }
}

#[derive(Debug)]
pub enum MouseRegionEvent {
    Move(MouseMovedEvent),
    Drag(Vector2F, MouseMovedEvent),
    Hover(bool, MouseMovedEvent),
    Down(MouseButtonEvent),
    Up(MouseButtonEvent),
    Click(MouseButtonEvent),
    DownOut(MouseButtonEvent),
    ScrollWheel(ScrollWheelEvent),
}

impl MouseRegionEvent {
    pub fn move_disc() -> Discriminant<MouseRegionEvent> {
        std::mem::discriminant(&MouseRegionEvent::Move(Default::default()))
    }
    pub fn drag_disc() -> Discriminant<MouseRegionEvent> {
        std::mem::discriminant(&MouseRegionEvent::Drag(
            Default::default(),
            Default::default(),
        ))
    }
    pub fn hover_disc() -> Discriminant<MouseRegionEvent> {
        std::mem::discriminant(&MouseRegionEvent::Hover(
            Default::default(),
            Default::default(),
        ))
    }
    pub fn down_disc() -> Discriminant<MouseRegionEvent> {
        std::mem::discriminant(&MouseRegionEvent::Down(Default::default()))
    }
    pub fn up_disc() -> Discriminant<MouseRegionEvent> {
        std::mem::discriminant(&MouseRegionEvent::Up(Default::default()))
    }
    pub fn click_disc() -> Discriminant<MouseRegionEvent> {
        std::mem::discriminant(&MouseRegionEvent::Click(Default::default()))
    }
    pub fn down_out_disc() -> Discriminant<MouseRegionEvent> {
        std::mem::discriminant(&MouseRegionEvent::DownOut(Default::default()))
    }
    pub fn scroll_wheel_disc() -> Discriminant<MouseRegionEvent> {
        std::mem::discriminant(&MouseRegionEvent::ScrollWheel(Default::default()))
    }

    pub fn handler_key(&self) -> (Discriminant<MouseRegionEvent>, Option<MouseButton>) {
        match self {
            MouseRegionEvent::Move(_) => (Self::move_disc(), None),
            MouseRegionEvent::Drag(_, MouseMovedEvent { pressed_button, .. }) => {
                (Self::drag_disc(), *pressed_button)
            }
            MouseRegionEvent::Hover(_, _) => (Self::hover_disc(), None),
            MouseRegionEvent::Down(MouseButtonEvent { button, .. }) => {
                (Self::down_disc(), Some(*button))
            }
            MouseRegionEvent::Up(MouseButtonEvent { button, .. }) => {
                (Self::up_disc(), Some(*button))
            }
            MouseRegionEvent::Click(MouseButtonEvent { button, .. }) => {
                (Self::click_disc(), Some(*button))
            }
            MouseRegionEvent::DownOut(MouseButtonEvent { button, .. }) => {
                (Self::down_out_disc(), Some(*button))
            }
            MouseRegionEvent::ScrollWheel(_) => (Self::scroll_wheel_disc(), None),
        }
    }
}
