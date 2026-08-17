use crate::hb::buffer::GlyphInfo;
use crate::hb::ot::{coverage_index, coverage_index_cached, CoverageInfo};
use crate::hb::ot_layout_gsubgpos::OT::hb_ot_apply_context_t;
use crate::hb::ot_layout_gsubgpos::{
    ligate_input, match_always, match_glyph, match_input, may_skip_t, skipping_iterator_t, Apply,
    LigatureSubstFormat1Cache, LigatureSubstFormat1SmallCache, SubtableExternalCache,
    SubtableExternalCacheMode, WouldApply, WouldApplyContext,
};
use crate::hb::set_digest::hb_set_digest_t;
use alloc::boxed::Box;
use read_fonts::tables::gsub::{Ligature, LigatureSet, LigatureSubstFormat1};
use read_fonts::types::GlyphId;

impl WouldApply for Ligature<'_> {
    fn would_apply(&self, ctx: &WouldApplyContext) -> bool {
        let components = self.component_glyph_ids();
        ctx.glyphs.len() == components.len() + 1
            && components
                .iter()
                .map(|comp| GlyphId::from(comp.get()))
                .enumerate()
                .all(|(i, comp)| ctx.glyphs[i + 1] == comp)
    }
}

impl Apply for Ligature<'_> {
    fn apply(&self, ctx: &mut hb_ot_apply_context_t) -> Option<()> {
        // Special-case to make it in-place and not consider this
        // as a "ligated" substitution.
        let components = self.component_glyph_ids();
        if components.is_empty() {
            ctx.replace_glyph(self.ligature_glyph().into());
            Some(())
        } else {
            let f = |info: &mut GlyphInfo, index| {
                let value = components.get(index as usize).unwrap().get().to_u16();
                match_glyph(info, value)
            };

            let mut match_end = 0;
            let mut total_component_count = 0;

            if !match_input(
                ctx,
                components.len() as u16,
                f,
                &mut match_end,
                Some(&mut total_component_count),
            ) {
                ctx.buffer
                    .unsafe_to_concat(Some(ctx.buffer.idx), Some(match_end));
                return None;
            }
            let count = components.len() + 1;
            ligate_input(
                ctx,
                count,
                match_end,
                total_component_count,
                self.ligature_glyph().into(),
            );
            Some(())
        }
    }
}

impl WouldApply for LigatureSet<'_> {
    fn would_apply(&self, ctx: &WouldApplyContext) -> bool {
        self.ligatures()
            .iter()
            .filter_map(Result::ok)
            .any(|lig| lig.would_apply(ctx))
    }
}

pub trait ApplyLigatureSet {
    fn apply(&self, ctx: &mut hb_ot_apply_context_t, seconds: &hb_set_digest_t) -> Option<()>;
}

impl ApplyLigatureSet for LigatureSet<'_> {
    fn apply(&self, ctx: &mut hb_ot_apply_context_t, seconds: &hb_set_digest_t) -> Option<()> {
        let mut second = GlyphId::new(u32::MAX);
        let mut unsafe_to = 0;
        let ligatures = self.ligatures();
        let slow_path = if ligatures.len() <= 1 {
            true
        } else {
            let mut iter = skipping_iterator_t::with_match_fn(ctx, true, Some(match_always));
            iter.reset(iter.buffer.idx);
            let matched = iter.next(Some(&mut unsafe_to));
            if !matched {
                true
            } else {
                second = iter.buffer.info[iter.index()].glyph_id.into();
                unsafe_to = iter.index() + 1;

                // Can't use the fast path if eg. the next char is a default-ignorable
                // or other skippable.
                iter.may_skip(&iter.buffer.info[iter.index()]) != may_skip_t::SKIP_NO
            }
        };

        if slow_path {
            // Slow path
            for lig in ligatures.iter().filter_map(Result::ok) {
                if lig.apply(ctx).is_some() {
                    return Some(());
                }
            }
        } else {
            // Fast path
            if !seconds.may_have(second.into()) {
                return None;
            }
            let mut unsafe_to_concat = false;
            for lig in ligatures.iter().filter_map(|lig| lig.ok()) {
                let components = lig.component_glyph_ids();
                if components.is_empty() || components[0].get() == second {
                    if lig.apply(ctx).is_some() {
                        if unsafe_to_concat {
                            ctx.buffer
                                .unsafe_to_concat(Some(ctx.buffer.idx), Some(unsafe_to));
                        }
                        return Some(());
                    }
                } else if !components.is_empty() {
                    unsafe_to_concat = true;
                }
            }
            if unsafe_to_concat {
                ctx.buffer
                    .unsafe_to_concat(Some(ctx.buffer.idx), Some(unsafe_to));
            }
        }
        None
    }
}

impl WouldApply for LigatureSubstFormat1<'_> {
    fn would_apply(&self, ctx: &WouldApplyContext) -> bool {
        self.coverage()
            .ok()
            .and_then(|coverage| coverage.get(ctx.glyphs[0]))
            .and_then(|index| self.ligature_sets().get(index as usize).ok())
            .is_some_and(|set| set.would_apply(ctx))
    }
}

impl Apply for LigatureSubstFormat1<'_> {
    fn apply_with_external_cache(
        &self,
        ctx: &mut hb_ot_apply_context_t,
        external_cache: &SubtableExternalCache,
    ) -> Option<()> {
        let glyph = ctx.buffer.cur(0).as_glyph();

        let (index, seconds) = match external_cache {
            SubtableExternalCache::LigatureSubstFormat1Cache(cache) => (
                coverage_index_cached(
                    |gid| self.coverage().ok()?.get(gid),
                    glyph,
                    &cache.coverage,
                )?,
                &cache.seconds,
            ),
            SubtableExternalCache::LigatureSubstFormat1SmallCache(cache) => (
                cache.coverage.index(&self.offset_data(), glyph)?,
                &cache.seconds,
            ),
            _ => (
                coverage_index(self.coverage(), glyph)?,
                &hb_set_digest_t::full(),
            ),
        };
        self.ligature_sets()
            .get(index as usize)
            .ok()
            .and_then(|set| set.apply(ctx, seconds))
    }

    fn external_cache_create(&self, mode: SubtableExternalCacheMode) -> SubtableExternalCache {
        match mode {
            SubtableExternalCacheMode::Full => SubtableExternalCache::LigatureSubstFormat1Cache(
                Box::new(LigatureSubstFormat1Cache::new(collect_seconds(self))),
            ),
            SubtableExternalCacheMode::Small => {
                if let Some(coverage) =
                    CoverageInfo::new(&self.offset_data(), self.coverage_offset().to_u32() as u16)
                {
                    SubtableExternalCache::LigatureSubstFormat1SmallCache(
                        LigatureSubstFormat1SmallCache {
                            coverage,
                            seconds: collect_seconds(self),
                        },
                    )
                } else {
                    SubtableExternalCache::None
                }
            }
            SubtableExternalCacheMode::None => SubtableExternalCache::None,
        }
    }
}

fn collect_seconds(lig_subst: &LigatureSubstFormat1) -> hb_set_digest_t {
    let mut seconds = hb_set_digest_t::new();
    lig_subst
        .ligature_sets()
        .iter()
        .filter_map(Result::ok)
        .for_each(|lig_set| {
            lig_set
                .ligatures()
                .iter()
                .filter_map(Result::ok)
                .for_each(|lig| {
                    if let Some(gid) = lig.component_glyph_ids().first() {
                        seconds.add(gid.get().into());
                    } else {
                        seconds = hb_set_digest_t::full();
                    };
                });
        });
    seconds
}
