use editor::Editor;
use gpui::{AppContext as _, Entity};
use ui::{
    ActiveTheme as _, Context, IntoElement, ParentElement as _, Styled as _, StyledTypography as _,
    Window, div, h_flex,
};

use crate::CsvPreviewView;

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

        // Subscribe to editor events to handle Enter key commits
        let subscription = cx.subscribe(
            &cell_editor,
            |this, _editor, event: &editor::EditorEvent, cx| {
                if let editor::EditorEvent::Edited { .. } = event {
                    this.commit_cell_edit(cx);
                }
            },
        );

        self.cell_editor = Some(cell_editor);
        self.cell_editor_subscription = Some(subscription);
    }

    /// POC: Commit the cell editor content back to the source buffer
    fn commit_cell_edit(&mut self, cx: &mut Context<Self>) {
        let Some(cell_editor) = &self.cell_editor else {
            return;
        };

        // Get the focused cell coordinates
        let Some((data_row, col)) = self.get_focused_cell() else {
            return;
        };

        // Check if we have the target cell
        if data_row >= self.contents.rows.len() || col >= self.contents.rows[data_row].len() {
            return;
        }

        let cell = &self.contents.rows[data_row][col];
        let Some(position) = &cell.position else {
            return;
        };

        let Some(active_editor_state) = &self.active_editor else {
            return;
        };

        // Get the new text from the cell editor
        let new_text = cell_editor.read(cx).text(cx);

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
    pub(crate) fn on_selection_changed(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        // Update cell editor content to match newly focused cell
        self.update_cell_editor_content(window, cx);
    }

    /// POC: Render the single-line cell editor
    pub fn render_cell_editor(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        if self.cell_editor.is_none() {
            return div().child("No cell editor available");
        }

        let theme = cx.theme();

        // Get focused cell info for display
        let focused_cell_info = if let Some((row, col)) = self.get_focused_cell() {
            format!("R{}C{}", row + 1, col + 1) // 1-based for display
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
