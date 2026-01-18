use editor::Editor;
use gpui::{AppContext as _, Entity, FocusHandle, Focusable};
use text::Anchor;
use ui::{Context, Window};

use crate::table_data_engine::TableDataEngine;
use crate::types::{CellContentSpan, TableCell};
use crate::{
    CancelCellEditing, CsvPreviewView, FinishCellEditing, StartCellEditing,
    table_data_engine::copy_selected::EscapedCellString,
    types::{AnyColumn, DataCellId, DisplayCellId, TableRow},
};

pub(crate) struct CellEditor {
    pub cell_editor: Entity<Editor>,
    pub focus_handle: FocusHandle,
}

/// Context of cell being edited
pub(crate) enum EditedCellContext {
    Virtual {
        /// How many steps needed to get to neares real cell. Used to calculate how many empty cells to insert
        distance_to_real_cell: usize,
        /// End Anchor of real cell. Used as start point of the virtual cell insertion
        end_anchor_of_nearest_cell: Anchor,
    },
    Real {
        position: CellContentSpan,
    },
}

pub(crate) struct CellEditorCtx {
    pub editor: Entity<CellEditor>,
    pub cell_to_edit: DisplayCellId,
    pub cell_context: EditedCellContext,
}

impl TableDataEngine {
    pub(crate) fn display_to_data_cell(&self, display_cid: &DisplayCellId) -> DataCellId {
        self.d2d_mapping().display_to_data_cell(display_cid)
    }
}

impl CsvPreviewView {
    /// Commit the cell editor content back to the source buffer
    fn commit_cell_edit(&mut self, cx: &mut Context<Self>) {
        println!("Committing cell edit");
        let CellEditorCtx {
            editor,
            cell_context,
            ..
        } = self
            .cell_editor
            .as_ref()
            .expect("Expected to have cell editor present, when commiting cell changes");

        // Get the new text from the cell editor
        let new_text = editor.read(cx).cell_editor.read(cx).text(cx);
        const DELIMITER: char = ',';
        const DELIMITER_STR: &str = ","; // TODO: derive from char
        let new_text = EscapedCellString::new(new_text, DELIMITER);

        let (position, new_text) = match cell_context {
            EditedCellContext::Virtual {
                distance_to_real_cell,
                end_anchor_of_nearest_cell,
            } => {
                let bridge = DELIMITER_STR.repeat(*distance_to_real_cell);
                let cell_with_bridge = format!("{bridge}{}", new_text.take());
                (
                    CellContentSpan {
                        start: *end_anchor_of_nearest_cell,
                        end: *end_anchor_of_nearest_cell,
                    },
                    cell_with_bridge,
                )
            }
            EditedCellContext::Real { position } => (position.clone(), new_text.take()),
        };

        // let text_chunk
        // Apply the edit to the source buffer
        let Some(buffer) = self
            .editor_state()
            .editor
            .read(cx)
            .buffer()
            .read(cx)
            .as_singleton()
        else {
            return;
        };

        // Edit the source buffer
        buffer.update(cx, |buffer, cx| {
            let range = position.start..position.end;
            buffer.edit([(range, new_text)], None, cx);
        });

        self.cell_edited_flag = true;

        // The buffer edit will trigger a reparse via the subscription in parser.rs
        cx.notify();
    }
}

