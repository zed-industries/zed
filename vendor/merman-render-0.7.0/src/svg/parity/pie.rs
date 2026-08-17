use super::{PieDiagramLayout, Result, SvgRenderOptions, apply_root_viewport_override, root_svg};
use crate::pie::{PIE_LEGEND_RECT_SIZE_PX, PIE_LEGEND_SPACING_PX};
use merman_core::diagrams::pie::PieDiagramRenderModel;
use std::fmt::Write as _;

const EMPTY_PIE_VIEWBOX: &str = "0 0 225 450";
const EMPTY_PIE_MAX_WIDTH: &str = "225";

fn pie_legend_rect_style(fill: &str) -> String {
    // Mermaid emits legend colors via inline `style` in rgb() form for default themes.
    // The compare tooling ignores `style`, but we keep this for human inspection parity.
    let color = css_color_to_rgb_string(fill).unwrap_or_else(|| fill.to_string());
    format!("fill: {color}; stroke: {color};")
}

fn parse_hex_rgb(s: &str) -> Option<(u8, u8, u8)> {
    let t = s.trim().strip_prefix('#').unwrap_or(s.trim());
    if t.len() != 6 || !t.chars().all(|c| c.is_ascii_hexdigit()) {
        return None;
    }
    let r = u8::from_str_radix(&t[0..2], 16).ok()?;
    let g = u8::from_str_radix(&t[2..4], 16).ok()?;
    let b = u8::from_str_radix(&t[4..6], 16).ok()?;
    Some((r, g, b))
}

fn parse_rgb_css(s: &str) -> Option<(u8, u8, u8)> {
    let inner = s.trim().strip_prefix("rgb(")?.strip_suffix(')')?;
    let mut parts = inner.split(',').map(|p| p.trim());
    let parse_channel = |part: &str| -> Option<u8> {
        let value = part.parse::<f64>().ok()?;
        if !value.is_finite() {
            return None;
        }
        Some(value.round().clamp(0.0, 255.0) as u8)
    };
    let r = parse_channel(parts.next()?)?;
    let g = parse_channel(parts.next()?)?;
    let b = parse_channel(parts.next()?)?;
    Some((r, g, b))
}

fn parse_hsl_css(s: &str) -> Option<(f64, f64, f64)> {
    let inner = s.trim().strip_prefix("hsl(")?.strip_suffix(')')?;
    let mut parts = inner.split(',').map(|p| p.trim());
    let h = parts.next()?.parse::<f64>().ok()?;
    let s = parts
        .next()?
        .strip_suffix('%')
        .unwrap_or_default()
        .parse::<f64>()
        .ok()?;
    let l = parts
        .next()?
        .strip_suffix('%')
        .unwrap_or_default()
        .parse::<f64>()
        .ok()?;
    Some((h, s, l))
}

fn hsl_to_rgb_u8(h_deg: f64, s_pct: f64, l_pct: f64) -> Option<(u8, u8, u8)> {
    if !(h_deg.is_finite() && s_pct.is_finite() && l_pct.is_finite()) {
        return None;
    }

    let h = (h_deg / 360.0).rem_euclid(1.0);
    let s = (s_pct / 100.0).clamp(0.0, 1.0);
    let l = (l_pct / 100.0).clamp(0.0, 1.0);

    if s == 0.0 {
        let v = (l * 255.0).round().clamp(0.0, 255.0) as u8;
        return Some((v, v, v));
    }

    let q = if l < 0.5 {
        l * (1.0 + s)
    } else {
        l + s - l * s
    };
    let p = 2.0 * l - q;

    fn hue_to_rgb(p: f64, q: f64, mut t: f64) -> f64 {
        if t < 0.0 {
            t += 1.0;
        }
        if t > 1.0 {
            t -= 1.0;
        }
        if t < 1.0 / 6.0 {
            return p + (q - p) * 6.0 * t;
        }
        if t < 1.0 / 2.0 {
            return q;
        }
        if t < 2.0 / 3.0 {
            return p + (q - p) * (2.0 / 3.0 - t) * 6.0;
        }
        p
    }

    let r = hue_to_rgb(p, q, h + 1.0 / 3.0);
    let g = hue_to_rgb(p, q, h);
    let b = hue_to_rgb(p, q, h - 1.0 / 3.0);

    let to_u8 = |v: f64| (v * 255.0).round().clamp(0.0, 255.0) as u8;
    Some((to_u8(r), to_u8(g), to_u8(b)))
}

