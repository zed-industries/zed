//! Support for applying embedded hinting instructions.

use super::{
    autohint, cff,
    glyf::{self, FreeTypeScaler},
    pen::PathStyle,
    AdjustedMetrics, DrawError, GlyphStyles, Hinting, LocationRef, NormalizedCoord,
    OutlineCollectionKind, OutlineGlyph, OutlineGlyphCollection, OutlineKind, OutlinePen, Size,
};
use crate::alloc::{boxed::Box, vec::Vec};
use raw::types::Fixed;

/// Configuration settings for a hinting instance.
#[derive(Clone, Default, Debug)]
pub struct HintingOptions {
    /// Specifies the hinting engine to use.
    ///
    /// Defaults to [`Engine::AutoFallback`].
    pub engine: Engine,
    /// Defines the properties of the intended target of a hinted outline.
    ///
    /// Defaults to a target with [`SmoothMode::Normal`] which is equivalent
    /// to `FT_RENDER_MODE_NORMAL` in FreeType.
    pub target: Target,
}

impl From<Target> for HintingOptions {
    fn from(value: Target) -> Self {
        Self {
            engine: Engine::AutoFallback,
            target: value,
        }
    }
}

/// Specifies the backend to use when applying hints.
#[derive(Clone, Default, Debug)]
pub enum Engine {
    /// The TrueType or PostScript interpreter.
    Interpreter,
    /// The automatic hinter that performs just-in-time adjustment of
    /// outlines.
    ///
    /// Glyph styles can be precomputed per font and may be provided here
    /// as an optimization to avoid recomputing them for each instance.
    Auto(Option<GlyphStyles>),
    /// Selects the engine based on the same rules that FreeType uses when
    /// neither of the `FT_LOAD_NO_AUTOHINT` or `FT_LOAD_FORCE_AUTOHINT`
    /// load flags are specified.
    ///
    /// Specifically, PostScript (CFF/CFF2) fonts will always use the hinting
    /// engine in the PostScript interpreter and TrueType fonts will use the
    /// interpreter for TrueType instructions if one of the `fpgm` or `prep`
    /// tables is non-empty, falling back to the automatic hinter otherwise.
    ///
    /// This uses [`OutlineGlyphCollection::prefer_interpreter`] to make a
    /// selection.
    #[default]
    AutoFallback,
}

impl Engine {
    /// Converts the `AutoFallback` variant into either `Interpreter` or
    /// `Auto` based on the given outline set's preference for interpreter
    /// mode.
    fn resolve_auto_fallback(self, outlines: &OutlineGlyphCollection) -> Engine {
        match self {
            Self::Interpreter => Self::Interpreter,
            Self::Auto(styles) => Self::Auto(styles),
            Self::AutoFallback => {
                if outlines.prefer_interpreter() {
                    Self::Interpreter
                } else {
                    Self::Auto(None)
                }
            }
        }
    }
}

impl From<Engine> for HintingOptions {
    fn from(value: Engine) -> Self {
        Self {
            engine: value,
            target: Default::default(),
        }
    }
}

/// Defines the target settings for hinting.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Target {
    /// Strong hinting style that should only be used for aliased, monochromatic
    /// rasterization.
    ///
    /// Corresponds to `FT_LOAD_TARGET_MONO` in FreeType.
    Mono,
    /// Hinting style that is suitable for anti-aliased rasterization.
    ///
    /// Corresponds to the non-monochrome load targets in FreeType. See
    /// [`SmoothMode`] for more detail.
    Smooth {
        /// The basic mode for smooth hinting.
        ///
        /// Defaults to [`SmoothMode::Normal`].
        mode: SmoothMode,
        /// If true, TrueType bytecode may assume that the resulting outline
        /// will be rasterized with supersampling in the vertical direction.
        ///
        /// When this is enabled, ClearType fonts will often generate wider
        /// horizontal stems that may lead to blurry images when rendered with
        /// an analytical area rasterizer (such as the one in FreeType).
        ///
        /// The effect of this setting is to control the "ClearType symmetric
        /// rendering bit" of the TrueType `GETINFO` instruction. For more
        /// detail, see this [issue](https://github.com/googlefonts/fontations/issues/1080).
        ///
        /// FreeType has no corresponding setting and behaves as if this is
        /// always enabled.
        ///
        /// This only applies to the TrueType interpreter.
        ///
        /// Defaults to `true`.
        symmetric_rendering: bool,
        /// If true, prevents adjustment of the outline in the horizontal
        /// direction and preserves inter-glyph spacing.
        ///
        /// This is useful for performing layout without concern that hinting
        /// will modify the advance width of a glyph. Specifically, it means
        /// that layout will not require evaluation of glyph outlines.
        ///
        /// FreeType has no corresponding setting and behaves as if this is
        /// always disabled.
        ///
        /// This applies to the TrueType interpreter and the automatic hinter.
        ///
        /// Defaults to `false`.       
        preserve_linear_metrics: bool,
    },
}

