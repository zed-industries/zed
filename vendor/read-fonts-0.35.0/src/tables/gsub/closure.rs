//! Computing the closure over a set of glyphs
//!
//! This means taking a set of glyphs and updating it to include any other glyphs
//! reachable from those glyphs via substitution, recursively.
use font_types::GlyphId;

use crate::{
    collections::IntSet,
    tables::layout::{ExtensionLookup, Subtables},
    FontRead, ReadError, Tag,
};

use super::{
    AlternateSubstFormat1, ChainedSequenceContext, Gsub, Ligature, LigatureSet,
    LigatureSubstFormat1, MultipleSubstFormat1, ReverseChainSingleSubstFormat1, SequenceContext,
    SingleSubst, SingleSubstFormat1, SingleSubstFormat2, SubstitutionLookup,
    SubstitutionLookupList, SubstitutionSubtables,
};

#[cfg(feature = "std")]
use crate::tables::layout::{
    ContextFormat1, ContextFormat2, ContextFormat3, Intersect, LayoutLookupList, LookupClosure,
    LookupClosureCtx,
};

// we put ClosureCtx in its own module to enforce visibility rules;
// specifically we don't want cur_glyphs to be reachable directly
mod ctx {
    use std::collections::HashMap;
    use types::GlyphId;

    use crate::{
        collections::IntSet,
        tables::gsub::{SubstitutionLookup, SubstitutionLookupList},
    };

    use super::GlyphClosure as _;
    use super::ReadError;

    #[cfg(feature = "std")]
    use crate::tables::layout::{MAX_LOOKUP_VISIT_COUNT, MAX_NESTING_LEVEL};

    pub(super) struct ClosureCtx<'a> {
        /// the current closure glyphs. This is updated as we go.
        glyphs: &'a mut IntSet<GlyphId>,
        active_glyphs_stack: Vec<IntSet<GlyphId>>,
        output: IntSet<GlyphId>,
        lookup_count: u16,
        nesting_level_left: u8,
        done_lookups_glyphs: HashMap<u16, (u64, IntSet<GlyphId>)>,
    }

    impl<'a> ClosureCtx<'a> {
        pub(super) fn new(glyphs: &'a mut IntSet<GlyphId>) -> Self {
            Self {
                glyphs,
                active_glyphs_stack: Vec::new(),
                output: IntSet::empty(),
                lookup_count: 0,
                nesting_level_left: MAX_NESTING_LEVEL,
                done_lookups_glyphs: Default::default(),
            }
        }

        pub(super) fn lookup_limit_exceed(&self) -> bool {
            self.lookup_count > MAX_LOOKUP_VISIT_COUNT
        }

        pub(super) fn parent_active_glyphs(&self) -> &IntSet<GlyphId> {
            if self.active_glyphs_stack.is_empty() {
                return &*self.glyphs;
            }

            self.active_glyphs_stack.last().unwrap()
        }

        pub(super) fn push_cur_active_glyphs(&mut self, glyphs: IntSet<GlyphId>) {
            self.active_glyphs_stack.push(glyphs)
        }

        pub(super) fn pop_cur_done_glyphs(&mut self) {
            self.active_glyphs_stack.pop();
        }

        pub(super) fn recurse(
            &mut self,
            lookup_list: &SubstitutionLookupList,
            lookup: &SubstitutionLookup,
            lookup_index: u16,
            glyphs: IntSet<GlyphId>,
        ) -> Result<(), ReadError> {
            if self.nesting_level_left == 0 {
                return Ok(());
            }

            self.nesting_level_left -= 1;
            self.push_cur_active_glyphs(glyphs);

            lookup.closure_glyphs(self, lookup_list, lookup_index)?;
            self.nesting_level_left += 1;
            self.pop_cur_done_glyphs();

            Ok(())
        }

        pub(super) fn reset_lookup_visit_count(&mut self) {
            self.lookup_count = 0;
        }

        pub(super) fn should_visit_lookup(&mut self, lookup_index: u16) -> bool {
            if self.lookup_limit_exceed() {
                return false;
            }
            self.lookup_count += 1;
            !self.is_lookup_done(lookup_index)
        }

        // Return true if we have visited this lookup with current set of glyphs
        pub(super) fn is_lookup_done(&mut self, lookup_index: u16) -> bool {
            {
                let (count, covered) = self
                    .done_lookups_glyphs
                    .entry(lookup_index)
                    .or_insert((0, IntSet::empty()));

                if *count != self.glyphs.len() {
                    *count = self.glyphs.len();
                    covered.clear();
                }
            }

            let mut cur_glyphs = IntSet::empty();
            {
                let covered = &self.done_lookups_glyphs.get(&lookup_index).unwrap().1;
                //TODO: add IntSet::is_subset
                if self
                    .parent_active_glyphs()
                    .iter()
                    .all(|g| covered.contains(g))
                {
                    return true;
                }
                cur_glyphs.extend(self.parent_active_glyphs().iter());
            }

            let (_, covered) = self.done_lookups_glyphs.get_mut(&lookup_index).unwrap();
            covered.union(&cur_glyphs);
            false
        }

        pub(super) fn glyphs(&self) -> &IntSet<GlyphId> {
            self.glyphs
        }

        pub(super) fn add(&mut self, gid: GlyphId) {
            self.output.insert(gid);
        }

        pub(super) fn add_glyphs(&mut self, iter: impl IntoIterator<Item = GlyphId>) {
            self.output.extend(iter)
        }

        pub(super) fn flush(&mut self) {
            self.glyphs.union(&self.output);
            self.output.clear();
            self.active_glyphs_stack.clear();
        }
    }
}

