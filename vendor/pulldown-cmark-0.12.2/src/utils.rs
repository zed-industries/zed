//! Miscellaneous utilities to increase comfort.
//! Special thanks to:
//!
//! - <https://github.com/BenjaminRi/Redwood-Wiki/blob/master/src/markdown_utils.rs>.
//! Its author authorized the use of this GPL code in this project in
//! <https://github.com/raphlinus/pulldown-cmark/issues/507>.
//!
//! - <https://gist.github.com/rambip/a507c312ed61c99c24b2a54f98325721>.
//! Its author proposed the solution in
//! <https://github.com/raphlinus/pulldown-cmark/issues/708>.

use crate::{CowStr, Event};
use std::ops::Range;

/// Merge consecutive `Event::Text` events into only one.
#[derive(Debug)]
pub struct TextMergeStream<'a, I> {
    inner: TextMergeWithOffset<'a, DummyOffsets<I>>,
}

impl<'a, I> TextMergeStream<'a, I>
where
    I: Iterator<Item = Event<'a>>,
{
    pub fn new(iter: I) -> Self {
        Self {
            inner: TextMergeWithOffset::new(DummyOffsets(iter)),
        }
    }
}

impl<'a, I> Iterator for TextMergeStream<'a, I>
where
    I: Iterator<Item = Event<'a>>,
{
    type Item = Event<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(event, _)| event)
    }
}

#[derive(Debug)]
struct DummyOffsets<I>(I);

impl<'a, I> Iterator for DummyOffsets<I>
where
    I: Iterator<Item = Event<'a>>,
{
    type Item = (Event<'a>, Range<usize>);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|event| (event, 0..0))
    }
}

/// Merge consecutive `Event::Text` events into only one, with offsets.
///
/// Compatible with with [`OffsetIter`](crate::OffsetIter).
#[derive(Debug)]
pub struct TextMergeWithOffset<'a, I> {
    iter: I,
    last_event: Option<(Event<'a>, Range<usize>)>,
}

impl<'a, I> TextMergeWithOffset<'a, I>
where
    I: Iterator<Item = (Event<'a>, Range<usize>)>,
{
    pub fn new(iter: I) -> Self {
        Self {
            iter,
            last_event: None,
        }
    }
}

impl<'a, I> Iterator for TextMergeWithOffset<'a, I>
where
    I: Iterator<Item = (Event<'a>, Range<usize>)>,
{
    type Item = (Event<'a>, Range<usize>);

    fn next(&mut self) -> Option<Self::Item> {
        match (self.last_event.take(), self.iter.next()) {
            (
                Some((Event::Text(last_text), last_offset)),
                Some((Event::Text(next_text), next_offset)),
            ) => {
                // We need to start merging consecutive text events together into one
                let mut string_buf: String = last_text.into_string();
                string_buf.push_str(&next_text);
                let mut offset = last_offset;
                offset.end = next_offset.end;
                loop {
                    // Avoid recursion to avoid stack overflow and to optimize concatenation
                    match self.iter.next() {
                        Some((Event::Text(next_text), next_offset)) => {
                            string_buf.push_str(&next_text);
                            offset.end = next_offset.end;
                        }
                        next_event => {
                            self.last_event = next_event;
                            if string_buf.is_empty() {
                                // Discard text event(s) altogether if there is no text
                                break self.next();
                            } else {
                                break Some((
                                    Event::Text(CowStr::Boxed(string_buf.into_boxed_str())),
                                    offset,
                                ));
                            }
                        }
                    }
                }
            }
            (None, Some(next_event)) => {
                // This only happens once during the first iteration and if there are items
                self.last_event = Some(next_event);
                self.next()
            }
            (None, None) => {
                // This happens when the iterator is depleted
                None
            }
            (last_event, next_event) => {
                // The ordinary case, emit one event after the other without modification
                self.last_event = next_event;
                last_event
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Parser;

    #[test]
    fn text_merge_stream_indent() {
        let source = r#"
    first line
    second line
"#;
        let parser = TextMergeStream::new(Parser::new(source));
        let text_events: Vec<_> = parser.filter(|e| matches!(e, Event::Text(_))).collect();
        assert_eq!(
            text_events,
            [Event::Text("first line\nsecond line\n".into())]
        );
    }

    #[test]
    fn text_merge_with_offset_indent() {
        let source = r#"
    first line
    second line
"#;
        let parser = TextMergeWithOffset::new(Parser::new(source).into_offset_iter());
        let text_events: Vec<_> = parser
            .filter(|e| matches!(e, (Event::Text(_), _)))
            .collect();
        assert_eq!(
            text_events,
            [(Event::Text("first line\nsecond line\n".into()), 5..32)]
        );
    }

    #[test]
    fn text_merge_empty_is_discarded() {
        let events = [
            Event::Rule,
            Event::Text("".into()),
            Event::Text("".into()),
            Event::Rule,
        ];
        let result: Vec<_> = TextMergeStream::new(events.into_iter()).collect();
        assert_eq!(result, [Event::Rule, Event::Rule]);
    }
}
