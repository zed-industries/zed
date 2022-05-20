use editor::{ClipboardSelection, Editor};
use gpui::{ClipboardItem, MutableAppContext};
use language::Point;

pub fn copy_selections_content(editor: &mut Editor, linewise: bool, cx: &mut MutableAppContext) {
    let selections = editor.selections.all::<Point>(cx);
    let buffer = editor.buffer().read(cx).snapshot(cx);
    let mut text = String::new();
    let mut clipboard_selections = Vec::with_capacity(selections.len());
    {
        for selection in selections.iter() {
            let initial_len = text.len();
            let start = selection.start;
            let end = selection.end;
            for chunk in buffer.text_for_range(start..end) {
                text.push_str(chunk);
            }
            clipboard_selections.push(ClipboardSelection {
                len: text.len() - initial_len,
                is_entire_line: linewise,
            });
        }
    }

    cx.write_to_clipboard(ClipboardItem::new(text).with_metadata(clipboard_selections));
}
