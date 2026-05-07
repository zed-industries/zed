use crate::{
    SelectNextMatch, SelectPreviousMatch, ToggleReplace,
    quick_search::{
        LayoutMode, QuickSearch, RESIZE_CORNER_HANDLE_SIZE, ResizeSide, ToggleFilters,
        ToggleLayout, ToggleSplitMenu, handle_resize_mouse_down,
        render::DragPreview,
        state::{StackedLayoutState, TelescopeLayoutState},
    },
};
use gpui::{Action, Context, DragMoveEvent, FocusHandle, Focusable, MouseButton, Styled, Window};
use ui::{ButtonLike, ContextMenu, PopoverMenu, PopoverMenuHandle, TintColor, Tooltip, prelude::*};
use workspace::pane;

pub(crate) const RESIZE_HANDLE_WIDTH: f32 = 6.0;
pub(crate) const RESIZE_HANDLE_HEIGHT: f32 = 6.0;
pub(crate) const RESIZE_DIVIDER_SIZE: f32 = 1.0;
pub(crate) const RESIZE_CORNER_CLEARANCE: f32 = 18.0;

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
    offset_start: Pixels,
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

impl QuickSearch {
    pub(crate) fn render_horizontal_resize(
        &self,
        side: ResizeSide,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let handle_width = px(RESIZE_HANDLE_WIDTH);
        let handle_offset = handle_width / 2.0;
        let corner_clearance = px(RESIZE_CORNER_CLEARANCE);
        let (min_width_rems, max_width_rems) = match self.layout_mode {
            LayoutMode::Stacked => (
                StackedLayoutState::MIN_MODAL_WIDTH_REMS,
                StackedLayoutState::MAX_MODAL_WIDTH_REMS,
            ),
            LayoutMode::Telescope => (
                TelescopeLayoutState::MIN_MODAL_WIDTH_REMS,
                TelescopeLayoutState::MAX_MODAL_WIDTH_REMS,
            ),
        };
        let min_width = rems(min_width_rems).to_pixels(window.rem_size());
        let max_width = rems(max_width_rems).to_pixels(window.rem_size());

        div()
            .id(match side {
                ResizeSide::Start => "left-resize-handle",
                ResizeSide::End => "right-resize-handle",
            })
            .absolute()
            .top(corner_clearance)
            .bottom(corner_clearance)
            .w(handle_width)
            .cursor_col_resize()
            .map(|this| match side {
                ResizeSide::Start => this.left(-handle_offset),
                ResizeSide::End => this.right(-handle_offset),
            })
            .block_mouse_except_scroll()
            .on_mouse_down(MouseButton::Left, handle_resize_mouse_down)
            .on_drag(
                HorizontalResizeDrag {
                    side,
                    mouse_start: window.mouse_position().x,
                    width_start: self.modal_width,
                    preview_width_start: self.telescope.preview_width,
                    offset_start: self.offset.x,
                },
                |_, _, _, cx| cx.new(|_| DragPreview),
            )
            .on_drag_move::<HorizontalResizeDrag>(cx.listener(
                move |this, event: &DragMoveEvent<HorizontalResizeDrag>, _window, cx| {
                    let drag = event.drag(cx);
                    let delta = event.event.position.x - drag.mouse_start;
                    let width_delta = match drag.side {
                        ResizeSide::Start => -delta,
                        ResizeSide::End => delta,
                    };
                    let new_width = (drag.width_start + width_delta)
                        .max(min_width)
                        .min(max_width);

                    let width_change = new_width - drag.width_start;
                    this.modal_width = new_width;

                    if this.layout_mode == LayoutMode::Telescope && drag.width_start > px(0.0) {
                        let ratio = drag.preview_width_start / drag.width_start;
                        let new_preview_width = (new_width * ratio)
                            .max(px(TelescopeLayoutState::MIN_PREVIEW_WIDTH))
                            .min(px(TelescopeLayoutState::MAX_PREVIEW_WIDTH));
                        this.telescope.preview_width = new_preview_width;
                    }

                    let offset_delta = width_change / 2.0;
                    this.offset.x = drag.offset_start
                        + match drag.side {
                            ResizeSide::Start => -offset_delta,
                            ResizeSide::End => offset_delta,
                        };
                    cx.notify();
                },
            ))
    }

