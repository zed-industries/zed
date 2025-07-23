use std::{ops::Range, rc::Rc, time::Duration};

use editor::{EditorSettings, ShowScrollbar, scroll::ScrollbarAutoHide};
use gpui::{
    AbsoluteLength, AppContext, Axis, Context, DefiniteLength, DragMoveEvent, Entity, FocusHandle,
    Length, ListHorizontalSizingBehavior, ListSizingBehavior, MouseButton, Point, Stateful, Task,
    UniformListScrollHandle, WeakEntity, transparent_black, uniform_list,
};

use itertools::intersperse_with;
use settings::Settings as _;
use ui::{
    ActiveTheme as _, AnyElement, App, Button, ButtonCommon as _, ButtonStyle, Color, Component,
    ComponentScope, Div, ElementId, FixedWidth as _, FluentBuilder as _, Indicator,
    InteractiveElement, IntoElement, ParentElement, Pixels, RegisterComponent, RenderOnce,
    Scrollbar, ScrollbarState, StatefulInteractiveElement, Styled, StyledExt as _,
    StyledTypography, Window, div, example_group_with_title, h_flex, px, single_example, v_flex,
};

#[derive(Debug)]
struct DraggedColumn(usize);

struct UniformListData<const COLS: usize> {
    render_item_fn: Box<dyn Fn(Range<usize>, &mut Window, &mut App) -> Vec<[AnyElement; COLS]>>,
    element_id: ElementId,
    row_count: usize,
}

enum TableContents<const COLS: usize> {
    Vec(Vec<[AnyElement; COLS]>),
    UniformList(UniformListData<COLS>),
}

impl<const COLS: usize> TableContents<COLS> {
    fn rows_mut(&mut self) -> Option<&mut Vec<[AnyElement; COLS]>> {
        match self {
            TableContents::Vec(rows) => Some(rows),
            TableContents::UniformList(_) => None,
        }
    }

    fn len(&self) -> usize {
        match self {
            TableContents::Vec(rows) => rows.len(),
            TableContents::UniformList(data) => data.row_count,
        }
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

pub struct TableInteractionState {
    pub focus_handle: FocusHandle,
    pub scroll_handle: UniformListScrollHandle,
    pub horizontal_scrollbar: ScrollbarProperties,
    pub vertical_scrollbar: ScrollbarProperties,
}

impl TableInteractionState {
    pub fn new(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| {
            let focus_handle = cx.focus_handle();

            cx.on_focus_out(&focus_handle, window, |this: &mut Self, _, window, cx| {
                this.hide_scrollbars(window, cx);
            })
            .detach();

            let scroll_handle = UniformListScrollHandle::new();
            let vertical_scrollbar = ScrollbarProperties {
                axis: Axis::Vertical,
                state: ScrollbarState::new(scroll_handle.clone()).parent_entity(&cx.entity()),
                show_scrollbar: false,
                show_track: false,
                auto_hide: false,
                hide_task: None,
            };

            let horizontal_scrollbar = ScrollbarProperties {
                axis: Axis::Horizontal,
                state: ScrollbarState::new(scroll_handle.clone()).parent_entity(&cx.entity()),
                show_scrollbar: false,
                show_track: false,
                auto_hide: false,
                hide_task: None,
            };

            let mut this = Self {
                focus_handle,
                scroll_handle,
                horizontal_scrollbar,
                vertical_scrollbar,
            };

            this.update_scrollbar_visibility(cx);
            this
        })
    }

    pub fn get_scrollbar_offset(&self, axis: Axis) -> Point<Pixels> {
        match axis {
            Axis::Vertical => self.vertical_scrollbar.state.scroll_handle().offset(),
            Axis::Horizontal => self.horizontal_scrollbar.state.scroll_handle().offset(),
        }
    }

    pub fn set_scrollbar_offset(&self, axis: Axis, offset: Point<Pixels>) {
        match axis {
            Axis::Vertical => self
                .vertical_scrollbar
                .state
                .scroll_handle()
                .set_offset(offset),
            Axis::Horizontal => self
                .horizontal_scrollbar
                .state
                .scroll_handle()
                .set_offset(offset),
        }
    }

    fn update_scrollbar_visibility(&mut self, cx: &mut Context<Self>) {
        let show_setting = EditorSettings::get_global(cx).scrollbar.show;

        let scroll_handle = self.scroll_handle.0.borrow();

        let autohide = |show: ShowScrollbar, cx: &mut Context<Self>| match show {
            ShowScrollbar::Auto => true,
            ShowScrollbar::System => cx
                .try_global::<ScrollbarAutoHide>()
                .map_or_else(|| cx.should_auto_hide_scrollbars(), |autohide| autohide.0),
            ShowScrollbar::Always => false,
            ShowScrollbar::Never => false,
        };

        let longest_item_width = scroll_handle.last_item_size.and_then(|size| {
            (size.contents.width > size.item.width).then_some(size.contents.width)
        });

        // is there an item long enough that we should show a horizontal scrollbar?
        let item_wider_than_container = if let Some(longest_item_width) = longest_item_width {
            longest_item_width > px(scroll_handle.base_handle.bounds().size.width.0)
        } else {
            true
        };

        let show_scrollbar = match show_setting {
            ShowScrollbar::Auto | ShowScrollbar::System | ShowScrollbar::Always => true,
            ShowScrollbar::Never => false,
        };
        let show_vertical = show_scrollbar;

        let show_horizontal = item_wider_than_container && show_scrollbar;

        let show_horizontal_track =
            show_horizontal && matches!(show_setting, ShowScrollbar::Always);

        // TODO: we probably should hide the scroll track when the list doesn't need to scroll
        let show_vertical_track = show_vertical && matches!(show_setting, ShowScrollbar::Always);

        self.vertical_scrollbar = ScrollbarProperties {
            axis: self.vertical_scrollbar.axis,
            state: self.vertical_scrollbar.state.clone(),
            show_scrollbar: show_vertical,
            show_track: show_vertical_track,
            auto_hide: autohide(show_setting, cx),
            hide_task: None,
        };

        self.horizontal_scrollbar = ScrollbarProperties {
            axis: self.horizontal_scrollbar.axis,
            state: self.horizontal_scrollbar.state.clone(),
            show_scrollbar: show_horizontal,
            show_track: show_horizontal_track,
            auto_hide: autohide(show_setting, cx),
            hide_task: None,
        };

        cx.notify();
    }

    fn hide_scrollbars(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.horizontal_scrollbar.hide(window, cx);
        self.vertical_scrollbar.hide(window, cx);
    }

    pub fn listener<E: ?Sized>(
        this: &Entity<Self>,
        f: impl Fn(&mut Self, &E, &mut Window, &mut Context<Self>) + 'static,
    ) -> impl Fn(&E, &mut Window, &mut App) + 'static {
        let view = this.downgrade();
        move |e: &E, window: &mut Window, cx: &mut App| {
            view.update(cx, |view, cx| f(view, e, window, cx)).ok();
        }
    }

