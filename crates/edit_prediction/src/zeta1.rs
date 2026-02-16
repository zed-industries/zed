use std::{fmt::Write, ops::Range, sync::Arc};

use crate::cursor_excerpt::{editable_and_context_ranges_for_cursor_position, guess_token_count};
use anyhow::Result;
use cloud_llm_client::PredictEditsBody;
use edit_prediction_types::PredictedCursorPosition;
use language::{Anchor, BufferSnapshot, Point, text_diff};
use text::Bias;
use zeta_prompt::{
    Event,
    zeta1::{
        CURSOR_MARKER, EDITABLE_REGION_END_MARKER, EDITABLE_REGION_START_MARKER,
        START_OF_FILE_MARKER,
    },
};

pub(crate) const MAX_CONTEXT_TOKENS: usize = 150;
pub(crate) const MAX_EVENT_TOKENS: usize = 500;

pub(crate) fn parse_edits(
    output_excerpt: &str,
    editable_range: Range<usize>,
    snapshot: &BufferSnapshot,
) -> Result<Vec<(Range<Anchor>, Arc<str>)>> {
    let content = output_excerpt.replace(CURSOR_MARKER, "");

    let start_markers = content
        .match_indices(EDITABLE_REGION_START_MARKER)
        .collect::<Vec<_>>();
    anyhow::ensure!(
        start_markers.len() <= 1,
        "expected at most one start marker, found {}",
        start_markers.len()
    );

    let end_markers = content
        .match_indices(EDITABLE_REGION_END_MARKER)
        .collect::<Vec<_>>();
    anyhow::ensure!(
        end_markers.len() <= 1,
        "expected at most one end marker, found {}",
        end_markers.len()
    );

    let sof_markers = content
        .match_indices(START_OF_FILE_MARKER)
        .collect::<Vec<_>>();
    anyhow::ensure!(
        sof_markers.len() <= 1,
        "expected at most one start-of-file marker, found {}",
        sof_markers.len()
    );

    let content_start = start_markers
        .first()
        .map(|e| e.0 + EDITABLE_REGION_START_MARKER.len())
        .map(|start| {
            if content.len() > start
                && content.is_char_boundary(start)
                && content[start..].starts_with('\n')
            {
                start + 1
            } else {
                start
            }
        })
        .unwrap_or(0);
    let content_end = end_markers
        .first()
        .map(|e| {
            if e.0 > 0 && content.is_char_boundary(e.0 - 1) && content[e.0 - 1..].starts_with('\n')
            {
                e.0 - 1
            } else {
                e.0
            }
        })
        .unwrap_or(content.strip_suffix("\n").unwrap_or(&content).len());

    // min to account for content_end and content_start both accounting for the same newline in the following case:
    // <|editable_region_start|>\n<|editable_region_end|>
    let new_text = &content[content_start.min(content_end)..content_end];

    let old_text = snapshot
        .text_for_range(editable_range.clone())
        .collect::<String>();

    Ok(compute_edits(
        old_text,
        new_text,
        editable_range.start,
        snapshot,
    ))
}

pub fn compute_edits(
    old_text: String,
    new_text: &str,
    offset: usize,
    snapshot: &BufferSnapshot,
) -> Vec<(Range<Anchor>, Arc<str>)> {
    compute_edits_and_cursor_position(old_text, new_text, offset, None, snapshot).0
}

