use crate::parser::{MarkdownEvent, MarkdownTag, MarkdownTagEnd};
use std::ops::Range;

struct InlineSpan {
    range: Range<usize>,
    content: Range<usize>,
    is_code: bool,
}

impl InlineSpan {
    fn opening<'a>(&self, source: &'a str) -> &'a str {
        source
            .get(self.range.start..self.content.start)
            .unwrap_or("")
    }

    fn closing<'a>(&self, source: &'a str) -> &'a str {
        source.get(self.content.end..self.range.end).unwrap_or("")
    }
}

fn is_inline_span_tag(tag: &MarkdownTag) -> bool {
    matches!(
        tag,
        MarkdownTag::Emphasis
            | MarkdownTag::Strong
            | MarkdownTag::Strikethrough
            | MarkdownTag::Superscript
            | MarkdownTag::Subscript
            | MarkdownTag::Link { .. }
    )
}

fn is_inline_span_tag_end(tag: &MarkdownTagEnd) -> bool {
    matches!(
        tag,
        MarkdownTagEnd::Emphasis
            | MarkdownTagEnd::Strong
            | MarkdownTagEnd::Strikethrough
            | MarkdownTagEnd::Superscript
            | MarkdownTagEnd::Subscript
            | MarkdownTagEnd::Link
    )
}

fn inline_code_full_range(source: &str, content: &Range<usize>) -> Range<usize> {
    let opening_ticks = source[..content.start]
        .bytes()
        .rev()
        .take_while(|&byte| byte == b'`')
        .count();
    let closing_ticks = source[content.end..]
        .bytes()
        .take_while(|&byte| byte == b'`')
        .count();
    let ticks = opening_ticks.min(closing_ticks);
    content.start - ticks..content.end + ticks
}

fn collect_inline_spans(source: &str, events: &[(Range<usize>, MarkdownEvent)]) -> Vec<InlineSpan> {
    fn note_child(stack: &mut [(Range<usize>, Option<Range<usize>>)], child: &Range<usize>) {
        for (_, content) in stack.iter_mut() {
            match content {
                Some(content) => content.end = content.end.max(child.end),
                None => *content = Some(child.clone()),
            }
        }
    }

    let mut spans = Vec::new();
    let mut stack: Vec<(Range<usize>, Option<Range<usize>>)> = Vec::new();
    for (event_range, event) in events {
        match event {
            MarkdownEvent::Start(tag) if is_inline_span_tag(tag) => {
                note_child(&mut stack, event_range);
                stack.push((event_range.clone(), None));
            }
            MarkdownEvent::End(tag) if is_inline_span_tag_end(tag) => {
                if let Some((range, content)) = stack.pop() {
                    let content = content.unwrap_or(range.clone());
                    spans.push(InlineSpan {
                        range,
                        content,
                        is_code: false,
                    });
                }
                note_child(&mut stack, event_range);
            }
            MarkdownEvent::Code | MarkdownEvent::SubstitutedCode(_) => {
                let range = inline_code_full_range(source, event_range);
                note_child(&mut stack, &range);
                spans.push(InlineSpan {
                    range,
                    content: event_range.clone(),
                    is_code: true,
                });
            }
            _ => note_child(&mut stack, event_range),
        }
    }
    spans
}

pub(crate) fn rebalanced_markdown_for_selection(
    source: &str,
    events: &[(Range<usize>, MarkdownEvent)],
    selection: Range<usize>,
) -> String {
    let Some(selection) = clamp_to_char_boundaries(source, selection) else {
        return String::new();
    };

    let spans = collect_inline_spans(source, events);

    let Some(selection) = snap_out_of_delimiters(&spans, selection) else {
        return String::new();
    };

    if selection_is_only_inside_code_spans(&spans, &selection) {
        return source[selection].to_string();
    }

    rebalance_delimiters(source, &spans, &selection)
}

fn clamp_to_char_boundaries(source: &str, selection: Range<usize>) -> Option<Range<usize>> {
    let mut start = selection.start.min(source.len());
    let mut end = selection.end.min(source.len());
    while start < end && !source.is_char_boundary(start) {
        start += 1;
    }
    while end > start && !source.is_char_boundary(end) {
        end -= 1;
    }
    (start < end).then(|| start..end)
}

