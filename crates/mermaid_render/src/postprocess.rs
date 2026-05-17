//! Post-processing passes applied to merman's raw SVG output.
//!
//! These run after merman has produced an SVG and adjust it so that the
//! result is themed correctly, can be parsed by `usvg`, and lays out text
//! that overflows its container. The single entry point is [`postprocess`];
//! everything else in this module is a private helper.

use anyhow::{Context as _, Result};
use gpui::{Hsla, Rgba};

use crate::{MermaidTheme, css_color};

/// Runs every post-merman transformation needed to turn the raw SVG that
/// merman produced into the final SVG that the rest of Zed can render.
///
/// The steps applied (in order) are:
///
/// 1. Convert literal `\n` sequences (which merman leaves in foreignObject
///    label HTML) into `<br/>` so the fallback step below can split them.
/// 2. Word-wrap foreignObject labels whose plain text exceeds the container
///    width (merman's text measurer can underestimate width).
/// 3. Replace `<foreignObject>` labels with plain SVG `<text>` fallbacks
///    via merman's helper, so renderers like usvg (which can't handle
///    foreignObject) still display text.
/// 4. Undo the double-escaping that merman's fallback introduces for HTML
///    entities in label text (e.g. `&lt;` becomes `&amp;lt;`).
/// 5. Run the main themed XML pass: inject theme-derived CSS into the
///    `<style>` element, rewrite colors on themed shapes, drop empty
///    elements, etc.
pub(super) fn postprocess(svg: &str, theme: &MermaidTheme) -> Result<String> {
    // Step 1: merman emits literal `\n` in foreignObject label HTML rather
    // than interpreting it as a line break. Convert to `<br/>` so the
    // fallback function below splits on it.
    let svg = svg.replace(r"\n", "<br/>");

    // Step 2: word-wrap foreignObject text that would overflow.
    let svg = wrap_foreignobject_labels(&svg);

    // Step 3: emit `<text>` fallbacks alongside the `<foreignObject>` labels.
    let svg = merman::render::foreign_object_label_fallback_svg_text(&svg);

    // Step 4: merman's fallback strips HTML tags but preserves HTML entities
    // (e.g. `&lt;`), then XML-escapes the result, turning `&lt;` into
    // `&amp;lt;`. Reverse that for the two entities that actually appear
    // in mermaid output (mostly classDiagram generics like `List~Animal~`
    // rendered as `List<Animal>`).
    let svg = svg
        .replace("&amp;lt;", "&lt;")
        .replace("&amp;gt;", "&gt;");

    // Step 5: main themed XML pass.
    let injected_css = build_injected_css(theme);
    postprocess_merman_svg(&svg, theme, &injected_css)
}

