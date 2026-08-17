use crate::hb::ot_layout_gpos_table::ValueRecordExt;
use crate::hb::ot_layout_gsubgpos::OT::hb_ot_apply_context_t;
use crate::hb::ot_layout_gsubgpos::{skipping_iterator_t, Apply};
use ttf_parser::gpos::{PairAdjustment, ValueRecord};

impl Apply for PairAdjustment<'_> {
    fn apply(&self, ctx: &mut hb_ot_apply_context_t) -> Option<()> {
        let first_glyph = ctx.buffer.cur(0).as_glyph();
        let first_glyph_coverage_index = self.coverage().get(first_glyph)?;

        let mut iter = skipping_iterator_t::new(ctx, ctx.buffer.idx, false);

        let mut unsafe_to = 0;
        if !iter.next(Some(&mut unsafe_to)) {
            ctx.buffer
                .unsafe_to_concat(Some(ctx.buffer.idx), Some(unsafe_to));
            return None;
        }

        let second_glyph_index = iter.index();
        let second_glyph = ctx.buffer.info[second_glyph_index].as_glyph();

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

        let bail = |ctx: &mut hb_ot_apply_context_t,
                    iter_index: &mut usize,
                    records: (ValueRecord, ValueRecord)| {
            let has_record1 = !records.0.is_empty();
            let has_record2 = !records.1.is_empty();

            let flag1 = has_record1 && records.0.apply(ctx, ctx.buffer.idx);
            let flag2 = has_record2 && records.1.apply(ctx, second_glyph_index);

            success(ctx, iter_index, flag1, flag2, has_record2)
        };

        let records = match self {
            Self::Format1 { sets, .. } => {
                sets.get(first_glyph_coverage_index)?.get(second_glyph)?
            }
            Self::Format2 {
                classes, matrix, ..
            } => {
                let classes = (classes.0.get(first_glyph), classes.1.get(second_glyph));

                let records = match matrix.get(classes) {
                    Some(v) => v,
                    None => {
                        ctx.buffer
                            .unsafe_to_concat(Some(ctx.buffer.idx), Some(iter.index() + 1));
                        return None;
                    }
                };

                return bail(ctx, &mut iter.buf_idx, records);
            }
        };

        bail(ctx, &mut iter.buf_idx, records)
    }
}