use ctx::ClosureCtx;

/// A trait for tables which participate in closure
trait GlyphClosure {
    /// Update the set of glyphs with any glyphs reachable via substitution.
    fn closure_glyphs(
        &self,
        ctx: &mut ClosureCtx,
        lookup_list: &SubstitutionLookupList,
        lookup_index: u16,
    ) -> Result<(), ReadError>;

    fn may_have_non_1to1(&self) -> Result<bool, ReadError> {
        Ok(false)
    }
}

const CLOSURE_MAX_STAGES: u8 = 12;
impl Gsub<'_> {
    /// Return the set of glyphs reachable from the input set via any substitution.
    /// ref: <https://github.com/harfbuzz/harfbuzz/blob/8d517f7e43f648cb804c46c47ae8009330fe4a47/src/hb-ot-layout.cc#L1616>
    pub fn closure_glyphs(
        &self,
        lookups: &IntSet<u16>,
        glyphs: &mut IntSet<GlyphId>,
    ) -> Result<(), ReadError> {
        let lookup_list = self.lookup_list()?;
        let num_lookups = lookup_list.lookup_count();
        let lookup_offsets = lookup_list.lookups();

        let mut ctx = ClosureCtx::new(glyphs);
        let mut iteration_count = 0;
        let mut glyphs_length;
        loop {
            ctx.reset_lookup_visit_count();
            glyphs_length = ctx.glyphs().len();

            if lookups.is_inverted() {
                for i in 0..num_lookups {
                    if !lookups.contains(i) {
                        continue;
                    }
                    let lookup = lookup_offsets.get(i as usize)?;
                    lookup.closure_glyphs(&mut ctx, &lookup_list, i)?;
                    ctx.flush();
                }
            } else {
                for i in lookups.iter() {
                    let lookup = lookup_offsets.get(i as usize)?;
                    lookup.closure_glyphs(&mut ctx, &lookup_list, i)?;
                    ctx.flush();
                }
            }
            if iteration_count > CLOSURE_MAX_STAGES || glyphs_length == ctx.glyphs().len() {
                break;
            }
            iteration_count += 1;
        }
        Ok(())
    }

    /// Return a set of lookups referenced by the specified features
    ///
    /// Pass `&IntSet::all()` to get the lookups referenced by all features.
    pub fn collect_lookups(&self, feature_indices: &IntSet<u16>) -> Result<IntSet<u16>, ReadError> {
        let feature_list = self.feature_list()?;
        let mut lookup_indices = feature_list.collect_lookups(feature_indices)?;

        if let Some(feature_variations) = self.feature_variations().transpose()? {
            let subs_lookup_indices = feature_variations.collect_lookups(feature_indices)?;
            lookup_indices.union(&subs_lookup_indices);
        }
        Ok(lookup_indices)
    }

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

//ref: <https://github.com/harfbuzz/harfbuzz/blob/8d517f7e43f648cb804c46c47ae8009330fe4a47/src/OT/Layout/GSUB/SubstLookup.hh#L50>
impl GlyphClosure for SubstitutionLookup<'_> {
    fn closure_glyphs(
        &self,
        ctx: &mut ClosureCtx,
        lookup_list: &SubstitutionLookupList,
        lookup_index: u16,
    ) -> Result<(), ReadError> {
        if !ctx.should_visit_lookup(lookup_index) {
            return Ok(());
        }
        self.subtables()?
            .closure_glyphs(ctx, lookup_list, lookup_index)
    }

    fn may_have_non_1to1(&self) -> Result<bool, ReadError> {
        self.subtables()?.may_have_non_1to1()
    }
}

impl GlyphClosure for SubstitutionSubtables<'_> {
    fn closure_glyphs(
        &self,
        ctx: &mut ClosureCtx,
        lookup_list: &SubstitutionLookupList,
        lookup_index: u16,
    ) -> Result<(), ReadError> {
        match self {
            SubstitutionSubtables::Single(tables) => {
                tables.closure_glyphs(ctx, lookup_list, lookup_index)
            }
            SubstitutionSubtables::Multiple(tables) => {
                tables.closure_glyphs(ctx, lookup_list, lookup_index)
            }
            SubstitutionSubtables::Alternate(tables) => {
                tables.closure_glyphs(ctx, lookup_list, lookup_index)
            }
            SubstitutionSubtables::Ligature(tables) => {
                tables.closure_glyphs(ctx, lookup_list, lookup_index)
            }
            SubstitutionSubtables::Reverse(tables) => {
                tables.closure_glyphs(ctx, lookup_list, lookup_index)
            }
            SubstitutionSubtables::Contextual(tables) => {
                tables.closure_glyphs(ctx, lookup_list, lookup_index)
            }
            SubstitutionSubtables::ChainContextual(tables) => {
                tables.closure_glyphs(ctx, lookup_list, lookup_index)
            }
        }
    }

    fn may_have_non_1to1(&self) -> Result<bool, ReadError> {
        match self {
            SubstitutionSubtables::Single(_) => Ok(false),
            SubstitutionSubtables::Multiple(_) => Ok(true),
            SubstitutionSubtables::Alternate(_) => Ok(false),
            SubstitutionSubtables::Ligature(_) => Ok(true),
            SubstitutionSubtables::Reverse(_) => Ok(false),
            SubstitutionSubtables::Contextual(_) => Ok(true),
            SubstitutionSubtables::ChainContextual(_) => Ok(true),
        }
    }
}

