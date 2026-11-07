//! Align Mermaid source with the typography usvg actually paints.
//!
//! merman measures class and flowchart labels only when `themeVariables.fontSize`
//! is a positive px string (or the bare number 16). Other spellings keep a
//! different layout size than the CSS cascade, and properties such as
//! `letter-spacing` are refused for the whole diagram. Rewrite those inputs to
//! the px size and declarations the rasterizer uses, so native metrics stay on
//! the same faces as painting.

use std::borrow::Cow;

use serde_json::{Number, Value};

const ROOT_FONT_SIZE: f64 = 16.0;
const MAX_CSS_NUMERIC_VALUE: f64 = 1_000_000.0;

const UNSUPPORTED_TEXT_PROPERTIES: &[&str] = &[
    "letter-spacing",
    "word-spacing",
    "text-transform",
    "white-space",
    "font",
    "font-variant",
    "font-stretch",
    "font-feature-settings",
    "font-variation-settings",
    "font-kerning",
    "line-height",
    "text-indent",
    "writing-mode",
    "direction",
    "unicode-bidi",
    "text-anchor",
    "dominant-baseline",
    "alignment-baseline",
    "baseline-shift",
    "kerning",
    "text-rendering",
    "font-size-adjust",
];

#[derive(Clone, Copy, PartialEq, Eq)]
enum DiagramFamily {
    Class,
    Flowchart,
}

pub(super) fn prepare_source(source: &str) -> Cow<'_, str> {
    let sized = match diagram_family(source) {
        Some(family) => rewrite_font_sizes(source, family),
        None => Cow::Borrowed(source),
    };
    let stripped = strip_unsupported_styles(sized.as_ref());
    if matches!(&stripped, Cow::Owned(_)) {
        Cow::Owned(stripped.into_owned())
    } else {
        drop(stripped);
        sized
    }
}

fn diagram_family(source: &str) -> Option<DiagramFamily> {
    let mut in_frontmatter = false;
    let mut saw_content = false;
    for line in source.lines() {
        let trimmed = line.trim();
        if !saw_content && trimmed.is_empty() {
            continue;
        }
        if !saw_content && trimmed == "---" {
            in_frontmatter = true;
            saw_content = true;
            continue;
        }
        if in_frontmatter {
            if trimmed == "---" {
                in_frontmatter = false;
            }
            continue;
        }
        if trimmed.is_empty() || trimmed.starts_with("%%") {
            continue;
        }
        saw_content = true;
        let token = trimmed.split_whitespace().next().unwrap_or(trimmed);
        let token = token.trim_end_matches(['{', ':']);
        match token {
            "classDiagram" | "classDiagram-v2" | "class" => return Some(DiagramFamily::Class),
            "flowchart" | "flowchart-v2" | "flowchart-elk" | "graph" | "agentflow" => {
                return Some(DiagramFamily::Flowchart);
            }
            _ => {}
        }
    }
    None
}

fn rewrite_font_sizes(source: &str, family: DiagramFamily) -> Cow<'_, str> {
    let with_init = rewrite_init_directives(source, family);
    let frontmatter = rewrite_frontmatter(with_init.as_ref(), family);
    if matches!(&frontmatter, Cow::Owned(_)) {
        Cow::Owned(frontmatter.into_owned())
    } else {
        drop(frontmatter);
        with_init
    }
}

fn rewrite_init_directives(source: &str, family: DiagramFamily) -> Cow<'_, str> {
    let mut output = String::new();
    let mut cursor = 0;
    let mut changed = false;
    while let Some(relative) = source[cursor..].find("%%{") {
        let marker = cursor + relative;
        let Some((json_start, json_end)) = init_json_span(source, marker) else {
            output.push_str(&source[cursor..marker + 3]);
            cursor = marker + 3;
            continue;
        };
        output.push_str(&source[cursor..json_start]);
        let json_text = &source[json_start..json_end];
        if let Some(rewritten) = rewrite_init_json(json_text, family) {
            output.push_str(&rewritten);
            changed = true;
        } else {
            output.push_str(json_text);
        }
        cursor = json_end;
    }
    if !changed {
        return Cow::Borrowed(source);
    }
    output.push_str(&source[cursor..]);
    Cow::Owned(output)
}

