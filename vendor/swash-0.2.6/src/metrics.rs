/*!
Font and glyph metrics.
*/

use super::internal::*;
use super::{FontRef, GlyphId, NormalizedCoord};

/// Proxy for rematerializing metrics.
#[derive(Copy, Clone, Default)]
pub struct MetricsProxy {
    units_per_em: u16,
    glyph_count: u16,
    is_monospace: bool,
    has_vertical_metrics: bool,
    ascent: i16,
    descent: i16,
    leading: i16,
    vertical_ascent: i16,
    vertical_descent: i16,
    vertical_leading: i16,
    cap_height: i16,
    x_height: i16,
    average_width: u16,
    max_width: u16,
    underline_offset: i16,
    strikeout_offset: i16,
    stroke_size: i16,
    mvar: u32,
    hmtx: u32,
    hvar: u32,
    hmtx_count: u16,
    has_vvar: bool,
    vertical: Vertical,
}

impl MetricsProxy {
    /// Creates a metrics proxy for the specified font.
    pub fn from_font(font: &FontRef) -> Self {
        let mut metadata = Self {
            units_per_em: 1,
            ..Self::default()
        };
        metadata.fill(font);
        metadata
    }

    /// Materializes font metrics for the specified font and
    /// normalized variation coordinates. This proxy must have been created
    /// from the same font.
    pub fn materialize_metrics(&self, font: &FontRef, coords: &[NormalizedCoord]) -> Metrics {
        let data = font.data;
        let mut m = Metrics {
            units_per_em: self.units_per_em,
            glyph_count: self.glyph_count,
            is_monospace: self.is_monospace,
            has_vertical_metrics: self.has_vertical_metrics,
            ascent: self.ascent as f32,
            descent: self.descent as f32,
            leading: self.leading as f32,
            vertical_ascent: self.vertical_ascent as f32,
            vertical_descent: self.vertical_descent as f32,
            vertical_leading: self.vertical_leading as f32,
            cap_height: self.cap_height as f32,
            x_height: self.x_height as f32,
            average_width: self.average_width as f32,
            max_width: self.max_width as f32,
            underline_offset: self.underline_offset as f32,
            strikeout_offset: self.strikeout_offset as f32,
            stroke_size: self.stroke_size as f32,
        };
        if self.mvar != 0 && !coords.is_empty() {
            if let Some(v) = var::Mvar::new(data, self.mvar, coords) {
                use var::mvar_tags::*;
                m.ascent += v.delta(HASC);
                m.descent += v.delta(HDSC);
                m.leading += v.delta(HLGP);
                if self.has_vertical_metrics {
                    m.vertical_ascent += v.delta(VASC);
                    m.vertical_descent += v.delta(VDSC);
                    m.vertical_leading += v.delta(VLGP);
                }
                m.cap_height += v.delta(CPHT);
                m.x_height += v.delta(XHGT);
                m.underline_offset += v.delta(UNDO);
                m.strikeout_offset += v.delta(STRO);
                m.stroke_size += v.delta(UNDS);
            }
        }
        m
    }

