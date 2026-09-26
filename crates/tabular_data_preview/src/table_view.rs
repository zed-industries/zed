//! The reusable tabular-data viewer component.
//!
//! `TableView` owns the [`TableDataEngine`] (client-side filter/sort + display-to-data mapping)
//! and all of the grid rendering over `ui::Table`.
//!
//! It is deliberately source-agnostic: it renders whatever [`TableLikeContent`] it is handed
//! via [`TableView::set_contents`], whether that content comes from the CSV parser or from some
//! other producer (e.g. a database result set).

use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

use gpui::{
    App, AppContext, Entity, FocusHandle, Focusable, ListAlignment, ListState, Point, Task, Window,
};
use ui::{
    AbsoluteLength, ResizableColumnsState, SharedString, TableInteractionState,
    TableResizeBehavior, prelude::*,
};

use crate::{
    MoveFocusedCell, NavigationDirection,
    settings::TableViewSettings,
    table_data_engine::{DisplayToDataMapping, TableDataEngine},
    types::{AnyColumn, DataCellId, DisplayRow, TableLikeContent},
};

/// The keyboard cursor: which cell is currently focused for navigation.
/// Both fields are stored in data-space so the selection survives sort/filter changes.
/// In single-cell navigation, `anchor == focus`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CellSelection {
    pub anchor: DataCellId,
    pub focus: DataCellId,
}

impl CellSelection {
    pub fn new(anchor: DataCellId, focus: DataCellId) -> Self {
        Self { anchor, focus }
    }

    pub fn single_cell(cell: DataCellId) -> Self {
        Self {
            anchor: cell,
            focus: cell,
        }
    }

    pub fn is_single_cell(&self) -> bool {
        self.anchor == self.focus
    }
}

#[derive(Debug, Default)]
pub struct PerformanceMetrics {
    /// Map of timing metrics with their duration and measurement time.
    pub timings: HashMap<&'static str, (Duration, Instant)>,
    /// List of display indices that were rendered in the current frame.
    pub rendered_indices: Vec<usize>,
}

