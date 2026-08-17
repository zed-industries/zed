//! A [Glyph Positioning Table](https://docs.microsoft.com/en-us/typography/opentype/spec/gpos)
//! implementation.

// A heavily modified port of https://github.com/harfbuzz/rustybuzz implementation
// originally written by https://github.com/laurmaedje

use core::convert::TryFrom;

use crate::opentype_layout::ChainedContextLookup;
use crate::opentype_layout::{Class, ClassDefinition, ContextLookup, Coverage, LookupSubtable};
use crate::parser::{
    FromData, FromSlice, LazyArray16, LazyArray32, NumFrom, Offset, Offset16, Stream,
};
use crate::GlyphId;

/// A [Device Table](
/// https://docs.microsoft.com/en-us/typography/opentype/spec/chapter2#devVarIdxTbls)
/// hinting values.
#[derive(Clone, Copy)]
pub struct HintingDevice<'a> {
    start_size: u16,
    end_size: u16,
    delta_format: u16,
    delta_values: LazyArray16<'a, u16>,
}

impl HintingDevice<'_> {
    /// Returns X-axis delta.
    pub fn x_delta(&self, units_per_em: u16, pixels_per_em: Option<(u16, u16)>) -> Option<i32> {
        let ppem = pixels_per_em.map(|(x, _)| x)?;
        self.get_delta(ppem, units_per_em)
    }

    /// Returns Y-axis delta.
    pub fn y_delta(&self, units_per_em: u16, pixels_per_em: Option<(u16, u16)>) -> Option<i32> {
        let ppem = pixels_per_em.map(|(_, y)| y)?;
        self.get_delta(ppem, units_per_em)
    }

    fn get_delta(&self, ppem: u16, scale: u16) -> Option<i32> {
        let f = self.delta_format;
        debug_assert!(matches!(f, 1..=3));

        if ppem == 0 || ppem < self.start_size || ppem > self.end_size {
            return None;
        }

        let s = ppem - self.start_size;
        let byte = self.delta_values.get(s >> (4 - f))?;
        let bits = byte >> (16 - (((s & ((1 << (4 - f)) - 1)) + 1) << f));
        let mask = 0xFFFF >> (16 - (1 << f));

        let mut delta = i64::from(bits & mask);
        if delta >= i64::from((mask + 1) >> 1) {
            delta -= i64::from(mask + 1);
        }

        i32::try_from(delta * i64::from(scale) / i64::from(ppem)).ok()
    }
}

impl core::fmt::Debug for HintingDevice<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "HintingDevice {{ ... }}")
    }
}

/// A [Device Table](https://docs.microsoft.com/en-us/typography/opentype/spec/chapter2#devVarIdxTbls)
/// indexes into [Item Variation Store](
/// https://docs.microsoft.com/en-us/typography/opentype/spec/otvarcommonformats#IVS).
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug)]
pub struct VariationDevice {
    pub outer_index: u16,
    pub inner_index: u16,
}

/// A [Device Table](
/// https://docs.microsoft.com/en-us/typography/opentype/spec/chapter2#devVarIdxTbls).
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug)]
pub enum Device<'a> {
    Hinting(HintingDevice<'a>),
    Variation(VariationDevice),
}

impl<'a> Device<'a> {
    pub(crate) fn parse(data: &'a [u8]) -> Option<Self> {
        let mut s = Stream::new(data);
        let first = s.read::<u16>()?;
        let second = s.read::<u16>()?;
        let format = s.read::<u16>()?;
        match format {
            1..=3 => {
                let start_size = first;
                let end_size = second;
                let count = (1 + (end_size - start_size)) >> (4 - format);
                let delta_values = s.read_array16(count)?;
                Some(Self::Hinting(HintingDevice {
                    start_size,
                    end_size,
                    delta_format: format,
                    delta_values,
                }))
            }
            0x8000 => Some(Self::Variation(VariationDevice {
                outer_index: first,
                inner_index: second,
            })),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Default, Debug)]
struct ValueFormatFlags(u8);

#[rustfmt::skip]
impl ValueFormatFlags {
    #[inline] fn x_placement(self) -> bool { self.0 & 0x01 != 0 }
    #[inline] fn y_placement(self) -> bool { self.0 & 0x02 != 0 }
    #[inline] fn x_advance(self) -> bool { self.0 & 0x04 != 0 }
    #[inline] fn y_advance(self) -> bool { self.0 & 0x08 != 0 }
    #[inline] fn x_placement_device(self) -> bool { self.0 & 0x10 != 0 }
    #[inline] fn y_placement_device(self) -> bool { self.0 & 0x20 != 0 }
    #[inline] fn x_advance_device(self) -> bool { self.0 & 0x40 != 0 }
    #[inline] fn y_advance_device(self) -> bool { self.0 & 0x80 != 0 }