fn postprocess_merman_svg(
    svg: &str,
    theme: &MermaidTheme,
    injected_css: &str,
) -> Result<String> {
    use quick_xml::events::{BytesStart, Event};

    let mut reader = quick_xml::Reader::from_str(svg);
    reader.config_mut().check_end_names = false;
    let mut writer = quick_xml::Writer::new(Vec::new());

    let mut svg_id = String::from("merman");
    let mut in_plot_group = false;
    let mut plot_g_depth: usize = 0;
    let mut plot_path_done = false;
    let mut pie_color_idx: usize = 0;
    let mut in_legend = false;
    let mut legend_color_idx: usize = 0;
    let mut foreign_object_depth: usize = 0;
    let mut in_fallback_group = false;
    let mut skip_element = false;

    let accent_styles: Vec<AccentStyle> = theme
        .accent_colors
        .iter()
        .map(|accent| {
            let (fill, text) = accent_fill_and_text(accent.background, theme.dark_mode);
            AccentStyle {
                fill: css_color(fill),
                stroke: css_color(accent.foreground),
                text: css_color(text),
            }
        })
        .collect();
    let mut accent_g_stack: Vec<Option<usize>> = Vec::new();
    let mut node_accent_counter: usize = 0;

    loop {
        let event = reader.read_event().context("SVG parse error")?.into_owned();
        let is_start = matches!(&event, Event::Start(_));

        // Skip everything inside <foreignObject> — usvg can't handle them
        // and we already have <text> fallbacks from foreign_object_label_fallback_svg_text.
        if foreign_object_depth > 0 {
            match &event {
                Event::Start(e) if e.name().local_name().as_ref() == b"foreignObject" => {
                    foreign_object_depth += 1;
                    continue;
                }
                Event::End(e) if e.name().local_name().as_ref() == b"foreignObject" => {
                    foreign_object_depth -= 1;
                    continue;
                }
                Event::Eof => break,
                _ if foreign_object_depth > 0 => continue,
                _ => {}
            }
        }

        match event {
            Event::Eof => break,

            Event::Start(e) | Event::Empty(e) if e.name().local_name().as_ref() == b"foreignObject" => {
                if is_start {
                    foreign_object_depth = 1;
                }
                continue;
            }

            Event::Start(e) | Event::Empty(e) => {
                let new_elem: Option<BytesStart<'static>> = {
                    let tag = e.name().local_name();
                    match tag.as_ref() {
                        b"svg" => {
                            let bg = format!("background-color: {}", css_color(theme.background));
                            let mut ne = BytesStart::new("svg");
                            for attr in e.attributes() {
                                let attr = attr.context("invalid SVG attribute")?;
                                let key = attr.key.local_name();
                                let val =
                                    attr.unescape_value().context("SVG attribute value")?;
                                match key.as_ref() {
                                    b"id" => {
                                        svg_id = val.to_string();
                                        ne.push_attribute(("id", val.as_ref()));
                                    }
                                    b"style" => {
                                        let fixed =
                                            val.replace("background-color: white", &bg);
                                        ne.push_attribute(("style", fixed.as_str()));
                                    }
                                    _ => ne.push_attribute(attr),
                                }
                            }
                            Some(ne)
                        }

                        b"g" => {
                            if in_plot_group {
                                plot_g_depth += 1;
                            }
                            let mut g_accent_idx: Option<usize> = None;
                            if let Some(class_attr) = e.try_get_attribute("class")? {
                                let class_val = class_attr.unescape_value()?;
                                if class_val.as_ref() == "plot" {
                                    in_plot_group = true;
                                    plot_g_depth = 1;
                                    plot_path_done = false;
                                } else if class_val.as_ref() == "legend" {
                                    in_legend = true;
                                }
                                if !accent_styles.is_empty()
                                    && has_class_token(&class_val, "node")
                                    && !has_class_token(&class_val, "mindmap-node")
                                {
                                    let idx =
                                        node_accent_counter % accent_styles.len();
                                    node_accent_counter += 1;
                                    g_accent_idx = Some(idx);
                                }
                            }
                            if let Some(attr) = e.try_get_attribute("data-merman-foreignobject")? {
                                if attr.unescape_value()?.as_ref() == "fallback" {
                                    in_fallback_group = true;
                                }
                            }
                            if is_start {
                                accent_g_stack.push(g_accent_idx);
                            }
                            None
                        }

                        b"rect" if in_fallback_group => {
                            in_fallback_group = false;
                            let mut ne = BytesStart::new("rect");
                            for attr in e.attributes() {
                                let attr = attr?;
                                if attr.key.local_name().as_ref() == b"fill" {
                                    ne.push_attribute((
                                        "fill",
                                        css_color(theme.edge_label_background).as_str(),
                                    ));
                                } else {
                                    ne.push_attribute(attr);
                                }
                            }
                            Some(ne)
                        }

                        b"rect" if in_legend => {
                            if legend_color_idx < theme.git_branch_colors.len() {
                                let color = css_color(theme.git_branch_colors[legend_color_idx]);
                                legend_color_idx += 1;
                                let mut ne = BytesStart::new("rect");
                                for attr in e.attributes() {
                                    let attr = attr?;
                                    if attr.key.local_name().as_ref() == b"style" {
                                        let new_style =
                                            format!("fill: {color}; stroke: {color};");
                                        ne.push_attribute(("style", new_style.as_str()));
                                    } else {
                                        ne.push_attribute(attr);
                                    }
                                }
                                in_legend = false;
                                Some(ne)
                            } else {
                                in_legend = false;
                                None
                            }
                        }

                        b"rect" if in_plot_group => {
                            let bar_color = css_color(theme.git_branch_colors[0]);
                            let mut ne = BytesStart::new("rect");
                            for attr in e.attributes() {
                                let attr = attr?;
                                match attr.key.local_name().as_ref() {
                                    b"fill" => {
                                        ne.push_attribute(("fill", bar_color.as_str()));
                                    }
                                    b"stroke" => {
                                        ne.push_attribute(("stroke", bar_color.as_str()));
                                    }
                                    _ => ne.push_attribute(attr),
                                }
                            }
                            Some(ne)
                        }

                        b"path" => {
                            let class_str = e
                                .try_get_attribute("class")?
                                .map(|a| a.unescape_value().map(|v| v.to_string()))
                                .transpose()?;

                            if class_str.as_deref() == Some("pieCircle")
                                && pie_color_idx < theme.git_branch_colors.len()
                            {
                                let color = css_color(theme.git_branch_colors[pie_color_idx]);
                                pie_color_idx += 1;
                                let mut ne = BytesStart::new("path");
                                for attr in e.attributes() {
                                    let attr = attr?;
                                    if attr.key.local_name().as_ref() == b"fill" {
                                        ne.push_attribute(("fill", color.as_str()));
                                    } else {
                                        ne.push_attribute(attr);
                                    }
                                }
                                Some(ne)
                            } else if in_plot_group
                                && !plot_path_done
                                && e.try_get_attribute("stroke")?.is_some()
                            {
                                plot_path_done = true;
                                let line_color = css_color(theme.git_branch_colors[1]);
                                let mut ne = BytesStart::new("path");
                                for attr in e.attributes() {
                                    let attr = attr?;
                                    if attr.key.local_name().as_ref() == b"stroke" {
                                        ne.push_attribute(("stroke", line_color.as_str()));
                                    } else {
                                        ne.push_attribute(attr);
                                    }
                                }
                                Some(ne)
                            } else {
                                None
                            }
                        }

                        b"text" => {
                            let fill_val = e
                                .try_get_attribute("fill")?
                                .map(|a| a.unescape_value().map(|v| v.to_string()))
                                .transpose()?;
                            let needs_fix = matches!(
                                fill_val.as_deref(),
                                Some("#333") | Some("")
                            );
                            if needs_fix {
                                let mut ne = BytesStart::new("text");
                                for attr in e.attributes() {
                                    let attr = attr?;
                                    if attr.key.local_name().as_ref() == b"fill" {
                                        ne.push_attribute((
                                            "fill",
                                            css_color(theme.text_color).as_str(),
                                        ));
                                    } else {
                                        ne.push_attribute(attr);
                                    }
                                }
                                Some(ne)
                            } else {
                                None
                            }
                        }

                        b"rect" if !in_plot_group && !in_legend => {
                            let width_val = e
                                .try_get_attribute("width")?
                                .map(|a| a.unescape_value().map(|v| v.to_string()))
                                .transpose()?;
                            let height_val = e
                                .try_get_attribute("height")?
                                .map(|a| a.unescape_value().map(|v| v.to_string()))
                                .transpose()?;
                            let bad_width =
                                matches!(width_val.as_deref(), Some("") | None);
                            let bad_height =
                                matches!(height_val.as_deref(), Some("") | None);
                            if bad_width || bad_height {
                                skip_element = true;
                                None
                            } else {
                                None
                            }
                        }

                        _ => None,
                    }
                };

                let elem = new_elem.unwrap_or(e);

                // Automatically apply accent colors to shapes and text
                // inside node groups. The accent index is assigned
                // round-robin from the theme's accent palette when
                // entering a <g class="node ..."> group.
                let elem = if !accent_styles.is_empty() {
                    let (is_shape, is_text, tag_string) = {
                        let tag = elem.name().local_name();
                        let tag_bytes = tag.as_ref();
                        (
                            matches!(
                                tag_bytes,
                                b"rect"
                                    | b"path"
                                    | b"circle"
                                    | b"polygon"
                                    | b"ellipse"
                            ),
                            tag_bytes == b"text",
                            String::from_utf8_lossy(tag_bytes).into_owned(),
                        )
                    };

                    if is_shape || is_text {
                        let accent_idx =
                            accent_g_stack.iter().rev().find_map(|x| *x);

                        if let Some(idx) = accent_idx {
                            let style = &accent_styles[idx];
                            let mut ne = BytesStart::new(tag_string);
                            let mut had_fill = false;
                            let mut had_stroke = false;

                            for attr in elem.attributes() {
                                let attr = attr.context("accent attr")?;
                                match attr.key.local_name().as_ref() {
                                    b"fill" => {
                                        had_fill = true;
                                        let color = if is_text {
                                            &style.text
                                        } else {
                                            &style.fill
                                        };
                                        ne.push_attribute(("fill", color.as_str()));
                                    }
                                    b"stroke" if is_shape => {
                                        had_stroke = true;
                                        ne.push_attribute((
                                            "stroke",
                                            style.stroke.as_str(),
                                        ));
                                    }
                                    _ => ne.push_attribute(attr),
                                }
                            }

                            if !had_fill {
                                let color = if is_text {
                                    &style.text
                                } else {
                                    &style.fill
                                };
                                ne.push_attribute(("fill", color.as_str()));
                            }
                            if is_shape && !had_stroke {
                                ne.push_attribute((
                                    "stroke",
                                    style.stroke.as_str(),
                                ));
                            }

                            ne
                        } else {
                            elem
                        }
                    } else {
                        elem
                    }
                } else {
                    elem
                };

                if skip_element {
                    skip_element = false;
                } else if is_start {
                    writer.write_event(Event::Start(elem))?;
                } else {
                    writer.write_event(Event::Empty(elem))?;
                }
            }

            Event::End(e) => {
                if e.name().local_name().as_ref() == b"style" {
                    let scoped_css: String = injected_css
                        .lines()
                        .map(|line| {
                            let trimmed = line.trim();
                            if trimmed.starts_with('.')
                                || trimmed.starts_with("foreignObject")
                            {
                                if let Some(brace) = trimmed.find('{') {
                                    let (selectors, rest) = trimmed.split_at(brace);
                                    let scoped_selectors: Vec<String> = selectors
                                        .split(',')
                                        .map(|s| format!("#{svg_id} {}", s.trim()))
                                        .collect();
                                    format!(
                                        "        {}{}\n",
                                        scoped_selectors.join(", "),
                                        rest
                                    )
                                } else {
                                    format!("{line}\n")
                                }
                            } else {
                                format!("{line}\n")
                            }
                        })
                        .collect();
                    writer.get_mut().extend_from_slice(scoped_css.as_bytes());
                }

                if e.name().local_name().as_ref() == b"g" {
                    accent_g_stack.pop();
                    if in_plot_group {
                        plot_g_depth -= 1;
                        if plot_g_depth == 0 {
                            in_plot_group = false;
                        }
                    }
                }

                writer.write_event(Event::End(e))?;
            }

            event => writer.write_event(event)?,
        }
    }

    let svg = String::from_utf8(writer.into_inner()).context("SVG output is not valid UTF-8")?;
    let svg = sanitize_nan_colors(&svg);
    Ok(strip_unsupported_css(&svg))
}

