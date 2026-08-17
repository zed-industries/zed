// font-kit/src/loaders/core_text.rs
//
// Copyright © 2018 The Pathfinder Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! A loader that uses Apple's Core Text API to load and rasterize fonts.

use byteorder::{BigEndian, ReadBytesExt};
use core_graphics::base::{kCGImageAlphaPremultipliedLast, CGFloat};
use core_graphics::color_space::CGColorSpace;
use core_graphics::context::{CGContext, CGTextDrawingMode};
use core_graphics::font::{CGFont, CGGlyph};
use core_graphics::geometry::{CGAffineTransform, CGPoint, CGRect, CGSize};
use core_graphics::geometry::{CG_AFFINE_TRANSFORM_IDENTITY, CG_ZERO_POINT, CG_ZERO_SIZE};
use core_graphics::path::CGPathElementType;
use core_text;
use core_text::font::CTFont;
use core_text::font_descriptor::kCTFontDefaultOrientation;
use core_text::font_descriptor::{SymbolicTraitAccessors, TraitAccessors};
use log::warn;
use pathfinder_geometry::line_segment::LineSegment2F;
use pathfinder_geometry::rect::{RectF, RectI};
use pathfinder_geometry::transform2d::Transform2F;
use pathfinder_geometry::vector::Vector2F;
use pathfinder_simd::default::F32x4;
use std::cmp::Ordering;
use std::f32;
use std::fmt::{self, Debug, Formatter};
use std::fs::File;
use std::io::{Seek, SeekFrom};
use std::ops::Deref;
use std::path::Path;
use std::sync::Arc;

use crate::canvas::{Canvas, Format, RasterizationOptions};
use crate::error::{FontLoadingError, GlyphLoadingError};
use crate::file_type::FileType;
use crate::handle::Handle;
use crate::hinting::HintingOptions;
use crate::loader::{FallbackResult, Loader};
use crate::metrics::Metrics;
use crate::outline::OutlineSink;
use crate::properties::{Properties, Stretch, Style, Weight};
use crate::utils;

const TTC_TAG: [u8; 4] = [b't', b't', b'c', b'f'];
const OTTO_TAG: [u8; 4] = [b'O', b'T', b'T', b'O'];
const OTTO_HEX: u32 = 0x4f54544f; // 'OTTO'
const TRUE_HEX: u32 = 0x74727565; // 'true'
const TYP1_HEX: u32 = 0x74797031; // 'typ1'
const SFNT_HEX: u32 = 0x73666e74; // 'sfnt'

#[allow(non_upper_case_globals)]
const kCGImageAlphaOnly: u32 = 7;

pub(crate) static FONT_WEIGHT_MAPPING: [f32; 9] = [-0.7, -0.5, -0.23, 0.0, 0.2, 0.3, 0.4, 0.6, 0.8];

/// Core Text's representation of a font.
pub type NativeFont = CTFont;

/// A loader that uses Apple's Core Text API to load and rasterize fonts.
#[derive(Clone)]
pub struct Font {
    core_text_font: CTFont,
    font_data: FontData,
}

impl Font {
    /// Loads a font from raw font data (the contents of a `.ttf`/`.otf`/etc. file).
    ///
    /// If the data represents a collection (`.ttc`/`.otc`/etc.), `font_index` specifies the index
    /// of the font to load from it. If the data represents a single font, pass 0 for `font_index`.
    pub fn from_bytes(
        mut font_data: Arc<Vec<u8>>,
        font_index: u32,
    ) -> Result<Font, FontLoadingError> {
        // Sadly, there's no API to load OpenType collections on macOS, I don't believe…
        // If not otf/ttf or otc/ttc, we unpack it as data fork font.
        if !font_is_single_otf(&*font_data) && !font_is_collection(&*font_data) {
            let mut new_font_data = (*font_data).clone();
            unpack_data_fork_font(&mut new_font_data)?;
            font_data = Arc::new(new_font_data);
        } else if font_is_collection(&*font_data) {
            let mut new_font_data = (*font_data).clone();
            unpack_otc_font(&mut new_font_data, font_index)?;
            font_data = Arc::new(new_font_data);
        }

        let core_text_font = match core_text::font::new_from_buffer(&*font_data) {
            Ok(ct_font) => ct_font,
            Err(_) => return Err(FontLoadingError::Parse),
        };

        Ok(Font {
            core_text_font,
            font_data: FontData::Memory(font_data),
        })
    }