impl Default for Target {
    fn default() -> Self {
        SmoothMode::Normal.into()
    }
}

/// Mode selector for a smooth hinting target.
#[derive(Copy, Clone, PartialEq, Eq, Default, Debug)]
pub enum SmoothMode {
    /// The standard smooth hinting mode.
    ///
    /// Corresponds to `FT_LOAD_TARGET_NORMAL` in FreeType.
    #[default]
    Normal,
    /// Hinting with a lighter touch, typically meaning less aggressive
    /// adjustment in the horizontal direction.
    ///
    /// Corresponds to `FT_LOAD_TARGET_LIGHT` in FreeType.
    Light,
    /// Hinting that is optimized for subpixel rendering with horizontal LCD
    /// layouts.
    ///
    /// Corresponds to `FT_LOAD_TARGET_LCD` in FreeType.
    Lcd,
    /// Hinting that is optimized for subpixel rendering with vertical LCD
    /// layouts.
    ///
    /// Corresponds to `FT_LOAD_TARGET_LCD_V` in FreeType.
    VerticalLcd,
}

impl From<SmoothMode> for Target {
    fn from(value: SmoothMode) -> Self {
        Self::Smooth {
            mode: value,
            symmetric_rendering: true,
            preserve_linear_metrics: false,
        }
    }
}

/// Modes that control hinting when using embedded instructions.
///
/// Only the TrueType interpreter supports all hinting modes.
///
/// # FreeType compatibility
///
/// The following table describes how to map FreeType hinting modes:
///
/// | FreeType mode         | Variant                                                                              |
/// |-----------------------|--------------------------------------------------------------------------------------|
/// | FT_LOAD_TARGET_MONO   | Strong                                                                               |
/// | FT_LOAD_TARGET_NORMAL | Smooth { lcd_subpixel: None, preserve_linear_metrics: false }                        |
/// | FT_LOAD_TARGET_LCD    | Smooth { lcd_subpixel: Some(LcdLayout::Horizontal), preserve_linear_metrics: false } |
/// | FT_LOAD_TARGET_LCD_V  | Smooth { lcd_subpixel: Some(LcdLayout::Vertical), preserve_linear_metrics: false }   |
///
/// Note: `FT_LOAD_TARGET_LIGHT` is equivalent to `FT_LOAD_TARGET_NORMAL` since
/// FreeType 2.7.
///
/// The default value of this type is equivalent to `FT_LOAD_TARGET_NORMAL`.
#[doc(hidden)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum HintingMode {
    /// Strong hinting mode that should only be used for aliased, monochromatic
    /// rasterization.
    ///
    /// Corresponds to `FT_LOAD_TARGET_MONO` in FreeType.
    Strong,
    /// Lighter hinting mode that is intended for anti-aliased rasterization.
    Smooth {
        /// If set, enables support for optimized hinting that takes advantage
        /// of subpixel layouts in LCD displays and corresponds to
        /// `FT_LOAD_TARGET_LCD` or `FT_LOAD_TARGET_LCD_V` in FreeType.
        ///
        /// If unset, corresponds to `FT_LOAD_TARGET_NORMAL` in FreeType.
        lcd_subpixel: Option<LcdLayout>,
        /// If true, prevents adjustment of the outline in the horizontal
        /// direction and preserves inter-glyph spacing.
        ///
        /// This is useful for performing layout without concern that hinting
        /// will modify the advance width of a glyph. Specifically, it means
        /// that layout will not require evaluation of glyph outlines.
        ///
        /// FreeType has no corresponding setting.
        preserve_linear_metrics: bool,
    },
}

impl Default for HintingMode {
    fn default() -> Self {
        Self::Smooth {
            lcd_subpixel: None,
            preserve_linear_metrics: false,
        }
    }
}

