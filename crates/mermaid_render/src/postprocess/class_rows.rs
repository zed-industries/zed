//! Re-anchor wrapped class-diagram rows on the line height.
//!
//! merman centers a class member or method row on its full block height. A
//! single line is already half a line height above the row origin, but a wrap
//! is half the whole block, so the first line climbs into the title. Move only
//! those wrapped rows down to the single-line anchor and let the extra lines
//! extend downward.

use std::borrow::Cow;
use std::collections::HashMap;

pub(super) fn shift_wrapped_class_rows(svg: &str) -> Cow<'_, str> {
    let metrics = wrapped_class_label_metrics(svg);
    if metrics.is_empty() {
        return Cow::Borrowed(svg);
    }

    let mut edits = Vec::new();
    let mut cursor = 0;
    while let Some(relative) = svg[cursor..].find("href=\"#") {
        let id_start = cursor + relative + "href=\"#".len();
        let Some(end_relative) = svg[id_start..].find('"') else {
            break;
        };
        let id_end = id_start + end_relative;
        let id = &svg[id_start..id_end];
        cursor = id_end + 1;
        let Some(label) = metrics.get(id) else {
            continue;
        };
        let Some((number_start, number_end, translate_y)) = nearest_translate_y(svg, id_start)
        else {
            continue;
        };
        if edits
            .last()
            .is_some_and(|(start, _, _)| *start == number_start)
        {
            continue;
        }
        if !is_class_member_or_method_row(svg, number_start) {
            continue;
        }
        let half_block = label.height / 2.0;
        if (translate_y + half_block).abs() > 0.05 {
            continue;
        }
        let anchored = -label.line_height / 2.0;
        if (anchored - translate_y).abs() < 0.05 {
            continue;
        }
        edits.push((number_start, number_end, format_translate_number(anchored)));
    }

    if edits.is_empty() {
        return Cow::Borrowed(svg);
    }
    let mut output = svg.to_string();
    for (start, end, replacement) in edits.into_iter().rev() {
        output.replace_range(start..end, &replacement);
    }
    Cow::Owned(output)
}

struct WrappedLabel {
    height: f64,
    line_height: f64,
}

fn wrapped_class_label_metrics(svg: &str) -> HashMap<&str, WrappedLabel> {
    let mut metrics = HashMap::new();
    let mut cursor = 0;
    while let Some(relative) = svg[cursor..].find("id=\"") {
        let id_start = cursor + relative + "id=\"".len();
        let Some(end_relative) = svg[id_start..].find('"') else {
            break;
        };
        let id_end = id_start + end_relative;
        let id = &svg[id_start..id_end];
        cursor = id_end + 1;
        if !id.contains("foreignobject-label") {
            continue;
        }
        let Some(svg_relative) = svg[id_end..].find("<svg") else {
            continue;
        };
        let svg_tag_start = id_end + svg_relative;
        let Some(tag_relative) = svg[svg_tag_start..].find('>') else {
            continue;
        };
        let tag_end = svg_tag_start + tag_relative;
        let Some(close_relative) = svg[tag_end..].find("</svg>") else {
            continue;
        };
        let close = tag_end + close_relative;
        let tag = &svg[svg_tag_start..tag_end];
        let inner = &svg[tag_end..close];
        cursor = close;
        let Some(height) = attribute_number(tag, "height") else {
            continue;
        };
        if inner.matches("<text").count() < 2 {
            continue;
        }
        let Some(line_height) = style_px(inner, "line-height")
            .or_else(|| style_px(inner, "font-size").map(|font_size| font_size * 1.5))
        else {
            continue;
        };
        if height <= line_height + 0.05 {
            continue;
        }
        metrics.insert(
            id,
            WrappedLabel {
                height,
                line_height,
            },
        );
    }
    metrics
}

fn attribute_number(tag: &str, name: &str) -> Option<f64> {
    let key = format!("{name}=\"");
    let start = tag.find(&key)? + key.len();
    let end = start + tag[start..].find('"')?;
    tag[start..end].parse().ok()
}

fn style_px(text: &str, name: &str) -> Option<f64> {
    let key = format!("{name}:");
    let start = text.find(&key)? + key.len();
    let rest = text[start..].trim_start();
    let mut number = String::new();
    for character in rest.chars() {
        if character.is_ascii_digit() || character == '.' || character == '-' || character == '+' {
            number.push(character);
            continue;
        }
        break;
    }
    if number.is_empty() || number == "+" || number == "-" || number == "." {
        return None;
    }
    number.parse().ok()
}

fn nearest_translate_y(svg: &str, before: usize) -> Option<(usize, usize, f64)> {
    let window_start = before.saturating_sub(1200);
    let window = &svg[window_start..before];
    let mut search = 0;
    let mut found = None;
    while let Some(relative) = window[search..].find("translate(0,") {
        let number_relative = search + relative + "translate(0,".len();
        let after = &window[number_relative..];
        let trimmed_after = after.trim_start();
        let trimmed = after.len() - trimmed_after.len();
        if let Some((value, consumed)) = parse_leading_number(trimmed_after) {
            let number_start = window_start + number_relative + trimmed;
            let number_end = number_start + consumed;
            if svg.as_bytes().get(number_end) == Some(&b')') {
                found = Some((number_start, number_end, value));
            }
        }
        search = number_relative;
    }
    found
}

fn parse_leading_number(text: &str) -> Option<(f64, usize)> {
    let bytes = text.as_bytes();
    if bytes.is_empty() {
        return None;
    }
    let mut index = 0;
    if bytes[0] == b'+' || bytes[0] == b'-' {
        index = 1;
    }
    let digits_start = index;
    let mut saw_dot = false;
    while index < bytes.len() {
        let byte = bytes[index];
        if byte.is_ascii_digit() {
            index += 1;
            continue;
        }
        if byte == b'.' && !saw_dot {
            saw_dot = true;
            index += 1;
            continue;
        }
        break;
    }
    if index == digits_start {
        return None;
    }
    let number = text[..index].parse().ok()?;
    Some((number, index))
}

fn is_class_member_or_method_row(svg: &str, index: usize) -> bool {
    let start = index.saturating_sub(800);
    let window = &svg[start..index];
    let members = window.rfind("members-group");
    let methods = window.rfind("methods-group");
    let label = window.rfind("label-group");
    match (members.max(methods), label) {
        (Some(row), Some(label)) => row > label,
        (Some(_), None) => true,
        _ => false,
    }
}

fn format_translate_number(value: f64) -> String {
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