    // The ValueRecord struct constrain either i16 values or Offset16 offsets
    // and the total size depend on how many flags are enabled.
    fn size(self) -> usize {
        // The high 8 bits are not used, so make sure we ignore them using 0xFF.
        u16::SIZE * usize::num_from(self.0.count_ones())
    }
}

impl FromData for ValueFormatFlags {
    const SIZE: usize = 2;

    #[inline]
    fn parse(data: &[u8]) -> Option<Self> {
        // There is no data in high 8 bits, so skip it.
        Some(Self(data[1]))
    }
}

/// A [Value Record](https://docs.microsoft.com/en-us/typography/opentype/spec/gpos#value-record).
#[derive(Clone, Copy, Default, Debug)]
pub struct ValueRecord<'a> {
    /// Horizontal adjustment for placement, in design units.
    pub x_placement: i16,
    /// Vertical adjustment for placement, in design units.
    pub y_placement: i16,
    /// Horizontal adjustment for advance, in design units — only used for horizontal layout.
    pub x_advance: i16,
    /// Vertical adjustment for advance, in design units — only used for vertical layout.
    pub y_advance: i16,

    /// A [`Device`] table with horizontal adjustment for placement.
    pub x_placement_device: Option<Device<'a>>,
    /// A [`Device`] table with vertical adjustment for placement.
    pub y_placement_device: Option<Device<'a>>,
    /// A [`Device`] table with horizontal adjustment for advance.
    pub x_advance_device: Option<Device<'a>>,
    /// A [`Device`] table with vertical adjustment for advance.
    pub y_advance_device: Option<Device<'a>>,
}

impl<'a> ValueRecord<'a> {
    // Returns `None` only on parsing error.
    fn parse(
        table_data: &'a [u8],
        s: &mut Stream,
        flags: ValueFormatFlags,
    ) -> Option<ValueRecord<'a>> {
        let mut record = ValueRecord::default();

        if flags.x_placement() {
            record.x_placement = s.read::<i16>()?;
        }

        if flags.y_placement() {
            record.y_placement = s.read::<i16>()?;
        }

        if flags.x_advance() {
            record.x_advance = s.read::<i16>()?;
        }

        if flags.y_advance() {
            record.y_advance = s.read::<i16>()?;
        }

        if flags.x_placement_device() {
            if let Some(offset) = s.read::<Option<Offset16>>()? {
                record.x_placement_device =
                    table_data.get(offset.to_usize()..).and_then(Device::parse)
            }
        }

        if flags.y_placement_device() {
            if let Some(offset) = s.read::<Option<Offset16>>()? {
                record.y_placement_device =
                    table_data.get(offset.to_usize()..).and_then(Device::parse)
            }
        }

        if flags.x_advance_device() {
            if let Some(offset) = s.read::<Option<Offset16>>()? {
                record.x_advance_device =
                    table_data.get(offset.to_usize()..).and_then(Device::parse)
            }
        }

        if flags.y_advance_device() {
            if let Some(offset) = s.read::<Option<Offset16>>()? {
                record.y_advance_device =
                    table_data.get(offset.to_usize()..).and_then(Device::parse)
            }
        }

        Some(record)
    }
}