fn css_color_to_rgb_string(s: &str) -> Option<String> {
    let t = s.trim();
    let (r, g, b) = parse_rgb_css(t)
        .or_else(|| parse_hex_rgb(t))
        .or_else(|| parse_hsl_css(t).and_then(|(h, s, l)| hsl_to_rgb_u8(h, s, l)))?;
    Some(format!("rgb({r}, {g}, {b})"))
}

fn pie_polar_xy(radius: f64, angle: f64) -> (f64, f64) {
    let x = radius * angle.sin();
    let y = -radius * angle.cos();
    (x, y)
}

fn pie_slice_class(effective_config: &serde_json::Value, label: &str) -> String {
    let _ = (effective_config, label);
    "pieCircle".to_string()
}

fn apply_empty_pie_root_viewport(
    model: &PieDiagramRenderModel,
    viewbox_attr: &mut String,
    max_width_attr: &mut String,
) -> bool {
    if !model.sections.is_empty() {
        return false;
    }

    let computed_root_is_finite = !viewbox_attr.contains("Infinity")
        && !viewbox_attr.contains("NaN")
        && !max_width_attr.contains("Infinity")
        && !max_width_attr.contains("NaN");
    if computed_root_is_finite {
        return false;
    }

    // Empty pie roots used to inherit upstream's invalid `-Infinity` viewport when no sections
    // were drawn. Keep a finite fallback for that legacy path, but do not clobber valid title-
    // widened roots that now come from layout bounds directly.
    *viewbox_attr = EMPTY_PIE_VIEWBOX.to_string();
    *max_width_attr = EMPTY_PIE_MAX_WIDTH.to_string();
    true
}

pub(super) fn render_pie_diagram_svg(
    layout: &PieDiagramLayout,
    semantic: &serde_json::Value,
    effective_config: &serde_json::Value,
    options: &SvgRenderOptions,
) -> Result<String> {
    let model: PieDiagramRenderModel = crate::json::from_value_ref(semantic)?;
    render_pie_diagram_svg_model(layout, &model, effective_config, options)
}