/// Shrinks selection boundaries that fall inside delimiter syntax (`**`,
/// etc.) so no delimiter is left half-selected:
///
/// - an end in `**bold*|*` snaps back to `**bold|**`
/// - a start in `*|*bold**` snaps forward to `**|bold**`
///
/// This repeats until stable, since snapping can land inside a nested span's
/// delimiter. Returns `None` if the selection becomes empty.
fn snap_out_of_delimiters(spans: &[InlineSpan], selection: Range<usize>) -> Option<Range<usize>> {
    let mut start = selection.start;
    let mut end = selection.end;
    loop {
        let mut changed = false;
        for span in spans {
            if end > span.range.start && end <= span.content.start {
                end = span.range.start;
                changed = true;
            } else if end > span.content.end && end <= span.range.end {
                end = span.content.end;
                changed = true;
            }
            if start >= span.range.start && start < span.content.start {
                start = span.content.start;
                changed = true;
            } else if start >= span.content.end && start < span.range.end {
                start = span.range.end;
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }
    (start < end).then(|| start..end)
}

fn selection_is_only_inside_code_spans(spans: &[InlineSpan], selection: &Range<usize>) -> bool {
    let contains = |span: &InlineSpan| {
        span.content.start <= selection.start && selection.end <= span.content.end
    };
    spans.iter().any(|span| span.is_code && contains(span))
        && !spans.iter().any(|span| !span.is_code && contains(span))
}

/// Re-adds delimiters cut off by the selection so the result is well-formed
/// markdown:
///
/// - selecting `old te` in `**bold text**` yields `**old te**`
/// - nested spans are reopened outermost first: selecting `alic` in
///   `**bold _italic_**` yields `**_alic_**`
fn rebalance_delimiters(source: &str, spans: &[InlineSpan], selection: &Range<usize>) -> String {
    let nesting_order = |a: &&InlineSpan, b: &&InlineSpan| {
        a.range
            .start
            .cmp(&b.range.start)
            .then(b.range.end.cmp(&a.range.end))
    };

    let mut open_at_start = spans
        .iter()
        .filter(|span| span.content.start <= selection.start && selection.start < span.content.end)
        .collect::<Vec<_>>();
    open_at_start.sort_by(nesting_order);

    let mut open_at_end = spans
        .iter()
        .filter(|span| span.content.start < selection.end && selection.end <= span.content.end)
        .collect::<Vec<_>>();
    open_at_end.sort_by(nesting_order);

    let mut result = String::new();
    for span in &open_at_start {
        result.push_str(span.opening(source));
    }
    result.push_str(&source[selection.clone()]);
    for span in open_at_end.iter().rev() {
        result.push_str(span.closing(source));
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse_markdown_with_options;
    use util::test::marked_text_ranges;

    fn markdown_for(source: &str, selection: Range<usize>) -> String {
        let parsed = parse_markdown_with_options(source, false, false, false);
        rebalanced_markdown_for_selection(source, &parsed.events, selection)
    }

    fn markdown_for_marked(marked_source: &str) -> String {
        let (source, selection) = marked_range(marked_source);
        markdown_for(&source, selection)
    }

    #[track_caller]
    fn assert_marked_selection(marked_source: &str, expected: &str) {
        assert_eq!(
            markdown_for_marked(marked_source),
            expected,
            "source: {marked_source:?}"
        );
    }

    #[track_caller]
    fn marked_range(marked_source: &str) -> (String, Range<usize>) {
        let (source, ranges) = marked_text_ranges(marked_source, false);
        match ranges.as_slice() {
            [selection] => (source, selection.clone()),
            _ => panic!("expected exactly one «» range in marked source"),
        }
    }

    fn inline_spans(source: &str) -> Vec<InlineSpan> {
        let parsed = parse_markdown_with_options(source, false, false, false);
        collect_inline_spans(source, &parsed.events)
    }

    #[track_caller]
    fn assert_snapped(marked_selection: &str, marked_expected: Option<&str>, message: &str) {
        let (source, selection) = marked_range(marked_selection);
        let expected = marked_expected.map(|marked_expected| {
            let (expected_source, range) = marked_range(marked_expected);
            assert_eq!(
                expected_source, source,
                "expected must be marked on the same source"
            );
            range
        });
        assert_eq!(
            snap_out_of_delimiters(&inline_spans(&source), selection),
            expected,
            "{message}"
        );
    }

    #[test]
    fn test_snap_out_of_delimiters() {
        assert_snapped(
            "**«bold»** rest",
            Some("**«bold»** rest"),
            "boundaries already in content must be untouched",
        );
        assert_snapped(
            "*«*bold»** rest",
            Some("**«bold»** rest"),
            "a start in the opening delimiter must advance into the content",
        );
        assert_snapped(
            "**«bold*»* rest",
            Some("**«bold»** rest"),
            "an end in the closing delimiter must retreat to the content end",
        );
        assert_snapped(
            "**bold*«* rest»",
            Some("**bold**« rest»"),
            "a start in the closing delimiter must advance past the whole span",
        );
        assert_snapped(
            "«*»*bold** rest",
            None,
            "an end in the opening delimiter must retreat to before the span",
        );
        assert_snapped(
            "**bold«**» rest",
            None,
            "a selection covering only delimiter text must collapse",
        );
    }

    #[test]
    fn test_snap_out_of_delimiters_cascades_through_nested_spans() {
        assert_snapped(
            "**`«code`*»*",
            Some("**`«code»`**"),
            "an end inside bold's closing `**` first snaps to code's closing \
             backtick, so it must cascade into the code content",
        );
    }

    fn selection_is_plain(marked_source: &str) -> bool {
        let (source, selection) = marked_range(marked_source);
        selection_is_only_inside_code_spans(&inline_spans(&source), &selection)
    }

    #[test]
    fn test_selection_is_only_inside_code_spans() {
        assert!(
            selection_is_plain("run `«cargo» test` now"),
            "a selection fully inside the code span's content must be plain"
        );
        assert!(
            !selection_is_plain("«run `cargo» test` now"),
            "a selection reaching outside the code span must not be plain"
        );
        assert!(
            !selection_is_plain("**`«code»`**"),
            "code nested in bold must not be plain: the bold span also contains it"
        );
    }

    #[test]
    fn test_markdown_for_selection_balances_inline_spans() {
        assert_marked_selection("This is **«bold»** text in a sentence.", "**bold**");
        assert_marked_selection("This is **«bold**» text in a sentence.", "**bold**");
        assert_marked_selection("Th«is is **bo»ld** text in a sentence.", "is is **bo**");

        assert_marked_selection("This is *«italic»* text in a sentence.", "*italic*");
        assert_marked_selection("This is *it«al»ic* text in a sentence.", "*al*");

        assert_marked_selection("T«his is `cod»e` all `in one` sentence.", "his is `cod`");
        assert_marked_selection(
            "This is `c«ode` all `in o»ne` sentence.",
            "`ode` all `in o`",
        );
        assert_marked_selection(
            "This is `«code` all `in one»` sentence.",
            "`code` all `in one`",
        );
        assert_marked_selection(
            "This is `«code` all `in one`» sentence.",
            "`code` all `in one`",
        );

        // Special case for single inline code blocks
        assert_marked_selection("This is `«code»` all `in one` sentence.", "code");
        assert_marked_selection("This is `«code`» all `in one` sentence.", "code");
        assert_marked_selection("This is `c«od»e` all `in one` sentence.", "od");
    }

    #[test]
    fn test_markdown_for_selection_nested_spans() {
        assert_marked_selection("**bo«ld wi»th `code` inside**", "**ld wi**");
        assert_marked_selection("**bold with `c«od»e` inside**", "**`od`**");
        assert_marked_selection("**bold with `«code»` inside**", "**`code`**");
        assert_marked_selection(
            "**«bold with `code` inside»**",
            "**bold with `code` inside**",
        );
        assert_marked_selection(
            "«**bold with `code` inside**»",
            "**bold with `code` inside**",
        );
    }

    #[test]
    fn test_markdown_for_selection_links() {
        assert_marked_selection(
            "[Visit Rust's we«bsite»](https://rust.org)",
            "[bsite](https://rust.org)",
        );
        assert_marked_selection(
            "[«Visit Rust's website»](https://rust.org)",
            "[Visit Rust's website](https://rust.org)",
        );
        assert_marked_selection("visit https://«example».com now", "example");
    }

    #[test]
    fn test_markdown_for_selection_plain_text_and_blocks() {
        assert_eq!(
            markdown_for_marked("some «text»"),
            "text",
            "plain text must be unchanged"
        );
        assert_eq!(
            markdown_for_marked("«para one\n\n- item **bold**\n- item two»"),
            "para one\n\n- item **bold**\n- item two",
            "selections spanning multiple blocks must keep interior syntax as-is"
        );
        assert_eq!(
            markdown_for_marked("```rust\n«let x = 1;»\n```"),
            "let x = 1;",
            "a selection inside a fenced code block must stay plain"
        );
        assert_eq!(
            markdown_for("abc", 2..100),
            "c",
            "out-of-bounds ends must be clamped"
        );
        assert_eq!(
            markdown_for("abc", Range { start: 3, end: 2 }),
            "",
            "inverted ranges must yield an empty string"
        );
    }
}
