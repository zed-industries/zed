//! Text spaces help prevent mixing up what a text position refers to. For example, `Point<Buffer>`
//! and `Point<Multibuffer>` shouldn't be intermixed.
//!
//! # Space conversions
//!
//! `OriginIn<Inner, Outer>` represents the position of `Inner` within `Outer`, allowing conversions
//! between these spaces. For example:
//!
//! `Point<OriginIn<Excerpt, Multibuffer>>` is the position of the text space `Excerpt` within the
//! `Multibuffer`, and so can be used to convert between `Point<Excerpt>` and `Point<Multibuffer>`.
//!
//! `Point<OriginIn<Excerpt, Buffer>>` is the position of the excerpt within the buffer it is an
//! excerpt of, and so can be used to convert between `Point<Excerpt>` and `Point<Buffer>`. Together
//! with `Point<Origin<Excerpt, Multibuffer>>`, this can be used to convert `Point<Multibuffer>` to
//! and from a `Point<Buffer>`.
//!
//! In some cases, it's known that two text spaces have the same positions. `SameSpace` handles this
//! case, providing compiletime evidence that the two spaces are the same.
//!
//! For example, a singleton `Multibuffer` has the same positions as its single `Buffer`.
//! `Multibuffer::as_singleton` can return a `Option<(Buffer, SameSpace<Buffer, Multibuffer>)>`.
//! This `SameSpace` type can then be used to do zero-cost conversions between `Point<Multibuffer>`
//! and `Point<Buffer>`.
//!
//! # Delta space
//!
//! The `Delta` space is used to represent differences in position. Position deltas are not aware of
//! the text space of the original positions, and so differences from one space can be used to shift
//! positions in another space.
//!
//! It has special treatment in the arithmetic traits to allow restriction to only sensible
//! operations, enabled by `Delta: TextSpace<Type = Relative>` whereas text spaces are typically
//! `Absolute`. Specficially:
//!
//! * `Point<S> - Point<S>` -> `Point<Delta>`
//! * `Point<S> + Point<Delta>` -> `Point<S>`
//! * `Point<Delta> + Point<S>` -> `Point<S>`
//! * `Point<Delta> - Point<Delta>` -> `Point<Delta>`
//! * `Point<Delta> + Point<Delta>` -> `Point<Delta>`
//!
//! This helps ensure that `Point<S>` always refers to an absolute position when `S` is not `Delta`
//! or `Utf16<Delta>`!
//!
//! # Utf16 space
//!
//! The `Utf16` space is used for positions that are in terms of UTF16 code units. This is useful
//! because these are used by the Language Server Protocol.

mod saturating_sub;

pub use saturating_sub::SaturatingSub;

use std::{
    fmt::Debug,
    marker::PhantomData,
    ops::{Add, Range, Sub},
};

pub trait TextSpace {
    /// Whether positions in this space are `Absolute` or `Relative`. This is always `Absolute`,
    /// except for in the `Delta` and `Utf16<Delta>` spaces.
    ///
    /// This is used to avoid arithmetic trait impl overlap for impls that involve both absolute
    /// positions and deltas.
    type Type;
    /// The type used for the `column` field of `Point`, typically `Chars<Delta>`.
    type Column;
}

/// Almost always used as the type for `TextSpace::Type`.
pub struct Absolute;

/// Used by the `Delta` space as the type for `TextSpace::Type`.
pub struct Relative;

/// `TextSpace` for positions in buffers. In practice this will be the actual `Buffer` /
/// `Multibuffer` / `Display` / `Inset` / etc types types.
pub struct Buffer;
impl TextSpace for Buffer {
    type Type = Absolute;
    type Column = Chars<Delta>;
}

/// `TextSpace` for differences in position.
pub struct Delta;
impl TextSpace for Delta {
    type Type = Relative;
    type Column = Chars<Delta>;
}

/// `TextSpace` that uses UTF16 code units.
pub struct Utf16<T>(T);
impl<Space: TextSpace> TextSpace for Utf16<Space> {
    type Type = Space::Type;
    type Column = OffsetUtf16<Delta>;
}

pub struct Point<Space: TextSpace> {
    row: Row<Space>,
    column: Space::Column,
}

