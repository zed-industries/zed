//! the [GDEF] table
//!
//! [GDEF]: https://docs.microsoft.com/en-us/typography/opentype/spec/gdef

pub use super::layout::{
    ChainedSequenceContext, ClassDef, CoverageTable, Device, DeviceOrVariationIndex, FeatureList,
    FeatureVariations, Lookup, LookupList, ScriptList, SequenceContext,
};

use super::variations::ItemVariationStore;

#[cfg(test)]
#[path = "../tests/test_gdef.rs"]
mod tests;

include!("../../generated/generated_gdef.rs");
