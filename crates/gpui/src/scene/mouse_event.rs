use crate::{
    platform::{MouseButtonEvent, MouseMovedEvent, ScrollWheelEvent},
    scene::mouse_region::HandlerKey,
};
use pathfinder_geometry::{rect::RectF, vector::Vector2F};
use std::{
    mem::{discriminant, Discriminant},
    ops::Deref,
};

#[derive(Debug, Default, Clone)]
pub struct MouseMove {
    pub region: RectF,
    pub platform_event: MouseMovedEvent,
}

impl Deref for MouseMove {
    type Target = MouseMovedEvent;

    fn deref(&self) -> &Self::Target {
        &self.platform_event
    }
}

#[derive(Debug, Default, Clone)]
pub struct MouseMoveOut {
    pub region: RectF,
}

#[derive(Debug, Default, Clone)]
pub struct MouseDrag {
    pub region: RectF,
    pub prev_mouse_position: Vector2F,
    pub platform_event: MouseMovedEvent,
    pub end: bool
}

impl Deref for MouseDrag {
    type Target = MouseMovedEvent;

    fn deref(&self) -> &Self::Target {
        &self.platform_event
    }
}

#[derive(Debug, Default, Clone)]
pub struct MouseHover {
    pub region: RectF,
    pub started: bool,
    pub platform_event: MouseMovedEvent,
}

impl Deref for MouseHover {
    type Target = MouseMovedEvent;

    fn deref(&self) -> &Self::Target {
        &self.platform_event
    }
}

#[derive(Debug, Default, Clone)]
pub struct MouseDown {
    pub region: RectF,
    pub platform_event: MouseButtonEvent,
}

impl Deref for MouseDown {
    type Target = MouseButtonEvent;

    fn deref(&self) -> &Self::Target {
        &self.platform_event
    }
}

#[derive(Debug, Default, Clone)]
pub struct MouseUp {
    pub region: RectF,
    pub platform_event: MouseButtonEvent,
}

impl Deref for MouseUp {
    type Target = MouseButtonEvent;

    fn deref(&self) -> &Self::Target {
        &self.platform_event
    }
}

#[derive(Debug, Default, Clone)]
pub struct MouseClick {
    pub region: RectF,
    pub platform_event: MouseButtonEvent,
}

impl Deref for MouseClick {
    type Target = MouseButtonEvent;

    fn deref(&self) -> &Self::Target {
        &self.platform_event
    }
}

#[derive(Debug, Default, Clone)]
pub struct MouseClickOut {
    pub region: RectF,
    pub platform_event: MouseButtonEvent,
}

impl Deref for MouseClickOut {
    type Target = MouseButtonEvent;

    fn deref(&self) -> &Self::Target {
        &self.platform_event
    }
}

#[derive(Debug, Default, Clone)]
pub struct MouseDownOut {
    pub region: RectF,
    pub platform_event: MouseButtonEvent,
}

impl Deref for MouseDownOut {
    type Target = MouseButtonEvent;

    fn deref(&self) -> &Self::Target {
        &self.platform_event
    }
}

#[derive(Debug, Default, Clone)]
pub struct MouseUpOut {
    pub region: RectF,
    pub platform_event: MouseButtonEvent,
}

impl Deref for MouseUpOut {
    type Target = MouseButtonEvent;

    fn deref(&self) -> &Self::Target {
        &self.platform_event
    }
}

#[derive(Debug, Default, Clone)]
pub struct MouseScrollWheel {
    pub region: RectF,
    pub platform_event: ScrollWheelEvent,
}

impl Deref for MouseScrollWheel {
    type Target = ScrollWheelEvent;

    fn deref(&self) -> &Self::Target {
        &self.platform_event
    }
}

#[derive(Debug, Clone)]
pub enum MouseEvent {
    Move(MouseMove),
    MoveOut(MouseMoveOut),
    Drag(MouseDrag),
    Hover(MouseHover),
    Down(MouseDown),
    Up(MouseUp),
    Click(MouseClick),
    ClickOut(MouseClickOut),
    DownOut(MouseDownOut),
    UpOut(MouseUpOut),
    ScrollWheel(MouseScrollWheel),
}

