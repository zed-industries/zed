use super::{coverage_binary_cached, coverage_index, covered, glyph_class};
use crate::hb::buffer::GlyphInfo;
use crate::hb::ot::{ClassDefInfo, CoverageInfo};
use crate::hb::ot_layout_gsubgpos::OT::hb_ot_apply_context_t;
use crate::hb::ot_layout_gsubgpos::{
    apply_lookup, match_always, match_backtrack, match_glyph, match_input, match_lookahead,
    may_skip_t, skipping_iterator_t, Apply, BinaryCache, ChainContextFormat2Cache,
    ContextFormat2Cache, SubtableExternalCache, SubtableExternalCacheMode, WouldApply,
    WouldApplyContext,
};
use read_fonts::tables::gsub::ClassDef;
use read_fonts::tables::layout::{
    ChainedClassSequenceRule, ChainedSequenceContextFormat1, ChainedSequenceContextFormat2,
    ChainedSequenceContextFormat3, ChainedSequenceRule, ClassSequenceRule, SequenceContextFormat1,
    SequenceContextFormat2, SequenceContextFormat3, SequenceLookupRecord, SequenceRule,
};
use read_fonts::types::{BigEndian, GlyphId, GlyphId16, Offset16};
use read_fonts::{ArrayOfOffsets, FontRead};

impl WouldApply for SequenceContextFormat1<'_> {
    fn would_apply(&self, ctx: &WouldApplyContext) -> bool {
        coverage_index(self.coverage(), ctx.glyphs[0])
            .and_then(|index| {
                self.seq_rule_sets()
                    .get(index as usize)
                    .transpose()
                    .ok()
                    .flatten()
            })
            .is_some_and(|set| {
                set.seq_rules().iter().any(|rule| {
                    rule.map(|rule| {
                        let input = rule.input_sequence();
                        ctx.glyphs.len() == input.len() + 1
                            && input.iter().enumerate().all(|(i, value)| {
                                let mut info = GlyphInfo {
                                    glyph_id: ctx.glyphs[i + 1].into(),
                                    ..GlyphInfo::default()
                                };
                                match_glyph(&mut info, value.get().to_u16())
                            })
                    })
                    .unwrap_or(false)
                })
            })
    }
}

impl Apply for SequenceContextFormat1<'_> {
    fn apply(&self, ctx: &mut hb_ot_apply_context_t) -> Option<()> {
        let glyph = ctx.buffer.cur(0).as_glyph();
        let index = self.coverage().ok()?.get(glyph)? as usize;
        let set = self.seq_rule_sets().get(index)?.ok()?;
        apply_context_rules(ctx, &set.seq_rules(), match_glyph)
    }
}

impl WouldApply for SequenceContextFormat2<'_> {
    fn would_apply(&self, ctx: &WouldApplyContext) -> bool {
        let class_def = self.class_def().ok();
        let match_fn = &match_class(&class_def);
        let class = glyph_class(self.class_def(), ctx.glyphs[0]);
        self.class_seq_rule_sets()
            .get(class as usize)
            .transpose()
            .ok()
            .flatten()
            .is_some_and(|set| {
                set.class_seq_rules().iter().any(|rule| {
                    rule.map(|rule| {
                        let input = rule.input_sequence();
                        ctx.glyphs.len() == input.len() + 1
                            && input.iter().enumerate().all(|(i, value)| {
                                let mut info = GlyphInfo {
                                    glyph_id: ctx.glyphs[i + 1].into(),
                                    ..GlyphInfo::default()
                                };
                                match_fn(&mut info, value.get())
                            })
                    })
                    .unwrap_or(false)
                })
            })
    }
}

