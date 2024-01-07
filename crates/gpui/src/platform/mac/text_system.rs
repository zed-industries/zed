use crate::{
    point, px, size, Bounds, DevicePixels, Font, FontFeatures, FontId, FontMetrics, FontRun,
    FontStyle, FontWeight, GlyphId, LineLayout, Pixels, PlatformTextSystem, Point,
    RenderGlyphParams, Result, ShapedGlyph, ShapedRun, SharedString, Size, SUBPIXEL_VARIANTS,
};
use anyhow::anyhow;
use cocoa::appkit::{CGFloat, CGPoint};
use collections::HashMap;
use core_foundation::{
    array::CFIndex,
    attributed_string::{CFAttributedStringRef, CFMutableAttributedString},
    base::{CFRange, TCFType},
    string::CFString,
};
use core_graphics::{
    base::{kCGImageAlphaPremultipliedLast, CGGlyph},
    color_space::CGColorSpace,
    context::CGContext,
};
use core_text::{font::CTFont, line::CTLine, string_attributes::kCTFontAttributeName};
use font_kit::{
    font::Font as FontKitFont,
    handle::Handle,
    hinting::HintingOptions,
    metrics::Metrics,
    properties::{Style as FontkitStyle, Weight as FontkitWeight},
    source::SystemSource,
    sources::mem::MemSource,
};
use parking_lot::{RwLock, RwLockUpgradableReadGuard};
use pathfinder_geometry::{
    rect::{RectF, RectI},
    transform2d::Transform2F,
    vector::{Vector2F, Vector2I},
};
use smallvec::SmallVec;
use std::{char, cmp, convert::TryFrom, ffi::c_void, sync::Arc};

use super::open_type;

#[allow(non_upper_case_globals)]
const kCGImageAlphaOnly: u32 = 7;

pub struct MacTextSystem(RwLock<MacTextSystemState>);

struct MacTextSystemState {
    memory_source: MemSource,
    system_source: SystemSource,
    fonts: Vec<FontKitFont>,
    font_selections: HashMap<Font, FontId>,
    font_ids_by_postscript_name: HashMap<String, FontId>,
    font_ids_by_family_name: HashMap<SharedString, SmallVec<[FontId; 4]>>,
    postscript_names_by_font_id: HashMap<FontId, String>,
}

impl MacTextSystem {
    pub fn new() -> Self {
        Self(RwLock::new(MacTextSystemState {
            memory_source: MemSource::empty(),
            system_source: SystemSource::new(),
            fonts: Vec::new(),
            font_selections: HashMap::default(),
            font_ids_by_postscript_name: HashMap::default(),
            font_ids_by_family_name: HashMap::default(),
            postscript_names_by_font_id: HashMap::default(),
        }))
    }
}

impl Default for MacTextSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl PlatformTextSystem for MacTextSystem {
    fn add_fonts(&self, fonts: &[Arc<Vec<u8>>]) -> Result<()> {
        self.0.write().add_fonts(fonts)
    }

    fn all_font_families(&self) -> Vec<String> {
        self.0
            .read()
            .system_source
            .all_families()
            .expect("core text should never return an error")
    }

    fn font_id(&self, font: &Font) -> Result<FontId> {
        let lock = self.0.upgradable_read();
        if let Some(font_id) = lock.font_selections.get(font) {
            Ok(*font_id)
        } else {
            let mut lock = RwLockUpgradableReadGuard::upgrade(lock);
            let candidates = if let Some(font_ids) = lock.font_ids_by_family_name.get(&font.family)
            {
                font_ids.as_slice()
            } else {
                let font_ids = lock.load_family(&font.family, font.features)?;
                lock.font_ids_by_family_name
                    .insert(font.family.clone(), font_ids);
                lock.font_ids_by_family_name[&font.family].as_ref()
            };

            let candidate_properties = candidates
                .iter()
                .map(|font_id| lock.fonts[font_id.0].properties())
                .collect::<SmallVec<[_; 4]>>();

            let ix = font_kit::matching::find_best_match(
                &candidate_properties,
                &font_kit::properties::Properties {
                    style: font.style.into(),
                    weight: font.weight.into(),
                    stretch: Default::default(),
                },
            )?;

            let font_id = candidates[ix];
            lock.font_selections.insert(font.clone(), font_id);
            Ok(font_id)
        }
    }

