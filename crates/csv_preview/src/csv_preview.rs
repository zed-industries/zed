use editor::Editor;
use gpui::{
    AppContext, Entity, EventEmitter, FocusHandle, Focusable, ListAlignment, ListState,
    ScrollHandle, Task, actions,
};
use std::{
    collections::HashMap,
    sync::Arc,
    time::{Duration, Instant},
};

use crate::{
    cell_editor::CellEditorCtx,
    data_table::TableInteractionState,
    table_data_engine::{
        DisplayToDataMapping, TableDataEngine, filtering_by_column::AppliedFiltering,
    },
};
use ui::{SharedString, prelude::*};
use workspace::{Item, Workspace};

use crate::renderer::nasty_code_duplication::ColumnWidths;
use crate::{
    parser::EditorState, settings::CsvPreviewSettings,
    table_data_engine::selection::TableSelection, types::TableLikeContent,
};

pub use types::data_table;
mod action_handlers;
mod cell_editor;
mod parser;
mod renderer;
mod settings;
mod table_data_engine;
mod types;

actions!(
    csv,
    [
        OpenPreview,
        CopySelected,
        SelectAll,
        ClearSelection,
        ///// Single cell selection /////
        SelectUp,
        SelectDown,
        SelectLeft,
        SelectRight,
        ///// Selection extension /////
        ExtendSelectionUp,
        ExtendSelectionDown,
        ExtendSelectionLeft,
        ExtendSelectionRight,
        ///// Single cell selection at edge /////
        SelectAtTopEdge,
        SelectAtBottomEdge,
        SelectAtLeftEdge,
        SelectAtRightEdge,
        ///// Selection extension to edge /////
        ExtendSelectionToTopEdge,
        ExtendSelectionToBottomEdge,
        ExtendSelectionToLeftEdge,
        ExtendSelectionToRightEdge,
        ///// Cell editing /////
        StartCellEditing,
        FinishCellEditing,
        CancelCellEditing,
    ]
);
const TABLE_CONTEXT_NAME: &'static str = "CsvPreview";
const CELL_EDITOR_CONTEXT_NAME: &'static str = "TableCellEditor";

pub struct CsvPreviewView {
    pub(crate) engine: TableDataEngine,

    pub(crate) focus_handle: FocusHandle,
    /// Horizontal table scroll handle. Stinks. Won't work normally unless table column resizing is rewritten
    pub(crate) scroll_handle: ScrollHandle,
    active_editor_state: Option<EditorState>,
    pub(crate) table_interaction_state: Entity<TableInteractionState>,
    pub(crate) column_widths: ColumnWidths,
    pub(crate) parsing_task: Option<Task<anyhow::Result<()>>>,
    pub(crate) settings: CsvPreviewSettings,
    /// Performance metrics for debugging and monitoring CSV operations.
    pub(crate) performance_metrics: PerformanceMetrics,
    pub(crate) list_state: gpui::ListState,
    /// Context of inline cell editor. If None - no editing is in progress.
    pub(crate) cell_editor: Option<CellEditorCtx>,
    /// Time when the last parsing operation ended, used for smart debouncing
    pub(crate) last_parse_end_time: Option<std::time::Instant>,
    /// Used to signalize parser that the cell was edited, and reparsing is needed.
    /// Emited by `cell_editor` module on submit.
    /// Is needed, as `buffer.edit([(range, new_text)], None, cx);` emits `BufferEdited` event,
    /// which is too generic, and this flag shows that the source of that event is indeed cell editing
    pub(crate) cell_edited_flag: bool,
}

pub fn init(cx: &mut App) {
    cx.observe_new(|workspace: &mut Workspace, window, cx| {
        let Some(window) = window else {
            return;
        };
        CsvPreviewView::register(workspace, window, cx);
    })
    .detach()
}

impl CsvPreviewView {
    pub fn register(
        workspace: &mut Workspace,
        _window: &mut Window,
        _cx: &mut Context<'_, Workspace>,
    ) {
        workspace.register_action(|workspace, _: &OpenPreview, window, cx| {
            if let Some(editor) = workspace
                .active_item(cx)
                .and_then(|item| item.act_as::<Editor>(cx))
                .filter(|editor| Self::is_csv_file(editor, cx))
            {
                let csv_preview = Self::new(&editor, cx);
                workspace.add_item_to_active_pane(Box::new(csv_preview), None, true, window, cx);
            }
        });
    }
}