impl Apply for SequenceContextFormat2<'_> {
    fn apply_with_external_cache(
        &self,
        ctx: &mut hb_ot_apply_context_t,
        external_cache: &SubtableExternalCache,
    ) -> Option<()> {
        let glyph = ctx.buffer.cur(0).as_glyph();
        let SubtableExternalCache::ContextFormat2Cache(cache) = external_cache else {
            return None;
        };
        let offset_data = self.offset_data();
        coverage_binary_cached(
            |gid| cache.coverage.index(&offset_data, gid),
            glyph,
            &cache.coverage_cache,
        )?;
        let input_class = |gid| cache.input.class(&offset_data, gid);
        let index = input_class(glyph) as usize;
        let set = self.class_seq_rule_sets().get(index)?.ok()?;
        apply_context_rules(ctx, &set.class_seq_rules(), |info, value| {
            input_class(info.as_glyph()) == value
        })
    }

    fn apply_cached(
        &self,
        ctx: &mut hb_ot_apply_context_t,
        external_cache: &SubtableExternalCache,
    ) -> Option<()> {
        let glyph = ctx.buffer.cur(0).as_glyph();
        let SubtableExternalCache::ContextFormat2Cache(cache) = external_cache else {
            return None;
        };
        let offset_data = self.offset_data();
        coverage_binary_cached(
            |gid| cache.coverage.index(&offset_data, gid),
            glyph,
            &cache.coverage_cache,
        )?;
        let input_class = |gid| cache.input.class(&offset_data, gid);
        let index = get_class_cached(&input_class, &mut ctx.buffer.info[ctx.buffer.idx]) as usize;
        let set = self.class_seq_rule_sets().get(index)?.ok()?;
        apply_context_rules(
            ctx,
            &set.class_seq_rules(),
            match_class_cached(&input_class),
        )
    }

    fn cache_cost(&self) -> u32 {
        self.class_def()
            .ok()
            .map_or(0, |class_def| class_def.cost())
    }

    fn external_cache_create(&self, _mode: SubtableExternalCacheMode) -> SubtableExternalCache {
        let data = self.offset_data();
        SubtableExternalCache::ContextFormat2Cache(ContextFormat2Cache {
            coverage_cache: BinaryCache::new(),
            coverage: CoverageInfo::new(&data, self.coverage_offset().to_u32() as u16)
                .unwrap_or_default(),
            input: ClassDefInfo::new(&data, self.class_def_offset().to_u32() as u16)
                .unwrap_or_default(),
        })
    }
}

impl WouldApply for SequenceContextFormat3<'_> {
    fn would_apply(&self, ctx: &WouldApplyContext) -> bool {
        let coverages = self.coverages();
        ctx.glyphs.len() == coverages.len() + 1
            && coverages
                .iter()
                .enumerate()
                .all(|(i, coverage)| covered(coverage, ctx.glyphs[i + 1]))
    }
}

impl Apply for SequenceContextFormat3<'_> {
    fn apply(&self, ctx: &mut hb_ot_apply_context_t) -> Option<()> {
        let glyph = ctx.buffer.cur(0).as_glyph();
        let input_coverages = self.coverages();
        input_coverages.get(0).ok()?.get(glyph)?;
        let input = |info: &mut GlyphInfo, index: u16| {
            input_coverages
                .get(index as usize + 1)
                .is_ok_and(|cov| cov.get(info.glyph_id).is_some())
        };
        let mut match_end = 0;
        if match_input(
            ctx,
            input_coverages.len() as u16 - 1,
            input,
            &mut match_end,
            None,
        ) {
            ctx.buffer
                .unsafe_to_break(Some(ctx.buffer.idx), Some(match_end));
            apply_lookup(
                ctx,
                input_coverages.len() - 1,
                match_end,
                self.seq_lookup_records(),
            );
            Some(())
        } else {
            ctx.buffer
                .unsafe_to_concat(Some(ctx.buffer.idx), Some(match_end));
            None
        }
    }
}

impl WouldApply for ChainedSequenceContextFormat1<'_> {
    fn would_apply(&self, ctx: &WouldApplyContext) -> bool {
        coverage_index(self.coverage(), ctx.glyphs[0])
            .and_then(|index| {
                self.chained_seq_rule_sets()
                    .get(index as usize)
                    .transpose()
                    .ok()
                    .flatten()
            })
            .is_some_and(|set| {
                set.chained_seq_rules().iter().any(|rule| {
                    rule.map(|rule| {
                        let input = rule.input_sequence();
                        (!ctx.zero_context
                            || (rule.backtrack_glyph_count() == 0
                                && rule.lookahead_glyph_count() == 0))
                            && ctx.glyphs.len() == input.len() + 1
                            && input.iter().enumerate().all(|(i, value)| {
                                let mut info = GlyphInfo {
                                    glyph_id: ctx.glyphs[i + 1].into(),
                                    ..GlyphInfo::default()
                                };
                                match_glyph(&mut info, value.get().to_u16())
                            })
                    })
                    .unwrap_or(false)
                })
            })
    }
}

