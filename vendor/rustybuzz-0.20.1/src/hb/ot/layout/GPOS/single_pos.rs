use crate::hb::ot_layout_gpos_table::ValueRecordExt;
use crate::hb::ot_layout_gsubgpos::Apply;
use crate::hb::ot_layout_gsubgpos::OT::hb_ot_apply_context_t;
use ttf_parser::gpos::SingleAdjustment;

impl Apply for SingleAdjustment<'_> {
    fn apply(&self, ctx: &mut hb_ot_apply_context_t) -> Option<()> {
        let glyph = ctx.buffer.cur(0).as_glyph();
        let record = match self {
            Self::Format1 { coverage, value } => {
                coverage.get(glyph)?;
                *value
            }
            Self::Format2 { coverage, values } => {
                let index = coverage.get(glyph)?;
                values.get(index)?
            }
        };
        record.apply(ctx, ctx.buffer.idx);
        ctx.buffer.idx += 1;
        Some(())
    }
}
