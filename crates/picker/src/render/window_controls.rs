//! This holds the resize logic for picker windows.
//!
//! # Resizing basics:
//! We have between three and four dedicated resize renders active:
//!  - render_width_resize
//!  - render_corner_resize (allows resizing in both directions)
//!  - render height_resize
//!
//! If there is a preview to the right or below there is an additional resize
//!  - render_horizontal_divider_resize
//!  - render_vertical_divider_resize
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

use gpui::{
    Action, Context, DragMoveEvent, Entity, FocusHandle, Focusable, MouseButton, Point, Styled,
    Window,
};
use ui::{ButtonLike, ContextMenu, PopoverMenu, PopoverMenuHandle, TintColor, Tooltip, prelude::*};
use workspace::pane;

use crate::{
    AbsolutePositionAndShape, Picker, PickerDelegate, Preview, Shape, ToggleLayout,
    ToggleSplitMenu,
    preview::{render::do_nothing, state::LayoutMode},
};

pub struct DragPreview;

impl Render for DragPreview {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
    }
}

#[derive(Clone, Copy)]
pub struct ResizeDrag {
    pub mouse_start_y: Pixels,
    pub results_height_start: Pixels,
    pub preview_height_start: Pixels,
}

#[derive(Clone, Copy)]
struct VerticalResizeDrag<S> {
    shape_before: AbsolutePositionAndShape,
    phantom_data: PhantomData<S>,
    mouse_pos_before: Point<Pixels>,
}

#[derive(Clone, Copy)]
struct CornerResizeDrag {
    mouse_start: gpui::Point<Pixels>,
    width_start: Pixels,
    preview_width_start: Pixels,
    results_height_start: Pixels,
    preview_height_start: Pixels,
    content_height_start: Pixels,
    offset_start: gpui::Point<Pixels>,
}

#[derive(Clone, Copy)]
pub struct TelescopePreviewResizeDrag {
    pub(crate) mouse_start_x: Pixels,
    pub(crate) preview_width_start: Pixels,
}

#[derive(Clone, Copy)]
pub struct TelescopeHeightResizeDrag {
    pub(crate) mouse_start_y: Pixels,
    pub(crate) content_height_start: Pixels,
    pub(crate) offset_start: Pixels,
}

#[derive(Clone, Copy)]
struct HorizontalResizeDrag<S> {
    shape_before: AbsolutePositionAndShape,
    phantom_data: PhantomData<S>,
    mouse_pos_before: Point<Pixels>,
}

pub(crate) trait Side {
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
    fn position(div: gpui::Stateful<Div>, window: &Window) -> gpui::Stateful<Div>;
    fn current_position_and_shape(
        shape_before: AbsolutePositionAndShape,
        mouse_movement: Pixels,
    ) -> AbsolutePositionAndShape;
}

pub(crate) struct Left;
impl Side for Left {
    fn position(div: gpui::Stateful<Div>, window: &Window) -> gpui::Stateful<Div> {
        div.left(-Self::handle_offset(window))
    }
    fn current_position_and_shape(
        mut shape_before: AbsolutePositionAndShape,
        mouse_movement: Pixels,
    ) -> AbsolutePositionAndShape {
        shape_before.left += mouse_movement;
        shape_before
    }
}
pub(crate) struct Right;
impl Side for Right {
    fn position(div: gpui::Stateful<Div>, window: &Window) -> gpui::Stateful<Div> {
        div.right(-Self::handle_offset(window))
    }
    fn current_position_and_shape(
        mut shape_before: AbsolutePositionAndShape,
        mouse_movement: Pixels,
    ) -> AbsolutePositionAndShape {
        shape_before.right += mouse_movement;
        shape_before
    }
}
pub(crate) struct Bottom;
impl Side for Bottom {
    fn position(div: gpui::Stateful<Div>, window: &Window) -> gpui::Stateful<Div> {
        div.bottom(-Self::handle_offset(window))
    }
    fn current_position_and_shape(
        mut shape_before: AbsolutePositionAndShape,
        mouse_movement: Pixels,
    ) -> AbsolutePositionAndShape {
        shape_before.bottom += mouse_movement;
        shape_before
    }
}