impl Apply for ChainedSequenceContextFormat1<'_> {
    fn apply(&self, ctx: &mut hb_ot_apply_context_t) -> Option<()> {
        let glyph = ctx.buffer.cur(0).as_glyph();
        let index = self.coverage().ok()?.get(glyph)? as usize;
        let set = self.chained_seq_rule_sets().get(index)?.ok()?;
        apply_chain_context_rules(
            ctx,
            &set.chained_seq_rules(),
            (match_glyph, match_glyph, match_glyph),
        )
    }
}

impl WouldApply for ChainedSequenceContextFormat2<'_> {
    fn would_apply(&self, ctx: &WouldApplyContext) -> bool {
        let class_def = self.input_class_def().ok();
        let match_fn = &match_class(&class_def);
        let class = glyph_class(self.input_class_def(), ctx.glyphs[0]);
        self.chained_class_seq_rule_sets()
            .get(class as usize)
            .transpose()
            .ok()
            .flatten()
            .is_some_and(|set| {
                set.chained_class_seq_rules().iter().any(|rule| {
                    rule.map(|rule| {
                        let input = rule.input_sequence();
                        (!ctx.zero_context
                            || (rule.backtrack_glyph_count() == 0
                                && rule.lookahead_glyph_count() == 0))
                            && ctx.glyphs.len() == input.len() + 1
                            && input.iter().enumerate().all(|(i, value)| {
                                let mut info = GlyphInfo {
                                    glyph_id: ctx.glyphs[i + 1].into(),
                                    ..GlyphInfo::default()
                                };
                                match_fn(&mut info, value.get())
                            })
                    })
                    .unwrap_or(false)
                })
            })
    }
}

/// Value represents glyph class.
fn match_class<'a>(
    class_def: &'a Option<ClassDef<'a>>,
) -> impl Fn(&mut GlyphInfo, u16) -> bool + 'a {
    |&mut info, value| {
        class_def
            .as_ref()
            .is_some_and(|class_def| class_def.get(info.as_glyph()) == value)
    }
}

fn get_class_cached(class_def: &impl Fn(GlyphId) -> u16, info: &mut GlyphInfo) -> u16 {
    let mut klass = info.syllable() as u16;
    if klass < 255 {
        return klass;
    }
    klass = class_def(info.as_glyph());
    if klass < 255 {
        info.set_syllable(klass as u8);
    }

    klass
}

fn match_class_cached<'a>(
    class_def: impl Fn(GlyphId) -> u16 + 'a,
) -> impl Fn(&mut GlyphInfo, u16) -> bool + 'a {
    move |info: &mut GlyphInfo, value| get_class_cached(&class_def, info) == value
}

fn get_class_cached1(class_def: &impl Fn(GlyphId) -> u16, info: &mut GlyphInfo) -> u16 {
    let mut klass = (info.syllable() & 0x0F) as u16;
    if klass < 15 {
        return klass;
    }

    klass = class_def(info.as_glyph());

    if klass < 15 {
        info.set_syllable((info.syllable() & 0xF0) | klass as u8);
    }

    klass
}

fn match_class_cached1<'a>(
    class_def: impl Fn(GlyphId) -> u16 + 'a,
) -> impl Fn(&mut GlyphInfo, u16) -> bool + 'a {
    move |info: &mut GlyphInfo, value| get_class_cached1(&class_def, info) == value
}

fn get_class_cached2(class_def: &impl Fn(GlyphId) -> u16, info: &mut GlyphInfo) -> u16 {
    let mut klass = (info.syllable() & 0xF0) as u16 >> 4;
    if klass < 15 {
        return klass;
    }
    klass = class_def(info.as_glyph());
    if klass < 15 {
        info.set_syllable((info.syllable() & 0x0F) | ((klass as u8) << 4));
    }
    klass
}

fn match_class_cached2<'a>(
    class_def: impl Fn(GlyphId) -> u16 + 'a,
) -> impl Fn(&mut GlyphInfo, u16) -> bool + 'a {
    move |info: &mut GlyphInfo, value| get_class_cached2(&class_def, info) == value
}