fn init_json_span(source: &str, marker: usize) -> Option<(usize, usize)> {
    let mut index = skip_ascii_whitespace(source, marker + "%%{".len());
    let keyword_len = if source[index..].starts_with("initialize") {
        "initialize".len()
    } else if source[index..].starts_with("init") {
        "init".len()
    } else {
        return None;
    };
    let after_keyword = index + keyword_len;
    if source
        .as_bytes()
        .get(after_keyword)
        .is_some_and(|byte| byte.is_ascii_alphanumeric() || *byte == b'_')
    {
        return None;
    }
    index = skip_ascii_whitespace(source, after_keyword);
    if source.as_bytes().get(index) != Some(&b':') {
        return None;
    }
    index = skip_ascii_whitespace(source, index + 1);
    if source.as_bytes().get(index) != Some(&b'{') {
        return None;
    }
    let json_end = matching_brace(source, index)?;
    Some((index, json_end))
}

fn matching_brace(source: &str, open: usize) -> Option<usize> {
    let bytes = source.as_bytes();
    let mut depth = 0i32;
    let mut quote: Option<u8> = None;
    let mut escaped = false;
    let mut index = open;
    while index < bytes.len() {
        let byte = bytes[index];
        if let Some(current) = quote {
            if escaped {
                escaped = false;
            } else if byte == b'\\' {
                escaped = true;
            } else if byte == current {
                quote = None;
            }
            index += 1;
            continue;
        }
        match byte {
            b'"' => quote = Some(byte),
            b'{' => depth += 1,
            b'}' => {
                depth -= 1;
                if depth == 0 {
                    return Some(index + 1);
                }
            }
            _ => {}
        }
        index += 1;
    }
    None
}

fn rewrite_init_json(json_text: &str, family: DiagramFamily) -> Option<String> {
    let mut value: Value = serde_json::from_str(json_text).ok()?;
    if !rewrite_theme_font_size(&mut value, family) {
        return None;
    }
    serde_json::to_string(&value).ok()
}

fn rewrite_theme_font_size(value: &mut Value, family: DiagramFamily) -> bool {
    let Some(theme_variables) = value.get_mut("themeVariables") else {
        return false;
    };
    let Some(size) = theme_variables.get("fontSize").cloned() else {
        return false;
    };
    if theme_font_size_is_admitted(&size, family) {
        return false;
    }
    let Some(theme_variables) = theme_variables.as_object_mut() else {
        return false;
    };
    theme_variables.insert(
        "fontSize".to_string(),
        Value::String(format_px(painted_font_size(&size, family))),
    );
    true
}

fn theme_font_size_is_admitted(size: &Value, family: DiagramFamily) -> bool {
    if size.as_f64() == Some(ROOT_FONT_SIZE) {
        return true;
    }
    let Some(text) = size.as_str() else {
        return false;
    };
    let text = text.trim();
    let text = if family == DiagramFamily::Class {
        text.trim_end_matches(';')
            .trim()
            .trim_end_matches("!important")
            .trim()
    } else {
        text
    };
    text.strip_suffix("px")
        .and_then(|number| number.trim().parse::<f64>().ok())
        .is_some_and(|number| number.is_finite() && number >= 1.0)
}

fn painted_font_size(size: &Value, family: DiagramFamily) -> f64 {
    match family {
        DiagramFamily::Class => class_painted_font_size(size),
        DiagramFamily::Flowchart => flowchart_painted_font_size(size),
    }
}

fn class_painted_font_size(size: &Value) -> f64 {
    resolve_painted_font_size(css_text(size).as_deref(), ROOT_FONT_SIZE, ROOT_FONT_SIZE)
        .unwrap_or(ROOT_FONT_SIZE)
        .max(1.0)
}

