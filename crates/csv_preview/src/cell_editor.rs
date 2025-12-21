use editor::Editor;
use gpui::{AppContext as _, Entity, ScrollStrategy};
use menu::Confirm;
use text::ToOffset;
use ui::{
    ActiveTheme as _, Context, IntoElement, ParentElement as _, Styled as _, StyledTypography as _,
    Window, div, h_flex,
};

use crate::{CsvPreviewView, copy_selected::EscapedCellString};

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
    /// Get the currently focused cell data from the selection
    pub(crate) fn get_focused_cell(&self) -> Option<(usize, usize)> {
        if let Some(focused_cell) = self.selection.get_focused_cell() {
            // Convert display coordinates to data coordinates using ordered indices
            let display_row = focused_cell.row;
            let col = focused_cell.col;

            if let Some(data_row) = self.ordered_indices.get_data_row(display_row) {
                return Some((data_row.get(), col.get()));
            }
        }
        None
    }

    /// Get the content of the currently focused cell
    pub(crate) fn get_focused_cell_content(&self) -> Option<String> {
        if let Some((data_row, col)) = self.get_focused_cell() {
            // Get cell content from the table
            if data_row < self.contents.rows.len() && col < self.contents.rows[data_row].len() {
                let cell = &self.contents.rows[data_row][col];
                return Some(cell.display_value().to_string());
            }
        }
        None
    }

    /// Get the position of the currently focused cell
    pub(crate) fn get_focused_cell_position(&self) -> Option<&crate::table_cell::CellPosition> {
        if let Some((data_row, col)) = self.get_focused_cell() {
            // Get cell position from the table
            if data_row < self.contents.rows.len() && col < self.contents.rows[data_row].len() {
                let cell = &self.contents.rows[data_row][col];
                return cell.position.as_ref();
            }
        }
        None
    }

    /// Update the cell editor to display the focused cell's content
    pub(crate) fn update_cell_editor_content(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(cell_editor) = &self.cell_editor {
            if let Some(focused_content) = self.get_focused_cell_content() {
                cell_editor.update(cx, |editor, cx| {
                    editor.set_text(&*focused_content, window, cx);
                });
            } else {
                // No focused cell or content, clear the editor
                cell_editor.update(cx, |editor, cx| {
                    editor.set_text("", window, cx);
                });
            }
        }
    }

    /// POC: Create a single-line editor
    pub(crate) fn create_cell_editor(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        // Create the cell editor
        let cell_editor = cx.new(|cx| {
            let mut editor = Editor::single_line(window, cx);

            // Set initial content from focused cell
            let initial_content = self.get_focused_cell_content().unwrap_or_default();
            editor.set_text(&*initial_content, window, cx);
            editor
        });

        self.cell_editor = Some(cell_editor);
        self.cell_editor_subscription = None;
    }

    /// POC: Handle Enter key press in cell editor to commit changes
    pub(crate) fn handle_cell_editor_confirm(
        &mut self,
        _: &Confirm,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.commit_cell_edit(cx);
    }

    /// POC: Commit the cell editor content back to the source buffer
    fn commit_cell_edit(&mut self, cx: &mut Context<Self>) {
        println!("Committing cell edit");
        let Some(cell_editor) = &self.cell_editor else {
            println!("No cell editor found");
            return;
        };

        // Get the focused cell coordinates
        let Some((data_row, col)) = self.get_focused_cell() else {
            println!("No focused cell found");
            return;
        };

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
        let new_text = cell_editor.read(cx).text(cx);
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

        // The buffer edit will trigger a reparse via the subscription in parser.rs
        cx.notify();
    }

    /// POC: Get the cell editor for rendering
    pub(crate) fn get_cell_editor(&self) -> Option<&Entity<Editor>> {
        self.cell_editor.as_ref()
    }

    /// POC: Handle cell editor focus and content updates when selection changes
    /// apply_scroll - Whether to apply scroll. If applied, offset direction is specified
    pub(crate) fn on_selection_changed(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
        apply_scroll: Option<ScrollOffset>,
    ) {
        // Update cell editor content to match newly focused cell
        self.update_cell_editor_content(window, cx);

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
        if self.cell_editor.is_none() {
            return div().child("No cell editor available");
        }

        let theme = cx.theme();

        // Get focused cell info for display
        let focused_cell_info = if let Some((row, col)) = self.get_focused_cell() {
            if let Some(position) = self.get_focused_cell_position() {
                let Some(active_editor_state) = &self.active_editor else {
                    return div().child("No active editor");
                };

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
            }
        } else {
            "No cell focused".to_string()
        };

        div()
            .p_2()
            .bg(theme.colors().editor_background)
            .border_1()
            .border_color(theme.colors().border)
            .child(
                h_flex()
                    .gap_2()
                    .items_center()
                    .child(
                        div()
                            .text_ui(cx)
                            .text_color(theme.colors().text)
                            .child(format!("Edit cell {}:", focused_cell_info)),
                    )
                    .child({
                        if let Some(cell_editor) = self.get_cell_editor() {
                            div()
                                .flex_1()
                                .min_w_48()
                                .child(cell_editor.clone())
                                .into_any_element()
                        } else {
                            div()
                                .flex_1()
                                .min_w_48()
                                .p_2()
                                .bg(theme.colors().editor_subheader_background)
                                .text_ui(cx)
                                .text_color(theme.colors().text_muted)
                                .child("No cell editor available")
                                .into_any_element()
                        }
                    })
                    .child(
                        div()
                            .text_ui(cx)
                            .text_color(theme.colors().text_muted)
                            .child("(Press Enter to commit)"),
                    ),
            )
    }
}
