use editor::Editor;
use gpui::App;
use language::Point;

pub struct CodePayload {
    pub text: String,
    pub advance_to: Option<Point>,
}

pub fn get_selection(editor: &Editor, cx: &mut App) -> Option<CodePayload> {
    let multibuffer = editor.buffer().clone();
    let buffer = multibuffer.read(cx).as_singleton()?;
    let snapshot = buffer.read(cx).snapshot();

    let display_snapshot = editor.display_snapshot(cx);
    let selection = editor.selections.newest_adjusted(&display_snapshot);
    let range = selection.range();

    if range.start == range.end {
        return None;
    }

    let text: String = snapshot.text_for_range(range.start..range.end).collect();
    if text.trim().is_empty() {
        return None;
    }

    Some(CodePayload {
        text: ensure_trailing_newline(text),
        advance_to: None,
    })
}

pub fn get_eval_at_cursor(editor: &Editor, cx: &mut App) -> Option<CodePayload> {
    let multibuffer = editor.buffer().clone();
    let buffer = multibuffer.read(cx).as_singleton()?;
    let snapshot = buffer.read(cx).snapshot();

    let display_snapshot = editor.display_snapshot(cx);
    let selection = editor.selections.newest_adjusted(&display_snapshot);
    let cursor = selection.head();

    let range = crate::eval::find_eval_at(&snapshot, cursor)?;

    let text: String = snapshot.text_for_range(range.start..range.end).collect();
    if text.trim().is_empty() {
        return None;
    }

    let next_row = range.end.row + 1;
    let advance_to = if next_row <= snapshot.max_point().row {
        Some(Point::new(next_row, 0))
    } else {
        None
    };

    Some(CodePayload {
        text: ensure_trailing_newline(text),
        advance_to,
    })
}

fn ensure_trailing_newline(mut text: String) -> String {
    if !text.ends_with('\n') {
        text.push('\n');
    }
    text
}
