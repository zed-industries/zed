//! Strips `<foreignObject>` elements and de-duplicates redundant fallback label
//! groups from the SVG.
//!
//! merman's raster-safe SVG pipeline converts `<foreignObject>` labels into
//! native `<text>` fallback groups (`data-merman-foreignobject="fallback"`) and
//! removes the original `<foreignObject>` elements before Zed-specific
//! post-processing runs. For most diagram types the fallback `<text>` group is
//! the *only* remaining copy of a label, so it must be preserved.
//!
//! However, some diagram types (e.g. user journey) render a label as a real
//! native `<text>` element *and* also wrap it in a `<foreignObject>` that
//! becomes a fallback group. Rasterizing both produces the same label twice at
//! slightly different sizes. To avoid that, we drop a fallback group only when
//! its text content duplicates a native (non-fallback) `<text>` elsewhere in the
//! diagram. Fallback groups whose label has no native twin are always kept.
//!
//! ```xml
//! <!-- before -->
//! <text class="task">Make tea</text>
//! <g data-merman-foreignobject="fallback"><text>Make tea</text></g>
//!
//! <!-- after -->
//! <text class="task">Make tea</text>
//! ```

use std::collections::{HashSet, VecDeque};

use anyhow::Result;
use quick_xml::Reader;
use quick_xml::events::Event;

use super::ReaderIter;

const FALLBACK_TEXT_CLASS: &str = "merman-foreignobject-fallback-text";

/// Collects the trimmed text content of every native (non-fallback) `<text>`
/// element. Fallback groups whose label matches one of these are duplicates.
fn collect_native_text_contents(svg: &str) -> HashSet<String> {
    let mut contents = HashSet::new();
    let mut reader = Reader::from_str(svg);
    reader.config_mut().check_end_names = false;

    let mut in_native_text = false;
    let mut current = String::new();
    for event in ReaderIter::new(reader) {
        match event {
            Ok(Event::Start(e)) if e.name().as_ref() == b"text" => {
                in_native_text = !is_fallback_text(&e);
                current.clear();
            }
            Ok(Event::Text(t)) if in_native_text => {
                if let Ok(decoded) = t.decode() {
                    current.push_str(&decoded);
                }
            }
            Ok(Event::End(e)) if e.name().as_ref() == b"text" => {
                if in_native_text {
                    let trimmed = current.trim();
                    if !trimmed.is_empty() {
                        contents.insert(trimmed.to_string());
                    }
                }
                in_native_text = false;
            }
            _ => {}
        }
    }
    contents
}

fn is_fallback_text(e: &quick_xml::events::BytesStart<'_>) -> bool {
    e.try_get_attribute("class")
        .ok()
        .flatten()
        .and_then(|a| a.unescape_value().ok())
        .is_some_and(|v| v.split_whitespace().any(|c| c == FALLBACK_TEXT_CLASS))
}

struct StripForeignObject<'a, I> {
    inner: I,
    /// Depth inside a `<foreignObject>` element being stripped.
    foreign_depth: usize,
    /// Trimmed contents of native `<text>` elements; fallback groups whose label
    /// matches one of these are dropped as duplicates.
    native_text_contents: HashSet<String>,
    /// Buffered events of the fallback group currently being inspected, plus the
    /// nesting depth within it and its accumulated text content.
    buffer: Vec<Event<'a>>,
    fallback_depth: usize,
    buffered_text: String,
    /// Events ready to emit (a flushed, non-duplicate fallback group).
    output: VecDeque<Event<'a>>,
}

impl<'a, I: Iterator<Item = Result<Event<'a>>>> Iterator for StripForeignObject<'a, I> {
    type Item = Result<Event<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(event) = self.output.pop_front() {
                return Some(Ok(event));
            }

            let event = match self.inner.next()? {
                Ok(event) => event,
                Err(e) => return Some(Err(e)),
            };

            // Strip foreignObject elements and their contents (defensive: merman
            // already removes them, but a stray one cannot be rasterized).
            match &event {
                Event::Start(e) if e.name().as_ref() == b"foreignObject" => {
                    self.foreign_depth += 1;
                    continue;
                }
                Event::Start(_) if self.foreign_depth > 0 => {
                    self.foreign_depth += 1;
                    continue;
                }
                Event::End(_) if self.foreign_depth > 0 => {
                    self.foreign_depth -= 1;
                    continue;
                }
                Event::Empty(e) if e.name().as_ref() == b"foreignObject" => {
                    continue;
                }
                _ if self.foreign_depth > 0 => {
                    continue;
                }
                _ => {}
            }

            if self.fallback_depth > 0 {
                self.buffer_fallback_event(event);
                continue;
            }

            // Start buffering a fallback group so we can decide whether it is a
            // duplicate of a native label once we have seen its text.
            if let Event::Start(e) = &event {
                if e.name().as_ref() == b"g" && is_fallback_group(e) {
                    self.fallback_depth = 1;
                    self.buffered_text.clear();
                    self.buffer.push(event);
                    continue;
                }
            }

            return Some(Ok(event));
        }
    }
}

impl<'a, I> StripForeignObject<'a, I> {
    fn buffer_fallback_event(&mut self, event: Event<'a>) {
        match &event {
            Event::Start(_) => self.fallback_depth += 1,
            Event::End(_) => self.fallback_depth = self.fallback_depth.saturating_sub(1),
            Event::Text(t) => {
                if let Ok(decoded) = t.decode() {
                    self.buffered_text.push_str(&decoded);
                }
            }
            _ => {}
        }
        self.buffer.push(event);

        if self.fallback_depth == 0 {
            let is_duplicate = self
                .native_text_contents
                .contains(self.buffered_text.trim());
            let group = std::mem::take(&mut self.buffer);
            if !is_duplicate {
                self.output.extend(group);
            }
        }
    }
}

fn is_fallback_group(e: &quick_xml::events::BytesStart<'_>) -> bool {
    e.try_get_attribute("data-merman-foreignobject")
        .ok()
        .flatten()
        .is_some_and(|attr| attr.value.as_ref() == b"fallback")
}

pub(super) fn process<'a>(
    inner: impl Iterator<Item = Result<Event<'a>>>,
    svg: &str,
) -> impl Iterator<Item = Result<Event<'a>>> {
    // if there's no foreignobjects,
    let native_text_contents = if svg.contains("data-merman-foreignobject=\"fallback\"") {
        collect_native_text_contents(svg)
    } else {
        HashSet::new()
    };
    StripForeignObject {
        inner,
        foreign_depth: 0,
        native_text_contents,
        buffer: Vec::new(),
        fallback_depth: 0,
        buffered_text: String::new(),
        output: VecDeque::new(),
    }
}
