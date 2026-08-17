// font-kit/src/loaders/directwrite.rs
//
// Copyright © 2018 The Pathfinder Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! A loader that uses the Windows DirectWrite API to load and rasterize fonts.

use byteorder::{BigEndian, ReadBytesExt};
use dwrote::CustomFontCollectionLoaderImpl;
use dwrote::Font as DWriteFont;
use dwrote::FontCollection as DWriteFontCollection;
use dwrote::FontFace as DWriteFontFace;
use dwrote::FontFallback as DWriteFontFallback;
use dwrote::FontFile as DWriteFontFile;
use dwrote::FontMetrics as DWriteFontMetrics;
use dwrote::FontStyle as DWriteFontStyle;
use dwrote::GlyphOffset as DWriteGlyphOffset;
use dwrote::GlyphRunAnalysis as DWriteGlyphRunAnalysis;
use dwrote::InformationalStringId as DWriteInformationalStringId;
use dwrote::OutlineBuilder as DWriteOutlineBuilder;
use dwrote::{DWRITE_TEXTURE_ALIASED_1x1, DWRITE_TEXTURE_CLEARTYPE_3x1};
use dwrote::{DWRITE_GLYPH_RUN, DWRITE_MEASURING_MODE_NATURAL};
use dwrote::{DWRITE_RENDERING_MODE_ALIASED, DWRITE_RENDERING_MODE_NATURAL};
use pathfinder_geometry::line_segment::LineSegment2F;
use pathfinder_geometry::rect::{RectF, RectI};
use pathfinder_geometry::transform2d::Transform2F;
use pathfinder_geometry::vector::{Vector2F, Vector2I};
use std::borrow::Cow;
use std::ffi::OsString;
use std::fmt::{self, Debug, Formatter};
use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};
use std::os::windows::ffi::OsStringExt;
use std::os::windows::io::AsRawHandle;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use winapi::shared::minwindef::{FALSE, MAX_PATH};
use winapi::um::dwrite::DWRITE_NUMBER_SUBSTITUTION_METHOD_NONE;
use winapi::um::dwrite::DWRITE_READING_DIRECTION;
use winapi::um::dwrite::DWRITE_READING_DIRECTION_LEFT_TO_RIGHT;
use winapi::um::fileapi;

use crate::canvas::{Canvas, Format, RasterizationOptions};
use crate::error::{FontLoadingError, GlyphLoadingError};
use crate::file_type::FileType;
use crate::handle::Handle;
use crate::hinting::HintingOptions;
use crate::loader::{FallbackFont, FallbackResult, Loader};
use crate::metrics::Metrics;
use crate::outline::{OutlineBuilder, OutlineSink};
use crate::properties::{Properties, Stretch, Style, Weight};

const ERROR_BOUND: f32 = 0.0001;

const OPENTYPE_TABLE_TAG_HEAD: u32 = 0x68656164;

/// DirectWrite's representation of a font.
#[allow(missing_debug_implementations)]
#[derive(Clone)]
pub struct NativeFont {
    /// The native DirectWrite font object.
    pub dwrite_font: DWriteFont,
    /// The native DirectWrite font face object.
    pub dwrite_font_face: DWriteFontFace,
}

/// A loader that uses the Windows DirectWrite API to load and rasterize fonts.
pub struct Font {
    dwrite_font: DWriteFont,
    dwrite_font_face: DWriteFontFace,
    cached_data: Mutex<Option<Arc<Vec<u8>>>>,
}

struct MyTextAnalysisSource {
    text_utf16_len: u32,
    locale: String,
}

impl dwrote::TextAnalysisSourceMethods for MyTextAnalysisSource {
    fn get_locale_name<'a>(&'a self, text_pos: u32) -> (Cow<'a, str>, u32) {
        (self.locale.as_str().into(), self.text_utf16_len - text_pos)
    }

    fn get_paragraph_reading_direction(&self) -> DWRITE_READING_DIRECTION {
        DWRITE_READING_DIRECTION_LEFT_TO_RIGHT
    }
}

