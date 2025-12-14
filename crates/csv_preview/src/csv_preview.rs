use editor::Editor;
use gpui::{AppContext, Entity, EventEmitter, FocusHandle, Focusable, Task, actions};
use std::{sync::Arc, time::Instant};

use ui::{SharedString, TableInteractionState, prelude::*};
use workspace::{Item, Workspace};

use crate::{
    data_ordering::{OrderedIndices, Ordering},
    nasty_code_duplication::ColumnWidths,
    parser::EditorState,
    performance_metrics_overlay::PerformanceMetrics,
    selection::TableSelection,
    settings::CsvPreviewSettings,
    table_like_content::TableLikeContent,
};

mod copy_selected;
mod data_ordering;
mod nasty_code_duplication;
mod parser;
mod performance_metrics_overlay;
mod render_table;
mod renderer;
mod row_identifiers;
mod selection;
mod selection_handlers;
mod settings;
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
        ExtendSelectionToRightEdge
    ]
);
const KEY_CONTEXT_NAME: &'static str = "CsvPreview";

pub struct CsvPreviewView {
    pub(crate) focus_handle: FocusHandle,
    pub(crate) active_editor: Option<EditorState>,
    pub(crate) contents: TableLikeContent,
    pub(crate) table_interaction_state: Entity<TableInteractionState>,
    pub(crate) column_widths: ColumnWidths,
    pub(crate) parsing_task: Option<Task<anyhow::Result<()>>>,
    pub(crate) ordering: Option<Ordering>,
    pub(crate) ordered_indices: Arc<OrderedIndices>,
    pub(crate) selection: TableSelection,
    pub(crate) settings: CsvPreviewSettings,
    /// Performance metrics for debugging and monitoring CSV operations.
    pub(crate) performance_metrics: PerformanceMetrics,
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
                let csv_preview = Self::from_editor(&editor, cx);
                workspace.add_item_to_active_pane(Box::new(csv_preview), None, true, window, cx);
            }
        });
    }
}

impl CsvPreviewView {
    /// Update ordered indices when ordering or content changes
    pub(crate) fn update_ordered_indices(&mut self) {
        let start_time = Instant::now();
        self.ordered_indices = Arc::new(crate::data_ordering::generate_ordered_indices(
            self.ordering,
            &self.contents,
        ));
        let ordering_duration = start_time.elapsed();
        self.performance_metrics.last_ordering_took = Some(ordering_duration);
    }

    /// Get reference to current ordered indices
    pub(crate) fn get_ordered_indices(&self) -> &Arc<OrderedIndices> {
        &self.ordered_indices
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

    fn from_editor(editor: &Entity<Editor>, cx: &mut Context<Workspace>) -> Entity<Self> {
        let table_interaction_state = cx.new(|cx| TableInteractionState::new(cx));
        let contents = TableLikeContent::default();

        cx.new(|cx| {
            let mut view = Self {
                focus_handle: cx.focus_handle(),
                active_editor: None,
                contents: contents.clone(),
                table_interaction_state,
                column_widths: ColumnWidths::new(cx),
                parsing_task: None,
                ordering: None,
                ordered_indices: Arc::new(crate::data_ordering::generate_ordered_indices(
                    None, &contents,
                )),
                selection: TableSelection::new(),
                settings: CsvPreviewSettings::default(),
                performance_metrics: PerformanceMetrics::default(),
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
