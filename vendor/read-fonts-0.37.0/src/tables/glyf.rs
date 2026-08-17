//! The [glyf (Glyph Data)](https://docs.microsoft.com/en-us/typography/opentype/spec/glyf) table

pub mod bytecode;

use bytemuck::AnyBitPattern;
use core::ops::{Add, AddAssign, Div, Mul, MulAssign, Sub};
use types::{F26Dot6, Point};

include!("../../generated/generated_glyf.rs");

/// Marker bits for point flags that are set during variation delta
/// processing and hinting.
#[derive(Copy, Clone, PartialEq, Eq, Default, Debug)]
pub struct PointMarker(u8);

impl PointMarker {
    /// Marker for points that have an explicit delta in a glyph variation
    /// tuple.
    pub const HAS_DELTA: Self = Self(0x4);

    /// Marker that signifies that the x coordinate of a point has been touched
    /// by an IUP hinting instruction.
    pub const TOUCHED_X: Self = Self(0x10);

    /// Marker that signifies that the y coordinate of a point has been touched
    /// by an IUP hinting instruction.
    pub const TOUCHED_Y: Self = Self(0x20);

    /// Marker that signifies that the both coordinates of a point has been touched
    /// by an IUP hinting instruction.
    pub const TOUCHED: Self = Self(Self::TOUCHED_X.0 | Self::TOUCHED_Y.0);

    /// Marks this point as a candidate for weak interpolation.
    ///
    /// Used by the automatic hinter.
    pub const WEAK_INTERPOLATION: Self = Self(0x2);

    /// Marker for points where the distance to next point is very small.
    ///
    /// Used by the automatic hinter.
    pub const NEAR: PointMarker = Self(0x8);
}

impl core::ops::BitOr for PointMarker {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

/// Flags describing the properties of a point.
///
/// Some properties, such as on- and off-curve flags are intrinsic to the point
/// itself. Others, designated as markers are set and cleared while an outline
/// is being transformed during variation application and hinting.
#[derive(
    Copy, Clone, PartialEq, Eq, Default, Debug, bytemuck::AnyBitPattern, bytemuck::NoUninit,
)]
#[repr(transparent)]
pub struct PointFlags(u8);

impl PointFlags {
    // Note: OFF_CURVE_QUAD is signified by the absence of both ON_CURVE
    // and OFF_CURVE_CUBIC bits, per FreeType and TrueType convention.
    const ON_CURVE: u8 = SimpleGlyphFlags::ON_CURVE_POINT.bits;
    const OFF_CURVE_CUBIC: u8 = SimpleGlyphFlags::CUBIC.bits;
    const CURVE_MASK: u8 = Self::ON_CURVE | Self::OFF_CURVE_CUBIC;

    /// Creates a new on curve point flag.
    pub const fn on_curve() -> Self {
        Self(Self::ON_CURVE)
    }

    /// Creates a new off curve quadratic point flag.
    pub const fn off_curve_quad() -> Self {
        Self(0)
    }

    /// Creates a new off curve cubic point flag.
    pub const fn off_curve_cubic() -> Self {
        Self(Self::OFF_CURVE_CUBIC)
    }

    /// Creates a point flag from the given bits. These are truncated
    /// to ignore markers.
    pub const fn from_bits(bits: u8) -> Self {
        Self(bits & Self::CURVE_MASK)
    }

    /// Returns true if this is an on curve point.
    #[inline]
    pub const fn is_on_curve(self) -> bool {
        self.0 & Self::ON_CURVE != 0
    }

    /// Returns true if this is an off curve quadratic point.
    #[inline]
    pub const fn is_off_curve_quad(self) -> bool {
        self.0 & Self::CURVE_MASK == 0
    }

    /// Returns true if this is an off curve cubic point.
    #[inline]
    pub const fn is_off_curve_cubic(self) -> bool {
        self.0 & Self::OFF_CURVE_CUBIC != 0
    }

    pub const fn is_off_curve(self) -> bool {
        self.is_off_curve_quad() || self.is_off_curve_cubic()
    }

    /// Flips the state of the on curve flag.
    ///
    /// This is used for the TrueType `FLIPPT` instruction.
    pub fn flip_on_curve(&mut self) {
        self.0 ^= 1;
    }

    /// Enables the on curve flag.
    ///
    /// This is used for the TrueType `FLIPRGON` instruction.
    pub fn set_on_curve(&mut self) {
        self.0 |= Self::ON_CURVE;
    }

    /// Disables the on curve flag.
    ///
    /// This is used for the TrueType `FLIPRGOFF` instruction.
    pub fn clear_on_curve(&mut self) {
        self.0 &= !Self::ON_CURVE;
    }

