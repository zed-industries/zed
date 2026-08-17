use crate::hb::ot_layout::MAX_NESTING_LEVEL;
use crate::hb::ot_layout_gsubgpos::OT::hb_ot_apply_context_t;
use crate::hb::ot_layout_gsubgpos::{
    match_backtrack, match_lookahead, Apply, WouldApply, WouldApplyContext,
};
use ttf_parser::gsub::ReverseChainSingleSubstitution;

// ReverseChainSingleSubstFormat1::would_apply
impl WouldApply for ReverseChainSingleSubstitution<'_> {
    fn would_apply(&self, ctx: &WouldApplyContext) -> bool {
        ctx.glyphs.len() == 1 && self.coverage.get(ctx.glyphs[0]).is_some()
    }
}

// ReverseChainSingleSubstFormat1::apply
impl Apply for ReverseChainSingleSubstitution<'_> {
    fn apply(&self, ctx: &mut hb_ot_apply_context_t) -> Option<()> {
        let glyph = ctx.buffer.cur(0).as_glyph();
        let index = self.coverage.get(glyph)?;
        if index >= self.substitutes.len() {
            return None;
        }

        // No chaining to this type.
        if ctx.nesting_level_left != MAX_NESTING_LEVEL {
            return None;
        }

        let subst = self.substitutes.get(index)?;

        let f1 = |glyph, index| {
            let value = self.backtrack_coverages.get(index).unwrap();
            value.contains(glyph)
        };

        let f2 = |glyph, index| {
            let value = self.lookahead_coverages.get(index).unwrap();
            value.contains(glyph)
        };

        let mut start_index = 0;
        let mut end_index = 0;

        if match_backtrack(ctx, self.backtrack_coverages.len(), &f1, &mut start_index) {
            if match_lookahead(
                ctx,
                self.lookahead_coverages.len(),
                &f2,
                ctx.buffer.idx + 1,
                &mut end_index,
            ) {
                ctx.buffer
                    .unsafe_to_break_from_outbuffer(Some(start_index), Some(end_index));
                ctx.replace_glyph_inplace(subst);

                // Note: We DON'T decrease buffer.idx.  The main loop does it
                // for us.  This is useful for preventing surprises if someone
                // calls us through a Context lookup.
                return Some(());
            }
        }

        ctx.buffer
            .unsafe_to_concat_from_outbuffer(Some(start_index), Some(end_index));
        None
    }
}