    /// Loads a font from a `.ttf`/`.otf`/etc. file.
    ///
    /// If the file is a collection (`.ttc`/`.otc`/etc.), `font_index` specifies the index of the
    /// font to load from it. If the file represents a single font, pass 0 for `font_index`.
    pub fn from_file(file: &mut File, font_index: u32) -> Result<Font, FontLoadingError> {
        file.seek(SeekFrom::Start(0))?;
        let font_data = Arc::new(utils::slurp_file(file).map_err(FontLoadingError::Io)?);
        Font::from_bytes(font_data, font_index)
    }

    /// Loads a font from the path to a `.ttf`/`.otf`/etc. file.
    ///
    /// If the file is a collection (`.ttc`/`.otc`/etc.), `font_index` specifies the index of the
    /// font to load from it. If the file represents a single font, pass 0 for `font_index`.
    #[inline]
    pub fn from_path<P: AsRef<Path>>(path: P, font_index: u32) -> Result<Font, FontLoadingError> {
        <Font as Loader>::from_path(path, font_index)
    }

    /// Creates a font from a native API handle.
    pub unsafe fn from_native_font(core_text_font: &NativeFont) -> Font {
        Font::from_core_text_font_no_path(core_text_font.clone())
    }
    /// Creates a font from a native API handle, without performing a lookup on the disk.
    pub unsafe fn from_core_text_font_no_path(core_text_font: NativeFont) -> Font {
        Font {
            core_text_font,
            font_data: FontData::Unavailable,
        }
    }

    /// Creates a font from a Core Graphics font handle.
    ///
    /// This function is only available on the Core Text backend.
    pub fn from_core_graphics_font(core_graphics_font: CGFont) -> Font {
        unsafe {
            Font::from_core_text_font_no_path(core_text::font::new_from_CGFont(
                &core_graphics_font,
                16.0,
            ))
        }
    }

    /// Loads the font pointed to by a handle.
    #[inline]
    pub fn from_handle(handle: &Handle) -> Result<Self, FontLoadingError> {
        <Self as Loader>::from_handle(handle)
    }

    /// Determines whether a file represents a supported font, and if so, what type of font it is.
    pub fn analyze_bytes(font_data: Arc<Vec<u8>>) -> Result<FileType, FontLoadingError> {
        if let Ok(font_count) = read_number_of_fonts_from_otc_header(&font_data) {
            return Ok(FileType::Collection(font_count));
        }
        match core_text::font::new_from_buffer(&*font_data) {
            Ok(_) => Ok(FileType::Single),
            Err(_) => Err(FontLoadingError::Parse),
        }
    }

    /// Determines whether a file represents a supported font, and if so, what type of font it is.
    pub fn analyze_file(file: &mut File) -> Result<FileType, FontLoadingError> {
        file.seek(SeekFrom::Start(0))?;

        let font_data = Arc::new(utils::slurp_file(file).map_err(FontLoadingError::Io)?);
        if let Ok(font_count) = read_number_of_fonts_from_otc_header(&font_data) {
            return Ok(FileType::Collection(font_count));
        }

        match core_text::font::new_from_buffer(&*font_data) {
            Ok(_) => Ok(FileType::Single),
            Err(_) => Err(FontLoadingError::Parse),
        }
    }

    /// Determines whether a path points to a supported font, and if so, what type of font it is.
    #[inline]
    pub fn analyze_path<P: AsRef<Path>>(path: P) -> Result<FileType, FontLoadingError> {
        <Self as Loader>::analyze_path(path)
    }

    /// Returns the wrapped native font handle.
    #[inline]
    pub fn native_font(&self) -> NativeFont {
        self.core_text_font.clone()
    }

    /// Returns the PostScript name of the font. This should be globally unique.
    #[inline]
    pub fn postscript_name(&self) -> Option<String> {
        Some(self.core_text_font.postscript_name())
    }

    /// Returns the full name of the font (also known as "display name" on macOS).
    #[inline]
    pub fn full_name(&self) -> String {
        self.core_text_font.display_name()
    }

    /// Returns the name of the font family.
    #[inline]
    pub fn family_name(&self) -> String {
        self.core_text_font.family_name()
    }

    /// Returns the name of the font style, according to Core Text.
    ///
    /// NB: This function is only available on the Core Text backend.
    #[inline]
    pub fn style_name(&self) -> String {
        self.core_text_font.style_name()
    }

    /// Returns true if and only if the font is monospace (fixed-width).
    #[inline]
    pub fn is_monospace(&self) -> bool {
        self.core_text_font.symbolic_traits().is_monospace()
    }

