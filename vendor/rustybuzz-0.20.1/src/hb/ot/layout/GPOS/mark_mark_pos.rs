use crate::hb::ot::layout::GPOS::mark_array::MarkArrayExt;
use crate::hb::ot_layout::{
    _hb_glyph_info_get_lig_comp, _hb_glyph_info_get_lig_id, _hb_glyph_info_is_mark,
};
use crate::hb::ot_layout_common::lookup_flags;
use crate::hb::ot_layout_gsubgpos::OT::hb_ot_apply_context_t;
use crate::hb::ot_layout_gsubgpos::{skipping_iterator_t, Apply};
use ttf_parser::gpos::MarkToMarkAdjustment;

impl Apply for MarkToMarkAdjustment<'_> {
    fn apply(&self, ctx: &mut hb_ot_apply_context_t) -> Option<()> {
        let buffer = &ctx.buffer;
        let mark1_glyph = ctx.buffer.cur(0).as_glyph();
        let mark1_index = self.mark1_coverage.get(mark1_glyph)?;

        // Now we search backwards for a suitable mark glyph until a non-mark glyph
        let mut iter = skipping_iterator_t::new(ctx, buffer.idx, false);
        iter.set_lookup_props(ctx.lookup_props & !u32::from(lookup_flags::IGNORE_FLAGS));

        let mut unsafe_from = 0;
        if !iter.prev(Some(&mut unsafe_from)) {
            ctx.buffer
                .unsafe_to_concat_from_outbuffer(Some(unsafe_from), Some(ctx.buffer.idx + 1));
            return None;
        }

        let iter_idx = iter.index();
        if !_hb_glyph_info_is_mark(&buffer.info[iter_idx]) {
            ctx.buffer
                .unsafe_to_concat_from_outbuffer(Some(iter_idx), Some(buffer.idx + 1));
            return None;
        }

        let id1 = _hb_glyph_info_get_lig_id(buffer.cur(0));
        let id2 = _hb_glyph_info_get_lig_id(&buffer.info[iter_idx]);
        let comp1 = _hb_glyph_info_get_lig_comp(buffer.cur(0));
        let comp2 = _hb_glyph_info_get_lig_comp(&buffer.info[iter_idx]);

        let matches = if id1 == id2 {
            // Marks belonging to the same base
            // or marks belonging to the same ligature component.
            id1 == 0 || comp1 == comp2
        } else {
            // If ligature ids don't match, it may be the case that one of the marks
            // itself is a ligature.  In which case match.
            (id1 > 0 && comp1 == 0) || (id2 > 0 && comp2 == 0)
        };

        if !matches {
            ctx.buffer
                .unsafe_to_concat_from_outbuffer(Some(iter_idx), Some(buffer.idx + 1));
            return None;
        }

        let mark2_glyph = buffer.info[iter_idx].as_glyph();
        let mark2_index = self.mark2_coverage.get(mark2_glyph)?;

        self.marks
            .apply(ctx, self.mark2_matrix, mark1_index, mark2_index, iter_idx)
    }
}