fn flowchart_painted_font_size(size: &Value) -> f64 {
    parse_leading_font_size(size)
        .unwrap_or(ROOT_FONT_SIZE)
        .max(1.0)
}

fn css_text(size: &Value) -> Option<String> {
    if let Some(raw) = size.as_str() {
        let text = raw.trim().trim_end_matches(';').trim();
        let text = text.trim_end_matches("!important").trim();
        return (!text.is_empty()).then(|| text.to_string());
    }
    let number = size.as_f64()?;
    number.is_finite().then(|| format_number(number))
}

fn parse_leading_font_size(size: &Value) -> Option<f64> {
    if let Some(number) = size.as_f64() {
        return Some(number);
    }
    let text = size.as_str()?.trim();
    if text.is_empty() {
        return None;
    }
    let mut number = String::new();
    for (index, character) in text.chars().enumerate() {
        if character.is_ascii_digit() {
            number.push(character);
            continue;
        }
        if index == 0 && (character == '-' || character == '+') {
            number.push(character);
            continue;
        }
        break;
    }
    if number.trim().is_empty() {
        return None;
    }
    number.parse().ok()
}

fn resolve_painted_font_size(value: Option<&str>, parent: f64, root: f64) -> Option<f64> {
    let value = value?.trim().to_ascii_lowercase();
    if matches!(value.as_str(), "inherit" | "unset") {
        return finite_positive(parent);
    }
    if value == "initial" {
        return finite_positive(ROOT_FONT_SIZE);
    }
    if let Some(number) = value.strip_suffix("px").and_then(parse_css_number) {
        return bounded_positive(number);
    }
    if let Some(number) = value.strip_suffix("rem").and_then(parse_css_number) {
        return bounded_positive(root * number);
    }
    if let Some(number) = value.strip_suffix("em").and_then(parse_css_number) {
        return bounded_positive(parent * number);
    }
    if let Some(number) = value.strip_suffix('%').and_then(parse_css_number) {
        return bounded_positive(parent * number / 100.0);
    }
    match value.as_str() {
        "xx-small" => finite_positive(9.0),
        "x-small" => finite_positive(10.0),
        "small" => finite_positive(13.0),
        "medium" => finite_positive(16.0),
        "large" => finite_positive(18.0),
        "x-large" => finite_positive(24.0),
        "xx-large" => finite_positive(32.0),
        "smaller" => finite_positive(parent * 0.833_333_333_3),
        "larger" => finite_positive(parent * 1.2),
        _ => None,
    }
}

fn parse_css_number(value: &str) -> Option<f64> {
    value.trim().parse().ok()
}

fn finite_positive(value: f64) -> Option<f64> {
    value
        .is_finite()
        .then_some(value)
        .filter(|value| *value > 0.0)
}

fn bounded_positive(value: f64) -> Option<f64> {
    finite_positive(value).filter(|value| *value <= MAX_CSS_NUMERIC_VALUE)
}

fn format_px(value: f64) -> String {
    format!("{}px", format_number(value))
}

fn format_number(value: f64) -> String {
    if !value.is_finite() {
        return "0".to_string();
    }
    let nearest = value.round();
    if (value - nearest).abs() < 1e-6 && nearest.abs() < 1e15 {
        return (nearest as i64).to_string();
    }
    let mut text = format!("{value:.8}");
    while text.contains('.') && (text.ends_with('0') || text.ends_with('.')) {
        text.pop();
    }
    text
}

fn rewrite_frontmatter(source: &str, family: DiagramFamily) -> Cow<'_, str> {
    let Some(frontmatter_end) = frontmatter_end(source) else {
        return Cow::Borrowed(source);
    };
    let frontmatter = &source[..frontmatter_end];
    let rewritten = rewrite_frontmatter_body(frontmatter, family);
    if rewritten == frontmatter {
        return Cow::Borrowed(source);
    }
    let mut output = rewritten;
    output.push_str(&source[frontmatter_end..]);
    Cow::Owned(output)
}

