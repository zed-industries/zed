use editor::Editor;
use gpui::{AppContext, Entity, EventEmitter, FocusHandle, Focusable, Task, actions};
use std::collections::HashSet;

use ui::{SharedString, TableInteractionState, prelude::*};
use workspace::{Item, Workspace};

use crate::{nasty_code_duplication::ColumnWidths, parsed_csv::ParsedCsv, parser::EditorState};

mod nasty_code_duplication;
mod parsed_csv;
mod parser;
mod renderer;

actions!(csv, [OpenPreview]);

pub fn init(cx: &mut App) {
    cx.observe_new(|workspace: &mut Workspace, window, cx| {
        let Some(window) = window else {
            return;
        };
        CsvPreviewView::register(workspace, window, cx);
    })
    .detach()
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum OrderingDirection {
    Asc,
    Desc,
}

#[derive(Clone, Copy)]
pub struct Ordering {
    /// 0-based column index
    pub col_idx: usize,
    /// Direction of ordering
    pub direction: OrderingDirection,
}

pub struct CsvPreviewView {
    pub(crate) focus_handle: FocusHandle,
    pub(crate) active_editor: Option<EditorState>,
    pub(crate) contents: ParsedCsv,
    pub(crate) table_interaction_state: Entity<TableInteractionState>,
    pub(crate) column_widths: ColumnWidths,
    pub(crate) parsing_task: Option<Task<anyhow::Result<()>>>,
    pub(crate) ordering: Option<Ordering>,
    /// Cell selection using data coordinates (data_row, col).
    ///
    /// IMPORTANT: This stores CSV data coordinates, NOT display coordinates.
    /// - data_row: The row index in the original CSV data (0-based)
    /// - col: The CSV data column index (0-based, excluding line number column)
    ///
    /// ## Selection Behavior Examples
    ///
    /// **Scenario 1: User selects, then sorts**
    /// 1. CSV data: [Alice, Bob, Charlie]  (display: Alice=0, Bob=1, Charlie=2)
    /// 2. User clicks display row 1 (Bob) → stores data_row=1, col=0
    /// 3. User sorts by name → display changes: [Alice=0, Bob=1, Charlie=2]
    /// 4. Bob stays selected at display row 1 (same position, data follows)
    ///
    /// **Scenario 2: User selects, then sorts differently**
    /// 1. CSV data: [Alice:30, Bob:25, Charlie:35]
    /// 2. User clicks display row 0 (Alice) → stores data_row=0, col=0
    /// 3. User sorts by age → display: [Bob:25=0, Alice:30=1, Charlie:35=2]
    /// 4. Alice moves to display row 1, but stays selected (data follows)
    ///
    /// ## Technical Flow
    /// 1. User input: Always display coordinates (what they see/click)
    /// 2. Storage: Immediately convert to data coordinates
    /// 3. Sorting: Selection automatically follows the data
    /// 4. Rendering: Convert data → display coordinates for highlighting
    pub(crate) selected_cells: HashSet<(usize, usize)>, // (data_row, col) - using data coordinates
    pub(crate) selection_start_display: Option<(usize, usize)>, // (display_row, col) - for extend_selection_to
    pub(crate) is_selecting: bool,
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
        let contents = ParsedCsv::default();

        cx.new(|cx| {
            let mut view = Self {
                focus_handle: cx.focus_handle(),
                active_editor: None,
                contents,
                table_interaction_state,
                column_widths: ColumnWidths::new(cx),
                parsing_task: None,
                ordering: None,
                selected_cells: HashSet::new(),
                selection_start_display: None,
                is_selecting: false,
            };

            view.set_editor(editor.clone(), cx);
            view
        })
    }

    /// Start cell selection at the given display position.
    ///
    /// **Input**: Display coordinates (what user clicked on screen)
    /// **Storage**: Data coordinates (so selection follows data when sorting)
    ///
    /// Example: User clicks visually displayed row 2, column 1
    /// → Converts to original CSV data row (e.g. row 5) for storage
    /// → If table gets sorted, selection stays with that data row
    pub(crate) fn start_selection(
        &mut self,
        display_row: usize,
        col: usize,
        cx: &mut Context<Self>,
    ) {
        self.selected_cells.clear();

        // Convert display coordinates to data coordinates for storage
        if let Some(data_row) = self.display_to_data_row(display_row) {
            self.selected_cells.insert((data_row, col));
        }

        // Remember display coordinates for extend_selection_to
        self.selection_start_display = Some((display_row, col));
        self.is_selecting = true;
        cx.notify();
    }

    /// Extend selection to include cells from start position to current position.
    ///
    /// **Algorithm**:
    /// 1. Create rectangle in display coordinates (what user sees)
    /// 2. Convert each display cell → data coordinates for storage
    /// 3. Selection persisted as data coordinates (follows data on sort)
    ///
    /// **Example**: User drags from display (0,0) to (2,1)
    /// → Creates 3×2 rectangle in display space
    /// → Converts each cell: display(0,0)→data(3,0), display(0,1)→data(3,1), etc.
    /// → Stores data coordinates so selection survives sorting
    pub(crate) fn extend_selection_to(
        &mut self,
        display_row: usize,
        col: usize,
        cx: &mut Context<Self>,
    ) {
        if let Some((start_display_row, start_col)) = self.selection_start_display {
            self.selected_cells.clear();

            // Create rectangle in display coordinates
            let min_display_row = start_display_row.min(display_row);
            let max_display_row = start_display_row.max(display_row);
            let min_col = start_col.min(col);
            let max_col = start_col.max(col);

            // Convert each display cell to data coordinates for storage
            for display_r in min_display_row..=max_display_row {
                for c in min_col..=max_col {
                    if let Some(data_row) = self.display_to_data_row(display_r) {
                        self.selected_cells.insert((data_row, c));
                    }
                }
            }
            cx.notify();
        }
    }

    /// End cell selection
    pub(crate) fn end_selection(&mut self, cx: &mut Context<Self>) {
        self.is_selecting = false;
        cx.notify();
    }

    /// Check if a cell is currently selected using display coordinates.
    ///
    /// **Input**: Display coordinates (renderer asking "is this visible cell selected?")
    /// **Process**: Convert display → data coordinates, check against stored selection
    /// **Output**: True if the data at this display position is selected
    ///
    /// This ensures selection highlighting follows the data regardless of sort order.
    pub(crate) fn is_cell_selected(&self, display_row: usize, col: usize) -> bool {
        if let Some(data_row) = self.display_to_data_row(display_row) {
            self.selected_cells.contains(&(data_row, col))
        } else {
            false
        }
    }

    /// Generate ordered row indices based on current ordering settings.
    /// Note: ordering.col_idx refers to CSV data columns (0-based), not display columns
    /// (display columns include the line number column at index 0)
    pub(crate) fn generate_ordered_indices(&self) -> Vec<usize> {
        let mut indices: Vec<usize> = (0..self.contents.rows.len()).collect();

        if let Some(ordering) = self.ordering {
            indices.sort_by(|&a, &b| {
                let row_a = &self.contents.rows[a];
                let row_b = &self.contents.rows[b];

                let val_a = row_a
                    .get(ordering.col_idx)
                    .map(|s| s.as_ref())
                    .unwrap_or("");
                let val_b = row_b
                    .get(ordering.col_idx)
                    .map(|s| s.as_ref())
                    .unwrap_or("");

                // Try numeric comparison first, fall back to string comparison
                let cmp = match (val_a.parse::<f64>(), val_b.parse::<f64>()) {
                    (Ok(num_a), Ok(num_b)) => num_a
                        .partial_cmp(&num_b)
                        .unwrap_or(std::cmp::Ordering::Equal),
                    _ => val_a.cmp(val_b),
                };

                match ordering.direction {
                    crate::OrderingDirection::Asc => cmp,
                    crate::OrderingDirection::Desc => cmp.reverse(),
                }
            });
        }

        indices
    }

    /// Get the actual data row index from display row index
    pub(crate) fn display_to_data_row(&self, display_row: usize) -> Option<usize> {
        let ordered_indices = self.generate_ordered_indices();
        ordered_indices.get(display_row).copied()
    }

    /// Convert data row index to current display row index
    pub(crate) fn data_to_display_row(&self, data_row: usize) -> Option<usize> {
        let ordered_indices = self.generate_ordered_indices();
        ordered_indices.iter().position(|&idx| idx == data_row)
    }

    /// Get selected data positions (already stored as data coordinates)
    pub(crate) fn get_selected_data_positions(&self) -> Vec<(usize, usize)> {
        self.selected_cells.iter().copied().collect()
    }

    /// Clear all cell selection
    pub(crate) fn clear_selection(&mut self, cx: &mut Context<Self>) {
        self.selected_cells.clear();
        self.selection_start_display = None;
        self.is_selecting = false;
        cx.notify();
    }
}

impl Focusable for CsvPreviewView {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<()> for CsvPreviewView {}

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