    /// Returns true if the given marker is set for this point.
    pub fn has_marker(self, marker: PointMarker) -> bool {
        self.0 & marker.0 != 0
    }

    /// Applies the given marker to this point.
    pub fn set_marker(&mut self, marker: PointMarker) {
        self.0 |= marker.0;
    }

    /// Clears the given marker for this point.
    pub fn clear_marker(&mut self, marker: PointMarker) {
        self.0 &= !marker.0
    }

    /// Returns a copy with all markers cleared.
    pub const fn without_markers(self) -> Self {
        Self(self.0 & Self::CURVE_MASK)
    }

    /// Returns the underlying bits.
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}

/// Trait for types that are usable for TrueType point coordinates.
pub trait PointCoord:
    Copy
    + Default
    // You could bytemuck with me
    + AnyBitPattern
    // You could compare me
    + PartialEq
    + PartialOrd
    // You could do math with me
    + Add<Output = Self>
    + AddAssign
    + Sub<Output = Self>
    + Div<Output = Self>
    + Mul<Output = Self>
    + MulAssign {
    fn from_fixed(x: Fixed) -> Self;
    fn from_i32(x: i32) -> Self;
    fn to_f32(self) -> f32;
    fn midpoint(self, other: Self) -> Self;
}

impl<'a> SimpleGlyph<'a> {
    /// Returns the total number of points.
    pub fn num_points(&self) -> usize {
        self.end_pts_of_contours()
            .last()
            .map(|last| last.get() as usize + 1)
            .unwrap_or(0)
    }

    /// Returns true if the contours in the simple glyph may overlap.
    pub fn has_overlapping_contours(&self) -> bool {
        // Checks the first flag for the OVERLAP_SIMPLE bit.
        // Spec says: "When used, it must be set on the first flag byte for
        // the glyph."
        FontData::new(self.glyph_data())
            .read_at::<SimpleGlyphFlags>(0)
            .map(|flag| flag.contains(SimpleGlyphFlags::OVERLAP_SIMPLE))
            .unwrap_or_default()
    }

    /// Reads points and flags into the provided buffers.
    ///
    /// Drops all flag bits except on-curve. The lengths of the buffers must be
    /// equal to the value returned by [num_points](Self::num_points).
    ///
    /// ## Performance
    ///
    /// As the name implies, this is faster than using the iterator returned by
    /// [points](Self::points) so should be used when it is possible to
    /// preallocate buffers.
    pub fn read_points_fast<C: PointCoord>(
        &self,
        points: &mut [Point<C>],
        flags: &mut [PointFlags],
    ) -> Result<(), ReadError> {
        let n_points = self.num_points();
        if points.len() != n_points || flags.len() != n_points {
            return Err(ReadError::InvalidArrayLen);
        }
        let mut cursor = FontData::new(self.glyph_data()).cursor();
        // We'll need at most n_points flags, but fewer if there are repeats
        let flags_data = cursor.read_array::<u8>(n_points.min(cursor.remaining_bytes()))?;
        let mut flags_iter = flags_data.iter().copied();
        // Keep track of the actual number of flag bytes read so that we can
        // create a new cursor for reading coordinates
        let mut read_flags_bytes = 0;
        let mut i = 0;
        while let Some(flag_bits) = flags_iter.next() {
            read_flags_bytes += 1;
            if SimpleGlyphFlags::from_bits_truncate(flag_bits)
                .contains(SimpleGlyphFlags::REPEAT_FLAG)
            {
                let count = (flags_iter.next().ok_or(ReadError::OutOfBounds)? as usize + 1)
                    .min(n_points - i);
                read_flags_bytes += 1;
                for f in &mut flags[i..i + count] {
                    f.0 = flag_bits;
                }
                i += count;
            } else {
                flags[i].0 = flag_bits;
                i += 1;
            }
            if i == n_points {
                break;
            }
        }
        let mut cursor = FontData::new(self.glyph_data()).cursor();
        cursor.advance_by(read_flags_bytes);
        let mut x = 0i32;
        for (&point_flags, point) in flags.iter().zip(points.as_mut()) {
            let mut delta = 0i32;
            let flag = SimpleGlyphFlags::from_bits_truncate(point_flags.0);
            if flag.contains(SimpleGlyphFlags::X_SHORT_VECTOR) {
                delta = cursor.read::<u8>()? as i32;
                if !flag.contains(SimpleGlyphFlags::X_IS_SAME_OR_POSITIVE_X_SHORT_VECTOR) {
                    delta = -delta;
                }
            } else if !flag.contains(SimpleGlyphFlags::X_IS_SAME_OR_POSITIVE_X_SHORT_VECTOR) {
                delta = cursor.read::<i16>()? as i32;
            }
            x = x.wrapping_add(delta);
            point.x = C::from_i32(x);
        }
        let mut y = 0i32;
        for (point_flags, point) in flags.iter_mut().zip(points.as_mut()) {
            let mut delta = 0i32;
            let flag = SimpleGlyphFlags::from_bits_truncate(point_flags.0);
            if flag.contains(SimpleGlyphFlags::Y_SHORT_VECTOR) {
                delta = cursor.read::<u8>()? as i32;
                if !flag.contains(SimpleGlyphFlags::Y_IS_SAME_OR_POSITIVE_Y_SHORT_VECTOR) {
                    delta = -delta;
                }
            } else if !flag.contains(SimpleGlyphFlags::Y_IS_SAME_OR_POSITIVE_Y_SHORT_VECTOR) {
                delta = cursor.read::<i16>()? as i32;
            }
            y = y.wrapping_add(delta);
            point.y = C::from_i32(y);
            let flags_mask = if cfg!(feature = "spec_next") {
                PointFlags::CURVE_MASK
            } else {
                // Drop the cubic bit if the spec_next feature is not enabled
                PointFlags::ON_CURVE
            };
            point_flags.0 &= flags_mask;
        }
        Ok(())
    }

