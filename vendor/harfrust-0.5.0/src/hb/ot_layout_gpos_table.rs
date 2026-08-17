#[cfg(not(feature = "std"))]
#[allow(unused_imports)]
use core_maths::CoreFloat as _;

use super::buffer::*;
use super::hb_font_t;
use super::ot_layout::*;
use super::ot_shape_plan::hb_ot_shape_plan_t;
use crate::Direction;

pub fn position(plan: &hb_ot_shape_plan_t, face: &hb_font_t, buffer: &mut hb_buffer_t) {
    apply_layout_table(plan, face, buffer, face.ot_tables.gpos.as_ref());
}

pub mod attach_type {
    pub const MARK: u8 = 1;
    pub const CURSIVE: u8 = 2;
}

/// See <https://github.com/harfbuzz/harfbuzz/blob/7d936359a27abb2d7cb14ecc102463bb15c11843/src/OT/Layout/GPOS/GPOS.hh#L75>
fn propagate_attachment_offsets(
    pos: &mut [GlyphPosition],
    len: usize,
    i: usize,
    direction: Direction,
    nesting_level: usize,
) {
    // Adjusts offsets of attached glyphs (both cursive and mark) to accumulate
    // offset of glyph they are attached to.
    let chain = pos[i].attach_chain();
    let kind = pos[i].attach_type();

    pos[i].set_attach_chain(0);

    let j = (i as isize + isize::from(chain)) as usize;
    if j >= len {
        return;
    }

    if nesting_level == 0 {
        return;
    }

    if pos[j].attach_chain() != 0 {
        propagate_attachment_offsets(pos, len, j, direction, nesting_level - 1);
    }

    match kind {
        attach_type::MARK => {
            pos[i].x_offset += pos[j].x_offset;
            pos[i].y_offset += pos[j].y_offset;

            // i is the position of the mark; j is the base.
            if j < i {
                // This is the common case: mark follows base.
                // And currently the only way in OpenType.
                if direction.is_forward() {
                    for k in j..i {
                        pos[i].x_offset -= pos[k].x_advance;
                        pos[i].y_offset -= pos[k].y_advance;
                    }
                } else {
                    for k in j + 1..i + 1 {
                        pos[i].x_offset += pos[k].x_advance;
                        pos[i].y_offset += pos[k].y_advance;
                    }
                }
            } else {
                // This can happen with `kerx`: a mark attaching
                // to a base after it in the logical order.
                if direction.is_forward() {
                    for k in i..j {
                        pos[i].x_offset += pos[k].x_advance;
                        pos[i].y_offset += pos[k].y_advance;
                    }
                } else {
                    for k in i + 1..j + 1 {
                        pos[i].x_offset -= pos[k].x_advance;
                        pos[i].y_offset -= pos[k].y_advance;
                    }
                }
            }
        }
        attach_type::CURSIVE => {
            if direction.is_horizontal() {
                pos[i].y_offset += pos[j].y_offset;
            } else {
                pos[i].x_offset += pos[j].x_offset;
            }
        }
        _ => {}
    }
}

pub mod GPOS {
    use super::*;

    pub fn position_start(_: &hb_font_t, buffer: &mut hb_buffer_t) {
        let len = buffer.len;
        for pos in &mut buffer.pos[..len] {
            pos.set_attach_chain(0);
            pos.set_attach_type(0);
        }
    }

    pub fn position_finish_advances(_: &hb_font_t, _: &mut hb_buffer_t) {
        //buffer.assert_gsubgpos_vars();
    }

    pub fn position_finish_offsets(_: &hb_font_t, buffer: &mut hb_buffer_t) {
        buffer.assert_gsubgpos_vars();

        let len = buffer.len;
        let direction = buffer.direction;

        // Handle attachments
        if buffer.scratch_flags & HB_BUFFER_SCRATCH_FLAG_HAS_GPOS_ATTACHMENT != 0 {
            // https://github.com/harfbuzz/harfbuzz/issues/5514
            if buffer.direction.is_forward() {
                for i in 0..len {
                    if buffer.pos[i].attach_chain() != 0 {
                        propagate_attachment_offsets(
                            &mut buffer.pos,
                            len,
                            i,
                            direction,
                            MAX_NESTING_LEVEL,
                        );
                    }
                }
            } else {
                for i in (0..len).rev() {
                    if buffer.pos[i].attach_chain() != 0 {
                        propagate_attachment_offsets(
                            &mut buffer.pos,
                            len,
                            i,
                            direction,
                            MAX_NESTING_LEVEL,
                        );
                    }
                }
            }
        }
    }
}
