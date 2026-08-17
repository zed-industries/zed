use crate::hb::ot::{coverage_index, coverage_index_cached, ClassDefInfo, CoverageInfo};
use crate::hb::ot::{glyph_class, glyph_class_cached};
use crate::hb::ot_layout_gsubgpos::OT::hb_ot_apply_context_t;
use crate::hb::ot_layout_gsubgpos::{
    skipping_iterator_t, Apply, PairPosFormat1Cache, PairPosFormat1SmallCache, PairPosFormat2Cache,
    PairPosFormat2SmallCache, SubtableExternalCache, SubtableExternalCacheMode,
};
use alloc::boxed::Box;
use read_fonts::tables::gpos::{PairPosFormat1, PairPosFormat2};

impl Apply for PairPosFormat1<'_> {
    fn apply_with_external_cache(
        &self,
        ctx: &mut hb_ot_apply_context_t,
        external_cache: &SubtableExternalCache,
    ) -> Option<()> {
        let first_glyph = ctx.buffer.cur(0).as_glyph();

        let first_glyph_coverage_index = match external_cache {
            SubtableExternalCache::PairPosFormat1Cache(cache) => coverage_index_cached(
                |gid| self.coverage().ok()?.get(gid),
                first_glyph,
                &cache.coverage,
            )?,
            SubtableExternalCache::PairPosFormat1SmallCache(cache) => {
                cache.coverage.index(&self.offset_data(), first_glyph)?
            }
            _ => coverage_index(self.coverage(), first_glyph)?,
        };

        let mut iter = skipping_iterator_t::new(ctx, false);
        iter.reset(iter.buffer.idx);

        let mut unsafe_to = 0;
        if !iter.next(Some(&mut unsafe_to)) {
            ctx.buffer
                .unsafe_to_concat(Some(ctx.buffer.idx), Some(unsafe_to));
            return None;
        }

        let second_glyph_index = iter.index();
        let second_glyph = iter.buffer.info[second_glyph_index].as_glyph();

        let finish = |ctx: &mut hb_ot_apply_context_t, iter_index: &mut usize, has_record2| {
            if has_record2 {
                *iter_index += 1;
                // https://github.com/harfbuzz/harfbuzz/issues/3824
                // https://github.com/harfbuzz/harfbuzz/issues/3888#issuecomment-1326781116
                ctx.buffer
                    .unsafe_to_break(Some(ctx.buffer.idx), Some(*iter_index + 1));
            }

            ctx.buffer.idx = *iter_index;

            Some(())
        };

        let boring = |ctx: &mut hb_ot_apply_context_t, iter_index: &mut usize, has_record2| {
            ctx.buffer
                .unsafe_to_concat(Some(ctx.buffer.idx), Some(second_glyph_index + 1));
            finish(ctx, iter_index, has_record2)
        };

        let success =
            |ctx: &mut hb_ot_apply_context_t, iter_index: &mut usize, flag1, flag2, has_record2| {
                if flag1 || flag2 {
                    ctx.buffer
                        .unsafe_to_break(Some(ctx.buffer.idx), Some(second_glyph_index + 1));
                    finish(ctx, iter_index, has_record2)
                } else {
                    boring(ctx, iter_index, has_record2)
                }
            };

        let mut buf_idx = iter.buf_idx;
        let set_offset = self
            .pair_set_offsets()
            .get(first_glyph_coverage_index as usize)?
            .get()
            .to_u32() as usize;
        let format1 = self.value_format1();
        let format1_len = format1.record_byte_len();
        let format2 = self.value_format2();
        let record_size = format1_len + format2.record_byte_len() + 2;
        let data = self.offset_data();
        let set_data = data.split_off(set_offset)?;
        let pair_count = set_data.read_at::<u16>(0).ok()? as usize;
        let mut hi = pair_count;
        let mut lo = 0;
        while lo < hi {
            // This recommends using usize::midpoint which expands to u128.
            // We definitely do not want to do that here since the input values
            // are 16-bit.
            #[allow(clippy::manual_midpoint)]
            let mid = (lo + hi) / 2;
            let record_offset = 2 + mid * record_size;
            let glyph_id = set_data
                .read_at::<read_fonts::types::GlyphId16>(record_offset)
                .ok()?;
            if glyph_id < second_glyph {
                lo = mid + 1;
            } else if glyph_id > second_glyph {
                hi = mid;
            } else {
                let has_record2 = !format2.is_empty();
                let worked1 = !format1.is_empty()
                    && super::apply_value(
                        ctx,
                        ctx.buffer.idx,
                        &set_data,
                        record_offset + 2,
                        format1,
                    ) == Some(true);
                let worked2 = has_record2
                    && super::apply_value(
                        ctx,
                        second_glyph_index,
                        &set_data,
                        record_offset + format1_len + 2,
                        format2,
                    ) == Some(true);
                return success(ctx, &mut buf_idx, worked1, worked2, has_record2);
            }
        }
        None
    }

    fn external_cache_create(&self, mode: SubtableExternalCacheMode) -> SubtableExternalCache {
        match mode {
            SubtableExternalCacheMode::Full => {
                SubtableExternalCache::PairPosFormat1Cache(Box::new(PairPosFormat1Cache::new()))
            }
            SubtableExternalCacheMode::Small => {
                if let Some(coverage) =
                    CoverageInfo::new(&self.offset_data(), self.coverage_offset().to_u32() as u16)
                {
                    SubtableExternalCache::PairPosFormat1SmallCache(PairPosFormat1SmallCache {
                        coverage,
                    })
                } else {
                    SubtableExternalCache::None
                }
            }
            SubtableExternalCacheMode::None => SubtableExternalCache::None,
        }
    }
}