/// An array of
/// [Value Records](https://docs.microsoft.com/en-us/typography/opentype/spec/gpos#value-record).
#[derive(Clone, Copy)]
pub struct ValueRecordsArray<'a> {
    // We have to store the original table data because ValueRecords can have
    // a offset to Device tables and offset is from the beginning of the table.
    table_data: &'a [u8],
    // A slice that contains all ValueRecords.
    data: &'a [u8],
    // Number of records.
    len: u16,
    // Size of the single record.
    value_len: usize,
    // Flags, used during ValueRecord parsing.
    flags: ValueFormatFlags,
}

impl<'a> ValueRecordsArray<'a> {
    fn parse(
        table_data: &'a [u8],
        count: u16,
        flags: ValueFormatFlags,
        s: &mut Stream<'a>,
    ) -> Option<Self> {
        Some(Self {
            table_data,
            flags,
            len: count,
            value_len: flags.size(),
            data: s.read_bytes(usize::from(count) * flags.size())?,
        })
    }

    /// Returns array's length.
    #[inline]
    pub fn len(&self) -> u16 {
        self.len
    }

    /// Checks if the array is empty.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns a [`ValueRecord`] at index.
    pub fn get(&self, index: u16) -> Option<ValueRecord<'a>> {
        let start = usize::from(index) * self.value_len;
        let end = start + self.value_len;
        let data = self.data.get(start..end)?;
        let mut s = Stream::new(data);
        ValueRecord::parse(self.table_data, &mut s, self.flags)
    }
}

impl core::fmt::Debug for ValueRecordsArray<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "ValueRecordsArray {{ ... }}")
    }
}

/// A [Single Adjustment Positioning Subtable](
/// https://docs.microsoft.com/en-us/typography/opentype/spec/gpos#SP).
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug)]
pub enum SingleAdjustment<'a> {
    Format1 {
        coverage: Coverage<'a>,
        value: ValueRecord<'a>,
    },
    Format2 {
        coverage: Coverage<'a>,
        values: ValueRecordsArray<'a>,
    },
}

impl<'a> SingleAdjustment<'a> {
    fn parse(data: &'a [u8]) -> Option<Self> {
        let mut s = Stream::new(data);
        match s.read::<u16>()? {
            1 => {
                let coverage = Coverage::parse(s.read_at_offset16(data)?)?;
                let flags = s.read::<ValueFormatFlags>()?;
                let value = ValueRecord::parse(data, &mut s, flags)?;
                Some(Self::Format1 { coverage, value })
            }
            2 => {
                let coverage = Coverage::parse(s.read_at_offset16(data)?)?;
                let flags = s.read::<ValueFormatFlags>()?;
                let count = s.read::<u16>()?;
                let values = ValueRecordsArray::parse(data, count, flags, &mut s)?;
                Some(Self::Format2 { coverage, values })
            }
            _ => None,
        }
    }

    /// Returns the subtable coverage.
    #[inline]
    pub fn coverage(&self) -> Coverage<'a> {
        match self {
            Self::Format1 { coverage, .. } => *coverage,
            Self::Format2 { coverage, .. } => *coverage,
        }
    }
}

/// A [`ValueRecord`] pairs set used by [`PairAdjustment`].
#[derive(Clone, Copy)]
pub struct PairSet<'a> {
    data: &'a [u8],
    flags: (ValueFormatFlags, ValueFormatFlags),
    record_len: u8,
}

impl<'a> PairSet<'a> {
    fn parse(data: &'a [u8], flags: (ValueFormatFlags, ValueFormatFlags)) -> Option<Self> {
        let mut s = Stream::new(data);
        let count = s.read::<u16>()?;
        // Max len is 34, so u8 is just enough.
        let record_len = (GlyphId::SIZE + flags.0.size() + flags.1.size()) as u8;
        let data = s.read_bytes(usize::from(count) * usize::from(record_len))?;
        Some(Self {
            data,
            flags,
            record_len,
        })
    }

    #[inline]
    fn binary_search(&self, second: GlyphId) -> Option<&'a [u8]> {
        // Based on Rust std implementation.

        let mut size = self.data.len() / usize::from(self.record_len);
        if size == 0 {
            return None;
        }

        let get_record = |index| {
            let start = index * usize::from(self.record_len);
            let end = start + usize::from(self.record_len);
            self.data.get(start..end)
        };

