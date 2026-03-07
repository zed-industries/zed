use crate::parser::{MarkdownEvent, MarkdownTag, MarkdownTagEnd};
use std::ops::Range;

/// Source-level metadata for one inline markdown wrapper.
///
/// `wrapper_range` always points to the full source slice that includes markdown syntax
/// (for example `**bold**`, `` `code` ``, or `[label](dest)`).
///
/// `content_range` points to the visible text region inside that wrapper.
/// During span construction it may be `None`, but `finish_spans` removes those entries
/// before resolution.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MarkdownSpan {
    /// Full markdown wrapper range in source, including opening and closing syntax markers.
    pub wrapper_range: Range<usize>,
    /// Visible content range inside `wrapper_range` used for boundary snapping/expansion.
    /// `None` is only expected before construction is finalized (or in defensive fallback paths).
    pub content_range: Option<Range<usize>>,
}

impl MarkdownSpan {
    fn new(wrapper_range: Range<usize>) -> Self {
        Self {
            wrapper_range,
            content_range: None,
        }
    }

    fn extend_content(&mut self, source_range: &Range<usize>) {
        self.content_range = Some(match self.content_range.take() {
            Some(existing_range) => {
                existing_range.start.min(source_range.start)
                    ..existing_range.end.max(source_range.end)
            }
            None => source_range.clone(),
        });
    }

    #[inline]
    fn bounds(&self) -> Option<(&Range<usize>, &Range<usize>)> {
        self.content_range
            .as_ref()
            .map(|content_range| (&self.wrapper_range, content_range))
    }

    fn snap(&self, range: &mut Range<usize>) -> bool {
        let Some((wrapper_range, content_range)) = self.bounds() else {
            return false;
        };

        let start = range.start;
        let end = range.end;
        let snap_start_open = start > wrapper_range.start && start < content_range.start;
        let snap_start_close = start >= content_range.end && start < wrapper_range.end;
        let snap_end_open = end > wrapper_range.start && end <= content_range.start;
        let snap_end_close = end > content_range.end && end < wrapper_range.end;

        if !(snap_start_open || snap_start_close || snap_end_open || snap_end_close) {
            return false;
        }

        if snap_start_open {
            range.start = wrapper_range.start;
        } else if snap_start_close {
            range.start = wrapper_range.end;
        }

        if snap_end_open {
            range.end = wrapper_range.start;
        } else if snap_end_close {
            range.end = wrapper_range.end;
        }

        true
    }

    fn trim_partial(&self, range: &mut Range<usize>) -> bool {
        let Some((wrapper_range, content_range)) = self.bounds() else {
            return false;
        };

        let original_start = range.start;
        let original_end = range.end;
        let at_left_boundary = original_start == wrapper_range.start;
        let at_right_boundary = original_end == wrapper_range.end;
        if !at_left_boundary && !at_right_boundary {
            return false;
        }

        let trim_start = at_left_boundary
            && original_end > content_range.start
            && original_end < content_range.end;
        let trim_end = at_right_boundary
            && original_start > content_range.start
            && original_start < content_range.end;

        if trim_start {
            range.start = content_range.start;
        }

        if trim_end {
            range.end = content_range.end;
        }

        trim_start || trim_end
    }

    fn expand(&self, normalized_range: &Range<usize>, range: &mut Range<usize>) {
        let Some((wrapper_range, content_range)) = self.bounds() else {
            return;
        };

        let covers_content = normalized_range.start <= content_range.start
            && normalized_range.end >= content_range.end;
        if !covers_content {
            return;
        }

        let left_aligned = normalized_range.start == content_range.start
            || normalized_range.start == wrapper_range.start;
        let right_aligned =
            normalized_range.end == content_range.end || normalized_range.end == wrapper_range.end;
        if !left_aligned && !right_aligned {
            return;
        }

        let expand_left =
            left_aligned && (right_aligned || normalized_range.end >= wrapper_range.end);
        let expand_right =
            right_aligned && (left_aligned || normalized_range.start <= wrapper_range.start);

        if expand_left {
            range.start = range.start.min(wrapper_range.start);
        }

        if expand_right {
            range.end = range.end.max(wrapper_range.end);
        }
    }
}