impl Apply for ChainedSequenceContextFormat2<'_> {
    fn apply_with_external_cache(
        &self,
        ctx: &mut hb_ot_apply_context_t,
        external_cache: &SubtableExternalCache,
    ) -> Option<()> {
        let glyph = ctx.buffer.cur(0).as_glyph();
        let SubtableExternalCache::ChainContextFormat2Cache(cache) = external_cache else {
            return None;
        };
        let offset_data = self.offset_data();
        coverage_binary_cached(
            |gid| cache.coverage.index(&offset_data, gid),
            glyph,
            &cache.coverage_cache,
        )?;
        let index = cache.input.class(&offset_data, glyph) as usize;
        let set = self.chained_class_seq_rule_sets().get(index)?.ok()?;
        apply_chain_context_rules(
            ctx,
            &set.chained_class_seq_rules(),
            (
                |info, val| cache.backtrack.class(&offset_data, info.as_glyph()) == val,
                |info, val| cache.input.class(&offset_data, info.as_glyph()) == val,
                |info, val| cache.lookahead.class(&offset_data, info.as_glyph()) == val,
            ),
        )
    }
    fn apply_cached(
        &self,
        ctx: &mut hb_ot_apply_context_t,
        external_cache: &SubtableExternalCache,
    ) -> Option<()> {
        let glyph = ctx.buffer.cur(0).as_glyph();
        let SubtableExternalCache::ChainContextFormat2Cache(cache) = external_cache else {
            return None;
        };
        let offset_data = self.offset_data();
        coverage_binary_cached(
            |gid| cache.coverage.index(&offset_data, gid),
            glyph,
            &cache.coverage_cache,
        )?;
        let input_class = |gid| cache.input.class(&offset_data, gid);
        let lookahead_class = |gid| cache.lookahead.class(&offset_data, gid);
        let index = get_class_cached2(&input_class, &mut ctx.buffer.info[ctx.buffer.idx]) as usize;
        let set = self.chained_class_seq_rule_sets().get(index)?.ok()?;
        apply_chain_context_rules(
            ctx,
            &set.chained_class_seq_rules(),
            (
                |info, val| cache.backtrack.class(&offset_data, info.as_glyph()) == val,
                match_class_cached2(&input_class),
                match_class_cached1(&lookahead_class),
            ),
        )
    }
    fn cache_cost(&self) -> u32 {
        self.input_class_def()
            .ok()
            .map_or(0, |class_def| class_def.cost())
            + self
                .lookahead_class_def()
                .ok()
                .map_or(0, |class_def| class_def.cost())
    }

    fn external_cache_create(&self, _mode: SubtableExternalCacheMode) -> SubtableExternalCache {
        let data = self.offset_data();
        SubtableExternalCache::ChainContextFormat2Cache(ChainContextFormat2Cache {
            coverage_cache: BinaryCache::new(),
            coverage: CoverageInfo::new(&data, self.coverage_offset().to_u32() as u16)
                .unwrap_or_default(),
            backtrack: ClassDefInfo::new(&data, self.backtrack_class_def_offset().to_u32() as u16)
                .unwrap_or_default(),
            input: ClassDefInfo::new(&data, self.input_class_def_offset().to_u32() as u16)
                .unwrap_or_default(),
            lookahead: ClassDefInfo::new(&data, self.lookahead_class_def_offset().to_u32() as u16)
                .unwrap_or_default(),
        })
    }
}

impl WouldApply for ChainedSequenceContextFormat3<'_> {
    fn would_apply(&self, ctx: &WouldApplyContext) -> bool {
        let input_coverages = self.input_coverages();
        (!ctx.zero_context
            || (self.backtrack_coverage_offsets().is_empty()
                && self.lookahead_coverage_offsets().is_empty()))
            && (ctx.glyphs.len() == input_coverages.len()
                && input_coverages
                    .iter()
                    .skip(1)
                    .enumerate()
                    .all(|(i, coverage)| {
                        coverage
                            .map(|cov| cov.get(ctx.glyphs[i + 1]).is_some())
                            .unwrap_or(false)
                    }))
    }
}