pub(super) fn render_pie_diagram_svg_model(
    layout: &PieDiagramLayout,
    model: &PieDiagramRenderModel,
    effective_config: &serde_json::Value,
    options: &SvgRenderOptions,
) -> Result<String> {
    let diagram_id = options.diagram_id.as_deref().unwrap_or("merman");
    let diagram_id_esc = super::escape_xml(diagram_id);

    let bounds = layout.bounds.clone().unwrap_or(super::Bounds {
        min_x: 0.0,
        min_y: 0.0,
        max_x: 450.0,
        max_y: 450.0,
    });
    let vb_min_x = bounds.min_x;
    let vb_min_y = bounds.min_y;
    let vb_w = (bounds.max_x - bounds.min_x).max(1.0);
    let vb_h = (bounds.max_y - bounds.min_y).max(1.0);

    let mut max_w_attr = super::fmt_max_width_px(vb_w);
    let mut viewbox_attr = format!(
        "{} {} {} {}",
        super::fmt(vb_min_x),
        super::fmt(vb_min_y),
        super::fmt(vb_w),
        super::fmt(vb_h)
    );
    apply_empty_pie_root_viewport(model, &mut viewbox_attr, &mut max_w_attr);
    let mut w_attr = super::fmt_string(vb_w);
    let mut h_attr = super::fmt_string(vb_h);
    if options.apply_root_overrides {
        apply_root_viewport_override(
            diagram_id,
            &mut viewbox_attr,
            &mut w_attr,
            &mut h_attr,
            &mut max_w_attr,
            crate::generated::pie_root_overrides_11_12_2::lookup_pie_root_viewport_override,
        );
    }

    let style_attr = format!("max-width: {max_w_attr}px; background-color: white;");

    let mut out = String::new();
    let aria_labelledby = model
        .acc_title
        .as_deref()
        .map(|_| format!("chart-title-{diagram_id_esc}"));
    let aria_describedby = model
        .acc_descr
        .as_deref()
        .map(|_| format!("chart-desc-{diagram_id_esc}"));
    root_svg::push_svg_root_open(
        &mut out,
        root_svg::SvgRootAttrs {
            width: root_svg::SvgRootWidth::Percent100,
            style_attr: Some(style_attr.as_str()),
            viewbox_attr: Some(viewbox_attr.as_str()),
            style_viewbox_order: root_svg::SvgRootStyleViewBoxOrder::ViewBoxThenStyle,
            aria_labelledby: aria_labelledby.as_deref(),
            aria_describedby: aria_describedby.as_deref(),
            trailing_newline: false,
            ..root_svg::SvgRootAttrs::new(diagram_id, "pie")
        },
    );

    if let Some(t) = model.acc_title.as_deref() {
        let _ = write!(
            &mut out,
            r#"<title id="chart-title-{id}">{text}</title>"#,
            id = diagram_id_esc,
            text = super::escape_xml(t)
        );
    }
    if let Some(d) = model.acc_descr.as_deref() {
        let _ = write!(
            &mut out,
            r#"<desc id="chart-desc-{id}">{text}</desc>"#,
            id = diagram_id_esc,
            text = super::escape_xml(d)
        );
    }

    let css = super::pie_css(diagram_id, effective_config);
    let _ = write!(&mut out, r#"<style>{}</style>"#, css);
    out.push_str(r#"<g/>"#);

    let _ = write!(
        &mut out,
        r#"<g transform="translate({x},{y})">"#,
        x = super::fmt(layout.center_x),
        y = super::fmt(layout.center_y)
    );

    let legend_position = crate::pie::pie_legend_position(effective_config);
    let pie_offset_x = if legend_position == crate::pie::PieLegendPosition::Left {
        (vb_w - 490.0).max(0.0)
    } else {
        0.0
    };
    let pie_offset_y = if legend_position == crate::pie::PieLegendPosition::Top {
        layout.legend_step_y * ((layout.legend_items.len() as f64) + 1.0)
    } else {
        0.0
    };
    let has_pie_offset = pie_offset_x != 0.0 || pie_offset_y != 0.0;
    if has_pie_offset {
        let _ = write!(
            &mut out,
            r#"<g transform="translate({x},{y})">"#,
            x = super::fmt(pie_offset_x),
            y = super::fmt(pie_offset_y)
        );
    }

    let _ = write!(
        &mut out,
        r#"<circle cx="0" cy="0" r="{r}" class="pieOuterCircle"/>"#,
        r = super::fmt(layout.outer_radius)
    );

    let inner_radius = crate::pie::pie_donut_hole(effective_config) * layout.radius;
    for slice in &layout.slices {
        let r = layout.radius;
        let slice_class = pie_slice_class(effective_config, &slice.label);
        if slice.is_full_circle {
            let d = if inner_radius > 0.0 {
                format!(
                    "M0,-{r}A{r},{r},0,1,1,0,{r}A{r},{r},0,1,1,0,-{r}M0,-{ir}A{ir},{ir},0,1,0,0,{ir}A{ir},{ir},0,1,0,0,-{ir}Z",
                    r = super::fmt(r),
                    ir = super::fmt(inner_radius)
                )
            } else {
                format!(
                    "M0,-{r}A{r},{r},0,1,1,0,{r}A{r},{r},0,1,1,0,-{r}Z",
                    r = super::fmt(r)
                )
            };
            let _ = write!(
                &mut out,
                r#"<path d="{d}" fill="{fill}" class="{class}"/>"#,
                d = d,
                fill = super::escape_xml(&slice.fill),
                class = super::escape_xml(&slice_class)
            );
        } else {
            let (x0, y0) = pie_polar_xy(r, slice.start_angle);
            let (x1, y1) = pie_polar_xy(r, slice.end_angle);
            let large = if (slice.end_angle - slice.start_angle) > std::f64::consts::PI {
                1
            } else {
                0
            };
            let d = if inner_radius > 0.0 {
                let (ix0, iy0) = pie_polar_xy(inner_radius, slice.start_angle);
                let (ix1, iy1) = pie_polar_xy(inner_radius, slice.end_angle);
                format!(
                    "M{x0},{y0}A{r},{r},0,{large},1,{x1},{y1}L{ix1},{iy1}A{ir},{ir},0,{large},0,{ix0},{iy0}Z",
                    x0 = super::fmt(x0),
                    y0 = super::fmt(y0),
                    r = super::fmt(r),
                    large = large,
                    x1 = super::fmt(x1),
                    y1 = super::fmt(y1),
                    ix1 = super::fmt(ix1),
                    iy1 = super::fmt(iy1),
                    ir = super::fmt(inner_radius),
                    ix0 = super::fmt(ix0),
                    iy0 = super::fmt(iy0)
                )
            } else {
                format!(
                    "M{x0},{y0}A{r},{r},0,{large},1,{x1},{y1}L0,0Z",
                    x0 = super::fmt(x0),
                    y0 = super::fmt(y0),
                    r = super::fmt(r),
                    large = large,
                    x1 = super::fmt(x1),
                    y1 = super::fmt(y1)
                )
            };
            let _ = write!(
                &mut out,
                r#"<path d="{d}" fill="{fill}" class="{class}"/>"#,
                d = d,
                fill = super::escape_xml(&slice.fill),
                class = super::escape_xml(&slice_class)
            );
        }
    }

    for slice in &layout.slices {
        let _ = write!(
            &mut out,
            r#"<text transform="translate({x},{y})" class="slice" style="text-anchor: middle;">{text}</text>"#,
            x = super::fmt(slice.text_x),
            y = super::fmt(slice.text_y),
            text = super::escape_xml(&format!("{}%", slice.percent))
        );
    }

    if has_pie_offset {
        out.push_str("</g>");
    }

    match model.title.as_deref() {
        Some(t) => {
            let _ = write!(
                &mut out,
                r#"<text x="0" y="{y}" class="pieTitleText">{text}</text>"#,
                y = super::fmt(-200.0),
                text = super::escape_xml(t)
            );
        }
        None => {
            let _ = write!(
                &mut out,
                r#"<text x="0" y="{y}" class="pieTitleText"/>"#,
                y = super::fmt(-200.0)
            );
        }
    }

    let legend_rect_size = PIE_LEGEND_RECT_SIZE_PX;
    let legend_text_x = legend_rect_size + PIE_LEGEND_SPACING_PX;

    for item in &layout.legend_items {
        let _ = write!(
            &mut out,
            r#"<g class="legend" transform="translate({x},{y})">"#,
            x = super::fmt(layout.legend_x),
            y = super::fmt(item.y)
        );
        let style = pie_legend_rect_style(&item.fill);
        let _ = write!(
            &mut out,
            r#"<rect width="{size}" height="{size}" style="{style}"/>"#,
            size = super::fmt(legend_rect_size),
            style = super::escape_xml(&style)
        );
        let text = if model.show_data {
            format!("{} [{}]", item.label, super::fmt(item.value))
        } else {
            item.label.clone()
        };
        let _ = write!(
            &mut out,
            r#"<text x="{x}" y="{y}">{text}</text>"#,
            x = super::fmt(legend_text_x),
            y = super::fmt(14.0),
            text = super::escape_xml(&text)
        );
        out.push_str("</g>");
    }

    out.push_str("</g></svg>\n");
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pie_legend_rect_style_serializes_default_palette_colors_as_rgb() {
        assert_eq!(
            pie_legend_rect_style("hsl(60, 100%, 57.0588235294%)"),
            "fill: rgb(255, 255, 36); stroke: rgb(255, 255, 36);"
        );
        assert_eq!(
            pie_legend_rect_style("#ECECFF"),
            "fill: rgb(236, 236, 255); stroke: rgb(236, 236, 255);"
        );
    }

    #[test]
    fn empty_pie_root_viewport_fallback_only_repairs_non_finite_roots() {
        let model = PieDiagramRenderModel::default();
        let mut viewbox = "0 0 -Infinity 450".to_string();
        let mut max_width = "NaN".to_string();

        assert!(apply_empty_pie_root_viewport(
            &model,
            &mut viewbox,
            &mut max_width,
        ));
        assert_eq!(viewbox, EMPTY_PIE_VIEWBOX);
        assert_eq!(max_width, EMPTY_PIE_MAX_WIDTH);
    }

    #[test]
    fn empty_pie_root_viewport_fallback_preserves_finite_title_bounds() {
        let model = PieDiagramRenderModel::default();
        let mut viewbox = "0 0 292.400390625 450".to_string();
        let mut max_width = "292.4".to_string();

        assert!(!apply_empty_pie_root_viewport(
            &model,
            &mut viewbox,
            &mut max_width,
        ));
        assert_eq!(viewbox, "0 0 292.400390625 450");
        assert_eq!(max_width, "292.4");
    }
}