        let get_glyph = |data: &[u8]| GlyphId(u16::from_be_bytes([data[0], data[1]]));

        let mut base = 0;
        while size > 1 {
            let half = size / 2;
            let mid = base + half;
            // mid is always in [0, size), that means mid is >= 0 and < size.
            // mid >= 0: by definition
            // mid < size: mid = size / 2 + size / 4 + size / 8 ...
            let cmp = get_glyph(get_record(mid)?).cmp(&second);
            base = if cmp == core::cmp::Ordering::Greater {
                base
            } else {
                mid
            };
            size -= half;
        }

        // base is always in [0, size) because base <= mid.
        let value = get_record(base)?;
        if get_glyph(value).cmp(&second) == core::cmp::Ordering::Equal {
            Some(value)
        } else {
            None
        }
    }

    /// Returns a [`ValueRecord`] pair using the second glyph.
    pub fn get(&self, second: GlyphId) -> Option<(ValueRecord<'a>, ValueRecord<'a>)> {
        let record_data = self.binary_search(second)?;
        let mut s = Stream::new(record_data);
        s.skip::<GlyphId>();
        Some((
            ValueRecord::parse(self.data, &mut s, self.flags.0)?,
            ValueRecord::parse(self.data, &mut s, self.flags.1)?,
        ))
    }
}

impl core::fmt::Debug for PairSet<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "PairSet {{ ... }}")
    }
}

// Essentially a `LazyOffsetArray16` but stores additional data required to parse [`PairSet`].

/// A list of [`PairSet`]s.
#[derive(Clone, Copy)]
pub struct PairSets<'a> {
    data: &'a [u8],
    // Zero offsets must be ignored, therefore we're using `Option<Offset16>`.
    offsets: LazyArray16<'a, Option<Offset16>>,
    flags: (ValueFormatFlags, ValueFormatFlags),
}

impl<'a> PairSets<'a> {
    fn new(
        data: &'a [u8],
        offsets: LazyArray16<'a, Option<Offset16>>,
        flags: (ValueFormatFlags, ValueFormatFlags),
    ) -> Self {
        Self {
            data,
            offsets,
            flags,
        }
    }

    /// Returns a value at `index`.
    #[inline]
    pub fn get(&self, index: u16) -> Option<PairSet<'a>> {
        let offset = self.offsets.get(index)??.to_usize();
        self.data
            .get(offset..)
            .and_then(|data| PairSet::parse(data, self.flags))
    }

    /// Returns array's length.
    #[inline]
    pub fn len(&self) -> u16 {
        self.offsets.len()
    }

    /// Checks if the array is empty.
    pub fn is_empty(&self) -> bool {
        self.offsets.is_empty()
    }
}

impl core::fmt::Debug for PairSets<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "PairSets {{ ... }}")
    }
}

/// A [`ValueRecord`] pairs matrix used by [`PairAdjustment`].
#[derive(Clone, Copy)]
pub struct ClassMatrix<'a> {
    // We have to store table's original slice,
    // because offsets in ValueRecords are from the begging of the table.
    table_data: &'a [u8],
    matrix: &'a [u8],
    counts: (u16, u16),
    flags: (ValueFormatFlags, ValueFormatFlags),
    record_len: u8,
}

impl<'a> ClassMatrix<'a> {
    fn parse(
        table_data: &'a [u8],
        counts: (u16, u16),
        flags: (ValueFormatFlags, ValueFormatFlags),
        s: &mut Stream<'a>,
    ) -> Option<Self> {
        let count = usize::num_from(u32::from(counts.0) * u32::from(counts.1));
        // Max len is 32, so u8 is just enough.
        let record_len = (flags.0.size() + flags.1.size()) as u8;
        let matrix = s.read_bytes(count * usize::from(record_len))?;
        Some(Self {
            table_data,
            matrix,
            counts,
            flags,
            record_len,
        })
    }

