//! Helpers for selecting a font size and location in variation space.

use read_fonts::types::Fixed;

use crate::collections::SmallVec;

/// Type for a normalized variation coordinate.
pub type NormalizedCoord = read_fonts::types::F2Dot14;

/// Font size in pixels per em units.
///
/// Sizes in this crate are represented as a ratio of pixels to the size of
/// the em square defined by the font. This is equivalent to the `px` unit
/// in CSS (assuming a DPI scale factor of 1.0).
///
/// To retrieve metrics and outlines in font units, use the [unscaled](Self::unscaled)
/// constructor on this type.
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub struct Size(Option<f32>);

impl Size {
    /// Creates a new font size from the given value in pixels per em units.
    pub fn new(ppem: f32) -> Self {
        Self(Some(ppem))
    }

    /// Creates a new font size for generating unscaled metrics or outlines in
    /// font units.
    pub fn unscaled() -> Self {
        Self(None)
    }

    /// Returns the raw size in pixels per em units.
    ///
    /// Results in `None` if the size is unscaled.
    pub fn ppem(self) -> Option<f32> {
        self.0
    }

    /// Computes a linear scale factor for this font size and the given units
    /// per em value which can be retrieved from the [Metrics](crate::metrics::Metrics)
    /// type or from the [head](read_fonts::tables::head::Head) table.
    ///
    /// Returns 1.0 for an unscaled size or when `units_per_em` is 0.
    pub fn linear_scale(self, units_per_em: u16) -> f32 {
        match self.0 {
            Some(ppem) if units_per_em != 0 => ppem / units_per_em as f32,
            _ => 1.0,
        }
    }

    /// Computes a fixed point linear scale factor that matches FreeType.
    pub(crate) fn fixed_linear_scale(self, units_per_em: u16) -> Fixed {
        // FreeType computes a 16.16 scale factor that converts to 26.6.
        // This is done in two steps, assuming use of FT_Set_Pixel_Size:
        // 1) height is multiplied by 64:
        //    <https://gitlab.freedesktop.org/freetype/freetype/-/blob/49781ab72b2dfd0f78172023921d08d08f323ade/src/base/ftobjs.c#L3596>
        // 2) this value is divided by UPEM:
        //    (here, scaled_h=height and h=upem)
        //    <https://gitlab.freedesktop.org/freetype/freetype/-/blob/49781ab72b2dfd0f78172023921d08d08f323ade/src/base/ftobjs.c#L3312>
        match self.0 {
            Some(ppem) if units_per_em > 0 => {
                Fixed::from_bits((ppem * 64.) as i32) / Fixed::from_bits(units_per_em as i32)
            }
            _ => {
                // This is an identity scale for the pattern
                // `mul_div(value, scale, 64)`
                Fixed::from_bits(0x10000 * 64)
            }
        }
    }
}

/// Reference to an ordered sequence of normalized variation coordinates.
///
/// To convert from user coordinates see [`crate::AxisCollection::location`].
///
/// This type represents a position in the variation space where each
/// coordinate corresponds to an axis (in the same order as the `fvar` table)
/// and is a normalized value in the range `[-1..1]`.
///
/// See [Coordinate Scales and Normalization](https://learn.microsoft.com/en-us/typography/opentype/spec/otvaroverview#coordinate-scales-and-normalization)
/// for further details.
///
/// If the array is larger in length than the number of axes, extraneous
/// values are ignored. If it is smaller, unrepresented axes are assumed to be
/// at their default positions (i.e. 0).
///
/// A value of this type constructed with `default()` represents the default
/// position for each axis.
///
/// Normalized coordinates are ignored for non-variable fonts.
#[derive(Copy, Clone, Default, Debug)]
pub struct LocationRef<'a>(&'a [NormalizedCoord]);

impl<'a> LocationRef<'a> {
    /// Creates a new sequence of normalized coordinates from the given array.
    pub fn new(coords: &'a [NormalizedCoord]) -> Self {
        Self(coords)
    }