    pub(crate) fn render_vertical_resize(
        &self,
        side: ResizeSide,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let handle_height = px(RESIZE_HANDLE_HEIGHT);
        let handle_offset = handle_height / 2.0;
        let corner_clearance = px(RESIZE_CORNER_CLEARANCE);

        div()
            .id(match side {
                ResizeSide::Start => "top-resize-handle",
                ResizeSide::End => "bottom-resize-handle",
            })
            .h(handle_height)
            .w_full()
            .cursor_row_resize()
            .map(|this| match side {
                ResizeSide::Start => this
                    .absolute()
                    .top(-handle_offset)
                    .left(corner_clearance)
                    .right(corner_clearance),
                ResizeSide::End => this.ml(corner_clearance).mr(corner_clearance),
            })
            .block_mouse_except_scroll()
            .on_mouse_down(MouseButton::Left, handle_resize_mouse_down)
            .on_drag(
                VerticalResizeDrag {
                    side,
                    mouse_start: window.mouse_position().y,
                    results_height_start: self.stacked.results_height,
                    preview_height_start: self.stacked.preview_height,
                    offset_start: self.offset.y,
                },
                |_, _, _, cx| cx.new(|_| DragPreview),
            )
            .on_drag_move::<VerticalResizeDrag>(cx.listener(
                move |this, event: &DragMoveEvent<VerticalResizeDrag>, _window, cx| {
                    let drag = event.drag(cx);
                    let delta = event.event.position.y - drag.mouse_start;
                    let total_growth = match drag.side {
                        ResizeSide::Start => -delta,
                        ResizeSide::End => delta,
                    };
                    let total_start = drag.results_height_start + drag.preview_height_start;
                    let min_total = px(StackedLayoutState::MIN_PANEL_HEIGHT * 2.0);

                    let new_total = (total_start + total_growth).max(min_total);
                    let scale = new_total / total_start;

                    this.stacked.results_height = drag.results_height_start * scale;
                    this.stacked.preview_height = drag.preview_height_start * scale;

                    if drag.side == ResizeSide::Start {
                        let actual_growth = new_total - total_start;
                        this.offset.y = drag.offset_start - actual_growth;
                    }
                    cx.notify();
                },
            ))
    }

