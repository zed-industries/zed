mod client;
mod clipboard;
mod cursor;
mod display;
mod serial;
mod window;

pub(crate) use client::*;

use wayland_protocols::wp::cursor_shape::v1::client::wp_cursor_shape_device_v1::Shape;

use crate::CursorStyle;

impl CursorStyle {
    pub(super) fn to_shape(&self) -> Shape {
        match self {
            CursorStyle::Arrow => Shape::Default,
            CursorStyle::IBeam => Shape::Text,
            CursorStyle::Crosshair => Shape::Crosshair,
            CursorStyle::ClosedHand => Shape::Grabbing,
            CursorStyle::OpenHand => Shape::Grab,
            CursorStyle::PointingHand => Shape::Pointer,
            CursorStyle::ResizeLeft => Shape::WResize,
            CursorStyle::ResizeRight => Shape::EResize,
            CursorStyle::ResizeLeftRight => Shape::EwResize,
            CursorStyle::ResizeUp => Shape::NResize,
            CursorStyle::ResizeDown => Shape::SResize,
            CursorStyle::ResizeUpDown => Shape::NsResize,
            CursorStyle::ResizeUpLeftDownRight => Shape::NwseResize,
            CursorStyle::ResizeUpRightDownLeft => Shape::NeswResize,
            CursorStyle::ResizeColumn => Shape::ColResize,
            CursorStyle::ResizeRow => Shape::RowResize,
            CursorStyle::IBeamCursorForVerticalLayout => Shape::VerticalText,
            CursorStyle::OperationNotAllowed => Shape::NotAllowed,
            CursorStyle::DragLink => Shape::Alias,
            CursorStyle::DragCopy => Shape::Copy,
            CursorStyle::ContextualMenu => Shape::ContextMenu,
        }
    }
}