    /// Returns an iterator over the points in the glyph.
    ///
    /// ## Performance
    ///
    /// This is slower than [read_points_fast](Self::read_points_fast) but
    /// provides access to the points without requiring a preallocated buffer.
    pub fn points(&self) -> impl Iterator<Item = CurvePoint> + 'a + Clone {
        self.points_impl()
            .unwrap_or_else(|| PointIter::new(&[], &[], &[]))
    }

    fn points_impl(&self) -> Option<PointIter<'a>> {
        let end_points = self.end_pts_of_contours();
        let n_points = end_points.last()?.get().checked_add(1)?;
        let data = self.glyph_data();
        let lens = resolve_coords_len(data, n_points).ok()?;
        let total_len = lens.flags + lens.x_coords + lens.y_coords;
        if data.len() < total_len as usize {
            return None;
        }

        let (flags, data) = data.split_at(lens.flags as usize);
        let (x_coords, y_coords) = data.split_at(lens.x_coords as usize);

        Some(PointIter::new(flags, x_coords, y_coords))
    }
}

/// Point with an associated on-curve flag in a simple glyph.
///
/// This type is a simpler representation of the data in the blob.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CurvePoint {
    /// X coordinate.
    pub x: i16,
    /// Y coordinate.
    pub y: i16,
    /// True if this is an on-curve point.
    pub on_curve: bool,
}

impl CurvePoint {
    /// Construct a new `CurvePoint`
    pub fn new(x: i16, y: i16, on_curve: bool) -> Self {
        Self { x, y, on_curve }
    }

    /// Convenience method to construct an on-curve point
    pub fn on_curve(x: i16, y: i16) -> Self {
        Self::new(x, y, true)
    }

    /// Convenience method to construct an off-curve point
    pub fn off_curve(x: i16, y: i16) -> Self {
        Self::new(x, y, false)
    }
}

#[derive(Clone)]
struct PointIter<'a> {
    flags: Cursor<'a>,
    x_coords: Cursor<'a>,
    y_coords: Cursor<'a>,
    flag_repeats: u8,
    cur_flags: SimpleGlyphFlags,
    cur_x: i16,
    cur_y: i16,
}

impl Iterator for PointIter<'_> {
    type Item = CurvePoint;
    fn next(&mut self) -> Option<Self::Item> {
        self.advance_flags()?;
        self.advance_points();
        let is_on_curve = self.cur_flags.contains(SimpleGlyphFlags::ON_CURVE_POINT);
        Some(CurvePoint::new(self.cur_x, self.cur_y, is_on_curve))
    }
}

impl<'a> PointIter<'a> {
    fn new(flags: &'a [u8], x_coords: &'a [u8], y_coords: &'a [u8]) -> Self {
        Self {
            flags: FontData::new(flags).cursor(),
            x_coords: FontData::new(x_coords).cursor(),
            y_coords: FontData::new(y_coords).cursor(),
            flag_repeats: 0,
            cur_flags: SimpleGlyphFlags::empty(),
            cur_x: 0,
            cur_y: 0,
        }
    }

    fn advance_flags(&mut self) -> Option<()> {
        if self.flag_repeats == 0 {
            self.cur_flags = SimpleGlyphFlags::from_bits_truncate(self.flags.read().ok()?);
            self.flag_repeats = self
                .cur_flags
                .contains(SimpleGlyphFlags::REPEAT_FLAG)
                .then(|| self.flags.read().ok())
                .flatten()
                .unwrap_or(0)
                + 1;
        }
        self.flag_repeats -= 1;
        Some(())
    }

