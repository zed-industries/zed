//! the [GPOS] table
//!
//! [GPOS]: https://docs.microsoft.com/en-us/typography/opentype/spec/gpos

#[path = "./value_record.rs"]
mod value_record;

#[cfg(feature = "std")]
mod closure;

use crate::array::ComputedArray;

/// reexport stuff from layout that we use
pub use super::layout::{
    ClassDef, CoverageTable, Device, DeviceOrVariationIndex, FeatureList, FeatureVariations,
    Lookup, ScriptList,
};
use super::layout::{ExtensionLookup, LookupFlag, Subtables};
pub use value_record::{Value, ValueContext, ValueRecord};

#[cfg(test)]
#[path = "../tests/test_gpos.rs"]
mod spec_tests;

include!("../../generated/generated_gpos.rs");

/// A typed GPOS [LookupList](super::layout::LookupList) table
pub type PositionLookupList<'a> = super::layout::LookupList<'a, PositionLookup<'a>>;

/// A GPOS [SequenceContext](super::layout::SequenceContext)
pub type PositionSequenceContext<'a> = super::layout::SequenceContext<'a>;

/// A GPOS [ChainedSequenceContext](super::layout::ChainedSequenceContext)
pub type PositionChainContext<'a> = super::layout::ChainedSequenceContext<'a>;

impl<'a> AnchorTable<'a> {
    /// Attempt to resolve the `Device` or `VariationIndex` table for the
    /// x_coordinate, if present
    pub fn x_device(&self) -> Option<Result<DeviceOrVariationIndex<'a>, ReadError>> {
        match self {
            AnchorTable::Format3(inner) => inner.x_device(),
            _ => None,
        }
    }

    /// Attempt to resolve the `Device` or `VariationIndex` table for the
    /// y_coordinate, if present
    pub fn y_device(&self) -> Option<Result<DeviceOrVariationIndex<'a>, ReadError>> {
        match self {
            AnchorTable::Format3(inner) => inner.y_device(),
            _ => None,
        }
    }
}

impl<'a, T: FontRead<'a>> ExtensionLookup<'a, T> for ExtensionPosFormat1<'a, T> {
    fn extension(&self) -> Result<T, ReadError> {
        self.extension()
    }
}

type PosSubtables<'a, T> = Subtables<'a, T, ExtensionPosFormat1<'a, T>>;

/// The subtables from a GPOS lookup.
///
/// This type is a convenience that removes the need to dig into the
/// [`PositionLookup`] enum in order to access subtables, and it also abstracts
/// away the distinction between extension and non-extension lookups.
pub enum PositionSubtables<'a> {
    Single(PosSubtables<'a, SinglePos<'a>>),
    Pair(PosSubtables<'a, PairPos<'a>>),
    Cursive(PosSubtables<'a, CursivePosFormat1<'a>>),
    MarkToBase(PosSubtables<'a, MarkBasePosFormat1<'a>>),
    MarkToLig(PosSubtables<'a, MarkLigPosFormat1<'a>>),
    MarkToMark(PosSubtables<'a, MarkMarkPosFormat1<'a>>),
    Contextual(PosSubtables<'a, PositionSequenceContext<'a>>),
    ChainContextual(PosSubtables<'a, PositionChainContext<'a>>),
}

impl<'a> PositionLookup<'a> {
    pub fn lookup_flag(&self) -> LookupFlag {
        self.of_unit_type().lookup_flag()
    }

    /// Different enumerations for GSUB and GPOS
    pub fn lookup_type(&self) -> u16 {
        self.of_unit_type().lookup_type()
    }

    pub fn mark_filtering_set(&self) -> Option<u16> {
        self.of_unit_type().mark_filtering_set()
    }