impl<S: Side> HorizontalResizeDrag<S> {
    fn start_new(shape: Shape, preview: Option<&Preview>, window: &mut Window) -> Self {
        Self {
            mouse_pos_before: window.mouse_position(),
            shape_before: shape.absolute_position_and_size(preview, window),
            phantom_data: PhantomData,
        }
    }
}

impl<S: Side> VerticalResizeDrag<S> {
    fn start_new(shape: Shape, preview: Option<&Preview>, window: &mut Window) -> Self {
        Self {
            mouse_pos_before: window.mouse_position(),
            shape_before: shape.absolute_position_and_size(preview, window),
            phantom_data: PhantomData,
        }
    }
}

// TODO!(yara) make this all work for with and without preview
impl<D: PickerDelegate> Picker<D> {
    /// Resizes the picker model by extending it on the left or right
    pub(crate) fn render_width_resize<S: Side + 'static>(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        div()
            .id(S::id())
            .absolute()
            .top(S::corner_clearance(window))
            .bottom(S::corner_clearance(window))
            .w(S::handle_width(window))
            .cursor_col_resize()
            .map(|this| S::position(this, window))
            .block_mouse_except_scroll()
            .on_mouse_down(MouseButton::Left, do_nothing)
            .on_drag(
                HorizontalResizeDrag::<S>::start_new(self.shape, self.preview.as_ref(), window),
                |_, _, _, cx| cx.new(|_| DragPreview),
            )
            .on_drag_move::<HorizontalResizeDrag<S>>(cx.listener(
                move |this, event: &DragMoveEvent<HorizontalResizeDrag<S>>, _, cx| {
                    let drag = event.drag(cx);
                    let delta = event.event.position.x - drag.mouse_pos_before.x;
                    let shape_before = drag.shape_before;
                    this.shape =
                        Shape::Resizing(S::current_position_and_shape(shape_before, delta));
                    // The transient `Resizing` shape is converted back to the resting,
                    // serializable form in `Picker::render` once the drag ends.
                    cx.notify();
                },
            ))
    }

    /// Resizes the picker model by extending it on the top or bottom
    pub(crate) fn render_height_resize<S: Side + 'static>(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        div()
            .id(S::id())
            .absolute()
            .left(S::corner_clearance(window))
            .right(S::corner_clearance(window))
            .h(S::handle_width(window))
            .cursor_row_resize()
            .map(|this| S::position(this, window))
            .block_mouse_except_scroll()
            .on_mouse_down(MouseButton::Left, do_nothing)
            .on_drag(
                VerticalResizeDrag::<S>::start_new(self.shape, self.preview.as_ref(), window),
                |_, _, _, cx| cx.new(|_| DragPreview),
            )
            .on_drag_move::<VerticalResizeDrag<S>>(cx.listener(
                move |this, event: &DragMoveEvent<VerticalResizeDrag<S>>, _, cx| {
                    let drag = event.drag(cx);
                    let delta = event.event.position.y - drag.mouse_pos_before.y;
                    let shape_before = drag.shape_before;
                    this.shape =
                        Shape::Resizing(S::current_position_and_shape(shape_before, delta));
                    // The transient `Resizing` shape is converted back to the resting,
                    // serializable form in `Picker::render` once the drag ends.
                    cx.notify();
                },
            ))
    }

    // TODO!(yara) enable and fix
    // pub(crate) fn render_corner_resize(
    //     &self,
    //     horizontal_side: ResizeSide,
    //     vertical_side: ResizeSide,
    //     window: &mut Window,
    //     cx: &mut Context<Self>,
    // ) -> impl IntoElement + use<> {
    //     let handle_size = px(RESIZE_CORNER_HANDLE_SIZE);
    //     let handle_offset = handle_size / 2.0;
    //     let corner_id: u32 = match (horizontal_side, vertical_side) {
    //         (ResizeSide::Start, ResizeSide::Start) => 0,
    //         (ResizeSide::End, ResizeSide::Start) => 1,
    //         (ResizeSide::Start, ResizeSide::End) => 2,
    //         (ResizeSide::End, ResizeSide::End) => 3,
    //     };
    //     let (min_width_rems, max_width_rems) = match self.preview {
    //          Some(preview) => preview.min_max_width(),
    //          None =>
    //     }
    //     let min_width = rems(min_width_rems).to_pixels(window.rem_size());
    //     let max_width = rems(max_width_rems).to_pixels(window.rem_size());

    //     div()
    //         .id(("corner-resize-handle", corner_id))
    //         .absolute()
    //         .w(handle_size)
    //         .h(handle_size)
    //         .map(|this| match horizontal_side {
    //             ResizeSide::Start => this.left(-handle_offset),
    //             ResizeSide::End => this.right(-handle_offset),
    //         })
    //         .map(|this| match vertical_side {
    //             ResizeSide::Start => this.top(-handle_offset),
    //             ResizeSide::End => this.bottom(-handle_offset),
    //         })
    //         .map(|this| match (horizontal_side, vertical_side) {
    //             (ResizeSide::Start, ResizeSide::Start) | (ResizeSide::End, ResizeSide::End) => {
    //                 this.cursor_nwse_resize()
    //             }
    //             (ResizeSide::Start, ResizeSide::End) | (ResizeSide::End, ResizeSide::Start) => {
    //                 this.cursor_nesw_resize()
    //             }
    //         })
    //         .block_mouse_except_scroll()
    //         .on_mouse_down(MouseButton::Left, handle_resize_mouse_down)
    //         .on_drag(
    //             CornerResizeDrag {
    //                 horizontal_side,
    //                 vertical_side,
    //                 mouse_start: window.mouse_position(),
    //                 width_start: self.modal_width,
    //                 preview_width_start: self.telescope.preview_width,
    //                 results_height_start: self.stacked.results_height,
    //                 preview_height_start: self.stacked.preview_height,
    //                 content_height_start: self.telescope.content_height,
    //                 offset_start: self.offset,
    //             },
    //             |_, _, _, cx| cx.new(|_| DragPreview),
    //         )
    //         .on_drag_move::<CornerResizeDrag>(cx.listener(
    //             move |this, event: &DragMoveEvent<CornerResizeDrag>, _window, cx| {
    //                 let drag = event.drag(cx);
    //                 let delta = event.event.position - drag.mouse_start;

    //                 let width_delta = match drag.horizontal_side {
    //                     ResizeSide::Start => -delta.x,
    //                     ResizeSide::End => delta.x,
    //                 };
    //                 let new_width = (drag.width_start + width_delta)
    //                     .max(min_width)
    //                     .min(max_width);
    //                 let width_change = new_width - drag.width_start;
    //                 this.modal_width = new_width;

    //                 if this.layout_mode == LayoutMode::Telescope && drag.width_start > px(0.0) {
    //                     let ratio = drag.preview_width_start / drag.width_start;
    //                     this.telescope.preview_width = (new_width * ratio)
    //                         .max(px(TelescopeLayoutState::MIN_PREVIEW_WIDTH))
    //                         .min(px(TelescopeLayoutState::MAX_PREVIEW_WIDTH));
    //                 }

    //                 this.offset.x = drag.offset_start.x
    //                     + match drag.horizontal_side {
    //                         ResizeSide::Start => -(width_change / 2.0),
    //                         ResizeSide::End => width_change / 2.0,
    //                     };

    //                 match this.layout_mode {
    //                     LayoutMode::Stacked => {
    //                         let height_delta = match drag.vertical_side {
    //                             ResizeSide::Start => -delta.y,
    //                             ResizeSide::End => delta.y,
    //                         };
    //                         let total_start = drag.results_height_start + drag.preview_height_start;
    //                         let min_total = px(StackedLayoutState::MIN_PANEL_HEIGHT * 2.0);
    //                         let new_total = (total_start + height_delta).max(min_total);
    //                         let scale = new_total / total_start;

    //                         this.stacked.results_height = drag.results_height_start * scale;
    //                         this.stacked.preview_height = drag.preview_height_start * scale;

    //                         if drag.vertical_side == ResizeSide::Start {
    //                             let actual_growth = new_total - total_start;
    //                             this.offset.y = drag.offset_start.y - actual_growth;
    //                         }
    //                     }
    //                     LayoutMode::Telescope => {
    //                         let height_delta = match drag.vertical_side {
    //                             ResizeSide::Start => -delta.y,
    //                             ResizeSide::End => delta.y,
    //                         };
    //                         let new_height = (drag.content_height_start + height_delta)
    //                             .max(px(TelescopeLayoutState::MIN_CONTENT_HEIGHT))
    //                             .min(px(TelescopeLayoutState::MAX_CONTENT_HEIGHT));

    //                         this.telescope.content_height = new_height;

    //                         if drag.vertical_side == ResizeSide::Start {
    //                             let actual_growth = new_height - drag.content_height_start;
    //                             this.offset.y = drag.offset_start.y - actual_growth;
    //                         }
    //                     }
    //                 }

    //                 cx.notify();
    //             },
    //         ))
    // }

    pub(crate) fn render_header_controls(
        &self,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<impl IntoElement> {
        let preview = self.preview.as_ref()?;

        Some(h_flex().gap_1().items_center().child({
            let focus_handle = self.focus_handle(cx);
            let (icon, tooltip_text) = match preview.layout {
                LayoutMode::Hidden => (IconName::Split, "Show preview to the right"),
                LayoutMode::Telescope(_) => (IconName::ListTree, "Show preview below"),
                LayoutMode::Stacked(_) => (IconName::ListCollapse, "Hide Preview"),
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

pub(crate) fn render_split_menu(
    split_menu_handle: PopoverMenuHandle<ContextMenu>,
    focus_handle: FocusHandle,
    _window: &mut Window,
    cx: &mut App,
) -> impl IntoElement {
    PopoverMenu::new("split-menu-popover")
        .with_handle(split_menu_handle)
        .attach(gpui::Anchor::BottomRight)
        .anchor(gpui::Anchor::TopRight)
        .offset(gpui::Point {
            x: px(0.0),
            y: px(-2.0),
        })
        .trigger_with_tooltip(
            ButtonLike::new("split-trigger")
                .child(Label::new("Split…").size(LabelSize::Small))
                .selected_style(ButtonStyle::Tinted(TintColor::Accent))
                .child(
                    ui::KeyBinding::for_action_in(&ToggleSplitMenu, &focus_handle, cx)
                        .size(rems_from_px(10.)),
                ),
            {
                let focus_handle = focus_handle.clone();
                move |_window, cx| {
                    Tooltip::for_action_in("Open in Split", &ToggleSplitMenu, &focus_handle, cx)
                }
            },
        )
        .menu({
            let focus_handle = focus_handle.clone();
            move |window, cx| {
                Some(ContextMenu::build(window, cx, {
                    let focus_handle = focus_handle.clone();
                    move |menu, _, _| {
                        menu.context(focus_handle)
                            .action("Split Left", pane::SplitLeft::default().boxed_clone())
                            .action("Split Right", pane::SplitRight::default().boxed_clone())
                            .action("Split Up", pane::SplitUp::default().boxed_clone())
                            .action("Split Down", pane::SplitDown::default().boxed_clone())
                    }
                }))
            }
        })
}

pub(crate) fn highlighted_drag_preview<T>(
    is_highlighted: gpui::Entity<bool>,
) -> impl Fn(&T, gpui::Point<Pixels>, &mut Window, &mut App) -> gpui::Entity<DragPreview> {
    move |_, _, _, cx| {
        is_highlighted.write(cx, true);
        cx.new(|_| DragPreview)
    }
}

pub(crate) fn clear_resize_highlight<T>(
    is_highlighted: Entity<bool>,
) -> impl Fn(&T, &mut Window, &mut App) {
    move |_, _, cx| is_highlighted.write(cx, false)
}