pub fn compute_edits_and_cursor_position(
    old_text: String,
    new_text: &str,
    offset: usize,
    cursor_offset_in_new_text: Option<usize>,
    snapshot: &BufferSnapshot,
) -> (
    Vec<(Range<Anchor>, Arc<str>)>,
    Option<PredictedCursorPosition>,
) {
    let diffs = text_diff(&old_text, new_text);

    // Delta represents the cumulative change in byte count from all preceding edits.
    // new_offset = old_offset + delta, so old_offset = new_offset - delta
    let mut delta: isize = 0;
    let mut cursor_position: Option<PredictedCursorPosition> = None;
    let buffer_len = snapshot.len();

    let edits = diffs
        .iter()
        .map(|(raw_old_range, new_text)| {
            // Compute cursor position if it falls within or before this edit.
            if let (Some(cursor_offset), None) = (cursor_offset_in_new_text, cursor_position) {
                let edit_start_in_new = (raw_old_range.start as isize + delta) as usize;
                let edit_end_in_new = edit_start_in_new + new_text.len();

                if cursor_offset < edit_start_in_new {
                    let cursor_in_old = (cursor_offset as isize - delta) as usize;
                    let buffer_offset = (offset + cursor_in_old).min(buffer_len);
                    cursor_position = Some(PredictedCursorPosition::at_anchor(
                        snapshot.anchor_after(buffer_offset),
                    ));
                } else if cursor_offset < edit_end_in_new {
                    let buffer_offset = (offset + raw_old_range.start).min(buffer_len);
                    let offset_within_insertion = cursor_offset - edit_start_in_new;
                    cursor_position = Some(PredictedCursorPosition::new(
                        snapshot.anchor_before(buffer_offset),
                        offset_within_insertion,
                    ));
                }

                delta += new_text.len() as isize - raw_old_range.len() as isize;
            }

            // Compute the edit with prefix/suffix trimming.
            let mut old_range = raw_old_range.clone();
            let old_slice = &old_text[old_range.clone()];

            let prefix_len = common_prefix(old_slice.chars(), new_text.chars());
            let suffix_len = common_prefix(
                old_slice[prefix_len..].chars().rev(),
                new_text[prefix_len..].chars().rev(),
            );

            old_range.start += offset;
            old_range.end += offset;
            old_range.start += prefix_len;
            old_range.end -= suffix_len;

            old_range.start = old_range.start.min(buffer_len);
            old_range.end = old_range.end.min(buffer_len);

            let new_text = new_text[prefix_len..new_text.len() - suffix_len].into();
            let range = if old_range.is_empty() {
                let anchor = snapshot.anchor_after(old_range.start);
                anchor..anchor
            } else {
                snapshot.anchor_after(old_range.start)..snapshot.anchor_before(old_range.end)
            };
            (range, new_text)
        })
        .collect();

    if let (Some(cursor_offset), None) = (cursor_offset_in_new_text, cursor_position) {
        let cursor_in_old = (cursor_offset as isize - delta) as usize;
        let buffer_offset = snapshot.clip_offset(offset + cursor_in_old, Bias::Right);
        cursor_position = Some(PredictedCursorPosition::at_anchor(
            snapshot.anchor_after(buffer_offset),
        ));
    }

    (edits, cursor_position)
}

fn common_prefix<T1: Iterator<Item = char>, T2: Iterator<Item = char>>(a: T1, b: T2) -> usize {
    a.zip(b)
        .take_while(|(a, b)| a == b)
        .map(|(a, _)| a.len_utf8())
        .sum()
}

pub struct GatherContextOutput {
    pub body: PredictEditsBody,
    pub context_range: Range<Point>,
    pub editable_range: Range<usize>,
    pub included_events_count: usize,
}

pub(crate) fn prompt_for_events(events: &[Arc<Event>], max_tokens: usize) -> String {
    prompt_for_events_impl(events, max_tokens).0
}

fn prompt_for_events_impl(events: &[Arc<Event>], mut remaining_tokens: usize) -> (String, usize) {
    let mut result = String::new();
    for (ix, event) in events.iter().rev().enumerate() {
        let event_string = format_event(event.as_ref());
        let event_tokens = guess_token_count(event_string.len());
        if event_tokens > remaining_tokens {
            return (result, ix);
        }

        if !result.is_empty() {
            result.insert_str(0, "\n\n");
        }
        result.insert_str(0, &event_string);
        remaining_tokens -= event_tokens;
    }
    return (result, events.len());
}

pub fn format_event(event: &Event) -> String {
    match event {
        Event::BufferChange {
            path,
            old_path,
            diff,
            ..
        } => {
            let mut prompt = String::new();

            if old_path != path {
                writeln!(
                    prompt,
                    "User renamed {} to {}\n",
                    old_path.display(),
                    path.display()
                )
                .unwrap();
            }

            if !diff.is_empty() {
                write!(
                    prompt,
                    "User edited {}:\n```diff\n{}\n```",
                    path.display(),
                    diff
                )
                .unwrap();
            }

            prompt
        }
    }
}

#[derive(Debug)]
pub struct InputExcerpt {
    pub context_range: Range<Point>,
    pub editable_range: Range<Point>,
    pub prompt: String,
}

pub fn excerpt_for_cursor_position(
    position: Point,
    path: &str,
    snapshot: &BufferSnapshot,
    editable_region_token_limit: usize,
    context_token_limit: usize,
) -> InputExcerpt {
    let (editable_range, context_range) = editable_and_context_ranges_for_cursor_position(
        position,
        snapshot,
        editable_region_token_limit,
        context_token_limit,
    );

    let mut prompt = String::new();

    writeln!(&mut prompt, "```{path}").unwrap();
    if context_range.start == Point::zero() {
        writeln!(&mut prompt, "{START_OF_FILE_MARKER}").unwrap();
    }

    for chunk in snapshot.chunks(context_range.start..editable_range.start, false) {
        prompt.push_str(chunk.text);
    }

    push_editable_range(position, snapshot, editable_range.clone(), &mut prompt);

    for chunk in snapshot.chunks(editable_range.end..context_range.end, false) {
        prompt.push_str(chunk.text);
    }
    write!(prompt, "\n```").unwrap();

    InputExcerpt {
        context_range,
        editable_range,
        prompt,
    }
}

