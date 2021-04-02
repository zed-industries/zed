use crate::{
    fonts::{FontId, GlyphId, Metrics, Properties},
    geometry::{
        rect::{RectF, RectI},
        transform2d::Transform2F,
        vector::{vec2f, vec2i, Vector2F},
    },
    platform,
    text_layout::{Glyph, Line, Run},
};
use cocoa::appkit::{CGFloat, CGPoint};
use core_foundation::{
    attributed_string::CFMutableAttributedString,
    base::{CFRange, TCFType},
    number::CFNumber,
    string::CFString,
};
use core_graphics::{
    base::CGGlyph, color_space::CGColorSpace, context::CGContext, geometry::CGAffineTransform,
};
use core_text::{line::CTLine, string_attributes::kCTFontAttributeName};
use font_kit::{canvas::RasterizationOptions, hinting::HintingOptions, source::SystemSource};
use parking_lot::RwLock;
use std::{char, convert::TryFrom};

#[allow(non_upper_case_globals)]
const kCGImageAlphaOnly: u32 = 7;

pub struct FontSystem(RwLock<FontSystemState>);

struct FontSystemState {
    source: SystemSource,
    fonts: Vec<font_kit::font::Font>,
}

impl FontSystem {
    pub fn new() -> Self {
        Self(RwLock::new(FontSystemState {
            source: SystemSource::new(),
            fonts: Vec::new(),
        }))
    }
}

impl platform::FontSystem for FontSystem {
    fn load_family(&self, name: &str) -> anyhow::Result<Vec<FontId>> {
        self.0.write().load_family(name)
    }

    fn select_font(&self, font_ids: &[FontId], properties: &Properties) -> anyhow::Result<FontId> {
        self.0.read().select_font(font_ids, properties)
    }

    fn font_metrics(&self, font_id: FontId) -> Metrics {
        self.0.read().font_metrics(font_id)
    }

    fn typographic_bounds(&self, font_id: FontId, glyph_id: GlyphId) -> anyhow::Result<RectF> {
        self.0.read().typographic_bounds(font_id, glyph_id)
    }

    fn glyph_for_char(&self, font_id: FontId, ch: char) -> Option<GlyphId> {
        self.0.read().glyph_for_char(font_id, ch)
    }

    fn rasterize_glyph(
        &self,
        font_id: FontId,
        font_size: f32,
        glyph_id: GlyphId,
        subpixel_shift: Vector2F,
        scale_factor: f32,
    ) -> Option<(RectI, Vec<u8>)> {
        self.0
            .read()
            .rasterize_glyph(font_id, font_size, glyph_id, subpixel_shift, scale_factor)
    }

    fn layout_str(
        &self,
        text: &str,
        font_size: f32,
        runs: &[(std::ops::Range<usize>, FontId)],
    ) -> Line {
        self.0.read().layout_str(text, font_size, runs)
    }
}

impl FontSystemState {
    fn load_family(&mut self, name: &str) -> anyhow::Result<Vec<FontId>> {
        let mut font_ids = Vec::new();
        for font in self.source.select_family_by_name(name)?.fonts() {
            let font = font.load()?;
            eprintln!("load font {:?}", font);
            font_ids.push(FontId(self.fonts.len()));
            self.fonts.push(font);
        }
        eprintln!("font ids: {:?}", font_ids);
        Ok(font_ids)
    }

    fn select_font(&self, font_ids: &[FontId], properties: &Properties) -> anyhow::Result<FontId> {
        let candidates = font_ids
            .iter()
            .map(|font_id| self.fonts[font_id.0].properties())
            .collect::<Vec<_>>();
        let idx = font_kit::matching::find_best_match(&candidates, properties)?;
        Ok(font_ids[idx])
    }

    fn font_metrics(&self, font_id: FontId) -> Metrics {
        self.fonts[font_id.0].metrics()
    }

    fn typographic_bounds(&self, font_id: FontId, glyph_id: GlyphId) -> anyhow::Result<RectF> {
        Ok(self.fonts[font_id.0].typographic_bounds(glyph_id)?)
    }

    fn glyph_for_char(&self, font_id: FontId, ch: char) -> Option<GlyphId> {
        self.fonts[font_id.0].glyph_for_char(ch)
    }

