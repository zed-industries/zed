use std::ops::Range;

use gpui::{
    Action, Context, DragMoveEvent, Entity, FocusHandle, Length, MouseButton, Styled, Window,
};
use ui::{ButtonLike, ContextMenu, PopoverMenu, PopoverMenuHandle, TintColor, Tooltip, prelude::*};
use workspace::pane;

use crate::{
    Picker, PickerDelegate, Preview, ToggleSplitMenu,
    preview::{
        render::do_nothing,
        state::{LayoutMode, TelescopeLayout},
    },
};

pub(crate) const RESIZE_HANDLE_WIDTH: f32 = 6.0;
pub(crate) const RESIZE_HANDLE_HEIGHT: f32 = 6.0;
pub(crate) const RESIZE_DIVIDER_SIZE: f32 = 1.0;
pub(crate) const RESIZE_CORNER_CLEARANCE: f32 = 18.0;

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
struct VerticalResizeDrag {
    side: ResizeSide,
    mouse_start: Pixels,
    results_height_start: Pixels,
    preview_height_start: Pixels,
    offset_start: Pixels,
}

#[derive(Clone, Copy)]
struct HorizontalResizeDrag {
    side: ResizeSide,
    mouse_start: Pixels,
    width_start: Pixels,
    preview_width_start: Pixels,
    left_extend: Pixels,
    right_extend: Pixels,
}

#[derive(Clone, Copy)]
struct CornerResizeDrag {
    horizontal_side: ResizeSide,
    vertical_side: ResizeSide,
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
    pub(crate) side: ResizeSide,
    pub(crate) mouse_start_y: Pixels,
    pub(crate) content_height_start: Pixels,
    pub(crate) offset_start: Pixels,
}

#[derive(Clone, Copy, PartialEq)]
pub(crate) enum ResizeSide {
    Left,
    Right,
}

// TODO!(yara) make this all work for with and without preview
impl<D: PickerDelegate> Picker<D> {
    pub(crate) fn render_horizontal_resize(
        &self,
        side: ResizeSide,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let handle_width = px(RESIZE_HANDLE_WIDTH);
        let handle_offset = handle_width / 2.0;
        let corner_clearance = px(RESIZE_CORNER_CLEARANCE);

        div()
            .id(match side {
                ResizeSide::Left => "left-resize-handle",
                ResizeSide::Right => "right-resize-handle",
            })
            .absolute()
            .top(corner_clearance)
            .bottom(corner_clearance)
            .w(handle_width)
            .cursor_col_resize()
            .map(|this| match side {
                ResizeSide::Left => this.left(-handle_offset),
                ResizeSide::Right => this.right(-handle_offset),
            })
            .block_mouse_except_scroll()
            .on_mouse_down(MouseButton::Left, do_nothing)
            .on_drag(
                // TODO keep adding logic conditional on preview being Some or None. Work trough all the resize controls like that.
                HorizontalResizeDrag {
                    side,
                    mouse_start: window.mouse_position().x,
                    width_start: self.shape.base_width(window),
                    preview_width_start: self
                        .preview
                        .as_ref()
                        .map(Preview::width)
                        .unwrap_or(Pixels::ZERO),
                    left_extend: self.shape.left_extend(window),
                    right_extend: self.shape.right_extend(window),
                },
                |_, _, _, cx| cx.new(|_| DragPreview),
            )
            .on_drag_move::<HorizontalResizeDrag>(cx.listener(
                move |this, event: &DragMoveEvent<HorizontalResizeDrag>, window, cx| {
                    let drag = event.drag(cx);
                    let delta = event.event.position.x - drag.mouse_start;
                    let width_delta = match drag.side {
                        ResizeSide::Left => -delta,
                        ResizeSide::Right => delta,
                    };
                    let new_width = (drag.width_start + width_delta)
                        .max(this.shape.min_width(window))
                        .min(this.shape.max_width(window));

                    let width_change = new_width - drag.width_start;
                    this.shape.base_width = Rems::from_pixels(new_width, window);

                    if let Some(Preview {
                        layout: LayoutMode::Telescope(layout),
                        ..
                    }) = this.preview.as_mut()
                        && drag.width_start > px(0.0)
                    {
                        let ratio = drag.preview_width_start / drag.width_start;
                        let new_preview_width = (new_width * ratio)
                            .max(px(TelescopeLayout::MIN_PREVIEW_WIDTH))
                            .min(px(TelescopeLayout::MAX_PREVIEW_WIDTH));
                        layout.preview_width = new_preview_width;
                    }

                    let offset_delta = width_change / 2.0;
                    match drag.side {
                        ResizeSide::Left => {
                            this.shape.left_extend +=
                                Rems::from_pixels(offset_delta, window);
                        }
                        ResizeSide::Right => {
                            this.shape.right_extend +=
                                Rems::from_pixels(offset_delta, window);
                        }
                    }
                    cx.notify();
                },
            ))
    }

    // TODO!(yara) enable and fix
    // pub(crate) fn render_vertical_resize(
    //     &self,
    //     side: ResizeSide,
    //     window: &mut Window,
    //     cx: &mut Context<Self>,
    // ) -> impl IntoElement {
    //     let handle_height = px(RESIZE_HANDLE_HEIGHT);
    //     let handle_offset = handle_height / 2.0;
    //     let corner_clearance = px(RESIZE_CORNER_CLEARANCE);

