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
                let processed = strip_unsupported_css(css_text);
                return Some(Ok(Event::Text(BytesText::from_escaped(processed))));
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

fn strip_unsupported_css(css: &str) -> String {
    let mut result = String::with_capacity(css.len());
    let mut chars = css.char_indices().peekable();

    while let Some(&(i, _)) = chars.peek() {
        let remaining = &css[i..];

        if remaining.starts_with("@keyframes") || remaining.starts_with("@-webkit-keyframes") {
            skip_css_block(&mut chars);
            continue;
        }

        if remaining.starts_with(":root") {
            skip_css_block(&mut chars);
            continue;
        }

        if remaining.starts_with(":not(") {
            for _ in 0..5 {
                chars.next();
            }
            let mut depth = 1u32;
            while let Some((_, c)) = chars.next() {
                if c == '(' {
                    depth += 1;
                }
                if c == ')' {
                    depth -= 1;
                    if depth == 0 {
                        break;
                    }
                }
            }
            continue;
        }

        let (_, ch) = chars.next().expect("peeked successfully above");
        result.push(ch);
    }

    strip_css_angle_units(&mut result);
    result
}

fn skip_css_block(chars: &mut std::iter::Peekable<std::str::CharIndices>) {
    let mut found_brace = false;
    let mut depth = 0u32;
    while let Some((_, c)) = chars.next() {
        if c == '{' {
            found_brace = true;
            depth += 1;
        } else if c == '}' {
            depth = depth.saturating_sub(1);
            if depth == 0 && found_brace {
                return;
            }
        }
    }
}

fn strip_css_angle_units(css: &mut String) {
    while let Some(pos) = css.find("deg)") {
        css.replace_range(pos..pos + 3, "");
    }
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
    fn strips_not_pseudo_selectors() {
        let input = ".node:not(.mindmap-node) rect { fill: red; }";
        let result = strip_unsupported_css(input);
        assert!(!result.contains(":not"), "got: {result}");
        assert!(result.contains(".node rect"), "got: {result}");
    }

    #[test]
    fn strips_deg_units() {
        let input = ".foo { transform: rotate(45deg); }";
        let result = strip_unsupported_css(input);
        assert!(result.contains("rotate(45)"), "got: {result}");
        assert!(!result.contains("deg"), "got: {result}");
    }
}
