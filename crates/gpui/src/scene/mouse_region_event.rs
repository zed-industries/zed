use std::{
    mem::{discriminant, Discriminant},
    ops::Deref,
};

use pathfinder_geometry::{rect::RectF, vector::Vector2F};

use crate::{MouseButton, MouseButtonEvent, MouseMovedEvent, ScrollWheelEvent};

#[derive(Debug, Default, Clone)]
pub struct MoveRegionEvent {
    pub region: RectF,
    pub platform_event: MouseMovedEvent,
}

impl Deref for MoveRegionEvent {
    type Target = MouseMovedEvent;

    fn deref(&self) -> &Self::Target {
        &self.platform_event
    }
}

#[derive(Debug, Default, Clone)]
pub struct DragRegionEvent {
    pub region: RectF,
    pub prev_mouse_position: Vector2F,
    pub platform_event: MouseMovedEvent,
}

impl Deref for DragRegionEvent {
    type Target = MouseMovedEvent;

    fn deref(&self) -> &Self::Target {
        &self.platform_event
    }
}

#[derive(Debug, Default, Clone)]
pub struct HoverRegionEvent {
    pub region: RectF,
    pub started: bool,
    pub platform_event: MouseMovedEvent,
}

impl Deref for HoverRegionEvent {
    type Target = MouseMovedEvent;

    fn deref(&self) -> &Self::Target {
        &self.platform_event
    }
}

#[derive(Debug, Default, Clone)]
pub struct DownRegionEvent {
    pub region: RectF,
    pub platform_event: MouseButtonEvent,
}

impl Deref for DownRegionEvent {
    type Target = MouseButtonEvent;

    fn deref(&self) -> &Self::Target {
        &self.platform_event
    }
}

#[derive(Debug, Default, Clone)]
pub struct UpRegionEvent {
    pub region: RectF,
    pub platform_event: MouseButtonEvent,
}

impl Deref for UpRegionEvent {
    type Target = MouseButtonEvent;

    fn deref(&self) -> &Self::Target {
        &self.platform_event
    }
}

#[derive(Debug, Default, Clone)]
pub struct ClickRegionEvent {
    pub region: RectF,
    pub platform_event: MouseButtonEvent,
}

impl Deref for ClickRegionEvent {
    type Target = MouseButtonEvent;

    fn deref(&self) -> &Self::Target {
        &self.platform_event
    }
}

#[derive(Debug, Default, Clone)]
pub struct DownOutRegionEvent {
    pub region: RectF,
    pub platform_event: MouseButtonEvent,
}

impl Deref for DownOutRegionEvent {
    type Target = MouseButtonEvent;

    fn deref(&self) -> &Self::Target {
        &self.platform_event
    }
}

#[derive(Debug, Default, Clone)]
pub struct UpOutRegionEvent {
    pub region: RectF,
    pub platform_event: MouseButtonEvent,
}

impl Deref for UpOutRegionEvent {
    type Target = MouseButtonEvent;

    fn deref(&self) -> &Self::Target {
        &self.platform_event
    }
}

#[derive(Debug, Default, Clone)]
pub struct ScrollWheelRegionEvent {
    pub region: RectF,
    pub platform_event: ScrollWheelEvent,
}

impl Deref for ScrollWheelRegionEvent {
    type Target = ScrollWheelEvent;

    fn deref(&self) -> &Self::Target {
        &self.platform_event
    }
}

#[derive(Debug, Clone)]
pub enum MouseRegionEvent {
    Move(MoveRegionEvent),
    Drag(DragRegionEvent),
    Hover(HoverRegionEvent),
    Down(DownRegionEvent),
    Up(UpRegionEvent),
    Click(ClickRegionEvent),
    DownOut(DownOutRegionEvent),
    UpOut(UpOutRegionEvent),
    ScrollWheel(ScrollWheelRegionEvent),
}

impl MouseRegionEvent {
    pub fn set_region(&mut self, region: RectF) {
        match self {
            MouseRegionEvent::Move(r) => r.region = region,
            MouseRegionEvent::Drag(r) => r.region = region,
            MouseRegionEvent::Hover(r) => r.region = region,
            MouseRegionEvent::Down(r) => r.region = region,
            MouseRegionEvent::Up(r) => r.region = region,
            MouseRegionEvent::Click(r) => r.region = region,
            MouseRegionEvent::DownOut(r) => r.region = region,
            MouseRegionEvent::UpOut(r) => r.region = region,
            MouseRegionEvent::ScrollWheel(r) => r.region = region,
        }
    }

    /// When true, mouse event handlers must call cx.propagate_event() to bubble
    /// the event to handlers they are painted on top of.
    pub fn is_capturable(&self) -> bool {
        match self {
            MouseRegionEvent::Move(_) => true,
            MouseRegionEvent::Drag(_) => false,
            MouseRegionEvent::Hover(_) => false,
            MouseRegionEvent::Down(_) => true,
            MouseRegionEvent::Up(_) => true,
            MouseRegionEvent::Click(_) => true,
            MouseRegionEvent::DownOut(_) => false,
            MouseRegionEvent::UpOut(_) => false,
            MouseRegionEvent::ScrollWheel(_) => true,
        }
    }
}

impl MouseRegionEvent {
    pub fn move_disc() -> Discriminant<MouseRegionEvent> {
        discriminant(&MouseRegionEvent::Move(Default::default()))
    }

    pub fn drag_disc() -> Discriminant<MouseRegionEvent> {
        discriminant(&MouseRegionEvent::Drag(Default::default()))
    }

    pub fn hover_disc() -> Discriminant<MouseRegionEvent> {
        discriminant(&MouseRegionEvent::Hover(Default::default()))
    }

    pub fn down_disc() -> Discriminant<MouseRegionEvent> {
        discriminant(&MouseRegionEvent::Down(Default::default()))
    }

    pub fn up_disc() -> Discriminant<MouseRegionEvent> {
        discriminant(&MouseRegionEvent::Up(Default::default()))
    }

    pub fn up_out_disc() -> Discriminant<MouseRegionEvent> {
        discriminant(&MouseRegionEvent::UpOut(Default::default()))
    }

    pub fn click_disc() -> Discriminant<MouseRegionEvent> {
        discriminant(&MouseRegionEvent::Click(Default::default()))
    }

    pub fn down_out_disc() -> Discriminant<MouseRegionEvent> {
        discriminant(&MouseRegionEvent::DownOut(Default::default()))
    }

    pub fn scroll_wheel_disc() -> Discriminant<MouseRegionEvent> {
        discriminant(&MouseRegionEvent::ScrollWheel(Default::default()))
    }

    pub fn handler_key(&self) -> (Discriminant<MouseRegionEvent>, Option<MouseButton>) {
        match self {
            MouseRegionEvent::Move(_) => (Self::move_disc(), None),
            MouseRegionEvent::Drag(e) => (Self::drag_disc(), e.pressed_button),
            MouseRegionEvent::Hover(_) => (Self::hover_disc(), None),
            MouseRegionEvent::Down(e) => (Self::down_disc(), Some(e.button)),
            MouseRegionEvent::Up(e) => (Self::up_disc(), Some(e.button)),
            MouseRegionEvent::Click(e) => (Self::click_disc(), Some(e.button)),
            MouseRegionEvent::UpOut(e) => (Self::up_out_disc(), Some(e.button)),
            MouseRegionEvent::DownOut(e) => (Self::down_out_disc(), Some(e.button)),
            MouseRegionEvent::ScrollWheel(_) => (Self::scroll_wheel_disc(), None),
        }
    }
}