    /// Returns a [`ValueRecord`] pair using specified classes.
    pub fn get(&self, classes: (u16, u16)) -> Option<(ValueRecord<'a>, ValueRecord<'a>)> {
        if classes.0 >= self.counts.0 || classes.1 >= self.counts.1 {
            return None;
        }

        let idx = usize::from(classes.0) * usize::from(self.counts.1) + usize::from(classes.1);
        let record = self.matrix.get(idx * usize::from(self.record_len)..)?;

        let mut s = Stream::new(record);
        Some((
            ValueRecord::parse(self.table_data, &mut s, self.flags.0)?,
            ValueRecord::parse(self.table_data, &mut s, self.flags.1)?,
        ))
    }
}

impl core::fmt::Debug for ClassMatrix<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "ClassMatrix {{ ... }}")
    }
}

/// A [Pair Adjustment Positioning Subtable](
/// https://docs.microsoft.com/en-us/typography/opentype/spec/gpos#PP).
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug)]
pub enum PairAdjustment<'a> {
    Format1 {
        coverage: Coverage<'a>,
        sets: PairSets<'a>,
    },
    Format2 {
        coverage: Coverage<'a>,
        classes: (ClassDefinition<'a>, ClassDefinition<'a>),
        matrix: ClassMatrix<'a>,
    },
}

impl<'a> PairAdjustment<'a> {
    fn parse(data: &'a [u8]) -> Option<Self> {
        let mut s = Stream::new(data);
        match s.read::<u16>()? {
            1 => {
                let coverage = Coverage::parse(s.read_at_offset16(data)?)?;
                let flags = (s.read::<ValueFormatFlags>()?, s.read::<ValueFormatFlags>()?);
                let count = s.read::<u16>()?;
                let offsets = s.read_array16(count)?;
                Some(Self::Format1 {
                    coverage,
                    sets: PairSets::new(data, offsets, flags),
                })
            }
            2 => {
                let coverage = Coverage::parse(s.read_at_offset16(data)?)?;
                let flags = (s.read::<ValueFormatFlags>()?, s.read::<ValueFormatFlags>()?);
                let classes = (
                    ClassDefinition::parse(s.read_at_offset16(data)?)?,
                    ClassDefinition::parse(s.read_at_offset16(data)?)?,
                );
                let counts = (s.read::<u16>()?, s.read::<u16>()?);
                Some(Self::Format2 {
                    coverage,
                    classes,
                    matrix: ClassMatrix::parse(data, counts, flags, &mut s)?,
                })
            }
            _ => None,
        }
    }

    /// Returns the subtable coverage.
    #[inline]
    pub fn coverage(&self) -> Coverage<'a> {
        match self {
            Self::Format1 { coverage, .. } => *coverage,
            Self::Format2 { coverage, .. } => *coverage,
        }
    }
}

#[derive(Clone, Copy)]
struct EntryExitRecord {
    entry_anchor_offset: Option<Offset16>,
    exit_anchor_offset: Option<Offset16>,
}

impl FromData for EntryExitRecord {
    const SIZE: usize = 4;

    #[inline]
    fn parse(data: &[u8]) -> Option<Self> {
        let mut s = Stream::new(data);
        Some(Self {
            entry_anchor_offset: s.read::<Option<Offset16>>()?,
            exit_anchor_offset: s.read::<Option<Offset16>>()?,
        })
    }
}

/// A list of entry and exit [`Anchor`] pairs.
#[derive(Clone, Copy)]
pub struct CursiveAnchorSet<'a> {
    data: &'a [u8],
    records: LazyArray16<'a, EntryExitRecord>,
}

impl<'a> CursiveAnchorSet<'a> {
    /// Returns an entry [`Anchor`] at index.
    pub fn entry(&self, index: u16) -> Option<Anchor<'a>> {
        let offset = self.records.get(index)?.entry_anchor_offset?.to_usize();
        self.data.get(offset..).and_then(Anchor::parse)
    }

    /// Returns an exit [`Anchor`] at index.
    pub fn exit(&self, index: u16) -> Option<Anchor<'a>> {
        let offset = self.records.get(index)?.exit_anchor_offset?.to_usize();
        self.data.get(offset..).and_then(Anchor::parse)
    }

    /// Returns the number of items.
    pub fn len(&self) -> u16 {
        self.records.len()
    }

    /// Checks if the set is empty.
    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}

