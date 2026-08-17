use crate::Result;
use crate::config::{config_f64, config_string, value_at};
use crate::model::{Bounds, PieDiagramLayout, PieLegendItemLayout, PieSliceLayout};
use crate::text::{TextMeasurer, TextStyle};
use merman_core::diagrams::pie::{PieDiagramRenderModel, PieRenderSection};
use ryu_js::Buffer;

pub(crate) const PIE_LEGEND_RECT_SIZE_PX: f64 = 18.0;
pub(crate) const PIE_LEGEND_SPACING_PX: f64 = 4.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PieLegendPosition {
    Top,
    Bottom,
    Left,
    Right,
    Center,
}

#[derive(Debug, Clone)]
struct ColorScale {
    palette: Vec<String>,
    mapping: std::collections::HashMap<String, usize>,
    next: usize,
}

#[derive(Debug, Clone, Copy)]
struct Rgb01 {
    r: f64,
    g: f64,
    b: f64,
}

#[derive(Debug, Clone, Copy)]
struct Hsl {
    h_deg: f64,
    s_pct: f64,
    l_pct: f64,
}

fn round_1e10(v: f64) -> f64 {
    let v = (v * 1e10).round() / 1e10;
    if v == -0.0 { 0.0 } else { v }
}

fn fmt_js_1e10(v: f64) -> String {
    let v = round_1e10(v);
    let mut b = Buffer::new();
    b.format_finite(v).to_string()
}

fn round_hsl_1e10(mut hsl: Hsl) -> Hsl {
    // Match Mermaid's base theme output: wrap using remainder without forcing positive hue.
    // (JS `%` keeps the sign, so negative hues remain negative.)
    hsl.h_deg = round_1e10(hsl.h_deg) % 360.0;
    hsl.s_pct = round_1e10(hsl.s_pct).clamp(0.0, 100.0);
    hsl.l_pct = round_1e10(hsl.l_pct).clamp(0.0, 100.0);
    hsl
}

fn parse_hex_rgb01(s: &str) -> Option<Rgb01> {
    let s = s.trim();
    let s = s.strip_prefix('#')?;
    if s.len() != 6 {
        return None;
    }
    let r = u8::from_str_radix(&s[0..2], 16).ok()? as f64 / 255.0;
    let g = u8::from_str_radix(&s[2..4], 16).ok()? as f64 / 255.0;
    let b = u8::from_str_radix(&s[4..6], 16).ok()? as f64 / 255.0;
    Some(Rgb01 { r, g, b })
}

fn rgb01_to_hsl(rgb: Rgb01) -> Hsl {
    let r = rgb.r;
    let g = rgb.g;
    let b = rgb.b;

    let max = r.max(g.max(b));
    let min = r.min(g.min(b));
    let mut h = 0.0;
    let mut s = 0.0;
    let l = (max + min) / 2.0;

    if max != min {
        let d = max - min;
        s = if l > 0.5 {
            d / (2.0 - max - min)
        } else {
            d / (max + min)
        };

        h = if max == r {
            (g - b) / d + if g < b { 6.0 } else { 0.0 }
        } else if max == g {
            (b - r) / d + 2.0
        } else {
            (r - g) / d + 4.0
        };
        h /= 6.0;
    }

    round_hsl_1e10(Hsl {
        h_deg: h * 360.0,
        s_pct: s * 100.0,
        l_pct: l * 100.0,
    })
}

fn parse_hsl(s: &str) -> Option<Hsl> {
    let s = s.trim();
    let inner = s.strip_prefix("hsl(")?.strip_suffix(')')?;
    let parts: Vec<&str> = inner.split(',').map(|p| p.trim()).collect();
    if parts.len() != 3 {
        return None;
    }
    let h = parts[0].parse::<f64>().ok()?;
    let s_pct = parts[1].trim_end_matches('%').parse::<f64>().ok()?;
    let l_pct = parts[2].trim_end_matches('%').parse::<f64>().ok()?;
    Some(round_hsl_1e10(Hsl {
        h_deg: h,
        s_pct,
        l_pct,
    }))
}

