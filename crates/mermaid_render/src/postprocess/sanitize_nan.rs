//! Replaces `hsl(...)` color values containing `NaN` with `transparent`.
//!
//! Merman sometimes emits invalid HSL values that `usvg`/`resvg` cannot handle.
//!
//! ```xml
//! <!-- before -->
//! <rect fill="hsl(240, 100%, NaN%)"/>
//!
//! <!-- after -->
//! <rect fill="transparent"/>
//! ```

use anyhow::{Context as _, Result};
use quick_xml::events::{BytesStart, BytesText, Event};

const COLOR_ATTRS: &[&[u8]] = &[
    b"fill",
    b"stroke",
    b"color",
    b"stop-color",
    b"flood-color",
    b"lighting-color",
    b"style",
];

pub(super) fn process<'a>(
    events: impl Iterator<Item = Result<Event<'a>>>,
) -> impl Iterator<Item = Result<Event<'a>>> {
    SanitizeNan {
        inner: events,
        in_style: false,
    }
}

struct SanitizeNan<I> {
    inner: I,
    in_style: bool,
}

impl<'a, I: Iterator<Item = Result<Event<'a>>>> Iterator for SanitizeNan<I> {
    type Item = Result<Event<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        let event = match self.inner.next()? {
            Ok(event) => event,
            Err(e) => return Some(Err(e)),
        };
        Some(self.process_event(event))
    }
}

impl<'a, I> SanitizeNan<I> {
    fn process_event(&mut self, event: Event<'a>) -> Result<Event<'a>> {
        match event {
            Event::Start(ref e) if e.name().as_ref() == b"style" => {
                self.in_style = true;
                Ok(event)
            }
            Event::End(ref e) if e.name().as_ref() == b"style" => {
                self.in_style = false;
                Ok(event)
            }

            Event::Start(ref e) | Event::Empty(ref e) => {
                if let Some(sanitized) = try_sanitize_element_attrs(e)? {
                    Ok(match event {
                        Event::Start(_) => Event::Start(sanitized),
                        _ => Event::Empty(sanitized),
                    })
                } else {
                    Ok(event)
                }
            }

            Event::Text(ref text) if self.in_style => {
                let raw = std::str::from_utf8(text.as_ref())
                    .context("non-UTF-8 in <style> text")?;
                match sanitize_hsl_nan(raw) {
                    Some(fixed) => Ok(Event::Text(BytesText::from_escaped(fixed))),
                    None => Ok(event),
                }
            }

            _ => Ok(event),
        }
    }
}

/// Returns `Some(sanitized)` if any color attribute contained NaN, `None` otherwise.
fn try_sanitize_element_attrs<'a>(e: &BytesStart<'a>) -> Result<Option<BytesStart<'a>>> {
    let has_nan = e.attributes().flatten().any(|attr| {
        COLOR_ATTRS.contains(&attr.key.local_name().as_ref())
            && attr.value.as_ref().windows(3).any(|w| w == b"NaN")
    });
    if !has_nan {
        return Ok(None);
    }
    let name = e.name();
    let tag_name =
        std::str::from_utf8(name.as_ref()).context("non-UTF-8 tag name")?;
    let mut new_elem = BytesStart::new(tag_name.to_string());
    for attr in e.attributes() {
        let attr = attr.context("malformed attribute")?;
        if COLOR_ATTRS.contains(&attr.key.local_name().as_ref()) {
            let val = attr.unescape_value().context("attribute value")?;
            match sanitize_hsl_nan(&val) {
                Some(fixed) => {
                    let key = std::str::from_utf8(attr.key.as_ref())
                        .context("non-UTF-8 attribute key")?;
                    new_elem.push_attribute((key, fixed.as_str()));
                }
                None => new_elem.push_attribute(attr),
            }
        } else {
            new_elem.push_attribute(attr);
        }
    }
    Ok(Some(new_elem))
}

fn sanitize_hsl_nan(value: &str) -> Option<String> {
    if !value.contains("NaN") {
        return None;
    }

    let mut result = String::with_capacity(value.len());
    let mut remaining = value;
    let mut modified = false;

    while let Some(hsl_pos) = remaining.find("hsl") {
        result.push_str(&remaining[..hsl_pos]);
        remaining = &remaining[hsl_pos..];

        if let Some(paren_end) = remaining.find(')') {
            let hsl_call = &remaining[..paren_end + 1];
            if hsl_call.contains("NaN") {
                result.push_str("transparent");
                modified = true;
            } else {
                result.push_str(hsl_call);
            }
            remaining = &remaining[paren_end + 1..];
        } else {
            result.push_str(&remaining[..3]);
            remaining = &remaining[3..];
        }
    }
    result.push_str(remaining);

    if modified {
        Some(result)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn replaces_nan_hsl_with_transparent() {
        let input = "hsl(240, 100%, NaN%)";
        assert_eq!(
            sanitize_hsl_nan(input),
            Some("transparent".to_string())
        );
    }

    #[test]
    fn preserves_valid_hsl() {
        let input = "hsl(240, 100%, 50%)";
        assert_eq!(sanitize_hsl_nan(input), None);
    }

    #[test]
    fn handles_nan_in_style_attribute() {
        let input = "fill: hsl(240, NaN%, 50%); stroke: #000;";
        assert_eq!(
            sanitize_hsl_nan(input),
            Some("fill: transparent; stroke: #000;".to_string())
        );
    }

    #[test]
    fn preserves_text_without_nan() {
        assert_eq!(sanitize_hsl_nan("hello world"), None);
    }
}