impl core::fmt::Debug for CursiveAnchorSet<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "CursiveAnchorSet {{ ... }}")
    }
}

/// A [Cursive Attachment Positioning Subtable](
/// https://docs.microsoft.com/en-us/typography/opentype/spec/gpos#CAP).
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug)]
pub struct CursiveAdjustment<'a> {
    pub coverage: Coverage<'a>,
    pub sets: CursiveAnchorSet<'a>,
}

impl<'a> CursiveAdjustment<'a> {
    fn parse(data: &'a [u8]) -> Option<Self> {
        let mut s = Stream::new(data);
        match s.read::<u16>()? {
            1 => {
                let coverage = Coverage::parse(s.read_at_offset16(data)?)?;
                let count = s.read::<u16>()?;
                let records = s.read_array16(count)?;
                Some(Self {
                    coverage,
                    sets: CursiveAnchorSet { data, records },
                })
            }
            _ => None,
        }
    }
}

/// A [Mark-to-Base Attachment Positioning Subtable](
/// https://docs.microsoft.com/en-us/typography/opentype/spec/gpos#MBP).
#[derive(Clone, Copy, Debug)]
pub struct MarkToBaseAdjustment<'a> {
    /// A mark coverage.
    pub mark_coverage: Coverage<'a>,
    /// A base coverage.
    pub base_coverage: Coverage<'a>,
    /// A list of mark anchors.
    pub marks: MarkArray<'a>,
    /// An anchors matrix.
    pub anchors: AnchorMatrix<'a>,
}

impl<'a> MarkToBaseAdjustment<'a> {
    fn parse(data: &'a [u8]) -> Option<Self> {
        let mut s = Stream::new(data);
        match s.read::<u16>()? {
            1 => {
                let mark_coverage = Coverage::parse(s.read_at_offset16(data)?)?;
                let base_coverage = Coverage::parse(s.read_at_offset16(data)?)?;
                let class_count = s.read::<u16>()?;
                let marks = MarkArray::parse(s.read_at_offset16(data)?)?;
                let anchors = AnchorMatrix::parse(s.read_at_offset16(data)?, class_count)?;
                Some(Self {
                    mark_coverage,
                    base_coverage,
                    marks,
                    anchors,
                })
            }
            _ => None,
        }
    }
}

/// A [Mark-to-Ligature Attachment Positioning Subtable](
/// https://docs.microsoft.com/en-us/typography/opentype/spec/gpos#MLP).
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug)]
pub struct MarkToLigatureAdjustment<'a> {
    pub mark_coverage: Coverage<'a>,
    pub ligature_coverage: Coverage<'a>,
    pub marks: MarkArray<'a>,
    pub ligature_array: LigatureArray<'a>,
}

impl<'a> MarkToLigatureAdjustment<'a> {
    fn parse(data: &'a [u8]) -> Option<Self> {
        let mut s = Stream::new(data);
        match s.read::<u16>()? {
            1 => {
                let mark_coverage = Coverage::parse(s.read_at_offset16(data)?)?;
                let ligature_coverage = Coverage::parse(s.read_at_offset16(data)?)?;
                let class_count = s.read::<u16>()?;
                let marks = MarkArray::parse(s.read_at_offset16(data)?)?;
                let ligature_array = LigatureArray::parse(s.read_at_offset16(data)?, class_count)?;
                Some(Self {
                    mark_coverage,
                    ligature_coverage,
                    marks,
                    ligature_array,
                })
            }
            _ => None,
        }
    }
}

/// An array or ligature anchor matrices.
#[derive(Clone, Copy)]
pub struct LigatureArray<'a> {
    data: &'a [u8],
    class_count: u16,
    offsets: LazyArray16<'a, Offset16>,
}

impl<'a> LigatureArray<'a> {
    fn parse(data: &'a [u8], class_count: u16) -> Option<Self> {
        let mut s = Stream::new(data);
        let count = s.read::<u16>()?;
        let offsets = s.read_array16(count)?;
        Some(Self {
            data,
            class_count,
            offsets,
        })
    }

