//! Helper for loading (possibly variable) horizontal glyph metrics.

use raw::{
    tables::{hmtx::Hmtx, hvar::Hvar},
    types::{F2Dot14, GlyphId},
    FontRef, TableProvider,
};

/// Access to horizontal glyph metrics.
#[derive(Clone)]
pub(crate) struct GlyphHMetrics<'a> {
    pub hmtx: Hmtx<'a>,
    pub hvar: Option<Hvar<'a>>,
}

impl<'a> GlyphHMetrics<'a> {
    pub fn new(font: &FontRef<'a>) -> Option<Self> {
        // Note: hmtx is required and HVAR is optional
        let hmtx = font.hmtx().ok()?;
        let hvar = font.hvar().ok();
        Some(Self { hmtx, hvar })
    }

    /// Returns the advance width (in font units) for the given glyph and
    /// the location in variation space represented by the set of normalized
    /// coordinates in 2.14 fixed point.
    pub fn advance_width(&self, gid: GlyphId, coords: &'a [F2Dot14]) -> i32 {
        let mut advance = self.hmtx.advance(gid).unwrap_or_default() as i32;
        if let (false, Some(hvar)) = (coords.is_empty(), &self.hvar) {
            advance += hvar
                .advance_width_delta(gid, coords)
                .map(|delta| delta.to_i32())
                .unwrap_or(0);
        }
        advance
    }

    /// Returns the left side bearing (in font units) for the given glyph and
    /// the location in variation space represented by the set of normalized
    /// coordinates in 2.14 fixed point.    
    pub fn lsb(&self, gid: GlyphId, coords: &'a [F2Dot14]) -> i32 {
        let mut lsb = self.hmtx.side_bearing(gid).unwrap_or_default() as i32;
        if let (false, Some(hvar)) = (coords.is_empty(), &self.hvar) {
            lsb += hvar
                .lsb_delta(gid, coords)
                .map(|delta| delta.to_i32())
                .unwrap_or(0);
        }
        lsb
    }
}
