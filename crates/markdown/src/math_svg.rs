use ratex_font::FontId;
use ratex_font_loader::FontSet;
use ratex_types::color::Color;
use ratex_types::display_item::{DisplayItem, DisplayList};
use ratex_types::path_command::PathCommand;
use std::collections::HashMap;

/// Metadata produced alongside the SVG for correct inline layout.
pub struct MathSvgOutput {
    pub svg_bytes: Vec<u8>,
    /// Logical width in pixels.
    pub width: f32,
    /// Logical height in pixels.
    pub height: f32,
    /// Baseline Y position from the top of the image, in pixels.
    pub baseline_y: f32,
}

const PAD: f32 = 2.0;

/// Convert a ratex DisplayList into SVG markup bytes.
///
/// This produces a resolution-independent SVG that GPUI's SvgRenderer
/// can rasterize at any scale factor for crisp rendering.
pub fn display_list_to_svg(display_list: &DisplayList, font_size: f32) -> MathSvgOutput {
    let em = font_size;

    let total_h = display_list.height + display_list.depth;
    let svg_w = (display_list.width as f32 * em + 2.0 * PAD).ceil();
    let svg_h = (total_h as f32 * em + 2.0 * PAD).ceil();
    let baseline_y = display_list.height as f32 * em + PAD;

    let mut svg = String::with_capacity(4096);
    svg.push_str(&format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{:.1}\" height=\"{:.1}\" viewBox=\"{:.1} {:.1} {:.1} {:.1}\">",
        svg_w, svg_h, 0.0, 0.0, svg_w, svg_h
    ));

    let fonts = match ratex_font_loader::load_fonts_for_items("", &display_list.items) {
        Ok(f) => f,
        Err(_) => {
            return MathSvgOutput {
                svg_bytes: svg.into_bytes(),
                width: svg_w,
                height: svg_h,
                baseline_y,
            };
        }
    };
    let mut font_refs: HashMap<FontId, ab_glyph::FontRef<'_>> = HashMap::new();
    let mut glyph_cache: HashMap<(FontId, u32), String> = HashMap::new();

    for item in &display_list.items {
        match item {
            DisplayItem::GlyphPath {
                x,
                y,
                scale,
                font,
                char_code,
                color,
            } => {
                let glyph_em = em * *scale as f32;
                let px = *x as f32 * em + PAD;
                let py = *y as f32 * em + PAD;

                let font_id = FontId::parse(font).unwrap_or(FontId::MainRegular);
                let cache_key = (font_id, *char_code);
                if let Some(path_data) = glyph_cache.get(&cache_key) {
                    let fill = color_to_svg(color);
                    svg.push_str(&format!(
                        "<path d=\"{}\" fill=\"{}\" transform=\"translate({:.2},{:.2}) scale({:.4})\" />",
                        path_data, fill, px, py, glyph_em / 1000.0
                    ));
                    continue;
                }

                let ch = ratex_font::katex_ttf_glyph_char(font_id, *char_code);

                if let Some(path_data) =
                    resolve_glyph_path(font_id, ch, &fonts, &mut font_refs)
                {
                    let fill = color_to_svg(color);
                    svg.push_str(&format!(
                        "<path d=\"{}\" fill=\"{}\" transform=\"translate({:.2},{:.2}) scale({:.4})\" />",
                        path_data, fill, px, py, glyph_em / 1000.0
                    ));
                    glyph_cache.insert(cache_key, path_data);
                }
            }
            DisplayItem::Line {
                x,
                y,
                width,
                thickness,
                color,
                dashed,
            } => {
                let px = *x as f32 * em + PAD;
                let py = *y as f32 * em + PAD;
                let w = *width as f32 * em;
                let t = (*thickness as f32 * em).max(1.0);
                let fill = color_to_svg(color);

                if *dashed {
                    svg.push_str(&format!(
                        "<line x1=\"{:.2}\" y1=\"{:.2}\" x2=\"{:.2}\" y2=\"{:.2}\" stroke=\"{}\" stroke-width=\"{:.2}\" stroke-dasharray=\"{:.2}\" />",
                        px, py, px + w, py, fill, t, t * 4.0
                    ));
                } else {
                    svg.push_str(&format!(
                        "<rect x=\"{:.2}\" y=\"{:.2}\" width=\"{:.2}\" height=\"{:.2}\" fill=\"{}\" />",
                        px, py - t / 2.0, w, t, fill
                    ));
                }
            }
            DisplayItem::Rect {
                x,
                y,
                width,
                height,
                color,
            } => {
                let px = *x as f32 * em + PAD;
                let py = *y as f32 * em + PAD;
                let w = (*width as f32 * em).max(2.0);
                let h = (*height as f32 * em).max(2.0);
                let fill = color_to_svg(color);
                svg.push_str(&format!(
                    "<rect x=\"{:.2}\" y=\"{:.2}\" width=\"{:.2}\" height=\"{:.2}\" fill=\"{}\" />",
                    px, py, w, h, fill
                ));
            }
            DisplayItem::Path {
                x,
                y,
                commands,
                fill: is_fill,
                color,
            } => {
                let px = *x as f32 * em + PAD;
                let py = *y as f32 * em + PAD;
                let d = path_commands_to_svg_d(commands, em, px, py);
                let fill_attr = color_to_svg(color);
                let fill_rule = if *is_fill { "evenodd" } else { "nonzero" };
                let stroke_attr = if *is_fill {
                    String::new()
                } else {
                    format!(" stroke=\"{}\" stroke-width=\"{:.2}\" fill=\"none\"", fill_attr, 1.5)
                };
                let fill_attr_final = if *is_fill {
                    format!(" fill=\"{}\"", fill_attr)
                } else {
                    String::new()
                };
                svg.push_str(&format!(
                    "<path d=\"{}\"{} fill-rule=\"{}\"{} />",
                    d, fill_attr_final, fill_rule, stroke_attr
                ));
            }
        }
    }

    svg.push_str("</svg>");
    MathSvgOutput {
        svg_bytes: svg.into_bytes(),
        width: svg_w,
        height: svg_h,
        baseline_y,
    }
}