    /// Materializes glyph metrics for the specified font and
    /// normalized variation coordinates. This proxy must have been created
    /// from the same font.
    pub fn materialize_glyph_metrics<'a>(
        &self,
        font: &FontRef<'a>,
        coords: &'a [NormalizedCoord],
    ) -> GlyphMetrics<'a> {
        let data = font.data;
        let mut vertical = self.vertical;
        if !coords.is_empty() {
            if let Vertical::Synthesized {
                mvar,
                advance,
                origin,
            } = &mut vertical
            {
                if *mvar != 0 {
                    if let Some(v) = var::Mvar::new(data, *mvar, coords) {
                        use var::mvar_tags::*;
                        let ascent_delta = v.delta(HASC);
                        let descent_delta = v.delta(HDSC);
                        *advance += ascent_delta + descent_delta;
                        *origin += ascent_delta;
                    }
                }
            }
        }
        GlyphMetrics {
            data,
            coords,
            units_per_em: self.units_per_em,
            glyph_count: self.glyph_count,
            hmtx: self.hmtx,
            hvar: self.hvar,
            hmtx_count: self.hmtx_count,
            has_vvar: self.has_vvar,
            vertical,
            scale: 1.,
        }
    }

    /// Returns the number of font design units per em unit.
    pub fn units_per_em(&self) -> u16 {
        self.units_per_em
    }

    /// Returns the number of glyphs in the font.
    pub fn glyph_count(&self) -> u16 {
        self.glyph_count
    }

    fn fill(&mut self, font: &FontRef) -> Option<()> {
        let head = font.head()?;
        self.units_per_em = head.units_per_em();
        self.glyph_count = font.maxp()?.glyph_count();
        let mut have_line_metrics = false;
        let os2 = font.os2();
        if let Some(os2) = os2 {
            let flags = os2.selection_flags();
            self.average_width = os2.average_char_width() as u16;
            self.strikeout_offset = os2.strikeout_position();
            self.stroke_size = os2.strikeout_size();
            self.x_height = os2.x_height();
            self.cap_height = os2.cap_height();
            if flags.use_typographic_metrics() {
                self.ascent = os2.typographic_ascender();
                self.descent = -os2.typographic_descender();
                self.leading = os2.typographic_line_gap();
                have_line_metrics = self.ascent != 0;
            }
        }
        let hhea = font.hhea();
        if let Some(hhea) = hhea {
            self.max_width = hhea.max_advance();
            if !have_line_metrics {
                self.ascent = hhea.ascender();
                self.descent = -hhea.descender();
                self.leading = hhea.line_gap();
            }
        }
        let vhea = font.vhea();
        if let Some(vhea) = vhea {
            self.has_vertical_metrics = true;
            self.vertical_ascent = vhea.ascender();
            self.vertical_descent = -vhea.descender();
            self.vertical_leading = vhea.line_gap();
        } else {
            self.vertical_ascent = (self.units_per_em / 2) as i16;
            self.vertical_descent = self.vertical_ascent;
        }
        if let Some(post) = font.post() {
            self.underline_offset = post.underline_position();
            self.stroke_size = post.underline_size();
            self.is_monospace = post.is_fixed_pitch();
        }
        self.mvar = font.table_offset(var::MVAR);
        self.hmtx_count = hhea.map(|t| t.num_long_metrics()).unwrap_or(1);
        self.hmtx = font.table_offset(xmtx::HMTX);
        self.hvar = font.table_offset(var::HVAR);
        let mut vmtx = 0;
        if vhea.is_some() {
            vmtx = font.table_offset(xmtx::VMTX);
        }
        if vmtx != 0 {
            let long_count = vhea.unwrap().num_long_metrics();
            let vvar = font.table_offset(var::VVAR);
            self.has_vvar = vvar != 0;
            let vorg = font.table_offset(vorg::VORG);
            if vorg != 0 {
                self.vertical = Vertical::VmtxVorg {
                    long_count,
                    vmtx,
                    vvar,
                    vorg,
                };
            } else {
                let glyf = font.table_offset(glyf::GLYF);
                let loca = font.table_offset(glyf::LOCA);
                let loca_fmt = font
                    .head()
                    .map(|t| t.index_to_location_format() as u8)
                    .unwrap_or(0xFF);
                if glyf != 0 && loca != 0 && loca_fmt != 0xFF {
                    self.vertical = Vertical::VmtxGlyf {
                        loca_fmt,
                        long_count,
                        vmtx,
                        vvar,
                        glyf,
                        loca,
                    }
                }
            }
        } else {
            self.vertical = Vertical::Synthesized {
                mvar: self.mvar,
                advance: self.ascent as f32 + self.descent as f32,
                origin: self.ascent as f32,
            };
        }
        Some(())
    }
}