impl Font {
    fn from_dwrite_font_file(
        font_file: DWriteFontFile,
        mut font_index: u32,
        font_data: Option<Arc<Vec<u8>>>,
    ) -> Result<Font, FontLoadingError> {
        let collection_loader = CustomFontCollectionLoaderImpl::new(&[font_file.clone()]);
        let collection = DWriteFontCollection::from_loader(collection_loader);
        let families = collection.families_iter();
        for family in families {
            for family_font_index in 0..family.get_font_count() {
                if font_index > 0 {
                    font_index -= 1;
                    continue;
                }
                let dwrite_font = family.get_font(family_font_index);
                let dwrite_font_face = dwrite_font.create_font_face();
                return Ok(Font {
                    dwrite_font,
                    dwrite_font_face,
                    cached_data: Mutex::new(font_data),
                });
            }
        }
        Err(FontLoadingError::NoSuchFontInCollection)
    }

    /// Loads a font from raw font data (the contents of a `.ttf`/`.otf`/etc. file).
    ///
    /// If the data represents a collection (`.ttc`/`.otc`/etc.), `font_index` specifies the index
    /// of the font to load from it. If the data represents a single font, pass 0 for `font_index`.
    pub fn from_bytes(font_data: Arc<Vec<u8>>, font_index: u32) -> Result<Font, FontLoadingError> {
        let font_file =
            DWriteFontFile::new_from_data(font_data.clone()).ok_or(FontLoadingError::Parse)?;
        Font::from_dwrite_font_file(font_file, font_index, Some(font_data))
    }

    /// Loads a font from a `.ttf`/`.otf`/etc. file.
    ///
    /// If the file is a collection (`.ttc`/`.otc`/etc.), `font_index` specifies the index of the
    /// font to load from it. If the file represents a single font, pass 0 for `font_index`.
    pub fn from_file(file: &mut File, font_index: u32) -> Result<Font, FontLoadingError> {
        unsafe {
            let mut path = vec![0; MAX_PATH + 1];
            let path_len = fileapi::GetFinalPathNameByHandleW(
                file.as_raw_handle(),
                path.as_mut_ptr(),
                path.len() as u32 - 1,
                0,
            );
            if path_len == 0 {
                return Err(FontLoadingError::Io(io::Error::last_os_error()));
            }
            path.truncate(path_len as usize);
            Font::from_path(PathBuf::from(OsString::from_wide(&path)), font_index)
        }
    }

    /// Loads a font from the path to a `.ttf`/`.otf`/etc. file.
    ///
    /// If the file is a collection (`.ttc`/`.otc`/etc.), `font_index` specifies the index of the
    /// font to load from it. If the file represents a single font, pass 0 for `font_index`.
    #[inline]
    pub fn from_path<P: AsRef<Path>>(path: P, font_index: u32) -> Result<Font, FontLoadingError> {
        let font_file = DWriteFontFile::new_from_path(path).ok_or(FontLoadingError::Parse)?;
        Font::from_dwrite_font_file(font_file, font_index, None)
    }

    /// Creates a font from a native API handle.
    #[inline]
    pub unsafe fn from_native_font(native_font: &NativeFont) -> Font {
        let native_font = native_font.clone();
        Font {
            dwrite_font: native_font.dwrite_font,
            dwrite_font_face: native_font.dwrite_font_face,
            cached_data: Mutex::new(None),
        }
    }

    /// Loads the font pointed to by a handle.
    #[inline]
    pub fn from_handle(handle: &Handle) -> Result<Self, FontLoadingError> {
        <Self as Loader>::from_handle(handle)
    }

    /// Determines whether a blob of raw font data represents a supported font, and, if so, what
    /// type of font it is.
    pub fn analyze_bytes(font_data: Arc<Vec<u8>>) -> Result<FileType, FontLoadingError> {
        match DWriteFontFile::analyze_data(font_data) {
            0 => Err(FontLoadingError::Parse),
            1 => Ok(FileType::Single),
            font_count => Ok(FileType::Collection(font_count)),
        }
    }

    /// Determines whether a file represents a supported font, and, if so, what type of font it is.
    pub fn analyze_file(file: &mut File) -> Result<FileType, FontLoadingError> {
        let mut font_data = vec![];
        file.seek(SeekFrom::Start(0))
            .map_err(FontLoadingError::Io)?;
        match file.read_to_end(&mut font_data) {
            Err(io_error) => Err(FontLoadingError::Io(io_error)),
            Ok(_) => Font::analyze_bytes(Arc::new(font_data)),
        }
    }

    /// Returns the wrapped native font handle.
    pub fn native_font(&self) -> NativeFont {
        NativeFont {
            dwrite_font: self.dwrite_font.clone(),
            dwrite_font_face: self.dwrite_font_face.clone(),
        }
    }