impl<'a, T: FontRead<'a> + GlyphClosure + 'a, Ext: ExtensionLookup<'a, T> + 'a> GlyphClosure
    for Subtables<'a, T, Ext>
{
    fn closure_glyphs(
        &self,
        ctx: &mut ClosureCtx,
        lookup_list: &SubstitutionLookupList,
        lookup_index: u16,
    ) -> Result<(), ReadError> {
        self.iter()
            .try_for_each(|t| t?.closure_glyphs(ctx, lookup_list, lookup_index))
    }
}

impl GlyphClosure for SingleSubst<'_> {
    fn closure_glyphs(
        &self,
        ctx: &mut ClosureCtx,
        lookup_list: &SubstitutionLookupList,
        lookup_index: u16,
    ) -> Result<(), ReadError> {
        match self {
            SingleSubst::Format1(t) => t.closure_glyphs(ctx, lookup_list, lookup_index),
            SingleSubst::Format2(t) => t.closure_glyphs(ctx, lookup_list, lookup_index),
        }
    }
}

// ref: <https://github.com/harfbuzz/harfbuzz/blob/8d517f7e43f648cb804c46c47ae8009330fe4a47/src/OT/Layout/GSUB/SingleSubstFormat1.hh#L48>
impl GlyphClosure for SingleSubstFormat1<'_> {
    fn closure_glyphs(
        &self,
        ctx: &mut ClosureCtx,
        _lookup_list: &SubstitutionLookupList,
        _lookup_index: u16,
    ) -> Result<(), ReadError> {
        let coverage = self.coverage()?;
        let num_glyphs = coverage.population();
        let mask = u16::MAX;
        // ref: <https://github.com/harfbuzz/harfbuzz/blob/fbf5b2aa035d6cd9b796d74252045e2b7156ad02/src/OT/Layout/GSUB/SingleSubstFormat1.hh#L55>
        if num_glyphs >= mask as usize {
            return Ok(());
        }

        let intersection = coverage.intersect_set(ctx.parent_active_glyphs());
        if intersection.is_empty() {
            return Ok(());
        }

        // help fuzzer
        // ref: <https://github.com/harfbuzz/harfbuzz/blob/fbf5b2aa035d6cd9b796d74252045e2b7156ad02/src/OT/Layout/GSUB/SingleSubstFormat1.hh#L61>
        let d = self.delta_glyph_id() as i32;
        let mask = mask as i32;
        let min_before = intersection.first().unwrap().to_u32() as i32;
        let max_before = intersection.last().unwrap().to_u32() as i32;
        let min_after = (min_before + d) & mask;
        let max_after = (max_before + d) & mask;

        if intersection.len() == (max_before - min_before + 1) as u64
            && ((min_before <= min_after && min_after <= max_before)
                || (min_before <= max_after && max_after <= max_before))
        {
            return Ok(());
        }

        for g in intersection.iter() {
            let new_g = (g.to_u32() as i32 + d) & mask;
            ctx.add(GlyphId::from(new_g as u32));
        }
        Ok(())
    }
}

impl GlyphClosure for SingleSubstFormat2<'_> {
    fn closure_glyphs(
        &self,
        ctx: &mut ClosureCtx,
        _lookup_list: &SubstitutionLookupList,
        _lookup_index: u16,
    ) -> Result<(), ReadError> {
        let coverage = self.coverage()?;
        let glyph_set = ctx.parent_active_glyphs();
        let subs_glyphs = self.substitute_glyph_ids();

        let new_glyphs: Vec<GlyphId> = if self.glyph_count() as u64 > glyph_set.len() {
            glyph_set
                .iter()
                .filter_map(|g| coverage.get(g))
                .filter_map(|idx| {
                    subs_glyphs
                        .get(idx as usize)
                        .map(|new_g| GlyphId::from(new_g.get()))
                })
                .collect()
        } else {
            coverage
                .iter()
                .zip(subs_glyphs)
                .filter(|&(g, _)| glyph_set.contains(GlyphId::from(g)))
                .map(|(_, &new_g)| GlyphId::from(new_g.get()))
                .collect()
        };
        ctx.add_glyphs(new_glyphs);
        Ok(())
    }
}

impl GlyphClosure for MultipleSubstFormat1<'_> {
    fn closure_glyphs(
        &self,
        ctx: &mut ClosureCtx,
        _lookup_list: &SubstitutionLookupList,
        _lookup_index: u16,
    ) -> Result<(), ReadError> {
        let coverage = self.coverage()?;
        let glyph_set = ctx.parent_active_glyphs();
        let sequences = self.sequences();

        let new_glyphs: Vec<GlyphId> = if self.sequence_count() as u64 > glyph_set.len() {
            glyph_set
                .iter()
                .filter_map(|g| coverage.get(g))
                .filter_map(|idx| sequences.get(idx as usize).ok())
                .flat_map(|seq| {
                    seq.substitute_glyph_ids()
                        .iter()
                        .map(|new_g| GlyphId::from(new_g.get()))
                })
                .collect()
        } else {
            coverage
                .iter()
                .zip(sequences.iter())
                .filter_map(|(g, seq)| {
                    glyph_set
                        .contains(GlyphId::from(g))
                        .then(|| seq.ok())
                        .flatten()
                })
                .flat_map(|seq| {
                    seq.substitute_glyph_ids()
                        .iter()
                        .map(|new_g| GlyphId::from(new_g.get()))
                })
                .collect()
        };

        ctx.add_glyphs(new_glyphs);
        Ok(())
    }
}