    /// Returns the values of various font properties, corresponding to those defined in CSS.
    pub fn properties(&self) -> Properties {
        let symbolic_traits = self.core_text_font.symbolic_traits();
        let all_traits = self.core_text_font.all_traits();

        let style = if symbolic_traits.is_italic() {
            Style::Italic
        } else if all_traits.normalized_slant() > 0.0 {
            Style::Oblique
        } else {
            Style::Normal
        };

        let weight = core_text_to_css_font_weight(all_traits.normalized_weight() as f32);
        let stretch = core_text_width_to_css_stretchiness(all_traits.normalized_width() as f32);

        Properties {
            style,
            weight,
            stretch,
        }
    }

    /// Returns the number of glyphs in the font.
    ///
    /// Glyph IDs range from 0 inclusive to this value exclusive.
    pub fn glyph_count(&self) -> u32 {
        self.core_text_font.glyph_count() as u32
    }

    /// Returns the usual glyph ID for a Unicode character.
    ///
    /// Be careful with this function; typographically correct character-to-glyph mapping must be
    /// done using a *shaper* such as HarfBuzz. This function is only useful for best-effort simple
    /// use cases like "what does character X look like on its own".
    pub fn glyph_for_char(&self, character: char) -> Option<u32> {
        unsafe {
            let (mut dest, mut src) = ([0, 0], [0, 0]);
            let src = character.encode_utf16(&mut src);
            self.core_text_font
                .get_glyphs_for_characters(src.as_ptr(), dest.as_mut_ptr(), 2);

            let id = dest[0] as u32;
            if id != 0 {
                Some(id)
            } else {
                None
            }
        }
    }

    /// Returns the glyph ID for the specified glyph name.
    #[inline]
    pub fn glyph_by_name(&self, name: &str) -> Option<u32> {
        let code = self.core_text_font.get_glyph_with_name(name);

        Some(u32::from(code))
    }

    /// Sends the vector path for a glyph to a path builder.
    ///
    /// If `hinting_mode` is not None, this function performs grid-fitting as requested before
    /// sending the hinding outlines to the builder.
    ///
    /// TODO(pcwalton): What should we do for bitmap glyphs?
    pub fn outline<S>(
        &self,
        glyph_id: u32,
        _: HintingOptions,
        sink: &mut S,
    ) -> Result<(), GlyphLoadingError>
    where
        S: OutlineSink,
    {
        let path = match self
            .core_text_font
            .create_path_for_glyph(glyph_id as u16, &CG_AFFINE_TRANSFORM_IDENTITY)
        {
            Ok(path) => path,
            Err(_) => {
                // This will happen if the path is empty (rdar://42832439). To distinguish this
                // case from the case in which the glyph does not exist, call another API.
                self.typographic_bounds(glyph_id)?;
                return Ok(());
            }
        };

        let units_per_point = self.units_per_point() as f32;
        path.apply(&|element| {
            let points = element.points();
            match element.element_type {
                CGPathElementType::MoveToPoint => {
                    sink.move_to(points[0].to_vector() * units_per_point)
                }
                CGPathElementType::AddLineToPoint => {
                    sink.line_to(points[0].to_vector() * units_per_point)
                }
                CGPathElementType::AddQuadCurveToPoint => sink.quadratic_curve_to(
                    points[0].to_vector() * units_per_point,
                    points[1].to_vector() * units_per_point,
                ),
                CGPathElementType::AddCurveToPoint => {
                    let ctrl = LineSegment2F::new(points[0].to_vector(), points[1].to_vector())
                        * units_per_point;
                    sink.cubic_curve_to(ctrl, points[2].to_vector() * units_per_point)
                }
                CGPathElementType::CloseSubpath => sink.close(),
            }
        });
        Ok(())
    }

    /// Returns the boundaries of a glyph in font units.
    pub fn typographic_bounds(&self, glyph_id: u32) -> Result<RectF, GlyphLoadingError> {
        let rect = self
            .core_text_font
            .get_bounding_rects_for_glyphs(kCTFontDefaultOrientation, &[glyph_id as u16]);
        let rect = RectF::new(
            Vector2F::new(rect.origin.x as f32, rect.origin.y as f32),
            Vector2F::new(rect.size.width as f32, rect.size.height as f32),
        );
        Ok(rect * self.units_per_point() as f32)
    }