impl From<HintingMode> for HintingOptions {
    fn from(value: HintingMode) -> Self {
        let target = match value {
            HintingMode::Strong => Target::Mono,
            HintingMode::Smooth {
                lcd_subpixel,
                preserve_linear_metrics,
            } => {
                let mode = match lcd_subpixel {
                    Some(LcdLayout::Horizontal) => SmoothMode::Lcd,
                    Some(LcdLayout::Vertical) => SmoothMode::VerticalLcd,
                    None => SmoothMode::Normal,
                };
                Target::Smooth {
                    mode,
                    preserve_linear_metrics,
                    symmetric_rendering: true,
                }
            }
        };
        target.into()
    }
}

/// Specifies direction of pixel layout for LCD based subpixel hinting.
#[doc(hidden)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum LcdLayout {
    /// Subpixels are ordered horizontally.
    ///
    /// Corresponds to `FT_LOAD_TARGET_LCD` in FreeType.
    Horizontal,
    /// Subpixels are ordered vertically.
    ///
    /// Corresponds to `FT_LOAD_TARGET_LCD_V` in FreeType.
    Vertical,
}

/// Hinting instance that uses information embedded in the font to perform
/// grid-fitting.
#[derive(Clone)]
pub struct HintingInstance {
    size: Size,
    coords: Vec<NormalizedCoord>,
    target: Target,
    kind: HinterKind,
}

impl HintingInstance {
    /// Creates a new embedded hinting instance for the given outline
    /// collection, size, location in variation space and hinting mode.
    pub fn new<'a>(
        outline_glyphs: &OutlineGlyphCollection,
        size: Size,
        location: impl Into<LocationRef<'a>>,
        options: impl Into<HintingOptions>,
    ) -> Result<Self, DrawError> {
        let options = options.into();
        let mut hinter = Self {
            size: Size::unscaled(),
            coords: vec![],
            target: options.target,
            kind: HinterKind::None,
        };
        hinter.reconfigure(outline_glyphs, size, location, options)?;
        Ok(hinter)
    }

    /// Returns the currently configured size.
    pub fn size(&self) -> Size {
        self.size
    }

    /// Returns the currently configured normalized location in variation space.
    pub fn location(&self) -> LocationRef<'_> {
        LocationRef::new(&self.coords)
    }

    /// Returns the currently configured hinting target.
    pub fn target(&self) -> Target {
        self.target
    }

    /// Resets the hinter state for a new font instance with the given
    /// outline collection and settings.
    pub fn reconfigure<'a>(
        &mut self,
        outlines: &OutlineGlyphCollection,
        size: Size,
        location: impl Into<LocationRef<'a>>,
        options: impl Into<HintingOptions>,
    ) -> Result<(), DrawError> {
        self.size = size;
        self.coords.clear();
        self.coords
            .extend_from_slice(location.into().effective_coords());
        let options = options.into();
        self.target = options.target;
        let engine = options.engine.resolve_auto_fallback(outlines);
        // Reuse memory if the font contains the same outline format
        let current_kind = core::mem::replace(&mut self.kind, HinterKind::None);
        match engine {
            Engine::Interpreter => match &outlines.kind {
                OutlineCollectionKind::Glyf(glyf) => {
                    let mut hint_instance = match current_kind {
                        HinterKind::Glyf(instance) => instance,
                        _ => Box::<glyf::HintInstance>::default(),
                    };
                    let ppem = size.ppem();
                    let scale = glyf.compute_hinted_scale(ppem).1.to_bits();
                    // Use fixed point rounding for ppem to match what FreeType does:
                    // <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/base/ftobjs.c#L3349>
                    // issue: <https://github.com/googlefonts/fontations/issues/1544>
                    let rounded_ppem = ((Fixed::from_bits(scale)
                        * Fixed::from_bits(glyf.units_per_em() as i32))
                    .to_bits()
                        + 32)
                        >> 6;
                    hint_instance.reconfigure(
                        glyf,
                        scale,
                        rounded_ppem,
                        self.target,
                        &self.coords,
                    )?;
                    self.kind = HinterKind::Glyf(hint_instance);
                }
                OutlineCollectionKind::Cff(cff) => {
                    let mut subfonts = match current_kind {
                        HinterKind::Cff(subfonts) => subfonts,
                        _ => vec![],
                    };
                    subfonts.clear();
                    let ppem = size.ppem();
                    for i in 0..cff.subfont_count() {
                        subfonts.push(cff.subfont(i, ppem, &self.coords)?);
                    }
                    self.kind = HinterKind::Cff(subfonts);
                }
                OutlineCollectionKind::None => {}
            },
            Engine::Auto(styles) => {
                let Some(font) = outlines.font() else {
                    return Ok(());
                };
                let instance = autohint::Instance::new(
                    font,
                    outlines,
                    &self.coords,
                    self.target,
                    styles,
                    true,
                );
                self.kind = HinterKind::Auto(instance);
            }
            _ => {}
        }
        Ok(())
    }

    /// Returns true if hinting should actually be applied for this instance.
    ///
    /// Some TrueType fonts disable hinting dynamically based on the instance
    /// configuration.
    pub fn is_enabled(&self) -> bool {
        match &self.kind {
            HinterKind::Glyf(instance) => instance.is_enabled(),
            HinterKind::Cff(_) | HinterKind::Auto(_) => true,
            _ => false,
        }
    }

    pub(super) fn draw(
        &self,
        glyph: &OutlineGlyph,
        memory: Option<&mut [u8]>,
        path_style: PathStyle,
        pen: &mut impl OutlinePen,
        is_pedantic: bool,
    ) -> Result<AdjustedMetrics, DrawError> {
        let ppem = self.size.ppem();
        let coords = self.coords.as_slice();
        match (&self.kind, &glyph.kind) {
            (HinterKind::Auto(instance), _) => {
                instance.draw(self.size, coords, glyph, path_style, pen)
            }
            (HinterKind::Glyf(instance), OutlineKind::Glyf(glyf, outline)) => {
                if matches!(path_style, PathStyle::HarfBuzz) {
                    return Err(DrawError::HarfBuzzHintingUnsupported);
                }
                super::with_glyf_memory(outline, Hinting::Embedded, memory, |buf| {
                    let scaled_outline = FreeTypeScaler::hinted(
                        glyf,
                        outline,
                        buf,
                        ppem,
                        coords,
                        instance,
                        is_pedantic,
                    )?
                    .scale(&outline.glyph, outline.glyph_id)?;
                    scaled_outline.to_path(path_style, pen)?;
                    Ok(AdjustedMetrics {
                        has_overlaps: outline.has_overlaps,
                        lsb: Some(scaled_outline.adjusted_lsb().to_f32()),
                        // When hinting is requested, we round the advance
                        // <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/base/ftobjs.c#L889>
                        advance_width: Some(
                            scaled_outline.adjusted_advance_width().round().to_f32(),
                        ),
                    })
                })
            }
            (HinterKind::Cff(subfonts), OutlineKind::Cff(cff, glyph_id, subfont_ix)) => {
                let Some(subfont) = subfonts.get(*subfont_ix as usize) else {
                    return Err(DrawError::NoSources);
                };
                cff.draw(subfont, *glyph_id, &self.coords, true, pen)?;
                Ok(AdjustedMetrics::default())
            }
            _ => Err(DrawError::NoSources),
        }
    }
}