    fn advance_points(&mut self) {
        let x_short = self.cur_flags.contains(SimpleGlyphFlags::X_SHORT_VECTOR);
        let x_same_or_pos = self
            .cur_flags
            .contains(SimpleGlyphFlags::X_IS_SAME_OR_POSITIVE_X_SHORT_VECTOR);
        let y_short = self.cur_flags.contains(SimpleGlyphFlags::Y_SHORT_VECTOR);
        let y_same_or_pos = self
            .cur_flags
            .contains(SimpleGlyphFlags::Y_IS_SAME_OR_POSITIVE_Y_SHORT_VECTOR);

        let delta_x = match (x_short, x_same_or_pos) {
            (true, false) => -(self.x_coords.read::<u8>().unwrap_or(0) as i16),
            (true, true) => self.x_coords.read::<u8>().unwrap_or(0) as i16,
            (false, false) => self.x_coords.read::<i16>().unwrap_or(0),
            _ => 0,
        };

        let delta_y = match (y_short, y_same_or_pos) {
            (true, false) => -(self.y_coords.read::<u8>().unwrap_or(0) as i16),
            (true, true) => self.y_coords.read::<u8>().unwrap_or(0) as i16,
            (false, false) => self.y_coords.read::<i16>().unwrap_or(0),
            _ => 0,
        };

        self.cur_x = self.cur_x.wrapping_add(delta_x);
        self.cur_y = self.cur_y.wrapping_add(delta_y);
    }
}

//taken from ttf_parser https://docs.rs/ttf-parser/latest/src/ttf_parser/tables/glyf.rs.html#1-677
/// Resolves coordinate arrays length.
///
/// The length depends on *Simple Glyph Flags*, so we have to process them all to find it.
fn resolve_coords_len(data: &[u8], points_total: u16) -> Result<FieldLengths, ReadError> {
    let mut cursor = FontData::new(data).cursor();
    let mut flags_left = u32::from(points_total);
    //let mut repeats;
    let mut x_coords_len = 0;
    let mut y_coords_len = 0;
    //let mut flags_seen = 0;
    while flags_left > 0 {
        let flags: SimpleGlyphFlags = cursor.read()?;

        // The number of times a glyph point repeats.
        let repeats = if flags.contains(SimpleGlyphFlags::REPEAT_FLAG) {
            let repeats: u8 = cursor.read()?;
            u32::from(repeats) + 1
        } else {
            1
        };

        if repeats > flags_left {
            return Err(ReadError::MalformedData("repeat count too large in glyf"));
        }

        // Non-obfuscated code below.
        // Branchless version is surprisingly faster.
        //
        // if flags.x_short() {
        //     // Coordinate is 1 byte long.
        //     x_coords_len += repeats;
        // } else if !flags.x_is_same_or_positive_short() {
        //     // Coordinate is 2 bytes long.
        //     x_coords_len += repeats * 2;
        // }
        // if flags.y_short() {
        //     // Coordinate is 1 byte long.
        //     y_coords_len += repeats;
        // } else if !flags.y_is_same_or_positive_short() {
        //     // Coordinate is 2 bytes long.
        //     y_coords_len += repeats * 2;
        // }
        let x_short = SimpleGlyphFlags::X_SHORT_VECTOR;
        let x_long = SimpleGlyphFlags::X_SHORT_VECTOR
            | SimpleGlyphFlags::X_IS_SAME_OR_POSITIVE_X_SHORT_VECTOR;
        let y_short = SimpleGlyphFlags::Y_SHORT_VECTOR;
        let y_long = SimpleGlyphFlags::Y_SHORT_VECTOR
            | SimpleGlyphFlags::Y_IS_SAME_OR_POSITIVE_Y_SHORT_VECTOR;
        x_coords_len += ((flags & x_short).bits() != 0) as u32 * repeats;
        x_coords_len += ((flags & x_long).bits() == 0) as u32 * repeats * 2;

        y_coords_len += ((flags & y_short).bits() != 0) as u32 * repeats;
        y_coords_len += ((flags & y_long).bits() == 0) as u32 * repeats * 2;

        flags_left -= repeats;
    }

    Ok(FieldLengths {
        flags: cursor.position()? as u32,
        x_coords: x_coords_len,
        y_coords: y_coords_len,
    })
    //Some((flags_len, x_coords_len, y_coords_len))
}

struct FieldLengths {
    flags: u32,
    x_coords: u32,
    y_coords: u32,
}

