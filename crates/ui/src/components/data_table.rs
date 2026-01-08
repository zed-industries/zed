use std::{ops::Range, rc::Rc};

use gpui::{
    AbsoluteLength, AppContext, Context, DefiniteLength, DragMoveEvent, Entity, EntityId,
    FocusHandle, Length, ListHorizontalSizingBehavior, ListSizingBehavior, ListState, Point,
    Stateful, UniformListScrollHandle, WeakEntity, list, transparent_black, uniform_list,
};

use crate::{
    ActiveTheme as _, AnyElement, App, Button, ButtonCommon as _, ButtonStyle, Color, Component,
    ComponentScope, Div, ElementId, FixedWidth as _, FluentBuilder as _, Indicator,
    InteractiveElement, IntoElement, ParentElement, Pixels, RegisterComponent, RenderOnce,
    ScrollAxes, ScrollableHandle, Scrollbars, SharedString, StatefulInteractiveElement, Styled,
    StyledExt as _, StyledTypography, Window, WithScrollbar, div, example_group_with_title, h_flex,
    px, single_example, v_flex,
};
use itertools::intersperse_with;

const RESIZE_COLUMN_WIDTH: f32 = 8.0;

#[derive(Debug)]
struct DraggedColumn(usize);

struct UniformListData<const COLS: usize> {
    render_list_of_rows_fn:
        Box<dyn Fn(Range<usize>, &mut Window, &mut App) -> Vec<[AnyElement; COLS]>>,
    element_id: ElementId,
    row_count: usize,
}

struct VariableRowHeightListData<const COLS: usize> {
    /// Unlike UniformList, this closure renders only single row, allowing each one to have its own height
    render_row_fn: Box<dyn Fn(usize, &mut Window, &mut App) -> [AnyElement; COLS]>,
    list_state: ListState,
    row_count: usize,
}

enum TableContents<const COLS: usize> {
    Vec(Vec<[AnyElement; COLS]>),
    UniformList(UniformListData<COLS>),
    VariableRowHeightList(VariableRowHeightListData<COLS>),
}

impl<const COLS: usize> TableContents<COLS> {
    fn rows_mut(&mut self) -> Option<&mut Vec<[AnyElement; COLS]>> {
        match self {
            TableContents::Vec(rows) => Some(rows),
            TableContents::UniformList(_) => None,
            TableContents::VariableRowHeightList(_) => None,
        }
    }

    fn len(&self) -> usize {
        match self {
            TableContents::Vec(rows) => rows.len(),
            TableContents::UniformList(data) => data.row_count,
            TableContents::VariableRowHeightList(data) => data.row_count,
        }
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

pub struct TableInteractionState {
    pub focus_handle: FocusHandle,
    pub scroll_handle: UniformListScrollHandle,
    pub custom_scrollbar: Option<Scrollbars>,
}

impl TableInteractionState {
    pub fn new(cx: &mut App) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
            scroll_handle: UniformListScrollHandle::new(),
            custom_scrollbar: None,
        }
    }

    pub fn with_custom_scrollbar(mut self, custom_scrollbar: Scrollbars) -> Self {
        self.custom_scrollbar = Some(custom_scrollbar);
        self
    }

    pub fn scroll_offset(&self) -> Point<Pixels> {
        self.scroll_handle.offset()
    }