impl Apply for ChainedSequenceContextFormat3<'_> {
    fn apply(&self, ctx: &mut hb_ot_apply_context_t) -> Option<()> {
        let glyph = ctx.buffer.cur(0).as_glyph();

        let input_coverages = self.input_coverages();
        input_coverages.get(0).ok()?.get(glyph)?;

        let backtrack_coverages = self.backtrack_coverages();
        let lookahead_coverages = self.lookahead_coverages();

        let back = |info: &mut GlyphInfo, index: u16| {
            backtrack_coverages
                .get(index as usize)
                .is_ok_and(|cov| cov.get(info.glyph_id).is_some())
        };

        let ahead = |info: &mut GlyphInfo, index: u16| {
            lookahead_coverages
                .get(index as usize)
                .is_ok_and(|cov| cov.get(info.glyph_id).is_some())
        };

        let input = |info: &mut GlyphInfo, index: u16| {
            input_coverages
                .get(index as usize + 1)
                .is_ok_and(|cov| cov.get(info.glyph_id).is_some())
        };

        let mut end_index = ctx.buffer.idx;
        let mut match_end = 0;

        let input_matches = match_input(
            ctx,
            input_coverages.len() as u16 - 1,
            input,
            &mut match_end,
            None,
        );

        if input_matches {
            end_index = match_end;
        }

        if !(input_matches
            && match_lookahead(
                ctx,
                lookahead_coverages.len() as u16,
                ahead,
                match_end,
                &mut end_index,
            ))
        {
            ctx.buffer
                .unsafe_to_concat(Some(ctx.buffer.idx), Some(end_index));
            return None;
        }

        let mut start_index = ctx.buffer.out_len;

        if !match_backtrack(
            ctx,
            backtrack_coverages.len() as u16,
            back,
            &mut start_index,
        ) {
            ctx.buffer
                .unsafe_to_concat_from_outbuffer(Some(start_index), Some(end_index));
            return None;
        }

        ctx.buffer
            .unsafe_to_break_from_outbuffer(Some(start_index), Some(end_index));
        apply_lookup(
            ctx,
            input_coverages.len() - 1,
            match_end,
            self.seq_lookup_records(),
        );

        Some(())
    }
}

trait ToU16: Copy {
    fn to_u16(self) -> u16;
}

impl ToU16 for BigEndian<GlyphId16> {
    fn to_u16(self) -> u16 {
        self.get().to_u16()
    }
}

impl ToU16 for BigEndian<u16> {
    fn to_u16(self) -> u16 {
        self.get()
    }
}

trait ContextRule<'a>: FontRead<'a> {
    type Input: ToU16 + 'a;

    fn input(&self) -> &'a [Self::Input];
    fn lookup_records(&self) -> &'a [SequenceLookupRecord];

    fn apply(
        &self,
        ctx: &mut hb_ot_apply_context_t,
        match_func: &impl Fn(&mut GlyphInfo, u16) -> bool,
    ) -> Option<()> {
        let inputs = self.input();
        let match_func = |info: &mut GlyphInfo, index| {
            let value = inputs.get(index as usize).unwrap().to_u16();
            match_func(info, value)
        };

        let mut match_end = 0;

        if match_input(ctx, inputs.len() as _, match_func, &mut match_end, None) {
            ctx.buffer
                .unsafe_to_break(Some(ctx.buffer.idx), Some(match_end));
            apply_lookup(ctx, inputs.len(), match_end, self.lookup_records());
            return Some(());
        }
        None
    }
}

impl<'a> ContextRule<'a> for SequenceRule<'a> {
    type Input = BigEndian<GlyphId16>;

    fn input(&self) -> &'a [Self::Input] {
        self.input_sequence()
    }

    fn lookup_records(&self) -> &'a [SequenceLookupRecord] {
        self.seq_lookup_records()
    }
}

impl<'a> ContextRule<'a> for ClassSequenceRule<'a> {
    type Input = BigEndian<u16>;

    fn input(&self) -> &'a [Self::Input] {
        self.input_sequence()
    }

    fn lookup_records(&self) -> &'a [SequenceLookupRecord] {
        self.seq_lookup_records()
    }
}

