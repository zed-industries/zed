use editor::{ClipboardSelection, Editor};
use gpui::{AppContext, ClipboardItem};

pub fn copy_selections_content(editor: &mut Editor, linewise: bool, cx: &mut AppContext) {
    let selections = editor.selections.all_adjusted(cx);
    let buffer = editor.buffer().read(cx).snapshot(cx);
    let mut text = String::new();
    let mut clipboard_selections = Vec::with_capacity(selections.len());
    {
        let mut is_first = true;
        for selection in selections.iter() {
            let start = selection.start;
            let end = selection.end;
            if is_first {
                is_first = false;
            } else {
                text.push_str("\n");
            }
            let initial_len = text.len();
            for chunk in buffer.text_for_range(start..end) {
                text.push_str(chunk);
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
