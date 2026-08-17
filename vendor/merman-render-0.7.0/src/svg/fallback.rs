#![forbid(unsafe_code)]

// NOTE: This fallback module intentionally keeps parsing "cheap" and non-validating.
// It is a best-effort readability fallback for SVG consumers that do not fully
// support HTML inside `<foreignObject>` (e.g. many rasterizers).

use crate::text::{
    DeterministicTextMeasurer, TextMeasurer, TextStyle, VendoredFontMetricsTextMeasurer, WrapMode,
};
use std::collections::VecDeque;

#[derive(Clone, Copy, Debug, Default)]
struct Translate {
    x: f64,
    y: f64,
}

#[derive(Clone, Debug, Default)]
struct GFrame {
    translate: Translate,
    class_tokens: Vec<String>,
    fill: Option<String>,
    font_size: Option<String>,
    font_family: Option<String>,
    font_weight: Option<String>,
    font_style: Option<String>,
}

fn parse_attr_str<'a>(tag: &'a str, key: &str) -> Option<&'a str> {
    let needle = format!(r#"{key}=""#);
    let i = tag.find(&needle)?;
    let rest = &tag[i + needle.len()..];
    let end = rest.find('"')?;
    Some(rest[..end].trim())
}

fn parse_attr_f64(tag: &str, key: &str) -> Option<f64> {
    parse_attr_str(tag, key)?.parse::<f64>().ok()
}

fn is_self_closing(tag: &str) -> bool {
    tag.trim_end().ends_with("/>")
}

fn parse_translate(transform: &str) -> Translate {
    let lower = transform.to_ascii_lowercase();
    let Some(i) = lower.find("translate(") else {
        return Translate::default();
    };
    let after = &transform[i + "translate(".len()..];
    let Some(end) = after.find(')') else {
        return Translate::default();
    };
    let args = &after[..end];

    let mut nums = Vec::<f64>::with_capacity(2);
    let mut cur = String::new();
    for ch in args.chars() {
        if ch.is_ascii_digit() || ch == '.' || ch == '-' || ch == '+' || ch == 'e' || ch == 'E' {
            cur.push(ch);
        } else if !cur.is_empty() {
            if let Ok(v) = cur.parse::<f64>() {
                nums.push(v);
            }
            cur.clear();
        }
    }
    if !cur.is_empty() {
        if let Ok(v) = cur.parse::<f64>() {
            nums.push(v);
        }
    }

    Translate {
        x: *nums.first().unwrap_or(&0.0),
        y: *nums.get(1).unwrap_or(&0.0),
    }
}

fn sum_translate(stack: &[GFrame]) -> Translate {
    let mut acc = Translate::default();
    for t in stack {
        acc.x += t.translate.x;
        acc.y += t.translate.y;
    }
    acc
}

fn escape_xml_text(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for ch in s.chars() {
        match ch {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            _ => out.push(ch),
        }
    }
    out
}

fn escape_xml_attr(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for ch in s.chars() {
        match ch {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&apos;"),
            _ => out.push(ch),
        }
    }
    out
}

fn strip_html_tags(s: &str) -> String {
    let mut out = String::new();
    let mut in_tag = false;
    for ch in s.chars() {
        match ch {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => out.push(ch),
            _ => {}
        }
    }
    out
}

fn decode_html_entities_once(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut rest = text;

    while let Some(amp) = rest.find('&') {
        out.push_str(&rest[..amp]);
        let after_amp = &rest[amp + 1..];
        let Some(semi) = after_amp.find(';') else {
            out.push('&');
            rest = after_amp;
            continue;
        };

        let entity = &after_amp[..semi];
        let decoded = match entity {
            "amp" => Some('&'),
            "lt" => Some('<'),
            "gt" => Some('>'),
            "quot" => Some('"'),
            "apos" => Some('\''),
            "nbsp" => Some(' '),
            _ => {
                if let Some(hex) = entity
                    .strip_prefix("#x")
                    .or_else(|| entity.strip_prefix("#X"))
                {
                    u32::from_str_radix(hex, 16).ok().and_then(char::from_u32)
                } else if let Some(decimal) = entity.strip_prefix('#') {
                    decimal.parse::<u32>().ok().and_then(char::from_u32)
                } else {
                    None
                }
            }
        };

        if let Some(ch) = decoded {
            out.push(ch);
        } else {
            out.push('&');
            out.push_str(entity);
            out.push(';');
        }
        rest = &after_amp[semi + 1..];
    }

    out.push_str(rest);
    out
}

fn decode_html_entities(text: &str) -> String {
    let mut current = text.to_string();
    for _ in 0..3 {
        if !current.contains('&') {
            break;
        }
        let next = decode_html_entities_once(&current);
        if next == current {
            break;
        }
        current = next;
    }
    current
}

