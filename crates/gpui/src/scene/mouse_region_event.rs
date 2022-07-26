use std::{
    mem::{discriminant, Discriminant},
    ops::Deref,
};

use pathfinder_geometry::{rect::RectF, vector::Vector2F};

use crate::{MouseButton, MouseButtonEvent, MouseMovedEvent, ScrollWheelEvent};

#[derive(Debug, Default)]
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

#[derive(Debug, Default)]
pub struct DragRegionEvent {
    pub region: RectF,
    pub prev_drag_position: Vector2F,
    pub platform_event: MouseMovedEvent,
}

impl Deref for DragRegionEvent {
    type Target = MouseMovedEvent;

    fn deref(&self) -> &Self::Target {
        &self.platform_event
    }
}

#[derive(Debug, Default)]
pub struct DragOverRegionEvent {
    pub region: RectF,
    pub started: bool,
    pub platform_event: MouseMovedEvent,
}

impl Deref for DragOverRegionEvent {
    type Target = MouseMovedEvent;

    fn deref(&self) -> &Self::Target {
        &self.platform_event
    }
}

#[derive(Debug, Default)]
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

#[derive(Debug, Default)]
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

#[derive(Debug, Default)]
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

#[derive(Debug, Default)]
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

#[derive(Debug, Default)]
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

#[derive(Debug, Default)]
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

#[derive(Debug, Default)]
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

#[derive(Debug)]
pub enum MouseRegionEvent {
    Move(MoveRegionEvent),
    Drag(DragRegionEvent),
    DragOver(DragOverRegionEvent),
    Hover(HoverRegionEvent),
    Down(DownRegionEvent),
    Up(UpRegionEvent),
    Click(ClickRegionEvent),
    DownOut(DownOutRegionEvent),
    UpOut(UpOutRegionEvent),
    ScrollWheel(ScrollWheelRegionEvent),
}

impl MouseRegionEvent {
    pub fn move_disc() -> Discriminant<MouseRegionEvent> {
        discriminant(&MouseRegionEvent::Move(Default::default()))
    }

    pub fn drag_disc() -> Discriminant<MouseRegionEvent> {
        discriminant(&MouseRegionEvent::Drag(Default::default()))
    }

    pub fn drag_over_disc() -> Discriminant<MouseRegionEvent> {
        discriminant(&MouseRegionEvent::DragOver(Default::default()))
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

    pub fn is_local(&self) -> bool {
        match self {
            MouseRegionEvent::DownOut(_)
            | MouseRegionEvent::UpOut(_)
            | MouseRegionEvent::DragOver(_) => false,
            _ => true,
        }
    }

    pub fn handler_key(&self) -> (Discriminant<MouseRegionEvent>, Option<MouseButton>) {
        match self {
            MouseRegionEvent::Move(_) => (Self::move_disc(), None),
            MouseRegionEvent::Drag(e) => (Self::drag_disc(), e.pressed_button),
            MouseRegionEvent::DragOver(e) => (Self::drag_over_disc(), e.pressed_button),
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
