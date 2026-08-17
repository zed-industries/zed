use super::{ClassDefinition, Coverage, LookupIndex};
use crate::parser::{FromData, FromSlice, LazyArray16, LazyOffsetArray16, Stream};

/// A [Contextual Lookup Subtable](
/// https://docs.microsoft.com/en-us/typography/opentype/spec/chapter2#seqctxt1).
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug)]
pub enum ContextLookup<'a> {
    /// Simple glyph contexts.
    Format1 {
        coverage: Coverage<'a>,
        sets: SequenceRuleSets<'a>,
    },
    /// Class-based glyph contexts.
    Format2 {
        coverage: Coverage<'a>,
        classes: ClassDefinition<'a>,
        sets: SequenceRuleSets<'a>,
    },
    /// Coverage-based glyph contexts.
    Format3 {
        coverage: Coverage<'a>,
        coverages: LazyOffsetArray16<'a, Coverage<'a>>,
        lookups: LazyArray16<'a, SequenceLookupRecord>,
    },
}

impl<'a> ContextLookup<'a> {
    pub(crate) fn parse(data: &'a [u8]) -> Option<Self> {
        let mut s = Stream::new(data);
        match s.read::<u16>()? {
            1 => {
                let coverage = Coverage::parse(s.read_at_offset16(data)?)?;
                let count = s.read::<u16>()?;
                let offsets = s.read_array16(count)?;
                Some(Self::Format1 {
                    coverage,
                    sets: SequenceRuleSets::new(data, offsets),
                })
            }
            2 => {
                let coverage = Coverage::parse(s.read_at_offset16(data)?)?;
                let classes = ClassDefinition::parse(s.read_at_offset16(data)?)?;
                let count = s.read::<u16>()?;
                let offsets = s.read_array16(count)?;
                Some(Self::Format2 {
                    coverage,
                    classes,
                    sets: SequenceRuleSets::new(data, offsets),
                })
            }
            3 => {
                let input_count = s.read::<u16>()?;
                let lookup_count = s.read::<u16>()?;
                let coverage = Coverage::parse(s.read_at_offset16(data)?)?;
                let coverages = s.read_array16(input_count.checked_sub(1)?)?;
                let lookups = s.read_array16(lookup_count)?;
                Some(Self::Format3 {
                    coverage,
                    coverages: LazyOffsetArray16::new(data, coverages),
                    lookups,
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
            Self::Format3 { coverage, .. } => *coverage,
        }
    }
}

/// A list of [`SequenceRuleSet`]s.
pub type SequenceRuleSets<'a> = LazyOffsetArray16<'a, SequenceRuleSet<'a>>;

impl<'a> FromSlice<'a> for SequenceRuleSet<'a> {
    fn parse(data: &'a [u8]) -> Option<Self> {
        Self::parse(data)
    }
}

impl<'a> FromSlice<'a> for SequenceRule<'a> {
    fn parse(data: &'a [u8]) -> Option<Self> {
        let mut s = Stream::new(data);
        let input_count = s.read::<u16>()?;
        let lookup_count = s.read::<u16>()?;
        let input = s.read_array16(input_count.checked_sub(1)?)?;
        let lookups = s.read_array16(lookup_count)?;
        Some(Self { input, lookups })
    }
}

/// A set of [`SequenceRule`]s.
pub type SequenceRuleSet<'a> = LazyOffsetArray16<'a, SequenceRule<'a>>;

/// A sequence rule.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug)]
pub struct SequenceRule<'a> {
    pub input: LazyArray16<'a, u16>,
    pub lookups: LazyArray16<'a, SequenceLookupRecord>,
}

/// A sequence rule record.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug)]
pub struct SequenceLookupRecord {
    pub sequence_index: u16,
    pub lookup_list_index: LookupIndex,
}

impl FromData for SequenceLookupRecord {
    const SIZE: usize = 4;

    #[inline]
    fn parse(data: &[u8]) -> Option<Self> {
        let mut s = Stream::new(data);
        Some(Self {
            sequence_index: s.read::<u16>()?,
            lookup_list_index: s.read::<LookupIndex>()?,
        })
    }
}
