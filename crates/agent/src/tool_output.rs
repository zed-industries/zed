use language_model::LanguageModelToolResultContent;
use std::fmt::Write as _;
use util::size::format_file_size;

/// Maximum number of lines a single tool result may occupy in the model's context.
pub const MAX_TOOL_OUTPUT_LINES: usize = 2000;

/// Maximum number of bytes a single tool result may occupy in the model's context.
pub const MAX_TOOL_OUTPUT_BYTES: usize = 50 * 1024;

/// Caps the text parts of a tool result to a shared line and byte budget so a
/// single tool call can't flood the model's context. Oversized parts are
/// middle-truncated: the head and tail are preserved and an inline notice
/// replaces the elided middle. Images pass through untouched.
///
/// Only the content sent to the model is affected; the raw output persisted
/// for the UI stays complete.
pub fn truncate_tool_result(content: &mut [LanguageModelToolResultContent]) {
    let mut remaining_lines = MAX_TOOL_OUTPUT_LINES;
    let mut remaining_bytes = MAX_TOOL_OUTPUT_BYTES;
    for part in content {
        let LanguageModelToolResultContent::Text(text) = part else {
            continue;
        };
        let line_count = text.lines().count();
        if line_count <= remaining_lines && text.len() <= remaining_bytes {
            remaining_lines -= line_count;
            remaining_bytes -= text.len();
        } else {
            let truncated = truncate_middle(text, remaining_lines, remaining_bytes);
            *part = truncated.into();
            remaining_lines = 0;
            remaining_bytes = 0;
        }
    }
}

/// Caps a display line to `max_chars` characters, returning the possibly
/// shortened line and whether it was truncated.
pub fn truncate_line(line: &str, max_chars: usize) -> (&str, bool) {
    match line.char_indices().nth(max_chars) {
        Some((ix, _)) => (&line[..ix], true),
        None => (line, false),
    }
}

fn truncate_middle(text: &str, max_lines: usize, max_bytes: usize) -> String {
    let head_end = head_boundary(text, max_lines / 2, max_bytes / 2);
    let tail_start =
        tail_boundary(text, max_lines.div_ceil(2), max_bytes.div_ceil(2)).max(head_end);

    let total_lines = text.lines().count();
    let omitted_lines = text[head_end..tail_start].lines().count();
    let omitted_bytes = (tail_start - head_end) as u64;

    let mut result = String::with_capacity(head_end + (text.len() - tail_start) + 256);
    result.push_str(&text[..head_end]);
    if !result.is_empty() && !result.ends_with('\n') {
        result.push('\n');
    }
    let _ = writeln!(
        result,
        "[... omitted {omitted_lines} of {total_lines} lines ({}) from tool output. \
         Re-run the tool with narrower parameters (e.g. offset/limit, start_line/end_line, \
         or head_lines/tail_lines) to see more ...]",
        format_file_size(omitted_bytes, true),
    );
    result.push_str(&text[tail_start..]);
    result
}

/// Byte offset after the last whole line that fits in the budgets, or a
/// mid-line cut when the first line alone exceeds `max_bytes`.
fn head_boundary(text: &str, max_lines: usize, max_bytes: usize) -> usize {
    let mut end = 0;
    for line in text.split_inclusive('\n').take(max_lines) {
        if end + line.len() > max_bytes {
            break;
        }
        end += line.len();
    }
    if end == 0 && max_lines > 0 {
        floor_char_boundary(text, max_bytes)
    } else {
        end
    }
}

/// Byte offset where the tail begins: at most `max_lines` of the final
/// `max_bytes` of `text`. May start mid-line when a single line exceeds the
/// byte budget.
fn tail_boundary(text: &str, max_lines: usize, max_bytes: usize) -> usize {
    if max_lines == 0 || max_bytes == 0 {
        return text.len();
    }
    let window_start = ceil_char_boundary(text, text.len().saturating_sub(max_bytes));
    let mut line_starts = Vec::new();
    let mut offset = window_start;
    for line in text[window_start..].split_inclusive('\n') {
        line_starts.push(offset);
        offset += line.len();
    }
    let keep = line_starts.len().min(max_lines);
    line_starts
        .get(line_starts.len() - keep)
        .copied()
        .unwrap_or(text.len())
}

fn floor_char_boundary(text: &str, mut index: usize) -> usize {
    while !text.is_char_boundary(index) {
        index -= 1;
    }
    index
}

fn ceil_char_boundary(text: &str, mut index: usize) -> usize {
    while !text.is_char_boundary(index) {
        index += 1;
    }
    index
}

#[cfg(test)]
mod tests {
    use super::*;

    fn text_part(text: &str) -> LanguageModelToolResultContent {
        LanguageModelToolResultContent::Text(text.into())
    }

    fn text(part: &LanguageModelToolResultContent) -> &str {
        match part {
            LanguageModelToolResultContent::Text(text) => text,
            LanguageModelToolResultContent::Image(_) => panic!("expected text part"),
        }
    }

    #[test]
    fn test_output_within_limits_is_unchanged() {
        let mut content = vec![text_part("hello\nworld")];
        truncate_tool_result(&mut content);
        assert_eq!(text(&content[0]), "hello\nworld");
    }

    #[test]
    fn test_line_limit_preserves_head_and_tail() {
        let input = (0..MAX_TOOL_OUTPUT_LINES * 2)
            .map(|i| format!("line {i}"))
            .collect::<Vec<_>>()
            .join("\n");
        let mut content = vec![text_part(&input)];
        truncate_tool_result(&mut content);
        let result = text(&content[0]);
        assert!(result.starts_with("line 0\n"));
        assert!(result.ends_with(&format!("line {}", MAX_TOOL_OUTPUT_LINES * 2 - 1)));
        assert!(result.contains("omitted"));
        assert!(result.lines().count() <= MAX_TOOL_OUTPUT_LINES + 1);
    }

    #[test]
    fn test_byte_limit_preserves_head_and_tail() {
        let line = "x".repeat(1024);
        let input = vec![line.clone(); 100].join("\n");
        let mut content = vec![text_part(&input)];
        truncate_tool_result(&mut content);
        let result = text(&content[0]);
        assert!(result.starts_with(&line));
        assert!(result.ends_with(&line));
        assert!(result.contains("omitted"));
        assert!(result.len() <= MAX_TOOL_OUTPUT_BYTES + 256);
    }

    #[test]
    fn test_single_line_exceeding_byte_limit() {
        let input = "y".repeat(MAX_TOOL_OUTPUT_BYTES * 4);
        let mut content = vec![text_part(&input)];
        truncate_tool_result(&mut content);
        let result = text(&content[0]);
        assert!(result.starts_with("yyy"));
        assert!(result.ends_with("yyy"));
        assert!(result.contains("omitted"));
        assert!(result.len() <= MAX_TOOL_OUTPUT_BYTES + 256);
    }

    #[test]
    fn test_budget_is_shared_across_parts() {
        let big = "line\n".repeat(MAX_TOOL_OUTPUT_LINES);
        let mut content = vec![text_part(&big), text_part("hello\nworld")];
        truncate_tool_result(&mut content);
        assert_eq!(text(&content[0]), big);
        assert!(text(&content[1]).contains("omitted 2 of 2 lines"));
    }

    #[test]
    fn test_truncate_line() {
        assert_eq!(truncate_line("hello", 10), ("hello", false));
        assert_eq!(truncate_line("hello", 5), ("hello", false));
        assert_eq!(truncate_line("hello", 4), ("hell", true));
        assert_eq!(truncate_line("èèèèè", 4), ("èèèè", true));
    }
}