pub struct Row<Space> {
    row_count: u32,
    _phantom: PhantomData<Space>,
}

pub struct Offset<Space> {
    byte_count: u32,
    _phantom: PhantomData<Space>,
}

pub struct OffsetUtf16<Space> {
    code_unit_count: u32,
    _phantom: PhantomData<Space>,
}

pub struct Chars<Space> {
    char_count: u32,
    _phantom: PhantomData<Space>,
}

// Design decisions:
//
// * An alternative might be to have `Offset<Utf16<Buffer>>` instead of `OffsetUtf16<Buffer>`, and
// make `Chars` into a space modifier: `Offset<Chars<Buffer>>` instead of `Chars<Buffer>`. This
// would deduplicate a lot of code, but I think the duplication is worth it for simpler types and
// better field names.
//
// * Not attempting to have column types distinguish between absolute positions and deltas. This
// could be represented cleanly by having another text space modifier `Line<Space>`, and
// representing absolute columns as `Chars<Line<Space>>`. The column type for `Delta` would still be
// `Chars<Delta>`. Decided against this for now because this is already a bit complex and the added
// complexity doesn't seem worth it.
//
// * Inclusion of `Delta`. At first I planned to defer this to a later phase, as I thought the only
// benefit was to prevent errors involving adding absolute positions. However, it became clear that
// distinct types for deltas are needed regardless of this, in order to take deltas from one space
// and use them in another. To not have a space for this would necessitate separate OffsetDelta /
// RowDelta / etc types and the related boilerplate. I'm quite pleased with how this duplication was
// avoided by having `Delta` also be a text space.

// todo! rename `Space` type variable to `S`?

impl<Space> Offset<Space>
where
    Space: TextSpace,
{
    const ZERO: Self = Self::new(0);

    pub const fn new(byte_count: u32) -> Self {
        Self {
            byte_count,
            _phantom: PhantomData,
        }
    }

    pub const fn is_zero(self) -> bool {
        self.byte_count == 0
    }

    pub const fn to_delta(self) -> Offset<Delta> {
        Offset::new(self.byte_count)
    }

    // todo! document why private
    const fn from_delta(delta: Offset<Delta>) -> Self {
        Self::new(delta.byte_count)
    }
}

/// `Offset<Delta>` + `Offset<Delta>` -> `Offset<Delta>`.
impl Add for Offset<Delta> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::new(self.byte_count + other.byte_count)
    }
}

/// `Offset<Delta>` - `Offset<Delta>` -> `Offset<Delta>`.
impl Sub for Offset<Delta> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self::new(self.byte_count - other.byte_count)
    }
}

/// `Offset` + `Offset<Delta>` -> `Offset`.
impl<Space: TextSpace<Type = Absolute>> Add<Offset<Delta>> for Offset<Space> {
    type Output = Self;

    fn add(self, delta: Offset<Delta>) -> Self {
        Self::new(self.byte_count + delta.byte_count)
    }
}

/// `Offset<Delta>` + `Offset` -> `Offset`.
impl<Space: TextSpace<Type = Absolute>> Add<Offset<Space>> for Offset<Delta> {
    type Output = Self;

    fn add(self, other: Offset<Space>) -> Self {
        Self::new(self.byte_count + other.byte_count)
    }
}

/// `Offset` - `Offset<Delta>` -> `Offset`.
impl<Space: TextSpace<Type = Absolute>> Sub<Offset<Delta>> for Offset<Space> {
    type Output = Offset<Delta>;

    fn sub(self, delta: Offset<Delta>) -> Offset<Delta> {
        Offset::new(self.byte_count - delta.byte_count)
    }
}

/// `Offset` - `Offset` -> `Offset<Delta>`.
impl<Space: TextSpace<Type = Absolute>> Sub for Offset<Space> {
    type Output = Offset<Delta>;

    fn sub(self, other: Offset<Space>) -> Offset<Delta> {
        Offset::new(self.byte_count - other.byte_count)
    }
}

impl<Space> Copy for Offset<Space> {}

impl<Space> Clone for Offset<Space> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<Space: TextSpace> Default for Offset<Space> {
    fn default() -> Self {
        Self::ZERO
    }
}