/// Transform for a composite component.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Transform {
    /// X scale factor.
    pub xx: F2Dot14,
    /// YX skew factor.
    pub yx: F2Dot14,
    /// XY skew factor.
    pub xy: F2Dot14,
    /// Y scale factor.
    pub yy: F2Dot14,
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            xx: F2Dot14::from_f32(1.0),
            yx: F2Dot14::from_f32(0.0),
            xy: F2Dot14::from_f32(0.0),
            yy: F2Dot14::from_f32(1.0),
        }
    }
}

/// A reference to another glyph. Part of [CompositeGlyph].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Component {
    /// Component flags.
    pub flags: CompositeGlyphFlags,
    /// Glyph identifier.
    pub glyph: GlyphId16,
    /// Anchor for component placement.
    pub anchor: Anchor,
    /// Component transformation matrix.
    pub transform: Transform,
}

/// Anchor position for a composite component.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Anchor {
    Offset { x: i16, y: i16 },
    Point { base: u16, component: u16 },
}

impl<'a> CompositeGlyph<'a> {
    /// Returns an iterator over the components of the composite glyph.
    pub fn components(&self) -> impl Iterator<Item = Component> + 'a + Clone {
        ComponentIter {
            cur_flags: CompositeGlyphFlags::empty(),
            done: false,
            cursor: FontData::new(self.component_data()).cursor(),
        }
    }

    /// Returns an iterator that yields the glyph identifier and flags of each
    /// component in the composite glyph.
    pub fn component_glyphs_and_flags(
        &self,
    ) -> impl Iterator<Item = (GlyphId16, CompositeGlyphFlags)> + 'a + Clone {
        ComponentGlyphIdFlagsIter {
            cur_flags: CompositeGlyphFlags::empty(),
            done: false,
            cursor: FontData::new(self.component_data()).cursor(),
        }
    }

    /// Returns the component count and TrueType interpreter instructions
    /// in a single pass.
    pub fn count_and_instructions(&self) -> (usize, Option<&'a [u8]>) {
        let mut iter = ComponentGlyphIdFlagsIter {
            cur_flags: CompositeGlyphFlags::empty(),
            done: false,
            cursor: FontData::new(self.component_data()).cursor(),
        };
        let mut count = 0;
        while iter.by_ref().next().is_some() {
            count += 1;
        }
        let instructions = if iter
            .cur_flags
            .contains(CompositeGlyphFlags::WE_HAVE_INSTRUCTIONS)
        {
            iter.cursor
                .read::<u16>()
                .ok()
                .map(|len| len as usize)
                .and_then(|len| iter.cursor.read_array(len).ok())
        } else {
            None
        };
        (count, instructions)
    }

    /// Returns the TrueType interpreter instructions.
    pub fn instructions(&self) -> Option<&'a [u8]> {
        self.count_and_instructions().1
    }
}

#[derive(Clone)]
struct ComponentIter<'a> {
    cur_flags: CompositeGlyphFlags,
    done: bool,
    cursor: Cursor<'a>,
}

impl Iterator for ComponentIter<'_> {
    type Item = Component;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }
        let flags: CompositeGlyphFlags = self.cursor.read().ok()?;
        self.cur_flags = flags;
        let glyph = self.cursor.read::<GlyphId16>().ok()?;
        let args_are_words = flags.contains(CompositeGlyphFlags::ARG_1_AND_2_ARE_WORDS);
        let args_are_xy_values = flags.contains(CompositeGlyphFlags::ARGS_ARE_XY_VALUES);
        let anchor = match (args_are_xy_values, args_are_words) {
            (true, true) => Anchor::Offset {
                x: self.cursor.read().ok()?,
                y: self.cursor.read().ok()?,
            },
            (true, false) => Anchor::Offset {
                x: self.cursor.read::<i8>().ok()? as _,
                y: self.cursor.read::<i8>().ok()? as _,
            },
            (false, true) => Anchor::Point {
                base: self.cursor.read().ok()?,
                component: self.cursor.read().ok()?,
            },
            (false, false) => Anchor::Point {
                base: self.cursor.read::<u8>().ok()? as _,
                component: self.cursor.read::<u8>().ok()? as _,
            },
        };
        let mut transform = Transform::default();
        if flags.contains(CompositeGlyphFlags::WE_HAVE_A_SCALE) {
            transform.xx = self.cursor.read().ok()?;
            transform.yy = transform.xx;
        } else if flags.contains(CompositeGlyphFlags::WE_HAVE_AN_X_AND_Y_SCALE) {
            transform.xx = self.cursor.read().ok()?;
            transform.yy = self.cursor.read().ok()?;
        } else if flags.contains(CompositeGlyphFlags::WE_HAVE_A_TWO_BY_TWO) {
            transform.xx = self.cursor.read().ok()?;
            transform.yx = self.cursor.read().ok()?;
            transform.xy = self.cursor.read().ok()?;
            transform.yy = self.cursor.read().ok()?;
        }
        self.done = !flags.contains(CompositeGlyphFlags::MORE_COMPONENTS);

        Some(Component {
            flags,
            glyph,
            anchor,
            transform,
        })
    }
}