    pub fn set_scroll_offset(&self, offset: Point<Pixels>) {
        self.scroll_handle.set_offset(offset);
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
        resizable_columns: &[TableResizeBehavior; COLS],
        initial_sizes: [DefiniteLength; COLS],
        columns: Option<Entity<TableColumnWidths<COLS>>>,
        window: &mut Window,
        cx: &mut App,
    ) -> AnyElement {
        let spacers = column_widths
            .iter()
            .map(|width| base_cell_style(Some(*width)).into_any_element());

        let mut column_ix = 0;
        let resizable_columns_slice = *resizable_columns;
        let mut resizable_columns = resizable_columns.iter();

        let dividers = intersperse_with(spacers, || {
            window.with_id(column_ix, |window| {
                let mut resize_divider = div()
                    // This is required because this is evaluated at a different time than the use_state call above
                    .id(column_ix)
                    .relative()
                    .top_0()
                    .w_px()
                    .h_full()
                    .bg(cx.theme().colors().border.opacity(0.8));

                let mut resize_handle = div()
                    .id("column-resize-handle")
                    .absolute()
                    .left_neg_0p5()
                    .w(px(RESIZE_COLUMN_WIDTH))
                    .h_full();

                if resizable_columns
                    .next()
                    .is_some_and(TableResizeBehavior::is_resizable)
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
                                if event.click_count() >= 2 {
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

        h_flex()
            .id("resize-handles")
            .absolute()
            .inset_0()
            .w_full()
            .children(dividers)
            .into_any_element()
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TableResizeBehavior {
    None,
    Resizable,
    MinSize(f32),
}

impl TableResizeBehavior {
    pub fn is_resizable(&self) -> bool {
        *self != TableResizeBehavior::None
    }

    pub fn min_size(&self) -> Option<f32> {
        match self {
            TableResizeBehavior::None => None,
            TableResizeBehavior::Resizable => Some(0.05),
            TableResizeBehavior::MinSize(min_size) => Some(*min_size),
        }
    }
}

pub struct TableColumnWidths<const COLS: usize> {
    widths: [DefiniteLength; COLS],
    visible_widths: [DefiniteLength; COLS],
    cached_bounds_width: Pixels,
    initialized: bool,
}

impl<const COLS: usize> TableColumnWidths<COLS> {
    pub fn new(_: &mut App) -> Self {
        Self {
            widths: [DefiniteLength::default(); COLS],
            visible_widths: [DefiniteLength::default(); COLS],
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
        resize_behavior: &[TableResizeBehavior; COLS],
        window: &mut Window,
    ) {
        let bounds_width = self.cached_bounds_width;
        let rem_size = window.rem_size();
        let initial_sizes =
            initial_sizes.map(|length| Self::get_fraction(&length, bounds_width, rem_size));
        let widths = self
            .widths
            .map(|length| Self::get_fraction(&length, bounds_width, rem_size));

        let updated_widths = Self::reset_to_initial_size(
            double_click_position,
            widths,
            initial_sizes,
            resize_behavior,
        );
        self.widths = updated_widths.map(DefiniteLength::Fraction);
        self.visible_widths = self.widths;
    }

    fn reset_to_initial_size(
        col_idx: usize,
        mut widths: [f32; COLS],
        initial_sizes: [f32; COLS],
        resize_behavior: &[TableResizeBehavior; COLS],
    ) -> [f32; COLS] {
        // RESET:
        // Part 1:
        // Figure out if we should shrink/grow the selected column
        // Get diff which represents the change in column we want to make initial size delta curr_size = diff
        //
        // Part 2: We need to decide which side column we should move and where
        //
        // If we want to grow our column we should check the left/right columns diff to see what side
        // has a greater delta than their initial size. Likewise, if we shrink our column we should check
        // the left/right column diffs to see what side has the smallest delta.
        //
        // Part 3: resize
        //
        // col_idx represents the column handle to the right of an active column
        //
        // If growing and right has the greater delta {
        //    shift col_idx to the right
        // } else if growing and left has the greater delta {
        //  shift col_idx - 1 to the left
        // } else if shrinking and the right has the greater delta {
        //  shift
        // } {
        //
        // }
        // }
        //
        // if we need to shrink, then if the right
        //

        // DRAGGING
        // we get diff which represents the change in the _drag handle_ position
        // -diff => dragging left ->
        //      grow the column to the right of the handle as much as we can shrink columns to the left of the handle
        // +diff => dragging right -> growing handles column
        //      grow the column to the left of the handle as much as we can shrink columns to the right of the handle
        //

        let diff = initial_sizes[col_idx] - widths[col_idx];

        let left_diff =
            initial_sizes[..col_idx].iter().sum::<f32>() - widths[..col_idx].iter().sum::<f32>();
        let right_diff = initial_sizes[col_idx + 1..].iter().sum::<f32>()
            - widths[col_idx + 1..].iter().sum::<f32>();

        let go_left_first = if diff < 0.0 {
            left_diff > right_diff
        } else {
            left_diff < right_diff
        };

        if !go_left_first {
            let diff_remaining =
                Self::propagate_resize_diff(diff, col_idx, &mut widths, resize_behavior, 1);

            if diff_remaining != 0.0 && col_idx > 0 {
                Self::propagate_resize_diff(
                    diff_remaining,
                    col_idx,
                    &mut widths,
                    resize_behavior,
                    -1,
                );
            }
        } else {
            let diff_remaining =
                Self::propagate_resize_diff(diff, col_idx, &mut widths, resize_behavior, -1);

            if diff_remaining != 0.0 {
                Self::propagate_resize_diff(
                    diff_remaining,
                    col_idx,
                    &mut widths,
                    resize_behavior,
                    1,
                );
            }
        }

        widths
    }

    fn on_drag_move(
        &mut self,
        drag_event: &DragMoveEvent<DraggedColumn>,
        resize_behavior: &[TableResizeBehavior; COLS],
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let drag_position = drag_event.event.position;
        let bounds = drag_event.bounds;

        let mut col_position = 0.0;
        let rem_size = window.rem_size();
        let bounds_width = bounds.right() - bounds.left();
        let col_idx = drag_event.drag(cx).0;

        let column_handle_width = Self::get_fraction(
            &DefiniteLength::Absolute(AbsoluteLength::Pixels(px(RESIZE_COLUMN_WIDTH))),
            bounds_width,
            rem_size,
        );

        let mut widths = self
            .widths
            .map(|length| Self::get_fraction(&length, bounds_width, rem_size));

        for length in widths[0..=col_idx].iter() {
            col_position += length + column_handle_width;
        }

        let mut total_length_ratio = col_position;
        for length in widths[col_idx + 1..].iter() {
            total_length_ratio += length;
        }
        total_length_ratio += (COLS - 1 - col_idx) as f32 * column_handle_width;

        let drag_fraction = (drag_position.x - bounds.left()) / bounds_width;
        let drag_fraction = drag_fraction * total_length_ratio;
        let diff = drag_fraction - col_position - column_handle_width / 2.0;

        Self::drag_column_handle(diff, col_idx, &mut widths, resize_behavior);

        self.visible_widths = widths.map(DefiniteLength::Fraction);
    }

    fn drag_column_handle(
        diff: f32,
        col_idx: usize,
        widths: &mut [f32; COLS],
        resize_behavior: &[TableResizeBehavior; COLS],
    ) {
        // if diff > 0.0 then go right
        if diff > 0.0 {
            Self::propagate_resize_diff(diff, col_idx, widths, resize_behavior, 1);
        } else {
            Self::propagate_resize_diff(-diff, col_idx + 1, widths, resize_behavior, -1);
        }
    }

    fn propagate_resize_diff(
        diff: f32,
        col_idx: usize,
        widths: &mut [f32; COLS],
        resize_behavior: &[TableResizeBehavior; COLS],
        direction: i8,
    ) -> f32 {
        let mut diff_remaining = diff;
        if resize_behavior[col_idx].min_size().is_none() {
            return diff;
        }

        let step_right;
        let step_left;
        if direction < 0 {
            step_right = 0;
            step_left = 1;
        } else {
            step_right = 1;
            step_left = 0;
        }
        if col_idx == 0 && direction < 0 {
            return diff;
        }
        let mut curr_column = col_idx + step_right - step_left;

        while diff_remaining != 0.0 && curr_column < COLS {
            let Some(min_size) = resize_behavior[curr_column].min_size() else {
                if curr_column == 0 {
                    break;
                }
                curr_column -= step_left;
                curr_column += step_right;
                continue;
            };

            let curr_width = widths[curr_column] - diff_remaining;
            widths[curr_column] = curr_width;

            if min_size > curr_width {
                diff_remaining = min_size - curr_width;
                widths[curr_column] = min_size;
            } else {
                diff_remaining = 0.0;
                break;
            }
            if curr_column == 0 {
                break;
            }
            curr_column -= step_left;
            curr_column += step_right;
        }
        widths[col_idx] = widths[col_idx] + (diff - diff_remaining);

        diff_remaining
    }
}

pub struct TableWidths<const COLS: usize> {
    initial: [DefiniteLength; COLS],
    current: Option<Entity<TableColumnWidths<COLS>>>,
    resizable: [TableResizeBehavior; COLS],
}

impl<const COLS: usize> TableWidths<COLS> {
    pub fn new(widths: [impl Into<DefiniteLength>; COLS]) -> Self {
        let widths = widths.map(Into::into);

        TableWidths {
            initial: widths,
            current: None,
            resizable: [TableResizeBehavior::None; COLS],
        }
    }

    fn lengths(&self, cx: &App) -> [Length; COLS] {
        self.current
            .as_ref()
            .map(|entity| entity.read(cx).visible_widths.map(Length::Definite))
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
    use_ui_font: bool,
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
            use_ui_font: true,
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
            row_count,
            render_list_of_rows_fn: Box::new(render_item_fn),
        });
        self
    }

    /// Enables rendering of tables with variable row heights, allowing each row to have its own height.
    ///
    /// This mode is useful for displaying content such as CSV data or multiline cells, where rows may not have uniform heights.
    /// It is generally slower than [`Table::uniform_list`] due to the need to measure each row individually, but it provides correct layout for non-uniform or multiline content.
    ///
    /// # Parameters
    /// - `row_count`: The total number of rows in the table.
    /// - `list_state`: The [`ListState`] used for managing scroll position and virtualization. This must be initialized and managed by the caller, and should be kept in sync with the number of rows.
    /// - `render_row_fn`: A closure that renders a single row, given the row index, a mutable reference to [`Window`], and a mutable reference to [`App`]. It should return an array of [`AnyElement`]s, one for each column.
    pub fn variable_row_height_list(
        mut self,
        row_count: usize,
        list_state: ListState,
        render_row_fn: impl Fn(usize, &mut Window, &mut App) -> [AnyElement; COLS] + 'static,
    ) -> Self {
        self.rows = TableContents::VariableRowHeightList(VariableRowHeightListData {
            render_row_fn: Box::new(render_row_fn),
            list_state,
            row_count,
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
        resizable: [TableResizeBehavior; COLS],
        column_widths: &Entity<TableColumnWidths<COLS>>,
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
                    widths.visible_widths = widths.widths;
                }
            })
        }
        self
    }

    pub fn no_ui_font(mut self) -> Self {
        self.use_ui_font = false;
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
        .whitespace_nowrap()
        .text_ellipsis()
        .overflow_hidden()
}

fn base_cell_style_text(width: Option<Length>, use_ui_font: bool, cx: &App) -> Div {
    base_cell_style(width).when(use_ui_font, |el| el.text_ui(cx))
}

pub fn render_table_row<const COLS: usize>(
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

    let mut row = div()
        // NOTE: `h_flex()` sneakily applies `items_center()` which is not default behavior for div element.
        // Applying `.flex().flex_row()` manually to overcome that
        .flex()
        .flex_row()
        .id(("table_row", row_index))
        .size_full()
        .when_some(bg, |row, bg| row.bg(bg))
        .hover(|s| s.bg(cx.theme().colors().element_hover.opacity(0.6)))
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
            .map(|(cell, width)| {
                base_cell_style_text(width, table_context.use_ui_font, cx)
                    .px_1()
                    .py_0p5()
                    .child(cell)
            }),
    );

    let row = if let Some(map_row) = table_context.map_row {
        map_row((row_index, row), window, cx)
    } else {
        row.into_any_element()
    };

    div().size_full().child(row).into_any_element()
}

pub fn render_table_header<const COLS: usize>(
    headers: [impl IntoElement; COLS],
    table_context: TableRenderContext<COLS>,
    columns_widths: Option<(
        WeakEntity<TableColumnWidths<COLS>>,
        [TableResizeBehavior; COLS],
        [DefiniteLength; COLS],
    )>,
    entity_id: Option<EntityId>,
    cx: &mut App,
) -> impl IntoElement {
    let column_widths = table_context
        .column_widths
        .map_or([None; COLS], |widths| widths.map(Some));

    let element_id = entity_id
        .map(|entity| entity.to_string())
        .unwrap_or_default();

    let shared_element_id: SharedString = format!("table-{}", element_id).into();

    div()
        .flex()
        .flex_row()
        .items_center()
        .justify_between()
        .w_full()
        .p_2()
        .border_b_1()
        .border_color(cx.theme().colors().border)
        .children(headers.into_iter().enumerate().zip(column_widths).map(
            |((header_idx, h), width)| {
                base_cell_style_text(width, table_context.use_ui_font, cx)
                    .child(h)
                    .id(ElementId::NamedInteger(
                        shared_element_id.clone(),
                        header_idx as u64,
                    ))
                    .when_some(
                        columns_widths.as_ref().cloned(),
                        |this, (column_widths, resizables, initial_sizes)| {
                            if resizables[header_idx].is_resizable() {
                                this.on_click(move |event, window, cx| {
                                    if event.click_count() > 1 {
                                        column_widths
                                            .update(cx, |column, _| {
                                                column.on_double_click(
                                                    header_idx,
                                                    &initial_sizes,
                                                    &resizables,
                                                    window,
                                                );
                                            })
                                            .ok();
                                    }
                                })
                            } else {
                                this
                            }
                        },
                    )
            },
        ))
}

#[derive(Clone)]
pub struct TableRenderContext<const COLS: usize> {
    pub striped: bool,
    pub total_row_count: usize,
    pub column_widths: Option<[Length; COLS]>,
    pub map_row: Option<Rc<dyn Fn((usize, Stateful<Div>), &mut Window, &mut App) -> AnyElement>>,
    pub use_ui_font: bool,
}

impl<const COLS: usize> TableRenderContext<COLS> {
    fn new(table: &Table<COLS>, cx: &App) -> Self {
        Self {
            striped: table.striped,
            total_row_count: table.rows.len(),
            column_widths: table.col_widths.as_ref().map(|widths| widths.lengths(cx)),
            map_row: table.map_row.clone(),
            use_ui_font: table.use_ui_font,
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

        let current_widths_with_initial_sizes = self
            .col_widths
            .as_ref()
            .and_then(|widths| Some((widths.current.as_ref()?, widths.resizable, widths.initial)))
            .map(|(curr, resize_behavior, initial)| (curr.downgrade(), resize_behavior, initial));

        let width = self.width;
        let no_rows_rendered = self.rows.is_empty();

        let table = div()
            .when_some(width, |this, width| this.w(width))
            .h_full()
            .v_flex()
            .when_some(self.headers.take(), |this, headers| {
                this.child(render_table_header(
                    headers,
                    table_context.clone(),
                    current_widths_with_initial_sizes,
                    interaction_state.as_ref().map(Entity::entity_id),
                    cx,
                ))
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
                    .on_children_prepainted({
                        let widths = widths.clone();
                        move |bounds, _, cx| {
                            widths
                                .update(cx, |widths, _| {
                                    // This works because all children x axis bounds are the same
                                    widths.cached_bounds_width =
                                        bounds[0].right() - bounds[0].left();
                                })
                                .ok();
                        }
                    })
                    .on_drop::<DraggedColumn>(move |_, _, cx| {
                        widths
                            .update(cx, |widths, _| {
                                widths.widths = widths.visible_widths;
                            })
                            .ok();
                        // Finish the resize operation
                    })
                }
            })
            .child({
                let content = div()
                    .flex_grow()
                    .w_full()
                    .relative()
                    .overflow_hidden()
                    .map(|parent| match self.rows {
                        TableContents::Vec(items) => {
                            parent.children(items.into_iter().enumerate().map(|(index, row)| {
                                div().child(render_table_row(
                                    index,
                                    row,
                                    table_context.clone(),
                                    window,
                                    cx,
                                ))
                            }))
                        }
                        TableContents::UniformList(uniform_list_data) => parent.child(
                            uniform_list(
                                uniform_list_data.element_id,
                                uniform_list_data.row_count,
                                {
                                    let render_item_fn = uniform_list_data.render_list_of_rows_fn;
                                    move |range: Range<usize>, window, cx| {
                                        let elements = render_item_fn(range.clone(), window, cx);
                                        elements
                                            .into_iter()
                                            .zip(range)
                                            .map(|(row, row_index)| {
                                                render_table_row(
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
                                        &state.read_with(cx, |s, _| s.scroll_handle.clone()),
                                    )
                                },
                            ),
                        ),
                        TableContents::VariableRowHeightList(variable_list_data) => parent.child(
                            list(variable_list_data.list_state.clone(), {
                                let render_item_fn = variable_list_data.render_row_fn;
                                move |row_index: usize, window: &mut Window, cx: &mut App| {
                                    let row = render_item_fn(row_index, window, cx);
                                    render_table_row(
                                        row_index,
                                        row,
                                        table_context.clone(),
                                        window,
                                        cx,
                                    )
                                }
                            })
                            .size_full()
                            .flex_grow()
                            .with_sizing_behavior(ListSizingBehavior::Auto),
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
                    );

                if let Some(state) = interaction_state.as_ref() {
                    let scrollbars = state
                        .read(cx)
                        .custom_scrollbar
                        .clone()
                        .unwrap_or_else(|| Scrollbars::new(ScrollAxes::Both));
                    content
                        .custom_scrollbars(
                            scrollbars.tracked_scroll_handle(&state.read(cx).scroll_handle),
                            window,
                            cx,
                        )
                        .into_any_element()
                } else {
                    content.into_any_element()
                }
            })
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
            );

        if let Some(interaction_state) = interaction_state.as_ref() {
            table
                .track_focus(&interaction_state.read(cx).focus_handle)
                .id(("table", interaction_state.entity_id()))
                .into_any_element()
        } else {
            table.into_any_element()
        }
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

#[cfg(test)]
mod test {
    use super::*;

    fn is_almost_eq(a: &[f32], b: &[f32]) -> bool {
        a.len() == b.len() && a.iter().zip(b).all(|(x, y)| (x - y).abs() < 1e-6)
    }

    fn cols_to_str<const COLS: usize>(cols: &[f32; COLS], total_size: f32) -> String {
        cols.map(|f| "*".repeat(f32::round(f * total_size) as usize))
            .join("|")
    }

    fn parse_resize_behavior<const COLS: usize>(
        input: &str,
        total_size: f32,
    ) -> [TableResizeBehavior; COLS] {
        let mut resize_behavior = [TableResizeBehavior::None; COLS];
        let mut max_index = 0;
        for (index, col) in input.split('|').enumerate() {
            if col.starts_with('X') || col.is_empty() {
                resize_behavior[index] = TableResizeBehavior::None;
            } else if col.starts_with('*') {
                resize_behavior[index] =
                    TableResizeBehavior::MinSize(col.len() as f32 / total_size);
            } else {
                panic!("invalid test input: unrecognized resize behavior: {}", col);
            }
            max_index = index;
        }

        if max_index + 1 != COLS {
            panic!("invalid test input: too many columns");
        }
        resize_behavior
    }

    mod reset_column_size {
        use super::*;

        fn parse<const COLS: usize>(input: &str) -> ([f32; COLS], f32, Option<usize>) {
            let mut widths = [f32::NAN; COLS];
            let mut column_index = None;
            for (index, col) in input.split('|').enumerate() {
                widths[index] = col.len() as f32;
                if col.starts_with('X') {
                    column_index = Some(index);
                }
            }

            for w in widths {
                assert!(w.is_finite(), "incorrect number of columns");
            }
            let total = widths.iter().sum::<f32>();
            for width in &mut widths {
                *width /= total;
            }
            (widths, total, column_index)
        }

        #[track_caller]
        fn check_reset_size<const COLS: usize>(
            initial_sizes: &str,
            widths: &str,
            expected: &str,
            resize_behavior: &str,
        ) {
            let (initial_sizes, total_1, None) = parse::<COLS>(initial_sizes) else {
                panic!("invalid test input: initial sizes should not be marked");
            };
            let (widths, total_2, Some(column_index)) = parse::<COLS>(widths) else {
                panic!("invalid test input: widths should be marked");
            };
            assert_eq!(
                total_1, total_2,
                "invalid test input: total width not the same {total_1}, {total_2}"
            );
            let (expected, total_3, None) = parse::<COLS>(expected) else {
                panic!("invalid test input: expected should not be marked: {expected:?}");
            };
            assert_eq!(
                total_2, total_3,
                "invalid test input: total width not the same"
            );
            let resize_behavior = parse_resize_behavior::<COLS>(resize_behavior, total_1);
            let result = TableColumnWidths::reset_to_initial_size(
                column_index,
                widths,
                initial_sizes,
                &resize_behavior,
            );
            let is_eq = is_almost_eq(&result, &expected);
            if !is_eq {
                let result_str = cols_to_str(&result, total_1);
                let expected_str = cols_to_str(&expected, total_1);
                panic!(
                    "resize failed\ncomputed: {result_str}\nexpected: {expected_str}\n\ncomputed values: {result:?}\nexpected values: {expected:?}\n:minimum widths: {resize_behavior:?}"
                );
            }
        }

        macro_rules! check_reset_size {
            (columns: $cols:expr, starting: $initial:expr, snapshot: $current:expr, expected: $expected:expr, resizing: $resizing:expr $(,)?) => {
                check_reset_size::<$cols>($initial, $current, $expected, $resizing);
            };
            ($name:ident, columns: $cols:expr, starting: $initial:expr, snapshot: $current:expr, expected: $expected:expr, minimums: $resizing:expr $(,)?) => {
                #[test]
                fn $name() {
                    check_reset_size::<$cols>($initial, $current, $expected, $resizing);
                }
            };
        }

        check_reset_size!(
            basic_right,
            columns: 5,
            starting: "**|**|**|**|**",
            snapshot: "**|**|X|***|**",
            expected: "**|**|**|**|**",
            minimums: "X|*|*|*|*",
        );

        check_reset_size!(
            basic_left,
            columns: 5,
            starting: "**|**|**|**|**",
            snapshot: "**|**|***|X|**",
            expected: "**|**|**|**|**",
            minimums: "X|*|*|*|**",
        );

        check_reset_size!(
            squashed_left_reset_col2,
            columns: 6,
            starting: "*|***|**|**|****|*",
            snapshot: "*|*|X|*|*|********",
            expected: "*|*|**|*|*|*******",
            minimums: "X|*|*|*|*|*",
        );

        check_reset_size!(
            grow_cascading_right,
            columns: 6,
            starting: "*|***|****|**|***|*",
            snapshot: "*|***|X|**|**|*****",
            expected: "*|***|****|*|*|****",
            minimums: "X|*|*|*|*|*",
        );

        check_reset_size!(
           squashed_right_reset_col4,
           columns: 6,
           starting: "*|***|**|**|****|*",
           snapshot: "*|********|*|*|X|*",
           expected: "*|*****|*|*|****|*",
           minimums: "X|*|*|*|*|*",
        );

        check_reset_size!(
            reset_col6_right,
            columns: 6,
            starting: "*|***|**|***|***|**",
            snapshot: "*|***|**|***|**|XXX",
            expected: "*|***|**|***|***|**",
            minimums: "X|*|*|*|*|*",
        );

        check_reset_size!(
            reset_col6_left,
            columns: 6,
            starting: "*|***|**|***|***|**",
            snapshot: "*|***|**|***|****|X",
            expected: "*|***|**|***|***|**",
            minimums: "X|*|*|*|*|*",
        );

        check_reset_size!(
            last_column_grow_cascading,
            columns: 6,
            starting: "*|***|**|**|**|***",
            snapshot: "*|*******|*|**|*|X",
            expected: "*|******|*|*|*|***",
            minimums: "X|*|*|*|*|*",
        );

        check_reset_size!(
            goes_left_when_left_has_extreme_diff,
            columns: 6,
            starting: "*|***|****|**|**|***",
            snapshot: "*|********|X|*|**|**",
            expected: "*|*****|****|*|**|**",
            minimums: "X|*|*|*|*|*",
        );

        check_reset_size!(
            basic_shrink_right,
            columns: 6,
            starting: "**|**|**|**|**|**",
            snapshot: "**|**|XXX|*|**|**",
            expected: "**|**|**|**|**|**",
            minimums: "X|*|*|*|*|*",
        );

        check_reset_size!(
            shrink_should_go_left,
            columns: 6,
            starting: "*|***|**|*|*|*",
            snapshot: "*|*|XXX|**|*|*",
            expected: "*|**|**|**|*|*",
            minimums: "X|*|*|*|*|*",
        );

        check_reset_size!(
            shrink_should_go_right,
            columns: 6,
            starting: "*|***|**|**|**|*",
            snapshot: "*|****|XXX|*|*|*",
            expected: "*|****|**|**|*|*",
            minimums: "X|*|*|*|*|*",
        );
    }

    mod drag_handle {
        use super::*;

        fn parse<const COLS: usize>(input: &str) -> ([f32; COLS], f32, Option<usize>) {
            let mut widths = [f32::NAN; COLS];
            let column_index = input.replace("*", "").find("I");
            for (index, col) in input.replace("I", "|").split('|').enumerate() {
                widths[index] = col.len() as f32;
            }

            for w in widths {
                assert!(w.is_finite(), "incorrect number of columns");
            }
            let total = widths.iter().sum::<f32>();
            for width in &mut widths {
                *width /= total;
            }
            (widths, total, column_index)
        }

        #[track_caller]
        fn check<const COLS: usize>(
            distance: i32,
            widths: &str,
            expected: &str,
            resize_behavior: &str,
        ) {
            let (mut widths, total_1, Some(column_index)) = parse::<COLS>(widths) else {
                panic!("invalid test input: widths should be marked");
            };
            let (expected, total_2, None) = parse::<COLS>(expected) else {
                panic!("invalid test input: expected should not be marked: {expected:?}");
            };
            assert_eq!(
                total_1, total_2,
                "invalid test input: total width not the same"
            );
            let resize_behavior = parse_resize_behavior::<COLS>(resize_behavior, total_1);

            let distance = distance as f32 / total_1;

            let result = TableColumnWidths::drag_column_handle(
                distance,
                column_index,
                &mut widths,
                &resize_behavior,
            );

            let is_eq = is_almost_eq(&widths, &expected);
            if !is_eq {
                let result_str = cols_to_str(&widths, total_1);
                let expected_str = cols_to_str(&expected, total_1);
                panic!(
                    "resize failed\ncomputed: {result_str}\nexpected: {expected_str}\n\ncomputed values: {result:?}\nexpected values: {expected:?}\n:minimum widths: {resize_behavior:?}"
                );
            }
        }

        macro_rules! check {
            (columns: $cols:expr, distance: $dist:expr, snapshot: $current:expr, expected: $expected:expr, resizing: $resizing:expr $(,)?) => {
                check!($cols, $dist, $snapshot, $expected, $resizing);
            };
            ($name:ident, columns: $cols:expr, distance: $dist:expr, snapshot: $current:expr, expected: $expected:expr, minimums: $resizing:expr $(,)?) => {
                #[test]
                fn $name() {
                    check::<$cols>($dist, $current, $expected, $resizing);
                }
            };
        }

        check!(
            basic_right_drag,
            columns: 3,
            distance: 1,
            snapshot: "**|**I**",
            expected: "**|***|*",
            minimums: "X|*|*",
        );

        check!(
            drag_left_against_mins,
            columns: 5,
            distance: -1,
            snapshot: "*|*|*|*I*******",
            expected: "*|*|*|*|*******",
            minimums: "X|*|*|*|*",
        );

        check!(
            drag_left,
            columns: 5,
            distance: -2,
            snapshot: "*|*|*|*****I***",
            expected: "*|*|*|***|*****",
            minimums: "X|*|*|*|*",
        );
    }
}