fn htmlish_to_text_lines(html: &str) -> Vec<String> {
    // Mermaid foreignObject labels often look like:
    //   <div class="label">Line 1<br/>Line 2</div>
    // We treat `<br>` as line breaks and strip remaining tags.
    let mut normalized = html.replace("<br/>", "\n");
    normalized = normalized.replace("<br />", "\n");
    normalized = normalized.replace("<br>", "\n");
    normalized = normalized.replace("</br>", "\n");
    normalized = normalized.replace("\\n", "\n");
    let text = decode_html_entities(&strip_html_tags(&normalized));

    text.lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| l.to_string())
        .collect()
}

fn line_width_html_px(measurer: &dyn TextMeasurer, style: &TextStyle, text: &str) -> f64 {
    measurer
        .measure_wrapped_raw(text, style, None, WrapMode::HtmlLike)
        .width
}

fn wrap_html_line_to_width(
    line: &str,
    max_width_px: f64,
    measurer: &dyn TextMeasurer,
    style: &TextStyle,
) -> Vec<String> {
    const EPS_PX: f64 = 0.125;
    if !max_width_px.is_finite()
        || max_width_px <= 0.0
        || line_width_html_px(measurer, style, line) <= max_width_px + EPS_PX
    {
        return vec![line.to_string()];
    }

    let mut tokens = VecDeque::from(DeterministicTextMeasurer::split_line_to_words(line));
    let mut out = Vec::new();
    let mut cur = String::new();

    while let Some(tok) = tokens.pop_front() {
        if cur.is_empty() && tok == " " {
            continue;
        }

        let candidate = format!("{cur}{tok}");
        let candidate_trimmed = candidate.trim_end();
        if line_width_html_px(measurer, style, candidate_trimmed) <= max_width_px + EPS_PX {
            cur = candidate;
            continue;
        }

        if !cur.trim().is_empty() {
            out.push(cur.trim_end().to_string());
            cur.clear();
            tokens.push_front(tok);
            continue;
        }

        if tok == " " {
            continue;
        }

        // HTML labels do not use `word-break: break-all`; preserve long tokens as readable units.
        out.push(tok);
    }

    if !cur.trim().is_empty() {
        out.push(cur.trim_end().to_string());
    }

    if out.is_empty() {
        vec![line.to_string()]
    } else {
        out
    }
}

fn wrap_html_lines_to_width(
    lines: Vec<String>,
    max_width_px: Option<f64>,
    measurer: &dyn TextMeasurer,
    style: &TextStyle,
) -> Vec<String> {
    let Some(max_width_px) = max_width_px.filter(|w| w.is_finite() && *w > 0.0) else {
        return lines;
    };

    lines
        .into_iter()
        .flat_map(|line| wrap_html_line_to_width(&line, max_width_px, measurer, style))
        .collect()
}

fn extract_css_background_color_for_class(svg: &str, class_name: &str) -> Option<String> {
    // Mermaid parity SVGs inline styles in a `<style>` element and typically emit rules like:
    //   #<id> .labelBkg{background-color:rgba(...);}
    // This is a cheap non-validating parser that looks for `.className{...}` and then extracts the
    // first `background-color:` declaration within that block.
    let needle = format!(".{class_name}{{");
    let mut search = 0usize;
    while let Some(rel) = svg[search..].find(&needle) {
        let i = search + rel + needle.len();
        let end_rel = svg[i..].find('}')?;
        let block = &svg[i..i + end_rel];
        if let Some(k) = block.find("background-color:") {
            let after = &block[k + "background-color:".len()..];
            let end = after.find(';').unwrap_or(after.len());
            let value = after[..end].trim();
            if !value.is_empty() {
                return Some(value.to_string());
            }
        }
        search = i + end_rel + 1;
    }
    None
}

fn extract_css_text_fill_for_class(svg: &str, class_name: &str) -> Option<String> {
    // Mermaid parity SVGs inline styles in a `<style>` element and typically emit rules like:
    //   #<id> .section-root text{fill:#ffffff;}
    //   #<id> .label text,#<id> span,#<id> p{fill:#ffffff;color:#ffffff;}
    // This is a cheap non-validating parser for scoped selector lists. It deliberately avoids
    // shape selectors like `.node rect` so node backgrounds do not become text fallback fills.
    let mut search = 0usize;
    while let Some(open_rel) = svg[search..].find('{') {
        let open = search + open_rel;
        let Some(close_rel) = svg[open + 1..].find('}') else {
            break;
        };
        let close = open + 1 + close_rel;
        let selector_start = svg[..open].rfind('}').map_or(0, |idx| idx + 1);
        let selector = svg[selector_start..open].trim();
        let declarations = &svg[open + 1..close];

        if selector_rule_applies_to_text_class(selector, class_name) {
            let property = extract_style_property(declarations, "fill")
                .or_else(|| extract_style_property(declarations, "color"));
            if property.is_some() {
                return property;
            }
        } else if selector_rule_applies_inherited_color_to_class(selector, class_name) {
            if let Some(color) = extract_style_property(declarations, "color") {
                return Some(color);
            }
        }

        search = close + 1;
    }

    // Preserve the historical fast path for compact unscoped rules.
    let needle = format!(".{class_name} text{{fill:");
    let mut search = 0usize;
    while let Some(rel) = svg[search..].find(&needle) {
        let i = search + rel + needle.len();
        let after = &svg[i..];
        let end = after
            .find(';')
            .or_else(|| after.find('}'))
            .unwrap_or(after.len());
        let value = after[..end].trim();
        if !value.is_empty() {
            return Some(value.to_string());
        }
        search = i + end;
    }
    None
}

