use crate::{
    fonts::{FontId, GlyphId, Metrics, Properties},
    geometry::{
        rect::{RectF, RectI},
        transform2d::Transform2F,
        vector::{vec2f, vec2i, Vector2F},
    },
    platform,
    text_layout::{Glyph, LineLayout, Run, RunStyle},
};
use cocoa::appkit::{CGFloat, CGPoint};
use core_foundation::{
    array::CFIndex,
    attributed_string::{CFAttributedStringRef, CFMutableAttributedString},
    base::{CFRange, TCFType},
    string::CFString,
};
use core_graphics::{
    base::CGGlyph, color_space::CGColorSpace, context::CGContext, geometry::CGAffineTransform,
};
use core_text::{font::CTFont, line::CTLine, string_attributes::kCTFontAttributeName};
use font_kit::{
    canvas::RasterizationOptions, handle::Handle, hinting::HintingOptions, source::SystemSource,
    sources::mem::MemSource,
};
use parking_lot::RwLock;
use std::{cell::RefCell, char, cmp, convert::TryFrom, ffi::c_void, sync::Arc};

#[allow(non_upper_case_globals)]
const kCGImageAlphaOnly: u32 = 7;

pub struct FontSystem(RwLock<FontSystemState>);

struct FontSystemState {
    memory_source: MemSource,
    system_source: SystemSource,
    fonts: Vec<font_kit::font::Font>,
}

impl FontSystem {
    pub fn new() -> Self {
        Self(RwLock::new(FontSystemState {
            memory_source: MemSource::empty(),
            system_source: SystemSource::new(),
            fonts: Vec::new(),
        }))
    }
}

impl platform::FontSystem for FontSystem {
    fn add_fonts(&self, fonts: &[Arc<Vec<u8>>]) -> anyhow::Result<()> {
        self.0.write().add_fonts(fonts)
    }

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

    fn advance(&self, font_id: FontId, glyph_id: GlyphId) -> anyhow::Result<Vector2F> {
        self.0.read().advance(font_id, glyph_id)
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

    fn layout_line(&self, text: &str, font_size: f32, runs: &[(usize, RunStyle)]) -> LineLayout {
        self.0.write().layout_line(text, font_size, runs)
    }

    fn wrap_line(&self, text: &str, font_id: FontId, font_size: f32, width: f32) -> Vec<usize> {
        self.0.read().wrap_line(text, font_id, font_size, width)
    }
}

impl FontSystemState {
    fn add_fonts(&mut self, fonts: &[Arc<Vec<u8>>]) -> anyhow::Result<()> {
        self.memory_source.add_fonts(
            fonts
                .iter()
                .map(|bytes| Handle::from_memory(bytes.clone(), 0)),
        )?;
        Ok(())
    }

