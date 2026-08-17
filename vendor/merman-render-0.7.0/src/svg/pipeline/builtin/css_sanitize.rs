use crate::Result;
use std::borrow::Cow;

use super::util::{find_matching_brace, find_tag_end};
use crate::svg::pipeline::{SvgPostprocessContext, SvgPostprocessor};

#[derive(Debug, Clone, Copy, Default)]
pub struct SanitizeCssPostprocessor;

impl SvgPostprocessor for SanitizeCssPostprocessor {
    fn name(&self) -> &'static str {
        "sanitize-css"
    }

    fn process<'a>(
        &self,
        svg: Cow<'a, str>,
        _ctx: &SvgPostprocessContext<'_>,
    ) -> Result<Cow<'a, str>> {
        if !svg.contains("<style") && !svg.contains("style=\"") {
            return Ok(svg);
        }
        Ok(Cow::Owned(sanitize_style_elements(&svg)))
    }
}

pub(crate) fn sanitize_style_elements(svg: &str) -> String {
    let mut out = String::with_capacity(svg.len());
    let mut cursor = 0;

    while let Some(rel_start) = svg[cursor..].find("<style") {
        let start = cursor + rel_start;
        out.push_str(&svg[cursor..start]);

        let Some(open_end) = find_tag_end(svg, start) else {
            out.push_str(&svg[start..]);
            return out;
        };

        let content_start = open_end + 1;
        let Some(rel_close_start) = svg[content_start..].find("</style") else {
            out.push_str(&svg[start..]);
            return out;
        };
        let close_start = content_start + rel_close_start;
        let Some(close_end) = find_tag_end(svg, close_start) else {
            out.push_str(&svg[start..]);
            return out;
        };

        out.push_str(&svg[start..=open_end]);
        out.push_str(&sanitize_css(&svg[content_start..close_start]));
        out.push_str(&svg[close_start..=close_end]);
        cursor = close_end + 1;
    }

    out.push_str(&svg[cursor..]);
    out
}

pub(crate) fn sanitize_css(css: &str) -> String {
    let css = strip_unsupported_css_rules(css);
    let css = strip_animation_declarations(&css);
    strip_css_deg_units(&css)
}

fn strip_unsupported_css_rules(css: &str) -> String {
    let mut out = String::with_capacity(css.len());
    let mut cursor = 0;

    while let Some(rel_open) = css[cursor..].find('{') {
        let open = cursor + rel_open;
        let selector = &css[cursor..open];
        let Some(close) = find_matching_brace(css, open) else {
            out.push_str(&css[cursor..]);
            return out;
        };

        let selector_lower = selector.to_ascii_lowercase();
        let unsupported = selector_lower.contains("@keyframes")
            || selector_lower.contains("@-webkit-keyframes")
            || selector_lower.contains(":root");

        if !unsupported {
            out.push_str(&css[cursor..=close]);
        }
        cursor = close + 1;
    }

    out.push_str(&css[cursor..]);
    out
}

fn strip_animation_declarations(css: &str) -> String {
    let mut out = String::with_capacity(css.len());
    let mut copied_until = 0usize;
    let mut cursor = 0usize;

    while cursor < css.len() {
        if cursor == 0 {
            if let Some(end) = animation_declaration_end_after_delimiter(css, 0) {
                out.push_str(&css[copied_until..cursor]);
                copied_until = end;
                cursor = end;
                continue;
            }
        }

        let Some(ch) = css[cursor..].chars().next() else {
            break;
        };
        if matches!(ch, ';' | '{') {
            let after_delimiter = cursor + ch.len_utf8();
            if let Some(end) = animation_declaration_end_after_delimiter(css, after_delimiter) {
                out.push_str(&css[copied_until..cursor]);
                out.push(ch);
                copied_until = end;
                cursor = end;
                continue;
            }
        }

        cursor += ch.len_utf8();
    }

    out.push_str(&css[copied_until..]);
    out
}