    fn render_resize_handles<const COLS: usize>(
        &self,
        column_widths: &[Length; COLS],
        resizable_columns: &[ResizeBehavior; COLS],
        initial_sizes: [DefiniteLength; COLS],
        columns: Option<Entity<ColumnWidths<COLS>>>,
        window: &mut Window,
        cx: &mut App,
    ) -> AnyElement {
        let spacers = column_widths
            .iter()
            .map(|width| base_cell_style(Some(*width)).into_any_element());

        let mut column_ix = 0;
        let resizable_columns_slice = *resizable_columns;
        let mut resizable_columns = resizable_columns.into_iter();
        let dividers = intersperse_with(spacers, || {
            window.with_id(column_ix, |window| {
                let mut resize_divider = div()
                    // This is required because this is evaluated at a different time than the use_state call above
                    .id(column_ix)
                    .relative()
                    .top_0()
                    .w_0p5()
                    .h_full()
                    .bg(cx.theme().colors().border.opacity(0.5));

                let mut resize_handle = div()
                    .id("column-resize-handle")
                    .absolute()
                    .left_neg_0p5()
                    .w(px(5.0))
                    .h_full();

                if resizable_columns
                    .next()
                    .is_some_and(ResizeBehavior::is_resizable)
                {
                    let hovered = window.use_state(cx, |_window, _cx| false);
                    resize_divider = resize_divider.when(*hovered.read(cx), |div| {
                        div.bg(cx.theme().colors().border_focused)
                    });
                    resize_handle = resize_handle
                        .on_hover(move |&was_hovered, _, cx| hovered.write(cx, was_hovered))
                        .cursor_col_resize()
                        .when_some(columns.clone(), |this, columns| {
                            this.on_click(move |event, window, cx| {
                                if event.down.click_count >= 2 {
                                    columns.update(cx, |columns, _| {
                                        columns.on_double_click(
                                            column_ix,
                                            &initial_sizes,
                                            &resizable_columns_slice,
                                            window,
                                        );
                                    })
                                }

                                cx.stop_propagation();
                            })
                        })
                        .on_drag(DraggedColumn(column_ix), |_, _offset, _window, cx| {
                            cx.new(|_cx| gpui::Empty)
                        })
                }

                column_ix += 1;
                resize_divider.child(resize_handle).into_any_element()
            })
        });

        div()
            .id("resize-handles")
            .h_flex()
            .absolute()
            .w_full()
            .inset_0()
            .children(dividers)
            .into_any_element()
    }

    fn render_vertical_scrollbar_track(
        this: &Entity<Self>,
        parent: Div,
        scroll_track_size: Pixels,
        cx: &mut App,
    ) -> Div {
        if !this.read(cx).vertical_scrollbar.show_track {
            return parent;
        }
        let child = v_flex()
            .h_full()
            .flex_none()
            .w(scroll_track_size)
            .bg(cx.theme().colors().background)
            .child(
                div()
                    .size_full()
                    .flex_1()
                    .border_l_1()
                    .border_color(cx.theme().colors().border),
            );
        parent.child(child)
    }