fn apply_context_rules<'a, 'b, R: ContextRule<'a>>(
    ctx: &mut hb_ot_apply_context_t,
    rules: &'b ArrayOfOffsets<'a, R, Offset16>,
    match_func: impl Fn(&mut GlyphInfo, u16) -> bool,
) -> Option<()> {
    // TODO: In HarfBuzz, the following condition makes NotoNastaliqUrdu
    // faster. But our lookup code is slower, so NOT using this condition
    // makes us faster.  Reconsider when lookup code is faster.
    //if rules.len() <= 4 {
    if false {
        for rule in rules.iter().filter_map(|r| r.ok()) {
            if rule.apply(ctx, &match_func).is_some() {
                return Some(());
            };
        }
        return None;
    }
    // This version is optimized for speed by matching the first & second
    // components of the rule here, instead of calling into the matching code.
    //
    // We use the iter_context instead of iter_input, to avoid skipping
    // default-ignorables and such.
    //
    // Related: https://github.com/harfbuzz/harfbuzz/issues/4813
    let mut skippy_iter = skipping_iterator_t::with_match_fn(ctx, true, Some(match_always));
    skippy_iter.reset(skippy_iter.buffer.idx);
    skippy_iter.set_glyph_data(0);
    let mut unsafe_to = None;
    let unsafe_to1;
    let mut unsafe_to2 = 0;
    let mut second = None;
    let first = if skippy_iter.next(None) {
        let g1 = skippy_iter.index();
        if skippy_iter.may_skip(&skippy_iter.buffer.info[g1]) != may_skip_t::SKIP_NO {
            // Can't use the fast path if eg. the next char is a default-ignorable
            // or other skippable.
            for rule in rules.iter().filter_map(|r| r.ok()) {
                if rule.apply(ctx, &match_func).is_some() {
                    return Some(());
                };
            }
            return None;
        }
        unsafe_to1 = skippy_iter.index() + 1;
        g1
    } else {
        // Failed to match a next glyph. Only try applying rules that have no
        // further impact.
        for rule in rules
            .iter()
            .filter_map(|r| r.ok())
            .filter(|r| r.input().len() <= 1)
        {
            if rule.apply(ctx, &match_func).is_some() {
                return Some(());
            };
        }
        return None;
    };
    let matched = skippy_iter.next(None);
    let g2 = skippy_iter.index();
    if matched {
        second = Some(g2);
        unsafe_to2 = skippy_iter.index() + 1;
        if skippy_iter.may_skip(&skippy_iter.buffer.info[g2]) != may_skip_t::SKIP_NO {
            // Can't use the fast path if eg. the next char is a default-ignorable
            // or other skippable.
            for rule in rules.iter().filter_map(|r| r.ok()) {
                if rule.apply(ctx, &match_func).is_some() {
                    return Some(());
                };
            }
            return None;
        }
    }
    let mut rules_iter = rules.iter().filter_map(|r| r.ok());
    let mut rule_box = rules_iter.next();
    loop {
        let Some(rule) = rule_box else {
            break;
        };
        let inputs = rule.input();
        let match_func2 = |info: &mut GlyphInfo, index| {
            if let Some(value) = inputs.get(index as usize).map(|v| v.to_u16()) {
                match_func(info, value)
            } else {
                false
            }
        };
        if inputs.len() <= 1 || match_func2(&mut ctx.buffer.info[first], 0) {
            if second.is_none()
                || (inputs.len() <= 2 || match_func2(&mut ctx.buffer.info[second.unwrap()], 1))
            {
                if rule.apply(ctx, &match_func).is_some() {
                    if let Some(unsafe_to) = unsafe_to {
                        ctx.buffer
                            .unsafe_to_concat(Some(ctx.buffer.idx), Some(unsafe_to));
                    }
                    return Some(());
                }
            } else {
                unsafe_to = Some(unsafe_to2);
            }
            rule_box = rules_iter.next();
        } else {
            if unsafe_to.is_none() {
                unsafe_to = Some(unsafe_to1);
            }

            // Skip ahead to next possible first glyph match.
            let first_glyph_value = inputs.first().unwrap().to_u16();
            loop {
                let next_rule_box = rules_iter.next();
                if next_rule_box.is_none() {
                    rule_box = None;
                    break;
                };

                let next_inputs = next_rule_box.as_ref().unwrap().input();
                if next_inputs.is_empty()
                    || next_inputs.first().unwrap().to_u16() != first_glyph_value
                {
                    rule_box = next_rule_box;
                    break;
                }
            }
        }
    }
    if let Some(unsafe_to) = unsafe_to {
        ctx.buffer
            .unsafe_to_concat(Some(ctx.buffer.idx), Some(unsafe_to));
    }
    None
}

trait ChainContextRule<'a>: ContextRule<'a> {
    fn backtrack(&self) -> &'a [Self::Input];
    fn lookahead(&self) -> &'a [Self::Input];
}

impl<'a> ContextRule<'a> for ChainedSequenceRule<'a> {
    type Input = BigEndian<GlyphId16>;

    fn input(&self) -> &'a [Self::Input] {
        self.input_sequence()
    }

    fn lookup_records(&self) -> &'a [SequenceLookupRecord] {
        self.seq_lookup_records()
    }
}

impl<'a> ChainContextRule<'a> for ChainedSequenceRule<'a> {
    fn backtrack(&self) -> &'a [Self::Input] {
        self.backtrack_sequence()
    }

    fn lookahead(&self) -> &'a [Self::Input] {
        self.lookahead_sequence()
    }
}

