use crate::hb::buffer::{hb_buffer_t, HB_BUFFER_SCRATCH_FLAG_HAS_GPOS_ATTACHMENT};
use crate::hb::ot_layout_common::lookup_flags;
use crate::hb::ot_layout_gpos_table::attach_type;
use crate::hb::ot_layout_gsubgpos::OT::hb_ot_apply_context_t;
use crate::hb::ot_layout_gsubgpos::{match_t, skipping_iterator_t, Apply, MatchSource};
use read_fonts::tables::gpos::{
    AnchorTable, MarkArray, MarkBasePosFormat1, MarkLigPosFormat1, MarkMarkPosFormat1,
};

trait MarkArrayExt {
    fn apply(
        &self,
        ctx: &mut hb_ot_apply_context_t,
        base_anchor: &AnchorTable,
        mark_anchor: &AnchorTable,
        glyph_pos: usize,
    ) -> Option<()>;
}

impl MarkArrayExt for MarkArray<'_> {
    fn apply(
        &self,
        ctx: &mut hb_ot_apply_context_t,
        base_anchor: &AnchorTable,
        mark_anchor: &AnchorTable,
        glyph_pos: usize,
    ) -> Option<()> {
        // If this subtable doesn't have an anchor for this base and this class
        // return `None` such that the subsequent subtables have a chance at it.

        let (base_x, base_y) = ctx.face.ot_tables.resolve_anchor(base_anchor);
        let (mark_x, mark_y) = ctx.face.ot_tables.resolve_anchor(mark_anchor);

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

impl Apply for MarkBasePosFormat1<'_> {
    fn apply(&self, ctx: &mut hb_ot_apply_context_t) -> Option<()> {
        let mark_glyph = ctx.buffer.cur(0).as_glyph();
        let mark_index = self.mark_coverage().ok()?.get(mark_glyph)?;

        let base_coverage = self.base_coverage().ok()?;
        let last_base_until = ctx.last_base_until;
        let mut last_base = ctx.last_base;

        // Due to borrowing rules, we have this piece of code before creating the
        // iterator, unlike in harfbuzz.
        if ctx.last_base_until > ctx.buffer.idx as u32 {
            ctx.last_base_until = 0;
            ctx.last_base = -1;
        }

        // Now we search backwards for a non-mark glyph
        // We don't use skippy_iter.prev() to avoid O(n^2) behavior.
        let mut iter = skipping_iterator_t::new(ctx, false);
        iter.set_lookup_props(u32::from(lookup_flags::IGNORE_MARKS));

        let mut j = iter.buffer.idx;
        while j > last_base_until as usize {
            let mut _match = iter.match_at(j - 1, MatchSource::Info);
            if _match == match_t::MATCH {
                // https://github.com/harfbuzz/harfbuzz/issues/4124
                if !accept(iter.buffer, j - 1)
                    && base_coverage
                        .get(iter.buffer.info[j - 1].as_glyph())
                        .is_none()
                {
                    _match = match_t::SKIP;
                }
            }

            if _match == match_t::MATCH {
                last_base = j as i32 - 1;
                break;
            }

            j -= 1;
        }
        ctx.last_base_until = ctx.buffer.idx as u32;
        ctx.last_base = last_base;

        if ctx.last_base == -1 {
            ctx.buffer
                .unsafe_to_concat_from_outbuffer(Some(0), Some(ctx.buffer.idx + 1));
            return None;
        }

        let idx = ctx.last_base as u32;

        let info = &ctx.buffer.info;

        // Checking that matched glyph is actually a base glyph by GDEF is too strong; disabled
        let base_glyph = info[idx as usize].as_glyph();
        let Some(base_index) = self.base_coverage().ok()?.get(base_glyph) else {
            ctx.buffer
                .unsafe_to_concat_from_outbuffer(Some(idx as usize), Some(ctx.buffer.idx + 1));
            return None;
        };

        let mark_array = self.mark_array().ok()?;
        let mark_record = mark_array.mark_records().get(mark_index as usize)?;
        let mark_anchor = mark_record.mark_anchor(mark_array.offset_data()).ok()?;

        let base_array = self.base_array().ok()?;
        let base_record = base_array.base_records().get(base_index as usize).ok()?;
        let base_anchor = base_record
            .base_anchors(base_array.offset_data())
            .get(mark_record.mark_class() as usize)?
            .ok()?;

        mark_array.apply(ctx, &base_anchor, &mark_anchor, idx as usize)
    }
}

fn accept(buffer: &hb_buffer_t, idx: usize) -> bool {
    /* We only want to attach to the first of a MultipleSubst sequence.
     * https://github.com/harfbuzz/harfbuzz/issues/740
     * Reject others...
     * ...but stop if we find a mark in the MultipleSubst sequence:
     * https://github.com/harfbuzz/harfbuzz/issues/1020 */
    !buffer.info[idx].multiplied()
        || 0 == buffer.info[idx].lig_comp()
        || (idx == 0
            || buffer.info[idx - 1].is_mark()
            || !buffer.info[idx - 1].multiplied()
            || buffer.info[idx].lig_id() != buffer.info[idx - 1].lig_id()
            || buffer.info[idx].lig_comp() != buffer.info[idx - 1].lig_comp() + 1)
}

