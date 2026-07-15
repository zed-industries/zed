use editor::Editor;
use gpui::App;
use language::{BufferSnapshot, Point};

pub struct CodePayload {
    pub text: String,
    pub advance_to: Option<Point>,
    pub language_name: Option<String>,
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
        language_name: None,
    })
}

pub fn get_eval_at_cursor(editor: &Editor, cx: &mut App) -> Option<CodePayload> {
    let multibuffer = editor.buffer().clone();
    let buffer = multibuffer.read(cx).as_singleton()?;
    let snapshot = buffer.read(cx).snapshot();

    let display_snapshot = editor.display_snapshot(cx);
    let selection = editor.selections.newest_adjusted(&display_snapshot);
    let cursor = selection.head();

    let Some(range) = crate::eval::find_eval_at(&snapshot, cursor) else {
        return get_blank_line(&snapshot, cursor);
    };

    let mut text: String = snapshot.text_for_range(range.start..range.end).collect();
    if text.trim().is_empty() {
        return None;
    }
    if is_python(&snapshot, cursor) {
        text = remove_blank_lines(text);
    }

    Some(CodePayload {
        text: ensure_eval_trailing_newlines(text),
        advance_to: next_row(&snapshot, range.end.row),
        language_name: language_name_at(&snapshot, cursor),
    })
}

fn get_blank_line(snapshot: &BufferSnapshot, cursor: Point) -> Option<CodePayload> {
    let line_end = Point::new(cursor.row, snapshot.line_len(cursor.row));
    let line_text: String = snapshot
        .text_for_range(Point::new(cursor.row, 0)..line_end)
        .collect();

    if !line_text.trim().is_empty() {
        return None;
    }

    Some(CodePayload {
        text: "\n".to_string(),
        advance_to: next_row(snapshot, cursor.row),
        language_name: language_name_at(snapshot, cursor),
    })
}

fn next_row(snapshot: &BufferSnapshot, row: u32) -> Option<Point> {
    let next_row = row + 1;
    if next_row <= snapshot.max_point().row {
        Some(Point::new(next_row, 0))
    } else {
        None
    }
}

fn ensure_trailing_newline(mut text: String) -> String {
    if !text.ends_with('\n') {
        text.push('\n');
    }
    text
}

fn ensure_eval_trailing_newlines(mut text: String) -> String {
    let trailing_newline_count = text.chars().rev().take_while(|ch| *ch == '\n').count();
    let required_newline_count = if text.contains('\n') { 2 } else { 1 };

    for _ in trailing_newline_count..required_newline_count {
        text.push('\n');
    }

    text
}

fn is_python(snapshot: &BufferSnapshot, cursor: Point) -> bool {
    language_name_at(snapshot, cursor).is_some_and(|language_name| language_name == "Python")
}

fn language_name_at(snapshot: &BufferSnapshot, cursor: Point) -> Option<String> {
    snapshot
        .language_at(cursor)
        .map(|language| language.config().name.to_string())
}

fn remove_blank_lines(text: String) -> String {
    text.lines()
        .filter(|line| !line.trim().is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{AppContext as _, TestAppContext};
    use language::Buffer;

    fn snapshot_for(text: &str, cx: &mut TestAppContext) -> BufferSnapshot {
        let buffer = cx.new(|cx| Buffer::local(text, cx));
        buffer.read_with(cx, |buffer, _| buffer.snapshot())
    }

    #[gpui::test]
    fn blank_line_payload_sends_return_and_advances(cx: &mut TestAppContext) {
        let snapshot = snapshot_for("one\n\nthree\n", cx);

        let payload = get_blank_line(&snapshot, Point::new(1, 0))
            .expect("expected blank line to send a return");

        assert_eq!(payload.text, "\n");
        assert_eq!(payload.advance_to, Some(Point::new(2, 0)));
    }

    #[gpui::test]
    fn whitespace_line_payload_sends_return_and_advances(cx: &mut TestAppContext) {
        let snapshot = snapshot_for("one\n  \nthree\n", cx);

        let payload = get_blank_line(&snapshot, Point::new(1, 1))
            .expect("expected whitespace-only line to send a return");

        assert_eq!(payload.text, "\n");
        assert_eq!(payload.advance_to, Some(Point::new(2, 0)));
    }

    #[gpui::test]
    fn non_blank_line_payload_is_none(cx: &mut TestAppContext) {
        let snapshot = snapshot_for("one\n# comment\nthree\n", cx);

        assert!(get_blank_line(&snapshot, Point::new(1, 0)).is_none());
    }

    #[test]
    fn eval_payload_single_line_has_one_trailing_newline() {
        assert_eq!(
            ensure_eval_trailing_newlines("x = 1".to_string()),
            "x = 1\n"
        );
    }

    #[test]
    fn eval_payload_multi_line_has_two_trailing_newlines() {
        assert_eq!(
            ensure_eval_trailing_newlines("if x > 3:\n    print(x)".to_string()),
            "if x > 3:\n    print(x)\n\n"
        );
    }

    #[test]
    fn python_eval_payload_removes_blank_lines_inside_class() {
        assert_eq!(
            remove_blank_lines(
                "class Counter:\n    def __init__(self):\n        self.value = 0\n\n    def increment(self):\n        self.value += 1".to_string()
            ),
            "class Counter:\n    def __init__(self):\n        self.value = 0\n    def increment(self):\n        self.value += 1"
        );
    }
}