/// Strip CSS rules that usvg's `simplecss` parser cannot handle:
/// `@keyframes` blocks, `:root` declarations, and any remaining `:not()`
/// selectors. These produce log warnings and are never applied.
fn strip_unsupported_css(svg: &str) -> String {
    let Some(style_start) = svg.find("<style>") else {
        return svg.to_string();
    };
    let content_start = style_start + "<style>".len();
    let Some(content_len) = svg[content_start..].find("</style>") else {
        return svg.to_string();
    };
    let content_end = content_start + content_len;
    let css = &svg[content_start..content_end];

    let mut cleaned = String::with_capacity(css.len());
    let bytes = css.as_bytes();
    let mut i = 0;

    while i < bytes.len() {
        if css[i..].starts_with("@keyframes") {
            i = skip_css_block(bytes, i);
            continue;
        }

        let Some(brace_offset) = css[i..].find('{') else {
            cleaned.push_str(&css[i..]);
            break;
        };
        let selector = &css[i..i + brace_offset];
        if selector.contains(":root") || selector.contains(":not(") {
            i = skip_css_block(bytes, i);
            continue;
        }

        let block_end = skip_css_block(bytes, i);
        cleaned.push_str(&css[i..block_end]);
        i = block_end;
    }

    // Strip CSS angle units (`deg`) from rotate() — usvg parses these as
    // SVG transform values which use bare numbers, not CSS angle units.
    let cleaned = strip_css_angle_units(&cleaned);

    let mut result = String::with_capacity(svg.len());
    result.push_str(&svg[..content_start]);
    result.push_str(&cleaned);
    result.push_str(&svg[content_end..]);
    result
}