    /// Returns an [`AnchorMatrix`] at index.
    pub fn get(&self, index: u16) -> Option<AnchorMatrix<'a>> {
        let offset = self.offsets.get(index)?.to_usize();
        let data = self.data.get(offset..)?;
        AnchorMatrix::parse(data, self.class_count)
    }

    /// Returns the array length.
    pub fn len(&self) -> u16 {
        self.offsets.len()
    }

    /// Checks if the array is empty.
    pub fn is_empty(&self) -> bool {
        self.offsets.is_empty()
    }
}

impl core::fmt::Debug for LigatureArray<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "LigatureArray {{ ... }}")
    }
}

#[derive(Clone, Copy)]
struct MarkRecord {
    class: Class,
    mark_anchor: Offset16,
}

impl FromData for MarkRecord {
    const SIZE: usize = 4;

    #[inline]
    fn parse(data: &[u8]) -> Option<Self> {
        let mut s = Stream::new(data);
        Some(Self {
            class: s.read::<Class>()?,
            mark_anchor: s.read::<Offset16>()?,
        })
    }
}

/// A [Mark Array](https://docs.microsoft.com/en-us/typography/opentype/spec/gpos#mark-array-table).
#[derive(Clone, Copy)]
pub struct MarkArray<'a> {
    data: &'a [u8],
    array: LazyArray16<'a, MarkRecord>,
}

impl<'a> MarkArray<'a> {
    fn parse(data: &'a [u8]) -> Option<Self> {
        let mut s = Stream::new(data);
        let count = s.read::<u16>()?;
        let array = s.read_array16(count)?;
        Some(Self { data, array })
    }

    /// Returns contained data at index.
    pub fn get(&self, index: u16) -> Option<(Class, Anchor<'a>)> {
        let record = self.array.get(index)?;
        let anchor = self
            .data
            .get(record.mark_anchor.to_usize()..)
            .and_then(Anchor::parse)?;
        Some((record.class, anchor))
    }

    /// Returns the array length.
    pub fn len(&self) -> u16 {
        self.array.len()
    }

    /// Checks if the array is empty.
    pub fn is_empty(&self) -> bool {
        self.array.is_empty()
    }
}

impl core::fmt::Debug for MarkArray<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "MarkArray {{ ... }}")
    }
}

/// An [Anchor Table](https://docs.microsoft.com/en-us/typography/opentype/spec/gpos#anchor-tables).
///
/// The *Anchor Table Format 2: Design Units Plus Contour Point* is not supported.
#[derive(Clone, Copy, Debug)]
pub struct Anchor<'a> {
    /// Horizontal value, in design units.
    pub x: i16,
    /// Vertical value, in design units.
    pub y: i16,
    /// A [`Device`] table with horizontal value.
    pub x_device: Option<Device<'a>>,
    /// A [`Device`] table with vertical value.
    pub y_device: Option<Device<'a>>,
}

impl<'a> Anchor<'a> {
    fn parse(data: &'a [u8]) -> Option<Self> {
        let mut s = Stream::new(data);
        let format = s.read::<u16>()?;
        if !matches!(format, 1..=3) {
            return None;
        }

        let mut table = Anchor {
            x: s.read::<i16>()?,
            y: s.read::<i16>()?,
            x_device: None,
            y_device: None,
        };

        // Note: Format 2 is not handled since there is currently no way to
        // get a glyph contour point by index.

        if format == 3 {
            table.x_device = s
                .read::<Option<Offset16>>()?
                .and_then(|offset| data.get(offset.to_usize()..))
                .and_then(Device::parse);

            table.y_device = s
                .read::<Option<Offset16>>()?
                .and_then(|offset| data.get(offset.to_usize()..))
                .and_then(Device::parse);
        }

        Some(table)
    }
}

/// An [`Anchor`] parsing helper.
#[derive(Clone, Copy)]
pub struct AnchorMatrix<'a> {
    data: &'a [u8],
    /// Number of rows in the matrix.
    pub rows: u16,
    /// Number of columns in the matrix.
    pub cols: u16,
    matrix: LazyArray32<'a, Option<Offset16>>,
}

