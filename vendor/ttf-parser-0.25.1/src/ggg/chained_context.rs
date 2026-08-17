use super::{ClassDefinition, Coverage, SequenceLookupRecord};
use crate::parser::{FromSlice, LazyArray16, LazyOffsetArray16, Offset, Offset16, Stream};

/// A [Chained Contextual Lookup Subtable](
/// https://docs.microsoft.com/en-us/typography/opentype/spec/chapter2#chseqctxt1).
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug)]
pub enum ChainedContextLookup<'a> {
    /// Simple glyph contexts.
    Format1 {
        coverage: Coverage<'a>,
        sets: ChainedSequenceRuleSets<'a>,
    },
    /// Class-based glyph contexts.
    Format2 {
        coverage: Coverage<'a>,
        backtrack_classes: ClassDefinition<'a>,
        input_classes: ClassDefinition<'a>,
        lookahead_classes: ClassDefinition<'a>,
        sets: ChainedSequenceRuleSets<'a>,
    },
    /// Coverage-based glyph contexts.
    Format3 {
        coverage: Coverage<'a>,
        backtrack_coverages: LazyOffsetArray16<'a, Coverage<'a>>,
        input_coverages: LazyOffsetArray16<'a, Coverage<'a>>,
        lookahead_coverages: LazyOffsetArray16<'a, Coverage<'a>>,
        lookups: LazyArray16<'a, SequenceLookupRecord>,
    },
}

impl<'a> ChainedContextLookup<'a> {
    pub(crate) fn parse(data: &'a [u8]) -> Option<Self> {
        let mut s = Stream::new(data);
        match s.read::<u16>()? {
            1 => {
                let coverage = Coverage::parse(s.read_at_offset16(data)?)?;
                let count = s.read::<u16>()?;
                let offsets = s.read_array16(count)?;
                Some(Self::Format1 {
                    coverage,
                    sets: ChainedSequenceRuleSets::new(data, offsets),
                })
            }
            2 => {
                let coverage = Coverage::parse(s.read_at_offset16(data)?)?;

                let mut parse_func = || match s.read::<Option<Offset16>>()? {
                    Some(offset) => Some(ClassDefinition::parse(data.get(offset.to_usize()..)?)?),
                    None => Some(ClassDefinition::Empty),
                };

                let backtrack_classes = parse_func()?;
                let input_classes = parse_func()?;
                let lookahead_classes = parse_func()?;

                let count = s.read::<u16>()?;
                let offsets = s.read_array16(count)?;
                Some(Self::Format2 {
                    coverage,
                    backtrack_classes,
                    input_classes,
                    lookahead_classes,
                    sets: LazyOffsetArray16::new(data, offsets),
                })
            }
            3 => {
                let backtrack_count = s.read::<u16>()?;
                let backtrack_coverages = s.read_array16(backtrack_count)?;
                let input_count = s.read::<u16>()?;
                let coverage = Coverage::parse(s.read_at_offset16(data)?)?;
                let input_coverages = s.read_array16(input_count.checked_sub(1)?)?;
                let lookahead_count = s.read::<u16>()?;
                let lookahead_coverages = s.read_array16(lookahead_count)?;
                let lookup_count = s.read::<u16>()?;
                let lookups = s.read_array16(lookup_count)?;
                Some(Self::Format3 {
                    coverage,
                    backtrack_coverages: LazyOffsetArray16::new(data, backtrack_coverages),
                    input_coverages: LazyOffsetArray16::new(data, input_coverages),
                    lookahead_coverages: LazyOffsetArray16::new(data, lookahead_coverages),
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

/// A list of [`ChainedSequenceRule`] sets.
pub type ChainedSequenceRuleSets<'a> = LazyOffsetArray16<'a, ChainedSequenceRuleSet<'a>>;

/// A set of [`ChainedSequenceRule`].
pub type ChainedSequenceRuleSet<'a> = LazyOffsetArray16<'a, ChainedSequenceRule<'a>>;

impl<'a> FromSlice<'a> for ChainedSequenceRuleSet<'a> {
    fn parse(data: &'a [u8]) -> Option<Self> {
        Self::parse(data)
    }
}

/// A [Chained Sequence Rule](https://docs.microsoft.com/en-us/typography/opentype/spec/chapter2#chained-sequence-context-format-1-simple-glyph-contexts).
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug)]
pub struct ChainedSequenceRule<'a> {
    /// Contains either glyph IDs or glyph Classes.
    pub backtrack: LazyArray16<'a, u16>,
    pub input: LazyArray16<'a, u16>,
    /// Contains either glyph IDs or glyph Classes.
    pub lookahead: LazyArray16<'a, u16>,
    pub lookups: LazyArray16<'a, SequenceLookupRecord>,
}

impl<'a> FromSlice<'a> for ChainedSequenceRule<'a> {
    fn parse(data: &'a [u8]) -> Option<Self> {
        let mut s = Stream::new(data);
        let backtrack_count = s.read::<u16>()?;
        let backtrack = s.read_array16(backtrack_count)?;
        let input_count = s.read::<u16>()?;
        let input = s.read_array16(input_count.checked_sub(1)?)?;
        let lookahead_count = s.read::<u16>()?;
        let lookahead = s.read_array16(lookahead_count)?;
        let lookup_count = s.read::<u16>()?;
        let lookups = s.read_array16(lookup_count)?;
        Some(Self {
            backtrack,
            input,
            lookahead,
            lookups,
        })
    }
}