    fn render_vertical_scrollbar(this: &Entity<Self>, parent: Div, cx: &mut App) -> Div {
        if !this.read(cx).vertical_scrollbar.show_scrollbar {
            return parent;
        }
        let child = div()
            .id(("table-vertical-scrollbar", this.entity_id()))
            .occlude()
            .flex_none()
            .h_full()
            .cursor_default()
            .absolute()
            .right_0()
            .top_0()
            .bottom_0()
            .w(px(12.))
            .on_mouse_move(Self::listener(this, |_, _, _, cx| {
                cx.notify();
                cx.stop_propagation()
            }))
            .on_hover(|_, _, cx| {
                cx.stop_propagation();
            })
            .on_mouse_up(
                MouseButton::Left,
                Self::listener(this, |this, _, window, cx| {
                    if !this.vertical_scrollbar.state.is_dragging()
                        && !this.focus_handle.contains_focused(window, cx)
                    {
                        this.vertical_scrollbar.hide(window, cx);
                        cx.notify();
                    }

                    cx.stop_propagation();
                }),
            )
            .on_any_mouse_down(|_, _, cx| {
                cx.stop_propagation();
            })
            .on_scroll_wheel(Self::listener(&this, |_, _, _, cx| {
                cx.notify();
            }))
            .children(Scrollbar::vertical(
                this.read(cx).vertical_scrollbar.state.clone(),
            ));
        parent.child(child)
    }

    /// Renders the horizontal scrollbar.
    ///
    /// The right offset is used to determine how far to the right the
    /// scrollbar should extend to, useful for ensuring it doesn't collide
    /// with the vertical scrollbar when visible.
    fn render_horizontal_scrollbar(
        this: &Entity<Self>,
        parent: Div,
        right_offset: Pixels,
        cx: &mut App,
    ) -> Div {
        if !this.read(cx).horizontal_scrollbar.show_scrollbar {
            return parent;
        }
        let child = div()
            .id(("table-horizontal-scrollbar", this.entity_id()))
            .occlude()
            .flex_none()
            .w_full()
            .cursor_default()
            .absolute()
            .bottom_neg_px()
            .left_0()
            .right_0()
            .pr(right_offset)
            .on_mouse_move(Self::listener(this, |_, _, _, cx| {
                cx.notify();
                cx.stop_propagation()
            }))
            .on_hover(|_, _, cx| {
                cx.stop_propagation();
            })
            .on_any_mouse_down(|_, _, cx| {
                cx.stop_propagation();
            })
            .on_mouse_up(
                MouseButton::Left,
                Self::listener(this, |this, _, window, cx| {
                    if !this.horizontal_scrollbar.state.is_dragging()
                        && !this.focus_handle.contains_focused(window, cx)
                    {
                        this.horizontal_scrollbar.hide(window, cx);
                        cx.notify();
                    }

                    cx.stop_propagation();
                }),
            )
            .on_scroll_wheel(Self::listener(this, |_, _, _, cx| {
                cx.notify();
            }))
            .children(Scrollbar::horizontal(
                // percentage as f32..end_offset as f32,
                this.read(cx).horizontal_scrollbar.state.clone(),
            ));
        parent.child(child)
    }

