//! Strips `<foreignObject>` elements and their contents from the SVG, since
//! `usvg`/`resvg` does not support them.
//!
//! ```xml
//! <!-- before -->
//! <foreignObject><div>Hello</div></foreignObject>
//! <text class="nodeLabel">Hello</text>
//!
//! <!-- after -->
//! <text class="nodeLabel">Hello</text>
//! ```

use anyhow::Result;
use quick_xml::events::Event;

struct StripForeignObject<I> {
    inner: I,
    /// Depth inside a `<foreignObject>` element being stripped.
    foreign_depth: usize,
    /// Depth inside a `<g data-merman-foreignobject="fallback">` being stripped.
    fallback_depth: usize,
    /// Set to true once we see a `<text>` element outside of foreignObjects
    /// and fallback groups. When true, fallback groups are redundant and
    /// should be stripped.
    has_native_text: bool,
}

impl<'a, I: Iterator<Item = Result<Event<'a>>>> Iterator for StripForeignObject<I> {
    type Item = Result<Event<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let event = self.inner.next()?;
            let event = match event {
                Ok(event) => event,
                Err(e) => return Some(Err(e)),
            };

            // Strip foreignObject elements and their contents.
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

            // Strip fallback groups when native text exists.
            match &event {
                Event::Start(e) if e.name().as_ref() == b"g" && self.fallback_depth == 0 => {
                    if self.has_native_text {
                        if let Ok(Some(attr)) = e.try_get_attribute("data-merman-foreignobject") {
                            if attr.value.as_ref() == b"fallback" {
                                self.fallback_depth = 1;
                                continue;
                            }
                        }
                    }
                }
                Event::Start(_) if self.fallback_depth > 0 => {
                    self.fallback_depth += 1;
                    continue;
                }
                Event::End(_) if self.fallback_depth > 0 => {
                    self.fallback_depth -= 1;
                    continue;
                }
                _ if self.fallback_depth > 0 => {
                    continue;
                }
                _ => {}
            }

            // Track whether the diagram has native <text> elements.
            if !self.has_native_text {
                match &event {
                    Event::Start(e) | Event::Empty(e) if e.name().as_ref() == b"text" => {
                        if e.try_get_attribute("class").ok().flatten().is_some() {
                            self.has_native_text = true;
                        }
                    }
                    _ => {}
                }
            }

            return Some(Ok(event));
        }
    }
}

pub(super) fn process<'a>(
    inner: impl Iterator<Item = Result<Event<'a>>>,
) -> impl Iterator<Item = Result<Event<'a>>> {
    StripForeignObject {
        inner,
        foreign_depth: 0,
        fallback_depth: 0,
        has_native_text: false,
    }
}
