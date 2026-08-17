use crate::hb::buffer::HB_BUFFER_SCRATCH_FLAG_HAS_GPOS_ATTACHMENT;
use crate::hb::ot_layout_gpos_table::{attach_type, AnchorExt};
use crate::hb::ot_layout_gsubgpos::OT::hb_ot_apply_context_t;
use ttf_parser::gpos::{AnchorMatrix, MarkArray};

pub(crate) trait MarkArrayExt {
    fn apply(
        &self,
        ctx: &mut hb_ot_apply_context_t,
        anchors: AnchorMatrix,
        mark_index: u16,
        glyph_index: u16,
        glyph_pos: usize,
    ) -> Option<()>;
}

impl MarkArrayExt for MarkArray<'_> {
    fn apply(
        &self,
        ctx: &mut hb_ot_apply_context_t,
        anchors: AnchorMatrix,
        mark_index: u16,
        glyph_index: u16,
        glyph_pos: usize,
    ) -> Option<()> {
        // If this subtable doesn't have an anchor for this base and this class
        // return `None` such that the subsequent subtables have a chance at it.
        let (mark_class, mark_anchor) = self.get(mark_index)?;
        let base_anchor = anchors.get(glyph_index, mark_class)?;

        let (mark_x, mark_y) = mark_anchor.get(ctx.face);
        let (base_x, base_y) = base_anchor.get(ctx.face);

        ctx.buffer
            .unsafe_to_break(Some(glyph_pos), Some(ctx.buffer.idx + 1));

        let idx = ctx.buffer.idx;
        let pos = ctx.buffer.cur_pos_mut();
        pos.x_offset = base_x - mark_x;
        pos.y_offset = base_y - mark_y;
        pos.set_attach_type(attach_type::MARK);
        pos.set_attach_chain((glyph_pos as isize - idx as isize) as i16);

        ctx.buffer.scratch_flags |= HB_BUFFER_SCRATCH_FLAG_HAS_GPOS_ATTACHMENT;
        ctx.buffer.idx += 1;

        Some(())
    }
}
