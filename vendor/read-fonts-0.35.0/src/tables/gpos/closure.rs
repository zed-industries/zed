//! support closure for GPOS

use super::{
    CursivePosFormat1, Gpos, MarkBasePosFormat1, MarkLigPosFormat1, MarkMarkPosFormat1, PairPos,
    PairPosFormat1, PairPosFormat2, PairSet, PositionLookup, PositionLookupList, PositionSubtables,
    SinglePos, SinglePosFormat1, SinglePosFormat2,
};
use crate::{collections::IntSet, GlyphId, ReadError, Tag};

#[cfg(feature = "std")]
use crate::tables::layout::{Intersect, LayoutLookupList, LookupClosure, LookupClosureCtx};

impl Gpos<'_> {
    /// Return a set of all feature indices underneath the specified scripts, languages and features
    pub fn collect_features(
        &self,
        scripts: &IntSet<Tag>,
        languages: &IntSet<Tag>,
        features: &IntSet<Tag>,
    ) -> Result<IntSet<u16>, ReadError> {
        let feature_list = self.feature_list()?;
        let script_list = self.script_list()?;
        let head_ptr = self.offset_data().as_bytes().as_ptr() as usize;
        script_list.collect_features(head_ptr, &feature_list, scripts, languages, features)
    }

    /// Return a set of lookups referenced by the specified features
    pub fn collect_lookups(&self, feature_indices: &IntSet<u16>) -> Result<IntSet<u16>, ReadError> {
        let feature_list = self.feature_list()?;
        let mut lookup_indices = feature_list.collect_lookups(feature_indices)?;

        if let Some(feature_variations) = self.feature_variations().transpose()? {
            let subs_lookup_indices = feature_variations.collect_lookups(feature_indices)?;
            lookup_indices.union(&subs_lookup_indices);
        }
        Ok(lookup_indices)
    }

    /// Update the set of lookup indices with all lookups reachable from specified glyph set and lookup_indices.
    pub fn closure_lookups(
        &self,
        glyphs: &IntSet<GlyphId>,
        lookup_indices: &mut IntSet<u16>,
    ) -> Result<(), ReadError> {
        let lookup_list = self.lookup_list()?;
        lookup_list.closure_lookups(glyphs, lookup_indices)
    }
}

impl PositionLookupList<'_> {
    pub fn closure_lookups(
        &self,
        glyph_set: &IntSet<GlyphId>,
        lookup_indices: &mut IntSet<u16>,
    ) -> Result<(), ReadError> {
        let lookup_list = LayoutLookupList::Gpos(self);
        let mut c = LookupClosureCtx::new(glyph_set, &lookup_list);

        let lookups = self.lookups();
        for idx in lookup_indices.iter() {
            let lookup = lookups.get(idx as usize)?;
            lookup.closure_lookups(&mut c, idx)?;
        }

        lookup_indices.union(c.visited_lookups());
        lookup_indices.subtract(c.inactive_lookups());
        Ok(())
    }
}

impl LookupClosure for PositionLookup<'_> {
    fn closure_lookups(
        &self,
        c: &mut LookupClosureCtx,
        lookup_index: u16,
    ) -> Result<(), ReadError> {
        if !c.should_visit_lookup(lookup_index) {
            return Ok(());
        }

        if !self.intersects(c.glyphs())? {
            c.set_lookup_inactive(lookup_index);
            return Ok(());
        }

        self.subtables()?.closure_lookups(c, lookup_index)
    }
}

impl Intersect for PositionLookup<'_> {
    fn intersects(&self, glyph_set: &IntSet<GlyphId>) -> Result<bool, ReadError> {
        self.subtables()?.intersects(glyph_set)
    }
}

impl LookupClosure for PositionSubtables<'_> {
    fn closure_lookups(&self, c: &mut LookupClosureCtx, arg: u16) -> Result<(), ReadError> {
        match self {
            PositionSubtables::Contextual(subtables) => subtables.closure_lookups(c, arg),
            PositionSubtables::ChainContextual(subtables) => subtables.closure_lookups(c, arg),
            _ => Ok(()),
        }
    }
}