/// Iterator that only returns glyph identifiers and flags for each component.
///
/// Significantly faster in cases where we're just processing the glyph
/// tree, counting components or accessing instructions.
#[derive(Clone)]
struct ComponentGlyphIdFlagsIter<'a> {
    cur_flags: CompositeGlyphFlags,
    done: bool,
    cursor: Cursor<'a>,
}

impl Iterator for ComponentGlyphIdFlagsIter<'_> {
    type Item = (GlyphId16, CompositeGlyphFlags);

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }
        let flags: CompositeGlyphFlags = self.cursor.read().ok()?;
        self.cur_flags = flags;
        let glyph = self.cursor.read::<GlyphId16>().ok()?;
        let args_are_words = flags.contains(CompositeGlyphFlags::ARG_1_AND_2_ARE_WORDS);
        if args_are_words {
            self.cursor.advance_by(4);
        } else {
            self.cursor.advance_by(2);
        }
        if flags.contains(CompositeGlyphFlags::WE_HAVE_A_SCALE) {
            self.cursor.advance_by(2);
        } else if flags.contains(CompositeGlyphFlags::WE_HAVE_AN_X_AND_Y_SCALE) {
            self.cursor.advance_by(4);
        } else if flags.contains(CompositeGlyphFlags::WE_HAVE_A_TWO_BY_TWO) {
            self.cursor.advance_by(8);
        }
        self.done = !flags.contains(CompositeGlyphFlags::MORE_COMPONENTS);
        Some((glyph, flags))
    }
}

#[cfg(feature = "experimental_traverse")]
impl<'a> SomeTable<'a> for Component {
    fn type_name(&self) -> &str {
        "Component"
    }

    fn get_field(&self, idx: usize) -> Option<Field<'a>> {
        match idx {
            0 => Some(Field::new("flags", self.flags.bits())),
            1 => Some(Field::new("glyph", self.glyph)),
            2 => match self.anchor {
                Anchor::Point { base, .. } => Some(Field::new("base", base)),
                Anchor::Offset { x, .. } => Some(Field::new("x", x)),
            },
            3 => match self.anchor {
                Anchor::Point { component, .. } => Some(Field::new("component", component)),
                Anchor::Offset { y, .. } => Some(Field::new("y", y)),
            },
            _ => None,
        }
    }
}

impl Anchor {
    /// Compute the flags that describe this anchor
    pub fn compute_flags(&self) -> CompositeGlyphFlags {
        const I8_RANGE: Range<i16> = i8::MIN as i16..i8::MAX as i16 + 1;
        const U8_MAX: u16 = u8::MAX as u16;

        let mut flags = CompositeGlyphFlags::empty();
        match self {
            Anchor::Offset { x, y } => {
                flags |= CompositeGlyphFlags::ARGS_ARE_XY_VALUES;
                if !I8_RANGE.contains(x) || !I8_RANGE.contains(y) {
                    flags |= CompositeGlyphFlags::ARG_1_AND_2_ARE_WORDS;
                }
            }
            Anchor::Point { base, component } => {
                if base > &U8_MAX || component > &U8_MAX {
                    flags |= CompositeGlyphFlags::ARG_1_AND_2_ARE_WORDS;
                }
            }
        }
        flags
    }
}

impl Transform {
    /// Compute the flags that describe this transform
    pub fn compute_flags(&self) -> CompositeGlyphFlags {
        if self.yx != F2Dot14::ZERO || self.xy != F2Dot14::ZERO {
            CompositeGlyphFlags::WE_HAVE_A_TWO_BY_TWO
        } else if self.xx != self.yy {
            CompositeGlyphFlags::WE_HAVE_AN_X_AND_Y_SCALE
        } else if self.xx != F2Dot14::ONE {
            CompositeGlyphFlags::WE_HAVE_A_SCALE
        } else {
            CompositeGlyphFlags::empty()
        }
    }
}

impl PointCoord for F26Dot6 {
    fn from_fixed(x: Fixed) -> Self {
        x.to_f26dot6()
    }

    #[inline]
    fn from_i32(x: i32) -> Self {
        Self::from_i32(x)
    }

    #[inline]
    fn to_f32(self) -> f32 {
        self.to_f32()
    }

