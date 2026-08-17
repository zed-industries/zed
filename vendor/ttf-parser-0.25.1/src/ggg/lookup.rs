use crate::parser::{
    FromData, FromSlice, LazyArray16, LazyOffsetArray16, Offset, Offset16, Offset32, Stream,
};

/// A list of [`Lookup`] values.
pub type LookupList<'a> = LazyOffsetArray16<'a, Lookup<'a>>;

/// A [Lookup Table](https://docs.microsoft.com/en-us/typography/opentype/spec/chapter2#lookup-table).
#[derive(Clone, Copy, Debug)]
pub struct Lookup<'a> {
    /// Lookup qualifiers.
    pub flags: LookupFlags,
    /// Available subtables.
    pub subtables: LookupSubtables<'a>,
    /// Index into GDEF mark glyph sets structure.
    pub mark_filtering_set: Option<u16>,
}

impl<'a> FromSlice<'a> for Lookup<'a> {
    fn parse(data: &'a [u8]) -> Option<Self> {
        let mut s = Stream::new(data);
        let kind = s.read::<u16>()?;
        let flags = s.read::<LookupFlags>()?;
        let count = s.read::<u16>()?;
        let offsets = s.read_array16(count)?;

        let mut mark_filtering_set: Option<u16> = None;
        if flags.use_mark_filtering_set() {
            mark_filtering_set = Some(s.read::<u16>()?);
        }

        Some(Self {
            flags,
            subtables: LookupSubtables {
                kind,
                data,
                offsets,
            },
            mark_filtering_set,
        })
    }
}

/// A trait for parsing Lookup subtables.
///
/// Internal use only.
pub trait LookupSubtable<'a>: Sized {
    /// Parses raw data.
    fn parse(data: &'a [u8], kind: u16) -> Option<Self>;
}

/// A list of lookup subtables.
#[derive(Clone, Copy)]
pub struct LookupSubtables<'a> {
    kind: u16,
    data: &'a [u8],
    offsets: LazyArray16<'a, Offset16>,
}

impl core::fmt::Debug for LookupSubtables<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "LookupSubtables {{ ... }}")
    }
}

impl<'a> LookupSubtables<'a> {
    /// Returns a number of items in the LookupSubtables.
    #[inline]
    pub fn len(&self) -> u16 {
        self.offsets.len()
    }

    /// Checks if there are any items.
    pub fn is_empty(&self) -> bool {
        self.offsets.is_empty()
    }

    /// Parses a subtable at index.
    ///
    /// Accepts either
    /// [`PositioningSubtable`](crate::gpos::PositioningSubtable)
    /// or [`SubstitutionSubtable`](crate::gsub::SubstitutionSubtable).
    ///
    /// Technically, we can enforce it at compile time, but it makes code too convoluted.
    pub fn get<T: LookupSubtable<'a>>(&self, index: u16) -> Option<T> {
        let offset = self.offsets.get(index)?.to_usize();
        let data = self.data.get(offset..)?;
        T::parse(data, self.kind)
    }

    /// Creates an iterator over subtables.
    ///
    /// We cannot use `IntoIterator` here, because we have to use user-provided base type.
    #[allow(clippy::should_implement_trait)]
    pub fn into_iter<T: LookupSubtable<'a>>(self) -> LookupSubtablesIter<'a, T> {
        LookupSubtablesIter {
            data: self,
            index: 0,
            data_type: core::marker::PhantomData,
        }
    }
}

/// An iterator over lookup subtables.
#[allow(missing_debug_implementations)]
pub struct LookupSubtablesIter<'a, T: LookupSubtable<'a>> {
    data: LookupSubtables<'a>,
    index: u16,
    data_type: core::marker::PhantomData<T>,
}

impl<'a, T: LookupSubtable<'a>> Iterator for LookupSubtablesIter<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.data.len() {
            self.index += 1;
            self.data.get(self.index - 1)
        } else {
            None
        }
    }
}

/// Lookup table flags.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug)]
pub struct LookupFlags(pub u16);

#[rustfmt::skip]
#[allow(missing_docs)]
impl LookupFlags {
    #[inline] pub fn right_to_left(self) -> bool { self.0 & 0x0001 != 0 }
    #[inline] pub fn ignore_base_glyphs(self) -> bool { self.0 & 0x0002 != 0 }
    #[inline] pub fn ignore_ligatures(self) -> bool { self.0 & 0x0004 != 0 }
    #[inline] pub fn ignore_marks(self) -> bool { self.0 & 0x0008 != 0 }
    #[inline] pub fn ignore_flags(self) -> bool { self.0 & 0x000E != 0 }
    #[inline] pub fn use_mark_filtering_set(self) -> bool { self.0 & 0x0010 != 0 }
    #[inline] pub fn mark_attachment_type(self) -> u8 { ((self.0 & 0xFF00) >> 8) as u8 }
}

impl FromData for LookupFlags {
    const SIZE: usize = 2;

    #[inline]
    fn parse(data: &[u8]) -> Option<Self> {
        u16::parse(data).map(Self)
    }
}

pub(crate) fn parse_extension_lookup<'a, T: 'a>(
    data: &'a [u8],
    parse: impl FnOnce(&'a [u8], u16) -> Option<T>,
) -> Option<T> {
    let mut s = Stream::new(data);
    let format = s.read::<u16>()?;
    match format {
        1 => {
            let kind = s.read::<u16>()?;
            let offset = s.read::<Offset32>()?.to_usize();
            parse(data.get(offset..)?, kind)
        }
        _ => None,
    }
}
