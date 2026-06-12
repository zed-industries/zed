//! This holds the resize logic for picker windows.
//!
//! # Resizing basics:
//! We render a resize handle (`render_resize`) for each side and corner that
//! can be dragged. The `Side` trait implementations (`Left`, `Right`,
//! `Bottom`, `LeftCorner`, `RightCorner`, ...) determine where each handle is
//! placed and how a drag changes the picker's shape.
//!
//! If there is a preview to the right or below there is an additional resize
//! handle on the divider between the results and the preview.
//!
//! The resize's are div's aka rectangles that are "placed" by specifying the
//! position of their sides. When hovering over these the cursor is changed
//! so it is clear you can resize.
//!
//! We set up two callbacks in each
//!  - on_drag: fires when the user starts dragging
//!  - on_drag_move: runs every frame while the user is dragging
//!
//! The actual shape of the resize is updated in `on_drag_move`. When a preview
//! is active dragging the outside edge modifies the
//!
//! # Resizing persistence
//! Each picker has a 'fixed' size and tracks it's last resize. When manually
//! resized the window size is stored as a percentage of the viewport
//! width/height. The size is serialized as soon as the use lets go of the drag.
//!
//! # Diagrams & Details
//! ================ CHANGING WIDTH ======================================
//! The picker position stays constant during the drag but it is centered
//! directly after. (when the user lets go)
//! ================ DRAGGING RIGHT ======================================
//!
//! ------------
//! |    |     |  <- dragging this edge right
//! |    |     |
//! ------------
//! leads to:
//! --------------------
//! |    |             |  <- dragged on this edge
//! |    |  preview    |
//! --------------------
//!
//! ```rust
//! self.w_preview = w_preview + drag
//! ```
//! ================ DRAGGING LEFT =======================================
//!
//!                       ------------
//! dragging this left -> |   |      |
//!                       |   |      |
//!                       ------------
//! leads to:
//! -------------------
//! | list     |      |
//! |          |      |
//! -------------------

use std::{any::type_name, marker::PhantomData};

use gpui::{Action, Context, CursorStyle, DragMoveEvent, Focusable, MouseButton, Point, Styled, Window};
use ui::{Tooltip, prelude::*};

use crate::{
    Picker, PickerDelegate, PositionAndShape, Shape, ToggleLayout, preview::PreviewLayout,
};

pub struct DragPreview;

impl Render for DragPreview {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
    }
}

#[derive(Clone, Copy)]
struct ResizeDrag<S> {
    shape_before: PositionAndShape,
    phantom_data: PhantomData<S>,
    mouse_pos_before: Point<Pixels>,
}

pub(crate) trait Side: Copy + 'static {
    fn id() -> &'static str {
        type_name::<Self>()
    }
    /// The thickness of the grab strip along the picker's edge.
    ///
    /// Expressed in rems so it scales with the user's UI font size.
    fn handle_width(window: &Window) -> Pixels {
        rems(0.375).to_pixels(window.rem_size())
    }
    fn handle_offset(window: &Window) -> Pixels {
        Self::handle_width(window) / 2.0
    }
    /// How far the grab strip is inset from the top and bottom corners so it doesn't overlap the
    /// corner resize handles.
    fn corner_clearance(window: &Window) -> Pixels {
        rems(1.125).to_pixels(window.rem_size())
    }
    /// The resize cursor for this side's handle.
    fn cursor(&self) -> CursorStyle;
    /// Places and sizes the grab strip along this side's edge.
    fn position(
        &self,
        div: gpui::Stateful<Div>,
        shape: PositionAndShape,
        window: &Window,
    ) -> gpui::Stateful<Div>;
    fn current_position_and_shape(
        &self,
        shape_before: PositionAndShape,
        mouse_movement: Point<Pixels>,
    ) -> PositionAndShape;
}