fn frontmatter_end(source: &str) -> Option<usize> {
    let mut offset = 0;
    let mut saw_opening = false;
    for line in source.split_inclusive('\n') {
        let content = line.trim_end_matches(['\n', '\r']);
        if content.trim() == "---" {
            if !saw_opening {
                if offset == 0 || source[..offset].trim().is_empty() {
                    saw_opening = true;
                    offset += line.len();
                    continue;
                }
                return None;
            }
            return Some(offset + line.len());
        }
        if !saw_opening && !content.trim().is_empty() {
            return None;
        }
        offset += line.len();
    }
    None
}

fn rewrite_frontmatter_body(frontmatter: &str, family: DiagramFamily) -> String {
    let mut output = String::with_capacity(frontmatter.len());
    let mut theme_indent: Option<usize> = None;
    for line in frontmatter.split_inclusive('\n') {
        let content = line.trim_end_matches(['\n', '\r']);
        let newline = &line[content.len()..];
        let trimmed = content.trim();
        let indent = content.len() - content.trim_start().len();
        if let Some(current) = theme_indent {
            let leaves_theme = !trimmed.is_empty()
                && !trimmed.starts_with('#')
                && indent <= current
                && !trimmed.starts_with("fontSize");
            if leaves_theme {
                theme_indent = None;
            }
        }
        if trimmed == "themeVariables:" || trimmed.starts_with("themeVariables:") {
            theme_indent = Some(indent);
        }
        if theme_indent.is_some_and(|current| indent > current) && trimmed.starts_with("fontSize:")
        {
            output.push_str(&rewrite_yaml_font_size_line(content, family));
            output.push_str(newline);
            continue;
        }
        output.push_str(line);
    }
    output
}

fn rewrite_yaml_font_size_line(content: &str, family: DiagramFamily) -> String {
    let trimmed = content.trim_start();
    let indent_len = content.len() - trimmed.len();
    let Some(after_key) = trimmed.strip_prefix("fontSize") else {
        return content.to_string();
    };
    let Some(after_colon) = after_key.trim_start().strip_prefix(':') else {
        return content.to_string();
    };
    let (raw_value, suffix) = split_yaml_scalar(after_colon.trim_start());
    if raw_value.is_empty() {
        return content.to_string();
    }
    let json = yaml_scalar_to_json(raw_value);
    if theme_font_size_is_admitted(&json, family) {
        return content.to_string();
    }
    format!(
        "{}fontSize: {}{}",
        &content[..indent_len],
        format_px(painted_font_size(&json, family)),
        suffix
    )
}

fn split_yaml_scalar(text: &str) -> (&str, &str) {
    let mut chars = text.char_indices();
    let Some((_, first)) = chars.next() else {
        return ("", "");
    };
    if first == '"' || first == '\'' {
        let mut escaped = false;
        for (index, character) in chars {
            if escaped {
                escaped = false;
                continue;
            }
            if character == '\\' && first == '"' {
                escaped = true;
                continue;
            }
            if character == first {
                let end = index + character.len_utf8();
                return (&text[..end], &text[end..]);
            }
        }
        return (text, "");
    }
    let end = text
        .find(|character: char| character.is_whitespace() || character == '#')
        .unwrap_or(text.len());
    (&text[..end], &text[end..])
}

fn yaml_scalar_to_json(raw: &str) -> Value {
    if raw.len() >= 2 {
        let bytes = raw.as_bytes();
        let quote = bytes[0];
        if (quote == b'"' || quote == b'\'') && bytes[raw.len() - 1] == quote {
            return Value::String(raw[1..raw.len() - 1].to_string());
        }
    }
    if let Ok(number) = raw.parse::<f64>()
        && number.is_finite()
        && let Some(number) = Number::from_f64(number)
    {
        return Value::Number(number);
    }
    Value::String(raw.to_string())
}