    fn rasterize_glyph(
        &self,
        font_id: FontId,
        font_size: f32,
        glyph_id: GlyphId,
        subpixel_shift: Vector2F,
        scale_factor: f32,
    ) -> Option<(RectI, Vec<u8>)> {
        let font = &self.fonts[font_id.0];
        let scale = Transform2F::from_scale(scale_factor);
        let bounds = font
            .raster_bounds(
                glyph_id,
                font_size,
                scale,
                HintingOptions::None,
                RasterizationOptions::GrayscaleAa,
            )
            .ok()?;

        if bounds.width() == 0 || bounds.height() == 0 {
            None
        } else {
            // Make room for subpixel variants.
            let bounds = RectI::new(bounds.origin(), bounds.size() + vec2i(1, 1));
            let mut pixels = vec![0; bounds.width() as usize * bounds.height() as usize];
            let ctx = CGContext::create_bitmap_context(
                Some(pixels.as_mut_ptr() as *mut _),
                bounds.width() as usize,
                bounds.height() as usize,
                8,
                bounds.width() as usize,
                &CGColorSpace::create_device_gray(),
                kCGImageAlphaOnly,
            );

            // Move the origin to bottom left and account for scaling, this
            // makes drawing text consistent with the font-kit's raster_bounds.
            ctx.translate(0.0, bounds.height() as CGFloat);
            let transform = scale.translate(-bounds.origin().to_f32());
            ctx.set_text_matrix(&CGAffineTransform {
                a: transform.matrix.m11() as CGFloat,
                b: -transform.matrix.m21() as CGFloat,
                c: -transform.matrix.m12() as CGFloat,
                d: transform.matrix.m22() as CGFloat,
                tx: transform.vector.x() as CGFloat,
                ty: -transform.vector.y() as CGFloat,
            });

            ctx.set_font(&font.native_font().copy_to_CGFont());
            ctx.set_font_size(font_size as CGFloat);
            ctx.show_glyphs_at_positions(
                &[glyph_id as CGGlyph],
                &[CGPoint::new(
                    (subpixel_shift.x() / scale_factor) as CGFloat,
                    (subpixel_shift.y() / scale_factor) as CGFloat,
                )],
            );

            Some((bounds, pixels))
        }
    }

