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
    #[allow(dead_code)]
    pub width: f32,
    /// Logical height in pixels.
    #[allow(dead_code)]
    pub height: f32,
    /// Baseline Y position from the top of the image, in pixels.
    #[allow(dead_code)]
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
        Err(e) => {
            log::error!("Failed to load math fonts: {}", e);
            svg.push_str("</svg>");
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

                if let Some(path_data) = resolve_glyph_path(font_id, ch, &fonts, &mut font_refs) {
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
                let color_str = color_to_svg(color);

                if *is_fill {
                    // KaTeX delimiters (e.g. matrix brackets, braces, parentheses) and stretchy
                    // symbols are assembled from multiple overlapping or adjacent path components.
                    // If combined into a single <path> with evenodd fill rule, overlapping segments
                    // (such as the vertical extension bar overlapping bracket caps) cancel out to
                    // an even winding count (hole), making delimiter stems disappear.
                    // If combined with nonzero winding, opposite-direction segments can also cancel out.
                    // Emitting each subpath (delimited by MoveTo) as a separate <path> ensures every
                    // component is filled solidly without mutual cancellation.
                    let mut start = 0;
                    for i in 1..commands.len() {
                        if matches!(commands[i], PathCommand::MoveTo { .. }) {
                            let d = path_commands_to_svg_d(&commands[start..i], em, px, py);
                            if !d.is_empty() {
                                svg.push_str(&format!(
                                    "<path d=\"{}\" fill=\"{}\" />",
                                    d, color_str
                                ));
                            }
                            start = i;
                        }
                    }
                    if start < commands.len() {
                        let d = path_commands_to_svg_d(&commands[start..], em, px, py);
                        if !d.is_empty() {
                            svg.push_str(&format!("<path d=\"{}\" fill=\"{}\" />", d, color_str));
                        }
                    }
                } else {
                    let d = path_commands_to_svg_d(commands, em, px, py);
                    if !d.is_empty() {
                        svg.push_str(&format!(
                            "<path d=\"{}\" stroke=\"{}\" stroke-width=\"{:.2}\" fill=\"none\" />",
                            d, color_str, 1.5
                        ));
                    }
                }
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

    let outlines =
        ratex_font_loader::outline_cache::get_or_compute_outline(font_id, font_ref, glyph_id)?;

    let units_per_em = font_ref.units_per_em().unwrap_or(1000.0);
    let scale = 1000.0 / units_per_em;

    let mut d = String::with_capacity(256);
    let mut last_point: Option<ab_glyph::Point> = None;
    for curve in outlines.iter() {
        use ab_glyph::OutlineCurve;
        let p0 = match curve {
            OutlineCurve::Line(p0, _)
            | OutlineCurve::Quad(p0, _, _)
            | OutlineCurve::Cubic(p0, _, _, _) => *p0,
        };
        let p_end = match curve {
            OutlineCurve::Line(_, p1) => *p1,
            OutlineCurve::Quad(_, _, p2) => *p2,
            OutlineCurve::Cubic(_, _, _, p3) => *p3,
        };

        let needs_move = match last_point {
            None => true,
            Some(last) => (last.x - p0.x).abs() > 0.001 || (last.y - p0.y).abs() > 0.001,
        };

        if needs_move {
            if last_point.is_some() {
                d.push_str(" Z ");
            }
            d.push_str(&format!("M{:.2} {:.2}", p0.x * scale, -p0.y * scale));
        }

        match curve {
            OutlineCurve::Line(_, p1) => {
                d.push_str(&format!(" L{:.2} {:.2}", p1.x * scale, -p1.y * scale));
            }
            OutlineCurve::Quad(_, p1, p2) => {
                d.push_str(&format!(
                    " Q{:.2} {:.2} {:.2} {:.2}",
                    p1.x * scale,
                    -p1.y * scale,
                    p2.x * scale,
                    -p2.y * scale
                ));
            }
            OutlineCurve::Cubic(_, p1, p2, p3) => {
                d.push_str(&format!(
                    " C{:.2} {:.2} {:.2} {:.2} {:.2} {:.2}",
                    p1.x * scale,
                    -p1.y * scale,
                    p2.x * scale,
                    -p2.y * scale,
                    p3.x * scale,
                    -p3.y * scale
                ));
            }
        }
        last_point = Some(p_end);
    }
    if last_point.is_some() {
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

#[cfg(test)]
mod tests {
    use super::*;
    use ratex_types::display_item::{DisplayItem, DisplayList};

    #[test]
    fn test_display_list_to_svg_rect() {
        let items = vec![DisplayItem::Rect {
            x: 0.0,
            y: 0.0,
            width: 10.0,
            height: 10.0,
            color: Color::new(0.0, 0.0, 0.0, 1.0),
        }];
        let list = DisplayList {
            items,
            width: 10.0,
            height: 10.0,
            depth: 0.0,
        };
        let output = display_list_to_svg(&list, 12.0);
        assert!(output.svg_bytes.len() > 50, "SVG should be non-trivial");
        let svg_str = String::from_utf8_lossy(&output.svg_bytes);
        assert!(svg_str.starts_with("<svg"));
        assert!(svg_str.ends_with("</svg>"));
        assert!(svg_str.contains("<rect"));
    }

    #[test]
    fn test_display_list_to_svg_empty() {
        let list = DisplayList {
            items: vec![],
            width: 0.0,
            height: 0.0,
            depth: 0.0,
        };
        let output = display_list_to_svg(&list, 12.0);
        assert!(output.svg_bytes.len() > 0);
    }

    #[test]
    fn test_display_list_to_svg_glyphs() {
        let nodes = ratex_parser::parse("x^2 + y = 0").unwrap();
        let layout = ratex_layout::layout(&nodes, &Default::default());
        let list = ratex_layout::to_display_list(&layout);
        let output = display_list_to_svg(&list, 16.0);
        let svg_str = String::from_utf8_lossy(&output.svg_bytes);
        assert!(svg_str.starts_with("<svg"), "must start with svg tag");
        assert!(svg_str.ends_with("</svg>"), "must end with svg closing tag");
        assert!(svg_str.contains("<path"), "must contain glyph paths");
    }

    #[test]
    fn test_matrix_delimiters_render_without_fill_rule_cancellation() {
        // bmatrix uses tall bracket delimiters constructed with multiple subpath segments
        let latex = r"\begin{bmatrix} f_x & s & c_x \\ 0 & f_y & c_y \\ 0 & 0 & 1 \end{bmatrix}";
        let nodes = ratex_parser::parse(latex).unwrap();
        let layout = ratex_layout::layout(&nodes, &Default::default());
        let list = ratex_layout::to_display_list(&layout);
        let output = display_list_to_svg(&list, 16.0);
        let svg_str = String::from_utf8_lossy(&output.svg_bytes);
        assert!(svg_str.starts_with("<svg"));
        assert!(svg_str.ends_with("</svg>"));
        // Must NOT use evenodd which causes overlapping bracket stem segments to cancel out and disappear
        assert!(
            !svg_str.contains("evenodd"),
            "delimiters must not use evenodd fill-rule"
        );
        // Each delimiter segment must be emitted as a distinct filled path
        assert!(svg_str.matches("<path d=").count() >= 4);

        // Also test pmatrix and Bmatrix
        for matrix_latex in [
            r"\begin{pmatrix} 1 & 2 \\ 3 & 4 \end{pmatrix}",
            r"\begin{Bmatrix} a & b \\ c & d \end{Bmatrix}",
        ] {
            let nodes = ratex_parser::parse(matrix_latex).unwrap();
            let layout = ratex_layout::layout(&nodes, &Default::default());
            let list = ratex_layout::to_display_list(&layout);
            let output = display_list_to_svg(&list, 16.0);
            let svg_str = String::from_utf8_lossy(&output.svg_bytes);
            assert!(svg_str.starts_with("<svg"));
            assert!(!svg_str.contains("evenodd"));
        }
    }
}

fn color_to_svg(color: &Color) -> String {
    let r = (color.r.clamp(0.0, 1.0) * 255.0).round() as u8;
    let g = (color.g.clamp(0.0, 1.0) * 255.0).round() as u8;
    let b = (color.b.clamp(0.0, 1.0) * 255.0).round() as u8;
    let a = color.a.clamp(0.0, 1.0);
    if (a - 1.0).abs() < 0.01 {
        format!("#{:02x}{:02x}{:02x}", r, g, b)
    } else {
        format!("rgba({},{},{},{:.2})", r, g, b, a)
    }
}