    //     div()
    //         .id(match side {
    //             ResizeSide::Start => "top-resize-handle",
    //             ResizeSide::End => "bottom-resize-handle",
    //         })
    //         .h(handle_height)
    //         .w_full()
    //         .cursor_row_resize()
    //         .map(|this| match side {
    //             ResizeSide::Start => this
    //                 .absolute()
    //                 .top(-handle_offset)
    //                 .left(corner_clearance)
    //                 .right(corner_clearance),
    //             ResizeSide::End => this.ml(corner_clearance).mr(corner_clearance),
    //         })
    //         .block_mouse_except_scroll()
    //         .on_mouse_down(MouseButton::Left, handle_resize_mouse_down)
    //         .on_drag(
    //             VerticalResizeDrag {
    //                 side,
    //                 mouse_start: window.mouse_position().y,
    //                 results_height_start: self.stacked.results_height,
    //                 preview_height_start: self.stacked.preview_height,
    //                 offset_start: self.offset.y,
    //             },
    //             |_, _, _, cx| cx.new(|_| DragPreview),
    //         )
    //         .on_drag_move::<VerticalResizeDrag>(cx.listener(
    //             move |this, event: &DragMoveEvent<VerticalResizeDrag>, _window, cx| {
    //                 let drag = event.drag(cx);
    //                 let delta = event.event.position.y - drag.mouse_start;
    //                 let total_growth = match drag.side {
    //                     ResizeSide::Start => -delta,
    //                     ResizeSide::End => delta,
    //                 };
    //                 let total_start = drag.results_height_start + drag.preview_height_start;
    //                 let min_total = px(StackedLayoutState::MIN_PANEL_HEIGHT * 2.0);

    //                 let new_total = (total_start + total_growth).max(min_total);
    //                 let scale = new_total / total_start;

    //                 this.stacked.results_height = drag.results_height_start * scale;
    //                 this.stacked.preview_height = drag.preview_height_start * scale;

    //                 if drag.side == ResizeSide::Start {
    //                     let actual_growth = new_total - total_start;
    //                     this.offset.y = drag.offset_start - actual_growth;
    //                 }
    //                 cx.notify();
    //             },
    //         ))
    // }

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

    // pub(crate) fn render_header_controls(
    //     &self,
    //     _window: &mut Window,
    //     cx: &mut Context<Self>,
    // ) -> impl IntoElement {
    //     let delegate = &self.picker.read(cx).delegate;
    //     let replace_enabled = delegate.replace_enabled;
    //     let filters_enabled = delegate.filters_enabled;
    //     let selected_index = delegate.selected_index;
    //     let match_count = delegate.matches.len();

    //     h_flex()
    //         .gap_1()
    //         .items_center()
    //         .child({
    //             let focus_handle = self.picker.focus_handle(cx);
    //             IconButton::new("replace-toggle", IconName::Replace)
    //                 .size(ButtonSize::Compact)
    //                 .toggle_state(replace_enabled)
    //                 .tooltip(move |_window, cx| {
    //                     Tooltip::for_action_in("Toggle Replace", &ToggleReplace, &focus_handle, cx)
    //                 })
    //                 .on_click(|_, window, cx| {
    //                     window.dispatch_action(ToggleReplace.boxed_clone(), cx);
    //                 })
    //         })
    //         .child({
    //             let focus_handle = self.picker.focus_handle(cx);
    //             IconButton::new("filters-toggle", IconName::Filter)
    //                 .size(ButtonSize::Compact)
    //                 .toggle_state(filters_enabled)
    //                 .tooltip(move |_window, cx| {
    //                     Tooltip::for_action_in("Toggle Filters", &ToggleFilters, &focus_handle, cx)
    //                 })
    //                 .on_click(|_, window, cx| {
    //                     window.dispatch_action(ToggleFilters.boxed_clone(), cx);
    //                 })
    //         })
    //         .child({
    //             let focus_handle = self.picker.focus_handle(cx);
    //             let (icon, tooltip_text) = match self.layout_mode {
    //                 LayoutMode::Stacked => (IconName::Split, "Switch to Telescope Layout"),
    //                 LayoutMode::Telescope => (IconName::ListTree, "Switch to Stacked Layout"),
    //             };
    //             IconButton::new("layout-toggle", icon)
    //                 .size(ButtonSize::Compact)
    //                 .tooltip(move |_window, cx| {
    //                     Tooltip::for_action_in(tooltip_text, &ToggleLayout, &focus_handle, cx)
    //                 })
    //                 .on_click(|_, window, cx| {
    //                     window.dispatch_action(ToggleLayout.boxed_clone(), cx);
    //                 })
    //         })
    //         .child({
    //             let focus_handle = self.picker.focus_handle(cx);
    //             IconButton::new("select-prev-match", IconName::ChevronLeft)
    //                 .size(ButtonSize::Compact)
    //                 .tooltip(move |_window, cx| {
    //                     Tooltip::for_action_in(
    //                         "Previous Match",
    //                         &SelectPreviousMatch,
    //                         &focus_handle,
    //                         cx,
    //                     )
    //                 })
    //                 .on_click(|_, window, cx| {
    //                     window.dispatch_action(SelectPreviousMatch.boxed_clone(), cx);
    //                 })
    //         })
    //         .child({
    //             let focus_handle = self.picker.focus_handle(cx);
    //             IconButton::new("select-next-match", IconName::ChevronRight)
    //                 .size(ButtonSize::Compact)
    //                 .tooltip(move |_window, cx| {
    //                     Tooltip::for_action_in("Next Match", &SelectNextMatch, &focus_handle, cx)
    //                 })
    //                 .on_click(|_, window, cx| {
    //                     window.dispatch_action(SelectNextMatch.boxed_clone(), cx);
    //                 })
    //         })
    //         .when(match_count > 0, |this| {
    //             this.child(
    //                 Label::new(format!("{}/{}", selected_index + 1, match_count))
    //                     .size(LabelSize::Small)
    //                     .color(Color::Muted),
    //             )
    //         })
    // }
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
