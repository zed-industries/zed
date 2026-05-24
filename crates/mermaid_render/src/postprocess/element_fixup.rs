use anyhow::Result;
use quick_xml::events::{BytesStart, Event};

use crate::MermaidTheme;

struct ElementFixup<I> {
    inner: I,
    background_css: String,
    text_color_css: String,
    svg_seen: bool,
    skip_rect_depth: usize,
}

fn is_bad_rect(e: &BytesStart) -> Result<bool> {
    if let Some(w) = e.try_get_attribute("width")? {
        if w.unescape_value()?.is_empty() {
            return Ok(true);
        }
    }
    if let Some(h) = e.try_get_attribute("height")? {
        if h.unescape_value()?.is_empty() {
            return Ok(true);
        }
    }
    Ok(false)
}

impl<'a, I: Iterator<Item = Result<Event<'a>>>> ElementFixup<I> {
    fn rewrite_svg_style(&self, e: &BytesStart<'_>) -> Result<BytesStart<'a>> {
        let mut new_elem = BytesStart::new("svg");
        for attr in e.attributes() {
            let attr = attr?;
            if attr.key.local_name().as_ref() == b"style" {
                let val = attr.unescape_value()?;
                let fixed = val.replace(
                    "background-color: white",
                    &format!("background-color: {}", self.background_css),
                );
                new_elem.push_attribute(("style", fixed.as_str()));
            } else {
                new_elem.push_attribute(attr);
            }
        }
        Ok(new_elem)
    }

    fn fix_text_fill(&self, e: &BytesStart<'_>) -> Result<Option<BytesStart<'a>>> {
        let needs_fix = if let Some(fill_attr) = e.try_get_attribute("fill")? {
            let val = fill_attr.unescape_value()?;
            val.as_ref() == "#333" || val.is_empty()
        } else {
            false
        };
        if !needs_fix {
            return Ok(None);
        }
        let mut new_elem = BytesStart::new("text");
        for attr in e.attributes() {
            let attr = attr?;
            if attr.key.local_name().as_ref() == b"fill" {
                new_elem.push_attribute(("fill", self.text_color_css.as_str()));
            } else {
                new_elem.push_attribute(attr);
            }
        }
        Ok(Some(new_elem))
    }

    fn process_event(&mut self, event: Event<'a>) -> Result<Option<Event<'a>>> {
        match event {
            Event::Start(ref e) if e.name().as_ref() == b"svg" && !self.svg_seen => {
                self.svg_seen = true;
                Ok(Some(Event::Start(self.rewrite_svg_style(e)?)))
            }
            Event::Empty(ref e) if e.name().as_ref() == b"svg" && !self.svg_seen => {
                self.svg_seen = true;
                Ok(Some(Event::Empty(self.rewrite_svg_style(e)?)))
            }

            Event::Start(ref e) if e.name().as_ref() == b"rect" => {
                if is_bad_rect(e)? {
                    self.skip_rect_depth = 1;
                    Ok(None)
                } else {
                    Ok(Some(event))
                }
            }
            Event::Empty(ref e) if e.name().as_ref() == b"rect" => {
                if is_bad_rect(e)? {
                    Ok(None)
                } else {
                    Ok(Some(event))
                }
            }

            Event::Start(ref e) if e.name().as_ref() == b"text" => {
                if let Some(new_elem) = self.fix_text_fill(e)? {
                    Ok(Some(Event::Start(new_elem)))
                } else {
                    Ok(Some(event))
                }
            }
            Event::Empty(ref e) if e.name().as_ref() == b"text" => {
                if let Some(new_elem) = self.fix_text_fill(e)? {
                    Ok(Some(Event::Empty(new_elem)))
                } else {
                    Ok(Some(event))
                }
            }

            _ => Ok(Some(event)),
        }
    }
}

impl<'a, I: Iterator<Item = Result<Event<'a>>>> Iterator for ElementFixup<I> {
    type Item = Result<Event<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let event = match self.inner.next()? {
                Ok(ev) => ev,
                Err(e) => return Some(Err(e)),
            };

            if self.skip_rect_depth > 0 {
                match &event {
                    Event::Start(_) => self.skip_rect_depth += 1,
                    Event::End(_) => self.skip_rect_depth -= 1,
                    _ => {}
                }
                continue;
            }

            match self.process_event(event) {
                Ok(Some(ev)) => return Some(Ok(ev)),
                Ok(None) => continue,
                Err(e) => return Some(Err(e)),
            }
        }
    }
}

pub(super) fn process<'a>(
    events: impl Iterator<Item = Result<Event<'a>>>,
    theme: &MermaidTheme,
) -> impl Iterator<Item = Result<Event<'a>>> {
    ElementFixup {
        inner: events,
        background_css: crate::css_color(theme.background),
        text_color_css: crate::css_color(theme.text_color),
        svg_seen: false,
        skip_rect_depth: 0,
    }
}
