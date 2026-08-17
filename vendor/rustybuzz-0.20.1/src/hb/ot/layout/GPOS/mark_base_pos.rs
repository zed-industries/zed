use crate::hb::buffer::hb_buffer_t;
use crate::hb::ot::layout::GPOS::mark_array::MarkArrayExt;
use crate::hb::ot_layout::{
    _hb_glyph_info_get_lig_comp, _hb_glyph_info_get_lig_id, _hb_glyph_info_is_mark,
    _hb_glyph_info_multiplied,
};
use crate::hb::ot_layout_common::lookup_flags;
use crate::hb::ot_layout_gsubgpos::OT::hb_ot_apply_context_t;
use crate::hb::ot_layout_gsubgpos::{match_t, skipping_iterator_t, Apply};
use ttf_parser::gpos::MarkToBaseAdjustment;

impl Apply for MarkToBaseAdjustment<'_> {
    fn apply(&self, ctx: &mut hb_ot_apply_context_t) -> Option<()> {
        let buffer = &ctx.buffer;
        let mark_glyph = ctx.buffer.cur(0).as_glyph();
        let mark_index = self.mark_coverage.get(mark_glyph)?;

        // Due to borrowing rules, we have this piece of code before creating the
        // iterator, unlike in harfbuzz.
        if ctx.last_base_until > buffer.idx as u32 {
            ctx.last_base_until = 0;
            ctx.last_base = -1;
        }

        // Now we search backwards for a non-mark glyph
        // We don't use skippy_iter.prev() to avoid O(n^2) behavior.
        let mut iter = skipping_iterator_t::new(ctx, 0, false);
        iter.set_lookup_props(u32::from(lookup_flags::IGNORE_MARKS));

        let mut j = buffer.idx;
        while j > ctx.last_base_until as usize {
            let mut _match = iter.match_(&buffer.info[j - 1]);
            if _match == match_t::MATCH {
                // https://github.com/harfbuzz/harfbuzz/issues/4124
                if !accept(buffer, j - 1)
                    && !self.base_coverage.contains(buffer.info[j - 1].as_glyph())
                {
                    _match = match_t::SKIP;
                }
            }

            if _match == match_t::MATCH {
                ctx.last_base = j as i32 - 1;
                break;
            }

            j -= 1;
        }
        ctx.last_base_until = buffer.idx as u32;

        if ctx.last_base == -1 {
            ctx.buffer
                .unsafe_to_concat_from_outbuffer(Some(0), Some(buffer.idx + 1));
            return None;
        }

        let idx = ctx.last_base as u32;

        let info = &buffer.info;

        // Checking that matched glyph is actually a base glyph by GDEF is too strong; disabled
        let base_glyph = info[idx as usize].as_glyph();
        let Some(base_index) = self.base_coverage.get(base_glyph) else {
            ctx.buffer
                .unsafe_to_concat_from_outbuffer(Some(idx as usize), Some(buffer.idx + 1));
            return None;
        };

        self.marks
            .apply(ctx, self.anchors, mark_index, base_index, idx as usize)
    }
}

fn accept(buffer: &hb_buffer_t, idx: usize) -> bool {
    /* We only want to attach to the first of a MultipleSubst sequence.
     * https://github.com/harfbuzz/harfbuzz/issues/740
     * Reject others...
     * ...but stop if we find a mark in the MultipleSubst sequence:
     * https://github.com/harfbuzz/harfbuzz/issues/1020 */
    !_hb_glyph_info_multiplied(&buffer.info[idx])
        || 0 == _hb_glyph_info_get_lig_comp(&buffer.info[idx])
        || (idx == 0
            || _hb_glyph_info_is_mark(&buffer.info[idx - 1])
            || !_hb_glyph_info_multiplied(&buffer.info[idx - 1])
            || _hb_glyph_info_get_lig_id(&buffer.info[idx])
                != _hb_glyph_info_get_lig_id(&buffer.info[idx - 1])
            || _hb_glyph_info_get_lig_comp(&buffer.info[idx])
                != _hb_glyph_info_get_lig_comp(&buffer.info[idx - 1]) + 1)
}