fn strip_unsupported_styles(source: &str) -> Cow<'_, str> {
    if !source.contains("style") && !source.contains("Style") && !source.contains("classDef") {
        return Cow::Borrowed(source);
    }
    let mut output = String::with_capacity(source.len());
    let mut changed = false;
    for line in source.split_inclusive('\n') {
        let rewritten = rewrite_style_line(line);
        if rewritten.as_ref() != line {
            changed = true;
        }
        output.push_str(rewritten.as_ref());
    }
    if changed {
        Cow::Owned(output)
    } else {
        Cow::Borrowed(source)
    }
}

fn rewrite_style_line(line: &str) -> Cow<'_, str> {
    let content = line.trim_end_matches(['\n', '\r']);
    let newline = &line[content.len()..];
    let trimmed = content.trim_start();
    if trimmed.starts_with("%%") {
        return Cow::Borrowed(line);
    }
    if is_style_directive(trimmed) {
        let filtered = remove_unsupported_declarations(content);
        let filtered_trimmed = filtered.trim();
        if filtered_trimmed.contains(':') {
            if filtered == content {
                Cow::Borrowed(line)
            } else {
                Cow::Owned(format!("{filtered}{newline}"))
            }
        } else {
            Cow::Owned(newline.to_string())
        }
    } else if content.contains("style") {
        let filtered = strip_html_style_attributes(content);
        if filtered == content {
            Cow::Borrowed(line)
        } else {
            Cow::Owned(format!("{filtered}{newline}"))
        }
    } else {
        Cow::Borrowed(line)
    }
}

fn is_style_directive(trimmed: &str) -> bool {
    trimmed.starts_with("style ")
        || trimmed.starts_with("classDef ")
        || trimmed.starts_with("linkStyle ")
}

fn remove_unsupported_declarations(text: &str) -> String {
    let mut output = String::with_capacity(text.len());
    let mut cursor = 0;
    while let Some((property_start, value_end)) = next_unsupported_declaration(text, cursor) {
        let bytes = text.as_bytes();
        let mut copy_to = property_start;
        while copy_to > cursor && bytes[copy_to - 1].is_ascii_whitespace() {
            copy_to -= 1;
        }
        let had_separator_before = copy_to > cursor && matches!(bytes[copy_to - 1], b',' | b';');
        let mut next = value_end;
        while next < bytes.len() && bytes[next].is_ascii_whitespace() {
            next += 1;
        }
        let has_separator_after = next < bytes.len() && matches!(bytes[next], b',' | b';');
        if had_separator_before && has_separator_after {
            output.push_str(&text[cursor..copy_to - 1]);
            cursor = next;
        } else if has_separator_after {
            output.push_str(&text[cursor..copy_to]);
            cursor = next + 1;
        } else if had_separator_before {
            output.push_str(&text[cursor..copy_to - 1]);
            cursor = next;
        } else {
            output.push_str(&text[cursor..copy_to]);
            cursor = next;
        }
    }
    output.push_str(&text[cursor..]);
    output
}

fn next_unsupported_declaration(text: &str, from: usize) -> Option<(usize, usize)> {
    let bytes = text.as_bytes();
    let mut index = from;
    let mut quote: Option<u8> = None;
    let mut parentheses = 0i32;
    while index < bytes.len() {
        let byte = bytes[index];
        if let Some(current) = quote {
            if byte == current {
                quote = None;
            }
            index += 1;
            continue;
        }
        match byte {
            b'\'' | b'"' => {
                quote = Some(byte);
                index += 1;
            }
            b'(' => {
                parentheses += 1;
                index += 1;
            }
            b')' => {
                parentheses -= 1;
                index += 1;
            }
            b':' if parentheses == 0 => {
                if let Some((start, name)) = property_ident(text, index) {
                    let value_end = declaration_value_end(text, index + 1);
                    if is_unsupported_property(name) {
                        return Some((start, value_end));
                    }
                    index = value_end;
                    continue;
                }
                index += 1;
            }
            _ => index += 1,
        }
    }
    None
}