impl PerformanceMetrics {
    pub fn record<F, R>(&mut self, name: &'static str, mut f: F) -> R
    where
        F: FnMut() -> R,
    {
        let start_time = Instant::now();
        let ret = f();
        let duration = start_time.elapsed();
        self.timings.insert(name, (duration, Instant::now()));
        ret
    }

    /// Displays all metrics sorted A-Z in format: `{name}: {took}ms {ago}s ago`
    pub fn display(&self) -> String {
        let mut metrics = self.timings.iter().collect::<Vec<_>>();
        metrics.sort_by_key(|&(name, _)| *name);
        metrics
            .iter()
            .map(|(name, (duration, time))| {
                let took = duration.as_secs_f32() * 1000.;
                let ago = time.elapsed().as_secs();
                format!("{name}: {took:.3}ms {ago}s ago")
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Get timing for a specific metric
    pub fn get_timing(&self, name: &str) -> Option<Duration> {
        self.timings.get(name).map(|(duration, _)| *duration)
    }
}

pub struct TableView {
    pub(crate) engine: TableDataEngine,
    pub(crate) focus_handle: FocusHandle,
    pub(crate) table_interaction_state: Entity<TableInteractionState>,
    pub(crate) column_widths: Entity<ResizableColumnsState>,
    /// Background task computing the display-to-data mapping after a filter/sort change.
    /// Stored here so that a new change cancels the previous in-flight computation.
    pub(crate) filter_sort_task: Option<Task<()>>,
    pub(crate) settings: TableViewSettings,
    /// Performance metrics for debugging and monitoring grid operations.
    pub(crate) performance_metrics: PerformanceMetrics,
    pub(crate) list_state: ListState,
    /// Cached row height, refreshed from the actual text line height on every render.
    /// Used to size not-yet-rendered rows for the scrollbar without a full `.measure_all()`
    /// pass, so it tracks the real row height instead of a hardcoded guess.
    pub(crate) row_height: Pixels,
    /// Whether the producer feeding this view is currently computing content. While set, the grid
    /// shows a loading indicator instead of the (stale or empty) table.
    pub(crate) is_loading: bool,
    /// The keyboard navigation cursor. `None` until the user first navigates.
    pub(crate) selection: Option<CellSelection>,
}

impl TableView {
    pub fn new(window: &Window, cx: &mut Context<Self>) -> Self {
        let contents = TableLikeContent::default();
        let table_interaction_state = cx.new(|cx| {
            TableInteractionState::new(cx).with_custom_scrollbar(ui::Scrollbars::for_settings::<
                editor::EditorSettingsScrollbarProxy,
            >())
        });
        let row_height = window.pixel_snap(window.line_height());

        Self {
            engine: TableDataEngine::default(),
            focus_handle: cx.focus_handle(),
            table_interaction_state,
            column_widths: cx.new(|_cx| {
                ResizableColumnsState::new(
                    1,
                    vec![AbsoluteLength::Pixels(px(150.))],
                    vec![TableResizeBehavior::Resizable],
                )
            }),
            filter_sort_task: None,
            settings: TableViewSettings::default(),
            performance_metrics: PerformanceMetrics::default(),
            list_state: gpui::ListState::new(contents.rows.len(), ListAlignment::Top, px(1.))
                .with_uniform_item_height(row_height + px(1.0)),
            row_height,
            is_loading: false,
            selection: None,
        }
    }

    /// Replace the data shown by the grid. Recomputes filter menus and column widths, kicks off the
    /// display-to-data recomputation, and clears the loading state.
    pub fn set_contents(&mut self, contents: TableLikeContent, cx: &mut Context<Self>) {
        self.engine.set_contents(contents);
        // The old mapping may reference rows removed by this change. Clear it immediately
        // rather than leaving the list showing stale rows until the background task below
        // recomputes the mapping.
        self.list_state
            .reset_with_uniform_height(0, self.row_height + px(1.0));
        self.sync_column_widths(cx);
        self.is_loading = false;
        self.apply_filter_sort(cx);
    }

    /// Toggle the loading indicator (shown while a producer computes new content).
    pub fn set_loading(&mut self, is_loading: bool, cx: &mut Context<Self>) {
        self.is_loading = is_loading;
        cx.notify();
    }

    pub(crate) fn sync_column_widths(&self, cx: &mut Context<Self>) {
        // plus 1 for the row identifier column
        let cols = self.engine.contents.headers.cols() + 1;
        let line_number_width = self.calculate_row_identifier_column_width();

        let mut widths: Vec<AbsoluteLength> = vec![AbsoluteLength::Pixels(px(150.)); cols];
        widths[0] = AbsoluteLength::Pixels(px(line_number_width));

        let mut resize_behaviors = vec![TableResizeBehavior::Resizable; cols];
        resize_behaviors[0] = TableResizeBehavior::None;

        self.column_widths.update(cx, |state, _cx| {
            if state.cols() != cols {
                *state = ResizableColumnsState::new(cols, widths, resize_behaviors);
            } else {
                state.set_column_configuration(
                    0,
                    AbsoluteLength::Pixels(px(line_number_width)),
                    TableResizeBehavior::None,
                );
            }
        });
    }

    pub fn clear_filters(&mut self, col: AnyColumn, cx: &mut Context<Self>) {
        self.engine.clear_filters_for_col(col);
        self.apply_filter_sort(cx);
    }

    pub fn toggle_filter(
        &mut self,
        col: AnyColumn,
        value: Option<SharedString>,
        cx: &mut Context<Self>,
    ) {
        if let Err(err) = self.engine.toggle_filter(col, value) {
            log::error!("Failed to toggle filter: {err}");
            return;
        }
        self.apply_filter_sort(cx);
    }

    /// Spawns a background task to recompute the display-to-data mapping after a filter or sort
    /// change. Storing the task cancels any previous in-flight computation automatically.
    pub(crate) fn apply_filter_sort(&mut self, cx: &mut Context<Self>) {
        let contents = self.engine.contents.clone();
        let filter_stack = self.engine.filter_stack.clone();
        let sorting = self.engine.applied_sorting;

        self.filter_sort_task = Some(cx.spawn(async move |this, cx| {
            let mapping = cx
                .background_spawn(async move {
                    DisplayToDataMapping::compute(&contents, &filter_stack, sorting)
                })
                .await;

            this.update(cx, |view, cx| {
                view.engine.set_d2d_mapping(mapping);
                let visible_rows = view.engine.d2d_mapping().visible_row_count();
                // Uses the row height measured on the last render. Cheaper than a full
                // `.measure_all()` pass; exact row heights are re-measured on scrolling.
                view.list_state
                    .reset_with_uniform_height(visible_rows, view.row_height + px(1.0));
                cx.notify();
            })
            .ok();
        }));
    }

    #[inline]
    pub(crate) fn is_selection_enabled(&self) -> bool {
        cfg!(any(test, debug_assertions))
    }

    pub(crate) fn move_focused_cell(
        &mut self,
        action: &MoveFocusedCell,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if !self.is_selection_enabled() {
            return;
        }

        let row_count = self.engine.d2d_mapping().visible_row_count();
        let column_count = self.engine.contents.number_of_cols;

        if row_count == 0 || column_count == 0 {
            return;
        }

        let current_pos = self.selection.as_ref().and_then(|selection| {
            self.engine
                .d2d_mapping()
                .get_display_row(selection.focus.row)
                .map(|r| (r.0, selection.focus.col.0))
        });

        let (new_row, new_column) = match current_pos {
            Some(pos) => self.compute_move(pos, action.direction),
            None => (0, 0),
        };

        let Some(new_data_row) = self.engine.d2d_mapping().get_data_row(DisplayRow(new_row)) else {
            return;
        };
        let new_cell = DataCellId::new(new_data_row, AnyColumn(new_column));
        self.selection = Some(CellSelection::single_cell(new_cell));

        self.scroll_to_reveal_row(new_row, action.direction);
        self.scroll_to_reveal_column(new_column, window, cx);
        cx.notify();
    }

    pub(crate) fn scroll_to_reveal_row(&self, row: usize, direction: NavigationDirection) {
        let row_count = self.engine.d2d_mapping().visible_row_count();
        if row_count == 0 {
            return;
        }

        match direction {
            NavigationDirection::Down => {
                // When moving down, reveal row + 1 (if not already at the end) so that the focused
                // cell isn't obscured by the bottom border or horizontal scrollbar overlay.
                let reveal_row = (row + 1).min(row_count.saturating_sub(1));
                self.list_state.scroll_to_reveal_item(reveal_row);
            }
            NavigationDirection::Up => {
                // When moving up, reveal row - 1 (if not at top) for top peek cushion.
                let reveal_row = row.saturating_sub(1);
                self.list_state.scroll_to_reveal_item(reveal_row);
            }
            NavigationDirection::Left | NavigationDirection::Right => {
                self.list_state.scroll_to_reveal_item(row);
            }
        }
    }

    pub(crate) fn compute_move(
        &self,
        current: (usize, usize),
        direction: NavigationDirection,
    ) -> (usize, usize) {
        let row_count = self.engine.d2d_mapping().visible_row_count();
        let column_count = self.engine.contents.number_of_cols;
        let (row, column) = current;

        match direction {
            NavigationDirection::Up => (row.saturating_sub(1), column),
            NavigationDirection::Down => ((row + 1).min(row_count.saturating_sub(1)), column),
            NavigationDirection::Left => (row, column.saturating_sub(1)),
            NavigationDirection::Right => (row, (column + 1).min(column_count.saturating_sub(1))),
        }
    }

    pub(crate) fn scroll_to_reveal_column(&self, column_index: usize, window: &Window, cx: &App) {
        let handle = self
            .table_interaction_state
            .read(cx)
            .horizontal_scroll_handle
            .clone();

        let widths_state = self.column_widths.read(cx);
        let total_cols = widths_state.cols();
        if total_cols <= 1 || column_index + 2 > total_cols {
            return;
        }

        let rem_size = window.rem_size();
        let peek_padding = px(20.0);

        let left_px = widths_state.pinned_width(column_index + 1, rem_size)
            - widths_state.pinned_width(1, rem_size);
        let right_px = widths_state.pinned_width(column_index + 2, rem_size)
            - widths_state.pinned_width(1, rem_size);

        let scroll_x = -handle.offset().x;
        let viewport_width = handle.bounds().size.width;

        if left_px < scroll_x {
            handle.set_offset(Point::new(-left_px, px(0.)));
        } else if right_px + peek_padding > scroll_x + viewport_width {
            let target = (right_px + peek_padding) - viewport_width;
            handle.set_offset(Point::new(-target, px(0.)));
        }
    }
}

impl Focusable for TableView {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

#[cfg(test)]
mod tests {
    use gpui::{Action, TestAppContext, VisualTestContext};

    use super::*;
    use crate::types::{DataRow, TableCell, TableLikeContent};
    use ui::table_row::TableRow;

    fn setup_test_view(
        cx: &mut TestAppContext,
        rows: usize,
        cols: usize,
    ) -> (Entity<TableView>, &mut VisualTestContext) {
        cx.update(|cx| {
            workspace::AppState::test(cx);
            editor::init(cx);
        });

        cx.add_window_view(|window, cx| {
            let mut view = TableView::new(window, cx);
            let mut contents = TableLikeContent::default();
            contents.number_of_cols = cols;
            contents.headers = TableRow::from_vec(
                (0..cols)
                    .map(|i| TableCell::Generated(format!("col_{i}").into()))
                    .collect(),
                cols,
            );
            for _ in 0..rows {
                let cells: Vec<TableCell> = (0..cols)
                    .map(|_| TableCell::Generated("val".into()))
                    .collect();
                contents.rows.push(TableRow::from_vec(cells, cols));
            }
            view.set_contents(contents, cx);
            view.engine.set_d2d_mapping(DisplayToDataMapping::compute(
                &view.engine.contents,
                &view.engine.filter_stack,
                view.engine.applied_sorting,
            ));
            view
        })
    }

    fn dispatch<A: Action>(view: &Entity<TableView>, action: &A, cx: &mut VisualTestContext) {
        cx.update(|window, cx| {
            let focus_handle = view.read(cx).focus_handle(cx);
            focus_handle.focus(window, cx);
            focus_handle.dispatch_action(action, window, cx);
        });
    }

    fn selection_cells(
        view: &Entity<TableView>,
        cx: &VisualTestContext,
    ) -> Option<(DataCellId, DataCellId)> {
        view.read_with(cx, |this, _| {
            let selection = this.selection.as_ref()?;
            Some((selection.anchor, selection.focus))
        })
    }

    #[gpui::test]
    fn test_compute_move_directions(cx: &mut TestAppContext) {
        let (view, cx) = setup_test_view(cx, 10, 5);

        use NavigationDirection as ND;
        view.read_with(cx, |this, _| {
            assert_eq!(this.compute_move((0, 0), ND::Down), (1, 0));
            assert_eq!(this.compute_move((0, 0), ND::Right), (0, 1));
            assert_eq!(this.compute_move((0, 0), ND::Up), (0, 0));
            assert_eq!(this.compute_move((0, 0), ND::Left), (0, 0));
            assert_eq!(this.compute_move((9, 4), ND::Down), (9, 4));
            assert_eq!(this.compute_move((9, 4), ND::Right), (9, 4));
        });
    }

    #[gpui::test]
    fn test_move_focused_cell_action(cx: &mut TestAppContext) {
        let (view, cx) = setup_test_view(cx, 10, 5);
        let cell = |row, column| DataCellId::new(DataRow(row), AnyColumn(column));

        // 1. First move initializes focus at (0, 0)
        dispatch(
            &view,
            &MoveFocusedCell {
                direction: NavigationDirection::Down,
            },
            cx,
        );
        assert_eq!(selection_cells(&view, cx), Some((cell(0, 0), cell(0, 0))));

        // 2. Step Down and Right -> single cell focus at (1, 1)
        dispatch(
            &view,
            &MoveFocusedCell {
                direction: NavigationDirection::Down,
            },
            cx,
        );
        dispatch(
            &view,
            &MoveFocusedCell {
                direction: NavigationDirection::Right,
            },
            cx,
        );
        assert_eq!(selection_cells(&view, cx), Some((cell(1, 1), cell(1, 1))));
    }
}