impl<Space> PartialOrd for Offset<Space> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<Space> Ord for Offset<Space> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.byte_count.cmp(&other.byte_count)
    }
}

impl<Space> PartialEq for Offset<Space> {
    fn eq(&self, other: &Self) -> bool {
        self.byte_count == other.byte_count
    }
}

impl<Space> Eq for Offset<Space> {}

impl<Space> Debug for Offset<Space> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}Offset({})", type_name::<Space>(), self.byte_count)
    }
}

fn type_name<T>() -> &'static str {
    std::any::type_name::<T>().split("::").last().unwrap()
}

// Space conversions

// todo! Would be nicer to have `to_inner` be methods on the position types instead of on the
// converter.

pub trait SpaceConverter<Inner, Outer> {
    fn to_inner(self, outer: Outer) -> Inner;

    fn to_outer(self, inner: Inner) -> Outer;
}

/// Indicates that two text spaces have identical positions. For example, the positions in a
/// singleton `Multibuffer` are the same the `Buffer`.
pub struct SameSpace<Inner, Outer>(PhantomData<Inner>, PhantomData<Outer>);

impl<Inner: TextSpace, Outer: TextSpace> SameSpace<Inner, Outer> {
    pub fn new() -> Self {
        Self(PhantomData, PhantomData)
    }
}

/// The origin of an `Inner` text space within the `Outer`, used for converting between positions in
/// the two spaces.
///
///
pub struct OriginIn<Inner, Outer>(PhantomData<Inner>, PhantomData<Outer>);

impl<
        Type,
        Column,
        Inner: TextSpace<Type = Type, Column = Column>,
        Outer: TextSpace<Type = Type, Column = Column>,
    > TextSpace for OriginIn<Inner, Outer>
{
    type Type = Absolute;
    type Column = Chars<Delta>;
}

impl<Inner, Outer, Type, Column> SpaceConverter<Offset<Inner>, Offset<Outer>>
    for SameSpace<Inner, Outer>
where
    Inner: TextSpace<Type = Type, Column = Column>,
    Outer: TextSpace<Type = Type, Column = Column>,
{
    // Hopefully Rust is clever enough to make this zero-cost.
    fn to_inner(self, outer: Offset<Outer>) -> Offset<Inner> {
        Offset {
            byte_count: outer.byte_count,
            _phantom: PhantomData,
        }
    }

    // Hopefully Rust is clever enough to make this zero-cost.
    fn to_outer(self, inner: Offset<Inner>) -> Offset<Outer> {
        Offset {
            byte_count: inner.byte_count,
            _phantom: PhantomData,
        }
    }
}

impl<Inner, Outer, Type, Column> SpaceConverter<Offset<Inner>, Offset<Outer>>
    for Offset<OriginIn<Inner, Outer>>
where
    Inner: TextSpace<Type = Type, Column = Column>,
    Outer: TextSpace<Type = Type, Column = Column>,
{
    fn to_inner(self, outer: Offset<Outer>) -> Offset<Inner> {
        Offset {
            byte_count: outer.byte_count - self.byte_count,
            _phantom: PhantomData,
        }
    }

    fn to_outer(self, inner: Offset<Inner>) -> Offset<Outer> {
        Offset {
            byte_count: inner.byte_count + self.byte_count,
            _phantom: PhantomData,
        }
    }
}

impl<Inner, Outer, Type, Column> SpaceConverter<Row<Inner>, Row<Outer>> for SameSpace<Inner, Outer>
where
    Inner: TextSpace<Type = Type, Column = Column>,
    Outer: TextSpace<Type = Type, Column = Column>,
{
    // Hopefully Rust is clever enough to make this zero-cost.
    fn to_inner(self, outer: Row<Outer>) -> Row<Inner> {
        Row {
            row_count: outer.row_count,
            _phantom: PhantomData,
        }
    }

    // Hopefully Rust is clever enough to make this zero-cost.
    fn to_outer(self, inner: Row<Inner>) -> Row<Outer> {
        Row {
            row_count: inner.row_count,
            _phantom: PhantomData,
        }
    }
}

impl<Inner, Outer, Type, Column> SpaceConverter<Row<Inner>, Row<Outer>>
    for Row<OriginIn<Inner, Outer>>
