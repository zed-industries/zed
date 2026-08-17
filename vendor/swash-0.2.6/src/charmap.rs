/*!
Mapping characters to nominal glyph identifiers.
*/

use super::internal::cmap;
use super::{FontRef, GlyphId};

/// Proxy for rematerializing a character map.
#[derive(Copy, Clone, Default, Debug)]
pub struct CharmapProxy(u32, u8, bool);

impl CharmapProxy {
    /// Creates character map proxy from the specified font.
    pub fn from_font(font: &FontRef) -> Self {
        if let Some((offset, format, symbol)) = cmap::subtable(font) {
            Self(offset, format, symbol)
        } else {
            Self(0, 0, false)
        }
    }

    /// Materializes a character map from the specified font. This proxy must
    /// have been created from the same font.
    pub fn materialize<'a>(&self, font: &FontRef<'a>) -> Charmap<'a> {
        Charmap {
            data: font.data,
            proxy: *self,
        }
    }
}

/// Maps characters to nominal glyph identifiers.
#[derive(Copy, Clone)]
pub struct Charmap<'a> {
    data: &'a [u8],
    proxy: CharmapProxy,
}

impl<'a> Charmap<'a> {
    /// Creates a character map from the specified font.
    pub fn from_font(font: &FontRef<'a>) -> Self {
        let proxy = CharmapProxy::from_font(font);
        Self {
            data: font.data,
            proxy,
        }
    }

    /// Returns the associated proxy.
    pub fn proxy(&self) -> CharmapProxy {
        self.proxy
    }

    /// Returns a nominal glyph identifier for the specified codepoint.
    pub fn map(&self, codepoint: impl Into<u32>) -> GlyphId {
        let codepoint = codepoint.into();
        let mut glyph_id = cmap::map(self.data, self.proxy.0, self.proxy.1, codepoint).unwrap_or(0);
        // Remap U+0000..=U+00FF to U+F000..=U+F0FF for symbol encodings
        if glyph_id == 0 && self.proxy.2 && codepoint <= 0x00FF {
            glyph_id =
                cmap::map(self.data, self.proxy.0, self.proxy.1, codepoint + 0xF000).unwrap_or(0);
        }
        glyph_id
    }

    /// Invokes the specified closure with all codepoint/glyph identifier
    /// pairs in the character map.
    pub fn enumerate(&self, f: impl FnMut(u32, GlyphId)) {
        cmap::enumerate(self.data, self.proxy.0, f);
    }
}