#[derive(Clone)]
enum HinterKind {
    /// Represents a hinting instance that is associated with an empty outline
    /// collection.
    None,
    Glyf(Box<glyf::HintInstance>),
    Cff(Vec<cff::Subfont>),
    Auto(autohint::Instance),
}

// Internal helpers for deriving various flags from the mode which
// change the behavior of certain instructions.
// See <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttgload.c#L2222>
impl Target {
    pub(crate) fn is_smooth(&self) -> bool {
        matches!(self, Self::Smooth { .. })
    }

    pub(crate) fn is_grayscale_cleartype(&self) -> bool {
        match self {
            Self::Smooth { mode, .. } => matches!(mode, SmoothMode::Normal | SmoothMode::Light),
            _ => false,
        }
    }

    pub(crate) fn is_light(&self) -> bool {
        matches!(
            self,
            Self::Smooth {
                mode: SmoothMode::Light,
                ..
            }
        )
    }

    pub(crate) fn is_lcd(&self) -> bool {
        matches!(
            self,
            Self::Smooth {
                mode: SmoothMode::Lcd,
                ..
            }
        )
    }

    pub(crate) fn is_vertical_lcd(&self) -> bool {
        matches!(
            self,
            Self::Smooth {
                mode: SmoothMode::VerticalLcd,
                ..
            }
        )
    }

    pub(crate) fn symmetric_rendering(&self) -> bool {
        matches!(
            self,
            Self::Smooth {
                symmetric_rendering: true,
                ..
            }
        )
    }