    /// Return the subtables for this lookup.
    ///
    /// This method handles both extension and non-extension lookups, and saves
    /// the caller needing to dig into the `PositionLookup` enum itself.
    pub fn subtables(&self) -> Result<PositionSubtables<'a>, ReadError> {
        let raw_lookup = self.of_unit_type();
        let offsets = raw_lookup.subtable_offsets();
        let data = raw_lookup.offset_data();
        match raw_lookup.lookup_type() {
            1 => Ok(PositionSubtables::Single(Subtables::new(offsets, data))),
            2 => Ok(PositionSubtables::Pair(Subtables::new(offsets, data))),
            3 => Ok(PositionSubtables::Cursive(Subtables::new(offsets, data))),
            4 => Ok(PositionSubtables::MarkToBase(Subtables::new(offsets, data))),
            5 => Ok(PositionSubtables::MarkToLig(Subtables::new(offsets, data))),
            6 => Ok(PositionSubtables::MarkToMark(Subtables::new(offsets, data))),
            7 => Ok(PositionSubtables::Contextual(Subtables::new(offsets, data))),
            8 => Ok(PositionSubtables::ChainContextual(Subtables::new(
                offsets, data,
            ))),
            9 => {
                let first = offsets.first().ok_or(ReadError::OutOfBounds)?.get();
                let ext: ExtensionPosFormat1<()> = first.resolve(data)?;
                match ext.extension_lookup_type() {
                    1 => Ok(PositionSubtables::Single(Subtables::new_ext(offsets, data))),
                    2 => Ok(PositionSubtables::Pair(Subtables::new_ext(offsets, data))),
                    3 => Ok(PositionSubtables::Cursive(Subtables::new_ext(
                        offsets, data,
                    ))),
                    4 => Ok(PositionSubtables::MarkToBase(Subtables::new_ext(
                        offsets, data,
                    ))),
                    5 => Ok(PositionSubtables::MarkToLig(Subtables::new_ext(
                        offsets, data,
                    ))),
                    6 => Ok(PositionSubtables::MarkToMark(Subtables::new_ext(
                        offsets, data,
                    ))),
                    7 => Ok(PositionSubtables::Contextual(Subtables::new_ext(
                        offsets, data,
                    ))),
                    8 => Ok(PositionSubtables::ChainContextual(Subtables::new_ext(
                        offsets, data,
                    ))),
                    other => Err(ReadError::InvalidFormat(other as _)),
                }
            }
            other => Err(ReadError::InvalidFormat(other as _)),
        }
    }
}

impl PairPosFormat2<'_> {
    /// Returns the pair of values for the given classes, optionally accounting
    /// for variations.
    ///
    /// The `class1` and `class2` parameters can be computed by passing the
    /// first and second glyphs of the pair to the [`ClassDef`]s returned by
    /// [`Self::class_def1`] and [`Self::class_def2`] respectively.
    #[inline]
    pub fn values(
        &self,
        class1: u16,
        class2: u16,
        context: &ValueContext,
    ) -> Result<[Value; 2], ReadError> {
        let format1 = self.value_format1();
        let format1_len = format1.record_byte_len();
        let format2 = self.value_format2();
        let record_size = format1_len + format2.record_byte_len();
        let data = self.offset_data();
        // Compute an offset into the 2D array of positioning records
        let record_offset = (class1 as usize * record_size * self.class2_count() as usize)
            + (class2 as usize * record_size)
            + self.shape().class1_records_byte_range().start;
        Ok([
            Value::read(data, record_offset, format1, context)?,
            Value::read(data, record_offset + format1_len, format2, context)?,
        ])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pair_pos2_values_match_value_records() {
        let data = FontData::new(font_test_data::gpos::PAIRPOSFORMAT2);
        let table = PairPosFormat2::read(data).unwrap();
        let class1_count = table.class1_count();
        let class2_count = table.class2_count();
        let records = table.class1_records();
        let context = ValueContext::default();
        for class1 in 0..class1_count {
            let class1_record = records.get(class1 as usize).unwrap();
            let class2_records = class1_record.class2_records();
            for class2 in 0..class2_count {
                let record = class2_records.get(class2 as usize).unwrap();
                let value_records = [record.value_record1, record.value_record2]
                    .map(|rec| rec.value(data, &context).unwrap());
                let values = table.values(class1, class2, &context).unwrap();
                assert_eq!(value_records, values);
            }
        }
    }
}