    fn render_horizontal_scrollbar_track(
        this: &Entity<Self>,
        parent: Div,
        scroll_track_size: Pixels,
        cx: &mut App,
    ) -> Div {
        if !this.read(cx).horizontal_scrollbar.show_track {
            return parent;
        }
        let child = h_flex()
            .w_full()
            .h(scroll_track_size)
            .flex_none()
            .relative()
            .child(
                div()
                    .w_full()
                    .flex_1()
                    // for some reason the horizontal scrollbar is 1px
                    // taller than the vertical scrollbar??
                    .h(scroll_track_size - px(1.))
                    .bg(cx.theme().colors().background)
                    .border_t_1()
                    .border_color(cx.theme().colors().border),
            )
            .when(this.read(cx).vertical_scrollbar.show_track, |parent| {
                parent
                    .child(
                        div()
                            .flex_none()
                            // -1px prevents a missing pixel between the two container borders
                            .w(scroll_track_size - px(1.))
                            .h_full(),
                    )
                    .child(
                        // HACK: Fill the missing 1px 🥲
                        div()
                            .absolute()
                            .right(scroll_track_size - px(1.))
                            .bottom(scroll_track_size - px(1.))
                            .size_px()
                            .bg(cx.theme().colors().border),
                    )
            });

        parent.child(child)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ResizeBehavior {
    None,
    Resizable,
    MinSize(f32),
}

impl ResizeBehavior {
    pub fn is_resizable(&self) -> bool {
        *self != ResizeBehavior::None
    }

    pub fn min_size(&self) -> Option<f32> {
        match self {
            ResizeBehavior::None => None,
            ResizeBehavior::Resizable => Some(0.05),
            ResizeBehavior::MinSize(min_size) => Some(*min_size),
        }
    }
}

pub struct ColumnWidths<const COLS: usize> {
    widths: [DefiniteLength; COLS],
    cached_bounds_width: Pixels,
    initialized: bool,
}

impl<const COLS: usize> ColumnWidths<COLS> {
    pub fn new(_: &mut App) -> Self {
        Self {
            widths: [DefiniteLength::default(); COLS],
            cached_bounds_width: Default::default(),
            initialized: false,
        }
    }

    fn get_fraction(length: &DefiniteLength, bounds_width: Pixels, rem_size: Pixels) -> f32 {
        match length {
            DefiniteLength::Absolute(AbsoluteLength::Pixels(pixels)) => *pixels / bounds_width,
            DefiniteLength::Absolute(AbsoluteLength::Rems(rems_width)) => {
                rems_width.to_pixels(rem_size) / bounds_width
            }
            DefiniteLength::Fraction(fraction) => *fraction,
        }
    }

    fn on_double_click(
        &mut self,
        double_click_position: usize,
        initial_sizes: &[DefiniteLength; COLS],
        resize_behavior: &[ResizeBehavior; COLS],
        window: &mut Window,
    ) {
        let bounds_width = self.cached_bounds_width;
        let rem_size = window.rem_size();
        let initial_sizes =
            initial_sizes.map(|length| Self::get_fraction(&length, bounds_width, rem_size));
        let mut widths = self
            .widths
            .map(|length| Self::get_fraction(&length, bounds_width, rem_size));

        let diff = initial_sizes[double_click_position] - widths[double_click_position];

        if diff > 0.0 {
            let diff_remaining = self.propagate_resize_diff_right(
                diff,
                double_click_position,
                &mut widths,
                resize_behavior,
            );

            if diff_remaining > 0.0 && double_click_position > 0 {
                self.propagate_resize_diff_left(
                    -diff_remaining,
                    double_click_position - 1,
                    &mut widths,
                    resize_behavior,
                );
            }
        } else if double_click_position > 0 {
            let diff_remaining = self.propagate_resize_diff_left(
                diff,
                double_click_position,
                &mut widths,
                resize_behavior,
            );

            if diff_remaining < 0.0 {
                self.propagate_resize_diff_right(
                    -diff_remaining,
                    double_click_position,
                    &mut widths,
                    resize_behavior,
                );
            }
        }
        self.widths = widths.map(DefiniteLength::Fraction);
    }

    fn on_drag_move(
        &mut self,
        drag_event: &DragMoveEvent<DraggedColumn>,
        resize_behavior: &[ResizeBehavior; COLS],
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let drag_position = drag_event.event.position;
        let bounds = drag_event.bounds;

        let mut col_position = 0.0;
        let rem_size = window.rem_size();
        let bounds_width = bounds.right() - bounds.left();
        let col_idx = drag_event.drag(cx).0;

        let mut widths = self
            .widths
            .map(|length| Self::get_fraction(&length, bounds_width, rem_size));

        for length in widths[0..=col_idx].iter() {
            col_position += length;
        }

        let mut total_length_ratio = col_position;
        for length in widths[col_idx + 1..].iter() {
            total_length_ratio += length;
        }

        let drag_fraction = (drag_position.x - bounds.left()) / bounds_width;
        let drag_fraction = drag_fraction * total_length_ratio;
        let diff = drag_fraction - col_position;

        let is_dragging_right = diff > 0.0;

        if is_dragging_right {
            self.propagate_resize_diff_right(diff, col_idx, &mut widths, resize_behavior);
        } else {
            // Resize behavior should be improved in the future by also seeking to the right column when there's not enough space
            self.propagate_resize_diff_left(diff, col_idx, &mut widths, resize_behavior);
        }
        self.widths = widths.map(DefiniteLength::Fraction);
    }

    fn propagate_resize_diff_right(
        &self,
        diff: f32,
        col_idx: usize,
        widths: &mut [f32; COLS],
        resize_behavior: &[ResizeBehavior; COLS],
    ) -> f32 {
        let mut diff_remaining = diff;
        let mut curr_column = col_idx + 1;

        while diff_remaining > 0.0 && curr_column < COLS {
            let Some(min_size) = resize_behavior[curr_column - 1].min_size() else {
                curr_column += 1;
                continue;
            };

            let mut curr_width = widths[curr_column] - diff_remaining;

            diff_remaining = 0.0;
            if min_size > curr_width {
                diff_remaining += min_size - curr_width;
                curr_width = min_size;
            }
            widths[curr_column] = curr_width;
            curr_column += 1;
        }

        widths[col_idx] = widths[col_idx] + (diff - diff_remaining);
        return diff_remaining;
    }

    fn propagate_resize_diff_left(
        &mut self,
        diff: f32,
        mut curr_column: usize,
        widths: &mut [f32; COLS],
        resize_behavior: &[ResizeBehavior; COLS],
    ) -> f32 {
        let mut diff_remaining = diff;
        let col_idx = curr_column;
        while diff_remaining < 0.0 {
            let Some(min_size) = resize_behavior[curr_column].min_size() else {
                if curr_column == 0 {
                    break;
                }
                curr_column -= 1;
                continue;
            };

            let mut curr_width = widths[curr_column] + diff_remaining;

            diff_remaining = 0.0;
            if curr_width < min_size {
                diff_remaining = curr_width - min_size;
                curr_width = min_size
            }

            widths[curr_column] = curr_width;
            if curr_column == 0 {
                break;
            }
            curr_column -= 1;
        }
        widths[col_idx + 1] = widths[col_idx + 1] - (diff - diff_remaining);

        return diff_remaining;
    }
}

pub struct TableWidths<const COLS: usize> {
    initial: [DefiniteLength; COLS],
    current: Option<Entity<ColumnWidths<COLS>>>,
    resizable: [ResizeBehavior; COLS],
}

impl<const COLS: usize> TableWidths<COLS> {
    pub fn new(widths: [impl Into<DefiniteLength>; COLS]) -> Self {
        let widths = widths.map(Into::into);

        TableWidths {
            initial: widths,
            current: None,
            resizable: [ResizeBehavior::None; COLS],
        }
    }

    fn lengths(&self, cx: &App) -> [Length; COLS] {
        self.current
            .as_ref()
            .map(|entity| entity.read(cx).widths.map(Length::Definite))
            .unwrap_or(self.initial.map(Length::Definite))
    }
}

/// A table component
#[derive(RegisterComponent, IntoElement)]
pub struct Table<const COLS: usize = 3> {
    striped: bool,
    width: Option<Length>,
    headers: Option<[AnyElement; COLS]>,
    rows: TableContents<COLS>,
    interaction_state: Option<WeakEntity<TableInteractionState>>,
    col_widths: Option<TableWidths<COLS>>,
    map_row: Option<Rc<dyn Fn((usize, Stateful<Div>), &mut Window, &mut App) -> AnyElement>>,
    empty_table_callback: Option<Rc<dyn Fn(&mut Window, &mut App) -> AnyElement>>,
}

impl<const COLS: usize> Table<COLS> {
    /// number of headers provided.
    pub fn new() -> Self {
        Self {
            striped: false,
            width: None,
            headers: None,
            rows: TableContents::Vec(Vec::new()),
            interaction_state: None,
            map_row: None,
            empty_table_callback: None,
            col_widths: None,
        }
    }

    /// Enables uniform list rendering.
    /// The provided function will be passed directly to the `uniform_list` element.
    /// Therefore, if this method is called, any calls to [`Table::row`] before or after
    /// this method is called will be ignored.
    pub fn uniform_list(
        mut self,
        id: impl Into<ElementId>,
        row_count: usize,
        render_item_fn: impl Fn(Range<usize>, &mut Window, &mut App) -> Vec<[AnyElement; COLS]>
        + 'static,
    ) -> Self {
        self.rows = TableContents::UniformList(UniformListData {
            element_id: id.into(),
            row_count: row_count,
            render_item_fn: Box::new(render_item_fn),
        });
        self
    }

    /// Enables row striping.
    pub fn striped(mut self) -> Self {
        self.striped = true;
        self
    }

    /// Sets the width of the table.
    /// Will enable horizontal scrolling if [`Self::interactable`] is also called.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = Some(width.into());
        self
    }

    /// Enables interaction (primarily scrolling) with the table.
    ///
    /// Vertical scrolling will be enabled by default if the table is taller than its container.
    ///
    /// Horizontal scrolling will only be enabled if [`Self::width`] is also called, otherwise
    /// the list will always shrink the table columns to fit their contents I.e. If [`Self::uniform_list`]
    /// is used without a width and with [`Self::interactable`], the [`ListHorizontalSizingBehavior`] will
    /// be set to [`ListHorizontalSizingBehavior::FitList`].
    pub fn interactable(mut self, interaction_state: &Entity<TableInteractionState>) -> Self {
        self.interaction_state = Some(interaction_state.downgrade());
        self
    }

    pub fn header(mut self, headers: [impl IntoElement; COLS]) -> Self {
        self.headers = Some(headers.map(IntoElement::into_any_element));
        self
    }

    pub fn row(mut self, items: [impl IntoElement; COLS]) -> Self {
        if let Some(rows) = self.rows.rows_mut() {
            rows.push(items.map(IntoElement::into_any_element));
        }
        self
    }

    pub fn column_widths(mut self, widths: [impl Into<DefiniteLength>; COLS]) -> Self {
        if self.col_widths.is_none() {
            self.col_widths = Some(TableWidths::new(widths));
        }
        self
    }

    pub fn resizable_columns(
        mut self,
        resizable: [ResizeBehavior; COLS],
        column_widths: &Entity<ColumnWidths<COLS>>,
        cx: &mut App,
    ) -> Self {
        if let Some(table_widths) = self.col_widths.as_mut() {
            table_widths.resizable = resizable;
            let column_widths = table_widths
                .current
                .get_or_insert_with(|| column_widths.clone());

            column_widths.update(cx, |widths, _| {
                if !widths.initialized {
                    widths.initialized = true;
                    widths.widths = table_widths.initial;
                }
            })
        }
        self
    }

    pub fn map_row(
        mut self,
        callback: impl Fn((usize, Stateful<Div>), &mut Window, &mut App) -> AnyElement + 'static,
    ) -> Self {
        self.map_row = Some(Rc::new(callback));
        self
    }

    /// Provide a callback that is invoked when the table is rendered without any rows
    pub fn empty_table_callback(
        mut self,
        callback: impl Fn(&mut Window, &mut App) -> AnyElement + 'static,
    ) -> Self {
        self.empty_table_callback = Some(Rc::new(callback));
        self
    }
}

fn base_cell_style(width: Option<Length>) -> Div {
    div()
        .px_1p5()
        .when_some(width, |this, width| this.w(width))
        .when(width.is_none(), |this| this.flex_1())
        .justify_start()
        .whitespace_nowrap()
        .text_ellipsis()
        .overflow_hidden()
}

fn base_cell_style_text(width: Option<Length>, cx: &App) -> Div {
    base_cell_style(width).text_ui(cx)
}

pub fn render_row<const COLS: usize>(
    row_index: usize,
    items: [impl IntoElement; COLS],
    table_context: TableRenderContext<COLS>,
    window: &mut Window,
    cx: &mut App,
) -> AnyElement {
    let is_striped = table_context.striped;
    let is_last = row_index == table_context.total_row_count - 1;
    let bg = if row_index % 2 == 1 && is_striped {
        Some(cx.theme().colors().text.opacity(0.05))
    } else {
        None
    };
    let column_widths = table_context
        .column_widths
        .map_or([None; COLS], |widths| widths.map(Some));

    let mut row = h_flex()
        .h_full()
        .id(("table_row", row_index))
        .w_full()
        .justify_between()
        .when_some(bg, |row, bg| row.bg(bg))
        .when(!is_striped, |row| {
            row.border_b_1()
                .border_color(transparent_black())
                .when(!is_last, |row| row.border_color(cx.theme().colors().border))
        });

    row = row.children(
        items
            .map(IntoElement::into_any_element)
            .into_iter()
            .zip(column_widths)
            .map(|(cell, width)| base_cell_style_text(width, cx).px_1p5().py_1().child(cell)),
    );

    let row = if let Some(map_row) = table_context.map_row {
        map_row((row_index, row), window, cx)
    } else {
        row.into_any_element()
    };

    div().h_full().w_full().child(row).into_any_element()
}

pub fn render_header<const COLS: usize>(
    headers: [impl IntoElement; COLS],
    table_context: TableRenderContext<COLS>,
    cx: &mut App,
) -> impl IntoElement {
    let column_widths = table_context
        .column_widths
        .map_or([None; COLS], |widths| widths.map(Some));
    div()
        .flex()
        .flex_row()
        .items_center()
        .justify_between()
        .w_full()
        .p_2()
        .border_b_1()
        .border_color(cx.theme().colors().border)
        .children(
            headers
                .into_iter()
                .zip(column_widths)
                .map(|(h, width)| base_cell_style_text(width, cx).child(h)),
        )
}

#[derive(Clone)]
pub struct TableRenderContext<const COLS: usize> {
    pub striped: bool,
    pub total_row_count: usize,
    pub column_widths: Option<[Length; COLS]>,
    pub map_row: Option<Rc<dyn Fn((usize, Stateful<Div>), &mut Window, &mut App) -> AnyElement>>,
}

impl<const COLS: usize> TableRenderContext<COLS> {
    fn new(table: &Table<COLS>, cx: &App) -> Self {
        Self {
            striped: table.striped,
            total_row_count: table.rows.len(),
            column_widths: table.col_widths.as_ref().map(|widths| widths.lengths(cx)),
            map_row: table.map_row.clone(),
        }
    }
}

impl<const COLS: usize> RenderOnce for Table<COLS> {
    fn render(mut self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let table_context = TableRenderContext::new(&self, cx);
        let interaction_state = self.interaction_state.and_then(|state| state.upgrade());
        let current_widths = self
            .col_widths
            .as_ref()
            .and_then(|widths| Some((widths.current.as_ref()?, widths.resizable)))
            .map(|(curr, resize_behavior)| (curr.downgrade(), resize_behavior));

        let scroll_track_size = px(16.);
        let h_scroll_offset = if interaction_state
            .as_ref()
            .is_some_and(|state| state.read(cx).vertical_scrollbar.show_scrollbar)
        {
            // magic number
            px(3.)
        } else {
            px(0.)
        };

        let width = self.width;
        let no_rows_rendered = self.rows.is_empty();

        let table = div()
            .when_some(width, |this, width| this.w(width))
            .h_full()
            .v_flex()
            .when_some(self.headers.take(), |this, headers| {
                this.child(render_header(headers, table_context.clone(), cx))
            })
            .when_some(current_widths, {
                |this, (widths, resize_behavior)| {
                    this.on_drag_move::<DraggedColumn>({
                        let widths = widths.clone();
                        move |e, window, cx| {
                            widths
                                .update(cx, |widths, cx| {
                                    widths.on_drag_move(e, &resize_behavior, window, cx);
                                })
                                .ok();
                        }
                    })
                    .on_children_prepainted(move |bounds, _, cx| {
                        widths
                            .update(cx, |widths, _| {
                                // This works because all children x axis bounds are the same
                                widths.cached_bounds_width = bounds[0].right() - bounds[0].left();
                            })
                            .ok();
                    })
                }
            })
            .on_drop::<DraggedColumn>(|_, _, _| {
                // Finish the resize operation
            })
            .child(
                div()
                    .flex_grow()
                    .w_full()
                    .relative()
                    .overflow_hidden()
                    .map(|parent| match self.rows {
                        TableContents::Vec(items) => {
                            parent.children(items.into_iter().enumerate().map(|(index, row)| {
                                render_row(index, row, table_context.clone(), window, cx)
                            }))
                        }
                        TableContents::UniformList(uniform_list_data) => parent.child(
                            uniform_list(
                                uniform_list_data.element_id,
                                uniform_list_data.row_count,
                                {
                                    let render_item_fn = uniform_list_data.render_item_fn;
                                    move |range: Range<usize>, window, cx| {
                                        let elements = render_item_fn(range.clone(), window, cx);
                                        elements
                                            .into_iter()
                                            .zip(range)
                                            .map(|(row, row_index)| {
                                                render_row(
                                                    row_index,
                                                    row,
                                                    table_context.clone(),
                                                    window,
                                                    cx,
                                                )
                                            })
                                            .collect()
                                    }
                                },
                            )
                            .size_full()
                            .flex_grow()
                            .with_sizing_behavior(ListSizingBehavior::Auto)
                            .with_horizontal_sizing_behavior(if width.is_some() {
                                ListHorizontalSizingBehavior::Unconstrained
                            } else {
                                ListHorizontalSizingBehavior::FitList
                            })
                            .when_some(
                                interaction_state.as_ref(),
                                |this, state| {
                                    this.track_scroll(
                                        state.read_with(cx, |s, _| s.scroll_handle.clone()),
                                    )
                                },
                            ),
                        ),
                    })
                    .when_some(
                        self.col_widths.as_ref().zip(interaction_state.as_ref()),
                        |parent, (table_widths, state)| {
                            parent.child(state.update(cx, |state, cx| {
                                let resizable_columns = table_widths.resizable;
                                let column_widths = table_widths.lengths(cx);
                                let columns = table_widths.current.clone();
                                let initial_sizes = table_widths.initial;
                                state.render_resize_handles(
                                    &column_widths,
                                    &resizable_columns,
                                    initial_sizes,
                                    columns,
                                    window,
                                    cx,
                                )
                            }))
                        },
                    )
                    .when_some(interaction_state.as_ref(), |this, interaction_state| {
                        this.map(|this| {
                            TableInteractionState::render_vertical_scrollbar_track(
                                interaction_state,
                                this,
                                scroll_track_size,
                                cx,
                            )
                        })
                        .map(|this| {
                            TableInteractionState::render_vertical_scrollbar(
                                interaction_state,
                                this,
                                cx,
                            )
                        })
                    }),
            )
            .when_some(
                no_rows_rendered
                    .then_some(self.empty_table_callback)
                    .flatten(),
                |this, callback| {
                    this.child(
                        h_flex()
                            .size_full()
                            .p_3()
                            .items_start()
                            .justify_center()
                            .child(callback(window, cx)),
                    )
                },
            )
            .when_some(
                width.and(interaction_state.as_ref()),
                |this, interaction_state| {
                    this.map(|this| {
                        TableInteractionState::render_horizontal_scrollbar_track(
                            interaction_state,
                            this,
                            scroll_track_size,
                            cx,
                        )
                    })
                    .map(|this| {
                        TableInteractionState::render_horizontal_scrollbar(
                            interaction_state,
                            this,
                            h_scroll_offset,
                            cx,
                        )
                    })
                },
            );

        if let Some(interaction_state) = interaction_state.as_ref() {
            table
                .track_focus(&interaction_state.read(cx).focus_handle)
                .id(("table", interaction_state.entity_id()))
                .on_hover({
                    let interaction_state = interaction_state.downgrade();
                    move |hovered, window, cx| {
                        interaction_state
                            .update(cx, |interaction_state, cx| {
                                if *hovered {
                                    interaction_state.horizontal_scrollbar.show(cx);
                                    interaction_state.vertical_scrollbar.show(cx);
                                    cx.notify();
                                } else if !interaction_state
                                    .focus_handle
                                    .contains_focused(window, cx)
                                {
                                    interaction_state.hide_scrollbars(window, cx);
                                }
                            })
                            .ok();
                    }
                })
                .into_any_element()
        } else {
            table.into_any_element()
        }
    }
}

// computed state related to how to render scrollbars
// one per axis
// on render we just read this off the keymap editor
// we update it when
// - settings change
// - on focus in, on focus out, on hover, etc.
#[derive(Debug)]
pub struct ScrollbarProperties {
    axis: Axis,
    show_scrollbar: bool,
    show_track: bool,
    auto_hide: bool,
    hide_task: Option<Task<()>>,
    state: ScrollbarState,
}

impl ScrollbarProperties {
    // Shows the scrollbar and cancels any pending hide task
    fn show(&mut self, cx: &mut Context<TableInteractionState>) {
        if !self.auto_hide {
            return;
        }
        self.show_scrollbar = true;
        self.hide_task.take();
        cx.notify();
    }