    /// Returns the distance from the origin of the glyph with the given ID to the next, in font
    /// units.
    pub fn advance(&self, glyph_id: u32) -> Result<Vector2F, GlyphLoadingError> {
        // FIXME(pcwalton): Apple's docs don't say what happens when the glyph is out of range!
        unsafe {
            let (glyph_id, mut advance) = (glyph_id as u16, CG_ZERO_SIZE);
            self.core_text_font.get_advances_for_glyphs(
                kCTFontDefaultOrientation,
                &glyph_id,
                &mut advance,
                1,
            );
            let advance = Vector2F::new(advance.width as f32, advance.height as f32);
            Ok(advance * self.units_per_point() as f32)
        }
    }

    /// Returns the amount that the given glyph should be displaced from the origin.
    pub fn origin(&self, glyph_id: u32) -> Result<Vector2F, GlyphLoadingError> {
        unsafe {
            // FIXME(pcwalton): Apple's docs don't say what happens when the glyph is out of range!
            let (glyph_id, mut translation) = (glyph_id as u16, CG_ZERO_SIZE);
            self.core_text_font.get_vertical_translations_for_glyphs(
                kCTFontDefaultOrientation,
                &glyph_id,
                &mut translation,
                1,
            );
            let translation = Vector2F::new(translation.width as f32, translation.height as f32);
            Ok(translation * self.units_per_point() as f32)
        }
    }

    /// Retrieves various metrics that apply to the entire font.
    pub fn metrics(&self) -> Metrics {
        let units_per_em = self.core_text_font.units_per_em();
        let units_per_point = (units_per_em as f64) / self.core_text_font.pt_size();

        let bounding_box = self.core_text_font.bounding_box();
        let bounding_box = RectF::new(
            Vector2F::new(bounding_box.origin.x as f32, bounding_box.origin.y as f32),
            Vector2F::new(
                bounding_box.size.width as f32,
                bounding_box.size.height as f32,
            ),
        );
        let bounding_box = bounding_box * units_per_point as f32;

        Metrics {
            units_per_em,
            ascent: (self.core_text_font.ascent() * units_per_point) as f32,
            descent: (-self.core_text_font.descent() * units_per_point) as f32,
            line_gap: (self.core_text_font.leading() * units_per_point) as f32,
            underline_position: (self.core_text_font.underline_position() * units_per_point) as f32,
            underline_thickness: (self.core_text_font.underline_thickness() * units_per_point)
                as f32,
            cap_height: (self.core_text_font.cap_height() * units_per_point) as f32,
            x_height: (self.core_text_font.x_height() * units_per_point) as f32,
            bounding_box,
        }
    }

    /// Returns a handle to this font, if possible.
    ///
    /// This is useful if you want to open the font with a different loader.
    #[inline]
    pub fn handle(&self) -> Option<Handle> {
        <Self as Loader>::handle(self)
    }

    /// Attempts to return the raw font data (contents of the font file).
    ///
    /// If this font is a member of a collection, this function returns the data for the entire
    /// collection.
    pub fn copy_font_data(&self) -> Option<Arc<Vec<u8>>> {
        match self.font_data {
            FontData::Unavailable => None,
            FontData::Memory(ref memory) => Some((*memory).clone()),
        }
    }

    /// Returns the pixel boundaries that the glyph will take up when rendered using this loader's
    /// rasterizer at the given size and transform.
    #[inline]
    pub fn raster_bounds(
        &self,
        glyph_id: u32,
        point_size: f32,
        transform: Transform2F,
        hinting_options: HintingOptions,
        rasterization_options: RasterizationOptions,
    ) -> Result<RectI, GlyphLoadingError> {
        <Self as Loader>::raster_bounds(
            self,
            glyph_id,
            point_size,
            transform,
            hinting_options,
            rasterization_options,
        )
    }