impl<'a> ContextRule<'a> for ChainedClassSequenceRule<'a> {
    type Input = BigEndian<u16>;

    fn input(&self) -> &'a [Self::Input] {
        self.input_sequence()
    }

    fn lookup_records(&self) -> &'a [SequenceLookupRecord] {
        self.seq_lookup_records()
    }
}

impl<'a> ChainContextRule<'a> for ChainedClassSequenceRule<'a> {
    fn backtrack(&self) -> &'a [Self::Input] {
        self.backtrack_sequence()
    }

    fn lookahead(&self) -> &'a [Self::Input] {
        self.lookahead_sequence()
    }
}

fn apply_chain_with_sequences<
    'a,
    T: ToU16 + 'a,
    F1: Fn(&mut GlyphInfo, u16) -> bool,
    F2: Fn(&mut GlyphInfo, u16) -> bool,
    F3: Fn(&mut GlyphInfo, u16) -> bool,
>(
    ctx: &mut hb_ot_apply_context_t,
    backtrack: &'a [T],
    input: &'a [T],
    lookahead: &'a [T],
    lookup_records: &'a [SequenceLookupRecord],
    match_funcs: &(F1, F2, F3),
) -> Option<()> {
    let f3 = |info: &mut GlyphInfo, index| {
        let value = (*input.get(index as usize).unwrap()).to_u16();
        match_funcs.1(info, value)
    };

    let mut end_index = ctx.buffer.idx;
    let mut match_end = 0;

    let input_matches = match_input(ctx, input.len() as u16, f3, &mut match_end, None);

    if input_matches {
        end_index = match_end;
    } else {
        ctx.buffer
            .unsafe_to_concat(Some(ctx.buffer.idx), Some(end_index));
        return None;
    }

    let f2 = |info: &mut GlyphInfo, index| {
        let value = (*lookahead.get(index as usize).unwrap()).to_u16();
        match_funcs.2(info, value)
    };

    if !match_lookahead(ctx, lookahead.len() as u16, f2, match_end, &mut end_index) {
        ctx.buffer
            .unsafe_to_concat(Some(ctx.buffer.idx), Some(end_index));
        return None;
    }

    let mut start_index = ctx.buffer.out_len;

    let f1 = |info: &mut GlyphInfo, index| {
        let value = (*backtrack.get(index as usize).unwrap()).to_u16();
        match_funcs.0(info, value)
    };

    if !match_backtrack(ctx, backtrack.len() as u16, f1, &mut start_index) {
        ctx.buffer
            .unsafe_to_concat_from_outbuffer(Some(start_index), Some(end_index));
        return None;
    }

    ctx.buffer
        .unsafe_to_break_from_outbuffer(Some(start_index), Some(end_index));
    apply_lookup(ctx, input.len(), match_end, lookup_records);

    Some(())
}

fn apply_chain_context_rules<
    'a,
    'b,
    R: ChainContextRule<'a>,
    F1: Fn(&mut GlyphInfo, u16) -> bool,
    F2: Fn(&mut GlyphInfo, u16) -> bool,
    F3: Fn(&mut GlyphInfo, u16) -> bool,