fn extract_css_root_text_fill(svg: &str) -> Option<String> {
    extract_css_root_style_property(svg, &["fill", "color"])
}

fn extract_css_root_style_property(svg: &str, properties: &[&str]) -> Option<String> {
    let svg_start = svg.find("<svg")?;
    let svg_end = svg[svg_start..].find('>').map(|rel| svg_start + rel + 1)?;
    let root_id = parse_attr_str(&svg[svg_start..svg_end], "id")?;
    let needle = format!("#{root_id}{{");
    let i = svg.find(&needle)? + needle.len();
    let end_rel = svg[i..].find('}')?;
    let declarations = &svg[i..i + end_rel];
    properties
        .iter()
        .find_map(|property| extract_style_property(declarations, property))
}

fn extract_css_style_property_for_class(
    svg: &str,
    class_name: &str,
    property: &str,
) -> Option<String> {
    let mut search = 0usize;
    while let Some(open_rel) = svg[search..].find('{') {
        let open = search + open_rel;
        let Some(close_rel) = svg[open + 1..].find('}') else {
            break;
        };
        let close = open + 1 + close_rel;
        let selector_start = svg[..open].rfind('}').map_or(0, |idx| idx + 1);
        let selector = svg[selector_start..open].trim();
        let declarations = &svg[open + 1..close];

        if selector_rule_applies_inherited_color_to_class(selector, class_name) {
            if let Some(value) = extract_style_property(declarations, property) {
                return Some(value);
            }
        }

        search = close + 1;
    }

    None
}

fn selector_rule_applies_to_text_class(selector_list: &str, class_name: &str) -> bool {
    selector_list.split(',').map(str::trim).any(|selector| {
        selector_has_class(selector, class_name) && selector_targets_text_like(selector)
    })
}

fn selector_rule_applies_inherited_color_to_class(selector_list: &str, class_name: &str) -> bool {
    selector_list.split(',').map(str::trim).any(|selector| {
        selector_has_class(selector, class_name) && !selector_targets_shape(selector)
    })
}

fn selector_has_class(selector: &str, class_name: &str) -> bool {
    let needle = format!(".{class_name}");
    let mut search = 0usize;
    while let Some(rel) = selector[search..].find(&needle) {
        let start = search + rel;
        let after = start + needle.len();
        if selector[after..]
            .chars()
            .next()
            .is_none_or(|ch| !is_css_identifier_char(ch))
        {
            return true;
        }
        search = after;
    }
    false
}

fn is_css_identifier_char(ch: char) -> bool {
    ch.is_ascii_alphanumeric() || ch == '-' || ch == '_'
}

fn selector_targets_text_like(selector: &str) -> bool {
    selector_targets_element(selector, "text")
        || selector_targets_element(selector, "tspan")
        || selector_targets_element(selector, "span")
        || selector_targets_element(selector, "p")
}

fn selector_targets_shape(selector: &str) -> bool {
    selector_targets_element(selector, "rect")
        || selector_targets_element(selector, "circle")
        || selector_targets_element(selector, "ellipse")
        || selector_targets_element(selector, "polygon")
        || selector_targets_element(selector, "path")
        || selector_targets_element(selector, "line")
}

fn selector_targets_element(selector: &str, element: &str) -> bool {
    let lower = selector.to_ascii_lowercase();
    let mut search = 0usize;
    while let Some(rel) = lower[search..].find(element) {
        let start = search + rel;
        let before = lower[..start].chars().next_back();
        let after = lower[start + element.len()..].chars().next();
        let before_ok = before.is_none_or(|ch| !is_css_identifier_char(ch));
        let after_ok = after.is_none_or(|ch| !is_css_identifier_char(ch));
        if before_ok && after_ok {
            return true;
        }
        search = start + element.len();
    }
    false
}