    /// Rasterizes a glyph to a canvas with the given size and origin.
    ///
    /// Format conversion will be performed if the canvas format does not match the rasterization
    /// options. For example, if bilevel (black and white) rendering is requested to an RGBA
    /// surface, this function will automatically convert the 1-bit raster image to the 32-bit
    /// format of the canvas. Note that this may result in a performance penalty, depending on the
    /// loader.
    ///
    /// If `hinting_options` is not None, the requested grid fitting is performed.
    ///
    /// TODO(pcwalton): This is woefully incomplete. See WebRender's code for a more complete
    /// implementation.
    pub fn rasterize_glyph(
        &self,
        canvas: &mut Canvas,
        glyph_id: u32,
        point_size: f32,
        transform: Transform2F,
        hinting_options: HintingOptions,
        rasterization_options: RasterizationOptions,
    ) -> Result<(), GlyphLoadingError> {
        if canvas.size.x() == 0 || canvas.size.y() == 0 {
            return Ok(());
        }

        let (cg_color_space, cg_image_format) =
            match format_to_cg_color_space_and_image_format(canvas.format) {
                None => {
                    // Core Graphics doesn't support the requested image format. Allocate a
                    // temporary canvas, then perform color conversion.
                    //
                    // FIXME(pcwalton): Could improve this by only allocating a canvas with a tight
                    // bounding rect and blitting only that part.
                    let mut temp_canvas = Canvas::new(canvas.size, Format::Rgba32);
                    self.rasterize_glyph(
                        &mut temp_canvas,
                        glyph_id,
                        point_size,
                        transform,
                        hinting_options,
                        rasterization_options,
                    )?;
                    canvas.blit_from_canvas(&temp_canvas);
                    return Ok(());
                }
                Some(cg_color_space_and_format) => cg_color_space_and_format,
            };

        let core_graphics_context = CGContext::create_bitmap_context(
            Some(canvas.pixels.as_mut_ptr() as *mut _),
            canvas.size.x() as usize,
            canvas.size.y() as usize,
            canvas.format.bits_per_component() as usize,
            canvas.stride,
            &cg_color_space,
            cg_image_format,
        );

        match canvas.format {
            Format::Rgba32 | Format::Rgb24 => {
                core_graphics_context.set_rgb_fill_color(0.0, 0.0, 0.0, 0.0);
            }
            Format::A8 => core_graphics_context.set_gray_fill_color(0.0, 0.0),
        }

        let core_graphics_size = CGSize::new(canvas.size.x() as f64, canvas.size.y() as f64);
        core_graphics_context.fill_rect(CGRect::new(&CG_ZERO_POINT, &core_graphics_size));

        match rasterization_options {
            RasterizationOptions::Bilevel => {
                core_graphics_context.set_allows_font_smoothing(false);
                core_graphics_context.set_should_smooth_fonts(false);
                core_graphics_context.set_should_antialias(false);
            }
            RasterizationOptions::GrayscaleAa | RasterizationOptions::SubpixelAa => {
                // FIXME(pcwalton): These shouldn't be handled the same!
                core_graphics_context.set_allows_font_smoothing(true);
                core_graphics_context.set_should_smooth_fonts(true);
                core_graphics_context.set_should_antialias(true);
            }
        }

        match canvas.format {
            Format::Rgba32 | Format::Rgb24 => {
                core_graphics_context.set_rgb_fill_color(1.0, 1.0, 1.0, 1.0);
            }
            Format::A8 => core_graphics_context.set_gray_fill_color(1.0, 1.0),
        }

        // CoreGraphics origin is in the bottom left. This makes behavior consistent.
        core_graphics_context.translate(0.0, canvas.size.y() as CGFloat);
        core_graphics_context.set_font(&self.core_text_font.copy_to_CGFont());
        core_graphics_context.set_font_size(point_size as CGFloat);
        core_graphics_context.set_text_drawing_mode(CGTextDrawingMode::CGTextFill);
        let matrix = transform.matrix.0 * F32x4::new(1.0, -1.0, -1.0, 1.0);
        core_graphics_context.set_text_matrix(&CGAffineTransform {
            a: matrix.x() as CGFloat,
            b: matrix.y() as CGFloat,
            c: matrix.z() as CGFloat,
            d: matrix.w() as CGFloat,
            tx: transform.vector.x() as CGFloat,
            ty: -transform.vector.y() as CGFloat,
        });
        let origin = CGPoint::new(0.0, 0.0);
        core_graphics_context.show_glyphs_at_positions(&[glyph_id as CGGlyph], &[origin]);

        Ok(())
    }

    /// Returns true if and only if the font loader can perform hinting in the requested way.
    ///
    /// Some APIs support only rasterizing glyphs with hinting, not retrieving hinted outlines. If
    /// `for_rasterization` is false, this function returns true if and only if the loader supports
    /// retrieval of hinted *outlines*. If `for_rasterization` is true, this function returns true
    /// if and only if the loader supports *rasterizing* hinted glyphs.
    #[inline]
    pub fn supports_hinting_options(&self, hinting_options: HintingOptions, _: bool) -> bool {
        match hinting_options {
            HintingOptions::None => true,
            HintingOptions::Vertical(..)
            | HintingOptions::VerticalSubpixel(..)
            | HintingOptions::Full(..) => false,
        }
    }

