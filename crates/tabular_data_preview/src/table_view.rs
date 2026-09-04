//! The reusable tabular-data viewer component.
//!
//! `TableView` owns the [`TableDataEngine`] (client-side filter/sort + display-to-data mapping)
//! and all of the grid rendering over `ui::Table`. It is deliberately source-agnostic: it renders
//! whatever [`TableLikeContent`] it is handed via [`TableView::set_contents`], whether that content
//! comes from the CSV parser or from some other producer (e.g. a database result set). Callers
//! embed it as a child entity and own their own chrome (tab, toolbar, connection state, etc.).

use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

use gpui::{App, AppContext, Entity, FocusHandle, Focusable, ListAlignment, Task};
use ui::{
    AbsoluteLength, ResizableColumnsState, SharedString, TableInteractionState,
    TableResizeBehavior, prelude::*,
};

use crate::{
    settings::TableViewSettings,
    table_data_engine::{DisplayToDataMapping, TableDataEngine},
    types::{AnyColumn, TableLikeContent},
};

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
    pub(crate) list_state: gpui::ListState,
    /// Cached row height, refreshed from the actual text line height on every render.
    /// Used to size not-yet-rendered rows for the scrollbar without a full `.measure_all()`
    /// pass, so it tracks the real row height instead of a hardcoded guess.
    pub(crate) row_height: Pixels,
    /// Whether the producer feeding this view is currently computing content. While set, the grid
    /// shows a loading indicator instead of the (stale or empty) table.
    pub(crate) is_loading: bool,
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
                .with_uniform_item_height(row_height),
            row_height,
            is_loading: false,
        }
    }

    /// Replace the data shown by the grid. Recomputes filter menus and column widths, kicks off the
    /// display-to-data recomputation, and clears the loading state.
    pub fn set_contents(&mut self, contents: TableLikeContent, cx: &mut Context<Self>) {
        self.engine.contents = std::sync::Arc::new(contents);
        self.engine.calculate_available_filters();
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
                    .reset_with_uniform_height(visible_rows, view.row_height);
                cx.notify();
            })
            .ok();
        }));
    }
}

impl Focusable for TableView {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}