fn extract_style_property(style: &str, property: &str) -> Option<String> {
    for decl in style.split(';') {
        let Some((name, value)) = decl.split_once(':') else {
            continue;
        };
        if name.trim().eq_ignore_ascii_case(property) {
            let value = strip_important(value.trim());
            if !value.is_empty() {
                return Some(value);
            }
        }
    }
    None
}

fn strip_important(value: &str) -> String {
    let mut value = value.trim().to_string();
    if let Some(v) = value.strip_suffix("!important") {
        value = v.trim().to_string();
    }
    value
}

fn extract_inline_html_style_property(html: &str, property: &str) -> Option<String> {
    parse_attr_str(html, "style").and_then(|style| extract_style_property(style, property))
}

fn extract_inline_html_color(html: &str) -> Option<String> {
    extract_inline_html_style_property(html, "color")
}

fn parse_class_tokens(tag: &str) -> Vec<String> {
    let Some(s) = parse_attr_str(tag, "class") else {
        return Vec::new();
    };
    s.split_whitespace().map(|t| t.to_string()).collect()
}

fn extract_svg_text_fill_from_ancestors(svg: &str, g_stack: &[GFrame]) -> Option<String> {
    // Prefer the closest ancestor's classes (more specific) by scanning frames from inner -> outer.
    for frame in g_stack.iter().rev() {
        for token in frame.class_tokens.iter().rev() {
            if let Some(fill) = extract_css_text_fill_for_class(svg, token) {
                return Some(fill);
            }
        }
        if let Some(fill) = &frame.fill {
            return Some(fill.clone());
        }
    }
    extract_css_root_text_fill(svg)
}

fn extract_svg_font_style_from_ancestors(g_stack: &[GFrame], property: &str) -> Option<String> {
    for frame in g_stack.iter().rev() {
        let value = match property {
            "font-size" => &frame.font_size,
            "font-family" => &frame.font_family,
            "font-weight" => &frame.font_weight,
            "font-style" => &frame.font_style,
            _ => return None,
        };
        if let Some(value) = value {
            return Some(value.clone());
        }
    }
    None
}

fn extract_svg_font_style_from_context(
    svg: &str,
    g_stack: &[GFrame],
    property: &str,
) -> Option<String> {
    extract_svg_font_style_from_ancestors(g_stack, property).or_else(|| {
        for frame in g_stack.iter().rev() {
            for token in frame.class_tokens.iter().rev() {
                if let Some(value) = extract_css_style_property_for_class(svg, token, property) {
                    return Some(value);
                }
            }
        }
        extract_css_root_style_property(svg, &[property])
    })
}

fn class_attr_tokens(g_stack: &[GFrame], inner: &str, base_class: &str) -> String {
    let mut tokens = vec![base_class.to_string()];
    for frame in g_stack {
        for token in &frame.class_tokens {
            if !tokens.iter().any(|existing| existing == token) {
                tokens.push(token.clone());
            }
        }
    }
    for token in parse_class_tokens(inner) {
        if !tokens.iter().any(|existing| existing == &token) {
            tokens.push(token);
        }
    }
    escape_xml_attr(&tokens.join(" "))
}

fn fallback_text_class_attr_tokens(g_stack: &[GFrame], inner: &str) -> String {
    let mut tokens = vec!["merman-foreignobject-fallback-text".to_string()];
    for frame in g_stack {
        for token in &frame.class_tokens {
            if is_fallback_text_safe_class(token)
                && !tokens.iter().any(|existing| existing == token)
            {
                tokens.push(token.clone());
            }
        }
    }
    for token in parse_class_tokens(inner) {
        if is_fallback_text_safe_class(&token) && !tokens.iter().any(|existing| existing == &token)
        {
            tokens.push(token);
        }
    }
    escape_xml_attr(&tokens.join(" "))
}

fn is_fallback_text_safe_class(class_name: &str) -> bool {
    !matches!(class_name, "label")
}

fn parse_css_px(value: &str, fallback: f64) -> f64 {
    parse_css_px_value(value).unwrap_or(fallback)
}

fn parse_css_px_value(value: &str) -> Option<f64> {
    let trimmed = value.trim();
    let trimmed = strip_important(trimmed);
    let number = trimmed.strip_suffix("px").unwrap_or(&trimmed).trim();
    number
        .parse::<f64>()
        .ok()
        .filter(|value| value.is_finite() && *value > 0.0)
}