    fn font_metrics(&self, font_id: FontId) -> FontMetrics {
        self.0.read().fonts[font_id.0].metrics().into()
    }

    fn typographic_bounds(&self, font_id: FontId, glyph_id: GlyphId) -> Result<Bounds<f32>> {
        Ok(self.0.read().fonts[font_id.0]
            .typographic_bounds(glyph_id.into())?
            .into())
    }

    fn advance(&self, font_id: FontId, glyph_id: GlyphId) -> Result<Size<f32>> {
        self.0.read().advance(font_id, glyph_id)
    }

    fn glyph_for_char(&self, font_id: FontId, ch: char) -> Option<GlyphId> {
        self.0.read().glyph_for_char(font_id, ch)
    }

    fn glyph_raster_bounds(&self, params: &RenderGlyphParams) -> Result<Bounds<DevicePixels>> {
        self.0.read().raster_bounds(params)
    }

    fn rasterize_glyph(
        &self,
        glyph_id: &RenderGlyphParams,
        raster_bounds: Bounds<DevicePixels>,
    ) -> Result<(Size<DevicePixels>, Vec<u8>)> {
        self.0.read().rasterize_glyph(glyph_id, raster_bounds)
    }

    fn layout_line(&self, text: &str, font_size: Pixels, font_runs: &[FontRun]) -> LineLayout {
        self.0.write().layout_line(text, font_size, font_runs)
    }

    fn wrap_line(
        &self,
        text: &str,
        font_id: FontId,
        font_size: Pixels,
        width: Pixels,
    ) -> Vec<usize> {
        self.0.read().wrap_line(text, font_id, font_size, width)
    }
}

impl MacTextSystemState {
    fn add_fonts(&mut self, fonts: &[Arc<Vec<u8>>]) -> Result<()> {
        self.memory_source.add_fonts(
            fonts
                .iter()
                .map(|bytes| Handle::from_memory(bytes.clone(), 0)),
        )?;
        Ok(())
    }

    fn load_family(
        &mut self,
        name: &SharedString,
        features: FontFeatures,
    ) -> Result<SmallVec<[FontId; 4]>> {
        let mut font_ids = SmallVec::new();
        let family = self
            .memory_source
            .select_family_by_name(name.as_ref())
            .or_else(|_| self.system_source.select_family_by_name(name.as_ref()))?;
        for font in family.fonts() {
            let mut font = font.load()?;
            open_type::apply_features(&mut font, features);
            let font_id = FontId(self.fonts.len());
            font_ids.push(font_id);
            let postscript_name = font.postscript_name().unwrap();
            self.font_ids_by_postscript_name
                .insert(postscript_name.clone(), font_id);
            self.postscript_names_by_font_id
                .insert(font_id, postscript_name);
            self.fonts.push(font);
        }
        Ok(font_ids)
    }

    fn advance(&self, font_id: FontId, glyph_id: GlyphId) -> Result<Size<f32>> {
        Ok(self.fonts[font_id.0].advance(glyph_id.into())?.into())
    }

    fn glyph_for_char(&self, font_id: FontId, ch: char) -> Option<GlyphId> {
        self.fonts[font_id.0].glyph_for_char(ch).map(Into::into)
    }

    fn id_for_native_font(&mut self, requested_font: CTFont) -> FontId {
        let postscript_name = requested_font.postscript_name();
        if let Some(font_id) = self.font_ids_by_postscript_name.get(&postscript_name) {
            *font_id
        } else {
            let font_id = FontId(self.fonts.len());
            self.font_ids_by_postscript_name
                .insert(postscript_name.clone(), font_id);
            self.postscript_names_by_font_id
                .insert(font_id, postscript_name);
            self.fonts
                .push(font_kit::font::Font::from_core_graphics_font(
                    requested_font.copy_to_CGFont(),
                ));
            font_id
        }
    }

    fn is_emoji(&self, font_id: FontId) -> bool {
        self.postscript_names_by_font_id
            .get(&font_id)
            .map_or(false, |postscript_name| {
                postscript_name == "AppleColorEmoji"
            })
    }

