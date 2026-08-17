use crate::hb::ot_layout_gsubgpos::Apply;
use crate::hb::ot_layout_gsubgpos::OT::hb_ot_apply_context_t;
use read_fonts::tables::gpos::{SinglePosFormat1, SinglePosFormat2};

impl Apply for SinglePosFormat1<'_> {
    fn apply(&self, ctx: &mut hb_ot_apply_context_t) -> Option<()> {
        let glyph = ctx.buffer.cur(0).as_glyph();
        self.coverage().ok()?.get(glyph)?;
        let format = self.value_format();
        let offset = self.shape().value_record_byte_range().start;
        super::apply_value(ctx, ctx.buffer.idx, &self.offset_data(), offset, format);
        ctx.buffer.idx += 1;
        Some(())
    }
}

impl Apply for SinglePosFormat2<'_> {
    fn apply(&self, ctx: &mut hb_ot_apply_context_t) -> Option<()> {
        let glyph = ctx.buffer.cur(0).as_glyph();
        let index = self.coverage().ok()?.get(glyph)? as usize;
        let format = self.value_format();
        let offset =
            self.shape().value_records_byte_range().start + (format.record_byte_len() * index);
        super::apply_value(ctx, ctx.buffer.idx, &self.offset_data(), offset, format);
        ctx.buffer.idx += 1;
        Some(())
    }
}
