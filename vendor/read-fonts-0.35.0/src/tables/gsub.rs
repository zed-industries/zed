//! the [GSUB] table
//!
//! [GSUB]: https://docs.microsoft.com/en-us/typography/opentype/spec/gsub

pub use super::layout::{
    ChainedSequenceContext, ClassDef, CoverageTable, Device, FeatureList, FeatureVariations,
    Lookup, LookupList, ScriptList, SequenceContext,
};
use super::layout::{ExtensionLookup, LookupFlag, Subtables};

#[cfg(feature = "std")]
mod closure;
#[cfg(test)]
#[path = "../tests/test_gsub.rs"]
mod tests;

include!("../../generated/generated_gsub.rs");

/// A typed GSUB [LookupList] table
pub type SubstitutionLookupList<'a> = LookupList<'a, SubstitutionLookup<'a>>;

/// A GSUB [SequenceContext]
pub type SubstitutionSequenceContext<'a> = super::layout::SequenceContext<'a>;

/// A GSUB [ChainedSequenceContext]
pub type SubstitutionChainContext<'a> = super::layout::ChainedSequenceContext<'a>;

impl<'a, T: FontRead<'a>> ExtensionLookup<'a, T> for ExtensionSubstFormat1<'a, T> {
    fn extension(&self) -> Result<T, ReadError> {
        self.extension()
    }
}

type SubSubtables<'a, T> = Subtables<'a, T, ExtensionSubstFormat1<'a, T>>;

/// The subtables from a GPOS lookup.
///
/// This type is a convenience that removes the need to dig into the
/// [`SubstitutionLookup`] enum in order to access subtables, and it also abstracts
/// away the distinction between extension and non-extension lookups.
pub enum SubstitutionSubtables<'a> {
    Single(SubSubtables<'a, SingleSubst<'a>>),
    Multiple(SubSubtables<'a, MultipleSubstFormat1<'a>>),
    Alternate(SubSubtables<'a, AlternateSubstFormat1<'a>>),
    Ligature(SubSubtables<'a, LigatureSubstFormat1<'a>>),
    Contextual(SubSubtables<'a, SubstitutionSequenceContext<'a>>),
    ChainContextual(SubSubtables<'a, SubstitutionChainContext<'a>>),
    Reverse(SubSubtables<'a, ReverseChainSingleSubstFormat1<'a>>),
}

impl<'a> SubstitutionLookup<'a> {
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
    /// the caller needing to dig into the `SubstitutionLookup` enum itself.
    pub fn subtables(&self) -> Result<SubstitutionSubtables<'a>, ReadError> {
        let raw_lookup = self.of_unit_type();
        let offsets = raw_lookup.subtable_offsets();
        let data = raw_lookup.offset_data();
        match raw_lookup.lookup_type() {
            1 => Ok(SubstitutionSubtables::Single(Subtables::new(offsets, data))),
            2 => Ok(SubstitutionSubtables::Multiple(Subtables::new(
                offsets, data,
            ))),
            3 => Ok(SubstitutionSubtables::Alternate(Subtables::new(
                offsets, data,
            ))),
            4 => Ok(SubstitutionSubtables::Ligature(Subtables::new(
                offsets, data,
            ))),
            5 => Ok(SubstitutionSubtables::Contextual(Subtables::new(
                offsets, data,
            ))),
            6 => Ok(SubstitutionSubtables::ChainContextual(Subtables::new(
                offsets, data,
            ))),
            8 => Ok(SubstitutionSubtables::Reverse(Subtables::new(
                offsets, data,
            ))),
            7 => {
                let first = offsets.first().ok_or(ReadError::OutOfBounds)?.get();
                let ext: ExtensionSubstFormat1<()> = first.resolve(data)?;
                match ext.extension_lookup_type() {
                    1 => Ok(SubstitutionSubtables::Single(Subtables::new_ext(
                        offsets, data,
                    ))),
                    2 => Ok(SubstitutionSubtables::Multiple(Subtables::new_ext(
                        offsets, data,
                    ))),
                    3 => Ok(SubstitutionSubtables::Alternate(Subtables::new_ext(
                        offsets, data,
                    ))),
                    4 => Ok(SubstitutionSubtables::Ligature(Subtables::new_ext(
                        offsets, data,
                    ))),
                    5 => Ok(SubstitutionSubtables::Contextual(Subtables::new_ext(
                        offsets, data,
                    ))),
                    6 => Ok(SubstitutionSubtables::ChainContextual(Subtables::new_ext(
                        offsets, data,
                    ))),
                    8 => Ok(SubstitutionSubtables::Reverse(Subtables::new_ext(
                        offsets, data,
                    ))),
                    other => Err(ReadError::InvalidFormat(other as _)),
                }
            }
            other => Err(ReadError::InvalidFormat(other as _)),
        }
    }
}