impl CsvPreviewView {
    pub(crate) fn editor_state(&self) -> &EditorState {
        self.active_editor_state
            .as_ref()
            .expect("Expected main editor to be initialized")
    }
    pub(crate) fn apply_sort(&mut self) {
        self.performance_metrics.record("Sort", || {
            self.engine.re_apply_sort();
        });
    }

    /// Update ordered indices when ordering or content changes
    pub(crate) fn apply_filter_sort(&mut self) {
        self.performance_metrics.record("Filter&sort", || {
            self.engine.calculate_d2d_mapping();
        });

        // Update list state with filtered row count
        let filtered_row_count = self.engine.get_d2d_mapping().filtered_row_count();
        self.list_state = gpui::ListState::new(filtered_row_count, ListAlignment::Top, px(1.));
    }

    fn is_csv_file(editor: &Entity<Editor>, cx: &App) -> bool {
        editor
            .read(cx)
            .buffer()
            .read(cx)
            .as_singleton()
            .and_then(|buffer| {
                buffer
                    .read(cx)
                    .file()
                    .and_then(|file| file.path().extension())
                    .map(|ext| ext == "csv")
            })
            .unwrap_or(false)
    }

    fn new(editor: &Entity<Editor>, cx: &mut Context<Workspace>) -> Entity<Self> {
        let contents = TableLikeContent::default();
        let list_state = ListState::new(contents.rows.len(), ListAlignment::Top, px(0.));
        let table_interaction_state = cx.new(|cx| {
            TableInteractionState::new(cx, list_state.clone())
                .with_custom_scrollbar(ui::Scrollbars::for_settings::<editor::EditorSettings>())
        });

        cx.new(|cx| {
            let mut view = CsvPreviewView {
                focus_handle: cx.focus_handle(),
                active_editor_state: None,
                table_interaction_state,
                column_widths: ColumnWidths::new(cx),
                parsing_task: None,
                performance_metrics: PerformanceMetrics::default(),
                list_state: gpui::ListState::new(contents.rows.len(), ListAlignment::Top, px(1.)),
                settings: CsvPreviewSettings::default(),
                cell_editor: None,
                last_parse_end_time: None,
                cell_edited_flag: false,
                scroll_handle: ScrollHandle::default(),
                engine: TableDataEngine {
                    applied_sorting: None,
                    available_filters: HashMap::new(),
                    d2d_mapping: Arc::new(DisplayToDataMapping::default()),
                    contents: contents.clone(),
                    selection: TableSelection::default(),
                    applied_filtering: AppliedFiltering::new(),
                },
            };

            // No need to trigger any filtering / sorting here, as it's retrigered on parsing

            view.set_editor(editor.clone(), cx);
            view
        })
    }
}

impl Focusable for CsvPreviewView {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<()> for CsvPreviewView {}

/// Definition of tab name / icon
impl Item for CsvPreviewView {
    type Event = ();

    fn tab_icon(&self, _window: &Window, _cx: &App) -> Option<Icon> {
        Some(Icon::new(IconName::FileDoc))
    }

    fn tab_content_text(&self, _detail: usize, cx: &App) -> SharedString {
        self.editor_state()
            .editor
            .read(cx)
            .buffer()
            .read(cx)
            .as_singleton()
            .and_then(|b| {
                let file = b.read(cx).file()?;
                let local_file = file.as_local()?;
                local_file
                    .abs_path(cx)
                    .file_name()
                    .map(|name| format!("Preview {}", name.to_string_lossy()).into())
            })
            .unwrap_or_else(|| SharedString::from("CSV Preview"))
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
                let took = duration.as_millis();
                let ago = time.elapsed().as_secs();
                format!("{name}: {took}ms {ago}s ago")
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Get timing for a specific metric
    pub fn get_timing(&self, name: &str) -> Option<Duration> {
        self.timings.get(name).map(|(duration, _)| *duration)
    }
}
