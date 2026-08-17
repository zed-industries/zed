use super::{Feature, FeatureIndex, RecordListItem, VariationIndex};
use crate::parser::{FromData, LazyArray16, LazyArray32};
use crate::parser::{Offset, Offset32, Stream};
use crate::{NormalizedCoordinate, Tag};

/// A [Feature Variations Table](https://docs.microsoft.com/en-us/typography/opentype/spec/chapter2#featurevariations-table).
#[derive(Clone, Copy, Debug)]
pub struct FeatureVariations<'a> {
    data: &'a [u8],
    records: LazyArray32<'a, FeatureVariationRecord>,
}

impl<'a> FeatureVariations<'a> {
    pub(crate) fn parse(data: &'a [u8]) -> Option<Self> {
        let mut s = Stream::new(data);
        let major_version = s.read::<u16>()?;
        s.skip::<u16>(); // minor version
        if major_version != 1 {
            return None;
        }

        let count = s.read::<u32>()?;
        let records = s.read_array32(count)?;
        Some(Self { data, records })
    }

    /// Returns a [`VariationIndex`] for variation coordinates.
    pub fn find_index(&self, coords: &[NormalizedCoordinate]) -> Option<VariationIndex> {
        for i in 0..self.records.len() {
            let record = self.records.get(i)?;
            let offset = record.conditions.to_usize();
            let set = ConditionSet::parse(self.data.get(offset..)?)?;
            if set.evaluate(coords) {
                return Some(i);
            }
        }
        None
    }

    /// Returns a [`Feature`] at specified indices.
    pub fn find_substitute(
        &self,
        feature_index: FeatureIndex,
        variation_index: VariationIndex,
    ) -> Option<Feature<'a>> {
        let offset = self.records.get(variation_index)?.substitutions.to_usize();
        let subst = FeatureTableSubstitution::parse(self.data.get(offset..)?)?;
        subst.find_substitute(feature_index)
    }
}

#[derive(Clone, Copy, Debug)]
struct FeatureVariationRecord {
    conditions: Offset32,
    substitutions: Offset32,
}

impl FromData for FeatureVariationRecord {
    const SIZE: usize = 8;

    #[inline]
    fn parse(data: &[u8]) -> Option<Self> {
        let mut s = Stream::new(data);
        Some(Self {
            conditions: s.read::<Offset32>()?,
            substitutions: s.read::<Offset32>()?,
        })
    }
}

#[derive(Clone, Copy, Debug)]
struct ConditionSet<'a> {
    data: &'a [u8],
    conditions: LazyArray16<'a, Offset32>,
}

impl<'a> ConditionSet<'a> {
    fn parse(data: &'a [u8]) -> Option<Self> {
        let mut s = Stream::new(data);
        let count = s.read::<u16>()?;
        let conditions = s.read_array16(count)?;
        Some(Self { data, conditions })
    }

    fn evaluate(&self, coords: &[NormalizedCoordinate]) -> bool {
        self.conditions.into_iter().all(|offset| {
            self.data
                .get(offset.to_usize()..)
                .and_then(Condition::parse)
                .map_or(false, |c| c.evaluate(coords))
        })
    }
}

#[derive(Clone, Copy, Debug)]
enum Condition {
    Format1 {
        axis_index: u16,
        filter_range_min: i16,
        filter_range_max: i16,
    },
}

impl Condition {
    fn parse(data: &[u8]) -> Option<Self> {
        let mut s = Stream::new(data);
        let format = s.read::<u16>()?;
        match format {
            1 => {
                let axis_index = s.read::<u16>()?;
                let filter_range_min = s.read::<i16>()?;
                let filter_range_max = s.read::<i16>()?;
                Some(Self::Format1 {
                    axis_index,
                    filter_range_min,
                    filter_range_max,
                })
            }
            _ => None,
        }
    }

    fn evaluate(&self, coords: &[NormalizedCoordinate]) -> bool {
        let Self::Format1 {
            axis_index,
            filter_range_min,
            filter_range_max,
        } = *self;
        let coord = coords
            .get(usize::from(axis_index))
            .map(|c| c.get())
            .unwrap_or(0);
        filter_range_min <= coord && coord <= filter_range_max
    }
}

#[derive(Clone, Copy, Debug)]
struct FeatureTableSubstitution<'a> {
    data: &'a [u8],
    records: LazyArray16<'a, FeatureTableSubstitutionRecord>,
}

impl<'a> FeatureTableSubstitution<'a> {
    fn parse(data: &'a [u8]) -> Option<Self> {
        let mut s = Stream::new(data);
        let major_version = s.read::<u16>()?;
        s.skip::<u16>(); // minor version
        if major_version != 1 {
            return None;
        }

        let count = s.read::<u16>()?;
        let records = s.read_array16(count)?;
        Some(Self { data, records })
    }

    fn find_substitute(&self, feature_index: FeatureIndex) -> Option<Feature<'a>> {
        for record in self.records {
            if record.feature_index == feature_index {
                let offset = record.feature.to_usize();
                // TODO: set tag
                return Feature::parse(Tag::from_bytes(b"DFLT"), self.data.get(offset..)?);
            }
        }
        None
    }
}

#[derive(Clone, Copy, Debug)]
struct FeatureTableSubstitutionRecord {
    feature_index: FeatureIndex,
    feature: Offset32,
}

impl FromData for FeatureTableSubstitutionRecord {
    const SIZE: usize = 6;

    #[inline]
    fn parse(data: &[u8]) -> Option<Self> {
        let mut s = Stream::new(data);
        Some(Self {
            feature_index: s.read::<FeatureIndex>()?,
            feature: s.read::<Offset32>()?,
        })
    }
}