fn resolve_glyph_path<'a>(
    font_id: FontId,
    ch: char,
    fonts: &'a FontSet,
    font_refs: &mut HashMap<FontId, ab_glyph::FontRef<'a>>,
) -> Option<String> {
    use ab_glyph::Font;

    let font_data = fonts.get(&font_id)?;
    if !font_refs.contains_key(&font_id) {
        if let Ok(font_ref) = ab_glyph::FontRef::try_from_slice(font_data) {
            font_refs.insert(font_id, font_ref);
        }
    }
    let font_ref = font_refs.get(&font_id)?;

    let glyph_id = font_ref.glyph_id(ch);
    if glyph_id.0 == 0 {
        return None;
    }

    let outlines = ratex_font_loader::outline_cache::get_or_compute_outline(
        font_id, font_ref, glyph_id,
    )?;

    let units_per_em = font_ref.units_per_em().unwrap_or(1000.0);
    let scale = 1000.0 / units_per_em;

    let mut d = String::with_capacity(256);
    for curve in outlines.iter() {
        use ab_glyph::OutlineCurve;
        match curve {
            OutlineCurve::Line(p0, p1) => {
                if d.is_empty() {
                    d.push_str(&format!("M{} {}", p0.x * scale, -p0.y * scale));
                }
                d.push_str(&format!(" L{} {}", p1.x * scale, -p1.y * scale));
            }
            OutlineCurve::Quad(p0, p1, p2) => {
                if d.is_empty() {
                    d.push_str(&format!("M{} {}", p0.x * scale, -p0.y * scale));
                }
                d.push_str(&format!(
                    " Q{} {} {} {}",
                    p1.x * scale, -p1.y * scale, p2.x * scale, -p2.y * scale
                ));
            }
            OutlineCurve::Cubic(p0, p1, p2, p3) => {
                if d.is_empty() {
                    d.push_str(&format!("M{} {}", p0.x * scale, -p0.y * scale));
                }
                d.push_str(&format!(
                    " C{} {} {} {} {} {}",
                    p1.x * scale, -p1.y * scale,
                    p2.x * scale, -p2.y * scale,
                    p3.x * scale, -p3.y * scale
                ));
            }
        }
    }
    if !d.is_empty() {
        d.push('Z');
    }

    Some(d)
}

fn path_commands_to_svg_d(commands: &[PathCommand], em: f32, x: f32, y: f32) -> String {
    let mut d = String::with_capacity(256);
    for cmd in commands {
        match cmd {
            PathCommand::MoveTo { x: cx, y: cy } => {
                d.push_str(&format!(
                    "M{:.2} {:.2}",
                    x + *cx as f32 * em,
                    y + *cy as f32 * em
                ));
            }
            PathCommand::LineTo { x: cx, y: cy } => {
                d.push_str(&format!(
                    " L{:.2} {:.2}",
                    x + *cx as f32 * em,
                    y + *cy as f32 * em
                ));
            }
            PathCommand::CubicTo {
                x1,
                y1,
                x2,
                y2,
                x: cx,
                y: cy,
            } => {
                d.push_str(&format!(
                    " C{:.2} {:.2} {:.2} {:.2} {:.2} {:.2}",
                    x + *x1 as f32 * em,
                    y + *y1 as f32 * em,
                    x + *x2 as f32 * em,
                    y + *y2 as f32 * em,
                    x + *cx as f32 * em,
                    y + *cy as f32 * em
                ));
            }
            PathCommand::QuadTo {
                x1,
                y1,
                x: cx,
                y: cy,
            } => {
                d.push_str(&format!(
                    " Q{:.2} {:.2} {:.2} {:.2}",
                    x + *x1 as f32 * em,
                    y + *y1 as f32 * em,
                    x + *cx as f32 * em,
                    y + *cy as f32 * em
                ));
            }
            PathCommand::Close => {
                d.push('Z');
            }
        }
    }
    d
}

fn color_to_svg(color: &Color) -> String {
    let r = (color.r.clamp(0.0, 1.0) * 255.0).round() as u8;
    let g = (color.g.clamp(0.0, 1.0) * 255.0).round() as u8;
    let b = (color.b.clamp(0.0, 1.0) * 255.0).round() as u8;
    let a = color.a.clamp(0.0, 1.0);
    if (a - 1.0).abs() < 0.01 {
        format!("#{:02x}{:02x}{:02x}", r, g, b)
    } else {
        format!(
            "rgba({},{},{},{:.2})",
            r,
            g,
            b,
            a
        )
    }
}