fn foreign_object_html_soft_wrap_width(tag: &str, inner: &str) -> Option<f64> {
    let white_space = extract_inline_html_style_property(inner, "white-space")
        .map(|value| value.trim().to_ascii_lowercase());
    if matches!(white_space.as_deref(), Some("nowrap" | "pre")) {
        return None;
    }

    let wrap_is_explicit = matches!(
        white_space.as_deref(),
        Some("break-spaces" | "normal" | "pre-wrap" | "pre-line")
    );
    if white_space.is_some() && !wrap_is_explicit {
        return None;
    }

    let css_width = extract_inline_html_style_property(inner, "width")
        .and_then(|value| parse_css_px_value(&value));
    let max_width = extract_inline_html_style_property(inner, "max-width")
        .and_then(|value| parse_css_px_value(&value));

    css_width
        .or(max_width)
        .or_else(|| parse_attr_f64(tag, "width"))
        .filter(|width| {
            *width > 0.0 && (wrap_is_explicit || css_width.is_some() || max_width.is_some())
        })
}

/// Adds a best-effort `<text>/<tspan>` overlay extracted from Mermaid label `<foreignObject>`
/// content.
///
/// Many headless SVG renderers and rasterizers do not fully support HTML inside `<foreignObject>`.
/// The returned SVG aims to be *more readable* for raster outputs and UI previews.
///
/// Important:
/// - This does not aim for Mermaid DOM parity.
/// - For parity-focused SVG output, keep the original SVG unchanged.
pub fn foreign_object_label_fallback_svg_text(svg: &str) -> String {
    if !svg.contains("<foreignObject") {
        return svg.to_string();
    }

    let close_tag = "</foreignObject>";
    let mut out = String::with_capacity(svg.len() + 2048);
    let mut overlays = String::new();
    let mut g_stack: Vec<GFrame> = Vec::new();
    let label_bkg_default = "rgba(232, 232, 232, 0.5)".to_string();
    let label_bkg =
        extract_css_background_color_for_class(svg, "labelBkg").unwrap_or(label_bkg_default);
    let fallback_measurer = VendoredFontMetricsTextMeasurer::default();

    let mut i = 0usize;
    while let Some(lt_rel) = svg[i..].find('<') {
        let lt = i + lt_rel;
        out.push_str(&svg[i..lt]);

        let Some(gt_rel) = svg[lt..].find('>') else {
            out.push_str(&svg[lt..]);
            i = svg.len();
            break;
        };
        let gt = lt + gt_rel + 1;
        let tag = &svg[lt..gt];

        // Comments / declarations: passthrough.
        if tag.starts_with("<!--") || tag.starts_with("<!") || tag.starts_with("<?") {
            out.push_str(tag);
            i = gt;
            continue;
        }

        if tag.starts_with("</g") {
            let _ = g_stack.pop();
            out.push_str(tag);
            i = gt;
            continue;
        }

        if tag.starts_with("<g") {
            let t = parse_attr_str(tag, "transform")
                .map(parse_translate)
                .unwrap_or_default();
            let class_tokens = parse_class_tokens(tag);
            let style = parse_attr_str(tag, "style");
            let fill = parse_attr_str(tag, "fill")
                .map(ToOwned::to_owned)
                .or_else(|| style.and_then(|style| extract_style_property(style, "fill")));
            let font_size = style.and_then(|style| extract_style_property(style, "font-size"));
            let font_family = style.and_then(|style| extract_style_property(style, "font-family"));
            let font_weight = style.and_then(|style| extract_style_property(style, "font-weight"));
            let font_style = style.and_then(|style| extract_style_property(style, "font-style"));
            if !is_self_closing(tag) {
                g_stack.push(GFrame {
                    translate: t,
                    class_tokens,
                    fill,
                    font_size,
                    font_family,
                    font_weight,
                    font_style,
                });
            }
            out.push_str(tag);
            i = gt;
            continue;
        }

        if tag.starts_with("<foreignObject") {
            let start_end = gt;
            let Some(close_rel) = svg[start_end..].find(close_tag) else {
                out.push_str(&svg[lt..]);
                i = svg.len();
                break;
            };
            let inner_start = start_end;
            let inner_end = inner_start + close_rel;
            let inner = &svg[inner_start..inner_end];
            let i_next = inner_end + close_tag.len();

            out.push_str(&svg[lt..i_next]);

            let width = parse_attr_f64(tag, "width").unwrap_or(0.0);
            let height = parse_attr_f64(tag, "height").unwrap_or(0.0);
            if width > 0.0 && height > 0.0 {
                let x = parse_attr_f64(tag, "x").unwrap_or(0.0);
                let y = parse_attr_f64(tag, "y").unwrap_or(0.0);
                let base = sum_translate(&g_stack);

                let abs_x = base.x + x;
                let abs_y = base.y + y;
                let (anchor, text_x) = match parse_attr_str(tag, "text-anchor") {
                    Some("start") => ("start", abs_x),
                    Some("end") => ("end", abs_x + width),
                    _ => ("middle", abs_x + width / 2.0),
                };
                let text_y = abs_y + height / 2.0;

                let raw_lines = htmlish_to_text_lines(inner);
                if !raw_lines.is_empty() {
                    overlays.push_str(&format!(
                        r#"<g data-merman-foreignobject="fallback" class="{}">"#,
                        class_attr_tokens(&g_stack, inner, "merman-foreignobject-fallback")
                    ));

                    let wants_label_bkg = inner.contains("labelBkg");
                    if wants_label_bkg {
                        overlays.push_str(&format!(
                            r#"<rect x="{}" y="{}" width="{}" height="{}" fill="{}"/>"#,
                            abs_x,
                            abs_y,
                            width,
                            height,
                            escape_xml_attr(&label_bkg)
                        ));
                    }

                    let font_size_value = extract_inline_html_style_property(inner, "font-size")
                        .or_else(|| extract_svg_font_style_from_context(svg, &g_stack, "font-size"))
                        .unwrap_or_else(|| "16px".to_string());
                    let font_size = parse_css_px(&font_size_value, 16.0);
                    let fill = extract_inline_html_color(inner)
                        .or_else(|| extract_svg_text_fill_from_ancestors(svg, &g_stack))
                        .unwrap_or_else(|| "#333".to_string());
                    let font_family = extract_inline_html_style_property(inner, "font-family")
                        .or_else(|| {
                            extract_svg_font_style_from_context(svg, &g_stack, "font-family")
                        })
                        .unwrap_or_else(|| "trebuchet ms,verdana,arial,sans-serif".to_string());
                    let font_weight = extract_inline_html_style_property(inner, "font-weight")
                        .or_else(|| {
                            extract_svg_font_style_from_context(svg, &g_stack, "font-weight")
                        });
                    let font_style = extract_inline_html_style_property(inner, "font-style")
                        .or_else(|| {
                            extract_svg_font_style_from_context(svg, &g_stack, "font-style")
                        });
                    let measure_style = TextStyle {
                        font_family: Some(font_family.clone()),
                        font_size,
                        font_weight: font_weight.clone(),
                    };
                    let wrap_width = foreign_object_html_soft_wrap_width(tag, inner);
                    let lines = wrap_html_lines_to_width(
                        raw_lines,
                        wrap_width,
                        &fallback_measurer,
                        &measure_style,
                    );
                    let line_height = font_size * 1.5;
                    let n = lines.len() as f64;
                    let y0 = text_y - (line_height * (n - 1.0)) / 2.0;
                    let mut text_style = format!(
                        "text-anchor: {anchor}; font-size: {font_size_value}; font-family: {font_family};"
                    );
                    if let Some(font_weight) = font_weight {
                        text_style.push_str(" font-weight: ");
                        text_style.push_str(&font_weight);
                        text_style.push(';');
                    }
                    if let Some(font_style) = font_style {
                        text_style.push_str(" font-style: ");
                        text_style.push_str(&font_style);
                        text_style.push(';');
                    }
                    let text_class = fallback_text_class_attr_tokens(&g_stack, inner);

                    for (idx, line) in lines.iter().enumerate() {
                        let y_line = y0 + (idx as f64) * line_height;
                        let text = escape_xml_text(line);
                        overlays.push_str(&format!(
                            r##"<text x="{}" y="{}" dominant-baseline="central" alignment-baseline="central" fill="{}" class="{}" style="{}">{}</text>"##,
                            text_x,
                            y_line,
                            escape_xml_attr(&fill),
                            text_class,
                            escape_xml_attr(&text_style),
                            text
                        ));
                    }

                    overlays.push_str("</g>");
                }
            }

            i = i_next;
            continue;
        }

        out.push_str(tag);
        i = gt;
    }

    if i < svg.len() {
        out.push_str(&svg[i..]);
    }

    if overlays.is_empty() {
        return out;
    }

    if let Some(idx) = out.rfind("</svg>") {
        let mut with_overlays = String::with_capacity(out.len() + overlays.len() + 64);
        with_overlays.push_str(&out[..idx]);
        with_overlays.push_str(&overlays);
        with_overlays.push_str(&out[idx..]);
        with_overlays
    } else {
        out
    }
}