    fn layout_str(
        &self,
        text: &str,
        font_size: f32,
        runs: &[(std::ops::Range<usize>, FontId)],
    ) -> Line {
        let font_id_attr_name = CFString::from_static_string("zed_font_id");

        let mut string = CFMutableAttributedString::new();
        string.replace_str(&CFString::new(text), CFRange::init(0, 0));

        let mut utf16_lens = text.chars().map(|c| c.len_utf16());
        let mut prev_char_ix = 0;
        let mut prev_utf16_ix = 0;

        for (range, font_id) in runs {
            let utf16_start = prev_utf16_ix
                + utf16_lens
                    .by_ref()
                    .take(range.start - prev_char_ix)
                    .sum::<usize>();
            let utf16_end = utf16_start
                + utf16_lens
                    .by_ref()
                    .take(range.end - range.start)
                    .sum::<usize>();
            prev_char_ix = range.end;
            prev_utf16_ix = utf16_end;

            let cf_range = CFRange::init(utf16_start as isize, (utf16_end - utf16_start) as isize);
            let font = &self.fonts[font_id.0];
            unsafe {
                string.set_attribute(
                    cf_range,
                    kCTFontAttributeName,
                    &font.native_font().clone_with_font_size(font_size as f64),
                );
                string.set_attribute(
                    cf_range,
                    font_id_attr_name.as_concrete_TypeRef(),
                    &CFNumber::from(font_id.0 as i64),
                );
            }
        }

        let line = CTLine::new_with_attributed_string(string.as_concrete_TypeRef());

        let width = line.get_typographic_bounds().width as f32;

        let mut utf16_chars = text.encode_utf16();
        let mut char_ix = 0;
        let mut prev_utf16_ix = 0;

        let mut runs = Vec::new();
        for run in line.glyph_runs().into_iter() {
            let font_id = FontId(
                run.attributes()
                    .unwrap()
                    .get(&font_id_attr_name)
                    .downcast::<CFNumber>()
                    .unwrap()
                    .to_i64()
                    .unwrap() as usize,
            );

            let mut glyphs = Vec::new();
            for ((glyph_id, position), utf16_ix) in run
                .glyphs()
                .iter()
                .zip(run.positions().iter())
                .zip(run.string_indices().iter())
            {
                let utf16_ix = usize::try_from(*utf16_ix).unwrap();
                char_ix +=
                    char::decode_utf16(utf16_chars.by_ref().take(utf16_ix - prev_utf16_ix)).count();
                prev_utf16_ix = utf16_ix;

                glyphs.push(Glyph {
                    id: *glyph_id as GlyphId,
                    position: vec2f(position.x as f32, position.y as f32),
                    index: char_ix,
                });
            }

            runs.push(Run { font_id, glyphs })
        }

        Line {
            width,
            runs,
            font_size,
            len: char_ix + 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use font_kit::properties::{Style, Weight};
    use platform::FontSystem as _;

    #[test]
    fn test_layout_str() -> anyhow::Result<()> {
        let fonts = FontSystem::new();
        let menlo = fonts.load_family("Menlo")?;
        let menlo_regular = fonts.select_font(&menlo, &Properties::new())?;
        let menlo_italic = fonts.select_font(&menlo, &Properties::new().style(Style::Italic))?;
        let menlo_bold = fonts.select_font(&menlo, &Properties::new().weight(Weight::BOLD))?;

        let line = fonts.layout_str(
            "hello world",
            16.0,
            &[
                (0..2, menlo_bold),
                (2..6, menlo_italic),
                (6..11, menlo_regular),
            ],
        );
        assert_eq!(line.runs.len(), 3);
        assert_eq!(line.runs[0].font_id, menlo_bold);
        assert_eq!(line.runs[0].glyphs.len(), 2);
        assert_eq!(line.runs[1].font_id, menlo_italic);
        assert_eq!(line.runs[1].glyphs.len(), 4);
        assert_eq!(line.runs[2].font_id, menlo_regular);
        assert_eq!(line.runs[2].glyphs.len(), 5);
        Ok(())
    }

    #[test]
    fn test_char_indices() -> anyhow::Result<()> {
        let fonts = FontSystem::new();
        let zapfino = fonts.load_family("Zapfino")?;
        let zapfino_regular = fonts.select_font(&zapfino, &Properties::new())?;
        let menlo = fonts.load_family("Menlo")?;
        let menlo_regular = fonts.select_font(&menlo, &Properties::new())?;

        let text = "This is, m𐍈re 𐍈r less, Zapfino!𐍈";
        let line = fonts.layout_str(
            text,
            16.0,
            &[
                (0..9, zapfino_regular),
                (9..22, menlo_regular),
                (22..text.encode_utf16().count(), zapfino_regular),
            ],
        );
        assert_eq!(
            line.runs
                .iter()
                .flat_map(|r| r.glyphs.iter())
                .map(|g| g.index)
                .collect::<Vec<_>>(),
            vec![
                0, 2, 4, 5, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 30, 31
            ]
        );
        Ok(())
    }

    // #[test]
    // fn test_rasterize_glyph() {
    //     use std::{fs::File, io::BufWriter, path::Path};

    //     let fonts = FontSystem::new();
    //     let font_ids = fonts.load_family("Fira Code").unwrap();
    //     let font_id = fonts.select_font(&font_ids, &Default::default()).unwrap();
    //     let glyph_id = fonts.glyph_for_char(font_id, 'G').unwrap();

    //     const VARIANTS: usize = 1;
    //     for i in 0..VARIANTS {
    //         let variant = i as f32 / VARIANTS as f32;
    //         let (bounds, bytes) = fonts
    //             .rasterize_glyph(font_id, 16.0, glyph_id, vec2f(variant, variant), 2.)
    //             .unwrap();

    //         let name = format!("/Users/as-cii/Desktop/twog-{}.png", i);
    //         let path = Path::new(&name);
    //         let file = File::create(path).unwrap();
    //         let ref mut w = BufWriter::new(file);

    //         let mut encoder = png::Encoder::new(w, bounds.width() as u32, bounds.height() as u32);
    //         encoder.set_color(png::ColorType::Grayscale);
    //         encoder.set_depth(png::BitDepth::Eight);
    //         let mut writer = encoder.write_header().unwrap();
    //         writer.write_image_data(&bytes).unwrap();
    //     }
    // }
}
