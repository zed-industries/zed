use gpui::ClipboardItem;
use ui::{Context, Window};
use workspace::{Toast, Workspace, notifications::NotificationId};

use crate::{
    CopySelected, CsvPreviewView, performance_metrics_overlay::TimingRecorder,
    settings::CopyFormat, table_data_engine::copy_selected::ToastInfo,
};

impl CsvPreviewView {
    pub(crate) fn copy_selected(
        &mut self,
        _: &CopySelected,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let copy_format = self.settings.copy_format;
        let copy_mode = self.settings.copy_mode;

        let maybe_copied = self
            .performance_metrics
            .last_copy_took
            .record_timing(|| self.engine.try_copy_selected(copy_format, copy_mode));

        let (toast_info, content) = match maybe_copied {
            Some(value) => value,
            None => return,
        };

        cx.write_to_clipboard(ClipboardItem::new_string(content));

        // Show toast notification
        if let Some(Some(workspace)) = window.root() {
            show_toast_with_copy_results(cx, copy_format, toast_info, workspace);
        }
    }
}

fn show_toast_with_copy_results(
    cx: &mut Context<'_, CsvPreviewView>,
    copy_format: CopyFormat,
    toast_info: ToastInfo,
    workspace: gpui::Entity<Workspace>,
) {
    let format_name = match copy_format {
        CopyFormat::Tsv => "TSV",
        CopyFormat::Csv => "CSV",
        CopyFormat::Semicolon => "Semicolon",
        CopyFormat::Markdown => "Markdown",
    };

    let (rows, cols) = toast_info.rectangle_dimensions;
    let message = if toast_info.selected_cell_count == 1 {
        format!("1 cell copied as {}", format_name)
    } else if toast_info.empty_cells_count == 0 {
        format!(
            "{} cells copied as {} ({}×{})",
            toast_info.selected_cell_count, format_name, rows, cols
        )
    } else {
        format!(
            "{} cells copied as {} ({}×{}, {} empty)",
            toast_info.selected_cell_count, format_name, rows, cols, toast_info.empty_cells_count
        )
    };

    workspace.update(cx, |workspace: &mut Workspace, cx| {
        struct CsvCopyToast;
        workspace.show_toast(
            Toast::new(NotificationId::unique::<CsvCopyToast>(), message).autohide(),
            cx,
        );
    });
}