impl CsvPreviewView {
    pub(crate) fn start_cell_editing(
        &mut self,
        _: &StartCellEditing,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(focused_cell_id) = self.engine.selection.get_focused_cell() else {
            println!("No focused cell. Skip editing start");
            return;
        };

        let data_cid = self.engine.display_to_data_cell(&focused_cell_id);

        let row = self
            .engine
            .contents
            .get_row(data_cid.row)
            .expect("Expected mapped data cell id to point to existing data row");

        let (initial_content, cell_context) = match row.expect_get(data_cid.col) {
            TableCell::Real {
                cached_value,
                position,
            } => (
                cached_value.as_str(),
                EditedCellContext::Real {
                    position: position.clone(),
                },
            ),
            TableCell::Virtual => {
                let distance = distance_to_nearest_real_cell_left(row, data_cid.col);
                let last_real_cell = row.expect_get(AnyColumn(*data_cid.col - distance));

                (
                    "",
                    EditedCellContext::Virtual {
                        distance_to_real_cell: distance,
                        end_anchor_of_nearest_cell: last_real_cell
                            .position()
                            .expect("Expected last real cell to have position")
                            .end,
                    },
                )
            }
        };

        // Create the cell editor
        let editor = cx.new(|cx| {
            let mut editor = Editor::auto_height(1, 100, window, cx);
            editor.set_text(initial_content, window, cx);
            editor
        });

        // Focus the editor immediately after creation
        editor.read(cx).focus_handle(cx).focus(window, cx);

        self.cell_editor = Some(CellEditorCtx {
            editor: cx.new(|cx| CellEditor {
                cell_editor: editor,
                focus_handle: cx.focus_handle(),
            }),
            cell_to_edit: focused_cell_id,
            cell_context,
        });
        cx.notify();
    }

    pub(crate) fn finish_cell_editing(
        &mut self,
        _: &FinishCellEditing,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.commit_cell_edit(cx);
        self.clear_cell_editor();
        cx.notify();
    }

    pub(crate) fn cancel_cell_editing_handler(
        &mut self,
        _: &CancelCellEditing,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        println!("Cancel cell editing");
        self.clear_cell_editor();
        cx.notify();
    }
}

/// Calculates the distance from a given column to the nearest real cell to the left in the row.
///
/// Returns the number of columns between `from_col` (exclusive) and the nearest `TableCell::Real` (inclusive).
/// If there is no real cell to the left (including the current column), returns `from_col` as the distance.
///
/// # Arguments
/// * `row` - The table row to search within.
/// * `from_col` - The column index (as `AnyColumn`) to start searching left from.
///
/// # Example
/// ```
/// use crate::table_cell::{TableCell, distance_to_nearest_real_cell_left};
/// use crate::types::AnyColumn;
/// let row = vec![TableCell::Real { position: ..., cached_value: ... }, TableCell::Virtual, TableCell::Virtual, TableCell::Virtual];
/// let distance = distance_to_nearest_real_cell_left(&row, AnyColumn::new(3));
/// assert_eq!(distance, 2);
/// ```
pub fn distance_to_nearest_real_cell_left(row: &TableRow<TableCell>, from_col: AnyColumn) -> usize {
    let col = from_col.get();
    row.as_slice()
        .iter()
        .enumerate()
        .rev()
        .skip(row.as_slice().len().saturating_sub(col))
        .find(|(_, cell)| matches!(cell, TableCell::Real { .. }))
        .map(|(idx, _)| col - idx)
        .unwrap_or(col)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{AnyColumn, CellContentSpan, IntoTableRow, TableCell};
    use text::{Anchor, BufferId};
    use ui::SharedString;

    fn real() -> TableCell {
        TableCell::Real {
            position: CellContentSpan {
                start: Anchor::min_for_buffer(BufferId::new(1).unwrap()),
                end: Anchor::max_for_buffer(BufferId::new(1).unwrap()),
            },
            cached_value: SharedString::from("x"),
        }
    }

    #[test]
    fn test_distance_to_nearest_real_cell_left() {
        let row = vec![
            real(),             // ix 0
            TableCell::Virtual, // ix 1
            TableCell::Virtual, // ix 2
            TableCell::Virtual, // ix 3
        ]
        .into_table_row(4);
        assert_eq!(
            distance_to_nearest_real_cell_left(&row, AnyColumn::new(3)),
            3
        );
        assert_eq!(
            distance_to_nearest_real_cell_left(&row, AnyColumn::new(2)),
            2
        );
        assert_eq!(
            distance_to_nearest_real_cell_left(&row, AnyColumn::new(1)),
            1
        );
        assert_eq!(
            distance_to_nearest_real_cell_left(&row, AnyColumn::new(0)),
            0
        );
    }
}