    pub(crate) fn preserve_linear_metrics(&self) -> bool {
        matches!(
            self,
            Self::Smooth {
                preserve_linear_metrics: true,
                ..
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        outline::{
            pen::{NullPen, SvgPen},
            DrawSettings,
        },
        raw::TableProvider,
        FontRef, GlyphId, MetadataProvider,
    };

    // FreeType ignores the hdmx table when backward compatibility mode
    // is enabled in the TrueType interpreter.
    #[test]
    fn ignore_hdmx_when_back_compat_enabled() {
        let font = FontRef::new(font_test_data::TINOS_SUBSET).unwrap();
        let outlines = font.outline_glyphs();
        // Double quote was the most egregious failure
        let gid = font.charmap().map('"').unwrap();
        let font_size = 16;
        let hinter = HintingInstance::new(
            &outlines,
            Size::new(font_size as f32),
            LocationRef::default(),
            HintingOptions::default(),
        )
        .unwrap();
        let HinterKind::Glyf(tt_hinter) = &hinter.kind else {
            panic!("this is definitely a TrueType hinter");
        };
        // Make sure backward compatibility mode is enabled
        assert!(tt_hinter.backward_compatibility());
        let outline = outlines.get(gid).unwrap();
        let metrics = outline.draw(&hinter, &mut NullPen).unwrap();
        // FreeType computes an advance width of 7 when hinting but hdmx contains 5
        let scaler_advance = metrics.advance_width.unwrap();
        assert_eq!(scaler_advance, 7.0);
        let hdmx_advance = font
            .hdmx()
            .unwrap()
            .record_for_size(font_size)
            .unwrap()
            .widths()[gid.to_u32() as usize];
        assert_eq!(hdmx_advance, 5);
    }

    // When hinting is disabled by the prep table, FreeType still returns
    // rounded advance widths
    #[test]
    fn round_advance_when_prep_disables_hinting() {
        let font = FontRef::new(font_test_data::TINOS_SUBSET).unwrap();
        let outlines = font.outline_glyphs();
        let gid = font.charmap().map('"').unwrap();
        let size = Size::new(16.0);
        let location = LocationRef::default();
        let mut hinter =
            HintingInstance::new(&outlines, size, location, HintingOptions::default()).unwrap();
        let HinterKind::Glyf(tt_hinter) = &mut hinter.kind else {
            panic!("this is definitely a TrueType hinter");
        };
        tt_hinter.simulate_prep_flag_suppress_hinting();
        let outline = outlines.get(gid).unwrap();
        // And we still have a rounded advance
        let metrics = outline.draw(&hinter, &mut NullPen).unwrap();
        assert_eq!(metrics.advance_width, Some(7.0));
        // Unhinted advance has some fractional bits
        let metrics = outline
            .draw(DrawSettings::unhinted(size, location), &mut NullPen)
            .unwrap();
        assert_eq!(metrics.advance_width, Some(6.53125));
    }

    // Check that we round the value for the MPPEM instruction when applying
    // a fractional font size
    // <https://github.com/googlefonts/fontations/issues/1544>
    #[test]
    fn hint_fractional_font_size() {
        let font = FontRef::new(font_test_data::COUSINE_HINT_SUBSET).unwrap();
        let outlines = font.outline_glyphs();
        let gid = GlyphId::new(1); // was 85 in the original font
        let size = Size::new(24.8);
        let location = LocationRef::default();
        let hinter =
            HintingInstance::new(&outlines, size, location, HintingOptions::default()).unwrap();
        let outline = outlines.get(gid).unwrap();
        let mut pen = SvgPen::new();
        outline.draw(&hinter, &mut pen).unwrap();
        assert_eq!(
            pen.to_string(),
            "M12.65625,11.015625 Q11.296875,11.421875 10.078125,11.421875 Q8.140625,11.421875 6.9375,9.9375 Q5.734375,8.46875 5.734375,6.1875 L5.734375,0 L3.5625,0 L3.5625,8.421875 Q3.5625,9.328125 3.390625,10.5625 Q3.234375,11.8125 2.9375,13 L5,13 Q5.484375,11.34375 5.578125,10 L5.640625,10 Q6.25,11.25 6.828125,11.828125 Q7.40625,12.40625 8.203125,12.703125 Q9.015625,13 10.15625,13 Q11.421875,13 12.65625,13 Z"
        );
    }
}