fn strip_css_angle_units(css: &str) -> String {
    let mut result = String::with_capacity(css.len());
    let mut remaining = css;
    while let Some(pos) = remaining.find("deg)") {
        result.push_str(&remaining[..pos]);
        result.push(')');
        remaining = &remaining[pos + 4..];
    }
    result.push_str(remaining);
    result
}

/// Insert `<br/>` at word boundaries in foreignObject text that is wider
/// than its container. This runs before the foreignObject-to-text fallback
/// conversion so that the fallback function splits the text into multiple
/// `<text>` elements.
fn wrap_foreignobject_labels(svg: &str) -> String {
    const AVG_CHAR_WIDTH: f64 = 8.5;
    // Merman's vendored text measurer underestimates character widths
    // by roughly 40%. Scale the foreignObject width up so we only wrap
    // text that genuinely overflows the node at actual rendering size.
    const WIDTH_SCALE: f64 = 1.4;

    let fo_tag = "<foreignObject";
    let fo_close = "</foreignObject>";

    let mut result = String::with_capacity(svg.len() + 256);
    let mut remaining = svg;

    while let Some(fo_start) = remaining.find(fo_tag) {
        let Some(tag_end) = remaining[fo_start..].find('>') else {
            break;
        };
        let tag = &remaining[fo_start..fo_start + tag_end + 1];

        let width = tag
            .find("width=\"")
            .and_then(|i| {
                let after = &tag[i + 7..];
                after.find('"').and_then(|end| after[..end].parse::<f64>().ok())
            })
            .unwrap_or(0.0);

        let content_start = fo_start + tag_end + 1;
        let Some(close_rel) = remaining[content_start..].find(fo_close) else {
            break;
        };
        let content_end = content_start + close_rel;
        let fo_end = content_end + fo_close.len();

        let available_width = width * WIDTH_SCALE;
        if available_width <= 0.0 {
            result.push_str(&remaining[..fo_end]);
            remaining = &remaining[fo_end..];
            continue;
        }

        let content = &remaining[content_start..content_end];

        // If the content already has explicit line breaks, skip wrapping.
        if content.contains("<br") {
            result.push_str(&remaining[..fo_end]);
            remaining = &remaining[fo_end..];
            continue;
        }

        // Extract plain text (strip HTML tags) for width estimation.
        let plain: String = {
            let mut text = String::new();
            let mut in_tag = false;
            for ch in content.chars() {
                match ch {
                    '<' => in_tag = true,
                    '>' => in_tag = false,
                    _ if !in_tag => text.push(ch),
                    _ => {}
                }
            }
            text.trim().to_string()
        };

        let estimated_width = plain.len() as f64 * AVG_CHAR_WIDTH;
        if estimated_width <= available_width || plain.split_whitespace().count() <= 1 {
            result.push_str(&remaining[..fo_end]);
            remaining = &remaining[fo_end..];
            continue;
        }

        // Build wrapped text with <br/> at word boundaries.
        let mut lines: Vec<String> = Vec::new();
        let mut current_line = String::new();
        for word in plain.split_whitespace() {
            let candidate_width = if current_line.is_empty() {
                word.len() as f64 * AVG_CHAR_WIDTH
            } else {
                (current_line.len() + 1 + word.len()) as f64 * AVG_CHAR_WIDTH
            };
            if !current_line.is_empty() && candidate_width > available_width {
                lines.push(current_line);
                current_line = word.to_string();
            } else if current_line.is_empty() {
                current_line = word.to_string();
            } else {
                current_line.push(' ');
                current_line.push_str(word);
            }
        }
        if !current_line.is_empty() {
            lines.push(current_line);
        }

        let wrapped_html = lines.join("<br/>");

        // Replace the inner text in the original HTML.
        // Find the deepest text node and replace it.
        let new_content = if let Some(last_open) = content.rfind('>') {
            if let Some(next_close) = content[last_open..].find('<') {
                let text_start = last_open + 1;
                let text_end = last_open + next_close;
                let mut new = String::new();
                new.push_str(&content[..text_start]);
                new.push_str(&wrapped_html);
                new.push_str(&content[text_end..]);
                new
            } else {
                wrapped_html
            }
        } else {
            wrapped_html
        };

        result.push_str(&remaining[..content_start]);
        result.push_str(&new_content);
        result.push_str(&remaining[content_end..fo_end]);
        remaining = &remaining[fo_end..];
    }

    result.push_str(remaining);
    result
}

