use editor::Editor;
use gpui::{
    AppContext, Entity, EventEmitter, FocusHandle, Focusable, ListAlignment, ListState,
    ScrollHandle, Subscription, Task, actions,
};
use std::{sync::Arc, time::Instant};

use crate::{cell_editor::CellEditorCtx, data_table::TableInteractionState};
use ui::{SharedString, prelude::*};
use workspace::{Item, Workspace};

use crate::{
    nasty_code_duplication::ColumnWidths,
    parser::EditorState,
    performance_metrics_overlay::PerformanceMetrics,
    selection::TableSelection,
    settings::CsvPreviewSettings,
    sorting_by_column::{SortedIndices, SortingConfig},
    table_like_content::TableLikeContent,
};

mod cell_editor;
mod copy_selected;
mod data_table;
mod nasty_code_duplication;
mod parser;
mod performance_metrics_overlay;
mod render_table;
mod renderer;
mod row_identifiers;
mod selection;
mod selection_handlers;
mod settings;
mod sorting_by_column;
mod table_cell;
mod table_like_content;
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
const KEY_CONTEXT_NAME: &'static str = "CsvPreview";

pub struct CsvPreviewView {
    pub(crate) focus_handle: FocusHandle,
    /// Horizontal table scroll handle. Stinks. Won't work normally unless table column resizing is rewritten
    pub(crate) scroll_handle: ScrollHandle,
    pub(crate) active_editor: Option<EditorState>,
    pub(crate) contents: TableLikeContent,
    pub(crate) table_interaction_state: Entity<TableInteractionState>,
    pub(crate) column_widths: ColumnWidths,
    pub(crate) parsing_task: Option<Task<anyhow::Result<()>>>,
    pub(crate) sorting_cfg: Option<SortingConfig>,
    pub(crate) sorted_indices: Arc<SortedIndices>,
    pub(crate) selection: TableSelection,
    pub(crate) settings: CsvPreviewSettings,
    /// Performance metrics for debugging and monitoring CSV operations.
    pub(crate) performance_metrics: PerformanceMetrics,
    pub(crate) list_state: gpui::ListState,
    /// POC: Single-line editor
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
                let csv_preview = Self::from_editor(&editor, window, cx);
                workspace.add_item_to_active_pane(Box::new(csv_preview), None, true, window, cx);
            }
        });
    }
}

impl CsvPreviewView {
    /// Update ordered indices when ordering or content changes
    pub(crate) fn update_ordered_indices(&mut self) {
        let start_time = Instant::now();
        self.sorted_indices = Arc::new(crate::sorting_by_column::generate_sorted_indices(
            self.sorting_cfg,
            &self.contents,
        ));
        let ordering_duration = start_time.elapsed();
        self.performance_metrics.last_ordering_took = Some(ordering_duration);
    }

    /// Get reference to current sorted indices
    pub(crate) fn get_sorted_indices(&self) -> &Arc<SortedIndices> {
        &self.sorted_indices
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

    fn from_editor(
        editor: &Entity<Editor>,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) -> Entity<Self> {
        let contents = TableLikeContent::default();
        let list_state = ListState::new(contents.rows.len(), ListAlignment::Top, px(0.));
        let table_interaction_state = cx.new(|cx| {
            TableInteractionState::new(cx, list_state.clone())
                .with_custom_scrollbar(ui::Scrollbars::for_settings::<editor::EditorSettings>())
        });

        cx.new(|cx| {
            let mut view = Self {
                focus_handle: cx.focus_handle(),
                active_editor: None,
                contents: contents.clone(),
                table_interaction_state,
                column_widths: ColumnWidths::new(cx),
                parsing_task: None,
                sorting_cfg: None,
                sorted_indices: Arc::new(crate::sorting_by_column::generate_sorted_indices(
                    None, &contents,
                )),
                selection: TableSelection::default(),
                performance_metrics: PerformanceMetrics::default(),
                list_state: gpui::ListState::new(contents.rows.len(), ListAlignment::Top, px(1.)),
                settings: CsvPreviewSettings::default(),
                cell_editor: None,
                last_parse_end_time: None,
                cell_edited_flag: false,
                scroll_handle: ScrollHandle::default(),
            };

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
        self.active_editor
            .as_ref()
            .and_then(|state| {
                state
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
            })
            .unwrap_or_else(|| SharedString::from("CSV Preview"))
    }
}