    pub(crate) fn render_corner_resize(
        &self,
        horizontal_side: ResizeSide,
        vertical_side: ResizeSide,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement + use<> {
        let handle_size = px(RESIZE_CORNER_HANDLE_SIZE);
        let handle_offset = handle_size / 2.0;
        let corner_id: u32 = match (horizontal_side, vertical_side) {
            (ResizeSide::Start, ResizeSide::Start) => 0,
            (ResizeSide::End, ResizeSide::Start) => 1,
            (ResizeSide::Start, ResizeSide::End) => 2,
            (ResizeSide::End, ResizeSide::End) => 3,
        };
        let (min_width_rems, max_width_rems) = match self.layout_mode {
            LayoutMode::Stacked => (
                StackedLayoutState::MIN_MODAL_WIDTH_REMS,
                StackedLayoutState::MAX_MODAL_WIDTH_REMS,
            ),
            LayoutMode::Telescope => (
                TelescopeLayoutState::MIN_MODAL_WIDTH_REMS,
                TelescopeLayoutState::MAX_MODAL_WIDTH_REMS,
            ),
        };
        let min_width = rems(min_width_rems).to_pixels(window.rem_size());
        let max_width = rems(max_width_rems).to_pixels(window.rem_size());

        div()
            .id(("corner-resize-handle", corner_id))
            .absolute()
            .w(handle_size)
            .h(handle_size)
            .map(|this| match horizontal_side {
                ResizeSide::Start => this.left(-handle_offset),
                ResizeSide::End => this.right(-handle_offset),
            })
            .map(|this| match vertical_side {
                ResizeSide::Start => this.top(-handle_offset),
                ResizeSide::End => this.bottom(-handle_offset),
            })
            .map(|this| match (horizontal_side, vertical_side) {
                (ResizeSide::Start, ResizeSide::Start) | (ResizeSide::End, ResizeSide::End) => {
                    this.cursor_nwse_resize()
                }
                (ResizeSide::Start, ResizeSide::End) | (ResizeSide::End, ResizeSide::Start) => {
                    this.cursor_nesw_resize()
                }
            })
            .block_mouse_except_scroll()
            .on_mouse_down(MouseButton::Left, handle_resize_mouse_down)
            .on_drag(
                CornerResizeDrag {
                    horizontal_side,
                    vertical_side,
                    mouse_start: window.mouse_position(),
                    width_start: self.modal_width,
                    preview_width_start: self.telescope.preview_width,
                    results_height_start: self.stacked.results_height,
                    preview_height_start: self.stacked.preview_height,
                    content_height_start: self.telescope.content_height,
                    offset_start: self.offset,
                },
                |_, _, _, cx| cx.new(|_| DragPreview),
            )
            .on_drag_move::<CornerResizeDrag>(cx.listener(
                move |this, event: &DragMoveEvent<CornerResizeDrag>, _window, cx| {
                    let drag = event.drag(cx);
                    let delta = event.event.position - drag.mouse_start;

                    let width_delta = match drag.horizontal_side {
                        ResizeSide::Start => -delta.x,
                        ResizeSide::End => delta.x,
                    };
                    let new_width = (drag.width_start + width_delta)
                        .max(min_width)
                        .min(max_width);
                    let width_change = new_width - drag.width_start;
                    this.modal_width = new_width;

                    if this.layout_mode == LayoutMode::Telescope && drag.width_start > px(0.0) {
                        let ratio = drag.preview_width_start / drag.width_start;
                        this.telescope.preview_width = (new_width * ratio)
                            .max(px(TelescopeLayoutState::MIN_PREVIEW_WIDTH))
                            .min(px(TelescopeLayoutState::MAX_PREVIEW_WIDTH));
                    }

                    this.offset.x = drag.offset_start.x
                        + match drag.horizontal_side {
                            ResizeSide::Start => -(width_change / 2.0),
                            ResizeSide::End => width_change / 2.0,
                        };

                    match this.layout_mode {
                        LayoutMode::Stacked => {
                            let height_delta = match drag.vertical_side {
                                ResizeSide::Start => -delta.y,
                                ResizeSide::End => delta.y,
                            };
                            let total_start = drag.results_height_start + drag.preview_height_start;
                            let min_total = px(StackedLayoutState::MIN_PANEL_HEIGHT * 2.0);
                            let new_total = (total_start + height_delta).max(min_total);
                            let scale = new_total / total_start;

                            this.stacked.results_height = drag.results_height_start * scale;
                            this.stacked.preview_height = drag.preview_height_start * scale;

                            if drag.vertical_side == ResizeSide::Start {
                                let actual_growth = new_total - total_start;
                                this.offset.y = drag.offset_start.y - actual_growth;
                            }
                        }
                        LayoutMode::Telescope => {
                            let height_delta = match drag.vertical_side {
                                ResizeSide::Start => -delta.y,
                                ResizeSide::End => delta.y,
                            };
                            let new_height = (drag.content_height_start + height_delta)
                                .max(px(TelescopeLayoutState::MIN_CONTENT_HEIGHT))
                                .min(px(TelescopeLayoutState::MAX_CONTENT_HEIGHT));

                            this.telescope.content_height = new_height;

                            if drag.vertical_side == ResizeSide::Start {
                                let actual_growth = new_height - drag.content_height_start;
                                this.offset.y = drag.offset_start.y - actual_growth;
                            }
                        }
                    }

                    cx.notify();
                },
            ))
    }

    pub(crate) fn render_header_controls(
        &self,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let delegate = &self.picker.read(cx).delegate;
        let replace_enabled = delegate.replace_enabled;
        let filters_enabled = delegate.filters_enabled;
        let selected_index = delegate.selected_index;
        let match_count = delegate.matches.len();

        h_flex()
            .gap_1()
            .items_center()
            .child({
                let focus_handle = self.picker.focus_handle(cx);
                IconButton::new("replace-toggle", IconName::Replace)
                    .size(ButtonSize::Compact)
                    .toggle_state(replace_enabled)
                    .tooltip(move |_window, cx| {
                        Tooltip::for_action_in("Toggle Replace", &ToggleReplace, &focus_handle, cx)
                    })
                    .on_click(|_, window, cx| {
                        window.dispatch_action(ToggleReplace.boxed_clone(), cx);
                    })
            })
            .child({
                let focus_handle = self.picker.focus_handle(cx);
                IconButton::new("filters-toggle", IconName::Filter)
                    .size(ButtonSize::Compact)
                    .toggle_state(filters_enabled)
                    .tooltip(move |_window, cx| {
                        Tooltip::for_action_in("Toggle Filters", &ToggleFilters, &focus_handle, cx)
                    })
                    .on_click(|_, window, cx| {
                        window.dispatch_action(ToggleFilters.boxed_clone(), cx);
                    })
            })
            .child({
                let focus_handle = self.picker.focus_handle(cx);
                let (icon, tooltip_text) = match self.layout_mode {
                    LayoutMode::Stacked => (IconName::Split, "Switch to Telescope Layout"),
                    LayoutMode::Telescope => (IconName::ListTree, "Switch to Stacked Layout"),
                };
                IconButton::new("layout-toggle", icon)
                    .size(ButtonSize::Compact)
                    .tooltip(move |_window, cx| {
                        Tooltip::for_action_in(tooltip_text, &ToggleLayout, &focus_handle, cx)
                    })
                    .on_click(|_, window, cx| {
                        window.dispatch_action(ToggleLayout.boxed_clone(), cx);
                    })
            })
            .child({
                let focus_handle = self.picker.focus_handle(cx);
                IconButton::new("select-prev-match", IconName::ChevronLeft)
                    .size(ButtonSize::Compact)
                    .tooltip(move |_window, cx| {
                        Tooltip::for_action_in(
                            "Previous Match",
                            &SelectPreviousMatch,
                            &focus_handle,
                            cx,
                        )
                    })
                    .on_click(|_, window, cx| {
                        window.dispatch_action(SelectPreviousMatch.boxed_clone(), cx);
                    })
            })
            .child({
                let focus_handle = self.picker.focus_handle(cx);
                IconButton::new("select-next-match", IconName::ChevronRight)
                    .size(ButtonSize::Compact)
                    .tooltip(move |_window, cx| {
                        Tooltip::for_action_in("Next Match", &SelectNextMatch, &focus_handle, cx)
                    })
                    .on_click(|_, window, cx| {
                        window.dispatch_action(SelectNextMatch.boxed_clone(), cx);
                    })
            })
            .when(match_count > 0, |this| {
                this.child(
                    Label::new(format!("{}/{}", selected_index + 1, match_count))
                        .size(LabelSize::Small)
                        .color(Color::Muted),
                )
            })
    }
}

pub(crate) fn render_split_menu(
    split_menu_handle: PopoverMenuHandle<ContextMenu>,
    focus_handle: FocusHandle,
    _window: &mut Window,
    cx: &mut Context<QuickSearch>,
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