fn skip_css_block(bytes: &[u8], start: usize) -> usize {
    let mut depth = 0;
    let mut i = start;
    while i < bytes.len() {
        if bytes[i] == b'{' {
            depth += 1;
        } else if bytes[i] == b'}' {
            depth -= 1;
            if depth == 0 {
                return i + 1;
            }
        }
        i += 1;
    }
    i
}

/// Replace any `hsl(…, NaN%)` values that merman's internal color derivation
/// can produce. These are unparseable by usvg and fall back to black.
fn sanitize_nan_colors(svg: &str) -> String {
    let mut result = String::with_capacity(svg.len());
    let mut remaining = svg;
    while let Some(start) = remaining.find("hsl(") {
        let after_hsl = start + 4;
        if let Some(end) = remaining[after_hsl..].find(')') {
            let hsl_body = &remaining[after_hsl..after_hsl + end];
            if hsl_body.contains("NaN") {
                result.push_str(&remaining[..start]);
                result.push_str("transparent");
                remaining = &remaining[after_hsl + end + 1..];
                continue;
            }
        }
        result.push_str(&remaining[..after_hsl]);
        remaining = &remaining[after_hsl..];
    }
    result.push_str(remaining);
    result
}

fn has_class_token(class: &str, token: &str) -> bool {
    class.split_whitespace().any(|t| t == token)
}

/// Returns a contrasting text color (black or white) for a given background.
fn text_color_for_bg(color: Hsla) -> Hsla {
    let rgba = Rgba::from(color);
    if luma(rgba) > 0.4 { Hsla::black() } else { Hsla::white() }
}

/// Returns relative luminance per the sRGB formula (range 0.0 – 1.0).
fn luma(rgba: Rgba) -> f32 {
    fn linearize(c: f32) -> f32 {
        if c <= 0.04045 { c / 12.92 } else { ((c + 0.055) / 1.055).powf(2.4) }
    }
    0.2126 * linearize(rgba.r) + 0.7152 * linearize(rgba.g) + 0.0722 * linearize(rgba.b)
}

fn mindmap_section_css(theme: &MermaidTheme) -> String {
    let colors = &theme.git_branch_colors;
    let mut css = String::new();

    let emit = |css: &mut String, selector: &str, color: Hsla| {
        let txt = css_color(text_color_for_bg(color));
        let color = css_color(color);
        css.push_str(&format!(
            "{selector} rect, {selector} path, {selector} circle, {selector} polygon \
             {{ fill: {color} !important; }}\n\
             {selector} text, {selector} span \
             {{ fill: {txt} !important; color: {txt} !important; }}\n\
             {selector} foreignObject div, {selector} foreignObject span, {selector} foreignObject p \
             {{ color: {txt} !important; }}\n\
             .section-edge{} {{ stroke: {color} !important; }}\n",
            selector.trim_start_matches(".section"),
        ));
    };

    emit(&mut css, ".section-root.section--1", colors[0]);
    emit(&mut css, ".section--1", colors[1]);

    for i in 0..=10 {
        let ci = 2 + (i % 6);
        emit(&mut css, &format!(".section-{i}"), colors[ci]);
    }

    css
}