    /// Determines whether a path points to a supported font, and, if so, what type of font it is.
    #[inline]
    pub fn analyze_path<P: AsRef<Path>>(path: P) -> Result<FileType, FontLoadingError> {
        <Self as Loader>::analyze_path(path)
    }

    /// Returns the PostScript name of the font. This should be globally unique.
    #[inline]
    pub fn postscript_name(&self) -> Option<String> {
        let dwrite_font = &self.dwrite_font;
        dwrite_font.informational_string(DWriteInformationalStringId::PostscriptName)
    }

    /// Returns the full name of the font (also known as "display name" on macOS).
    #[inline]
    pub fn full_name(&self) -> String {
        let dwrite_font = &self.dwrite_font;
        dwrite_font
            .informational_string(DWriteInformationalStringId::FullName)
            .unwrap_or_else(|| dwrite_font.family_name())
    }

    /// Returns the name of the font family.
    #[inline]
    pub fn family_name(&self) -> String {
        self.dwrite_font.family_name()
    }

    /// Returns true if and only if the font is monospace (fixed-width).
    #[inline]
    pub fn is_monospace(&self) -> bool {
        self.dwrite_font.is_monospace().unwrap_or(false)
    }

    /// Returns the values of various font properties, corresponding to those defined in CSS.
    pub fn properties(&self) -> Properties {
        let dwrite_font = &self.dwrite_font;
        Properties {
            style: style_for_dwrite_style(dwrite_font.style()),
            stretch: Stretch(Stretch::MAPPING[(dwrite_font.stretch() as usize) - 1]),
            weight: Weight(dwrite_font.weight().to_u32() as f32),
        }
    }

    /// Returns the usual glyph ID for a Unicode character.
    ///
    /// Be careful with this function; typographically correct character-to-glyph mapping must be
    /// done using a *shaper* such as HarfBuzz. This function is only useful for best-effort simple
    /// use cases like "what does character X look like on its own".
    pub fn glyph_for_char(&self, character: char) -> Option<u32> {
        let chars = [character as u32];
        self.dwrite_font_face
            .get_glyph_indices(&chars)
            .into_iter()
            .next()
            .and_then(|g| {
                // 0 means the char is not present in the font per
                // https://docs.microsoft.com/en-us/windows/win32/api/dwrite/nf-dwrite-idwritefontface-getglyphindices
                if g != 0 {
                    Some(g as u32)
                } else {
                    None
                }
            })
    }

    /// Returns the number of glyphs in the font.
    ///
    /// Glyph IDs range from 0 inclusive to this value exclusive.
    #[inline]
    pub fn glyph_count(&self) -> u32 {
        self.dwrite_font_face.get_glyph_count() as u32
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
        let outline_sink = OutlineCanonicalizer::new();
        self.dwrite_font_face.get_glyph_run_outline(
            self.metrics().units_per_em as f32,
            &[glyph_id as u16],
            None,
            None,
            false,
            false,
            Box::new(outline_sink.clone()),
        );
        outline_sink
            .0
            .lock()
            .unwrap()
            .builder
            .take_outline()
            .copy_to(&mut *sink);
        Ok(())
    }

    /// Returns the boundaries of a glyph in font units.
    pub fn typographic_bounds(&self, glyph_id: u32) -> Result<RectF, GlyphLoadingError> {
        let metrics = self
            .dwrite_font_face
            .get_design_glyph_metrics(&[glyph_id as u16], false);

        let metrics = &metrics[0];
        let advance_width = metrics.advanceWidth as i32;
        let advance_height = metrics.advanceHeight as i32;
        let left_side_bearing = metrics.leftSideBearing as i32;
        let right_side_bearing = metrics.rightSideBearing as i32;
        let top_side_bearing = metrics.topSideBearing as i32;
        let bottom_side_bearing = metrics.bottomSideBearing as i32;
        let vertical_origin_y = metrics.verticalOriginY as i32;

        let y_offset = vertical_origin_y + bottom_side_bearing - advance_height;
        let width = advance_width - (left_side_bearing + right_side_bearing);
        let height = advance_height - (top_side_bearing + bottom_side_bearing);

        Ok(RectI::new(
            Vector2I::new(left_side_bearing, y_offset),
            Vector2I::new(width, height),
        )
        .to_f32())
    }

