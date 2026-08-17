use crate::hb::ot_layout_gsubgpos::OT::hb_ot_apply_context_t;
use crate::hb::ot_layout_gsubgpos::{Apply, WouldApply, WouldApplyContext};
use ttf_parser::gsub::SingleSubstitution;
use ttf_parser::GlyphId;

// SingleSubstFormat1::would_apply
// SingleSubstFormat2::would_apply
impl WouldApply for SingleSubstitution<'_> {
    fn would_apply(&self, ctx: &WouldApplyContext) -> bool {
        ctx.glyphs.len() == 1 && self.coverage().get(ctx.glyphs[0]).is_some()
    }
}

// SingleSubstFormat1::apply
// SingleSubstFormat2::apply
impl Apply for SingleSubstitution<'_> {
    fn apply(&self, ctx: &mut hb_ot_apply_context_t) -> Option<()> {
        let glyph = ctx.buffer.cur(0).as_glyph();
        let subst = match *self {
            Self::Format1 { coverage, delta } => {
                coverage.get(glyph)?;
                // According to the Adobe Annotated OpenType Suite, result is always
                // limited to 16bit, so we explicitly want to truncate.
                GlyphId((i32::from(glyph.0) + i32::from(delta)) as u16)
            }
            Self::Format2 {
                coverage,
                substitutes,
            } => {
                let index = coverage.get(glyph)?;
                substitutes.get(index)?
            }
        };

        ctx.replace_glyph(subst);
        Some(())
    }
}