fn git_branch_css(theme: &MermaidTheme) -> String {
    let mut css = String::new();
    for i in 0..8 {
        let c = css_color(theme.git_branch_colors[i]);
        let lbl = css_color(theme.git_branch_label_colors[i]);
        css.push_str(&format!(
            ".commit{i} {{ stroke: {c}; fill: {c}; }}
             .arrow{i} {{ stroke: {c}; }}
             .label{i} {{ fill: {c}; }}
             .branch-label{i} {{ fill: {lbl}; }}\n"
        ));
    }
    css
}

/// Constructs the theme-derived CSS that gets spliced into the SVG's
/// `<style>` element during the main XML pass.
fn build_injected_css(theme: &MermaidTheme) -> String {
    format!(
        r#"
        /* text in foreignObject labels */
        foreignObject div, foreignObject span, foreignObject p {{
            font-family: {font};
            font-size: 16px;
            color: {text};
        }}
        foreignObject p {{ margin: 0; }}
        foreignObject {{ overflow: visible; }}
        foreignObject div {{ max-width: none !important; }}
        .label-group foreignObject {{ font-weight: bold; }}

        /* mindmap section colors (use git branch accent palette) */
        {mindmap_section_css}

        /* state diagram */
        g.stateGroup rect {{ fill: {primary} !important; stroke: {border} !important; }}
        g.stateGroup text {{ fill: {text} !important; }}
        g.stateGroup .state-title {{ fill: {text} !important; }}
        .stateGroup .composit {{ fill: {background} !important; }}
        .stateGroup .alt-composit {{ fill: {tertiary} !important; }}
        .state-note {{ stroke: {note_border} !important; fill: {note_bg} !important; }}
        .state-note text {{ fill: {note_text} !important; }}
        .stateLabel .box {{ fill: {primary} !important; }}
        .stateLabel text {{ fill: {text} !important; }}
        .node circle.state-start {{ fill: {line} !important; stroke: {line} !important; }}
        .node .fork-join {{ fill: {line} !important; stroke: {line} !important; }}
        .node circle.state-end {{ fill: {border} !important; stroke: {background} !important; }}
        .end-state-inner {{ fill: {background} !important; }}
        .node rect, .node path {{ fill: {primary} !important; stroke: {border} !important; }}
        .node polygon {{ fill: {primary} !important; stroke: {border} !important; }}
        .label-container path {{ fill: {primary} !important; stroke: {border} !important; }}
        .statediagram-cluster rect {{ fill: {primary} !important; stroke: {border} !important; }}
        .statediagram-cluster.statediagram-cluster .inner {{ fill: {background} !important; }}
        .statediagram-cluster.statediagram-cluster-alt .inner {{ fill: {tertiary} !important; }}
        .statediagram-state rect.divider {{ fill: {tertiary} !important; }}
        .statediagram-note rect {{ fill: {note_bg} !important; stroke: {note_border} !important; }}
        .statediagram-note text {{ fill: {note_text} !important; }}
        .statediagramTitleText {{ fill: {text} !important; }}
        .transition {{ stroke: {line} !important; }}
        .cluster-label, .nodeLabel {{ color: {text} !important; }}
        defs #statediagram-barbEnd {{ fill: {line} !important; stroke: {line} !important; }}
        #statediagram-barbEnd {{ fill: {line} !important; }}
        .edgeLabel .label rect {{ fill: {primary} !important; }}
        .edgeLabel rect {{ fill: {primary} !important; background-color: {primary} !important; }}
        .edgeLabel .label text {{ fill: {text} !important; }}
        .edgeLabel p {{ background-color: {primary} !important; }}
        .edgeLabel {{ background-color: {primary} !important; }}

        /* sequence diagram */
        .actor {{ stroke: {actor_border}; fill: {actor_bg} !important; }}
        text.actor>tspan {{ fill: {actor_text} !important; stroke: none; }}
        .labelText, .labelText>tspan {{ fill: {actor_text} !important; }}
        .actor-line {{ stroke: {actor_border} !important; }}
        .messageLine0 {{ stroke: {text} !important; }}
        .messageLine1 {{ stroke: {text} !important; }}
        #arrowhead path {{ fill: {text} !important; stroke: {text} !important; }}
        #crosshead path {{ fill: {text} !important; stroke: {text} !important; }}
        .messageText {{ fill: {text} !important; }}
        .loopText, .loopText>tspan {{ fill: {text} !important; }}
        .loopLine {{ stroke: {actor_border} !important; fill: {actor_border} !important; }}
        .note {{ stroke: {note_border} !important; fill: {note_bg} !important; }}
        .noteText, .noteText>tspan {{ fill: {note_text} !important; }}
        .activation0, .activation1, .activation2 {{ fill: {secondary} !important; stroke: {border} !important; }}
        .labelBox {{ stroke: {actor_border} !important; fill: {actor_bg} !important; }}
        .actor-man line {{ stroke: {actor_border} !important; fill: {actor_bg} !important; }}
        .actor-man circle {{ stroke: {actor_border} !important; fill: {actor_bg} !important; }}

        /* pie chart */
        .pieTitleText {{ fill: {text} !important; }}
        .slice {{ fill: {text} !important; }}
        .legend text {{ fill: {text} !important; }}
        .pieOuterCircle {{ stroke: {border} !important; }}
        .pieCircle {{ stroke: {border} !important; }}

        /* journey diagram section/task fills */
        .task-type-0, .section-type-0 {{ fill: {primary} !important; }}
        .task-type-1, .section-type-1 {{ fill: {secondary} !important; }}
        .task-type-2, .section-type-2 {{ fill: {tertiary} !important; }}
        .task-type-3, .section-type-3 {{ fill: {primary} !important; }}
        .task-type-4, .section-type-4 {{ fill: {secondary} !important; }}
        .task-type-5, .section-type-5 {{ fill: {tertiary} !important; }}
        .task-type-6, .section-type-6 {{ fill: {primary} !important; }}
        .task-type-7, .section-type-7 {{ fill: {secondary} !important; }}

        /* ER diagram */
        .relationshipLabelBox {{ fill: {tertiary} !important; opacity: 0.7; background-color: {tertiary} !important; }}
        .labelBkg {{ background-color: {tertiary} !important; }}
        .edgeLabel .label {{ fill: {border} !important; }}
        .label {{ color: {text} !important; }}
        .relationshipLine {{ stroke: {line} !important; fill: none !important; }}
        .entityBox {{ fill: {primary}; stroke: {border}; }}
        .node .row-rect-odd path {{ fill: {er_odd} !important; }}
        .node .row-rect-even path {{ fill: {er_even} !important; }}

        /* edges and markers */
        .edge-thickness-normal {{ stroke-width: 1px; }}
        .relation {{ stroke: {line}; stroke-width: 1; fill: none; }}
        .edgePaths path {{ fill: none; }}
        .marker {{ fill: {line} !important; stroke: {line} !important; }}
        .marker.er {{ fill: none !important; stroke: {line} !important; }}
        .composition {{ fill: {line} !important; stroke: {line} !important; stroke-width: 1; }}
        .extension {{ fill: transparent !important; stroke: {line} !important; stroke-width: 1; }}
        .aggregation {{ fill: transparent !important; stroke: {line} !important; stroke-width: 1; }}
        .dependency {{ fill: {line} !important; stroke: {line} !important; stroke-width: 1; }}
        .lollipop {{ fill: {primary} !important; stroke: {line} !important; stroke-width: 1; }}

        /* gantt chart overrides (need !important to beat #id-scoped rules) */
        .section0 {{ fill: {tertiary} !important; }}
        .section2 {{ fill: {primary} !important; }}
        .section1, .section3 {{ fill: {secondary} !important; opacity: 0.2; }}
        .sectionTitle0, .sectionTitle1, .sectionTitle2, .sectionTitle3 {{ fill: {text} !important; }}
        .sectionTitle {{ font-family: {font} !important; }}
        .task0, .task1, .task2, .task3 {{ fill: {primary} !important; stroke: {border} !important; }}
        .taskText0, .taskText1, .taskText2, .taskText3 {{ fill: {text} !important; }}
        .taskTextOutside0, .taskTextOutside1, .taskTextOutside2, .taskTextOutside3 {{ fill: {text} !important; }}
        .taskTextOutsideRight {{ fill: {text} !important; font-family: {font} !important; }}
        .taskTextOutsideLeft {{ fill: {text} !important; }}
        .active0, .active1, .active2, .active3 {{ fill: {secondary} !important; stroke: {border} !important; }}
        .activeText0, .activeText1, .activeText2, .activeText3 {{ fill: {text} !important; }}
        .done0, .done1, .done2, .done3 {{ stroke: {border} !important; fill: {secondary} !important; stroke-width: 2; }}
        .doneText0, .doneText1, .doneText2, .doneText3 {{ fill: {text} !important; }}
        .doneCritText0, .doneCritText1, .doneCritText2, .doneCritText3 {{ fill: {text} !important; }}
        .activeCritText0, .activeCritText1, .activeCritText2, .activeCritText3 {{ fill: {text} !important; }}
        .titleText {{ fill: {text} !important; font-family: {font} !important; }}
        .grid .tick text {{ fill: {text} !important; font-family: {font} !important; }}
        .grid .tick {{ stroke: {border} !important; }}

        /* gitgraph branch colors */
        {git_branch_css}
        .commit-merge  {{ stroke: {primary}; fill: {primary}; }}
        .commit-reverse {{ stroke: {primary}; fill: {primary}; stroke-width: 3; }}
        .commit-highlight-inner {{ stroke: {primary}; fill: {primary}; }}
        .tag-label {{ font-size: 10px; }}
        .tag-label-bkg {{ fill: {primary}; stroke: {border}; }}
        .tag-hole {{ fill: {line}; }}
        .commit-label {{ fill: {text}; }}
        .commit-label-bkg {{ fill: {edge_label_bg}; }}
        .commit-id, .commit-msg, .branch-label {{
            fill: {text}; color: {text};
            font-family: {font};
        }}
    "#,
        font = theme.font_family,
        text = css_color(theme.text_color),
        line = css_color(theme.line_color),
        primary = css_color(theme.primary_color),
        border = css_color(theme.primary_border_color),
        secondary = css_color(theme.secondary_color),
        tertiary = css_color(theme.tertiary_color),
        background = css_color(theme.background),
        edge_label_bg = css_color(theme.edge_label_background),
        actor_bg = css_color(theme.actor_background),
        actor_border = css_color(theme.actor_border),
        actor_text = css_color(text_color_for_bg(theme.actor_background)),
        note_bg = css_color(theme.note_background),
        note_border = css_color(theme.note_border),
        note_text = css_color(text_color_for_bg(theme.note_background)),
        er_odd = css_color(theme.er_attr_bg_odd),
        er_even = css_color(theme.er_attr_bg_even),
        mindmap_section_css = mindmap_section_css(theme),
        git_branch_css = git_branch_css(theme),
    )
}