    #[inline]
    fn midpoint(self, other: Self) -> Self {
        // FreeType uses integer division on 26.6 to compute midpoints.
        // See: https://github.com/freetype/freetype/blob/de8b92dd7ec634e9e2b25ef534c54a3537555c11/src/base/ftoutln.c#L123
        Self::from_bits(midpoint_i32(self.to_bits(), other.to_bits()))
    }
}

impl PointCoord for Fixed {
    fn from_fixed(x: Fixed) -> Self {
        x
    }

    fn from_i32(x: i32) -> Self {
        Self::from_i32(x)
    }

    fn to_f32(self) -> f32 {
        self.to_f32()
    }

    fn midpoint(self, other: Self) -> Self {
        Self::from_bits(midpoint_i32(self.to_bits(), other.to_bits()))
    }
}

impl PointCoord for i32 {
    fn from_fixed(x: Fixed) -> Self {
        x.to_i32()
    }

    fn from_i32(x: i32) -> Self {
        x
    }

    fn to_f32(self) -> f32 {
        self as f32
    }

    fn midpoint(self, other: Self) -> Self {
        midpoint_i32(self, other)
    }
}

// Midpoint function that avoids overflow on large values.
#[inline(always)]
fn midpoint_i32(a: i32, b: i32) -> i32 {
    // Original overflowing code was: (a + b) / 2
    // Choose wrapping arithmetic here because we shouldn't ever
    // hit this outside of fuzzing or broken fonts _and_ this is
    // called from the outline to path conversion code which is
    // very performance sensitive
    a.wrapping_add(b) / 2
}

impl PointCoord for f32 {
    fn from_fixed(x: Fixed) -> Self {
        x.to_f32()
    }

    fn from_i32(x: i32) -> Self {
        x as f32
    }

    fn to_f32(self) -> f32 {
        self
    }

