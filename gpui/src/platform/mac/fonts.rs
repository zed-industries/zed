use crate::{
    color::ColorU,
    fonts::{FontId, GlyphId, Metrics, Properties},
    geometry::{
        rect::{RectF, RectI},
        transform2d::Transform2F,
        vector::{vec2f, vec2i, Vector2F},
    },
    platform,
    text_layout::{Glyph, LineLayout, Run},
};
use cocoa::appkit::{CGFloat, CGPoint};
use core_foundation::{
    array::CFIndex,
    attributed_string::{CFAttributedStringRef, CFMutableAttributedString},
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
use std::{cell::RefCell, char, convert::TryFrom, ffi::c_void};

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

    fn layout_line(
        &self,
        text: &str,
        font_size: f32,
        runs: &[(usize, FontId, ColorU)],
    ) -> LineLayout {
        self.0.read().layout_line(text, font_size, runs)
    }

    fn wrap_line(&self, text: &str, font_id: FontId, font_size: f32, width: f32) -> Vec<usize> {
        self.0.read().wrap_line(text, font_id, font_size, width)
    }
}

impl FontSystemState {
    fn load_family(&mut self, name: &str) -> anyhow::Result<Vec<FontId>> {
        let mut font_ids = Vec::new();
        for font in self.source.select_family_by_name(name)?.fonts() {
            let font = font.load()?;
            font_ids.push(FontId(self.fonts.len()));
            self.fonts.push(font);
        }
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
            let cx = CGContext::create_bitmap_context(
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
            cx.translate(0.0, bounds.height() as CGFloat);
            let transform = scale.translate(-bounds.origin().to_f32());
            cx.set_text_matrix(&CGAffineTransform {
                a: transform.matrix.m11() as CGFloat,
                b: -transform.matrix.m21() as CGFloat,
                c: -transform.matrix.m12() as CGFloat,
                d: transform.matrix.m22() as CGFloat,
                tx: transform.vector.x() as CGFloat,
                ty: -transform.vector.y() as CGFloat,
            });

            cx.set_font(&font.native_font().copy_to_CGFont());
            cx.set_font_size(font_size as CGFloat);
            cx.show_glyphs_at_positions(
                &[glyph_id as CGGlyph],
                &[CGPoint::new(
                    (subpixel_shift.x() / scale_factor) as CGFloat,
                    (subpixel_shift.y() / scale_factor) as CGFloat,
                )],
            );

            Some((bounds, pixels))
        }
    }

    fn layout_line(
        &self,
        text: &str,
        font_size: f32,
        runs: &[(usize, FontId, ColorU)],
    ) -> LineLayout {
        let font_id_attr_name = CFString::from_static_string("zed_font_id");

        // Construct the attributed string, converting UTF8 ranges to UTF16 ranges.
        let mut string = CFMutableAttributedString::new();
        {
            string.replace_str(&CFString::new(text), CFRange::init(0, 0));

            let last_run: RefCell<Option<(usize, FontId)>> = Default::default();
            let font_runs = runs
                .iter()
                .filter_map(|(len, font_id, _)| {
                    let mut last_run = last_run.borrow_mut();
                    if let Some((last_len, last_font_id)) = last_run.as_mut() {
                        if font_id == last_font_id {
                            *last_len += *len;
                            None
                        } else {
                            let result = (*last_len, *last_font_id);
                            *last_len = *len;
                            *last_font_id = *font_id;
                            Some(result)
                        }
                    } else {
                        *last_run = Some((*len, *font_id));
                        None
                    }
                })
                .chain(std::iter::from_fn(|| last_run.borrow_mut().take()));

            let mut ix_converter = StringIndexConverter::new(text);
            for (run_len, font_id) in font_runs {
                let utf8_end = ix_converter.utf8_ix + run_len;
                let utf16_start = ix_converter.utf16_ix;
                ix_converter.advance_to_utf8_ix(utf8_end);

                let cf_range = CFRange::init(
                    utf16_start as isize,
                    (ix_converter.utf16_ix - utf16_start) as isize,
                );
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
        }

        // Retrieve the glyphs from the shaped line, converting UTF16 offsets to UTF8 offsets.
        let line = CTLine::new_with_attributed_string(string.as_concrete_TypeRef());

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

            let mut ix_converter = StringIndexConverter::new(text);
            let mut glyphs = Vec::new();
            for ((glyph_id, position), glyph_utf16_ix) in run
                .glyphs()
                .iter()
                .zip(run.positions().iter())
                .zip(run.string_indices().iter())
            {
                let glyph_utf16_ix = usize::try_from(*glyph_utf16_ix).unwrap();
                ix_converter.advance_to_utf16_ix(glyph_utf16_ix);
                glyphs.push(Glyph {
                    id: *glyph_id as GlyphId,
                    position: vec2f(position.x as f32, position.y as f32),
                    index: ix_converter.utf8_ix,
                });
            }

            runs.push(Run { font_id, glyphs })
        }

        let typographic_bounds = line.get_typographic_bounds();
        LineLayout {
            width: typographic_bounds.width as f32,
            ascent: typographic_bounds.ascent as f32,
            descent: typographic_bounds.descent as f32,
            runs,
            font_size,
            len: text.len(),
        }
    }