    /// Returns the distance from the origin of the glyph with the given ID to the next, in font
    /// units.
    pub fn advance(&self, glyph_id: u32) -> Result<Vector2F, GlyphLoadingError> {
        let metrics = self
            .dwrite_font_face
            .get_design_glyph_metrics(&[glyph_id as u16], false);
        let metrics = &metrics[0];
        Ok(Vector2F::new(metrics.advanceWidth as f32, 0.0))
    }

    /// Returns the amount that the given glyph should be displaced from the origin.
    pub fn origin(&self, glyph: u32) -> Result<Vector2F, GlyphLoadingError> {
        let metrics = self
            .dwrite_font_face
            .get_design_glyph_metrics(&[glyph as u16], false);
        Ok(Vector2I::new(
            metrics[0].leftSideBearing,
            metrics[0].verticalOriginY + metrics[0].bottomSideBearing,
        )
        .to_f32())
    }

    /// Retrieves various metrics that apply to the entire font.
    pub fn metrics(&self) -> Metrics {
        let dwrite_font = &self.dwrite_font;

        // Unfortunately, the bounding box info is Windows 8 only, so we need a fallback. First,
        // try to grab it from the font. If that fails, we try the `head` table. If there's no
        // `head` table, we give up.
        match dwrite_font.metrics() {
            DWriteFontMetrics::Metrics1(metrics) => Metrics {
                units_per_em: metrics.designUnitsPerEm as u32,
                ascent: metrics.ascent as f32,
                descent: -(metrics.descent as f32),
                line_gap: metrics.lineGap as f32,
                cap_height: metrics.capHeight as f32,
                x_height: metrics.xHeight as f32,
                underline_position: metrics.underlinePosition as f32,
                underline_thickness: metrics.underlineThickness as f32,
                bounding_box: RectI::new(
                    Vector2I::new(metrics.glyphBoxLeft as i32, metrics.glyphBoxBottom as i32),
                    Vector2I::new(
                        metrics.glyphBoxRight as i32 - metrics.glyphBoxLeft as i32,
                        metrics.glyphBoxTop as i32 - metrics.glyphBoxBottom as i32,
                    ),
                )
                .to_f32(),
            },
            DWriteFontMetrics::Metrics0(metrics) => {
                let bounding_box = match self
                    .dwrite_font_face
                    .get_font_table(OPENTYPE_TABLE_TAG_HEAD.swap_bytes())
                {
                    Some(head) => {
                        let mut reader = &head[36..];
                        let x_min = reader.read_i16::<BigEndian>().unwrap();
                        let y_min = reader.read_i16::<BigEndian>().unwrap();
                        let x_max = reader.read_i16::<BigEndian>().unwrap();
                        let y_max = reader.read_i16::<BigEndian>().unwrap();
                        RectI::new(
                            Vector2I::new(x_min as i32, y_min as i32),
                            Vector2I::new(x_max as i32 - x_min as i32, y_max as i32 - y_min as i32),
                        )
                        .to_f32()
                    }
                    None => RectF::default(),
                };
                Metrics {
                    units_per_em: metrics.designUnitsPerEm as u32,
                    ascent: metrics.ascent as f32,
                    descent: -(metrics.descent as f32),
                    line_gap: metrics.lineGap as f32,
                    cap_height: metrics.capHeight as f32,
                    x_height: metrics.xHeight as f32,
                    underline_position: metrics.underlinePosition as f32,
                    underline_thickness: metrics.underlineThickness as f32,
                    bounding_box,
                }
            }
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
        let mut font_data = self.cached_data.lock().unwrap();
        if font_data.is_none() {
            let files = self.dwrite_font_face.get_files();
            // FIXME(pcwalton): Is this right? When can a font have multiple files?
            if let Some(file) = files.get(0) {
                *font_data = Some(Arc::new(file.get_font_file_bytes()))
            }
        }
        (*font_data).clone()
    }

    /// Returns the pixel boundaries that the glyph will take up when rendered using this loader's
    /// rasterizer at the given size and origin.
    #[inline]
    pub fn raster_bounds(
        &self,
        glyph_id: u32,
        point_size: f32,
        transform: Transform2F,
        hinting_options: HintingOptions,
        rasterization_options: RasterizationOptions,
    ) -> Result<RectI, GlyphLoadingError> {
        let dwrite_analysis = self.build_glyph_analysis(
            glyph_id,
            point_size,
            transform,
            hinting_options,
            rasterization_options,
        )?;

        let texture_type = match rasterization_options {
            RasterizationOptions::Bilevel => DWRITE_TEXTURE_ALIASED_1x1,
            RasterizationOptions::GrayscaleAa | RasterizationOptions::SubpixelAa => {
                DWRITE_TEXTURE_CLEARTYPE_3x1
            }
        };

        let texture_bounds = dwrite_analysis.get_alpha_texture_bounds(texture_type)?;
        let texture_width = texture_bounds.right - texture_bounds.left;
        let texture_height = texture_bounds.bottom - texture_bounds.top;

        Ok(RectI::new(
            Vector2I::new(texture_bounds.left, texture_bounds.top),
            Vector2I::new(texture_width, texture_height),
        ))
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
    pub fn rasterize_glyph(
        &self,
        canvas: &mut Canvas,
        glyph_id: u32,
        point_size: f32,
        transform: Transform2F,
        hinting_options: HintingOptions,
        rasterization_options: RasterizationOptions,
    ) -> Result<(), GlyphLoadingError> {
        // TODO(pcwalton): This is woefully incomplete. See WebRender's code for a more complete
        // implementation.

        let dwrite_analysis = self.build_glyph_analysis(
            glyph_id,
            point_size,
            transform,
            hinting_options,
            rasterization_options,
        )?;

        let texture_type = match rasterization_options {
            RasterizationOptions::Bilevel => DWRITE_TEXTURE_ALIASED_1x1,
            RasterizationOptions::GrayscaleAa | RasterizationOptions::SubpixelAa => {
                DWRITE_TEXTURE_CLEARTYPE_3x1
            }
        };

        // TODO(pcwalton): Avoid a copy in some cases by writing directly to the canvas.
        let texture_bounds = dwrite_analysis.get_alpha_texture_bounds(texture_type)?;
        let texture_width = texture_bounds.right - texture_bounds.left;
        let texture_height = texture_bounds.bottom - texture_bounds.top;

        // 'Returns an empty rectangle if there are no glyphs of the specified texture type.'
        // https://docs.microsoft.com/en-us/windows/win32/api/dwrite/nf-dwrite-idwriteglyphrunanalysis-getalphatexturebounds
        if texture_width == 0 || texture_height == 0 {
            return Ok(());
        }

        let texture_format = if texture_type == DWRITE_TEXTURE_ALIASED_1x1 {
            Format::A8
        } else {
            Format::Rgb24
        };
        let texture_bits_per_pixel = texture_format.bits_per_pixel();
        let texture_bytes_per_pixel = texture_bits_per_pixel as usize / 8;
        let texture_size = Vector2I::new(texture_width, texture_height);
        let texture_stride = texture_width as usize * texture_bytes_per_pixel;

        let mut texture_bytes =
            dwrite_analysis.create_alpha_texture(texture_type, texture_bounds)?;
        canvas.blit_from(
            Vector2I::new(texture_bounds.left, texture_bounds.top),
            &mut texture_bytes,
            texture_size,
            texture_stride,
            texture_format,
        );

        Ok(())
    }

    /// Returns true if and only if the font loader can perform hinting in the requested way.
    ///
    /// Some APIs support only rasterizing glyphs with hinting, not retrieving hinted outlines. If
    /// `for_rasterization` is false, this function returns true if and only if the loader supports
    /// retrieval of hinted *outlines*. If `for_rasterization` is true, this function returns true
    /// if and only if the loader supports *rasterizing* hinted glyphs.
    pub fn supports_hinting_options(
        &self,
        hinting_options: HintingOptions,
        for_rasterization: bool,
    ) -> bool {
        match (hinting_options, for_rasterization) {
            (HintingOptions::None, _)
            | (HintingOptions::Vertical(_), true)
            | (HintingOptions::VerticalSubpixel(_), true) => true,
            (HintingOptions::Vertical(_), false)
            | (HintingOptions::VerticalSubpixel(_), false)
            | (HintingOptions::Full(_), _) => false,
        }
    }

    fn build_glyph_analysis(
        &self,
        glyph_id: u32,
        point_size: f32,
        transform: Transform2F,
        _hinting_options: HintingOptions,
        rasterization_options: RasterizationOptions,
    ) -> Result<DWriteGlyphRunAnalysis, GlyphLoadingError> {
        unsafe {
            let glyph_id = glyph_id as u16;
            let advance = 0.0;
            let offset = DWriteGlyphOffset {
                advanceOffset: 0.0,
                ascenderOffset: 0.0,
            };
            let glyph_run = DWRITE_GLYPH_RUN {
                fontFace: self.dwrite_font_face.as_ptr(),
                fontEmSize: point_size,
                glyphCount: 1,
                glyphIndices: &glyph_id,
                glyphAdvances: &advance,
                glyphOffsets: &offset,
                isSideways: FALSE,
                bidiLevel: 0,
            };

            let rendering_mode = match rasterization_options {
                RasterizationOptions::Bilevel => DWRITE_RENDERING_MODE_ALIASED,
                RasterizationOptions::GrayscaleAa | RasterizationOptions::SubpixelAa => {
                    DWRITE_RENDERING_MODE_NATURAL
                }
            };

            Ok(DWriteGlyphRunAnalysis::create(
                &glyph_run,
                1.0,
                Some(dwrote::DWRITE_MATRIX {
                    m11: transform.m11(),
                    m12: transform.m12(),
                    m21: transform.m21(),
                    m22: transform.m22(),
                    dx: transform.vector.x(),
                    dy: transform.vector.y(),
                }),
                rendering_mode,
                DWRITE_MEASURING_MODE_NATURAL,
                0.0,
                0.0,
            )?)
        }
    }

    /// Get font fallback results for the given text and locale.
    ///
    /// The `locale` argument is a language tag such as `"en-US"` or `"zh-Hans-CN"`.
    ///
    /// Note: on Windows 10, the result is a single font.
    fn get_fallbacks(&self, text: &str, locale: &str) -> FallbackResult<Font> {
        let sys_fallback = DWriteFontFallback::get_system_fallback();
        if sys_fallback.is_none() {
            unimplemented!("Need Windows 7 method for font fallbacks")
        }
        let text_utf16: Vec<u16> = text.encode_utf16().collect();
        let text_utf16_len = text_utf16.len() as u32;
        let number_subst =
            dwrote::NumberSubstitution::new(DWRITE_NUMBER_SUBSTITUTION_METHOD_NONE, locale, true);
        let text_analysis_source = MyTextAnalysisSource {
            text_utf16_len,
            locale: locale.to_owned(),
        };
        let text_analysis = dwrote::TextAnalysisSource::from_text_and_number_subst(
            Box::new(text_analysis_source),
            text_utf16.into(),
            number_subst,
        );
        let sys_fallback = sys_fallback.unwrap();
        // TODO: I think the MapCharacters can take a null pointer, update
        // dwrote to accept an optional collection. This appears to be what
        // blink does.
        let collection = DWriteFontCollection::get_system(false);
        let fallback_result = sys_fallback.map_characters(
            &text_analysis,
            0,
            text_utf16_len,
            &collection,
            Some(&self.dwrite_font.family_name()),
            self.dwrite_font.weight(),
            self.dwrite_font.style(),
            self.dwrite_font.stretch(),
        );
        let valid_len = convert_len_utf16_to_utf8(text, fallback_result.mapped_length);
        let fonts = if let Some(dwrite_font) = fallback_result.mapped_font {
            let dwrite_font_face = dwrite_font.create_font_face();
            let font = Font {
                dwrite_font,
                dwrite_font_face,
                cached_data: Mutex::new(None),
            };
            let fallback_font = FallbackFont {
                font,
                scale: fallback_result.scale,
            };
            vec![fallback_font]
        } else {
            vec![]
        };
        FallbackResult { fonts, valid_len }
    }

    /// Returns the raw contents of the OpenType table with the given tag.
    ///
    /// Tags are four-character codes. A list of tags can be found in the [OpenType specification].
    ///
    /// [OpenType specification]: https://docs.microsoft.com/en-us/typography/opentype/spec/
    pub fn load_font_table(&self, table_tag: u32) -> Option<Box<[u8]>> {
        self.dwrite_font_face
            .get_font_table(table_tag.swap_bytes())
            .map(|v| v.into())
    }
}

// There might well be a more efficient impl that doesn't fully decode the text,
// just looks at the utf-8 bytes.
fn convert_len_utf16_to_utf8(text: &str, len_utf16: usize) -> usize {
    let mut l_utf8 = 0;
    let mut l_utf16 = 0;
    let mut chars = text.chars();
    while l_utf16 < len_utf16 {
        if let Some(c) = chars.next() {
            l_utf8 += c.len_utf8();
            l_utf16 += c.len_utf16();
        } else {
            break;
        }
    }
    l_utf8
}

impl Clone for Font {
    #[inline]
    fn clone(&self) -> Font {
        Font {
            dwrite_font: self.dwrite_font.clone(),
            dwrite_font_face: self.dwrite_font_face.clone(),
            cached_data: Mutex::new((*self.cached_data.lock().unwrap()).clone()),
        }
    }
}

impl Debug for Font {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), fmt::Error> {
        self.family_name().fmt(fmt)
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

    fn from_path<P>(path: P, font_index: u32) -> Result<Self, FontLoadingError>
    where
        P: AsRef<Path>,
    {
        Font::from_path(path, font_index)
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
    fn glyph_count(&self) -> u32 {
        self.glyph_count()
    }

    #[inline]
    fn outline<S>(
        &self,
        glyph_id: u32,
        hinting: HintingOptions,
        sink: &mut S,
    ) -> Result<(), GlyphLoadingError>
    where
        S: OutlineSink,
    {
        self.outline(glyph_id, hinting, sink)
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
    fn origin(&self, origin: u32) -> Result<Vector2F, GlyphLoadingError> {
        self.origin(origin)
    }

    #[inline]
    fn metrics(&self) -> Metrics {
        self.metrics()
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
    fn copy_font_data(&self) -> Option<Arc<Vec<u8>>> {
        self.copy_font_data()
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

#[derive(Clone)]
struct OutlineCanonicalizer(Arc<Mutex<OutlineCanonicalizerInfo>>);

struct OutlineCanonicalizerInfo {
    builder: OutlineBuilder,
    last_position: Vector2F,
}

impl OutlineCanonicalizer {
    fn new() -> OutlineCanonicalizer {
        OutlineCanonicalizer(Arc::new(Mutex::new(OutlineCanonicalizerInfo {
            builder: OutlineBuilder::new(),
            last_position: Vector2F::default(),
        })))
    }
}

impl DWriteOutlineBuilder for OutlineCanonicalizer {
    fn move_to(&mut self, to_x: f32, to_y: f32) {
        let to = Vector2F::new(to_x, -to_y);

        let mut this = self.0.lock().unwrap();
        this.last_position = to;
        this.builder.move_to(to);
    }

    fn line_to(&mut self, to_x: f32, to_y: f32) {
        let to = Vector2F::new(to_x, -to_y);

        let mut this = self.0.lock().unwrap();
        this.last_position = to;
        this.builder.line_to(to);
    }

    fn close(&mut self) {
        let mut this = self.0.lock().unwrap();
        this.builder.close();
    }

    fn curve_to(
        &mut self,
        ctrl0_x: f32,
        ctrl0_y: f32,
        ctrl1_x: f32,
        ctrl1_y: f32,
        to_x: f32,
        to_y: f32,
    ) {
        let ctrl = LineSegment2F::new(
            Vector2F::new(ctrl0_x, -ctrl0_y),
            Vector2F::new(ctrl1_x, -ctrl1_y),
        );
        let to = Vector2F::new(to_x, -to_y);

        // This might be a degree-elevated quadratic curve. Try to detect that.
        // See Sederberg § 2.6, "Distance Between Two Bézier Curves".
        let mut this = self.0.lock().unwrap();
        let baseline = LineSegment2F::new(this.last_position, to);
        let approx_ctrl = LineSegment2F((ctrl * 3.0).0 - baseline.0) * 0.5;
        let delta_ctrl = (approx_ctrl.to() - approx_ctrl.from()) * 2.0;
        let max_error = delta_ctrl.length() / 6.0;

        if max_error < ERROR_BOUND {
            // Round to nearest 0.5.
            let approx_ctrl = (approx_ctrl.midpoint() * 2.0).round() * 0.5;
            this.builder.quadratic_curve_to(approx_ctrl, to);
        } else {
            this.builder.cubic_curve_to(ctrl, to);
        }

        this.last_position = to;
    }
}

fn style_for_dwrite_style(style: DWriteFontStyle) -> Style {
    match style {
        DWriteFontStyle::Normal => Style::Normal,
        DWriteFontStyle::Oblique => Style::Oblique,
        DWriteFontStyle::Italic => Style::Italic,
    }
}
