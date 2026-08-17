//! Helpers for rendering buffers and editors

#[cfg(not(feature = "std"))]
use core_maths::CoreFloat;

use crate::{Color, DecorationSpan, LayoutRun, PhysicalGlyph, UnderlineStyle};
#[cfg(feature = "swash")]
use crate::{FontSystem, SwashCache};

/// Custom renderer for buffers and editors
pub trait Renderer {
    /// Render a rectangle at x, y with size w, h and the provided [`Color`].
    fn rectangle(&mut self, x: i32, y: i32, w: u32, h: u32, color: Color);

    /// Render a [`PhysicalGlyph`] with the provided [`Color`].
    /// For performance, consider using [`SwashCache`].
    fn glyph(&mut self, physical_glyph: PhysicalGlyph, color: Color);
}

/// Draw text decoration lines (underline, strikethrough, overline) for a layout run.
pub fn render_decoration<R: Renderer>(renderer: &mut R, run: &LayoutRun, default_color: Color) {
    for span in run.decorations {
        draw_decoration_span(renderer, run, span, default_color);
    }
}

fn draw_decoration_span<R: Renderer>(
    renderer: &mut R,
    run: &LayoutRun,
    span: &DecorationSpan,
    default_color: Color,
) {
    let glyphs = &run.glyphs[span.glyph_range.clone()];
    if glyphs.is_empty() {
        return;
    }

    let deco = &span.data;
    let td = &deco.text_decoration;
    let font_size = span.font_size;

    // Compute x extent as min/max over all glyphs, not first/last,
    // because RTL paragraphs store glyphs in right-to-left order.
    let mut x_min = f32::INFINITY;
    let mut x_max = f32::NEG_INFINITY;
    for g in glyphs {
        x_min = x_min.min(g.x);
        x_max = x_max.max(g.x + g.w);
    }
    let width = x_max - x_min;
    if width <= 0.0 {
        return;
    }
    let w = width as u32;
    if w == 0 {
        return;
    }
    let x_start = x_min;

    // Underline
    match td.underline {
        UnderlineStyle::None => {}
        UnderlineStyle::Single => {
            let color = td
                .underline_color_opt
                .or(span.color_opt)
                .unwrap_or(default_color);
            let thickness = (deco.underline_metrics.thickness * font_size)
                .max(1.0)
                .ceil();
            let y = run.line_y - deco.underline_metrics.offset * font_size;
            renderer.rectangle(x_start as i32, y as i32, w, thickness as u32, color);
        }
        UnderlineStyle::Double => {
            let color = td
                .underline_color_opt
                .or(span.color_opt)
                .unwrap_or(default_color);
            let thickness = (deco.underline_metrics.thickness * font_size)
                .max(1.0)
                .ceil();
            let gap = thickness;
            let y = run.line_y - deco.underline_metrics.offset * font_size;
            renderer.rectangle(x_start as i32, y as i32, w, thickness as u32, color);
            renderer.rectangle(
                x_start as i32,
                (y + thickness + gap) as i32,
                w,
                thickness as u32,
                color,
            );
        }
    }

    // Strikethrough
    if td.strikethrough {
        let color = td
            .strikethrough_color_opt
            .or(span.color_opt)
            .unwrap_or(default_color);
        let thickness = (deco.strikethrough_metrics.thickness * font_size)
            .max(1.0)
            .ceil();
        let y = run.line_y - deco.strikethrough_metrics.offset * font_size;
        renderer.rectangle(x_start as i32, y as i32, w, thickness as u32, color);
    }

    // Overline
    if td.overline {
        let color = td
            .overline_color_opt
            .or(span.color_opt)
            .unwrap_or(default_color);
        // Reuse underline thickness for overline
        let thickness = (deco.underline_metrics.thickness * font_size)
            .max(1.0)
            .ceil();
        // clamped so it doesn't go above the line top.
        let y = (run.line_y - deco.ascent * font_size).max(run.line_top);
        renderer.rectangle(x_start as i32, y as i32, w, thickness as u32, color);
    }
}

/// Helper to migrate from old renderer
//TODO: remove in future version
#[cfg(feature = "swash")]
#[derive(Debug)]
pub struct LegacyRenderer<'a, F: FnMut(i32, i32, u32, u32, Color)> {
    pub font_system: &'a mut FontSystem,
    pub cache: &'a mut SwashCache,
    pub callback: F,
}

#[cfg(feature = "swash")]
impl<'a, F: FnMut(i32, i32, u32, u32, Color)> Renderer for LegacyRenderer<'a, F> {
    fn rectangle(&mut self, x: i32, y: i32, w: u32, h: u32, color: Color) {
        (self.callback)(x, y, w, h, color);
    }

    fn glyph(&mut self, physical_glyph: PhysicalGlyph, color: Color) {
        self.cache.with_pixels(
            self.font_system,
            physical_glyph.cache_key,
            color,
            |x, y, pixel_color| {
                (self.callback)(
                    physical_glyph.x + x,
                    physical_glyph.y + y,
                    1,
                    1,
                    pixel_color,
                );
            },
        );
    }
}