    fn raster_bounds(&self, params: &RenderGlyphParams) -> Result<Bounds<DevicePixels>> {
        let font = &self.fonts[params.font_id.0];
        let scale = Transform2F::from_scale(params.scale_factor);
        Ok(font
            .raster_bounds(
                params.glyph_id.into(),
                params.font_size.into(),
                scale,
                HintingOptions::None,
                font_kit::canvas::RasterizationOptions::GrayscaleAa,
            )?
            .into())
    }

    fn rasterize_glyph(
        &self,
        params: &RenderGlyphParams,
        glyph_bounds: Bounds<DevicePixels>,
    ) -> Result<(Size<DevicePixels>, Vec<u8>)> {
        if glyph_bounds.size.width.0 == 0 || glyph_bounds.size.height.0 == 0 {
            Err(anyhow!("glyph bounds are empty"))
        } else {
            // Add an extra pixel when the subpixel variant isn't zero to make room for anti-aliasing.
            let mut bitmap_size = glyph_bounds.size;
            if params.subpixel_variant.x > 0 {
                bitmap_size.width += DevicePixels(1);
            }
            if params.subpixel_variant.y > 0 {
                bitmap_size.height += DevicePixels(1);
            }
            let bitmap_size = bitmap_size;

            let mut bytes;
            let cx;
            if params.is_emoji {
                bytes = vec![0; bitmap_size.width.0 as usize * 4 * bitmap_size.height.0 as usize];
                cx = CGContext::create_bitmap_context(
                    Some(bytes.as_mut_ptr() as *mut _),
                    bitmap_size.width.0 as usize,
                    bitmap_size.height.0 as usize,
                    8,
                    bitmap_size.width.0 as usize * 4,
                    &CGColorSpace::create_device_rgb(),
                    kCGImageAlphaPremultipliedLast,
                );
            } else {
                bytes = vec![0; bitmap_size.width.0 as usize * bitmap_size.height.0 as usize];
                cx = CGContext::create_bitmap_context(
                    Some(bytes.as_mut_ptr() as *mut _),
                    bitmap_size.width.0 as usize,
                    bitmap_size.height.0 as usize,
                    8,
                    bitmap_size.width.0 as usize,
                    &CGColorSpace::create_device_gray(),
                    kCGImageAlphaOnly,
                );
            }

            // Move the origin to bottom left and account for scaling, this
            // makes drawing text consistent with the font-kit's raster_bounds.
            cx.translate(
                -glyph_bounds.origin.x.0 as CGFloat,
                (glyph_bounds.origin.y.0 + glyph_bounds.size.height.0) as CGFloat,
            );
            cx.scale(
                params.scale_factor as CGFloat,
                params.scale_factor as CGFloat,
            );

            let subpixel_shift = params
                .subpixel_variant
                .map(|v| v as f32 / SUBPIXEL_VARIANTS as f32);
            cx.set_allows_font_subpixel_positioning(true);
            cx.set_should_subpixel_position_fonts(true);
            cx.set_allows_font_subpixel_quantization(false);
            cx.set_should_subpixel_quantize_fonts(false);
            self.fonts[params.font_id.0]
                .native_font()
                .clone_with_font_size(f32::from(params.font_size) as CGFloat)
                .draw_glyphs(
                    &[u32::from(params.glyph_id) as CGGlyph],
                    &[CGPoint::new(
                        (subpixel_shift.x / params.scale_factor) as CGFloat,
                        (subpixel_shift.y / params.scale_factor) as CGFloat,
                    )],
                    cx,
                );

            if params.is_emoji {
                // Convert from RGBA with premultiplied alpha to BGRA with straight alpha.
                for pixel in bytes.chunks_exact_mut(4) {
                    pixel.swap(0, 2);
                    let a = pixel[3] as f32 / 255.;
                    pixel[0] = (pixel[0] as f32 / a) as u8;
                    pixel[1] = (pixel[1] as f32 / a) as u8;
                    pixel[2] = (pixel[2] as f32 / a) as u8;
                }
            }

            Ok((bitmap_size, bytes))
        }
    }