impl GlyphClosure for AlternateSubstFormat1<'_> {
    fn closure_glyphs(
        &self,
        ctx: &mut ClosureCtx,
        _lookup_list: &SubstitutionLookupList,
        _lookup_index: u16,
    ) -> Result<(), ReadError> {
        let coverage = self.coverage()?;
        let glyph_set = ctx.parent_active_glyphs();
        let alts = self.alternate_sets();

        let new_glyphs: Vec<GlyphId> = if self.alternate_set_count() as u64 > glyph_set.len() {
            glyph_set
                .iter()
                .filter_map(|g| coverage.get(g))
                .filter_map(|idx| alts.get(idx as usize).ok())
                .flat_map(|alt_set| {
                    alt_set
                        .alternate_glyph_ids()
                        .iter()
                        .map(|new_g| GlyphId::from(new_g.get()))
                })
                .collect()
        } else {
            coverage
                .iter()
                .zip(alts.iter())
                .filter_map(|(g, alt_set)| {
                    glyph_set
                        .contains(GlyphId::from(g))
                        .then(|| alt_set.ok())
                        .flatten()
                })
                .flat_map(|alt_set| {
                    alt_set
                        .alternate_glyph_ids()
                        .iter()
                        .map(|new_g| GlyphId::from(new_g.get()))
                })
                .collect()
        };

        ctx.add_glyphs(new_glyphs);
        Ok(())
    }
}

impl GlyphClosure for LigatureSubstFormat1<'_> {
    fn closure_glyphs(
        &self,
        ctx: &mut ClosureCtx,
        _lookup_list: &SubstitutionLookupList,
        _lookup_index: u16,
    ) -> Result<(), ReadError> {
        let coverage = self.coverage()?;
        let ligs = self.ligature_sets();
        let lig_set_idxes: Vec<usize> =
            if self.ligature_set_count() as u64 > ctx.parent_active_glyphs().len() {
                ctx.parent_active_glyphs()
                    .iter()
                    .filter_map(|g| coverage.get(g))
                    .map(|idx| idx as usize)
                    .collect()
            } else {
                coverage
                    .iter()
                    .enumerate()
                    .filter(|&(_idx, g)| ctx.parent_active_glyphs().contains(GlyphId::from(g)))
                    .map(|(idx, _)| idx)
                    .collect()
            };

        for idx in lig_set_idxes {
            let lig_set = ligs.get(idx)?;
            for lig in lig_set.ligatures().iter() {
                let lig = lig?;
                if lig.intersects(ctx.glyphs())? {
                    ctx.add(GlyphId::from(lig.ligature_glyph()));
                }
            }
        }
        Ok(())
    }
}

impl GlyphClosure for ReverseChainSingleSubstFormat1<'_> {
    fn closure_glyphs(
        &self,
        ctx: &mut ClosureCtx,
        _lookup_list: &SubstitutionLookupList,
        _lookup_index: u16,
    ) -> Result<(), ReadError> {
        if !self.intersects(ctx.glyphs())? {
            return Ok(());
        }

        let coverage = self.coverage()?;
        let glyph_set = ctx.parent_active_glyphs();
        let idxes: Vec<usize> = if self.glyph_count() as u64 > glyph_set.len() {
            glyph_set
                .iter()
                .filter_map(|g| coverage.get(g))
                .map(|idx| idx as usize)
                .collect()
        } else {
            coverage
                .iter()
                .enumerate()
                .filter(|&(_idx, g)| glyph_set.contains(GlyphId::from(g)))
                .map(|(idx, _)| idx)
                .collect()
        };

        let sub_glyphs = self.substitute_glyph_ids();
        for i in idxes {
            let Some(g) = sub_glyphs.get(i) else {
                continue;
            };
            ctx.add(GlyphId::from(g.get()));
        }

        Ok(())
    }
}

impl GlyphClosure for SequenceContext<'_> {
    fn closure_glyphs(
        &self,
        ctx: &mut ClosureCtx,
        lookup_list: &SubstitutionLookupList,
        lookup_index: u16,
    ) -> Result<(), ReadError> {
        match self {
            Self::Format1(table) => {
                ContextFormat1::Plain(table.clone()).closure_glyphs(ctx, lookup_list, lookup_index)
            }
            Self::Format2(table) => {
                ContextFormat2::Plain(table.clone()).closure_glyphs(ctx, lookup_list, lookup_index)
            }
            Self::Format3(table) => {
                ContextFormat3::Plain(table.clone()).closure_glyphs(ctx, lookup_list, lookup_index)
            }
        }
    }
}

impl GlyphClosure for ChainedSequenceContext<'_> {
    fn closure_glyphs(
        &self,
        ctx: &mut ClosureCtx,
        lookup_list: &SubstitutionLookupList,
        lookup_index: u16,
    ) -> Result<(), ReadError> {
        match self {
            Self::Format1(table) => {
                ContextFormat1::Chain(table.clone()).closure_glyphs(ctx, lookup_list, lookup_index)
            }
            Self::Format2(table) => {
                ContextFormat2::Chain(table.clone()).closure_glyphs(ctx, lookup_list, lookup_index)
            }
            Self::Format3(table) => {
                ContextFormat3::Chain(table.clone()).closure_glyphs(ctx, lookup_list, lookup_index)
            }
        }
    }
}