impl MouseEvent {
    pub fn set_region(&mut self, region: RectF) {
        match self {
            MouseEvent::Move(r) => r.region = region,
            MouseEvent::MoveOut(r) => r.region = region,
            MouseEvent::Drag(r) => r.region = region,
            MouseEvent::Hover(r) => r.region = region,
            MouseEvent::Down(r) => r.region = region,
            MouseEvent::Up(r) => r.region = region,
            MouseEvent::Click(r) => r.region = region,
            MouseEvent::ClickOut(r) => r.region = region,
            MouseEvent::DownOut(r) => r.region = region,
            MouseEvent::UpOut(r) => r.region = region,
            MouseEvent::ScrollWheel(r) => r.region = region,
        }
    }

    /// When true, mouse event handlers must call cx.propagate_event() to bubble
    /// the event to handlers they are painted on top of.
    pub fn is_capturable(&self) -> bool {
        match self {
            MouseEvent::Move(_) => true,
            MouseEvent::MoveOut(_) => false,
            MouseEvent::Drag(_) => true,
            MouseEvent::Hover(_) => false,
            MouseEvent::Down(_) => true,
            MouseEvent::Up(_) => true,
            MouseEvent::Click(_) => true,
            MouseEvent::ClickOut(_) => true,
            MouseEvent::DownOut(_) => false,
            MouseEvent::UpOut(_) => false,
            MouseEvent::ScrollWheel(_) => true,
        }
    }
}

impl MouseEvent {
    pub fn move_disc() -> Discriminant<MouseEvent> {
        discriminant(&MouseEvent::Move(Default::default()))
    }

    pub fn move_out_disc() -> Discriminant<MouseEvent> {
        discriminant(&MouseEvent::MoveOut(Default::default()))
    }

    pub fn drag_disc() -> Discriminant<MouseEvent> {
        discriminant(&MouseEvent::Drag(Default::default()))
    }

    pub fn hover_disc() -> Discriminant<MouseEvent> {
        discriminant(&MouseEvent::Hover(Default::default()))
    }

    pub fn down_disc() -> Discriminant<MouseEvent> {
        discriminant(&MouseEvent::Down(Default::default()))
    }

    pub fn up_disc() -> Discriminant<MouseEvent> {
        discriminant(&MouseEvent::Up(Default::default()))
    }

    pub fn up_out_disc() -> Discriminant<MouseEvent> {
        discriminant(&MouseEvent::UpOut(Default::default()))
    }

    pub fn click_disc() -> Discriminant<MouseEvent> {
        discriminant(&MouseEvent::Click(Default::default()))
    }

    pub fn click_out_disc() -> Discriminant<MouseEvent> {
        discriminant(&MouseEvent::ClickOut(Default::default()))
    }

    pub fn down_out_disc() -> Discriminant<MouseEvent> {
        discriminant(&MouseEvent::DownOut(Default::default()))
    }

    pub fn scroll_wheel_disc() -> Discriminant<MouseEvent> {
        discriminant(&MouseEvent::ScrollWheel(Default::default()))
    }

    pub fn handler_key(&self) -> HandlerKey {
        match self {
            MouseEvent::Move(_) => HandlerKey::new(Self::move_disc(), None),
            MouseEvent::MoveOut(_) => HandlerKey::new(Self::move_out_disc(), None),
            MouseEvent::Drag(e) => HandlerKey::new(Self::drag_disc(), e.pressed_button),
            MouseEvent::Hover(_) => HandlerKey::new(Self::hover_disc(), None),
            MouseEvent::Down(e) => HandlerKey::new(Self::down_disc(), Some(e.button)),
            MouseEvent::Up(e) => HandlerKey::new(Self::up_disc(), Some(e.button)),
            MouseEvent::Click(e) => HandlerKey::new(Self::click_disc(), Some(e.button)),
            MouseEvent::ClickOut(e) => HandlerKey::new(Self::click_out_disc(), Some(e.button)),
            MouseEvent::UpOut(e) => HandlerKey::new(Self::up_out_disc(), Some(e.button)),
            MouseEvent::DownOut(e) => HandlerKey::new(Self::down_out_disc(), Some(e.button)),
            MouseEvent::ScrollWheel(_) => HandlerKey::new(Self::scroll_wheel_disc(), None),
        }
    }
}
