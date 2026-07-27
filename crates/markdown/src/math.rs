use std::collections::BTreeMap;
use std::ops::Range;

use gpui::SharedString;

use crate::parser::MarkdownEvent;

/// A math expression extracted from parsed markdown events.
#[derive(Clone, Debug)]
pub(crate) struct ParsedMarkdownMathExpression {
    /// Byte range of the full math expression in the source (including delimiters).
    pub(crate) source_range: Range<usize>,
    /// The raw LaTeX content (without delimiters).
    pub(crate) latex: SharedString,
    /// Whether this is a display math expression (`$$...$$` / `\[...\]`).
    pub(crate) is_display: bool,
}

/// Extracts math expressions from parsed markdown events.
///
/// Walks the event list looking for `MarkdownEvent::InlineMath` and
/// `MarkdownEvent::DisplayMath` events, collecting them into a map keyed
/// by source offset for O(log n) lookup during rendering.
pub(crate) fn extract_math_expressions(
    events: &[(Range<usize>, MarkdownEvent)],
) -> BTreeMap<usize, ParsedMarkdownMathExpression> {
    let mut expressions = BTreeMap::default();

    for (source_range, event) in events {
        match event {
            MarkdownEvent::InlineMath(latex) => {
                expressions.insert(
                    source_range.start,
                    ParsedMarkdownMathExpression {
                        source_range: source_range.clone(),
                        latex: SharedString::from(latex.as_str()),
                        is_display: false,
                    },
                );
            }
            MarkdownEvent::DisplayMath(latex) => {
                expressions.insert(
                    source_range.start,
                    ParsedMarkdownMathExpression {
                        source_range: source_range.clone(),
                        latex: SharedString::from(latex.as_str()),
                        is_display: true,
                    },
                );
            }
            _ => {}
        }
    }

    expressions
}