impl Intersect for PositionSubtables<'_> {
    fn intersects(&self, glyph_set: &IntSet<GlyphId>) -> Result<bool, ReadError> {
        match self {
            PositionSubtables::Single(subtables) => subtables.intersects(glyph_set),
            PositionSubtables::Pair(subtables) => subtables.intersects(glyph_set),
            PositionSubtables::Cursive(subtables) => subtables.intersects(glyph_set),
            PositionSubtables::MarkToBase(subtables) => subtables.intersects(glyph_set),
            PositionSubtables::MarkToLig(subtables) => subtables.intersects(glyph_set),
            PositionSubtables::MarkToMark(subtables) => subtables.intersects(glyph_set),
            PositionSubtables::Contextual(subtables) => subtables.intersects(glyph_set),
            PositionSubtables::ChainContextual(subtables) => subtables.intersects(glyph_set),
        }
    }
}

impl Intersect for SinglePos<'_> {
    fn intersects(&self, glyph_set: &IntSet<GlyphId>) -> Result<bool, ReadError> {
        match self {
            Self::Format1(item) => item.intersects(glyph_set),
            Self::Format2(item) => item.intersects(glyph_set),
        }
    }
}

impl Intersect for SinglePosFormat1<'_> {
    fn intersects(&self, glyph_set: &IntSet<GlyphId>) -> Result<bool, ReadError> {
        Ok(self.coverage()?.intersects(glyph_set))
    }
}

impl Intersect for SinglePosFormat2<'_> {
    fn intersects(&self, glyph_set: &IntSet<GlyphId>) -> Result<bool, ReadError> {
        Ok(self.coverage()?.intersects(glyph_set))
    }
}

impl Intersect for PairPos<'_> {
    fn intersects(&self, glyph_set: &IntSet<GlyphId>) -> Result<bool, ReadError> {
        match self {
            Self::Format1(item) => item.intersects(glyph_set),
            Self::Format2(item) => item.intersects(glyph_set),
        }
    }
}

impl Intersect for PairPosFormat1<'_> {
    fn intersects(&self, glyph_set: &IntSet<GlyphId>) -> Result<bool, ReadError> {
        let coverage = self.coverage()?;
        let pair_sets = self.pair_sets();

        let num_pair_sets = self.pair_set_count();
        let num_bits = 16 - num_pair_sets.leading_zeros();
        if num_pair_sets as u64 > glyph_set.len() * num_bits as u64 {
            for g in glyph_set.iter() {
                let Some(i) = coverage.get(g) else {
                    continue;
                };

                let pair_set = pair_sets.get(i as usize)?;
                if pair_set.intersects(glyph_set)? {
                    return Ok(true);
                }
            }
        } else {
            for (g, pair_set) in coverage.iter().zip(pair_sets.iter()) {
                if !glyph_set.contains(GlyphId::from(g)) {
                    continue;
                }
                if pair_set?.intersects(glyph_set)? {
                    return Ok(true);
                }
            }
        }
        Ok(false)
    }
}

impl Intersect for PairSet<'_> {
    fn intersects(&self, glyph_set: &IntSet<GlyphId>) -> Result<bool, ReadError> {
        for record in self.pair_value_records().iter() {
            let second_glyph = record?.second_glyph();
            if glyph_set.contains(GlyphId::from(second_glyph)) {
                return Ok(true);
            }
        }
        Ok(false)
    }
}

impl Intersect for PairPosFormat2<'_> {
    fn intersects(&self, glyph_set: &IntSet<GlyphId>) -> Result<bool, ReadError> {
        Ok(self.coverage()?.intersects(glyph_set) && self.class_def2()?.intersects(glyph_set)?)
    }
}

impl Intersect for CursivePosFormat1<'_> {
    fn intersects(&self, glyph_set: &IntSet<GlyphId>) -> Result<bool, ReadError> {
        Ok(self.coverage()?.intersects(glyph_set))
    }
}

impl Intersect for MarkBasePosFormat1<'_> {
    fn intersects(&self, glyph_set: &IntSet<GlyphId>) -> Result<bool, ReadError> {
        Ok(self.mark_coverage()?.intersects(glyph_set)
            && self.base_coverage()?.intersects(glyph_set))
    }
}

impl Intersect for MarkLigPosFormat1<'_> {
    fn intersects(&self, glyph_set: &IntSet<GlyphId>) -> Result<bool, ReadError> {
        Ok(self.mark_coverage()?.intersects(glyph_set)
            && self.ligature_coverage()?.intersects(glyph_set))
    }
}

impl Intersect for MarkMarkPosFormat1<'_> {
    fn intersects(&self, glyph_set: &IntSet<GlyphId>) -> Result<bool, ReadError> {
        Ok(self.mark1_coverage()?.intersects(glyph_set)
            && self.mark2_coverage()?.intersects(glyph_set))
    }
}
