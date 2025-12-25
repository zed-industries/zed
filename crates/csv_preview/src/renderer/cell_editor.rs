use gpui::{FocusHandle, Focusable};
use text::ToOffset as _;
use ui::{
    ActiveTheme, App, Context, InteractiveElement as _, IntoElement, ParentElement as _, Render,
    Styled as _, StyledTypography as _, Window, div, h_flex,
};

use crate::{
    CELL_EDITOR_CONTEXT_NAME, CsvPreviewView,
    cell_editor::{CellEditor, CellEditorCtx},
    types::DataCellId,
};

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

        let data_cid = self.engine.display_to_data_cell(cell_to_edit);
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
