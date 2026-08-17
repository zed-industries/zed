//! A [Metrics Variations Table](
//! https://docs.microsoft.com/en-us/typography/opentype/spec/mvar) implementation.

use crate::parser::{FromData, LazyArray16, Offset, Offset16, Stream};
use crate::var_store::ItemVariationStore;
use crate::{NormalizedCoordinate, Tag};

#[derive(Clone, Copy)]
struct ValueRecord {
    value_tag: Tag,
    delta_set_outer_index: u16,
    delta_set_inner_index: u16,
}

impl FromData for ValueRecord {
    const SIZE: usize = 8;

    #[inline]
    fn parse(data: &[u8]) -> Option<Self> {
        let mut s = Stream::new(data);
        Some(ValueRecord {
            value_tag: s.read::<Tag>()?,
            delta_set_outer_index: s.read::<u16>()?,
            delta_set_inner_index: s.read::<u16>()?,
        })
    }
}

/// A [Metrics Variations Table](
/// https://docs.microsoft.com/en-us/typography/opentype/spec/mvar).
#[derive(Clone, Copy)]
pub struct Table<'a> {
    variation_store: ItemVariationStore<'a>,
    records: LazyArray16<'a, ValueRecord>,
}

impl<'a> Table<'a> {
    /// Parses a table from raw data.
    pub fn parse(data: &'a [u8]) -> Option<Self> {
        let mut s = Stream::new(data);

        let version = s.read::<u32>()?;
        if version != 0x00010000 {
            return None;
        }

        s.skip::<u16>(); // reserved
        let value_record_size = s.read::<u16>()?;

        if usize::from(value_record_size) != ValueRecord::SIZE {
            return None;
        }

        let count = s.read::<u16>()?;
        if count == 0 {
            return None;
        }

        let var_store_offset = s.read::<Option<Offset16>>()??.to_usize();
        let records = s.read_array16::<ValueRecord>(count)?;
        let variation_store = ItemVariationStore::parse(Stream::new_at(data, var_store_offset)?)?;

        Some(Table {
            variation_store,
            records,
        })
    }

    /// Returns a metric offset by tag.
    pub fn metric_offset(&self, tag: Tag, coordinates: &[NormalizedCoordinate]) -> Option<f32> {
        let (_, record) = self.records.binary_search_by(|r| r.value_tag.cmp(&tag))?;
        self.variation_store.parse_delta(
            record.delta_set_outer_index,
            record.delta_set_inner_index,
            coordinates,
        )
    }
}

impl core::fmt::Debug for Table<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "Table {{ ... }}")
    }
}