//https://github.com/fonttools/fonttools/blob/a6f59a4f8/Lib/fontTools/subset/__init__.py#L1182
impl GlyphClosure for ContextFormat1<'_> {
    fn closure_glyphs(
        &self,
        ctx: &mut ClosureCtx,
        lookup_list: &SubstitutionLookupList,
        _lookup_index: u16,
    ) -> Result<(), ReadError> {
        let coverage = self.coverage()?;
        let cov_active_glyphs = coverage.intersect_set(ctx.parent_active_glyphs());
        if cov_active_glyphs.is_empty() {
            return Ok(());
        }

        for (gid, rule_set) in coverage
            .iter()
            .zip(self.rule_sets())
            .filter_map(|(g, rule_set)| {
                rule_set
                    .filter(|_| cov_active_glyphs.contains(GlyphId::from(g)))
                    .map(|rs| (g, rs))
            })
        {
            if ctx.lookup_limit_exceed() {
                return Ok(());
            }

            for rule in rule_set?.rules() {
                if ctx.lookup_limit_exceed() {
                    return Ok(());
                }
                let rule = rule?;
                if !rule.intersects(ctx.glyphs())? {
                    continue;
                }

                let input_seq = rule.input_sequence();
                let input_count = input_seq.len() + 1;
                // python calls this 'chaos'. Basically: if there are multiple
                // lookups applied at a single position they can interact, and
                // we can no longer trivially determine the state of the context
                // at that point. In this case we give up, and assume that the
                // second lookup is reachable by all glyphs.
                let mut seen_sequence_indices = IntSet::new();

                for lookup_record in rule.lookup_records() {
                    let sequence_idx = lookup_record.sequence_index();
                    if sequence_idx as usize >= input_count {
                        continue;
                    }

                    let mut active_glyphs = IntSet::empty();
                    if !seen_sequence_indices.insert(sequence_idx) {
                        // During processing, when we see an empty set we will replace
                        // it with the full current glyph set
                        active_glyphs.extend(ctx.glyphs().iter());
                    } else if sequence_idx == 0 {
                        active_glyphs.insert(GlyphId::from(gid));
                    } else {
                        let g = input_seq[sequence_idx as usize - 1].get();
                        active_glyphs.insert(GlyphId::from(g));
                    };

                    let lookup_index = lookup_record.lookup_list_index();
                    let lookup = lookup_list.lookups().get(lookup_index as usize)?;
                    if lookup.may_have_non_1to1()? {
                        seen_sequence_indices.insert_range(sequence_idx..=input_count as u16);
                    }
                    ctx.recurse(lookup_list, &lookup, lookup_index, active_glyphs)?;
                }
            }
        }
        Ok(())
    }
}

//https://github.com/fonttools/fonttools/blob/a6f59a4f87a0111/Lib/fontTools/subset/__init__.py#L1215
impl GlyphClosure for ContextFormat2<'_> {
    fn closure_glyphs(
        &self,
        ctx: &mut ClosureCtx,
        lookup_list: &SubstitutionLookupList,
        _lookup_index: u16,
    ) -> Result<(), ReadError> {
        let coverage = self.coverage()?;
        let cov_active_glyphs = coverage.intersect_set(ctx.parent_active_glyphs());
        if cov_active_glyphs.is_empty() {
            return Ok(());
        }

        let input_class_def = self.input_class_def()?;
        let coverage_glyph_classes = input_class_def.intersect_classes(&cov_active_glyphs);

        let input_glyph_classes = input_class_def.intersect_classes(ctx.glyphs());
        let backtrack_classes = match self {
            Self::Plain(_) => IntSet::empty(),
            Self::Chain(table) => table.backtrack_class_def()?.intersect_classes(ctx.glyphs()),
        };
        let lookahead_classes = match self {
            Self::Plain(_) => IntSet::empty(),
            Self::Chain(table) => table.lookahead_class_def()?.intersect_classes(ctx.glyphs()),
        };

        for (i, rule_set) in self
            .rule_sets()
            .enumerate()
            .filter_map(|(class, rs)| rs.map(|rs| (class as u16, rs)))
            .filter(|&(class, _)| coverage_glyph_classes.contains(class))
        {
            if ctx.lookup_limit_exceed() {
                return Ok(());
            }

            for rule in rule_set?.rules() {
                if ctx.lookup_limit_exceed() {
                    return Ok(());
                }
                let rule = rule?;
                if !rule.intersects(&input_glyph_classes, &backtrack_classes, &lookahead_classes) {
                    continue;
                }

                let input_seq = rule.input_sequence();
                let input_count = input_seq.len() + 1;

                let mut seen_sequence_indices = IntSet::new();

                for lookup_record in rule.lookup_records() {
                    let sequence_idx = lookup_record.sequence_index();
                    if sequence_idx as usize >= input_count {
                        continue;
                    }

                    let active_glyphs = if !seen_sequence_indices.insert(sequence_idx) {
                        ctx.glyphs().clone()
                    } else if sequence_idx == 0 {
                        input_class_def.intersected_class_glyphs(ctx.parent_active_glyphs(), i)
                    } else {
                        let c = input_seq[sequence_idx as usize - 1].get();
                        input_class_def.intersected_class_glyphs(ctx.parent_active_glyphs(), c)
                    };

                    let lookup_index = lookup_record.lookup_list_index();
                    let lookup = lookup_list.lookups().get(lookup_index as usize)?;
                    if lookup.may_have_non_1to1()? {
                        seen_sequence_indices.insert_range(sequence_idx..=input_count as u16);
                    }
                    ctx.recurse(lookup_list, &lookup, lookup_index, active_glyphs)?;
                }
            }
        }
        Ok(())
    }
}