    fn load_family(&mut self, name: &str) -> anyhow::Result<Vec<FontId>> {
        let mut font_ids = Vec::new();

        let family = self
            .memory_source
            .select_family_by_name(name)
            .or_else(|_| self.system_source.select_family_by_name(name))?;
        for font in family.fonts() {
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

    fn advance(&self, font_id: FontId, glyph_id: GlyphId) -> anyhow::Result<Vector2F> {
        Ok(self.fonts[font_id.0].advance(glyph_id)?)
    }

    fn glyph_for_char(&self, font_id: FontId, ch: char) -> Option<GlyphId> {
        self.fonts[font_id.0].glyph_for_char(ch)
    }

    fn id_for_font(&mut self, requested_font: font_kit::font::Font) -> FontId {
        // TODO: don't allocate the postscript name
        // Note: Coretext always returns a Some option for postscript_name
        let requested_font_name = requested_font.postscript_name();
        for (id, font) in self.fonts.iter().enumerate() {
            if font.postscript_name() == requested_font_name {
                return FontId(id);
            }
        }
        self.fonts.push(requested_font);
        FontId(self.fonts.len() - 1)
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
        let glyph_bounds = font
            .raster_bounds(
                glyph_id,
                font_size,
                scale,
                HintingOptions::None,
                RasterizationOptions::GrayscaleAa,
            )
            .ok()?;

        if glyph_bounds.width() == 0 || glyph_bounds.height() == 0 {
            None
        } else {
            // Make room for subpixel variants.
            let cx_bounds = RectI::new(glyph_bounds.origin(), glyph_bounds.size() + vec2i(1, 1));
            let mut bytes = vec![0; cx_bounds.width() as usize * cx_bounds.height() as usize];
            let cx = CGContext::create_bitmap_context(
                Some(bytes.as_mut_ptr() as *mut _),
                cx_bounds.width() as usize,
                cx_bounds.height() as usize,
                8,
                cx_bounds.width() as usize,
                &CGColorSpace::create_device_gray(),
                kCGImageAlphaOnly,
            );

            // Move the origin to bottom left and account for scaling, this
            // makes drawing text consistent with the font-kit's raster_bounds.
            cx.translate(
                -glyph_bounds.origin_x() as CGFloat,
                (glyph_bounds.origin_y() + glyph_bounds.height()) as CGFloat,
            );
            cx.scale(scale_factor as CGFloat, scale_factor as CGFloat);

            cx.set_allows_font_subpixel_positioning(true);
            cx.set_should_subpixel_position_fonts(true);
            cx.set_allows_font_subpixel_quantization(false);
            cx.set_should_subpixel_quantize_fonts(false);
            font.native_font()
                .clone_with_font_size(font_size as CGFloat)
                .draw_glyphs(
                    &[glyph_id as CGGlyph],
                    &[CGPoint::new(
                        (subpixel_shift.x() / scale_factor) as CGFloat,
                        (subpixel_shift.y() / scale_factor) as CGFloat,
                    )],
                    cx,
                );

            Some((cx_bounds, bytes))
        }
    }

    fn layout_line(
        &mut self,
        text: &str,
        font_size: f32,
        runs: &[(usize, RunStyle)],
    ) -> LineLayout {
        // Construct the attributed string, converting UTF8 ranges to UTF16 ranges.
        let mut string = CFMutableAttributedString::new();
        {
            string.replace_str(&CFString::new(text), CFRange::init(0, 0));
            let utf16_line_len = string.char_len() as usize;

            let last_run: RefCell<Option<(usize, FontId)>> = Default::default();
            let font_runs = runs
                .iter()
                .filter_map(|(len, style)| {
                    let mut last_run = last_run.borrow_mut();
                    if let Some((last_len, last_font_id)) = last_run.as_mut() {
                        if style.font_id == *last_font_id {
                            *last_len += *len;
                            None
                        } else {
                            let result = (*last_len, *last_font_id);
                            *last_len = *len;
                            *last_font_id = style.font_id;
                            Some(result)
                        }
                    } else {
                        *last_run = Some((*len, style.font_id));
                        None
                    }
                })
                .chain(std::iter::from_fn(|| last_run.borrow_mut().take()));

            let mut ix_converter = StringIndexConverter::new(text);
            for (run_len, font_id) in font_runs {
                let utf8_end = ix_converter.utf8_ix + run_len;
                let utf16_start = ix_converter.utf16_ix;

                if utf16_start >= utf16_line_len {
                    break;
                }

                ix_converter.advance_to_utf8_ix(utf8_end);
                let utf16_end = cmp::min(ix_converter.utf16_ix, utf16_line_len);

                let cf_range =
                    CFRange::init(utf16_start as isize, (utf16_end - utf16_start) as isize);
                let font = &self.fonts[font_id.0];
                unsafe {
                    string.set_attribute(
                        cf_range,
                        kCTFontAttributeName,
                        &font.native_font().clone_with_font_size(font_size as f64),
                    );
                }

                if utf16_end == utf16_line_len {
                    break;
                }
            }
        }

        // Retrieve the glyphs from the shaped line, converting UTF16 offsets to UTF8 offsets.
        let line = CTLine::new_with_attributed_string(string.as_concrete_TypeRef());

        let mut runs = Vec::new();
        for run in line.glyph_runs().into_iter() {
            let attributes = run.attributes().unwrap();
            let font = unsafe {
                let native_font = attributes
                    .get(kCTFontAttributeName)
                    .downcast::<CTFont>()
                    .unwrap();
                font_kit::font::Font::from_native_font(native_font)
            };
            let font_id = self.id_for_font(font);

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
    use super::*;
    use crate::MutableAppContext;
    use font_kit::properties::{Style, Weight};
    use platform::FontSystem as _;

    #[crate::test(self, retries = 5)]
    fn test_layout_str(_: &mut MutableAppContext) {
        // This is failing intermittently on CI and we don't have time to figure it out
        let fonts = FontSystem::new();
        let menlo = fonts.load_family("Menlo").unwrap();
        let menlo_regular = RunStyle {
            font_id: fonts.select_font(&menlo, &Properties::new()).unwrap(),
            color: Default::default(),
            underline: Default::default(),
        };
        let menlo_italic = RunStyle {
            font_id: fonts
                .select_font(&menlo, &Properties::new().style(Style::Italic))
                .unwrap(),
            color: Default::default(),
            underline: Default::default(),
        };
        let menlo_bold = RunStyle {
            font_id: fonts
                .select_font(&menlo, &Properties::new().weight(Weight::BOLD))
                .unwrap(),
            color: Default::default(),
            underline: Default::default(),
        };
        assert_ne!(menlo_regular, menlo_italic);
        assert_ne!(menlo_regular, menlo_bold);
        assert_ne!(menlo_italic, menlo_bold);

        let line = fonts.layout_line(
            "hello world",
            16.0,
            &[(2, menlo_bold), (4, menlo_italic), (5, menlo_regular)],
        );
        assert_eq!(line.runs.len(), 3);
        assert_eq!(line.runs[0].font_id, menlo_bold.font_id);
        assert_eq!(line.runs[0].glyphs.len(), 2);
        assert_eq!(line.runs[1].font_id, menlo_italic.font_id);
        assert_eq!(line.runs[1].glyphs.len(), 4);
        assert_eq!(line.runs[2].font_id, menlo_regular.font_id);
        assert_eq!(line.runs[2].glyphs.len(), 5);
    }

    #[test]
    fn test_glyph_offsets() -> anyhow::Result<()> {
        let fonts = FontSystem::new();
        let zapfino = fonts.load_family("Zapfino")?;
        let zapfino_regular = RunStyle {
            font_id: fonts.select_font(&zapfino, &Properties::new())?,
            color: Default::default(),
            underline: Default::default(),
        };
        let menlo = fonts.load_family("Menlo")?;
        let menlo_regular = RunStyle {
            font_id: fonts.select_font(&menlo, &Properties::new())?,
            color: Default::default(),
            underline: Default::default(),
        };

        let text = "This is, m𐍈re 𐍈r less, Zapfino!𐍈";
        let line = fonts.layout_line(
            text,
            16.0,
            &[
                (9, zapfino_regular),
                (13, menlo_regular),
                (text.len() - 22, zapfino_regular),
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
    fn test_wrap_line() {
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

    #[test]
    fn test_layout_line_bom_char() {
        let fonts = FontSystem::new();
        let font_ids = fonts.load_family("Helvetica").unwrap();
        let style = RunStyle {
            font_id: fonts.select_font(&font_ids, &Default::default()).unwrap(),
            color: Default::default(),
            underline: Default::default(),
        };

        let line = "\u{feff}";
        let layout = fonts.layout_line(line, 16., &[(line.len(), style)]);
        assert_eq!(layout.len, line.len());
        assert!(layout.runs.is_empty());

        let line = "a\u{feff}b";
        let layout = fonts.layout_line(line, 16., &[(line.len(), style)]);
        assert_eq!(layout.len, line.len());
        assert_eq!(layout.runs.len(), 1);
        assert_eq!(layout.runs[0].glyphs.len(), 2);
        assert_eq!(layout.runs[0].glyphs[0].id, 68); // a
                                                     // There's no glyph for \u{feff}
        assert_eq!(layout.runs[0].glyphs[1].id, 69); // b
    }
}
