//! Fixes double-escaped HTML entities inside fallback `<text>` groups that
//! were generated as replacements for `<foreignObject>` content.
//!
//! ```xml
//! <!-- before -->
//! <g data-merman-foreignobject="fallback">
//!   <text>List&amp;lt;T&amp;gt; &amp;apos;x&amp;apos;</text>
//! </g>
//!
//! <!-- after -->
//! <g data-merman-foreignobject="fallback">
//!   <text>List&lt;T&gt; &apos;x&apos;</text>
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
        let text = fix_double_escaped_entities(std::mem::take(&mut self.text_buffer));
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

fn fix_double_escaped_entities(input: String) -> String {
    const PREFIX: &str = "&amp;";
    if !input.contains(PREFIX) {
        return input;
    }

    let mut out = String::with_capacity(input.len());
    let mut rest = input.as_str();
    while let Some(idx) = rest.find(PREFIX) {
        let after = &rest[idx + PREFIX.len()..];
        if let Some(entity_len) = recognized_entity_len(after) {
            out.push_str(&rest[..idx]);
            out.push('&');
            out.push_str(&after[..entity_len]);
            rest = &after[entity_len..];
        } else {
            let consumed = idx + PREFIX.len();
            out.push_str(&rest[..consumed]);
            rest = &rest[consumed..];
        }
    }
    out.push_str(rest);
    out
}

fn recognized_entity_len(s: &str) -> Option<usize> {
    for body in ["lt;", "gt;", "amp;", "apos;", "quot;"] {
        if s.starts_with(body) {
            return Some(body.len());
        }
    }

    let bytes = s.as_bytes();
    if bytes.first() != Some(&b'#') {
        return None;
    }
    let mut i = 1;
    let hex = matches!(bytes.get(i), Some(b'x' | b'X'));
    if hex {
        i += 1;
    }
    let digits_start = i;
    while let Some(&b) = bytes.get(i) {
        let is_digit = if hex {
            b.is_ascii_hexdigit()
        } else {
            b.is_ascii_digit()
        };
        if is_digit {
            i += 1;
        } else {
            break;
        }
    }
    if i == digits_start || bytes.get(i) != Some(&b';') {
        return None;
    }
    Some(i + 1)
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
        assert_eq!(
            run_fixup(svg),
            r##"<g data-merman-foreignobject="fallback"><text fill="#333">-List&lt;Animal&gt; animals</text></g>"##
        );
    }

    #[test]
    fn preserves_text_outside_fallback_group() {
        let svg = r##"<text>-List&amp;lt;Animal&amp;gt;</text>"##;
        assert_eq!(run_fixup(svg), svg);
    }

    #[test]
    fn fixes_double_escaped_apostrophes_and_quotes() {
        let svg = r##"<g data-merman-foreignobject="fallback"><text fill="#333">Parent{7, &amp;apos;main.rs&amp;apos;} &amp;quot;x&amp;quot;</text></g>"##;
        assert_eq!(
            run_fixup(svg),
            r##"<g data-merman-foreignobject="fallback"><text fill="#333">Parent{7, &apos;main.rs&apos;} &quot;x&quot;</text></g>"##
        );
    }

    #[test]
    fn fixes_double_escaped_numeric_references() {
        let svg =
            r##"<g data-merman-foreignobject="fallback"><text>a&amp;#39;b&amp;#x2F;c</text></g>"##;
        assert_eq!(
            run_fixup(svg),
            r##"<g data-merman-foreignobject="fallback"><text>a&#39;b&#x2F;c</text></g>"##
        );
    }

    #[test]
    fn fixes_double_escaped_ampersand() {
        let svg =
            r##"<g data-merman-foreignobject="fallback"><text>Tom &amp;amp; Jerry</text></g>"##;
        assert_eq!(
            run_fixup(svg),
            r##"<g data-merman-foreignobject="fallback"><text>Tom &amp; Jerry</text></g>"##
        );
    }

    #[test]
    fn leaves_single_escaped_and_unknown_entities_untouched() {
        // A lone `&amp;` (literal `&`) and an unknown named entity must survive
        // so the SVG stays parseable.
        let svg =
            r##"<g data-merman-foreignobject="fallback"><text>A &amp; B &amp;nbsp; C</text></g>"##;
        assert_eq!(run_fixup(svg), svg);
    }

    #[test]
    fn fix_helper_direct() {
        assert_eq!(
            fix_double_escaped_entities("&amp;apos;x&amp;apos;".to_string()),
            "&apos;x&apos;"
        );
        assert_eq!(
            fix_double_escaped_entities("a&amp;#39;b".to_string()),
            "a&#39;b"
        );
    }

    #[test]
    fn reuses_allocation_when_no_double_escaping_present() {
        let input = String::from("no entities here & there");
        let ptr = input.as_ptr();
        let output = fix_double_escaped_entities(input);
        assert_eq!(output, "no entities here & there");
        assert_eq!(
            output.as_ptr(),
            ptr,
            "unchanged input should not reallocate"
        );
    }
}
