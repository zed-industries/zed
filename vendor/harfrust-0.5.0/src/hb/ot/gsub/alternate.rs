use crate::hb::ot_layout_gsubgpos::OT::hb_ot_apply_context_t;
use crate::hb::ot_layout_gsubgpos::{Apply, WouldApply, WouldApplyContext};
use read_fonts::tables::gsub::{AlternateSet, AlternateSubstFormat1};

impl Apply for AlternateSet<'_> {
    fn apply(&self, ctx: &mut hb_ot_apply_context_t) -> Option<()> {
        let alternates = self.alternate_glyph_ids();
        let len = alternates.len() as u16;
        if len == 0 {
            return None;
        }

        let glyph_mask = ctx.buffer.cur(0).mask;

        // Note: This breaks badly if two features enabled this lookup together.
        let shift = ctx.lookup_mask().trailing_zeros();
        let mut alt_index = (ctx.lookup_mask() & glyph_mask) >> shift;

        // If alt_index is MAX_VALUE, randomize feature if it is the rand feature.
        if alt_index == crate::hb::ot_map::hb_ot_map_t::MAX_VALUE && ctx.random {
            // Maybe we can do better than unsafe-to-break all; but since we are
            // changing random state, it would be hard to track that.  Good 'nough.
            ctx.buffer.unsafe_to_break(Some(0), Some(ctx.buffer.len));
            alt_index = ctx.random_number() % u32::from(len) + 1;
        }

        let idx = u16::try_from(alt_index).ok()?.checked_sub(1)?;
        ctx.replace_glyph(alternates.get(idx as usize)?.get().into());

        Some(())
    }
}

impl WouldApply for AlternateSubstFormat1<'_> {
    fn would_apply(&self, ctx: &WouldApplyContext) -> bool {
        ctx.glyphs.len() == 1
            && self
                .coverage()
                .is_ok_and(|cov| cov.get(ctx.glyphs[0]).is_some())
    }
}

impl Apply for AlternateSubstFormat1<'_> {
    fn apply(&self, ctx: &mut hb_ot_apply_context_t) -> Option<()> {
        let glyph = ctx.buffer.cur(0).as_glyph();
        let index = self.coverage().ok()?.get(glyph)?;
        let set = self.alternate_sets().get(index as usize).ok()?;
        set.apply(ctx)
    }
}