    fn hide(&mut self, window: &mut Window, cx: &mut Context<TableInteractionState>) {
        const SCROLLBAR_SHOW_INTERVAL: Duration = Duration::from_secs(1);

        if !self.auto_hide {
            return;
        }

        let axis = self.axis;
        self.hide_task = Some(cx.spawn_in(window, async move |keymap_editor, cx| {
            cx.background_executor()
                .timer(SCROLLBAR_SHOW_INTERVAL)
                .await;

            if let Some(keymap_editor) = keymap_editor.upgrade() {
                keymap_editor
                    .update(cx, |keymap_editor, cx| {
                        match axis {
                            Axis::Vertical => {
                                keymap_editor.vertical_scrollbar.show_scrollbar = false
                            }
                            Axis::Horizontal => {
                                keymap_editor.horizontal_scrollbar.show_scrollbar = false
                            }
                        }
                        cx.notify();
                    })
                    .ok();
            }
        }));
    }
}

impl Component for Table<3> {
    fn scope() -> ComponentScope {
        ComponentScope::Layout
    }

    fn description() -> Option<&'static str> {
        Some("A table component for displaying data in rows and columns with optional styling.")
    }

    fn preview(_window: &mut Window, _cx: &mut App) -> Option<AnyElement> {
        Some(
            v_flex()
                .gap_6()
                .children(vec![
                    example_group_with_title(
                        "Basic Tables",
                        vec![
                            single_example(
                                "Simple Table",
                                Table::new()
                                    .width(px(400.))
                                    .header(["Name", "Age", "City"])
                                    .row(["Alice", "28", "New York"])
                                    .row(["Bob", "32", "San Francisco"])
                                    .row(["Charlie", "25", "London"])
                                    .into_any_element(),
                            ),
                            single_example(
                                "Two Column Table",
                                Table::new()
                                    .header(["Category", "Value"])
                                    .width(px(300.))
                                    .row(["Revenue", "$100,000"])
                                    .row(["Expenses", "$75,000"])
                                    .row(["Profit", "$25,000"])
                                    .into_any_element(),
                            ),
                        ],
                    ),
                    example_group_with_title(
                        "Styled Tables",
                        vec![
                            single_example(
                                "Default",
                                Table::new()
                                    .width(px(400.))
                                    .header(["Product", "Price", "Stock"])
                                    .row(["Laptop", "$999", "In Stock"])
                                    .row(["Phone", "$599", "Low Stock"])
                                    .row(["Tablet", "$399", "Out of Stock"])
                                    .into_any_element(),
                            ),
                            single_example(
                                "Striped",
                                Table::new()
                                    .width(px(400.))
                                    .striped()
                                    .header(["Product", "Price", "Stock"])
                                    .row(["Laptop", "$999", "In Stock"])
                                    .row(["Phone", "$599", "Low Stock"])
                                    .row(["Tablet", "$399", "Out of Stock"])
                                    .row(["Headphones", "$199", "In Stock"])
                                    .into_any_element(),
                            ),
                        ],
                    ),
                    example_group_with_title(
                        "Mixed Content Table",
                        vec![single_example(
                            "Table with Elements",
                            Table::new()
                                .width(px(840.))
                                .header(["Status", "Name", "Priority", "Deadline", "Action"])
                                .row([
                                    Indicator::dot().color(Color::Success).into_any_element(),
                                    "Project A".into_any_element(),
                                    "High".into_any_element(),
                                    "2023-12-31".into_any_element(),
                                    Button::new("view_a", "View")
                                        .style(ButtonStyle::Filled)
                                        .full_width()
                                        .into_any_element(),
                                ])
                                .row([
                                    Indicator::dot().color(Color::Warning).into_any_element(),
                                    "Project B".into_any_element(),
                                    "Medium".into_any_element(),
                                    "2024-03-15".into_any_element(),
                                    Button::new("view_b", "View")
                                        .style(ButtonStyle::Filled)
                                        .full_width()
                                        .into_any_element(),
                                ])
                                .row([
                                    Indicator::dot().color(Color::Error).into_any_element(),
                                    "Project C".into_any_element(),
                                    "Low".into_any_element(),
                                    "2024-06-30".into_any_element(),
                                    Button::new("view_c", "View")
                                        .style(ButtonStyle::Filled)
                                        .full_width()
                                        .into_any_element(),
                                ])
                                .into_any_element(),
                        )],
                    ),
                ])
                .into_any_element(),
        )
    }
}