    /// Returns the underlying array of normalized coordinates.
    pub fn coords(&self) -> &'a [NormalizedCoord] {
        self.0
    }

    /// Returns true if this represents the default location in variation
    /// space.
    ///
    /// This is represented a set of normalized coordinates that is either
    /// empty or contains all zeroes.
    pub fn is_default(&self) -> bool {
        self.0.is_empty() || self.0.iter().all(|coord| *coord == NormalizedCoord::ZERO)
    }

    /// Returns the underlying coordinate array if any of the entries are
    /// non-zero. Otherwise returns the empty slice.
    ///
    /// This allows internal routines to bypass expensive variation code
    /// paths by just checking for an empty slice.
    pub(crate) fn effective_coords(&self) -> &'a [NormalizedCoord] {
        if self.is_default() {
            &[]
        } else {
            self.0
        }
    }
}

impl<'a> From<&'a [NormalizedCoord]> for LocationRef<'a> {
    fn from(value: &'a [NormalizedCoord]) -> Self {
        Self(value)
    }
}

impl<'a> IntoIterator for LocationRef<'a> {
    type IntoIter = core::slice::Iter<'a, NormalizedCoord>;
    type Item = &'a NormalizedCoord;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'a> IntoIterator for &'_ LocationRef<'a> {
    type IntoIter = core::slice::Iter<'a, NormalizedCoord>;
    type Item = &'a NormalizedCoord;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

/// Maximum number of coords to store inline in a `Location` object.
///
/// This value was chosen to maximize use of space in the underlying
/// `SmallVec` storage.
const MAX_INLINE_COORDS: usize = 8;

/// Ordered sequence of normalized variation coordinates.
///
/// To produce from user coordinates see [`crate::AxisCollection::location`].
///
/// This is an owned version of [`LocationRef`]. See the documentation on that
/// type for more detail.
#[derive(Clone, Debug, Hash, Eq, PartialEq, Default)]
pub struct Location {
    coords: SmallVec<NormalizedCoord, MAX_INLINE_COORDS>,
}

impl Location {
    /// Creates a new location with the given number of normalized coordinates.
    ///
    /// Each element will be initialized to the default value (0.0).
    pub fn new(len: usize) -> Self {
        Self {
            coords: SmallVec::with_len(len, NormalizedCoord::default()),
        }
    }

    /// Returns the underlying slice of normalized coordinates.
    pub fn coords(&self) -> &[NormalizedCoord] {
        self.coords.as_slice()
    }

    /// Returns a mutable reference to the underlying slice of normalized
    /// coordinates.
    pub fn coords_mut(&mut self) -> &mut [NormalizedCoord] {
        self.coords.as_mut_slice()
    }
}

impl<'a> From<&'a Location> for LocationRef<'a> {
    fn from(value: &'a Location) -> Self {
        LocationRef(value.coords())
    }
}

impl<'a> IntoIterator for &'a Location {
    type IntoIter = core::slice::Iter<'a, NormalizedCoord>;
    type Item = &'a NormalizedCoord;

    fn into_iter(self) -> Self::IntoIter {
        self.coords().iter()
    }
}

impl<'a> IntoIterator for &'a mut Location {
    type IntoIter = core::slice::IterMut<'a, NormalizedCoord>;
    type Item = &'a mut NormalizedCoord;

    fn into_iter(self) -> Self::IntoIter {
        self.coords_mut().iter_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{FontRef, MetadataProvider};

    #[test]
    fn effective_coords() {
        let font = FontRef::new(font_test_data::AVAR2_CHECKER).unwrap();
        let location = font.axes().location([("AVAR", 50.0), ("AVWK", 75.0)]);
        let loc_ref = LocationRef::from(&location);
        assert!(!loc_ref.is_default());
        assert_eq!(loc_ref.effective_coords().len(), 2);
    }

    #[test]
    fn effective_coords_for_default() {
        let font = FontRef::new(font_test_data::AVAR2_CHECKER).unwrap();
        let location = font.axes().location([("AVAR", 0.0), ("AVWK", 0.0)]);
        let loc_ref = LocationRef::from(&location);
        assert!(loc_ref.is_default());
        assert_eq!(loc_ref.effective_coords().len(), 0);
        assert_eq!(loc_ref.coords().len(), 2);
    }
}
