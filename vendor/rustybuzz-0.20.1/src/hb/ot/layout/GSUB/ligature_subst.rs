use crate::hb::ot_layout_gsubgpos::OT::hb_ot_apply_context_t;
use crate::hb::ot_layout_gsubgpos::{Apply, WouldApply, WouldApplyContext};
use ttf_parser::gsub::LigatureSubstitution;

// LigatureSubstFormat1::would_apply
impl WouldApply for LigatureSubstitution<'_> {
    fn would_apply(&self, ctx: &WouldApplyContext) -> bool {
        self.coverage
            .get(ctx.glyphs[0])
            .and_then(|index| self.ligature_sets.get(index))
            .map_or(false, |set| set.would_apply(ctx))
    }
}

// LigatureSubstFormat1::apply
impl Apply for LigatureSubstitution<'_> {
    fn apply(&self, ctx: &mut hb_ot_apply_context_t) -> Option<()> {
        let glyph = ctx.buffer.cur(0).as_glyph();
        self.coverage
            .get(glyph)
            .and_then(|index| self.ligature_sets.get(index))
            .and_then(|set| set.apply(ctx))
    }
}