fn adjust_hsl(mut hsl: Hsl, h_delta: f64, s_delta: f64, l_delta: f64) -> Hsl {
    hsl.h_deg = (hsl.h_deg + h_delta) % 360.0;
    hsl.s_pct = (hsl.s_pct + s_delta).clamp(0.0, 100.0);
    hsl.l_pct = (hsl.l_pct + l_delta).clamp(0.0, 100.0);
    round_hsl_1e10(hsl)
}

fn fmt_hsl(hsl: Hsl) -> String {
    format!(
        "hsl({}, {}%, {}%)",
        fmt_js_1e10(hsl.h_deg),
        fmt_js_1e10(hsl.s_pct),
        fmt_js_1e10(hsl.l_pct)
    )
}

fn adjust_color_to_hsl_string(
    color: &str,
    h_delta: f64,
    s_delta: f64,
    l_delta: f64,
) -> Option<String> {
    let base = if let Some(rgb) = parse_hex_rgb01(color) {
        rgb01_to_hsl(rgb)
    } else if let Some(hsl) = parse_hsl(color) {
        hsl
    } else {
        return None;
    };
    Some(fmt_hsl(adjust_hsl(base, h_delta, s_delta, l_delta)))
}

impl ColorScale {
    fn default_palette() -> Vec<String> {
        // Default theme colors as emitted by Mermaid 11.12.2 in SVG.
        //
        // Mermaid derives this palette from `theme-default.js` `pie1..pie12` (using `adjust()`),
        // where the base colors are:
        // - primaryColor = "#ECECFF"
        // - secondaryColor = "#ffffde"
        // - tertiaryColor = "hsl(80, 100%, 96.2745098039%)"
        //
        // Note: `adjust(...)` serializes as `hsl(...)` (not hex), so the palette contains a mix.
        const PRIMARY: &str = "#ECECFF";
        const SECONDARY: &str = "#ffffde";
        const TERTIARY: &str = "hsl(80, 100%, 96.2745098039%)";

        let pie3 = adjust_color_to_hsl_string(TERTIARY, 0.0, 0.0, -40.0)
            .unwrap_or_else(|| "hsl(80, 100%, 56.2745098039%)".to_string());
        let pie4 = adjust_color_to_hsl_string(PRIMARY, 0.0, 0.0, -10.0)
            .unwrap_or_else(|| "hsl(240, 100%, 86.2745098039%)".to_string());
        let pie5 = adjust_color_to_hsl_string(SECONDARY, 0.0, 0.0, -30.0)
            .unwrap_or_else(|| "hsl(60, 100%, 57.0588235294%)".to_string());
        let pie6 = adjust_color_to_hsl_string(TERTIARY, 0.0, 0.0, -20.0)
            .unwrap_or_else(|| "hsl(80, 100%, 76.2745098039%)".to_string());
        let pie7 = adjust_color_to_hsl_string(PRIMARY, 60.0, 0.0, -20.0)
            .unwrap_or_else(|| "hsl(300, 100%, 76.2745098039%)".to_string());
        let pie8 = adjust_color_to_hsl_string(PRIMARY, -60.0, 0.0, -40.0)
            .unwrap_or_else(|| "hsl(180, 100%, 56.2745098039%)".to_string());
        let pie9 = adjust_color_to_hsl_string(PRIMARY, 120.0, 0.0, -40.0)
            .unwrap_or_else(|| "hsl(0, 100%, 56.2745098039%)".to_string());
        let pie10 = adjust_color_to_hsl_string(PRIMARY, 60.0, 0.0, -40.0)
            .unwrap_or_else(|| "hsl(300, 100%, 56.2745098039%)".to_string());
        let pie11 = adjust_color_to_hsl_string(PRIMARY, -90.0, 0.0, -40.0)
            .unwrap_or_else(|| "hsl(150, 100%, 56.2745098039%)".to_string());
        let pie12 = adjust_color_to_hsl_string(PRIMARY, 120.0, 0.0, -30.0)
            .unwrap_or_else(|| "hsl(0, 100%, 66.2745098039%)".to_string());

        vec![
            PRIMARY.to_string(),
            SECONDARY.to_string(),
            pie3,
            pie4,
            pie5,
            pie6,
            pie7,
            pie8,
            pie9,
            pie10,
            pie11,
            pie12,
        ]
    }