impl Apply for PairPosFormat2<'_> {
    fn apply_with_external_cache(
        &self,
        ctx: &mut hb_ot_apply_context_t,
        external_cache: &SubtableExternalCache,
    ) -> Option<()> {
        let first_glyph = ctx.buffer.cur(0).as_glyph();
        match external_cache {
            SubtableExternalCache::PairPosFormat2Cache(cache) => coverage_index_cached(
                |gid| self.coverage().ok()?.get(gid),
                first_glyph,
                &cache.coverage,
            )?,
            SubtableExternalCache::PairPosFormat2SmallCache(cache) => {
                cache.coverage.index(&self.offset_data(), first_glyph)?
            }
            _ => coverage_index(self.coverage(), first_glyph)?,
        };
        let mut iter = skipping_iterator_t::new(ctx, false);
        iter.reset(iter.buffer.idx);

        let mut unsafe_to = 0;
        if !iter.next(Some(&mut unsafe_to)) {
            ctx.buffer
                .unsafe_to_concat(Some(ctx.buffer.idx), Some(unsafe_to));
            return None;
        }

        let second_glyph_index = iter.index();
        let second_glyph = iter.buffer.info[second_glyph_index].as_glyph();

        let finish = |ctx: &mut hb_ot_apply_context_t, iter_index: &mut usize, has_record2| {
            if has_record2 {
                *iter_index += 1;
                // https://github.com/harfbuzz/harfbuzz/issues/3824
                // https://github.com/harfbuzz/harfbuzz/issues/3888#issuecomment-1326781116
                ctx.buffer
                    .unsafe_to_break(Some(ctx.buffer.idx), Some(*iter_index + 1));
            }

            ctx.buffer.idx = *iter_index;

            Some(())
        };

        let boring = |ctx: &mut hb_ot_apply_context_t, iter_index: &mut usize, has_record2| {
            ctx.buffer
                .unsafe_to_concat(Some(ctx.buffer.idx), Some(second_glyph_index + 1));
            finish(ctx, iter_index, has_record2)
        };

        let success =
            |ctx: &mut hb_ot_apply_context_t, iter_index: &mut usize, flag1, flag2, has_record2| {
                if flag1 || flag2 {
                    ctx.buffer
                        .unsafe_to_break(Some(ctx.buffer.idx), Some(second_glyph_index + 1));
                    finish(ctx, iter_index, has_record2)
                } else {
                    boring(ctx, iter_index, has_record2)
                }
            };
        let data = self.offset_data();
        let (class1, class2) = match external_cache {
            SubtableExternalCache::PairPosFormat2Cache(cache) => (
                glyph_class_cached(
                    |gid| glyph_class(self.class_def1(), gid),
                    first_glyph,
                    &cache.first,
                ),
                glyph_class_cached(
                    |gid| glyph_class(self.class_def2(), gid),
                    second_glyph,
                    &cache.second,
                ),
            ),
            SubtableExternalCache::PairPosFormat2SmallCache(cache) => (
                cache.first.class(&data, first_glyph),
                cache.second.class(&data, second_glyph),
            ),
            _ => (
                glyph_class(self.class_def1(), first_glyph),
                glyph_class(self.class_def2(), second_glyph),
            ),
        };
        let mut buf_idx = iter.buf_idx;
        let format1 = self.value_format1();
        let format1_len = format1.record_byte_len();
        let format2 = self.value_format2();
        let record_size = format1_len + format2.record_byte_len();
        // Compute an offset into the 2D array of positioning records
        let record_offset = (class1 as usize * record_size * self.class2_count() as usize)
            + (class2 as usize * record_size)
            + self.shape().class1_records_byte_range().start;
        let has_record2 = !format2.is_empty();
        let worked1 = !format1.is_empty()
            && super::apply_value(ctx, ctx.buffer.idx, &data, record_offset, format1) == Some(true);
        let worked2 = has_record2
            && super::apply_value(
                ctx,
                second_glyph_index,
                &data,
                record_offset + format1_len,
                format2,
            ) == Some(true);
        success(ctx, &mut buf_idx, worked1, worked2, has_record2)
    }

    fn external_cache_create(&self, mode: SubtableExternalCacheMode) -> SubtableExternalCache {
        match mode {
            SubtableExternalCacheMode::Full => {
                SubtableExternalCache::PairPosFormat2Cache(Box::new(PairPosFormat2Cache::new()))
            }
            SubtableExternalCacheMode::Small => {
                let data = self.offset_data();
                let coverage = CoverageInfo::new(&data, self.coverage_offset().to_u32() as u16);
                let class1 = ClassDefInfo::new(&data, self.class_def1_offset().to_u32() as u16);
                let class2 = ClassDefInfo::new(&data, self.class_def2_offset().to_u32() as u16);
                if let Some((coverage, (first, second))) = coverage.zip(class1.zip(class2)) {
                    SubtableExternalCache::PairPosFormat2SmallCache(PairPosFormat2SmallCache {
                        coverage,
                        first,
                        second,
                    })
                } else {
                    SubtableExternalCache::None
                }
            }
            SubtableExternalCacheMode::None => SubtableExternalCache::None,
        }
    }
}
