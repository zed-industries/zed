use std::time::Duration;

use editor::{ClipboardSelection, Editor};
use gpui::{ClipboardItem, ViewContext};
use language::{CharKind, Point};
use settings::Settings;

use crate::{state::Mode, UseSystemClipboard, Vim, VimSettings};

pub struct HighlightOnYank;

pub fn yank_selections_content(
    vim: &mut Vim,
    editor: &mut Editor,
    linewise: bool,
    cx: &mut ViewContext<Editor>,
) {
    copy_selections_content_internal(vim, editor, linewise, true, cx);
}

pub fn copy_selections_content(
    vim: &mut Vim,
    editor: &mut Editor,
    linewise: bool,
    cx: &mut ViewContext<Editor>,
) {
    copy_selections_content_internal(vim, editor, linewise, false, cx);
}

fn copy_selections_content_internal(
    vim: &mut Vim,
    editor: &mut Editor,
    linewise: bool,
    is_yank: bool,
    cx: &mut ViewContext<Editor>,
) {
    let selections = editor.selections.all_adjusted(cx);
    let buffer = editor.buffer().read(cx).snapshot(cx);
    let mut text = String::new();
    let mut clipboard_selections = Vec::with_capacity(selections.len());
    let mut ranges_to_highlight = Vec::new();
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
                && start.row < buffer.max_buffer_row()
                && start == Point::new(start.row, buffer.line_len(start.row));

            if is_last_line {
                start = Point::new(start.row + 1, 0);
            }

            let start_anchor = buffer.anchor_after(start);
            let end_anchor = buffer.anchor_before(end);
            ranges_to_highlight.push(start_anchor..end_anchor);

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

    let setting = VimSettings::get_global(cx).use_system_clipboard;
    if setting == UseSystemClipboard::Always || setting == UseSystemClipboard::OnYank && is_yank {
        cx.write_to_clipboard(ClipboardItem::new(text.clone()).with_metadata(clipboard_selections));
        vim.workspace_state
            .registers
            .insert(".system.".to_string(), text.clone());
    } else {
        vim.workspace_state.registers.insert(
            ".system.".to_string(),
            cx.read_from_clipboard()
                .map(|item| item.text().clone())
                .unwrap_or_default(),
        );
    }
    vim.workspace_state.registers.insert("\"".to_string(), text);
    if !is_yank || vim.state().mode == Mode::Visual {
        return;
    }

    editor.highlight_background::<HighlightOnYank>(
        ranges_to_highlight,
        |colors| colors.editor_document_highlight_read_background,
        cx,
    );
    cx.spawn(|this, mut cx| async move {
        cx.background_executor()
            .timer(Duration::from_millis(200))
            .await;
        this.update(&mut cx, |editor, cx| {
            editor.clear_background_highlights::<HighlightOnYank>(cx)
        })
        .ok();
    })
    .detach();
}

pub fn coerce_punctuation(kind: CharKind, treat_punctuation_as_word: bool) -> CharKind {
    if treat_punctuation_as_word && kind == CharKind::Punctuation {
        CharKind::Word
    } else {
        kind
    }
}