/// Global font metrics.
#[derive(Copy, Clone, Default, Debug)]
pub struct Metrics {
    /// Number of font design units per em unit.
    pub units_per_em: u16,
    /// Number of glyphs in the font.
    pub glyph_count: u16,
    /// True if the font is monospace.
    pub is_monospace: bool,
    /// True if the font provides canonical vertical metrics.
    pub has_vertical_metrics: bool,
    /// Distance from the baseline to the top of the alignment box.
    pub ascent: f32,
    /// Distance from the baseline to the bottom of the alignment box.
    pub descent: f32,
    /// Recommended additional spacing between lines.
    pub leading: f32,
    /// Distance from the vertical center baseline to the right edge of
    /// the design space.
    pub vertical_ascent: f32,
    /// Distance from the vertical center baseline to the left edge of
    /// the design space.
    pub vertical_descent: f32,
    /// Recommended additional spacing between columns.
    pub vertical_leading: f32,
    /// Distance from the baseline to the top of a typical English capital.
    pub cap_height: f32,
    /// Distance from the baseline to the top of the lowercase "x" or
    /// similar character.
    pub x_height: f32,
    /// Average width of all non-zero characters in the font.
    pub average_width: f32,
    /// Maximum advance width of all characters in the font.
    pub max_width: f32,
    /// Recommended distance from the baseline to the top of an underline
    /// stroke.
    pub underline_offset: f32,
    /// Recommended distance from the baseline to the top of a strikeout
    /// stroke.
    pub strikeout_offset: f32,
    /// Recommended thickness of an underline or strikeout stroke.
    pub stroke_size: f32,
}

impl Metrics {
    /// Creates a new set of metrics from the specified font and
    /// normalized variation coordinates.
    pub(crate) fn from_font(font: &FontRef, coords: &[i16]) -> Self {
        let meta = MetricsProxy::from_font(font);
        meta.materialize_metrics(font, coords)
    }

    /// Creates a new set of metrics scaled for the specified pixels
    /// per em unit.
    pub fn scale(&self, ppem: f32) -> Self {
        self.linear_scale(if self.units_per_em != 0 {
            ppem / self.units_per_em as f32
        } else {
            1.
        })
    }

    /// Creates a new set of metrics scaled by the specified factor.
    pub fn linear_scale(&self, s: f32) -> Self {
        let mut m = *self;
        m.ascent *= s;
        m.descent *= s;
        m.leading *= s;
        m.vertical_ascent *= s;
        m.vertical_descent *= s;
        m.vertical_leading *= s;
        m.cap_height *= s;
        m.x_height *= s;
        m.average_width *= s;
        m.max_width *= s;
        m.underline_offset *= s;
        m.strikeout_offset *= s;
        m.stroke_size *= s;
        m
    }
}

/// Glyph advances, side bearings and vertical origins.
#[derive(Copy, Clone)]
pub struct GlyphMetrics<'a> {
    data: &'a [u8],
    coords: &'a [i16],
    units_per_em: u16,
    glyph_count: u16,
    hmtx: u32,
    hvar: u32,
    hmtx_count: u16,
    has_vvar: bool,
    vertical: Vertical,
    scale: f32,
}

impl<'a> GlyphMetrics<'a> {
    /// Creates a new set of glyph metrics from the specified font and
    /// normalized variation coordinates.
    pub(crate) fn from_font(font: &FontRef<'a>, coords: &'a [NormalizedCoord]) -> Self {
        let proxy = MetricsProxy::from_font(font);
        proxy.materialize_glyph_metrics(font, coords)
    }

    /// Returns the number of font design units per em unit.
    pub fn units_per_em(&self) -> u16 {
        self.units_per_em
    }

    /// Returns the number of glyphs in the font.
    pub fn glyph_count(&self) -> u16 {
        self.glyph_count
    }

    /// Returns true if the font provides canonical vertical glyph metrics.
    pub fn has_vertical_metrics(&self) -> bool {
        !matches!(self.vertical, Vertical::Synthesized { .. })
    }

    /// Returns true if variations are supported.
    pub fn has_variations(&self) -> bool {
        self.hvar != 0 || self.has_vvar
    }