    fn midpoint(self, other: Self) -> Self {
        // HarfBuzz uses a lerp here so we copy the style to
        // preserve compatibility
        self + 0.5 * (other - self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{FontRef, GlyphId, TableProvider};

    #[test]
    fn simple_glyph() {
        let font = FontRef::new(font_test_data::COLR_GRADIENT_RECT).unwrap();
        let loca = font.loca(None).unwrap();
        let glyf = font.glyf().unwrap();
        let glyph = loca.get_glyf(GlyphId::new(0), &glyf).unwrap().unwrap();
        assert_eq!(glyph.number_of_contours(), 2);
        let simple_glyph = if let Glyph::Simple(simple) = glyph {
            simple
        } else {
            panic!("expected simple glyph");
        };
        assert_eq!(
            simple_glyph
                .end_pts_of_contours()
                .iter()
                .map(|x| x.get())
                .collect::<Vec<_>>(),
            &[3, 7]
        );
        assert_eq!(
            simple_glyph
                .points()
                .map(|pt| (pt.x, pt.y, pt.on_curve))
                .collect::<Vec<_>>(),
            &[
                (5, 0, true),
                (5, 100, true),
                (45, 100, true),
                (45, 0, true),
                (10, 5, true),
                (40, 5, true),
                (40, 95, true),
                (10, 95, true),
            ]
        );
    }

    // Test helper to enumerate all TrueType glyphs in the given font
    fn all_glyphs(font_data: &[u8]) -> impl Iterator<Item = Option<Glyph>> {
        let font = FontRef::new(font_data).unwrap();
        let loca = font.loca(None).unwrap();
        let glyf = font.glyf().unwrap();
        let glyph_count = font.maxp().unwrap().num_glyphs() as u32;
        (0..glyph_count).map(move |gid| loca.get_glyf(GlyphId::new(gid), &glyf).unwrap())
    }

    #[test]
    fn simple_glyph_overlapping_contour_flag() {
        let gids_with_overlap: Vec<_> = all_glyphs(font_test_data::VAZIRMATN_VAR)
            .enumerate()
            .filter_map(|(gid, glyph)| match glyph {
                Some(Glyph::Simple(glyph)) if glyph.has_overlapping_contours() => Some(gid),
                _ => None,
            })
            .collect();
        // Only GID 3 has the overlap bit set
        let expected_gids_with_overlap = vec![3];
        assert_eq!(expected_gids_with_overlap, gids_with_overlap);
    }

    #[test]
    fn composite_glyph_overlapping_contour_flag() {
        let gids_components_with_overlap: Vec<_> = all_glyphs(font_test_data::VAZIRMATN_VAR)
            .enumerate()
            .filter_map(|(gid, glyph)| match glyph {
                Some(Glyph::Composite(glyph)) => Some((gid, glyph)),
                _ => None,
            })
            .flat_map(|(gid, glyph)| {
                glyph
                    .components()
                    .enumerate()
                    .filter_map(move |(comp_ix, comp)| {
                        comp.flags
                            .contains(CompositeGlyphFlags::OVERLAP_COMPOUND)
                            .then_some((gid, comp_ix))
                    })
            })
            .collect();
        // Only GID 2, component 1 has the overlap bit set
        let expected_gids_components_with_overlap = vec![(2, 1)];
        assert_eq!(
            expected_gids_components_with_overlap,
            gids_components_with_overlap
        );
    }

    #[test]
    fn compute_anchor_flags() {
        let anchor = Anchor::Offset { x: -128, y: 127 };
        assert_eq!(
            anchor.compute_flags(),
            CompositeGlyphFlags::ARGS_ARE_XY_VALUES
        );

        let anchor = Anchor::Offset { x: -129, y: 127 };
        assert_eq!(
            anchor.compute_flags(),
            CompositeGlyphFlags::ARGS_ARE_XY_VALUES | CompositeGlyphFlags::ARG_1_AND_2_ARE_WORDS
        );
        let anchor = Anchor::Offset { x: -1, y: 128 };
        assert_eq!(
            anchor.compute_flags(),
            CompositeGlyphFlags::ARGS_ARE_XY_VALUES | CompositeGlyphFlags::ARG_1_AND_2_ARE_WORDS
        );

        let anchor = Anchor::Point {
            base: 255,
            component: 20,
        };
        assert_eq!(anchor.compute_flags(), CompositeGlyphFlags::empty());

        let anchor = Anchor::Point {
            base: 256,
            component: 20,
        };
        assert_eq!(
            anchor.compute_flags(),
            CompositeGlyphFlags::ARG_1_AND_2_ARE_WORDS
        )
    }

    #[test]
    fn compute_transform_flags() {
        fn make_xform(xx: f32, yx: f32, xy: f32, yy: f32) -> Transform {
            Transform {
                xx: F2Dot14::from_f32(xx),
                yx: F2Dot14::from_f32(yx),
                xy: F2Dot14::from_f32(xy),
                yy: F2Dot14::from_f32(yy),
            }
        }

        assert_eq!(
            make_xform(1.0, 0., 0., 1.0).compute_flags(),
            CompositeGlyphFlags::empty()
        );
        assert_eq!(
            make_xform(2.0, 0., 0., 2.0).compute_flags(),
            CompositeGlyphFlags::WE_HAVE_A_SCALE
        );
        assert_eq!(
            make_xform(2.0, 0., 0., 1.0).compute_flags(),
            CompositeGlyphFlags::WE_HAVE_AN_X_AND_Y_SCALE
        );
        assert_eq!(
            make_xform(2.0, 0., 1.0, 1.0).compute_flags(),
            CompositeGlyphFlags::WE_HAVE_A_TWO_BY_TWO
        );
    }

    #[test]
    fn point_flags_and_marker_bits() {
        let bits = [
            PointFlags::OFF_CURVE_CUBIC,
            PointFlags::ON_CURVE,
            PointMarker::HAS_DELTA.0,
            PointMarker::TOUCHED_X.0,
            PointMarker::TOUCHED_Y.0,
        ];
        // Ensure bits don't overlap
        for (i, a) in bits.iter().enumerate() {
            for b in &bits[i + 1..] {
                assert_eq!(a & b, 0);
            }
        }
    }

    #[test]
    fn cubic_glyf() {
        let font = FontRef::new(font_test_data::CUBIC_GLYF).unwrap();
        let loca = font.loca(None).unwrap();
        let glyf = font.glyf().unwrap();
        let glyph = loca.get_glyf(GlyphId::new(2), &glyf).unwrap().unwrap();
        assert_eq!(glyph.number_of_contours(), 1);
        let simple_glyph = if let Glyph::Simple(simple) = glyph {
            simple
        } else {
            panic!("expected simple glyph");
        };
        assert_eq!(
            simple_glyph
                .points()
                .map(|pt| (pt.x, pt.y, pt.on_curve))
                .collect::<Vec<_>>(),
            &[
                (278, 710, true),
                (278, 470, true),
                (300, 500, false),
                (800, 500, false),
                (998, 470, true),
                (998, 710, true),
            ]
        );
    }

    // Minimized test case from https://issues.oss-fuzz.com/issues/382732980
    // Add with overflow when computing midpoint of 1084092352 and 1085243712
    // during outline -> path conversion
    #[test]
    fn avoid_midpoint_overflow() {
        let a = F26Dot6::from_bits(1084092352);
        let b = F26Dot6::from_bits(1085243712);
        let expected = (a + b).to_bits() / 2;
        // Don't panic!
        let midpoint = a.midpoint(b);
        assert_eq!(midpoint.to_bits(), expected);
    }
}
