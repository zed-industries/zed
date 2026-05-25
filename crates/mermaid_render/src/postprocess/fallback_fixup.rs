//! Fixes double-escaped HTML entities inside fallback `<text>` groups that
//! were generated as replacements for `<foreignObject>` content.
//!
//! ```xml
//! <!-- before -->
//! <g data-merman-foreignobject="fallback">
//!   <text>List&amp;lt;T&amp;gt;</text>
//! </g>
//!
//! <!-- after -->
//! <g data-merman-foreignobject="fallback">
//!   <text>List&lt;T&gt;</text>
//! </g>
//! ```

use std::collections::VecDeque;

use anyhow::Result;

use quick_xml::events::{BytesStart, BytesText, Event};

use crate::MermaidTheme;

struct FallbackFixup<'a, I> {
    inner: I,
    edge_label_bg: String,
    fallback_depth: usize,
    text_buffer: String,
    output_queue: VecDeque<Event<'a>>,
}

impl<'a, I: Iterator<Item = Result<Event<'a>>>> Iterator for FallbackFixup<'a, I> {
    type Item = Result<Event<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(event) = self.output_queue.pop_front() {
            return Some(Ok(event));
        }

        loop {
            let event = match self.inner.next()? {
                Ok(ev) => ev,
                Err(e) => return Some(Err(e)),
            };

            match &event {
                Event::Start(e) if e.name().as_ref() == b"g" => {
                    if self.fallback_depth > 0 {
                        self.fallback_depth += 1;
                    } else {
                        match e.try_get_attribute("data-merman-foreignobject") {
                            Ok(Some(attr)) if attr.value.as_ref() == b"fallback" => {
                                self.fallback_depth = 1;
                            }
                            Err(e) => return Some(Err(e.into())),
                            _ => {}
                        }
                    }
                }
                Event::End(e) if e.name().as_ref() == b"g" && self.fallback_depth > 0 => {
                    self.flush_text_buffer();
                    self.fallback_depth -= 1;
                }
                _ => {}
            }

            if self.fallback_depth == 0 {
                return Some(Ok(event));
            }

            // Inside fallback group: accumulate text-like events, process others
            match &event {
                Event::Text(t) => {
                    match std::str::from_utf8(t.as_ref()) {
                        Ok(raw) => self.text_buffer.push_str(raw),
                        Err(e) => eprintln!("Invalid UTF-8 in fallback text: {e}"),
                    }
                    continue;
                }
                Event::GeneralRef(r) => {
                    self.text_buffer.push('&');
                    match std::str::from_utf8(r.as_ref()) {
                        Ok(name) => self.text_buffer.push_str(name),
                        Err(e) => eprintln!("Invalid UTF-8 in fallback entity ref: {e}"),
                    }
                    self.text_buffer.push(';');
                    continue;
                }
                _ => {}
            }

            self.flush_text_buffer();

            match self.process_non_text_event(event) {
                Ok(ev) => self.output_queue.push_back(ev),
                Err(e) => return Some(Err(e)),
            }

            if let Some(event) = self.output_queue.pop_front() {
                return Some(Ok(event));
            }
        }
    }
}

impl<'a, I> FallbackFixup<'a, I> {
    fn flush_text_buffer(&mut self) {
        if self.text_buffer.is_empty() {
            return;
        }
        let text = if self.text_buffer.contains("&amp;lt;") || self.text_buffer.contains("&amp;gt;")
        {
            let fixed = self
                .text_buffer
                .replace("&amp;lt;", "&lt;")
                .replace("&amp;gt;", "&gt;");
            self.text_buffer.clear();
            fixed
        } else {
            std::mem::take(&mut self.text_buffer)
        };
        self.output_queue
            .push_back(Event::Text(BytesText::from_escaped(text)));
    }

    fn process_non_text_event(&self, event: Event<'a>) -> Result<Event<'a>> {
        let is_start = matches!(event, Event::Start(_));
        match &event {
            Event::Start(e) | Event::Empty(e) if e.name().as_ref() == b"rect" => {
                let mut new_elem = BytesStart::new("rect");
                for attr in e.attributes() {
                    let attr = attr?;
                    if attr.key.local_name().as_ref() == b"fill" {
                        new_elem.push_attribute(("fill", self.edge_label_bg.as_str()));
                    } else {
                        new_elem.push_attribute(attr);
                    }
                }
                Ok(if is_start {
                    Event::Start(new_elem)
                } else {
                    Event::Empty(new_elem)
                })
            }
            _ => Ok(event),
        }
    }
}

pub(super) fn process<'a>(
    events: impl Iterator<Item = Result<Event<'a>>>,
    theme: &MermaidTheme,
) -> impl Iterator<Item = Result<Event<'a>>> {
    let edge_label_bg = crate::css_color(theme.edge_label_background);
    FallbackFixup {
        inner: events,
        edge_label_bg,
        fallback_depth: 0,
        text_buffer: String::new(),
        output_queue: VecDeque::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quick_xml::Reader;

    fn run_fixup(svg: &str) -> String {
        let reader = Reader::from_str(svg);
        let events = std::iter::from_fn({
            let mut reader = reader;
            let mut done = false;
            move || {
                if done {
                    return None;
                }
                match reader.read_event() {
                    Ok(quick_xml::events::Event::Eof) => {
                        done = true;
                        None
                    }
                    Ok(ev) => Some(Ok(ev)),
                    Err(e) => {
                        done = true;
                        Some(Err(e.into()))
                    }
                }
            }
        });
        let theme = crate::MermaidTheme::default();
        let fixed = process(events, &theme);
        let mut writer = quick_xml::Writer::new(Vec::new());
        for ev in fixed {
            writer.write_event(ev.unwrap()).unwrap();
        }
        String::from_utf8(writer.into_inner()).unwrap()
    }

    #[test]
    fn fixes_double_escaped_entities_in_fallback() {
        let svg = r##"<g data-merman-foreignobject="fallback"><text fill="#333">-List&amp;lt;Animal&amp;gt; animals</text></g>"##;
        let result = run_fixup(svg);
        assert!(
            !result.contains("&amp;lt;"),
            "Should fix double-escaped entities, got: {result}"
        );
        assert!(
            result.contains("&lt;"),
            "Should contain single-escaped entity, got: {result}"
        );
    }

    #[test]
    fn preserves_text_outside_fallback_group() {
        let svg = r##"<text>-List&amp;lt;Animal&amp;gt;</text>"##;
        let result = run_fixup(svg);
        assert!(
            result.contains("&amp;lt;"),
            "Should not fix entities outside fallback group, got: {result}"
        );
    }
}