    fn wrap_line(&self, text: &str, font_id: FontId, font_size: f32, width: f32) -> Vec<usize> {
        let mut string = CFMutableAttributedString::new();
        string.replace_str(&CFString::new(text), CFRange::init(0, 0));
        let cf_range = CFRange::init(0 as isize, text.encode_utf16().count() as isize);
        let font = &self.fonts[font_id.0];
        unsafe {
            string.set_attribute(
                cf_range,
                kCTFontAttributeName,
                &font.native_font().clone_with_font_size(font_size as f64),
            );

            let typesetter = CTTypesetterCreateWithAttributedString(string.as_concrete_TypeRef());
            let mut ix_converter = StringIndexConverter::new(text);
            let mut break_indices = Vec::new();
            while ix_converter.utf8_ix < text.len() {
                let utf16_len = CTTypesetterSuggestLineBreak(
                    typesetter,
                    ix_converter.utf16_ix as isize,
                    width as f64,
                ) as usize;
                ix_converter.advance_to_utf16_ix(ix_converter.utf16_ix + utf16_len);
                if ix_converter.utf8_ix >= text.len() {
                    break;
                }
                break_indices.push(ix_converter.utf8_ix as usize);
            }
            break_indices
        }
    }
}

#[derive(Clone)]
struct StringIndexConverter<'a> {
    text: &'a str,
    utf8_ix: usize,
    utf16_ix: usize,
}

impl<'a> StringIndexConverter<'a> {
    fn new(text: &'a str) -> Self {
        Self {
            text,
            utf8_ix: 0,
            utf16_ix: 0,
        }
    }

    fn advance_to_utf8_ix(&mut self, utf8_target: usize) {
        for (ix, c) in self.text[self.utf8_ix..].char_indices() {
            if self.utf8_ix + ix >= utf8_target {
                self.utf8_ix += ix;
                return;
            }
            self.utf16_ix += c.len_utf16();
        }
        self.utf8_ix = self.text.len();
    }

    fn advance_to_utf16_ix(&mut self, utf16_target: usize) {
        for (ix, c) in self.text[self.utf8_ix..].char_indices() {
            if self.utf16_ix >= utf16_target {
                self.utf8_ix += ix;
                return;
            }
            self.utf16_ix += c.len_utf16();
        }
        self.utf8_ix = self.text.len();
    }
}

#[repr(C)]
pub struct __CFTypesetter(c_void);

pub type CTTypesetterRef = *const __CFTypesetter;

#[link(name = "CoreText", kind = "framework")]
extern "C" {
    fn CTTypesetterCreateWithAttributedString(string: CFAttributedStringRef) -> CTTypesetterRef;

    fn CTTypesetterSuggestLineBreak(
        typesetter: CTTypesetterRef,
        start_index: CFIndex,
        width: f64,
    ) -> CFIndex;
}

#[cfg(test)]
mod tests {
    use crate::MutableAppContext;

    use super::*;
    use font_kit::properties::{Style, Weight};
    use platform::FontSystem as _;