#[derive(Clone, Copy)]
pub(crate) struct Left;
impl Side for Left {
    fn cursor(&self) -> CursorStyle {
        CursorStyle::ResizeColumn
    }
    fn position(
        &self,
        div: gpui::Stateful<Div>,
        _: PositionAndShape,
        window: &Window,
    ) -> gpui::Stateful<Div> {
        div.top_0()
            .bottom(Self::corner_clearance(window))
            .w(Self::handle_width(window))
            .left(-Self::handle_offset(window))
    }
    fn current_position_and_shape(
        &self,
        mut shape_before: PositionAndShape,
        mouse_movement: Point<Pixels>,
    ) -> PositionAndShape {
        shape_before.left += mouse_movement.x;
        shape_before
    }
}
#[derive(Clone, Copy)]
pub(crate) struct Right(pub(crate) PreviewLayout);
impl Side for Right {
    fn cursor(&self) -> CursorStyle {
        CursorStyle::ResizeColumn
    }
    fn position(
        &self,
        div: gpui::Stateful<Div>,
        _: PositionAndShape,
        window: &Window,
    ) -> gpui::Stateful<Div> {
        div.top_0()
            .bottom(Self::corner_clearance(window))
            .w(Self::handle_width(window))
            .right(-Self::handle_offset(window))
    }
    fn current_position_and_shape(
        &self,
        mut shape_before: PositionAndShape,
        mouse_movement: Point<Pixels>,
    ) -> PositionAndShape {
        if let PreviewLayout::Right = self.0 {
            shape_before.preview += mouse_movement.x;
        }
        shape_before.right += mouse_movement.x;
        shape_before
    }
}

#[derive(Clone, Copy)]
pub(crate) struct Middle(pub(crate) PreviewLayout);
impl Side for Middle {
    fn cursor(&self) -> CursorStyle {
        match self.0 {
            PreviewLayout::Hidden => {
                unreachable!("This resize handle is not drawn when the preview is hidden")
            }
            PreviewLayout::Below => CursorStyle::ResizeRow,
            PreviewLayout::Right => CursorStyle::ResizeColumn,
        }
    }

    fn position(
        &self,
        div: gpui::Stateful<Div>,
        shape: PositionAndShape,
        window: &Window,
    ) -> gpui::Stateful<Div> {
        match self.0 {
            PreviewLayout::Hidden => {
                unreachable!("This resize handle is not drawn when the preview is hidden")
            }
            PreviewLayout::Below => div
                .left(Self::corner_clearance(window))
                .right(Self::corner_clearance(window))
                .h(Self::handle_width(window))
                .bottom(shape.preview - Self::handle_offset(window)),
            PreviewLayout::Right => div
                .top_0()
                .bottom(Self::corner_clearance(window))
                .w(Self::handle_width(window))
                .right(shape.preview - Self::handle_offset(window)),
        }
    }

    fn current_position_and_shape(
        &self,
        mut shape_before: PositionAndShape,
        mouse_movement: Point<Pixels>,
    ) -> PositionAndShape {
        match self.0 {
            PreviewLayout::Hidden => {
                unreachable!("This resize handle is not drawn when the preview is hidden")
            }
            PreviewLayout::Below => shape_before.preview -= mouse_movement.y,
            PreviewLayout::Right => shape_before.preview -= mouse_movement.x,
        }
        shape_before
    }
}

