use editor::Editor;
use gpui::{AppContext as _, Entity, FocusHandle, Focusable, ScrollStrategy};
use text::ToOffset;
use ui::{
    ActiveTheme as _, App, Context, InteractiveElement, IntoElement, ParentElement as _, Render, Styled as _, StyledTypography as _, Window, div, h_flex,
};

use crate::{
    CELL_EDITOR_CONTEXT_NAME, CancelCellEditing, CsvPreviewView, FinishCellEditing,
    StartCellEditing,
    copy_selected::EscapedCellString,
    table_cell::CellContentSpan,
    types::{DataCellId, DisplayCellId},
};

pub(crate) struct CellEditorCtx {
    pub editor: Entity<CellEditor>,
    pub cell_to_edit: DisplayCellId,
}

/// Enum representing the offset of the focused cell in the list.
/// Used to keep viewport following the focused cell with some buffer, so mouse selection and autoscrollihng works correctly.
/// // TODO: rewrite this nonsence doc to be more clear
#[derive(Debug)]
pub enum ScrollOffset {
    NoOffset,
    Negative,
    Positive,
}

impl CsvPreviewView {
    /// Get the currently focused cell from the selection
    pub(crate) fn get_focused_data_cell(&self) -> Option<DataCellId> {
        let cid = self.selection.get_focused_cell()?;
        self.display_to_data_cell(&cid)
    }

    pub(crate) fn display_to_data_cell(&self, focused_cell: &DisplayCellId) -> Option<DataCellId> {
        let data_row = self.sorted_indices.get_data_row(focused_cell.row)?;
        Some(DataCellId::new(data_row, focused_cell.col))
    }

    /// Get the content of the currently focused cell
    pub(crate) fn get_focused_cell_content(&self) -> Option<String> {
        if let Some(cid) = self.get_focused_data_cell() {
            if let Some(value) = self.get_cell_content(cid) {
                return value;
            }
        }
        None
    }

    fn get_cell_content(&self, cid: DataCellId) -> Option<Option<String>> {
        let data_row = *cid.row;
        let col = *cid.col;
        // Get cell content from the table
        if data_row < self.contents.rows.len() && col < self.contents.rows[data_row].len() {
            let cell = &self.contents.rows[data_row][col];
            return Some(Some(cell.display_value().to_string()));
        }
        None
    }

    fn get_cell_content_span(&self, data_cell: DataCellId) -> Option<&CellContentSpan> {
        let (data_row, col) = (*data_cell.row, *data_cell.col);
        // Get cell position from the table
        if data_row < self.contents.rows.len() && col < self.contents.rows[data_row].len() {
            let cell = &self.contents.rows[data_row][col];
            return cell.position.as_ref();
        }
        None
    }

    /// POC: Commit the cell editor content back to the source buffer
    /// TODO: Refactor. It stinks
    fn commit_cell_edit(&mut self, cx: &mut Context<Self>) {
        println!("Committing cell edit");
        let Some(CellEditorCtx {
            editor,
            cell_to_edit,
        }) = &self.cell_editor
        else {
            println!("No cell editor found");
            return;
        };

        let (data_row, col) = cell_to_edit.to_raw();

        // Check if we have the target cell
        if data_row >= self.contents.rows.len() || col >= self.contents.rows[data_row].len() {
            println!("No target cell found");
            return;
        }

        let cell = &self.contents.rows[data_row][col];
        let Some(position) = &cell.position else {
            println!("Target cell has no position");
            return;
        };

        let Some(active_editor_state) = &self.active_editor else {
            println!("No active editor found to write changes to");
            return;
        };

        // Get the new text from the cell editor
        let new_text = editor.read(cx).cell_editor.read(cx).text(cx);
        const DELIMITER: char = ',';
        let new_text = EscapedCellString::new(new_text, DELIMITER);

        // Apply the edit to the source buffer
        let buffer_snapshot = active_editor_state
            .editor
            .read(cx)
            .buffer()
            .read(cx)
            .as_singleton();

        let Some(buffer) = buffer_snapshot else {
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

    // TODO: Move to `selection.rs`
    /// POC: Handle cell editor focus and content updates when selection changes
    /// apply_scroll - Whether to apply scroll. If applied, offset direction is specified
    pub(crate) fn on_selection_changed(
        &mut self,
        _window: &mut Window,
        cx: &mut Context<Self>,
        apply_scroll: Option<ScrollOffset>,
    ) {
        self.clear_cell_editor();

        // Follow focused cell in list viewport
        if let Some(focused_cell) = self.selection.get_focused_cell()
            && let Some(scroll) = apply_scroll
        {
            let display_row_index = focused_cell.row;
            let ix = display_row_index.0;

            match self.settings.rendering_with {
                crate::settings::RowRenderMechanism::VariableList => {
                    // Variable height list uses ListState::scroll_to_reveal_item
                    let ix_with_offset = match scroll {
                        ScrollOffset::NoOffset => ix,
                        ScrollOffset::Negative => ix.saturating_sub(2), // Avoid overflowing
                        ScrollOffset::Positive => ix + 2,
                    };
                    self.list_state.scroll_to_reveal_item(ix_with_offset);
                }
                crate::settings::RowRenderMechanism::UniformList => {
                    // Uniform list uses UniformListScrollHandle
                    let table_interaction_state = &self.table_interaction_state;
                    table_interaction_state.update(cx, |state, _| {
                        let ix_with_offset = match scroll {
                            ScrollOffset::NoOffset => ix,
                            ScrollOffset::Negative => ix.saturating_sub(2),
                            ScrollOffset::Positive => ix + 2,
                        };
                        // Use ScrollStrategy::Nearest for minimal scrolling (like scroll_to_reveal_item)
                        state.scroll_handle.scroll_to_item_with_offset(
                            ix_with_offset,
                            ScrollStrategy::Nearest,
                            0, // No additional offset since we already calculate it above
                        );
                    });
                }
            }
        }
    }

    /// POC: Render the single-line cell editor
    pub fn render_cell_editor(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        let Some(active_editor_state) = &self.active_editor else {
            return div().child("WARN: No active editor attached to preview pane");
        };

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

        let (row, col) = data_cid.to_raw();
        let theme = cx.theme().clone();
        // Get focused cell info for display
        let edited_cell_info = if let Some(position) = self.get_cell_content_span(data_cid) {
            let buffer_snapshot = active_editor_state
                .editor
                .read(cx)
                .buffer()
                .read(cx)
                .as_singleton();

            let Some(buffer) = buffer_snapshot else {
                return div().child("No buffer available");
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

    fn clear_cell_editor(&mut self) {
        self.cell_editor = None;
    }
}

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

impl CsvPreviewView {
    pub(crate) fn start_cell_editing(
        &mut self,
        _: &StartCellEditing,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(focused_cell_id) = self.selection.get_focused_cell() else {
            println!("No focused cell id. Skip editing start");
            return;
        };

        let Some(initial_content) = self.get_focused_cell_content() else {
            println!("No focused cell. Skip editing start");
            return;
        };

        // Create the cell editor
        let editor = cx.new(|cx| {
            let mut editor = Editor::auto_height(1, 100, window, cx);
            editor.set_text(&*initial_content, window, cx);
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