#[cfg(test)]
mod tests {
    use super::foreign_object_label_fallback_svg_text;

    #[test]
    fn foreign_object_overlay_accounts_for_parent_translate() {
        let svg = r#"<svg viewBox="90 -310 425 99" xmlns="http://www.w3.org/2000/svg"><g transform="translate(183.3046875, -300)"><foreignObject width="33.390625" height="24"><div xmlns="http://www.w3.org/1999/xhtml"><p>Todo</p></div></foreignObject></g></svg>"#;
        let out = foreign_object_label_fallback_svg_text(svg);
        assert!(
            out.contains(r#"x="200""#),
            "expected x=200 center placement"
        );
        assert!(
            out.contains(r#"y="-288""#),
            "expected y=-288 center placement"
        );
        assert!(
            out.contains(">Todo<"),
            "expected text content to be present"
        );
    }

    #[test]
    fn foreign_object_overlay_renders_label_bkg_rect_when_present() {
        let svg = r#"<svg xmlns="http://www.w3.org/2000/svg"><style>#d .labelBkg{background-color:rgba(232,232,232,0.5);}</style><g id="d"><foreignObject x="10" y="20" width="30" height="24"><div xmlns="http://www.w3.org/1999/xhtml" class="labelBkg"><p>Hello</p></div></foreignObject></g></svg>"#;
        let out = foreign_object_label_fallback_svg_text(svg);
        assert!(
            out.contains(r#"fill="rgba(232,232,232,0.5)""#),
            "expected labelBkg fill"
        );
        assert!(
            out.contains(r#"<rect x="10" y="20" width="30" height="24""#),
            "expected rect with foreignObject bounds"
        );
    }

    #[test]
    fn foreign_object_overlay_splits_literal_backslash_n() {
        let svg = r#"<svg xmlns="http://www.w3.org/2000/svg"><g transform="translate(10, 20)"><foreignObject width="80" height="48"><div xmlns="http://www.w3.org/1999/xhtml"><p>Layer 7\nHTTP</p></div></foreignObject></g></svg>"#;
        let out = foreign_object_label_fallback_svg_text(svg);
        assert!(out.contains(">Layer 7<"), "got: {out}");
        assert!(out.contains(">HTTP<"), "got: {out}");
        assert!(
            !out.contains(">Layer 7\\nHTTP</text>"),
            "literal backslash-n should not remain in fallback text overlay: {out}"
        );
    }

    #[test]
    fn foreign_object_overlay_propagates_style_context() {
        let svg = r##"<svg xmlns="http://www.w3.org/2000/svg"><g class="node selected" fill="#112233" style="font-size: 14px; font-family: Inter; font-weight: 600;"><foreignObject width="80" height="24"><div xmlns="http://www.w3.org/1999/xhtml" class="labelBkg host-label" style="color: #abcdef; font-style: italic;"><p>Hello</p></div></foreignObject></g></svg>"##;
        let out = foreign_object_label_fallback_svg_text(svg);

        assert!(
            out.contains(
                r#"class="merman-foreignobject-fallback node selected labelBkg host-label""#
            ),
            "expected fallback group to keep host-relevant classes: {out}"
        );
        assert!(
            out.contains(
                r#"class="merman-foreignobject-fallback-text node selected labelBkg host-label""#
            ),
            "expected fallback text to keep host-relevant classes: {out}"
        );
        assert!(
            out.contains(r##"fill="#abcdef""##),
            "expected inline HTML color to drive fallback fill: {out}"
        );
        assert!(
            out.contains("font-size: 14px")
                && out.contains("font-family: Inter")
                && out.contains("font-weight: 600")
                && out.contains("font-style: italic"),
            "expected font context to propagate: {out}"
        );
    }

    #[test]
    fn foreign_object_overlay_uses_scoped_label_css_for_fallback_fill() {
        let svg = r##"<svg id="host-theme-block" xmlns="http://www.w3.org/2000/svg"><style>#host-theme-block{fill:#eeeeee;}#host-theme-block .node rect{fill:#111827;}#host-theme-block .label text,#host-theme-block span,#host-theme-block p{fill:#e5e7eb;color:#e5e7eb;}</style><g class="block"><g class="node flowchart-label"><g class="label"><foreignObject width="80" height="24"><div xmlns="http://www.w3.org/1999/xhtml"><p>Alpha</p></div></foreignObject></g></g></g></svg>"##;
        let out = foreign_object_label_fallback_svg_text(svg);

        assert!(
            out.contains(r##"fill="#e5e7eb""##),
            "expected scoped label CSS, not shape CSS/default fill, to drive fallback text: {out}"
        );
        assert!(
            !out.contains(r##"fill="#111827""##),
            "fallback text should not inherit node rectangle fill: {out}"
        );
    }

    #[test]
    fn foreign_object_overlay_uses_root_fill_when_no_label_context_matches() {
        let svg = r##"<svg id="host-theme-root" xmlns="http://www.w3.org/2000/svg"><style>#host-theme-root{font-family:Inter;fill:#ddeeff;}#host-theme-root .node rect{fill:#111827;}</style><g><foreignObject width="80" height="24"><div xmlns="http://www.w3.org/1999/xhtml"><p>Alpha</p></div></foreignObject></g></svg>"##;
        let out = foreign_object_label_fallback_svg_text(svg);

        assert!(
            out.contains(r##"fill="#ddeeff""##),
            "expected root fill to be the final readable fallback: {out}"
        );
    }

    #[test]
    fn foreign_object_overlay_uses_root_font_context() {
        let svg = r##"<svg id="host-theme-root-font" xmlns="http://www.w3.org/2000/svg"><style>#host-theme-root-font{font-family:Inter,system-ui;font-size:14px;fill:#ddeeff;}</style><g><foreignObject width="80" height="21"><div xmlns="http://www.w3.org/1999/xhtml"><p>Alpha</p></div></foreignObject></g></svg>"##;
        let out = foreign_object_label_fallback_svg_text(svg);

        assert!(out.contains(r##"fill="#ddeeff""##), "got: {out}");
        assert!(out.contains("font-size: 14px"), "got: {out}");
        assert!(out.contains("font-family: Inter,system-ui"), "got: {out}");
    }

    #[test]
    fn foreign_object_overlay_does_not_put_structural_label_class_on_text() {
        let svg = r##"<svg id="host-theme-edge-label" xmlns="http://www.w3.org/2000/svg"><style>#host-theme-edge-label .edgeLabel .label{fill:#665c54;font-size:14px;}#host-theme-edge-label .edgeLabel .label text{fill:#ebdbb2;}</style><g class="edgeLabel"><g class="label"><foreignObject width="80" height="21"><div xmlns="http://www.w3.org/1999/xhtml" class="labelBkg"><span class="edgeLabel">places</span></div></foreignObject></g></g></svg>"##;
        let out = foreign_object_label_fallback_svg_text(svg);
        let text_tag_start = out
            .find(r#"<text "#)
            .unwrap_or_else(|| panic!("expected fallback text: {out}"));
        let text_tag_end = out[text_tag_start..]
            .find('>')
            .map(|offset| text_tag_start + offset)
            .unwrap_or_else(|| panic!("expected fallback text tag end: {out}"));
        let text_tag = &out[text_tag_start..=text_tag_end];

        assert!(text_tag.contains(r##"fill="#ebdbb2""##), "got: {out}");
        assert!(
            !text_tag.contains(r#"class="merman-foreignobject-fallback-text edgeLabel label "#)
                && !text_tag.contains(r#" label""#),
            "fallback text should not keep the structural label class: {out}"
        );
    }

    #[test]
    fn foreign_object_overlay_decodes_double_escaped_html_entities_for_fallback_text() {
        let svg = r#"<svg xmlns="http://www.w3.org/2000/svg"><g><foreignObject width="120" height="24"><div xmlns="http://www.w3.org/1999/xhtml"><p>List&amp;lt;Animal&amp;gt; &amp;amp; friends</p></div></foreignObject></g></svg>"#;
        let out = foreign_object_label_fallback_svg_text(svg);

        assert!(
            out.contains(">List&lt;Animal&gt; &amp; friends<"),
            "expected fallback text to avoid double-escaped entities: {out}"
        );
        let fallback = &out[out
            .find(r#"data-merman-foreignobject="fallback""#)
            .expect("fallback group")..];
        assert!(!fallback.contains("&amp;lt;"), "got: {fallback}");
        assert!(!fallback.contains("&amp;gt;"), "got: {fallback}");
    }

    #[test]
    fn foreign_object_overlay_wraps_break_spaces_labels_to_foreign_object_width() {
        let svg = r#"<svg xmlns="http://www.w3.org/2000/svg"><g transform="translate(20, 30)"><foreignObject width="200" height="48"><div xmlns="http://www.w3.org/1999/xhtml" style="display: table; white-space: break-spaces; line-height: 1.5; max-width: 200px; text-align: center; width: 200px;"><span class="nodeLabel">Import / WebSurface / Data Egress Gates</span></div></foreignObject></g></svg>"#;
        let out = foreign_object_label_fallback_svg_text(svg);

        assert!(
            !out.contains(">Import / WebSurface / Data Egress Gates</text>"),
            "fallback text should inherit Mermaid HTML soft wrapping instead of flattening into one SVG text line: {out}"
        );
        assert!(
            out.contains(">Import / WebSurface /<") && out.contains(">Data Egress Gates<"),
            "expected fallback text to wrap into readable SVG lines: {out}"
        );
    }

    #[test]
    fn foreign_object_overlay_keeps_nowrap_labels_as_single_fallback_line() {
        let svg = r#"<svg xmlns="http://www.w3.org/2000/svg"><g><foreignObject width="200" height="24"><div xmlns="http://www.w3.org/1999/xhtml" style="display: table-cell; white-space: nowrap; line-height: 1.5; max-width: 200px; text-align: center;"><span class="nodeLabel">Import / WebSurface / Data Egress Gates</span></div></foreignObject></g></svg>"#;
        let out = foreign_object_label_fallback_svg_text(svg);

        assert!(
            out.contains(">Import / WebSurface / Data Egress Gates</text>"),
            "explicit nowrap labels should keep the existing single-line fallback behavior: {out}"
        );
    }
}