    /// Creates a new set of metrics scaled for the specified pixels
    /// per em unit.
    pub fn scale(&self, ppem: f32) -> Self {
        self.linear_scale(if self.units_per_em() != 0 {
            ppem / self.units_per_em() as f32
        } else {
            1.
        })
    }

    /// Creates a new set of metrics scaled by the specified factor.
    pub fn linear_scale(&self, scale: f32) -> Self {
        let mut copy = *self;
        copy.scale = scale;
        copy
    }

    /// Returns the horizontal advance for the specified glyph.
    pub fn advance_width(&self, glyph_id: GlyphId) -> f32 {
        let mut v = xmtx::advance(self.data, self.hmtx, self.hmtx_count, glyph_id) as f32;
        if self.hvar != 0 {
            v += var::advance_delta(self.data, self.hvar, glyph_id, self.coords);
        }
        v * self.scale
    }

    /// Returns the left side bearing for the specified glyph.
    pub fn lsb(&self, glyph_id: GlyphId) -> f32 {
        let mut v = xmtx::sb(self.data, self.hmtx, self.hmtx_count, glyph_id) as f32;
        if self.hvar != 0 {
            v += var::sb_delta(self.data, self.hvar, glyph_id, self.coords);
        }
        v * self.scale
    }

    /// Returns the vertical advance for the specified glyph.
    pub fn advance_height(&self, glyph_id: GlyphId) -> f32 {
        self.scale
            * match self.vertical {
                Vertical::VmtxGlyf {
                    vmtx,
                    vvar,
                    long_count,
                    ..
                }
                | Vertical::VmtxVorg {
                    vmtx,
                    vvar,
                    long_count,
                    ..
                } => {
                    let mut v = xmtx::advance(self.data, vmtx, long_count, glyph_id) as f32;
                    if vvar != 0 {
                        v += var::advance_delta(self.data, vvar, glyph_id, self.coords);
                    }
                    v
                }
                Vertical::Synthesized { advance, .. } => advance,
            }
    }

    /// Returns the top side bearing for the specified glyph.
    pub fn tsb(&self, glyph_id: GlyphId) -> f32 {
        self.scale
            * match self.vertical {
                Vertical::VmtxGlyf {
                    vmtx,
                    vvar,
                    long_count,
                    ..
                }
                | Vertical::VmtxVorg {
                    vmtx,
                    vvar,
                    long_count,
                    ..
                } => {
                    let mut v = xmtx::sb(self.data, vmtx, long_count, glyph_id) as f32;
                    if vvar != 0 {
                        v += var::sb_delta(self.data, vvar, glyph_id, self.coords);
                    }
                    v
                }
                Vertical::Synthesized { .. } => 0.,
            }
    }

    /// Returns the vertical origin for the specified glyph id.
    pub fn vertical_origin(&self, glyph_id: GlyphId) -> f32 {
        self.scale
            * match self.vertical {
                Vertical::VmtxGlyf {
                    loca_fmt,
                    loca,
                    glyf,
                    ..
                } => {
                    if let Some(max_y) = glyf::ymax(self.data, loca_fmt, loca, glyf, glyph_id) {
                        max_y as f32 + self.tsb(glyph_id)
                    } else {
                        self.units_per_em as f32
                    }
                }
                Vertical::VmtxVorg { vorg, .. } => vorg::origin(self.data, vorg, glyph_id)
                    .unwrap_or(self.units_per_em as i16)
                    as f32,
                Vertical::Synthesized { origin, .. } => origin,
            }
    }
}

#[derive(Copy, Clone)]
#[repr(u8)]
enum Vertical {
    VmtxGlyf {
        loca_fmt: u8,
        long_count: u16,
        vmtx: u32,
        vvar: u32,
        glyf: u32,
        loca: u32,
    },
    VmtxVorg {
        long_count: u16,
        vmtx: u32,
        vvar: u32,
        vorg: u32,
    },
    Synthesized {
        mvar: u32,
        advance: f32,
        origin: f32,
    },
}

impl Default for Vertical {
    fn default() -> Self {
        Self::Synthesized {
            mvar: 0,
            advance: 0.,
            origin: 0.,
        }
    }
}