    fn layout_line(&mut self, text: &str, font_size: Pixels, font_runs: &[FontRun]) -> LineLayout {
        // Construct the attributed string, converting UTF8 ranges to UTF16 ranges.
        let mut string = CFMutableAttributedString::new();
        {
            string.replace_str(&CFString::new(text), CFRange::init(0, 0));
            let utf16_line_len = string.char_len() as usize;

            let mut ix_converter = StringIndexConverter::new(text);
            for run in font_runs {
                let utf8_end = ix_converter.utf8_ix + run.len;
                let utf16_start = ix_converter.utf16_ix;

                if utf16_start >= utf16_line_len {
                    break;
                }

                ix_converter.advance_to_utf8_ix(utf8_end);
                let utf16_end = cmp::min(ix_converter.utf16_ix, utf16_line_len);

                let cf_range =
                    CFRange::init(utf16_start as isize, (utf16_end - utf16_start) as isize);

                let font: &FontKitFont = &self.fonts[run.font_id.0];
                unsafe {
                    string.set_attribute(
                        cf_range,
                        kCTFontAttributeName,
                        &font.native_font().clone_with_font_size(font_size.into()),
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
                attributes
                    .get(kCTFontAttributeName)
                    .downcast::<CTFont>()
                    .unwrap()
            };
            let font_id = self.id_for_native_font(font);

            let mut ix_converter = StringIndexConverter::new(text);
            let mut glyphs = SmallVec::new();
            for ((glyph_id, position), glyph_utf16_ix) in run
                .glyphs()
                .iter()
                .zip(run.positions().iter())
                .zip(run.string_indices().iter())
            {
                let glyph_utf16_ix = usize::try_from(*glyph_utf16_ix).unwrap();
                ix_converter.advance_to_utf16_ix(glyph_utf16_ix);
                glyphs.push(ShapedGlyph {
                    id: (*glyph_id).into(),
                    position: point(position.x as f32, position.y as f32).map(px),
                    index: ix_converter.utf8_ix,
                    is_emoji: self.is_emoji(font_id),
                });
            }

            runs.push(ShapedRun { font_id, glyphs })
        }

        let typographic_bounds = line.get_typographic_bounds();
        LineLayout {
            runs,
            font_size,
            width: typographic_bounds.width.into(),
            ascent: typographic_bounds.ascent.into(),
            descent: typographic_bounds.descent.into(),
            len: text.len(),
        }
    }

    fn wrap_line(
        &self,
        text: &str,
        font_id: FontId,
        font_size: Pixels,
        width: Pixels,
    ) -> Vec<usize> {
        let mut string = CFMutableAttributedString::new();
        string.replace_str(&CFString::new(text), CFRange::init(0, 0));
        let cf_range = CFRange::init(0, text.encode_utf16().count() as isize);
        let font = &self.fonts[font_id.0];
        unsafe {
            string.set_attribute(
                cf_range,
                kCTFontAttributeName,
                &font.native_font().clone_with_font_size(font_size.into()),
            );

            let typesetter = CTTypesetterCreateWithAttributedString(string.as_concrete_TypeRef());
            let mut ix_converter = StringIndexConverter::new(text);
            let mut break_indices = Vec::new();
            while ix_converter.utf8_ix < text.len() {
                let utf16_len = CTTypesetterSuggestLineBreak(
                    typesetter,
                    ix_converter.utf16_ix as isize,
                    width.into(),
                ) as usize;
                ix_converter.advance_to_utf16_ix(ix_converter.utf16_ix + utf16_len);
                if ix_converter.utf8_ix >= text.len() {
                    break;
                }
                break_indices.push(ix_converter.utf8_ix);
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
pub(crate) struct __CFTypesetter(c_void);

type CTTypesetterRef = *const __CFTypesetter;

#[link(name = "CoreText", kind = "framework")]
extern "C" {
    fn CTTypesetterCreateWithAttributedString(string: CFAttributedStringRef) -> CTTypesetterRef;

    fn CTTypesetterSuggestLineBreak(
        typesetter: CTTypesetterRef,
        start_index: CFIndex,
        width: f64,
    ) -> CFIndex;
}

impl From<Metrics> for FontMetrics {
    fn from(metrics: Metrics) -> Self {
        FontMetrics {
            units_per_em: metrics.units_per_em,
            ascent: metrics.ascent,
            descent: metrics.descent,
            line_gap: metrics.line_gap,
            underline_position: metrics.underline_position,
            underline_thickness: metrics.underline_thickness,
            cap_height: metrics.cap_height,
            x_height: metrics.x_height,
            bounding_box: metrics.bounding_box.into(),
        }
    }
}

impl From<RectF> for Bounds<f32> {
    fn from(rect: RectF) -> Self {
        Bounds {
            origin: point(rect.origin_x(), rect.origin_y()),
            size: size(rect.width(), rect.height()),
        }
    }
}

impl From<RectI> for Bounds<DevicePixels> {
    fn from(rect: RectI) -> Self {
        Bounds {
            origin: point(DevicePixels(rect.origin_x()), DevicePixels(rect.origin_y())),
            size: size(DevicePixels(rect.width()), DevicePixels(rect.height())),
        }
    }
}

impl From<Vector2I> for Size<DevicePixels> {
    fn from(value: Vector2I) -> Self {
        size(value.x().into(), value.y().into())
    }
}

impl From<RectI> for Bounds<i32> {
    fn from(rect: RectI) -> Self {
        Bounds {
            origin: point(rect.origin_x(), rect.origin_y()),
            size: size(rect.width(), rect.height()),
        }
    }
}

impl From<Point<u32>> for Vector2I {
    fn from(size: Point<u32>) -> Self {
        Vector2I::new(size.x as i32, size.y as i32)
    }
}

impl From<Vector2F> for Size<f32> {
    fn from(vec: Vector2F) -> Self {
        size(vec.x(), vec.y())
    }
}

impl From<FontWeight> for FontkitWeight {
    fn from(value: FontWeight) -> Self {
        FontkitWeight(value.0)
    }
}

impl From<FontStyle> for FontkitStyle {
    fn from(style: FontStyle) -> Self {
        match style {
            FontStyle::Normal => FontkitStyle::Normal,
            FontStyle::Italic => FontkitStyle::Italic,
            FontStyle::Oblique => FontkitStyle::Oblique,
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::AppContext;
//     use font_kit::properties::{Style, Weight};
//     use platform::FontSystem as _;

//     #[crate::test(self, retries = 5)]
//     fn test_layout_str(_: &mut AppContext) {
//         // This is failing intermittently on CI and we don't have time to figure it out
//         let fonts = FontSystem::new();
//         let menlo = fonts.load_family("Menlo", &Default::default()).unwrap();
//         let menlo_regular = RunStyle {
//             font_id: fonts.select_font(&menlo, &Properties::new()).unwrap(),
//             color: Default::default(),
//             underline: Default::default(),
//         };
//         let menlo_italic = RunStyle {
//             font_id: fonts
//                 .select_font(&menlo, Properties::new().style(Style::Italic))
//                 .unwrap(),
//             color: Default::default(),
//             underline: Default::default(),
//         };
//         let menlo_bold = RunStyle {
//             font_id: fonts
//                 .select_font(&menlo, Properties::new().weight(Weight::BOLD))
//                 .unwrap(),
//             color: Default::default(),
//             underline: Default::default(),
//         };
//         assert_ne!(menlo_regular, menlo_italic);
//         assert_ne!(menlo_regular, menlo_bold);
//         assert_ne!(menlo_italic, menlo_bold);

//         let line = fonts.layout_line(
//             "hello world",
//             16.0,
//             &[(2, menlo_bold), (4, menlo_italic), (5, menlo_regular)],
//         );
//         assert_eq!(line.runs.len(), 3);
//         assert_eq!(line.runs[0].font_id, menlo_bold.font_id);
//         assert_eq!(line.runs[0].glyphs.len(), 2);
//         assert_eq!(line.runs[1].font_id, menlo_italic.font_id);
//         assert_eq!(line.runs[1].glyphs.len(), 4);
//         assert_eq!(line.runs[2].font_id, menlo_regular.font_id);
//         assert_eq!(line.runs[2].glyphs.len(), 5);
//     }

//     #[test]
//     fn test_glyph_offsets() -> crate::Result<()> {
//         let fonts = FontSystem::new();
//         let zapfino = fonts.load_family("Zapfino", &Default::default())?;
//         let zapfino_regular = RunStyle {
//             font_id: fonts.select_font(&zapfino, &Properties::new())?,
//             color: Default::default(),
//             underline: Default::default(),
//         };
//         let menlo = fonts.load_family("Menlo", &Default::default())?;
//         let menlo_regular = RunStyle {
//             font_id: fonts.select_font(&menlo, &Properties::new())?,
//             color: Default::default(),
//             underline: Default::default(),
//         };

//         let text = "This is, m𐍈re 𐍈r less, Zapfino!𐍈";
//         let line = fonts.layout_line(
//             text,
//             16.0,
//             &[
//                 (9, zapfino_regular),
//                 (13, menlo_regular),
//                 (text.len() - 22, zapfino_regular),
//             ],
//         );
//         assert_eq!(
//             line.runs
//                 .iter()
//                 .flat_map(|r| r.glyphs.iter())
//                 .map(|g| g.index)
//                 .collect::<Vec<_>>(),
//             vec![0, 2, 4, 5, 7, 8, 9, 10, 14, 15, 16, 17, 21, 22, 23, 24, 26, 27, 28, 29, 36, 37],
//         );
//         Ok(())
//     }

//     #[test]
//     #[ignore]
//     fn test_rasterize_glyph() {
//         use std::{fs::File, io::BufWriter, path::Path};

//         let fonts = FontSystem::new();
//         let font_ids = fonts.load_family("Fira Code", &Default::default()).unwrap();
//         let font_id = fonts.select_font(&font_ids, &Default::default()).unwrap();
//         let glyph_id = fonts.glyph_for_char(font_id, 'G').unwrap();

//         const VARIANTS: usize = 1;
//         for i in 0..VARIANTS {
//             let variant = i as f32 / VARIANTS as f32;
//             let (bounds, bytes) = fonts
//                 .rasterize_glyph(
//                     font_id,
//                     16.0,
//                     glyph_id,
//                     vec2f(variant, variant),
//                     2.,
//                     RasterizationOptions::Alpha,
//                 )
//                 .unwrap();

//             let name = format!("/Users/as-cii/Desktop/twog-{}.png", i);
//             let path = Path::new(&name);
//             let file = File::create(path).unwrap();
//             let w = &mut BufWriter::new(file);

//             let mut encoder = png::Encoder::new(w, bounds.width() as u32, bounds.height() as u32);
//             encoder.set_color(png::ColorType::Grayscale);
//             encoder.set_depth(png::BitDepth::Eight);
//             let mut writer = encoder.write_header().unwrap();
//             writer.write_image_data(&bytes).unwrap();
//         }
//     }

//     #[test]
//     fn test_wrap_line() {
//         let fonts = FontSystem::new();
//         let font_ids = fonts.load_family("Helvetica", &Default::default()).unwrap();
//         let font_id = fonts.select_font(&font_ids, &Default::default()).unwrap();

//         let line = "one two three four five\n";
//         let wrap_boundaries = fonts.wrap_line(line, font_id, 16., 64.0);
//         assert_eq!(wrap_boundaries, &["one two ".len(), "one two three ".len()]);

//         let line = "aaa ααα ✋✋✋ 🎉🎉🎉\n";
//         let wrap_boundaries = fonts.wrap_line(line, font_id, 16., 64.0);
//         assert_eq!(
//             wrap_boundaries,
//             &["aaa ααα ".len(), "aaa ααα ✋✋✋ ".len(),]
//         );
//     }

//     #[test]
//     fn test_layout_line_bom_char() {
//         let fonts = FontSystem::new();
//         let font_ids = fonts.load_family("Helvetica", &Default::default()).unwrap();
//         let style = RunStyle {
//             font_id: fonts.select_font(&font_ids, &Default::default()).unwrap(),
//             color: Default::default(),
//             underline: Default::default(),
//         };

//         let line = "\u{feff}";
//         let layout = fonts.layout_line(line, 16., &[(line.len(), style)]);
//         assert_eq!(layout.len, line.len());
//         assert!(layout.runs.is_empty());

//         let line = "a\u{feff}b";
//         let layout = fonts.layout_line(line, 16., &[(line.len(), style)]);
//         assert_eq!(layout.len, line.len());
//         assert_eq!(layout.runs.len(), 1);
//         assert_eq!(layout.runs[0].glyphs.len(), 2);
//         assert_eq!(layout.runs[0].glyphs[0].id, 68); // a
//                                                      // There's no glyph for \u{feff}
//         assert_eq!(layout.runs[0].glyphs[1].id, 69); // b
//     }
// }
