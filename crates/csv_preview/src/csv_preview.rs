use editor::Editor;
use gpui::{AppContext, Entity, EventEmitter, FocusHandle, Focusable, Task, actions};
use std::time::Instant;

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
mod settings;
mod table_cell;
mod table_like_content;
mod types;

actions!(
    csv,
    [
        OpenPreview,
        CopySelected,
        ClearSelection,
        MoveFocusUp,
        MoveFocusDown,
        MoveFocusLeft,
        MoveFocusRight,
        SelectUp,
        SelectDown,
        SelectLeft,
        SelectRight,
        SelectAll,
        JumpToTopEdge,
        JumpToBottomEdge,
        JumpToLeftEdge,
        JumpToRightEdge,
        SelectionToTopEdge,
        SelectionToBottomEdge,
        SelectionToLeftEdge,
        SelectionToRightEdge
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
    pub(crate) ordered_indices: OrderedIndices,
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

    fn clear_selection(
        &mut self,
        _: &ClearSelection,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.selection = TableSelection::new();
        cx.notify();
    }

    fn move_focus_up(&mut self, _: &MoveFocusUp, _window: &mut Window, cx: &mut Context<Self>) {
        self.selection.move_focus_up(&self.ordered_indices);
        cx.notify();
    }

    fn move_focus_down(&mut self, _: &MoveFocusDown, _window: &mut Window, cx: &mut Context<Self>) {
        let max_rows = self.contents.rows.len();
        self.selection
            .move_focus_down(&self.ordered_indices, max_rows);
        cx.notify();
    }

    fn move_focus_left(&mut self, _: &MoveFocusLeft, _window: &mut Window, cx: &mut Context<Self>) {
        self.selection.move_focus_left(&self.ordered_indices);
        cx.notify();
    }

    fn move_focus_right(
        &mut self,
        _: &MoveFocusRight,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let max_cols = self.contents.headers.len();
        self.selection
            .move_focus_right(&self.ordered_indices, max_cols);
        cx.notify();
    }

    fn select_up(&mut self, _: &SelectUp, _window: &mut Window, cx: &mut Context<Self>) {
        self.selection.extend_selection_up(&self.ordered_indices);
        cx.notify();
    }

    fn select_down(&mut self, _: &SelectDown, _window: &mut Window, cx: &mut Context<Self>) {
        let max_rows = self.contents.rows.len();
        self.selection
            .extend_selection_down(&self.ordered_indices, max_rows);
        cx.notify();
    }

    fn select_left(&mut self, _: &SelectLeft, _window: &mut Window, cx: &mut Context<Self>) {
        self.selection.extend_selection_left(&self.ordered_indices);
        cx.notify();
    }

    fn select_right(&mut self, _: &SelectRight, _window: &mut Window, cx: &mut Context<Self>) {
        let max_cols = self.contents.headers.len();
        self.selection
            .extend_selection_right(&self.ordered_indices, max_cols);
        cx.notify();
    }

    fn select_all(&mut self, _: &SelectAll, _window: &mut Window, cx: &mut Context<Self>) {
        let max_rows = self.contents.rows.len();
        let max_cols = self.contents.headers.len();
        self.selection
            .select_all(&self.ordered_indices, max_rows, max_cols);
        cx.notify();
    }

    fn jump_to_top_edge(
        &mut self,
        _: &JumpToTopEdge,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.selection.jump_to_top_edge(&self.ordered_indices);
        cx.notify();
    }

    fn jump_to_bottom_edge(
        &mut self,
        _: &JumpToBottomEdge,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let max_rows = self.contents.rows.len();
        self.selection
            .jump_to_bottom_edge(&self.ordered_indices, max_rows);
        cx.notify();
    }

    fn jump_to_left_edge(
        &mut self,
        _: &JumpToLeftEdge,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.selection.jump_to_left_edge(&self.ordered_indices);
        cx.notify();
    }

    fn jump_to_right_edge(
        &mut self,
        _: &JumpToRightEdge,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let max_cols = self.contents.headers.len();
        self.selection
            .jump_to_right_edge(&self.ordered_indices, max_cols);
        cx.notify();
    }

    fn extend_selection_to_top_edge(
        &mut self,
        _: &SelectionToTopEdge,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.selection
            .extend_selection_to_top_edge(&self.ordered_indices);
        cx.notify();
    }

    fn extend_selection_to_bottom_edge(
        &mut self,
        _: &SelectionToBottomEdge,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let max_rows = self.contents.rows.len();
        self.selection
            .extend_selection_to_bottom_edge(&self.ordered_indices, max_rows);
        cx.notify();
    }

    fn extend_selection_to_left_edge(
        &mut self,
        _: &SelectionToLeftEdge,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.selection
            .extend_selection_to_left_edge(&self.ordered_indices);
        cx.notify();
    }

    fn extend_selection_to_right_edge(
        &mut self,
        _: &SelectionToRightEdge,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let max_cols = self.contents.headers.len();
        self.selection
            .extend_selection_to_right_edge(&self.ordered_indices, max_cols);
        cx.notify();
    }

    /// Update ordered indices when ordering or content changes
    pub(crate) fn update_ordered_indices(&mut self) {
        let start_time = Instant::now();
        self.ordered_indices =
            crate::data_ordering::generate_ordered_indices(self.ordering, &self.contents);
        let ordering_duration = start_time.elapsed();
        self.performance_metrics.last_ordering_took = Some(ordering_duration);
    }

    /// Get reference to current ordered indices
    pub(crate) fn get_ordered_indices(&self) -> &OrderedIndices {
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
                ordered_indices: crate::data_ordering::generate_ordered_indices(None, &contents),
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