impl GlyphClosure for ContextFormat3<'_> {
    fn closure_glyphs(
        &self,
        ctx: &mut ClosureCtx,
        lookup_list: &SubstitutionLookupList,
        _lookup_index: u16,
    ) -> Result<(), ReadError> {
        if !self.intersects(ctx.glyphs())? {
            return Ok(());
        }

        let mut seen_sequence_indices = IntSet::new();
        let input_coverages = self.coverages();
        let input_count = input_coverages.len();
        for record in self.lookup_records() {
            let seq_idx = record.sequence_index();
            if seq_idx as usize >= input_count {
                continue;
            }

            let active_glyphs = if !seen_sequence_indices.insert(seq_idx) {
                ctx.glyphs().clone()
            } else if seq_idx == 0 {
                let cov = input_coverages.get(0)?;
                cov.intersect_set(ctx.parent_active_glyphs())
            } else {
                let cov = input_coverages.get(seq_idx as usize)?;
                cov.intersect_set(ctx.glyphs())
            };

            let lookup_index = record.lookup_list_index();
            let lookup = lookup_list.lookups().get(lookup_index as usize)?;
            if lookup.may_have_non_1to1()? {
                seen_sequence_indices.insert_range(seq_idx..=input_count as u16);
            }
            ctx.recurse(lookup_list, &lookup, lookup_index, active_glyphs)?;
        }
        Ok(())
    }
}

impl SubstitutionLookupList<'_> {
    pub fn closure_lookups(
        &self,
        glyph_set: &IntSet<GlyphId>,
        lookup_indices: &mut IntSet<u16>,
    ) -> Result<(), ReadError> {
        let lookup_list = LayoutLookupList::Gsub(self);
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

impl LookupClosure for SubstitutionLookup<'_> {
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

impl Intersect for SubstitutionLookup<'_> {
    fn intersects(&self, glyph_set: &IntSet<GlyphId>) -> Result<bool, ReadError> {
        self.subtables()?.intersects(glyph_set)
    }
}

impl LookupClosure for SubstitutionSubtables<'_> {
    fn closure_lookups(&self, c: &mut LookupClosureCtx, arg: u16) -> Result<(), ReadError> {
        match self {
            SubstitutionSubtables::ChainContextual(subtables) => subtables.closure_lookups(c, arg),
            SubstitutionSubtables::Contextual(subtables) => subtables.closure_lookups(c, arg),
            _ => Ok(()),
        }
    }
}

impl Intersect for SubstitutionSubtables<'_> {
    fn intersects(&self, glyph_set: &IntSet<GlyphId>) -> Result<bool, ReadError> {
        match self {
            SubstitutionSubtables::Single(subtables) => subtables.intersects(glyph_set),
            SubstitutionSubtables::Multiple(subtables) => subtables.intersects(glyph_set),
            SubstitutionSubtables::Alternate(subtables) => subtables.intersects(glyph_set),
            SubstitutionSubtables::Ligature(subtables) => subtables.intersects(glyph_set),
            SubstitutionSubtables::Contextual(subtables) => subtables.intersects(glyph_set),
            SubstitutionSubtables::ChainContextual(subtables) => subtables.intersects(glyph_set),
            SubstitutionSubtables::Reverse(subtables) => subtables.intersects(glyph_set),
        }
    }
}

impl Intersect for SingleSubst<'_> {
    fn intersects(&self, glyph_set: &IntSet<GlyphId>) -> Result<bool, ReadError> {
        match self {
            Self::Format1(item) => item.intersects(glyph_set),
            Self::Format2(item) => item.intersects(glyph_set),
        }
    }
}

impl Intersect for SingleSubstFormat1<'_> {
    fn intersects(&self, glyph_set: &IntSet<GlyphId>) -> Result<bool, ReadError> {
        Ok(self.coverage()?.intersects(glyph_set))
    }
}

impl Intersect for SingleSubstFormat2<'_> {
    fn intersects(&self, glyph_set: &IntSet<GlyphId>) -> Result<bool, ReadError> {
        Ok(self.coverage()?.intersects(glyph_set))
    }
}

impl Intersect for MultipleSubstFormat1<'_> {
    fn intersects(&self, glyph_set: &IntSet<GlyphId>) -> Result<bool, ReadError> {
        Ok(self.coverage()?.intersects(glyph_set))
    }
}

impl Intersect for AlternateSubstFormat1<'_> {
    fn intersects(&self, glyph_set: &IntSet<GlyphId>) -> Result<bool, ReadError> {
        Ok(self.coverage()?.intersects(glyph_set))
    }
}

impl Intersect for LigatureSubstFormat1<'_> {
    fn intersects(&self, glyph_set: &IntSet<GlyphId>) -> Result<bool, ReadError> {
        let coverage = self.coverage()?;
        let lig_sets = self.ligature_sets();
        for lig_set in coverage
            .iter()
            .zip(lig_sets.iter())
            .filter_map(|(g, lig_set)| glyph_set.contains(GlyphId::from(g)).then_some(lig_set))
        {
            if lig_set?.intersects(glyph_set)? {
                return Ok(true);
            }
        }
        Ok(false)
    }
}

impl Intersect for LigatureSet<'_> {
    fn intersects(&self, glyph_set: &IntSet<GlyphId>) -> Result<bool, ReadError> {
        let ligs = self.ligatures();
        for lig in ligs.iter() {
            if lig?.intersects(glyph_set)? {
                return Ok(true);
            }
        }
        Ok(false)
    }
}

impl Intersect for Ligature<'_> {
    fn intersects(&self, glyph_set: &IntSet<GlyphId>) -> Result<bool, ReadError> {
        let ret = self
            .component_glyph_ids()
            .iter()
            .all(|g| glyph_set.contains(GlyphId::from(g.get())));
        Ok(ret)
    }
}

