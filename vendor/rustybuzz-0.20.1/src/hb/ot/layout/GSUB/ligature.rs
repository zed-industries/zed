use crate::hb::ot_layout_gsubgpos::OT::hb_ot_apply_context_t;
use crate::hb::ot_layout_gsubgpos::{
    ligate_input, match_glyph, match_input, Apply, WouldApply, WouldApplyContext,
};
use ttf_parser::gsub::Ligature;

impl WouldApply for Ligature<'_> {
    fn would_apply(&self, ctx: &WouldApplyContext) -> bool {
        ctx.glyphs.len() == usize::from(self.components.len()) + 1
            && self
                .components
                .into_iter()
                .enumerate()
                .all(|(i, comp)| ctx.glyphs[i + 1] == comp)
    }
}

impl Apply for Ligature<'_> {
    fn apply(&self, ctx: &mut hb_ot_apply_context_t) -> Option<()> {
        // Special-case to make it in-place and not consider this
        // as a "ligated" substitution.
        if self.components.is_empty() {
            ctx.replace_glyph(self.glyph);
            Some(())
        } else {
            let f = |glyph, index| {
                let value = self.components.get(index).unwrap();
                match_glyph(glyph, value.0)
            };

            let mut match_end = 0;
            let mut match_positions = smallvec::SmallVec::from_elem(0, 4);
            let mut total_component_count = 0;

            if !match_input(
                ctx,
                self.components.len(),
                &f,
                &mut match_end,
                &mut match_positions,
                Some(&mut total_component_count),
            ) {
                ctx.buffer
                    .unsafe_to_concat(Some(ctx.buffer.idx), Some(match_end));
                return None;
            }

            let count = usize::from(self.components.len()) + 1;
            ligate_input(
                ctx,
                count,
                &match_positions,
                match_end,
                total_component_count,
                self.glyph,
            );
            Some(())
        }
    }
}