/// Incrementally builds inline source spans from markdown parser events.
///
/// This keeps just enough transient state (`span_stack`) to handle nested wrappers
/// in a single event pass, so callers can derive spans while doing other work over
/// the same event stream (for example image extraction).
pub(crate) struct SourceSpanBuilder {
    /// Collected spans in encounter order.
    spans: Vec<MarkdownSpan>,
    /// Stack of indices into `spans` representing currently open inline wrappers.
    span_stack: Vec<usize>,
}

impl From<SourceSpanBuilder> for Vec<MarkdownSpan> {
    /// Finalizes builder output by dropping wrappers that never accumulated visible content.
    fn from(source_span_builder: SourceSpanBuilder) -> Self {
        source_span_builder.into_spans()
    }
}

impl SourceSpanBuilder {
    /// Creates an empty builder for a new event stream.
    pub(crate) fn new() -> Self {
        Self {
            spans: Vec::new(),
            span_stack: Vec::new(),
        }
    }

    fn begin_span(&mut self, wrapper_range: Range<usize>) {
        let span_index = self.spans.len();
        self.spans.push(MarkdownSpan::new(wrapper_range));
        self.span_stack.push(span_index);
    }

    fn end_span(&mut self) {
        self.span_stack.pop();
    }

    fn expand_open_spans(&mut self, source_range: &Range<usize>) {
        for &span_index in &self.span_stack {
            if let Some(span) = self.spans.get_mut(span_index) {
                span.extend_content(source_range);
            }
        }
    }

    fn push_closed_span(&mut self, wrapper_range: Range<usize>, content_range: Range<usize>) {
        let mut span = MarkdownSpan::new(wrapper_range);
        span.extend_content(&content_range);
        self.spans.push(span);
    }

    /// Incorporates one parser event into the current span state.
    ///
    /// Only inline wrappers that affect source-copy semantics are tracked.
    pub(crate) fn push_event(&mut self, range: &Range<usize>, event: &MarkdownEvent) {
        match event {
            MarkdownEvent::Start(
                MarkdownTag::Strong
                | MarkdownTag::Emphasis
                | MarkdownTag::Strikethrough
                | MarkdownTag::Link { .. }
                | MarkdownTag::Superscript
                | MarkdownTag::Subscript,
            ) => {
                self.begin_span(range.clone());
            }
            MarkdownEvent::End(
                MarkdownTagEnd::Strong
                | MarkdownTagEnd::Emphasis
                | MarkdownTagEnd::Strikethrough
                | MarkdownTagEnd::Link
                | MarkdownTagEnd::Superscript
                | MarkdownTagEnd::Subscript,
            ) => {
                self.end_span();
            }
            MarkdownEvent::Text | MarkdownEvent::SubstitutedText(_) => {
                self.expand_open_spans(range);
            }
            MarkdownEvent::Code { wrapper_range } => {
                self.push_closed_span(wrapper_range.clone(), range.clone());
            }
            _ => {}
        }
    }

    /// Finalizes builder output by dropping wrappers that never accumulated visible content.
    pub(crate) fn into_spans(mut self) -> Vec<MarkdownSpan> {
        self.spans.retain(|span| span.content_range.is_some());
        self.spans
    }
}

/// Stateless resolver over parse-time markdown spans.
///
/// It applies copy semantics to a source selection:
/// - snap edges out of hidden wrapper syntax,
/// - trim unbalanced partial wrappers,
/// - expand to full wrappers when the selection semantically covers formatted content.
pub(crate) struct MarkdownRangeResolver<'a> {
    /// Precomputed inline wrapper spans extracted from markdown events.
    spans: &'a [MarkdownSpan],
}

impl<'a> MarkdownRangeResolver<'a> {
    pub(crate) fn new(spans: &'a [MarkdownSpan]) -> Self {
        Self { spans }
    }

    /// Resolves a selection range to a markdown source range suitable for copy.
    ///
    /// The returned range avoids leaking unmatched wrapper syntax while preserving
    /// full wrappers when the selection semantically covers formatted content.
    pub(crate) fn resolve(&self, range: Range<usize>) -> Range<usize> {
        let range = self.normalize_edges(range);
        if range.start >= range.end {
            return range;
        }

        self.expand_wrappers(range)
    }

    fn normalize_edges(&self, mut range: Range<usize>) -> Range<usize> {
        loop {
            let mut snapped = range.clone();
            for span in self.spans {
                span.snap(&mut snapped);
            }

            if snapped.start > snapped.end {
                snapped.end = snapped.start;
            }
            if snapped.start >= snapped.end {
                return snapped;
            }

            let mut trimmed = snapped.clone();
            for span in self.spans {
                span.trim_partial(&mut trimmed);
            }

            if trimmed.start > trimmed.end {
                trimmed.end = trimmed.start;
            }
            if trimmed.start >= trimmed.end || trimmed == range {
                return trimmed;
            }
            range = trimmed;
        }
    }