impl Intersect for ReverseChainSingleSubstFormat1<'_> {
    fn intersects(&self, glyph_set: &IntSet<GlyphId>) -> Result<bool, ReadError> {
        if !self.coverage()?.intersects(glyph_set) {
            return Ok(false);
        }

        for coverage in self.backtrack_coverages().iter() {
            if !coverage?.intersects(glyph_set) {
                return Ok(false);
            }
        }

        for coverage in self.lookahead_coverages().iter() {
            if !coverage?.intersects(glyph_set) {
                return Ok(false);
            }
        }
        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};

    use crate::{FontRef, TableProvider};

    use super::*;
    use font_test_data::closure as test_data;

    struct GlyphMap {
        to_gid: HashMap<&'static str, GlyphId>,
        from_gid: HashMap<GlyphId, &'static str>,
    }

    impl GlyphMap {
        fn new(raw_order: &'static str) -> GlyphMap {
            let to_gid: HashMap<_, _> = raw_order
                .split('\n')
                .map(|line| line.trim())
                .filter(|line| !(line.starts_with('#') || line.is_empty()))
                .enumerate()
                .map(|(gid, name)| (name, GlyphId::new(gid.try_into().unwrap())))
                .collect();
            let from_gid = to_gid.iter().map(|(name, gid)| (*gid, *name)).collect();
            GlyphMap { from_gid, to_gid }
        }

        fn get_gid(&self, name: &str) -> Option<GlyphId> {
            self.to_gid.get(name).copied()
        }

        fn get_name(&self, gid: GlyphId) -> Option<&str> {
            self.from_gid.get(&gid).copied()
        }
    }

    fn get_gsub(test_data: &'static [u8]) -> Gsub<'static> {
        let font = FontRef::new(test_data).unwrap();
        font.gsub().unwrap()
    }

    fn compute_closure(gsub: &Gsub, glyph_map: &GlyphMap, input: &[&str]) -> IntSet<GlyphId> {
        let lookup_indices = gsub.collect_lookups(&IntSet::all()).unwrap();
        let mut input_glyphs = input
            .iter()
            .map(|name| glyph_map.get_gid(name).unwrap())
            .collect();
        gsub.closure_glyphs(&lookup_indices, &mut input_glyphs)
            .unwrap();
        input_glyphs
    }

    /// assert a set of glyph ids matches a slice of names
    macro_rules! assert_closure_result {
        ($glyph_map:expr, $result:expr, $expected:expr) => {
            let result = $result
                .iter()
                .map(|gid| $glyph_map.get_name(gid).unwrap())
                .collect::<HashSet<_>>();
            let expected = $expected.iter().copied().collect::<HashSet<_>>();
            if expected != result {
                let in_output = result.difference(&expected).collect::<Vec<_>>();
                let in_expected = expected.difference(&result).collect::<Vec<_>>();
                let mut msg = format!("Closure output does not match\n");
                if !in_expected.is_empty() {
                    msg.push_str(format!("missing {in_expected:?}\n").as_str());
                }
                if !in_output.is_empty() {
                    msg.push_str(format!("unexpected {in_output:?}").as_str());
                }
                panic!("{msg}")
            }
        };
    }

    #[test]
    fn smoke_test() {
        // tests various lookup types.
        // test input is font-test-data/test_data/fea/simple_closure.fea
        let gsub = get_gsub(test_data::SIMPLE);
        let glyph_map = GlyphMap::new(test_data::SIMPLE_GLYPHS);
        let result = compute_closure(&gsub, &glyph_map, &["a"]);

        assert_closure_result!(
            glyph_map,
            result,
            &["a", "A", "b", "c", "d", "a_a", "a.1", "a.2", "a.3"]
        );
    }

    #[test]
    fn recursive() {
        // a scenario in which one substitution adds glyphs that trigger additional
        // substitutions.
        //
        // test input is font-test-data/test_data/fea/recursive_closure.fea
        let gsub = get_gsub(test_data::RECURSIVE);
        let glyph_map = GlyphMap::new(test_data::RECURSIVE_GLYPHS);
        let result = compute_closure(&gsub, &glyph_map, &["a"]);
        assert_closure_result!(glyph_map, result, &["a", "b", "c", "d"]);
    }

    #[test]
    fn contextual_lookups_nop() {
        let gsub = get_gsub(test_data::CONTEXTUAL);
        let glyph_map = GlyphMap::new(test_data::CONTEXTUAL_GLYPHS);

        // these match the lookups but not the context
        let nop = compute_closure(&gsub, &glyph_map, &["three", "four", "e", "f"]);
        assert_closure_result!(glyph_map, nop, &["three", "four", "e", "f"]);
    }

    #[test]
    fn contextual_lookups_chained_f1() {
        let gsub = get_gsub(test_data::CONTEXTUAL);
        let glyph_map = GlyphMap::new(test_data::CONTEXTUAL_GLYPHS);
        let gsub6f1 = compute_closure(
            &gsub,
            &glyph_map,
            &["one", "two", "three", "four", "five", "six", "seven"],
        );
        assert_closure_result!(
            glyph_map,
            gsub6f1,
            &["one", "two", "three", "four", "five", "six", "seven", "X", "Y"]
        );
    }

    #[test]
    fn contextual_lookups_chained_f3() {
        let gsub = get_gsub(test_data::CONTEXTUAL);
        let glyph_map = GlyphMap::new(test_data::CONTEXTUAL_GLYPHS);
        let gsub6f3 = compute_closure(&gsub, &glyph_map, &["space", "e"]);
        assert_closure_result!(glyph_map, gsub6f3, &["space", "e", "e.2"]);

        let gsub5f3 = compute_closure(&gsub, &glyph_map, &["f", "g"]);
        assert_closure_result!(glyph_map, gsub5f3, &["f", "g", "f.2"]);
    }

    #[test]
    fn contextual_plain_f1() {
        let gsub = get_gsub(test_data::CONTEXTUAL);
        let glyph_map = GlyphMap::new(test_data::CONTEXTUAL_GLYPHS);
        let gsub5f1 = compute_closure(&gsub, &glyph_map, &["a", "b"]);
        assert_closure_result!(glyph_map, gsub5f1, &["a", "b", "a_b"]);
    }

    #[test]
    fn contextual_plain_f3() {
        let gsub = get_gsub(test_data::CONTEXTUAL);
        let glyph_map = GlyphMap::new(test_data::CONTEXTUAL_GLYPHS);
        let gsub5f3 = compute_closure(&gsub, &glyph_map, &["f", "g"]);
        assert_closure_result!(glyph_map, gsub5f3, &["f", "g", "f.2"]);
    }

    #[test]
    fn recursive_context() {
        let gsub = get_gsub(test_data::RECURSIVE_CONTEXTUAL);
        let glyph_map = GlyphMap::new(test_data::RECURSIVE_CONTEXTUAL_GLYPHS);

        let nop = compute_closure(&gsub, &glyph_map, &["b", "B"]);
        assert_closure_result!(glyph_map, nop, &["b", "B"]);

        let full = compute_closure(&gsub, &glyph_map, &["a", "b", "c"]);
        assert_closure_result!(glyph_map, full, &["a", "b", "c", "B", "B.2", "B.3"]);

        let intermediate = compute_closure(&gsub, &glyph_map, &["a", "B.2"]);
        assert_closure_result!(glyph_map, intermediate, &["a", "B.2", "B.3"]);
    }

    #[test]
    fn feature_variations() {
        let gsub = get_gsub(test_data::VARIATIONS_CLOSURE);
        let glyph_map = GlyphMap::new(test_data::VARIATIONS_GLYPHS);

        let input = compute_closure(&gsub, &glyph_map, &["a"]);
        assert_closure_result!(glyph_map, input, &["a", "b", "c"]);
    }

    #[test]
    fn chain_context_format3() {
        let gsub = get_gsub(test_data::CHAIN_CONTEXT_FORMAT3_BITS);
        let glyph_map = GlyphMap::new(test_data::CHAIN_CONTEXT_FORMAT3_BITS_GLYPHS);

        let nop = compute_closure(&gsub, &glyph_map, &["c", "z"]);
        assert_closure_result!(glyph_map, nop, &["c", "z"]);

        let full = compute_closure(&gsub, &glyph_map, &["a", "b", "c", "z"]);
        assert_closure_result!(glyph_map, full, &["a", "b", "c", "z", "A", "B"]);
    }

    #[test]
    fn closure_ignore_unreachable_glyphs() {
        let font = FontRef::new(font_test_data::closure::CONTEXT_ONLY_REACHABLE).unwrap();
        let gsub = font.gsub().unwrap();
        let glyph_map = GlyphMap::new(test_data::CONTEXT_ONLY_REACHABLE_GLYPHS);
        let result = compute_closure(&gsub, &glyph_map, &["a", "b", "c", "d", "e", "f", "period"]);
        assert_closure_result!(
            glyph_map,
            result,
            &["a", "b", "c", "d", "e", "f", "period", "A", "B", "C"]
        );
    }

    #[test]
    fn cyclical_context() {
        let gsub = get_gsub(test_data::CYCLIC_CONTEXTUAL);
        let glyph_map = GlyphMap::new(test_data::RECURSIVE_CONTEXTUAL_GLYPHS);
        // we mostly care that this terminates
        let nop = compute_closure(&gsub, &glyph_map, &["a", "b", "c"]);
        assert_closure_result!(glyph_map, nop, &["a", "b", "c"]);
    }

    #[test]
    fn collect_all_features() {
        let font = FontRef::new(font_test_data::closure::CONTEXTUAL).unwrap();
        let gsub = font.gsub().unwrap();
        let ret = gsub
            .collect_features(&IntSet::all(), &IntSet::all(), &IntSet::all())
            .unwrap();
        assert_eq!(ret.len(), 2);
        assert!(ret.contains(0));
        assert!(ret.contains(1));
    }

    #[test]
    fn collect_all_features_with_feature_filter() {
        let font = FontRef::new(font_test_data::closure::CONTEXTUAL).unwrap();
        let gsub = font.gsub().unwrap();

        let mut feature_tags = IntSet::empty();
        feature_tags.insert(Tag::new(b"SUB5"));

        let ret = gsub
            .collect_features(&IntSet::all(), &IntSet::all(), &feature_tags)
            .unwrap();
        assert_eq!(ret.len(), 1);
        assert!(ret.contains(0));
    }

    #[test]
    fn collect_all_features_with_script_filter() {
        let font = FontRef::new(font_test_data::closure::CONTEXTUAL).unwrap();
        let gsub = font.gsub().unwrap();

        let mut script_tags = IntSet::empty();
        script_tags.insert(Tag::new(b"LATN"));

        let ret = gsub
            .collect_features(&script_tags, &IntSet::all(), &IntSet::all())
            .unwrap();
        assert!(ret.is_empty());
    }
}
