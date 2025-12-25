use editor::Editor;
use gpui::{AppContext as _, Entity, FocusHandle, Focusable};
use text::{Anchor, ToOffset};
use ui::{
    ActiveTheme as _, App, Context, InteractiveElement, IntoElement, ParentElement as _, Render,
    Styled as _, StyledTypography as _, Window, div, h_flex,
};

use crate::types::{CellContentSpan, TableCell};
use crate::{
    CELL_EDITOR_CONTEXT_NAME, CancelCellEditing, CsvPreviewView, FinishCellEditing,
    StartCellEditing,
    table_data_engine::copy_selected::EscapedCellString,
    types::{AnyColumn, DataCellId, DisplayCellId, TableRow},
};

pub(crate) struct CellEditor {
    pub cell_editor: Entity<Editor>,
    pub focus_handle: FocusHandle,
}

impl Focusable for CellEditor {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for CellEditor {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let cell_editor = self.cell_editor.clone();
        let theme = cx.theme();

        div()
            .track_focus(&self.focus_handle)
            .flex_1()
            .min_w_48()
            .key_context(CELL_EDITOR_CONTEXT_NAME)
            .bg(theme.colors().editor_background)
            .child(cell_editor)
            .into_any_element()
    }
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

impl CsvPreviewView {
    pub(crate) fn display_to_data_cell(&self, focused_cell: &DisplayCellId) -> Option<DataCellId> {
        let data_row = self.engine.d2d_mapping.get_data_row(focused_cell.row)?;
        Some(DataCellId::new(data_row, focused_cell.col))
    }

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

        let data_cid = self
            .display_to_data_cell(&focused_cell_id)
            .expect("Expected focused cell to point to existing data cell id");

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
        editor.read(cx).focus_handle(cx).focus(window);

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

impl CsvPreviewView {
    /// POC: Render the single-line cell editor
    pub fn render_cell_editor(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        let Some(CellEditorCtx {
            editor,
            cell_to_edit,
            ..
        }) = &self.cell_editor
        else {
            return div().child("Not editing cell. Select cell and press enter to start editing");
        };

        let Some(data_cid) = self.display_to_data_cell(cell_to_edit) else {
            return div().child("ERROR: Can't find data cell by display cell");
        };

        let edited_cell_info = match self.calculate_cell_info(cx, self.editor_state(), data_cid) {
            Some(v) => v,
            None => return div().child("No buffer available"),
        };

        let theme = cx.theme().clone();
        div()
            // .track_focus(&self.focus_handle)
            .p_2()
            .bg(theme.colors().panel_background)
            .border_1()
            .border_color(theme.colors().border)
            .child(
                h_flex()
                    .items_stretch()
                    .gap_2()
                    .items_start()
                    .child(
                        div()
                            .text_ui(cx)
                            .text_color(theme.colors().text)
                            .child(format!("Editing cell {edited_cell_info}:")),
                    )
                    .child(editor.clone())
                    .child(
                        div()
                            .text_ui(cx)
                            .text_color(theme.colors().text_muted)
                            .child("(Press Enter to commit)"),
                    ),
            )
    }

    fn calculate_cell_info(
        &self,
        cx: &mut Context<'_, CsvPreviewView>,
        active_editor_state: &crate::parser::EditorState,
        data_cid: DataCellId,
    ) -> Option<String> {
        let (row, col) = data_cid.to_raw();
        let edited_cell_info = if let Some(position) = self
            .engine
            .contents
            .get_cell(&data_cid)
            .and_then(|tc| tc.position())
        {
            let buffer_snapshot = active_editor_state
                .editor
                .read(cx)
                .buffer()
                .read(cx)
                .as_singleton();

            let Some(buffer) = buffer_snapshot else {
                return None;
            };

            let buffer = buffer.read(cx);
            let start_offset = position.start.to_offset(&buffer);
            let end_offset = position.end.to_offset(&buffer);

            format!(
                "R{}C{} at {}..{}",
                row + 1,
                col + 1,
                start_offset,
                end_offset
            )
        } else {
            format!("R{}C{}", row + 1, col + 1) // 1-based for display
        };
        Some(edited_cell_info)
    }

    pub fn clear_cell_editor(&mut self) {
        self.cell_editor = None;
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
    use crate::table_cell::{CellContentSpan, TableCell};
    use crate::types::{AnyColumn, IntoTableRow};
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