fn property_ident(text: &str, colon: usize) -> Option<(usize, &str)> {
    let head = text[..colon].trim_end();
    if head.is_empty() {
        return None;
    }
    let end = head.len();
    let start = head
        .char_indices()
        .rev()
        .find(|(_, character)| !is_css_ident(*character))
        .map(|(index, character)| index + character.len_utf8())
        .unwrap_or(0);
    if start >= end {
        return None;
    }
    Some((start, &head[start..end]))
}

fn is_css_ident(character: char) -> bool {
    character.is_ascii_alphanumeric() || character == '-' || character == '_'
}

fn declaration_value_end(text: &str, start: usize) -> usize {
    let bytes = text.as_bytes();
    let mut index = start;
    let mut quote: Option<u8> = None;
    let mut parentheses = 0i32;
    while index < bytes.len() {
        let byte = bytes[index];
        if let Some(current) = quote {
            if byte == current {
                quote = None;
            }
            index += 1;
            continue;
        }
        match byte {
            b'\'' | b'"' => quote = Some(byte),
            b'(' => parentheses += 1,
            b')' => parentheses -= 1,
            b',' | b';' if parentheses == 0 => return index,
            _ => {}
        }
        index += 1;
    }
    index
}

fn is_unsupported_property(name: &str) -> bool {
    UNSUPPORTED_TEXT_PROPERTIES
        .iter()
        .any(|property| property.eq_ignore_ascii_case(name))
}

fn strip_html_style_attributes(text: &str) -> String {
    let mut output = String::with_capacity(text.len());
    let mut cursor = 0;
    let bytes = text.as_bytes();
    while cursor < text.len() {
        let Some(relative) = text[cursor..].find("style") else {
            output.push_str(&text[cursor..]);
            break;
        };
        let start = cursor + relative;
        let boundary = if start == 0 {
            true
        } else {
            let previous = bytes[start - 1];
            !previous.is_ascii_alphanumeric() && previous != b'_' && previous != b'-'
        };
        let after_name = start + "style".len();
        if !boundary {
            output.push_str(&text[cursor..after_name]);
            cursor = after_name;
            continue;
        }
        let mut index = skip_ascii_whitespace(text, after_name);
        if bytes.get(index) != Some(&b'=') {
            output.push_str(&text[cursor..after_name]);
            cursor = after_name;
            continue;
        }
        index = skip_ascii_whitespace(text, index + 1);
        let Some(quote) = bytes.get(index).copied() else {
            output.push_str(&text[cursor..]);
            break;
        };
        if quote != b'"' && quote != b'\'' {
            output.push_str(&text[cursor..after_name]);
            cursor = after_name;
            continue;
        }
        let value_start = index + 1;
        let Some(value_end) = text[value_start..]
            .find(quote as char)
            .map(|relative| value_start + relative)
        else {
            output.push_str(&text[cursor..]);
            break;
        };
        let cleaned = remove_unsupported_declarations(&text[value_start..value_end]);
        let cleaned = cleaned.trim().trim_matches(';').trim();
        if cleaned.is_empty() {
            let mut attribute_start = start;
            if attribute_start > cursor && bytes[attribute_start - 1].is_ascii_whitespace() {
                attribute_start -= 1;
            }
            output.push_str(&text[cursor..attribute_start]);
        } else {
            output.push_str(&text[cursor..value_start]);
            output.push_str(cleaned);
            output.push(quote as char);
        }
        cursor = value_end + 1;
    }
    output
}

fn skip_ascii_whitespace(text: &str, mut index: usize) -> usize {
    let bytes = text.as_bytes();
    while index < bytes.len() && bytes[index].is_ascii_whitespace() {
        index += 1;
    }
    index
}

#[cfg(test)]
mod tests {
    use super::*;