    #[crate::test(self, retries = 5)]
    fn test_layout_str(_: &mut MutableAppContext) {
        // This is failing intermittently on CI and we don't have time to figure it out
        let fonts = FontSystem::new();
        let menlo = fonts.load_family("Menlo").unwrap();
        let menlo_regular = fonts.select_font(&menlo, &Properties::new()).unwrap();
        let menlo_italic = fonts
            .select_font(&menlo, &Properties::new().style(Style::Italic))
            .unwrap();
        let menlo_bold = fonts
            .select_font(&menlo, &Properties::new().weight(Weight::BOLD))
            .unwrap();
        assert_ne!(menlo_regular, menlo_italic);
        assert_ne!(menlo_regular, menlo_bold);
        assert_ne!(menlo_italic, menlo_bold);

        let line = fonts.layout_line(
            "hello world",
            16.0,
            &[
                (2, menlo_bold, Default::default()),
                (4, menlo_italic, Default::default()),
                (5, menlo_regular, Default::default()),
            ],
        );
        assert_eq!(line.runs.len(), 3);
        assert_eq!(line.runs[0].font_id, menlo_bold);
        assert_eq!(line.runs[0].glyphs.len(), 2);
        assert_eq!(line.runs[1].font_id, menlo_italic);
        assert_eq!(line.runs[1].glyphs.len(), 4);
        assert_eq!(line.runs[2].font_id, menlo_regular);
        assert_eq!(line.runs[2].glyphs.len(), 5);
    }

    #[test]
    fn test_glyph_offsets() -> anyhow::Result<()> {
        let fonts = FontSystem::new();
        let zapfino = fonts.load_family("Zapfino")?;
        let zapfino_regular = fonts.select_font(&zapfino, &Properties::new())?;
        let menlo = fonts.load_family("Menlo")?;
        let menlo_regular = fonts.select_font(&menlo, &Properties::new())?;

        let text = "This is, m𐍈re 𐍈r less, Zapfino!𐍈";
        let line = fonts.layout_line(
            text,
            16.0,
            &[
                (9, zapfino_regular, ColorU::default()),
                (13, menlo_regular, ColorU::default()),
                (text.len() - 22, zapfino_regular, ColorU::default()),
            ],
        );
        assert_eq!(
            line.runs
                .iter()
                .flat_map(|r| r.glyphs.iter())
                .map(|g| g.index)
                .collect::<Vec<_>>(),
            vec![0, 2, 4, 5, 7, 8, 9, 10, 14, 15, 16, 17, 21, 22, 23, 24, 26, 27, 28, 29, 36, 37],
        );
        Ok(())
    }

    #[test]
    #[ignore]
    fn test_rasterize_glyph() {
        use std::{fs::File, io::BufWriter, path::Path};

        let fonts = FontSystem::new();
        let font_ids = fonts.load_family("Fira Code").unwrap();
        let font_id = fonts.select_font(&font_ids, &Default::default()).unwrap();
        let glyph_id = fonts.glyph_for_char(font_id, 'G').unwrap();

        const VARIANTS: usize = 1;
        for i in 0..VARIANTS {
            let variant = i as f32 / VARIANTS as f32;
            let (bounds, bytes) = fonts
                .rasterize_glyph(font_id, 16.0, glyph_id, vec2f(variant, variant), 2.)
                .unwrap();

            let name = format!("/Users/as-cii/Desktop/twog-{}.png", i);
            let path = Path::new(&name);
            let file = File::create(path).unwrap();
            let ref mut w = BufWriter::new(file);

            let mut encoder = png::Encoder::new(w, bounds.width() as u32, bounds.height() as u32);
            encoder.set_color(png::ColorType::Grayscale);
            encoder.set_depth(png::BitDepth::Eight);
            let mut writer = encoder.write_header().unwrap();
            writer.write_image_data(&bytes).unwrap();
        }
    }

    #[test]
    fn test_layout_line() {
        let fonts = FontSystem::new();
        let font_ids = fonts.load_family("Helvetica").unwrap();
        let font_id = fonts.select_font(&font_ids, &Default::default()).unwrap();

        let line = "one two three four five\n";
        let wrap_boundaries = fonts.wrap_line(line, font_id, 16., 64.0);
        assert_eq!(wrap_boundaries, &["one two ".len(), "one two three ".len()]);

        let line = "aaa ααα ✋✋✋ 🎉🎉🎉\n";
        let wrap_boundaries = fonts.wrap_line(line, font_id, 16., 64.0);
        assert_eq!(
            wrap_boundaries,
            &["aaa ααα ".len(), "aaa ααα ✋✋✋ ".len(),]
        );
    }
}