#[derive(Clone, Copy)]
pub(crate) struct Bottom(pub(crate) PreviewLayout);
impl Side for Bottom {
    fn cursor(&self) -> CursorStyle {
        CursorStyle::ResizeRow
    }
    fn position(
        &self,
        div: gpui::Stateful<Div>,
        _: PositionAndShape,
        window: &Window,
    ) -> gpui::Stateful<Div> {
        div.left(Self::corner_clearance(window))
            .right(Self::corner_clearance(window))
            .h(Self::handle_width(window))
            .bottom(-Self::handle_offset(window))
    }
    fn current_position_and_shape(
        &self,
        mut shape_before: PositionAndShape,
        mouse_movement: Point<Pixels>,
    ) -> PositionAndShape {
        if let PreviewLayout::Below = self.0 {
            shape_before.preview += mouse_movement.y;
        }
        shape_before.bottom += mouse_movement.y;
        shape_before
    }
}
#[derive(Clone, Copy)]
pub(crate) struct LeftCorner(pub(crate) PreviewLayout);
impl Side for LeftCorner {
    fn cursor(&self) -> CursorStyle {
        CursorStyle::ResizeUpRightDownLeft
    }
    fn position(
        &self,
        div: gpui::Stateful<Div>,
        _: PositionAndShape,
        window: &Window,
    ) -> gpui::Stateful<Div> {
        div.w(Self::handle_width(window))
            .h(Self::handle_width(window))
            .left(-Self::handle_offset(window))
            .bottom(-Self::handle_offset(window))
    }
    fn current_position_and_shape(
        &self,
        mut shape_before: PositionAndShape,
        mouse_movement: Point<Pixels>,
    ) -> PositionAndShape {
        match self.0 {
            PreviewLayout::Hidden => (),
            PreviewLayout::Below => shape_before.preview += mouse_movement.y,
            PreviewLayout::Right => shape_before.preview += mouse_movement.x,
        }
        shape_before.left += mouse_movement.x;
        shape_before.bottom += mouse_movement.y;
        shape_before
    }
}
#[derive(Clone, Copy)]
pub(crate) struct RightCorner(pub(crate) PreviewLayout);
impl Side for RightCorner {
    fn cursor(&self) -> CursorStyle {
        CursorStyle::ResizeUpLeftDownRight
    }
    fn position(
        &self,
        div: gpui::Stateful<Div>,
        _: PositionAndShape,
        window: &Window,
    ) -> gpui::Stateful<Div> {
        div.w(Self::handle_width(window))
            .h(Self::handle_width(window))
            .right(-Self::handle_offset(window))
            .bottom(-Self::handle_offset(window))
    }
    fn current_position_and_shape(
        &self,
        mut shape_before: PositionAndShape,
        mouse_movement: Point<Pixels>,
    ) -> PositionAndShape {
        match self.0 {
            PreviewLayout::Hidden => (),
            PreviewLayout::Below => shape_before.preview += mouse_movement.y,
            PreviewLayout::Right => shape_before.preview += mouse_movement.x,
        }
        shape_before.right += mouse_movement.x;
        shape_before.bottom += mouse_movement.y;
        shape_before
    }
}

impl<S: Side> ResizeDrag<S> {
    fn start_new(shape: Shape, window: &mut Window) -> Self {
        Self {
            mouse_pos_before: window.mouse_position(),
            shape_before: shape.picker_position_and_size(window),
            phantom_data: PhantomData,
        }
    }
}

// TODO!(yara) make this all work for with and without preview
impl<D: PickerDelegate> Picker<D> {
    /// Resizes the picker modal by dragging the handle on the given side or corner
    pub(crate) fn render_resize<S: Side>(
        &self,
        side: S,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        div()
            .id(S::id())
            .absolute()
            .cursor(side.cursor())
            .map(|this| side.position(this, self.shape.picker_position_and_size(window), window))
            .block_mouse_except_scroll()
            .on_mouse_down(MouseButton::Left, do_nothing) // TODO!(yara) do we need this?
            .on_drag(
                ResizeDrag::<S>::start_new(self.shape, window),
                |_, _, _, cx| cx.new(|_| DragPreview),
            )
            .on_drag_move::<ResizeDrag<S>>(cx.listener(
                move |this, event: &DragMoveEvent<ResizeDrag<S>>, _, cx| {
                    let drag = event.drag(cx);
                    let delta = event.event.position - drag.mouse_pos_before;
                    this.shape = Shape::Resizing(
                        side.current_position_and_shape(drag.shape_before, delta),
                    );
                    cx.notify();
                },
            ))
    }

    pub(crate) fn render_header_controls(
        &self,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<impl IntoElement> {
        let preview = self.preview.as_ref()?;

        Some(h_flex().gap_1().items_center().child({
            let focus_handle = self.focus_handle(cx);
            let (icon, tooltip_text) = match preview.layout {
                PreviewLayout::Hidden => (IconName::Split, "Show preview to the right"),
                PreviewLayout::Right => (IconName::ListTree, "Show preview below"),
                PreviewLayout::Below => (IconName::ListCollapse, "Hide Preview"),
            };
            IconButton::new("layout-cycle", icon)
                .size(ButtonSize::Compact)
                .tooltip(move |_window, cx| {
                    Tooltip::for_action_in(tooltip_text, &ToggleLayout, &focus_handle, cx)
                })
                .on_click(|_, window, cx| {
                    window.dispatch_action(ToggleLayout.boxed_clone(), cx);
                })
        }))
    }
}

fn do_nothing(_: &gpui::MouseDownEvent, window: &mut Window, cx: &mut App) {
    window.prevent_default();
    cx.stop_propagation();
}