fn push_editable_range(
    cursor_position: Point,
    snapshot: &BufferSnapshot,
    editable_range: Range<Point>,
    prompt: &mut String,
) {
    writeln!(prompt, "{EDITABLE_REGION_START_MARKER}").unwrap();
    for chunk in snapshot.chunks(editable_range.start..cursor_position, false) {
        prompt.push_str(chunk.text);
    }
    prompt.push_str(CURSOR_MARKER);
    for chunk in snapshot.chunks(cursor_position..editable_range.end, false) {
        prompt.push_str(chunk.text);
    }
    write!(prompt, "\n{EDITABLE_REGION_END_MARKER}").unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{App, AppContext};
    use indoc::indoc;
    use language::Buffer;
    use text::OffsetRangeExt as _;

    #[gpui::test]
    fn test_excerpt_for_cursor_position(cx: &mut App) {
        let text = indoc! {r#"
            fn foo() {
                let x = 42;
                println!("Hello, world!");
            }

            fn bar() {
                let x = 42;
                let mut sum = 0;
                for i in 0..x {
                    sum += i;
                }
                println!("Sum: {}", sum);
                return sum;
            }

            fn generate_random_numbers() -> Vec<i32> {
                let mut rng = rand::thread_rng();
                let mut numbers = Vec::new();
                for _ in 0..5 {
                    numbers.push(rng.random_range(1..101));
                }
                numbers
            }
        "#};
        let buffer = cx.new(|cx| Buffer::local(text, cx).with_language(language::rust_lang(), cx));
        let snapshot = buffer.read(cx).snapshot();

        // The excerpt expands to syntax boundaries.
        // With 50 token editable limit, we get a region that expands to syntax nodes.
        let excerpt = excerpt_for_cursor_position(Point::new(12, 5), "main.rs", &snapshot, 50, 32);
        assert_eq!(
            excerpt.prompt,
            indoc! {r#"
            ```main.rs

            fn bar() {
                let x = 42;
            <|editable_region_start|>
                let mut sum = 0;
                for i in 0..x {
                    sum += i;
                }
                println!("Sum: {}", sum);
                r<|user_cursor_is_here|>eturn sum;
            }

            fn generate_random_numbers() -> Vec<i32> {
            <|editable_region_end|>
                let mut rng = rand::thread_rng();
                let mut numbers = Vec::new();
            ```"#}
        );

        // With smaller budget, the region expands to syntax boundaries but is tighter.
        let excerpt = excerpt_for_cursor_position(Point::new(12, 5), "main.rs", &snapshot, 40, 32);
        assert_eq!(
            excerpt.prompt,
            indoc! {r#"
            ```main.rs
            fn bar() {
                let x = 42;
                let mut sum = 0;
                for i in 0..x {
            <|editable_region_start|>
                    sum += i;
                }
                println!("Sum: {}", sum);
                r<|user_cursor_is_here|>eturn sum;
            }

            fn generate_random_numbers() -> Vec<i32> {
            <|editable_region_end|>
                let mut rng = rand::thread_rng();
            ```"#}
        );
    }

    #[gpui::test]
    fn test_parse_edits_empty_editable_region(cx: &mut App) {
        let text = "fn foo() {\n    let x = 42;\n}\n";
        let buffer = cx.new(|cx| Buffer::local(text, cx));
        let snapshot = buffer.read(cx).snapshot();

        let output = "<|editable_region_start|>\n<|editable_region_end|>";
        let editable_range = 0..text.len();
        let edits = parse_edits(output, editable_range, &snapshot).unwrap();
        assert_eq!(edits.len(), 1);
        let (range, new_text) = &edits[0];
        assert_eq!(range.to_offset(&snapshot), 0..text.len(),);
        assert_eq!(new_text.as_ref(), "");
    }

    #[gpui::test]
    fn test_parse_edits_multibyte_char_before_end_marker(cx: &mut App) {
        let text = "// café";
        let buffer = cx.new(|cx| Buffer::local(text, cx));
        let snapshot = buffer.read(cx).snapshot();

        let output = "<|editable_region_start|>\n// café<|editable_region_end|>";
        let editable_range = 0..text.len();

        let edits = parse_edits(output, editable_range, &snapshot).unwrap();
        assert_eq!(edits, vec![]);
    }

    #[gpui::test]
    fn test_parse_edits_multibyte_char_after_start_marker(cx: &mut App) {
        let text = "é is great";
        let buffer = cx.new(|cx| Buffer::local(text, cx));
        let snapshot = buffer.read(cx).snapshot();

        let output = "<|editable_region_start|>é is great\n<|editable_region_end|>";
        let editable_range = 0..text.len();

        let edits = parse_edits(output, editable_range, &snapshot).unwrap();
        assert!(edits.is_empty());
    }
}