impl<'a> AnchorMatrix<'a> {
    fn parse(data: &'a [u8], cols: u16) -> Option<Self> {
        let mut s = Stream::new(data);
        let rows = s.read::<u16>()?;
        let count = u32::from(rows) * u32::from(cols);
        let matrix = s.read_array32(count)?;
        Some(Self {
            data,
            rows,
            cols,
            matrix,
        })
    }

    /// Returns an [`Anchor`] at position.
    pub fn get(&self, row: u16, col: u16) -> Option<Anchor> {
        let idx = u32::from(row) * u32::from(self.cols) + u32::from(col);
        let offset = self.matrix.get(idx)??.to_usize();
        Anchor::parse(self.data.get(offset..)?)
    }
}

impl core::fmt::Debug for AnchorMatrix<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "AnchorMatrix {{ ... }}")
    }
}

/// A [Mark-to-Mark Attachment Positioning Subtable](
/// https://docs.microsoft.com/en-us/typography/opentype/spec/gpos#MMP).
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug)]
pub struct MarkToMarkAdjustment<'a> {
    pub mark1_coverage: Coverage<'a>,
    pub mark2_coverage: Coverage<'a>,
    pub marks: MarkArray<'a>,
    pub mark2_matrix: AnchorMatrix<'a>,
}

impl<'a> MarkToMarkAdjustment<'a> {
    fn parse(data: &'a [u8]) -> Option<Self> {
        let mut s = Stream::new(data);
        match s.read::<u16>()? {
            1 => {
                let mark1_coverage = Coverage::parse(s.read_at_offset16(data)?)?;
                let mark2_coverage = Coverage::parse(s.read_at_offset16(data)?)?;
                let class_count = s.read::<u16>()?;
                let marks = MarkArray::parse(s.read_at_offset16(data)?)?;
                let mark2_matrix = AnchorMatrix::parse(s.read_at_offset16(data)?, class_count)?;
                Some(Self {
                    mark1_coverage,
                    mark2_coverage,
                    marks,
                    mark2_matrix,
                })
            }
            _ => None,
        }
    }
}

/// A glyph positioning
/// [lookup subtable](https://docs.microsoft.com/en-us/typography/opentype/spec/gpos#table-organization)
/// enumeration.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug)]
pub enum PositioningSubtable<'a> {
    Single(SingleAdjustment<'a>),
    Pair(PairAdjustment<'a>),
    Cursive(CursiveAdjustment<'a>),
    MarkToBase(MarkToBaseAdjustment<'a>),
    MarkToLigature(MarkToLigatureAdjustment<'a>),
    MarkToMark(MarkToMarkAdjustment<'a>),
    Context(ContextLookup<'a>),
    ChainContext(ChainedContextLookup<'a>),
}

impl<'a> LookupSubtable<'a> for PositioningSubtable<'a> {
    fn parse(data: &'a [u8], kind: u16) -> Option<Self> {
        match kind {
            1 => SingleAdjustment::parse(data).map(Self::Single),
            2 => PairAdjustment::parse(data).map(Self::Pair),
            3 => CursiveAdjustment::parse(data).map(Self::Cursive),
            4 => MarkToBaseAdjustment::parse(data).map(Self::MarkToBase),
            5 => MarkToLigatureAdjustment::parse(data).map(Self::MarkToLigature),
            6 => MarkToMarkAdjustment::parse(data).map(Self::MarkToMark),
            7 => ContextLookup::parse(data).map(Self::Context),
            8 => ChainedContextLookup::parse(data).map(Self::ChainContext),
            9 => crate::ggg::parse_extension_lookup(data, Self::parse),
            _ => None,
        }
    }
}

impl<'a> PositioningSubtable<'a> {
    /// Returns the subtable coverage.
    #[inline]
    pub fn coverage(&self) -> Coverage<'a> {
        match self {
            Self::Single(t) => t.coverage(),
            Self::Pair(t) => t.coverage(),
            Self::Cursive(t) => t.coverage,
            Self::MarkToBase(t) => t.mark_coverage,
            Self::MarkToLigature(t) => t.mark_coverage,
            Self::MarkToMark(t) => t.mark1_coverage,
            Self::Context(t) => t.coverage(),
            Self::ChainContext(t) => t.coverage(),
        }
    }
}