    /// Get font fallback results for the given text and locale.
    ///
    /// Note: this is currently just a stub implementation, a proper implementation
    /// would use CTFontCopyDefaultCascadeListForLanguages.
    fn get_fallbacks(&self, text: &str, _locale: &str) -> FallbackResult<Font> {
        warn!("unsupported");
        FallbackResult {
            fonts: Vec::new(),
            valid_len: text.len(),
        }
    }

    #[inline]
    fn units_per_point(&self) -> f64 {
        (self.core_text_font.units_per_em() as f64) / self.core_text_font.pt_size()
    }

    /// Returns the raw contents of the OpenType table with the given tag.
    ///
    /// Tags are four-character codes. A list of tags can be found in the [OpenType specification].
    ///
    /// [OpenType specification]: https://docs.microsoft.com/en-us/typography/opentype/spec/
    #[inline]
    pub fn load_font_table(&self, table_tag: u32) -> Option<Box<[u8]>> {
        self.core_text_font
            .get_font_table(table_tag)
            .map(|data| data.bytes().into())
    }
}

impl Loader for Font {
    type NativeFont = NativeFont;

    #[inline]
    fn from_bytes(font_data: Arc<Vec<u8>>, font_index: u32) -> Result<Self, FontLoadingError> {
        Font::from_bytes(font_data, font_index)
    }

    #[inline]
    fn from_file(file: &mut File, font_index: u32) -> Result<Font, FontLoadingError> {
        Font::from_file(file, font_index)
    }

    #[inline]
    unsafe fn from_native_font(native_font: &Self::NativeFont) -> Self {
        Font::from_native_font(native_font)
    }

    #[inline]
    fn analyze_bytes(font_data: Arc<Vec<u8>>) -> Result<FileType, FontLoadingError> {
        Font::analyze_bytes(font_data)
    }

    #[inline]
    fn analyze_file(file: &mut File) -> Result<FileType, FontLoadingError> {
        Font::analyze_file(file)
    }

    #[inline]
    fn native_font(&self) -> Self::NativeFont {
        self.native_font()
    }

    #[inline]
    fn postscript_name(&self) -> Option<String> {
        self.postscript_name()
    }

    #[inline]
    fn full_name(&self) -> String {
        self.full_name()
    }

    #[inline]
    fn family_name(&self) -> String {
        self.family_name()
    }

    #[inline]
    fn is_monospace(&self) -> bool {
        self.is_monospace()
    }

    #[inline]
    fn properties(&self) -> Properties {
        self.properties()
    }

    #[inline]
    fn glyph_for_char(&self, character: char) -> Option<u32> {
        self.glyph_for_char(character)
    }

    #[inline]
    fn glyph_by_name(&self, name: &str) -> Option<u32> {
        self.glyph_by_name(name)
    }

    #[inline]
    fn glyph_count(&self) -> u32 {
        self.glyph_count()
    }

    #[inline]
    fn outline<S>(
        &self,
        glyph_id: u32,
        hinting_mode: HintingOptions,
        sink: &mut S,
    ) -> Result<(), GlyphLoadingError>
    where
        S: OutlineSink,
    {
        self.outline(glyph_id, hinting_mode, sink)
    }

    #[inline]
    fn typographic_bounds(&self, glyph_id: u32) -> Result<RectF, GlyphLoadingError> {
        self.typographic_bounds(glyph_id)
    }

    #[inline]
    fn advance(&self, glyph_id: u32) -> Result<Vector2F, GlyphLoadingError> {
        self.advance(glyph_id)
    }

    #[inline]
    fn origin(&self, glyph_id: u32) -> Result<Vector2F, GlyphLoadingError> {
        self.origin(glyph_id)
    }

    #[inline]
    fn metrics(&self) -> Metrics {
        self.metrics()
    }

    #[inline]
    fn copy_font_data(&self) -> Option<Arc<Vec<u8>>> {
        self.copy_font_data()
    }

    #[inline]
    fn supports_hinting_options(
        &self,
        hinting_options: HintingOptions,
        for_rasterization: bool,
    ) -> bool {
        self.supports_hinting_options(hinting_options, for_rasterization)
    }

    #[inline]
    fn rasterize_glyph(
        &self,
        canvas: &mut Canvas,
        glyph_id: u32,
        point_size: f32,
        transform: Transform2F,
        hinting_options: HintingOptions,
        rasterization_options: RasterizationOptions,
    ) -> Result<(), GlyphLoadingError> {
        self.rasterize_glyph(
            canvas,
            glyph_id,
            point_size,
            transform,
            hinting_options,
            rasterization_options,
        )
    }