where
    Inner: TextSpace<Type = Type, Column = Column>,
    Outer: TextSpace<Type = Type, Column = Column>,
{
    fn to_inner(self, outer: Row<Outer>) -> Row<Inner> {
        Row {
            row_count: outer.row_count - self.row_count,
            _phantom: PhantomData,
        }
    }

    fn to_outer(self, inner: Row<Inner>) -> Row<Outer> {
        Row {
            row_count: inner.row_count + self.row_count,
            _phantom: PhantomData,
        }
    }
}

impl<Inner, Outer, InnerPosition, OuterPosition>
    SpaceConverter<Range<InnerPosition>, Range<OuterPosition>> for SameSpace<Inner, Outer>
where
    Self: Copy + SpaceConverter<InnerPosition, OuterPosition>,
{
    fn to_inner(self, outer: Range<OuterPosition>) -> Range<InnerPosition> {
        Range {
            start: self.to_inner(outer.start),
            end: self.to_inner(outer.end),
        }
    }

    fn to_outer(self, inner: Range<InnerPosition>) -> Range<OuterPosition> {
        Range {
            start: self.to_outer(inner.start),
            end: self.to_outer(inner.end),
        }
    }
}

impl<Inner, Outer, Type, Column> SpaceConverter<Range<Offset<Inner>>, Range<Offset<Outer>>>
    for Offset<OriginIn<Inner, Outer>>
where
    Self: Copy + SpaceConverter<Offset<Inner>, Offset<Outer>>,
    Inner: TextSpace<Type = Type, Column = Column>,
    Outer: TextSpace<Type = Type, Column = Column>,
{
    fn to_inner(self, outer: Range<Offset<Outer>>) -> Range<Offset<Inner>> {
        Range {
            start: self.to_inner(outer.start),
            end: self.to_inner(outer.end),
        }
    }

    fn to_outer(self, inner: Range<Offset<Inner>>) -> Range<Offset<Outer>> {
        Range {
            start: self.to_outer(inner.start),
            end: self.to_outer(inner.end),
        }
    }
}

// Position conversions

struct BufferSnapshot;

trait ToOffset<Space: TextSpace> {
    fn to_offset(&self, snapshot: &BufferSnapshot) -> Offset<Space>;
}

impl<Space: TextSpace> ToOffset<Space> for Point<Space> {
    fn to_offset(&self, _snapshot: &BufferSnapshot) -> Offset<Space> {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    struct Excerpt;
    impl TextSpace for Excerpt {
        type Type = Absolute;
        type Column = Chars<Delta>;
    }

    struct Multibuffer;
    impl TextSpace for Multibuffer {
        type Type = Absolute;
        type Column = Chars<Delta>;
    }

    impl Multibuffer {
        fn as_singleton(&self) -> Option<(Buffer, SameSpace<Buffer, Multibuffer>)> {
            Some((Buffer, SameSpace::new()))
        }
    }

    #[test]
    fn arithmetic() {
        let a: Offset<Buffer> = Offset::new(10);
        let b: Offset<Buffer> = Offset::new(5);
        let c: Offset<Buffer> = Offset::new(2);
        let d: Offset<Delta> = b - c;
        assert_eq!(a + d, Offset::<Buffer>::new(13));
        assert_eq!(a + (d + d), Offset::<Buffer>::new(16));
        assert_eq!(a + d + d, Offset::<Buffer>::new(16));
    }

    #[test]
    fn range_in_excerpt() {
        let multibuffer_range: Range<Offset<Multibuffer>> = Offset::new(10)..Offset::new(20);

        let excerpt_origin: Offset<OriginIn<Excerpt, Multibuffer>> = Offset::new(5);

        let excerpt_range: Range<Offset<Excerpt>> = excerpt_origin.to_inner(multibuffer_range);

        assert_eq!(
            excerpt_range,
            Offset::<Excerpt>::new(5)..Offset::<Excerpt>::new(15)
        );
    }

    #[test]
    fn as_singleton() {
        let multibuffer = Multibuffer;
        let offset = Offset::<Multibuffer>::new(5);

        if let Some((buffer, same_positions)) = multibuffer.as_singleton() {
            assert_eq!(same_positions.to_inner(offset), Offset::<Buffer>::new(5));
        }
    }
}
