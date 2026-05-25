//! Removes CSS constructs that `usvg`/`resvg` cannot handle.
//!
//! - `@keyframes` and `@-webkit-keyframes` blocks
//! - `:root { ... }` blocks (CSS custom properties)
//! - `:not(...)` pseudo-selectors
//! - `deg` angle units (e.g. `rotate(45deg)` → `rotate(45)`)
//!
//! Also removes `!important` declarations (so that our injected theme CSS
//! always wins).

use std::borrow::Cow;

use anyhow::Result;
use quick_xml::events::{BytesText, Event};

struct StripInvalidCss<I> {
    inner: I,
    in_style: bool,
}

impl<'a, I: Iterator<Item = Result<Event<'a>>>> Iterator for StripInvalidCss<I> {
    type Item = Result<Event<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        let event = match self.inner.next()? {
            Ok(ev) => ev,
            Err(e) => return Some(Err(e)),
        };

        match &event {
            Event::Start(e) if e.name().as_ref() == b"style" => {
                self.in_style = true;
            }
            Event::End(e) if e.name().as_ref() == b"style" => {
                self.in_style = false;
            }
            Event::Text(text) if self.in_style => {
                let css_text = match std::str::from_utf8(text.as_ref()) {
                    Ok(s) => s,
                    Err(e) => return Some(Err(e.into())),
                };
                return Some(match strip_unsupported_css(css_text) {
                    Cow::Borrowed(_) => Ok(event),
                    Cow::Owned(processed) => Ok(Event::Text(BytesText::from_escaped(processed))),
                });
            }
            _ => {}
        }

        Some(Ok(event))
    }
}

pub(super) fn process<'a>(
    events: impl Iterator<Item = Result<Event<'a>>>,
) -> impl Iterator<Item = Result<Event<'a>>> {
    StripInvalidCss {
        inner: events,
        in_style: false,
    }
}

fn strip_unsupported_css(css: &str) -> Cow<'_, str> {
    let mut chars = css.char_indices().peekable();
    let mut result = None;
    let mut copied_until = 0;

    while let Some((i, _)) = chars.next() {
        let remaining = &css[i..];

        if remaining.starts_with("@keyframes")
            || remaining.starts_with("@-webkit-keyframes")
            || remaining.starts_with(":root")
        {
            let result = result.get_or_insert_with(|| String::with_capacity(css.len()));
            result.push_str(&css[copied_until..i]);
            skip_css_block(&mut chars);
            copied_until = chars.peek().map_or(css.len(), |&(i, _)| i);
        }
    }

    let mut result = if let Some(mut result) = result {
        result.push_str(&css[copied_until..]);
        Cow::Owned(result)
    } else {
        Cow::Borrowed(css)
    };

    strip_css_angle_units(&mut result);
    strip_css_important(&mut result);
    result
}

fn skip_css_block(chars: &mut std::iter::Peekable<std::str::CharIndices>) {
    for (_, c) in chars.by_ref() {
        if c == '{' {
            break;
        }
    }
    let mut depth = 1u32;
    for (_, c) in chars.by_ref() {
        match c {
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth == 0 {
                    return;
                }
            }
            _ => {}
        }
    }
}

fn replace_all_in_place(css: &mut Cow<'_, str>, needle: &str, replacement: &str) {
    while let Some(pos) = css.as_ref().find(needle) {
        css.to_mut()
            .replace_range(pos..pos + needle.len(), replacement);
    }
}

fn strip_css_angle_units(css: &mut Cow<'_, str>) {
    replace_all_in_place(css, "deg)", ")");
}

/// Strip `!important` from mermaid's generated CSS so that our injected
/// theme CSS (which uses `!important`) always takes priority. This works
/// around a usvg cascade bug where competing `!important` rules are
/// resolved by first-wins rather than the CSS spec's last-wins.
fn strip_css_important(css: &mut Cow<'_, str>) {
    replace_all_in_place(css, "!important", "");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strips_keyframes() {
        let input = "@keyframes bounce { 0% { transform: scale(1); } 100% { transform: scale(1.1); } } .node rect { fill: red; }";
        let result = strip_unsupported_css(input);
        assert!(!result.contains("@keyframes"), "got: {result}");
        assert!(result.contains(".node rect"), "got: {result}");
    }

    #[test]
    fn strips_root_blocks() {
        let input = ":root { --bg: white; } .foo { color: red; }";
        let result = strip_unsupported_css(input);
        assert!(!result.contains(":root"), "got: {result}");
        assert!(result.contains(".foo"), "got: {result}");
    }

    #[test]
    fn strips_deg_units() {
        let input = ".foo { transform: rotate(45deg); }";
        let result = strip_unsupported_css(input);
        assert!(result.contains("rotate(45)"), "got: {result}");
        assert!(!result.contains("deg"), "got: {result}");
    }
}