    fn class_source(font_size: &str) -> String {
        format!(
            "classDiagram\n%%{{init: {{\"themeVariables\": {{\"fontSize\": {font_size}}}}}}}%%\nclass User\n"
        )
    }

    fn prepared(source: &str) -> String {
        prepare_source(source).into_owned()
    }

    fn prepared_class(font_size: &str) -> String {
        let source = class_source(font_size);
        prepared(&source)
    }

    #[test]
    fn theme_font_size_is_rewritten_to_the_painted_px_size() {
        let class_em = prepared_class(r#""1.5em""#);
        assert!(
            class_em.contains(r#""fontSize":"24px""#),
            "1.5em paints at 24px: {class_em}"
        );
        assert!(!class_em.contains("1.5em"));

        let class_number = prepared_class("24");
        assert!(
            class_number.contains(r#""fontSize":"16px""#),
            "unitless 24 is ignored by class CSS: {class_number}"
        );

        let class_string = prepared_class(r#""24""#);
        assert!(class_string.contains(r#""fontSize":"16px""#));

        let class_px = prepared_class(r#""24px""#);
        assert!(class_px.contains(r#""fontSize": "24px""#), "{class_px}");

        let class_sixteen = prepared_class("16");
        assert!(
            class_sixteen.contains(r#""fontSize": 16"#),
            "{class_sixteen}"
        );

        let flowchart = prepared(
            "flowchart LR\n%%{init: {\"themeVariables\": {\"fontSize\": 24}}}%%\n    A[Hi]\n",
        );
        assert!(
            flowchart.contains(r#""fontSize":"24px""#),
            "flowchart numeric 24 paints at 24px: {flowchart}"
        );

        let flowchart_em = prepared(
            "flowchart LR\n%%{init: {\"themeVariables\": {\"fontSize\": \"1.5em\"}}}%%\n    A[Hi]\n",
        );
        assert!(
            flowchart_em.contains(r#""fontSize":"1px""#),
            "flowchart keeps the leading integer of 1.5em: {flowchart_em}"
        );

        let flowchart_fraction = prepared(
            "flowchart LR\n%%{init: {\"themeVariables\": {\"fontSize\": \"24.5px\"}}}%%\n    A[Hi]\n",
        );
        assert!(
            flowchart_fraction.contains(r#""fontSize": "24.5px""#),
            "{flowchart_fraction}"
        );

        let sequence = prepared(
            "sequenceDiagram\n%%{init: {\"themeVariables\": {\"fontSize\": \"1.5em\"}}}%%\n    Alice->>Bob: Hi\n",
        );
        assert!(sequence.contains("1.5em"), "{sequence}");

        let frontmatter = prepared(
            "---\nconfig:\n  themeVariables:\n    fontSize: 1.5em\n---\nclassDiagram\nclass User\n",
        );
        assert!(frontmatter.contains("fontSize: 24px"), "{frontmatter}");
    }

    #[test]
    fn unsupported_style_declarations_are_removed_without_touching_labels() {
        let source = r#"flowchart LR
    A["letter-spacing:4px and white-space:nowrap"]
    style B letter-spacing:0px
    style A fill:#fff,letter-spacing:1px,stroke:#111
    classDef spaced fill:#abc,word-spacing:2px
    A["<span style='letter-spacing:4px;font-size:20px'>Hi</span>"]
"#;
        let prepared = prepare_source(source);
        assert!(
            prepared.contains("letter-spacing:4px and white-space:nowrap"),
            "{prepared}"
        );
        assert!(!prepared.contains("style B"), "{prepared}");
        assert!(
            prepared.contains("style A fill:#fff,stroke:#111"),
            "{prepared}"
        );
        assert!(prepared.contains("classDef spaced fill:#abc"), "{prepared}");
        assert!(
            !prepared.contains("word-spacing") && !prepared.contains("letter-spacing:1px"),
            "{prepared}"
        );
        assert!(prepared.contains("style='font-size:20px'"), "{prepared}");
    }
}
