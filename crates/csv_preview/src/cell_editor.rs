use editor::Editor;
use gpui::{AppContext as _, Entity};
use ui::{
    ActiveTheme as _, Context, IntoElement, ParentElement as _, Styled as _, StyledTypography as _,
    Window, div, h_flex,
};

use crate::CsvPreviewView;

impl CsvPreviewView {
    // dead code for now
    /// POC: Create a single-line editor
    pub(crate) fn create_cell_editor(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        // Check if we have enough rows and columns
        if self.contents.rows.len() < 2 {
            return;
        }
        if self.contents.rows[1].len() < 2 {
            return;
        }

        let cell = &self.contents.rows[1][1]; // Row 2, Col 2 (0-indexed)

        // Only proceed if we have buffer position tracking
        let Some(_position) = &cell.position else {
            return;
        };

        let Some(active_editor_state) = &self.active_editor else {
            return;
        };

        // Create a MultiBuffer with an excerpt for this cell
        let buffer_snapshot = active_editor_state
            .editor
            .read(cx)
            .buffer()
            .read(cx)
            .as_singleton();

        let Some(_buffer) = buffer_snapshot else {
            return;
        };

        let cell_editor = cx.new(|cx| {
            let mut editor = Editor::single_line(window, cx);
            editor.set_text(cell.display_value().as_ref(), window, cx);
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

        // Check if we have the target cell
        if self.contents.rows.len() < 2 || self.contents.rows[1].len() < 2 {
            return;
        }

        let cell = &self.contents.rows[1][1];
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

    /// POC: Render the single-line cell editor
    pub fn render_cell_editor(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        if self.cell_editor.is_none() {
            return div().child("No cell selected");
        }

        let theme = cx.theme();
        div()
            .p_2()
            .bg(theme.colors().editor_background)
            .border_1()
            .border_color(theme.colors().border)
            .size_full()
            .child(
                h_flex()
                    .size_full()
                    .gap_2()
                    .items_center()
                    .child(
                        div()
                            .text_ui(cx)
                            .text_color(theme.colors().text)
                            .child("Edit focused cell:"),
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