struct AccentStyle {
    fill: String,
    stroke: String,
    text: String,
}

/// Derive a fill color and contrasting text color from an accent background.
///
/// On dark themes the fill is darkened until text contrast is sufficient;
/// on light themes it is lightened. Returns `(fill, text_color)`.
fn accent_fill_and_text(background: Hsla, dark_mode: bool) -> (Hsla, Hsla) {
    let mut color = background;

    if dark_mode {
        for _ in 0..50 {
            if luma(Rgba::from(color)) <= 0.18 {
                break;
            }
            color.l = (color.l - 0.02).max(0.0);
        }
    } else {
        for _ in 0..50 {
            if luma(Rgba::from(color)) >= 0.35 {
                break;
            }
            color.l = (color.l + 0.02).min(1.0);
        }
    }

    let text = if dark_mode { Hsla::white() } else { Hsla::black() };
    (color, text)
}

#[cfg(test)]
mod tests {
    //! Tests that demonstrate defects in the current string-based post-processing
    //! passes. They should turn green once those passes are reimplemented on top
    //! of the existing quick-xml event loop.

    use super::*;
    use crate::MermaidTheme;

    /// `sanitize_nan_colors` greedily replaces any substring matching
    /// `hsl(...NaN...)`, regardless of XML position. A label whose text content
    /// happens to contain the literal characters `hsl(NaN, 0%, 50%)` is rewritten
    /// as `transparent`. A structure-aware pass should only touch CSS / attribute
    /// values.
    #[test]
    fn hsl_nan_in_text_content_is_preserved() {
        let svg = r#"<svg xmlns="http://www.w3.org/2000/svg" id="test"><text x="0" y="0">value hsl(NaN, 0%, 50%) here</text></svg>"#;
        let theme = MermaidTheme::default();
        let result = postprocess_merman_svg(svg, &theme, "").expect("postprocess");
        assert!(
            result.contains("hsl(NaN, 0%, 50%)"),
            "expected literal 'hsl(NaN, 0%, 50%)' to survive in <text> content; got:\n{result}",
        );
    }