impl Apply for MarkMarkPosFormat1<'_> {
    fn apply(&self, ctx: &mut hb_ot_apply_context_t) -> Option<()> {
        let mark1_glyph = ctx.buffer.cur(0).as_glyph();
        let mark1_index = self.mark1_coverage().ok()?.get(mark1_glyph)?;
        let lookup_props = ctx.lookup_props;
        // Now we search backwards for a suitable mark glyph until a non-mark glyph
        let mut iter = skipping_iterator_t::new(ctx, false);
        iter.reset_fast(iter.buffer.idx);
        iter.set_lookup_props(lookup_props & !u32::from(lookup_flags::IGNORE_FLAGS));

        let mut unsafe_from = 0;
        if !iter.prev(Some(&mut unsafe_from)) {
            iter.buffer
                .unsafe_to_concat_from_outbuffer(Some(unsafe_from), Some(iter.buffer.idx + 1));
            return None;
        }

        let iter_idx = iter.index();
        if !ctx.buffer.info[iter_idx].is_mark() {
            ctx.buffer
                .unsafe_to_concat_from_outbuffer(Some(iter_idx), Some(ctx.buffer.idx + 1));
            return None;
        }

        let id1 = ctx.buffer.cur(0).lig_id();
        let id2 = ctx.buffer.info[iter_idx].lig_id();
        let comp1 = ctx.buffer.cur(0).lig_comp();
        let comp2 = ctx.buffer.info[iter_idx].lig_comp();

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
                .unsafe_to_concat_from_outbuffer(Some(iter_idx), Some(ctx.buffer.idx + 1));
            return None;
        }

        let mark2_glyph = ctx.buffer.info[iter_idx].as_glyph();
        let mark2_index = self.mark2_coverage().ok()?.get(mark2_glyph)?;

        let mark1_array = self.mark1_array().ok()?;
        let mark1_record = mark1_array.mark_records().get(mark1_index as usize)?;
        let mark1_anchor = mark1_record.mark_anchor(mark1_array.offset_data()).ok()?;

        let base_array = self.mark2_array().ok()?;
        let base_record = base_array.mark2_records().get(mark2_index as usize).ok()?;
        let base_anchor = base_record
            .mark2_anchors(base_array.offset_data())
            .get(mark1_record.mark_class() as usize)?
            .ok()?;

        mark1_array.apply(ctx, &base_anchor, &mark1_anchor, iter_idx)
    }
}

impl Apply for MarkLigPosFormat1<'_> {
    fn apply(&self, ctx: &mut hb_ot_apply_context_t) -> Option<()> {
        let mark_glyph = ctx.buffer.cur(0).as_glyph();
        let mark_index = self.mark_coverage().ok()?.get(mark_glyph)? as usize;

        // Due to borrowing rules, we have this piece of code before creating the
        // iterator, unlike in harfbuzz.
        if ctx.last_base_until > ctx.buffer.idx as u32 {
            ctx.last_base_until = 0;
            ctx.last_base = -1;
        }

        let last_base_until = ctx.last_base_until;
        let mut last_base = ctx.last_base;

        // Now we search backwards for a non-mark glyph
        let mut iter = skipping_iterator_t::new(ctx, false);
        iter.set_lookup_props(u32::from(lookup_flags::IGNORE_MARKS));

        let mut j = iter.buffer.idx;
        while j > last_base_until as usize {
            let mut _match = iter.match_at(j - 1, MatchSource::Info);
            if _match == match_t::MATCH {
                last_base = j as i32 - 1;
                break;
            }
            j -= 1;
        }

        ctx.last_base_until = ctx.buffer.idx as u32;
        ctx.last_base = last_base;

        if ctx.last_base == -1 {
            ctx.buffer
                .unsafe_to_concat_from_outbuffer(Some(0), Some(ctx.buffer.idx + 1));
            return None;
        }

        let idx = ctx.last_base as usize;

        // Checking that matched glyph is actually a ligature by GDEF is too strong; disabled

        let lig_glyph = ctx.buffer.info[idx].as_glyph();
        let Some(lig_index) = self.ligature_coverage().ok()?.get(lig_glyph) else {
            ctx.buffer
                .unsafe_to_concat_from_outbuffer(Some(idx), Some(ctx.buffer.idx + 1));
            return None;
        };
        let lig_attach = self
            .ligature_array()
            .ok()?
            .ligature_attaches()
            .get(lig_index as usize)
            .ok()?;

        // Find component to attach to
        let comp_count = lig_attach.component_count();
        if comp_count == 0 {
            ctx.buffer
                .unsafe_to_concat_from_outbuffer(Some(idx), Some(ctx.buffer.idx + 1));
            return None;
        }

        // We must now check whether the ligature ID of the current mark glyph
        // is identical to the ligature ID of the found ligature.  If yes, we
        // can directly use the component index.  If not, we attach the mark
        // glyph to the last component of the ligature.
        let lig_id = ctx.buffer.info[idx].lig_id();
        let mark_id = ctx.buffer.cur(0).lig_id();
        let mark_comp = u16::from(ctx.buffer.cur(0).lig_comp());
        let matches = lig_id != 0 && lig_id == mark_id && mark_comp > 0;
        let comp_index = if matches {
            mark_comp.min(comp_count)
        } else {
            comp_count
        } - 1;

        let mark_array = self.mark_array().ok()?;
        let mark_record = mark_array.mark_records().get(mark_index)?;
        let mark_anchor = mark_record.mark_anchor(mark_array.offset_data()).ok()?;

        let base_record = lig_attach
            .component_records()
            .get(comp_index as usize)
            .ok()?;
        let base_anchor = base_record
            .ligature_anchors(lig_attach.offset_data())
            .get(mark_record.mark_class() as usize)?
            .ok()?;

        mark_array.apply(ctx, &base_anchor, &mark_anchor, idx)
    }
}
