use crate::hb::ot_layout_gsubgpos::OT::hb_ot_apply_context_t;
use crate::hb::ot_layout_gsubgpos::{Apply, WouldApply, WouldApplyContext};
use ttf_parser::gsub::AlternateSubstitution;

// AlternateSubstFormat1::would_apply
impl WouldApply for AlternateSubstitution<'_> {
    fn would_apply(&self, ctx: &WouldApplyContext) -> bool {
        ctx.glyphs.len() == 1 && self.coverage.get(ctx.glyphs[0]).is_some()
    }
}

// AlternateSubstFormat1::apply
impl Apply for AlternateSubstitution<'_> {
    fn apply(&self, ctx: &mut hb_ot_apply_context_t) -> Option<()> {
        let glyph = ctx.buffer.cur(0).as_glyph();
        let index = self.coverage.get(glyph)?;
        let set = self.alternate_sets.get(index)?;
        set.apply(ctx)
    }
}
