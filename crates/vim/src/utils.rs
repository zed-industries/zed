use editor::{ClipboardSelection, Editor};
use gpui::{AppContext, ClipboardItem};
use language::Point;

pub fn copy_selections_content(editor: &mut Editor, linewise: bool, cx: &mut AppContext) {
    let selections = editor.selections.all_adjusted(cx);
    let buffer = editor.buffer().read(cx).snapshot(cx);
    let mut text = String::new();
    let mut clipboard_selections = Vec::with_capacity(selections.len());
    {
        let mut is_first = true;
        for selection in selections.iter() {
            let mut start = selection.start;
            let end = selection.end;
            if is_first {
                is_first = false;
            } else {
                text.push_str("\n");
            }
            let initial_len = text.len();

            // if the file does not end with \n, and our line-mode selection ends on
            // that line, we will have expanded the start of the selection to ensure it
            // contains a newline (so that delete works as expected). We undo that change
            // here.
            let is_last_line = linewise
                && end.row == buffer.max_buffer_row()
                && buffer.max_point().column > 0
                && start == Point::new(start.row, buffer.line_len(start.row));

            if is_last_line {
                start = Point::new(buffer.max_buffer_row(), 0);
            }
            for chunk in buffer.text_for_range(start..end) {
                text.push_str(chunk);
            }
            if is_last_line {
                text.push_str("\n");
            }
            clipboard_selections.push(ClipboardSelection {
                len: text.len() - initial_len,
                is_entire_line: linewise,
                first_line_indent: buffer.indent_size_for_line(start.row).len,
            });
        }
    }

    cx.write_to_clipboard(ClipboardItem::new(text).with_metadata(clipboard_selections));
}