    /// `wrap_foreignobject_labels` extracts the width via `find("width=\"")`,
    /// which matches any attribute whose name ends in `width`. A foreignObject
    /// with `data-original-width="9999"` before `width="100"` parses 9999, so a
    /// long label that should wrap at width=100 is left as a single line.
    #[test]
    fn wrap_foreignobject_reads_actual_width_attr() {
        let long_text = "word ".repeat(20);
        let svg = format!(
            r#"<svg xmlns="http://www.w3.org/2000/svg"><foreignObject data-original-width="9999" width="100" height="20"><div xmlns="http://www.w3.org/1999/xhtml">{long_text}</div></foreignObject></svg>"#,
        );
        let result = wrap_foreignobject_labels(&svg);
        assert!(
            result.contains("<br/>"),
            "expected the label to be wrapped using width=100 (not 9999); got:\n{result}",
        );
    }

    /// `strip_unsupported_css` looks for the literal token `<style>` and so
    /// silently does nothing when the style element carries attributes such
    /// as `<style type="text/css">`. A structure-aware pass should find the
    /// element regardless of attributes.
    #[test]
    fn strip_css_handles_style_element_with_attributes() {
        let svg = r#"<svg xmlns="http://www.w3.org/2000/svg" id="test"><style type="text/css">@keyframes wobble { 0% { opacity: 1 } 100% { opacity: 0 } } .x { fill: red; }</style></svg>"#;
        let result = strip_unsupported_css(svg);
        assert!(
            !result.contains("@keyframes"),
            "expected @keyframes to be stripped from <style type=\"text/css\">; got:\n{result}",
        );
    }
}
