use editor::Editor;
use gpui::{AppContext as _, Entity, FocusHandle, Focusable};
use text::ToOffset;
use ui::{
    ActiveTheme as _, App, Context, InteractiveElement, IntoElement, ParentElement as _, Render,
    SharedString, Styled as _, StyledTypography as _, Window, div, h_flex,
};

use crate::{
    CELL_EDITOR_CONTEXT_NAME, CancelCellEditing, CsvPreviewView, FinishCellEditing,
    StartCellEditing,
    copy_selected::EscapedCellString,
    table_cell::TableCell,
    types::{DataCellId, DisplayCellId},
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

pub(crate) struct CellEditorCtx {
    pub editor: Entity<CellEditor>,
    pub cell_to_edit: DisplayCellId,
}

impl CsvPreviewView {
    pub(crate) fn display_to_data_cell(&self, focused_cell: &DisplayCellId) -> Option<DataCellId> {
        let data_row = self.sorted_indices.get_data_row(focused_cell.row)?;
        Some(DataCellId::new(data_row, focused_cell.col))
    }

    fn get_cell_content(&self, cid: DataCellId) -> Option<SharedString> {
        self.contents.get_cell(&cid)?.display_value().cloned()
    }

    /// POC: Commit the cell editor content back to the source buffer
    /// TODO: Refactor. It stinks
    fn commit_cell_edit(&mut self, cx: &mut Context<Self>) {
        println!("Committing cell edit");
        let CellEditorCtx {
            editor,
            cell_to_edit,
        } = self
            .cell_editor
            .as_ref()
            .expect("Expected to have cell editor present, when commiting cell changes");

        let data_cell_id = self
            .display_to_data_cell(cell_to_edit)
            .expect("Cell editor bug: Expected to map valid display cell id to data cell id");

        let cell = self
            .contents
            .get_cell(&data_cell_id)
            .expect("Cell editor bug: Expected to get cell by data cell id");

        let position = match cell {
            TableCell::Real { position, .. } => position,
            TableCell::Virtual => {
                println!("Virtual cell editing is not yet supported");
                return;
            }
        };

        // Get the new text from the cell editor
        let new_text = editor.read(cx).cell_editor.read(cx).text(cx);
        const DELIMITER: char = ',';
        let new_text = EscapedCellString::new(new_text, DELIMITER);

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
        let Some(focused_cell_id) = self.selection.get_focused_cell() else {
            println!("No focused cell. Skip editing start");
            return;
        };

        let data_cid = self
            .display_to_data_cell(&focused_cell_id)
            .expect("Expected focused cell to point to existing data cell id");

        let cell = self
            .contents
            .get_cell(&data_cid)
            .expect("Expected mapped data cell id to point to existing cell");

        let initial_content = match cell {
            TableCell::Real { cached_value, .. } => cached_value,
            TableCell::Virtual => {
                println!("Editing of virtual cell is not supported yet");
                return;
            }
        };

        // Create the cell editor
        let editor = cx.new(|cx| {
            let mut editor = Editor::auto_height(1, 100, window, cx);
            editor.set_text(initial_content.as_ref(), window, cx);
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