>(
    ctx: &mut hb_ot_apply_context_t,
    rules: &'b ArrayOfOffsets<'a, R, Offset16>,
    match_funcs: (F1, F2, F3),
) -> Option<()> {
    if rules.len() <= 4 {
        for rule in rules.iter().filter_map(|r| r.ok()) {
            if apply_chain_with_sequences(
                ctx,
                rule.backtrack(),
                rule.input(),
                rule.lookahead(),
                rule.lookup_records(),
                &match_funcs,
            )
            .is_some()
            {
                return Some(());
            };
        }
        return None;
    }
    // This version is optimized for speed by matching the first & second
    // components of the rule here, instead of calling into the matching code.
    //
    // We use the iter_context instead of iter_input, to avoid skipping
    // default-ignorables and such.
    //
    // Related: https://github.com/harfbuzz/harfbuzz/issues/4813
    let mut skippy_iter = skipping_iterator_t::with_match_fn(ctx, true, Some(match_always));
    skippy_iter.reset(skippy_iter.buffer.idx);
    skippy_iter.set_glyph_data(0);
    let mut unsafe_to = None;
    let unsafe_to1;
    let mut unsafe_to2 = 0;
    let mut second = None;
    let first = if skippy_iter.next(None) {
        let g1 = skippy_iter.index();
        if skippy_iter.may_skip(&skippy_iter.buffer.info[g1]) != may_skip_t::SKIP_NO {
            // Can't use the fast path if eg. the next char is a default-ignorable
            // or other skippable.
            for rule in rules.iter().filter_map(|r| r.ok()) {
                if apply_chain_with_sequences(
                    ctx,
                    rule.backtrack(),
                    rule.input(),
                    rule.lookahead(),
                    rule.lookup_records(),
                    &match_funcs,
                )
                .is_some()
                {
                    return Some(());
                };
            }
            return None;
        }
        unsafe_to1 = skippy_iter.index() + 1;
        g1
    } else {
        // Failed to match a next glyph. Only try applying rules that have no
        // further impact.
        for rule in rules
            .iter()
            .filter_map(|r| r.ok())
            .filter(|r| r.input().len() <= 1 && r.lookahead().is_empty())
        {
            if apply_chain_with_sequences(
                ctx,
                rule.backtrack(),
                rule.input(),
                rule.lookahead(),
                rule.lookup_records(),
                &match_funcs,
            )
            .is_some()
            {
                return Some(());
            };
        }
        return None;
    };
    let matched = skippy_iter.next(None);
    let g2 = skippy_iter.index();
    if matched {
        second = Some(g2);
        unsafe_to2 = skippy_iter.index() + 1;
        if skippy_iter.may_skip(&skippy_iter.buffer.info[g2]) != may_skip_t::SKIP_NO {
            // Can't use the fast path if eg. the next char is a default-ignorable
            // or other skippable.
            for rule in rules.iter().filter_map(|r| r.ok()) {
                if apply_chain_with_sequences(
                    ctx,
                    rule.backtrack(),
                    rule.input(),
                    rule.lookahead(),
                    rule.lookup_records(),
                    &match_funcs,
                )
                .is_some()
                {
                    return Some(());
                };
            }
            return None;
        }
    }
    let mut rules_iter = rules.iter().filter_map(|r| r.ok());
    let mut rule_box = rules_iter.next();
    loop {
        let Some(rule) = rule_box else {
            break;
        };
        let input = rule.input();
        let lookahead = rule.lookahead();
        let match_input = |info: &mut GlyphInfo, index: usize| {
            input
                .get(index)
                .is_some_and(|v| match_funcs.1(info, v.to_u16()))
        };
        let match_lookahead = |info: &mut GlyphInfo, index: usize| {
            lookahead
                .get(index)
                .is_some_and(|v| match_funcs.2(info, v.to_u16()))
        };
        let len_p1 = input.len() + 1;
        let matched_first = if len_p1 > 1 {
            match_input(&mut ctx.buffer.info[first], 0)
        } else {
            lookahead.is_empty() || match_lookahead(&mut ctx.buffer.info[first], 0)
        };
        if matched_first {
            let matched_second = if let Some(second) = second {
                if len_p1 > 2 {
                    match_input(&mut ctx.buffer.info[second], 1)
                } else {
                    (lookahead.len() <= 2 - len_p1)
                        || match_lookahead(&mut ctx.buffer.info[second], 2 - len_p1)
                }
            } else {
                true
            };
            if matched_second {
                if apply_chain_with_sequences(
                    ctx,
                    rule.backtrack(),
                    rule.input(),
                    rule.lookahead(),
                    rule.lookup_records(),
                    &match_funcs,
                )
                .is_some()
                {
                    if let Some(unsafe_to) = unsafe_to {
                        ctx.buffer
                            .unsafe_to_concat(Some(ctx.buffer.idx), Some(unsafe_to));
                    }
                    return Some(());
                }
            } else {
                unsafe_to = Some(unsafe_to2);
            }

            rule_box = rules_iter.next();
        } else {
            if unsafe_to.is_none() {
                unsafe_to = Some(unsafe_to1);
            }

            if len_p1 > 1 {
                // Skip ahead to next possible first glyph match.
                let first_glyph_value = input.first().unwrap().to_u16();
                loop {
                    let next_rule_box = rules_iter.next();
                    if next_rule_box.is_none() {
                        rule_box = None;
                        break;
                    }
                    let next_inputs = next_rule_box.as_ref().unwrap().input();
                    if next_inputs.is_empty()
                        || next_inputs.first().unwrap().to_u16() != first_glyph_value
                    {
                        rule_box = next_rule_box;
                        break;
                    }
                }
            } else {
                rule_box = rules_iter.next();
            }
        }
    }
    if let Some(unsafe_to) = unsafe_to {
        ctx.buffer
            .unsafe_to_concat(Some(ctx.buffer.idx), Some(unsafe_to));
    }
    None
}