fn animation_declaration_end_after_delimiter(css: &str, start: usize) -> Option<usize> {
    let mut cursor = skip_css_regex_whitespace(css, start);
    let name_end = cursor + "animation".len();
    if !css.get(cursor..name_end)?.eq_ignore_ascii_case("animation") {
        return None;
    }
    cursor = name_end;

    if css.get(cursor..)?.starts_with('-') {
        let suffix_start = cursor + 1;
        let suffix_end = consume_ascii_alpha_hyphen(css, suffix_start);
        if suffix_end == suffix_start {
            return None;
        }
        cursor = suffix_end;
    }

    cursor = skip_css_regex_whitespace(css, cursor);
    if !css.get(cursor..)?.starts_with(':') {
        return None;
    }
    cursor += 1;

    while let Some(ch) = css.get(cursor..)?.chars().next() {
        if ch == ';' {
            return Some(cursor + 1);
        }
        if ch == '}' {
            return Some(cursor);
        }
        cursor += ch.len_utf8();
    }

    Some(cursor)
}

pub(crate) fn strip_css_deg_units(css: &str) -> String {
    let mut out = String::with_capacity(css.len());
    let mut copied_until = 0usize;
    let mut cursor = 0usize;

    while cursor < css.len() {
        if let Some((number_end, unit_end)) = css_deg_unit_match_at(css, cursor) {
            out.push_str(&css[copied_until..number_end]);
            copied_until = unit_end;
            cursor = unit_end;
            continue;
        }

        let Some(ch) = css[cursor..].chars().next() else {
            break;
        };
        cursor += ch.len_utf8();
    }

    out.push_str(&css[copied_until..]);
    out
}

fn css_deg_unit_match_at(css: &str, start: usize) -> Option<(usize, usize)> {
    let mut cursor = start;
    if css.get(cursor..)?.starts_with('-') {
        cursor += 1;
    }

    let digit_start = cursor;
    cursor = consume_ascii_digits(css, cursor);
    if cursor == digit_start {
        return None;
    }

    if css.get(cursor..)?.starts_with('.') {
        let fraction_start = cursor + 1;
        let fraction_end = consume_ascii_digits(css, fraction_start);
        if fraction_end == fraction_start {
            return None;
        }
        cursor = fraction_end;
    }

    let unit_end = cursor + "deg".len();
    if !css.get(cursor..unit_end)?.eq_ignore_ascii_case("deg") {
        return None;
    }
    if let Some(next) = css.get(unit_end..).and_then(|tail| tail.chars().next()) {
        if is_css_regex_word_char(next) {
            return None;
        }
    }

    Some((cursor, unit_end))
}

fn skip_css_regex_whitespace(css: &str, mut cursor: usize) -> usize {
    while let Some(ch) = css.get(cursor..).and_then(|tail| tail.chars().next()) {
        if !ch.is_whitespace() {
            break;
        }
        cursor += ch.len_utf8();
    }
    cursor
}

fn consume_ascii_alpha_hyphen(css: &str, mut cursor: usize) -> usize {
    while let Some(b) = css.as_bytes().get(cursor) {
        if !(b.is_ascii_alphabetic() || *b == b'-') {
            break;
        }
        cursor += 1;
    }
    cursor
}

fn consume_ascii_digits(css: &str, mut cursor: usize) -> usize {
    while let Some(b) = css.as_bytes().get(cursor) {
        if !b.is_ascii_digit() {
            break;
        }
        cursor += 1;
    }
    cursor
}

fn is_css_regex_word_char(ch: char) -> bool {
    ch == '_' || ch.is_alphanumeric()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn css_sanitize_strips_animation_declarations_without_regex() {
        assert_eq!(
            strip_animation_declarations(".a{ animation: spin 1s; fill:red; }"),
            ".a{ fill:red; }"
        );
        assert_eq!(
            strip_animation_declarations("; animation-duration: 1s; stroke:blue"),
            "; stroke:blue"
        );
        assert_eq!(
            strip_animation_declarations("x animation:spin;"),
            "x animation:spin;"
        );
        assert_eq!(
            strip_animation_declarations("animation-:keep;animation--:drop;fill:red"),
            "animation-:keep;fill:red"
        );
    }

    #[test]
    fn css_sanitize_strips_deg_units_without_regex() {
        assert_eq!(
            strip_css_deg_units(
                "rotate(45deg) rotate(-10.5DEG) 90degree 1.deg .5deg 90deg-foo 90degé"
            ),
            "rotate(45) rotate(-10.5) 90degree 1.deg .5 90-foo 90degé"
        );
    }
}