    fn expand_wrappers(&self, mut range: Range<usize>) -> Range<usize> {
        loop {
            let mut next = range.clone();

            for span in self.spans {
                span.expand(&range, &mut next);
            }

            if next.start > next.end {
                next.end = next.start;
            }
            if next == range {
                return next;
            }
            range = next;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{MarkdownRangeResolver, MarkdownSpan, SourceSpanBuilder};
    use crate::parser::parse_markdown;
    use std::ops::Range;

    fn spans_for(source: &str) -> Vec<MarkdownSpan> {
        let (events, _, _) = parse_markdown(source);
        let mut source_span_builder = SourceSpanBuilder::new();
        for (range, event) in &events {
            source_span_builder.push_event(range, event);
        }
        source_span_builder.into()
    }

    fn resolved_range_for(source: &str, selection: Range<usize>) -> Range<usize> {
        let (events, _, _) = parse_markdown(source);
        let mut source_span_builder = SourceSpanBuilder::new();
        for (range, event) in &events {
            source_span_builder.push_event(range, event);
        }
        let spans: Vec<MarkdownSpan> = source_span_builder.into();
        let source_length = source.len();
        let start = selection.start.min(source_length);
        let end = selection.end.min(source_length);
        let clamped_range = if start <= end {
            start..end
        } else {
            start..start
        };

        MarkdownRangeResolver::new(spans.as_slice()).resolve(clamped_range)
    }

    #[test]
    fn test_build_source_spans_for_strong() {
        let source = "**bold**";
        let spans = spans_for(source);
        assert_eq!(spans.len(), 1);
        let span = &spans[0];
        assert_eq!(&source[span.wrapper_range.clone()], "**bold**");
        assert_eq!(
            span.content_range
                .as_ref()
                .map(|content_range| &source[content_range.clone()]),
            Some("bold")
        );
    }

    #[test]
    fn test_build_source_spans_for_link() {
        let source = "[label](https://example.com)";
        let spans = spans_for(source);
        assert_eq!(spans.len(), 1);
        let span = &spans[0];
        assert_eq!(&source[span.wrapper_range.clone()], source);
        assert_eq!(
            span.content_range
                .as_ref()
                .map(|content_range| &source[content_range.clone()]),
            Some("label")
        );
    }

    #[test]
    fn test_build_source_spans_for_inline_code() {
        let source = "Use `code` here";
        let spans = spans_for(source);
        assert_eq!(spans.len(), 1);
        let span = &spans[0];
        assert_eq!(&source[span.wrapper_range.clone()], "`code`");
        assert_eq!(
            span.content_range
                .as_ref()
                .map(|content_range| &source[content_range.clone()]),
            Some("code")
        );
    }

    #[test]
    fn test_resolved_range_expands_strong_wrapper() {
        let source = "- **How** should the new flow work?";

        let expanded = resolved_range_for(source, 4..9);
        assert_eq!(expanded, 2..9);
        assert_eq!(&source[expanded], "**How**");
    }

    #[test]
    fn test_resolved_range_trims_wrapper_suffix_for_partial_selection() {
        let source = "- **How** should the new flow work?";

        let range = 5..9;
        let expanded = resolved_range_for(source, range.clone());
        let expected_start = source.find("ow").expect("partial content should exist");
        let expected = expected_start..expected_start + "ow".len();
        assert_eq!(expanded, expected);
        assert_eq!(&source[expanded], "ow");
    }

    #[test]
    fn test_resolved_range_expands_full_link_selection() {
        let source = "[label](https://example.com)";

        let label_start = source.find("label").expect("label should exist");
        let label_end = label_start + "label".len();
        let expanded = resolved_range_for(source, label_start..label_end);
        assert_eq!(expanded, 0..source.len());
    }

    #[test]
    fn test_resolved_range_expands_nested_wrappers() {
        let source = "***x***";

        let content_start = source.find('x').expect("content should exist");
        let expanded = resolved_range_for(source, content_start..content_start + 1);
        assert_eq!(expanded, 0..source.len());
    }

    #[test]
    fn test_resolved_range_keeps_partial_inline_code_without_backticks() {
        let source = "`Test of selection`";
        let content_start = source.find("Test").expect("content should exist");

        let partial = content_start..content_start + "Test".len();
        let expanded = resolved_range_for(source, partial.clone());
        assert_eq!(expanded, partial);
        assert_eq!(&source[expanded], "Test");
    }

    #[test]
    fn test_resolved_range_keeps_partial_link_label_without_wrapper() {
        let source = "[Test of selection](link)";
        let label_start = source.find("Test").expect("label should exist");
        let label_end = label_start + "Test of selection".len();

        let left_partial = label_start..label_start + "Test".len();
        let left_expanded = resolved_range_for(source, left_partial.clone());
        assert_eq!(left_expanded, left_partial);
        assert_eq!(&source[left_expanded], "Test");

        let right_partial = label_end - "selection".len()..label_end;
        let right_expanded = resolved_range_for(source, right_partial.clone());
        assert_eq!(right_expanded, right_partial);
        assert_eq!(&source[right_expanded], "selection");
    }

    #[test]
    fn test_resolved_range_trims_link_suffix_when_start_is_inside_label() {
        let link_label = "dummy_guide.md";
        let link_destination = "file:///tmp/dummy_guide.md";
        let source = concat!(
            "See details in ",
            "[dummy_guide.md]",
            "(file:///tmp/dummy_guide.md)."
        );
        let link_prefix = format!("[{link_label}](");
        let link_start = source.find(&link_prefix).expect("link should exist");
        let content_start = link_start + 1;
        let content_end = content_start + link_label.len();
        let selected_end = source[link_start..]
            .find(')')
            .map(|offset| link_start + offset + 1)
            .expect("link wrapper end should exist");

        let selected_start = source.find("guide.md").expect("partial label should exist");
        let expanded = resolved_range_for(source, selected_start..selected_end);

        assert_eq!(expanded, selected_start..content_end);
        assert_eq!(&source[expanded.clone()], "guide.md");
        assert!(!source[expanded.clone()].starts_with(&format!("]({link_destination})")));
    }

    #[test]
    fn test_resolved_range_for_multiline_list_selection() {
        let source = concat!(
            "1. **Lorem heading item**\n",
            "Lorem ipsum dolor sit amet.\n",
            "Aliquam tincidunt [ref](file:///tmp/ref.md)"
        );

        let selection_start = source
            .find("Lorem heading item")
            .expect("selection start should exist");
        let selection_end = source.len();
        let expanded = resolved_range_for(source, selection_start..selection_end);

        let expected_start = source
            .find("**Lorem heading item**")
            .expect("bold wrapper should exist");
        assert_eq!(expanded, expected_start..selection_end);
        assert_eq!(
            &source[expanded],
            concat!(
                "**Lorem heading item**\n",
                "Lorem ipsum dolor sit amet.\n",
                "Aliquam tincidunt [ref](file:///tmp/ref.md)"
            )
        );
    }

    #[test]
    fn test_resolved_range_skips_previous_link_suffix_in_long_list() {
        let link_label = "dummy_guide.md";
        let link_destination = "file:///tmp/dummy_guide.md";
        let source = concat!(
            "1. **First item title**  \n",
            "Lorem ipsum dolor sit amet, consectetur adipiscing elit.  \n",
            "[dummy_guide.md](file:///tmp/dummy_guide.md)\n",
            "\n",
            "2. **Second item title**  \n",
            "Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.  \n",
            "[dummy_guide.md](file:///tmp/dummy_guide.md)"
        );
        let link_prefix = format!("[{link_label}](");

        let second_item_start = source
            .find("2. **Second item title**")
            .expect("second item should exist");
        let first_link_start = source
            .find(&link_prefix)
            .expect("first item link should exist");
        assert!(first_link_start < second_item_start);
        let first_link_content_end = first_link_start + 1 + link_label.len();
        let first_link_wrapper_end = source[first_link_start..]
            .find(')')
            .map(|offset| first_link_start + offset + 1)
            .expect("first item link wrapper end should exist");
        let selection_start = first_link_content_end;

        let expanded = resolved_range_for(source, selection_start..source.len());
        let copied = &source[expanded.clone()];

        assert!(expanded.start >= first_link_wrapper_end);
        assert!(!copied.starts_with(&format!("]({link_destination})")));
        assert!(copied.trim_start().starts_with("2. **Second item title**"));
    }
}