    fn from_config(effective_config: &serde_json::Value) -> Self {
        let mut palette = Self::default_palette();
        for (idx, color) in palette.iter_mut().enumerate() {
            let key = format!("pie{}", idx + 1);
            if let Some(value) = config_string(effective_config, &["themeVariables", &key]) {
                *color = value;
            }
        }
        Self {
            palette,
            mapping: std::collections::HashMap::new(),
            next: 0,
        }
    }

    fn color_for(&mut self, label: &str) -> String {
        if let Some(idx) = self.mapping.get(label).copied() {
            return self.palette[idx % self.palette.len()].clone();
        }
        let idx = self.next;
        self.next += 1;
        self.mapping.insert(label.to_string(), idx);
        self.palette[idx % self.palette.len()].clone()
    }
}

fn polar_xy(radius: f64, angle: f64) -> (f64, f64) {
    // Mermaid pie charts use a "12 o'clock is zero" convention with y increasing downwards.
    let x = radius * angle.sin();
    let y = -radius * angle.cos();
    (x, y)
}

fn fmt_number(v: f64) -> String {
    if !v.is_finite() {
        return "0".to_string();
    }
    if v.abs() < 0.0005 {
        return "0".to_string();
    }
    let mut r = (v * 1000.0).round() / 1000.0;
    if r.abs() < 0.0005 {
        r = 0.0;
    }
    let mut s = format!("{r:.3}");
    if s.contains('.') {
        while s.ends_with('0') {
            s.pop();
        }
        if s.ends_with('.') {
            s.pop();
        }
    }
    if s == "-0" { "0".to_string() } else { s }
}

pub(crate) fn pie_text_position(effective_config: &serde_json::Value) -> f64 {
    config_f64(effective_config, &["pie", "textPosition"]).unwrap_or(0.75)
}

pub(crate) fn pie_donut_hole(effective_config: &serde_json::Value) -> f64 {
    let donut_hole = config_f64(effective_config, &["pie", "donutHole"]).unwrap_or(0.0);
    if donut_hole > 0.0 && donut_hole <= 0.9 {
        donut_hole
    } else {
        0.0
    }
}

pub(crate) fn pie_legend_position(effective_config: &serde_json::Value) -> PieLegendPosition {
    match value_at(effective_config, &["pie", "legendPosition"]).and_then(|v| v.as_str()) {
        Some("top") => PieLegendPosition::Top,
        Some("bottom") => PieLegendPosition::Bottom,
        Some("left") => PieLegendPosition::Left,
        Some("center") => PieLegendPosition::Center,
        _ => PieLegendPosition::Right,
    }
}

pub fn layout_pie_diagram(
    semantic: &serde_json::Value,
    effective_config: &serde_json::Value,
    measurer: &dyn TextMeasurer,
) -> Result<PieDiagramLayout> {
    let model: PieDiagramRenderModel = crate::json::from_value_ref(semantic)?;
    layout_pie_diagram_typed(&model, effective_config, measurer)
}