    #[inline]
    fn get_fallbacks(&self, text: &str, locale: &str) -> FallbackResult<Self> {
        self.get_fallbacks(text, locale)
    }

    #[inline]
    fn load_font_table(&self, table_tag: u32) -> Option<Box<[u8]>> {
        self.load_font_table(table_tag)
    }
}

impl Debug for Font {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), fmt::Error> {
        self.full_name().fmt(fmt)
    }
}

#[derive(Clone)]
enum FontData {
    Unavailable,
    Memory(Arc<Vec<u8>>),
}

impl Deref for FontData {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        match *self {
            FontData::Unavailable => panic!("Font data unavailable!"),
            FontData::Memory(ref data) => &***data,
        }
    }
}

trait CGPointExt {
    fn to_vector(&self) -> Vector2F;
}

impl CGPointExt for CGPoint {
    #[inline]
    fn to_vector(&self) -> Vector2F {
        Vector2F::new(self.x as f32, self.y as f32)
    }
}

fn core_text_to_css_font_weight(core_text_weight: f32) -> Weight {
    let index = piecewise_linear_find_index(core_text_weight, &FONT_WEIGHT_MAPPING);

    Weight(index * 100.0 + 100.0)
}

fn core_text_width_to_css_stretchiness(core_text_width: f32) -> Stretch {
    Stretch(piecewise_linear_lookup(
        (core_text_width + 1.0) * 4.0,
        &Stretch::MAPPING,
    ))
}

fn font_is_collection(header: &[u8]) -> bool {
    header.len() >= 4 && header[0..4] == TTC_TAG
}

fn read_number_of_fonts_from_otc_header(header: &[u8]) -> Result<u32, FontLoadingError> {
    if !font_is_collection(header) {
        return Err(FontLoadingError::UnknownFormat);
    }
    Ok((&header[8..]).read_u32::<BigEndian>()?)
}

fn get_slice_from_start(slice: &[u8], start: usize) -> Result<&[u8], FontLoadingError> {
    slice.get(start..).ok_or(FontLoadingError::Parse)
}

// Unpacks an OTC font "in-place".
fn unpack_otc_font(data: &mut [u8], font_index: u32) -> Result<(), FontLoadingError> {
    if font_index >= read_number_of_fonts_from_otc_header(data)? {
        return Err(FontLoadingError::NoSuchFontInCollection);
    }

    let offset_table_pos_pos = 12 + 4 * font_index as usize;

    let offset_table_pos =
        get_slice_from_start(&data, offset_table_pos_pos)?.read_u32::<BigEndian>()? as usize;
    debug_assert!(utils::SFNT_VERSIONS
        .iter()
        .any(|version| { data[offset_table_pos..(offset_table_pos + 4)] == *version }));
    let num_tables = get_slice_from_start(&data, offset_table_pos + 4)?.read_u16::<BigEndian>()?;

    // Must copy forward in order to avoid problems with overlapping memory.
    let offset_table_and_table_record_size = 12 + (num_tables as usize) * 16;
    for offset in 0..offset_table_and_table_record_size {
        data[offset] = data[offset_table_pos + offset]
    }

    Ok(())
}

// NB: This assumes little-endian, but that's true for all extant Apple hardware.
fn format_to_cg_color_space_and_image_format(format: Format) -> Option<(CGColorSpace, u32)> {
    match format {
        Format::Rgb24 => {
            // Unsupported by Core Graphics.
            None
        }
        Format::Rgba32 => Some((
            CGColorSpace::create_device_rgb(),
            kCGImageAlphaPremultipliedLast,
        )),
        Format::A8 => Some((CGColorSpace::create_device_gray(), kCGImageAlphaOnly)),
    }
}

fn font_is_single_otf(header: &[u8]) -> bool {
    header.len() >= 4
        && ((&header[..4]).read_u32::<BigEndian>().unwrap() == 0x00010000
            || header[..4] == OTTO_TAG)
}

