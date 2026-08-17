use crate::hb::buffer::HB_BUFFER_SCRATCH_FLAG_HAS_GPOS_ATTACHMENT;
use crate::hb::ot_layout_common::lookup_flags;
use crate::hb::ot_layout_gpos_table::attach_type;
use crate::hb::ot_layout_gsubgpos::OT::hb_ot_apply_context_t;
use crate::hb::ot_layout_gsubgpos::{skipping_iterator_t, Apply};
use crate::{Direction, GlyphPosition};
use read_fonts::tables::gpos::CursivePosFormat1;

impl Apply for CursivePosFormat1<'_> {
    fn apply(&self, ctx: &mut hb_ot_apply_context_t) -> Option<()> {
        let this = ctx.buffer.cur(0).as_glyph();

        let coverage = self.coverage().ok()?;
        let index_this = coverage.get(this)? as usize;
        let records = self.entry_exit_record();
        let offset_data = self.offset_data();
        let entry_this = records.get(index_this)?.entry_anchor(offset_data)?.ok()?;

        let mut iter = skipping_iterator_t::new(ctx, false);
        iter.reset_fast(iter.buffer.idx);

        let mut unsafe_from = 0;
        if !iter.prev(Some(&mut unsafe_from)) {
            ctx.buffer
                .unsafe_to_concat_from_outbuffer(Some(unsafe_from), Some(ctx.buffer.idx + 1));
            return None;
        }

        let i = iter.index();
        let prev = iter.buffer.info[i].as_glyph();
        let index_prev = coverage.get(prev)? as usize;
        let Some(exit_prev) = records
            .get(index_prev)
            .and_then(|rec| rec.exit_anchor(offset_data).transpose().ok().flatten())
        else {
            iter.buffer
                .unsafe_to_concat_from_outbuffer(Some(iter.index()), Some(iter.buffer.idx + 1));
            return None;
        };

        let (exit_x, exit_y) = ctx.face.ot_tables.resolve_anchor(&exit_prev);
        let (entry_x, entry_y) = ctx.face.ot_tables.resolve_anchor(&entry_this);

        let direction = ctx.buffer.direction;
        let j = ctx.buffer.idx;
        ctx.buffer.unsafe_to_break(Some(i), Some(j + 1));

        let pos = &mut ctx.buffer.pos;
        match direction {
            Direction::LeftToRight => {
                pos[i].x_advance = exit_x + pos[i].x_offset;
                let d = entry_x + pos[j].x_offset;
                pos[j].x_advance -= d;
                pos[j].x_offset -= d;
            }
            Direction::RightToLeft => {
                let d = exit_x + pos[i].x_offset;
                pos[i].x_advance -= d;
                pos[i].x_offset -= d;
                pos[j].x_advance = entry_x + pos[j].x_offset;
            }
            Direction::TopToBottom => {
                pos[i].y_advance = exit_y + pos[i].y_offset;
                let d = entry_y + pos[j].y_offset;
                pos[j].y_advance -= d;
                pos[j].y_offset -= d;
            }
            Direction::BottomToTop => {
                let d = exit_y + pos[i].y_offset;
                pos[i].y_advance -= d;
                pos[i].y_offset -= d;
                pos[j].y_advance = entry_y;
            }
            Direction::Invalid => {}
        }

        // Cross-direction adjustment

        // We attach child to parent (think graph theory and rooted trees whereas
        // the root stays on baseline and each node aligns itself against its
        // parent.
        //
        // Optimize things for the case of RightToLeft, as that's most common in
        // Arabic.
        let mut child = i;
        let mut parent = j;
        let mut x_offset = entry_x - exit_x;
        let mut y_offset = entry_y - exit_y;

        // Low bits are lookup flags, so we want to truncate.
        if ctx.lookup_props as u16 & lookup_flags::RIGHT_TO_LEFT == 0 {
            core::mem::swap(&mut child, &mut parent);
            x_offset = -x_offset;
            y_offset = -y_offset;
        }

        // If child was already connected to someone else, walk through its old
        // chain and reverse the link direction, such that the whole tree of its
        // previous connection now attaches to new parent.  Watch out for case
        // where new parent is on the path from old chain...
        reverse_cursive_minor_offset(pos, child, direction, parent);

        pos[child].set_attach_type(attach_type::CURSIVE);
        pos[child].set_attach_chain((parent as isize - child as isize) as i16);

        ctx.buffer.scratch_flags |= HB_BUFFER_SCRATCH_FLAG_HAS_GPOS_ATTACHMENT;
        if direction.is_horizontal() {
            pos[child].y_offset = y_offset;
        } else {
            pos[child].x_offset = x_offset;
        }

        // If parent was attached to child, separate them.
        // https://github.com/harfbuzz/harfbuzz/issues/2469
        if pos[parent].attach_chain() == -pos[child].attach_chain() {
            pos[parent].set_attach_chain(0);

            if direction.is_horizontal() {
                pos[parent].y_offset = 0;
            } else {
                pos[parent].x_offset = 0;
            }
        }

        ctx.buffer.idx += 1;
        Some(())
    }
}

fn reverse_cursive_minor_offset(
    pos: &mut [GlyphPosition],
    i: usize,
    direction: Direction,
    new_parent: usize,
) {
    let chain = pos[i].attach_chain();
    let attach_type = pos[i].attach_type();
    if chain == 0 || attach_type & attach_type::CURSIVE == 0 {
        return;
    }

    pos[i].set_attach_chain(0);

    // Stop if we see new parent in the chain.
    let j = (i as isize + isize::from(chain)) as _;
    if j == new_parent {
        return;
    }

    reverse_cursive_minor_offset(pos, j, direction, new_parent);

    if direction.is_horizontal() {
        pos[j].y_offset = -pos[i].y_offset;
    } else {
        pos[j].x_offset = -pos[i].x_offset;
    }

    pos[j].set_attach_chain(-chain);
    pos[j].set_attach_type(attach_type);
}