pub fn layout_pie_diagram_typed(
    model: &PieDiagramRenderModel,
    effective_config: &serde_json::Value,
    measurer: &dyn TextMeasurer,
) -> Result<PieDiagramLayout> {
    let _ = (
        model.title.as_deref(),
        model.acc_title.as_deref(),
        model.acc_descr.as_deref(),
    );

    // Mermaid@11.15 `packages/mermaid/src/diagrams/pie/pieRenderer.ts` constants.
    let margin: f64 = 40.0;
    let legend_rect_size = PIE_LEGEND_RECT_SIZE_PX;
    let legend_spacing = PIE_LEGEND_SPACING_PX;

    let center: f64 = 225.0;
    let radius: f64 = 185.0;
    let outer_radius = radius + 1.0;
    let label_radius = radius.max(0.0) * pie_text_position(effective_config);
    let legend_step_y: f64 = legend_rect_size + legend_spacing;
    let legend_position = pie_legend_position(effective_config);
    let total_legend_height = (model.sections.len() as f64) * legend_step_y;
    let centered_legend_start_y = -(legend_step_y * (model.sections.len().max(1) as f64)) / 2.0;
    let legend_start_y = match legend_position {
        PieLegendPosition::Top => -radius,
        PieLegendPosition::Bottom => radius + legend_step_y,
        _ => centered_legend_start_y,
    };

    let total: f64 = model
        .sections
        .iter()
        .filter(|s| s.value.is_finite() && s.value >= 0.0)
        .map(|s| s.value)
        .sum();

    let mut color_scale = ColorScale::from_config(effective_config);
    for sec in &model.sections {
        let _ = color_scale.color_for(&sec.label);
    }

    let mut slices: Vec<PieSliceLayout> = Vec::new();
    if total.is_finite() && total > 0.0 {
        // Mermaid@11.15 `packages/mermaid/src/diagrams/pie/pieRenderer.ts`:
        //
        // - filter out values < 1% (based on the original total)
        // - preserve input order before D3 pie() computes angles (`sort(null)`)
        // - angles are normalized over the filtered set (so drawn slices fill the whole circle)
        // - percentage labels are still computed using the original total
        let pie_sections: Vec<&PieRenderSection> = model
            .sections
            .iter()
            .filter(|s| s.value.is_finite() && s.value > 0.0)
            .filter(|s| (s.value / total) * 100.0 >= 1.0)
            .collect();

        let pie_total: f64 = pie_sections.iter().map(|s| s.value).sum();
        if !pie_sections.is_empty() && pie_total.is_finite() && pie_total > 0.0 {
            if pie_sections.len() == 1 {
                let s = pie_sections[0];
                let fill = color_scale.color_for(&s.label);
                let (tx, ty) = polar_xy(label_radius, std::f64::consts::PI);
                let percent = ((100.0 * (s.value / total)).max(0.0)).round() as i64;
                slices.push(PieSliceLayout {
                    label: s.label.clone(),
                    value: s.value,
                    start_angle: 0.0,
                    end_angle: std::f64::consts::TAU,
                    is_full_circle: true,
                    percent,
                    text_x: tx,
                    text_y: ty,
                    fill,
                });
            } else {
                let mut start = 0.0;
                for s in pie_sections {
                    let frac = (s.value / pie_total).max(0.0);
                    let delta = (frac * std::f64::consts::TAU).max(0.0);
                    let end = start + delta;
                    let mid = (start + end) / 2.0;
                    let (tx, ty) = polar_xy(label_radius, mid);
                    let fill = color_scale.color_for(&s.label);
                    let percent = ((100.0 * (s.value / total)).max(0.0)).round() as i64;
                    if percent != 0 {
                        slices.push(PieSliceLayout {
                            label: s.label.clone(),
                            value: s.value,
                            start_angle: start,
                            end_angle: end,
                            is_full_circle: false,
                            percent,
                            text_x: tx,
                            text_y: ty,
                            fill,
                        });
                    }
                    start = end;
                }
            }
        }
    }

    // Lock the color scale domain based on the drawn slices first, then compute legend colors in
    // the original section order (this matches Mermaid's zero-slice behavior).
    let mut legend_items: Vec<PieLegendItemLayout> = Vec::new();
    for (i, sec) in model.sections.iter().enumerate() {
        let y = legend_start_y + (i as f64) * legend_step_y;
        let fill = color_scale.color_for(&sec.label);
        legend_items.push(PieLegendItemLayout {
            label: sec.label.clone(),
            value: sec.value,
            fill,
            y,
        });
    }

    let legend_style = TextStyle {
        font_family: None,
        font_size: 17.0,
        font_weight: None,
    };
    let title_style = TextStyle {
        font_family: None,
        font_size: 25.0,
        font_weight: None,
    };
    let mut max_legend_width: f64 = 0.0;
    for sec in &model.sections {
        let label = if model.show_data {
            format!("{} [{}]", sec.label, fmt_number(sec.value))
        } else {
            sec.label.clone()
        };
        let trimmed = label.trim_end();
        // Mermaid 11.15 measures pie legend text via a single SVG `<text>` node's
        // `getBoundingClientRect().width`. In headless mode we cannot reproduce that browser value
        // exactly, but the single-run/simple-text SVG width path is closer than the generic
        // multi-run bbox approximation used by Mermaid's wrapped `<tspan>` labels.
        let w = if trimmed.is_empty() {
            0.0
        } else {
            crate::text::round_to_1_64_px(
                measurer.measure_svg_simple_text_bbox_width_px(trimmed, &legend_style),
            )
        };
        max_legend_width = max_legend_width.max(w);
    }

    let title_width = model
        .title
        .as_deref()
        .map(str::trim_end)
        .filter(|title| !title.is_empty())
        .map(|title| {
            crate::text::round_to_1_64_px(
                measurer.measure_svg_simple_text_bbox_width_px(title, &title_style),
            )
        })
        .unwrap_or(0.0);

    let base_w: f64 = center * 2.0;
    let legend_extra_width = legend_rect_size + legend_spacing + max_legend_width;
    let centered_legend_x = -max_legend_width / 2.0 - (legend_rect_size + legend_spacing);

    let chart_and_legend_width = match legend_position {
        PieLegendPosition::Top => (base_w + margin).max(1.0),
        PieLegendPosition::Bottom => (base_w + margin).max(1.0),
        PieLegendPosition::Left => (base_w + margin + legend_extra_width).max(1.0),
        PieLegendPosition::Center => (base_w + margin).max(1.0),
        PieLegendPosition::Right => {
            if model.sections.is_empty() {
                f64::NEG_INFINITY
            } else {
                (base_w + margin + legend_extra_width).max(1.0)
            }
        }
    };

    let height = match legend_position {
        PieLegendPosition::Top | PieLegendPosition::Bottom => {
            (base_w + total_legend_height).max(1.0)
        }
        PieLegendPosition::Left | PieLegendPosition::Center | PieLegendPosition::Right => {
            base_w.max(1.0)
        }
    };

    let legend_x = match legend_position {
        PieLegendPosition::Top | PieLegendPosition::Bottom | PieLegendPosition::Center => {
            centered_legend_x
        }
        PieLegendPosition::Left => -radius - (legend_rect_size + legend_spacing),
        PieLegendPosition::Right => 12.0 * legend_rect_size,
    };

    let title_left = base_w / 2.0 - title_width / 2.0;
    let title_right = base_w / 2.0 + title_width / 2.0;
    let min_x = title_left.min(0.0);
    let max_x = chart_and_legend_width.max(title_right);

    Ok(PieDiagramLayout {
        bounds: Some(Bounds {
            min_x,
            min_y: 0.0,
            max_x,
            max_y: height,
        }),
        center_x: center,
        center_y: center,
        radius,
        outer_radius,
        legend_x,
        legend_start_y,
        legend_step_y,
        slices,
        legend_items,
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn pie_legend_geometry_constants_match_mermaid() {
        assert_eq!(super::PIE_LEGEND_RECT_SIZE_PX, 18.0);
        assert_eq!(super::PIE_LEGEND_SPACING_PX, 4.0);
    }
}