/// https://developer.apple.com/library/archive/documentation/mac/pdf/MoreMacintoshToolbox.pdf#page=151
fn unpack_data_fork_font(data: &mut [u8]) -> Result<(), FontLoadingError> {
    let data_offset = (&data[..]).read_u32::<BigEndian>()? as usize;
    let map_offset = get_slice_from_start(&data, 4)?.read_u32::<BigEndian>()? as usize;
    let num_types =
        get_slice_from_start(&data, map_offset + 28)?.read_u16::<BigEndian>()? as usize + 1;

    let mut font_data_offset = 0;
    let mut font_data_len = 0;

    let type_list_offset = get_slice_from_start(&data, map_offset + 24)?.read_u16::<BigEndian>()?
        as usize
        + map_offset;
    for i in 0..num_types {
        let res_type =
            get_slice_from_start(&data, map_offset + 30 + i * 8)?.read_u32::<BigEndian>()?;

        if res_type == SFNT_HEX {
            let ref_list_offset = get_slice_from_start(&data, map_offset + 30 + i * 8 + 6)?
                .read_u16::<BigEndian>()? as usize;
            let res_data_offset =
                get_slice_from_start(&data, type_list_offset + ref_list_offset + 5)?
                    .read_u24::<BigEndian>()? as usize;
            font_data_len = get_slice_from_start(&data, data_offset + res_data_offset)?
                .read_u32::<BigEndian>()? as usize;
            font_data_offset = data_offset + res_data_offset + 4;
            let sfnt_version =
                get_slice_from_start(&data, font_data_offset)?.read_u32::<BigEndian>()?;

            // TrueType outline, 'OTTO', 'true', 'typ1'
            if sfnt_version == 0x00010000
                || sfnt_version == OTTO_HEX
                || sfnt_version == TRUE_HEX
                || sfnt_version == TYP1_HEX
            {
                break;
            }
        }
    }

    if font_data_len == 0 {
        return Err(FontLoadingError::Parse);
    }

    for offset in 0..font_data_len {
        data[offset] = data[font_data_offset + offset];
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::Font;
    use crate::properties::{Stretch, Weight};

    #[cfg(feature = "source")]
    use crate::source::SystemSource;

    static TEST_FONT_POSTSCRIPT_NAME: &'static str = "ArialMT";

    #[cfg(feature = "source")]
    #[test]
    fn test_from_core_graphics_font() {
        let font0 = SystemSource::new()
            .select_by_postscript_name(TEST_FONT_POSTSCRIPT_NAME)
            .unwrap()
            .load()
            .unwrap();
        let core_text_font = font0.native_font();
        let core_graphics_font = core_text_font.copy_to_CGFont();
        let font1 = Font::from_core_graphics_font(core_graphics_font);
        assert_eq!(font1.postscript_name().unwrap(), TEST_FONT_POSTSCRIPT_NAME);
    }

    #[test]
    fn test_core_text_to_css_font_weight() {
        // Exact matches
        assert_eq!(super::core_text_to_css_font_weight(-0.7), Weight(100.0));
        assert_eq!(super::core_text_to_css_font_weight(0.0), Weight(400.0));
        assert_eq!(super::core_text_to_css_font_weight(0.4), Weight(700.0));
        assert_eq!(super::core_text_to_css_font_weight(0.8), Weight(900.0));

        // Linear interpolation
        assert_eq!(super::core_text_to_css_font_weight(0.1), Weight(450.0));
    }

    #[test]
    fn test_core_text_to_css_font_stretch() {
        // Exact matches
        assert_eq!(
            super::core_text_width_to_css_stretchiness(0.0),
            Stretch(1.0)
        );
        assert_eq!(
            super::core_text_width_to_css_stretchiness(-1.0),
            Stretch(0.5)
        );
        assert_eq!(
            super::core_text_width_to_css_stretchiness(1.0),
            Stretch(2.0)
        );

        // Linear interpolation
        assert_eq!(
            super::core_text_width_to_css_stretchiness(0.85),
            Stretch(1.7)
        );
    }
}

pub(crate) fn piecewise_linear_lookup(index: f32, mapping: &[f32]) -> f32 {
    let lower_value = mapping[f32::floor(index) as usize];
    let upper_value = mapping[f32::ceil(index) as usize];
    utils::lerp(lower_value, upper_value, f32::fract(index))
}

pub(crate) fn piecewise_linear_find_index(query_value: f32, mapping: &[f32]) -> f32 {
    let upper_index = match mapping
        .binary_search_by(|value| value.partial_cmp(&query_value).unwrap_or(Ordering::Less))
    {
        Ok(index) => return index as f32,
        Err(upper_index) => upper_index,
    };
    if upper_index == 0 || upper_index >= mapping.len() {
        return upper_index as f32;
    }
    let lower_index = upper_index - 1;
    let (upper_value, lower_value) = (mapping[upper_index], mapping[lower_index]);
    let t = (query_value - lower_value) / (upper_value - lower_value);
    lower_index as f32 + t
}
