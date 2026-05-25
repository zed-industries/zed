//! Fixes various issues in merman's SVG output.
//!
//! Replaces hardcoded white backgrounds with the theme background:
//! ```xml
//! <!-- before --> <svg style="background-color: white">
//! <!-- after  --> <svg style="background-color: #1e1e2e">
//! ```
//!
//! Removes `<rect>` elements with missing or invalid dimensions:
//! ```xml
//! <!-- before --> <rect width="NaN" height="10"/>
//! <!-- after  --> (removed)
//! ```
//!
//! Replaces hardcoded text colors with the theme text color:
//! ```xml
//! <!-- before --> <text fill="#333">Hello</text>
//! <!-- after  --> <text fill="#cdd6f4">Hello</text>
//! ```

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

fn rewrite_attr<'a>(
    e: &BytesStart<'_>,
    attr_name: &[u8],
    new_value: &str,
) -> Result<BytesStart<'a>> {
    let name = e.name();
    let tag = std::str::from_utf8(name.as_ref())?;
    let mut new_elem = BytesStart::new(tag.to_owned());
    for attr in e.attributes() {
        let attr = attr?;
        if attr.key.local_name().as_ref() == attr_name {
            let local_name = attr.key.local_name();
            let key = std::str::from_utf8(local_name.as_ref())?;
            new_elem.push_attribute((key, new_value));
        } else {
            new_elem.push_attribute(attr);
        }
    }
    Ok(new_elem)
}

fn rewrap<'a>(event: &Event<'_>, elem: BytesStart<'a>) -> Event<'a> {
    match event {
        Event::Start(_) => Event::Start(elem),
        _ => Event::Empty(elem),
    }
}

fn is_bad_rect(e: &BytesStart) -> Result<bool> {
    for attr_name in ["width", "height"] {
        match e.try_get_attribute(attr_name)? {
            None => return Ok(true),
            Some(attr) => {
                let val = attr.unescape_value()?;
                let trimmed = val.trim();
                if trimmed.is_empty() {
                    return Ok(true);
                }
                if let Ok(n) = trimmed.parse::<f64>() {
                    if !n.is_finite() || n <= 0.0 {
                        return Ok(true);
                    }
                }
            }
        }
    }
    Ok(false)
}

impl<'a, I: Iterator<Item = Result<Event<'a>>>> ElementFixup<I> {
    fn rewrite_svg_style(&self, e: &BytesStart<'_>) -> Result<BytesStart<'a>> {
        let style = e
            .try_get_attribute("style")?
            .map(|a| a.unescape_value())
            .transpose()?;
        let new_style = style.as_deref().unwrap_or_default().replace(
            "background-color: white",
            &format!("background-color: {}", self.background_css),
        );
        rewrite_attr(e, b"style", &new_style)
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
        Ok(Some(rewrite_attr(e, b"fill", &self.text_color_css)?))
    }

    fn process_event(&mut self, event: Event<'a>) -> Result<Option<Event<'a>>> {
        match &event {
            Event::Start(e) | Event::Empty(e) if e.name().as_ref() == b"svg" && !self.svg_seen => {
                self.svg_seen = true;
                let new_elem = self.rewrite_svg_style(e)?;
                Ok(Some(rewrap(&event, new_elem)))
            }

            Event::Start(e) | Event::Empty(e) if e.name().as_ref() == b"rect" => {
                if is_bad_rect(e)? {
                    if matches!(event, Event::Start(_)) {
                        self.skip_rect_depth = 1;
                    }
                    Ok(None)
                } else {
                    Ok(Some(event))
                }
            }

            Event::Start(e) | Event::Empty(e) if e.name().as_ref() == b"text" => {
                if let Some(new_elem) = self.fix_text_fill(e)? {
                    Ok(Some(rewrap(&event, new_elem)))
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
