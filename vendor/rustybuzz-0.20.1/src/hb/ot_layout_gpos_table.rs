#[cfg(not(feature = "std"))]
use core_maths::CoreFloat;

use super::buffer::*;
use super::hb_font_t;
use super::ot_layout::*;
use super::ot_layout_common::{PositioningLookup, PositioningTable};
use super::ot_layout_gsubgpos::{Apply, OT::hb_ot_apply_context_t};
use super::ot_shape_plan::hb_ot_shape_plan_t;
use crate::Direction;
use ttf_parser::gpos::*;
use ttf_parser::opentype_layout::LookupIndex;

pub fn position(plan: &hb_ot_shape_plan_t, face: &hb_font_t, buffer: &mut hb_buffer_t) {
    apply_layout_table(plan, face, buffer, face.gpos.as_ref());
}

pub(crate) trait ValueRecordExt {
    fn is_empty(&self) -> bool;
    fn apply(&self, ctx: &mut hb_ot_apply_context_t, idx: usize) -> bool;
    fn apply_to_pos(&self, ctx: &mut hb_ot_apply_context_t, pos: &mut GlyphPosition) -> bool;
}

impl ValueRecordExt for ValueRecord<'_> {
    fn is_empty(&self) -> bool {
        self.x_placement == 0
            && self.y_placement == 0
            && self.x_advance == 0
            && self.y_advance == 0
            && self.x_placement_device.is_none()
            && self.y_placement_device.is_none()
            && self.x_advance_device.is_none()
            && self.y_advance_device.is_none()
    }

    fn apply(&self, ctx: &mut hb_ot_apply_context_t, idx: usize) -> bool {
        let mut pos = ctx.buffer.pos[idx];
        let worked = self.apply_to_pos(ctx, &mut pos);
        ctx.buffer.pos[idx] = pos;
        worked
    }

    fn apply_to_pos(&self, ctx: &mut hb_ot_apply_context_t, pos: &mut GlyphPosition) -> bool {
        let horizontal = ctx.buffer.direction.is_horizontal();
        let mut worked = false;

        if self.x_placement != 0 {
            pos.x_offset += i32::from(self.x_placement);
            worked = true;
        }

        if self.y_placement != 0 {
            pos.y_offset += i32::from(self.y_placement);
            worked = true;
        }

        if self.x_advance != 0 && horizontal {
            pos.x_advance += i32::from(self.x_advance);
            worked = true;
        }

        if self.y_advance != 0 && !horizontal {
            // y_advance values grow downward but font-space grows upward, hence negation
            pos.y_advance -= i32::from(self.y_advance);
            worked = true;
        }

        {
            let (ppem_x, ppem_y) = ctx.face.pixels_per_em().unwrap_or((0, 0));
            let coords = ctx.face.ttfp_face.variation_coordinates().len();
            let use_x_device = ppem_x != 0 || coords != 0;
            let use_y_device = ppem_y != 0 || coords != 0;

            if use_x_device {
                if let Some(device) = self.x_placement_device {
                    pos.x_offset += device.get_x_delta(ctx.face).unwrap_or(0);
                    worked = true; // TODO: even when 0?
                }
            }

            if use_y_device {
                if let Some(device) = self.y_placement_device {
                    pos.y_offset += device.get_y_delta(ctx.face).unwrap_or(0);
                    worked = true;
                }
            }

            if horizontal && use_x_device {
                if let Some(device) = self.x_advance_device {
                    pos.x_advance += device.get_x_delta(ctx.face).unwrap_or(0);
                    worked = true;
                }
            }

            if !horizontal && use_y_device {
                if let Some(device) = self.y_advance_device {
                    // y_advance values grow downward but face-space grows upward, hence negation
                    pos.y_advance -= device.get_y_delta(ctx.face).unwrap_or(0);
                    worked = true;
                }
            }
        }

        worked
    }
}

pub(crate) trait AnchorExt {
    fn get(&self, face: &hb_font_t) -> (i32, i32);
}

impl AnchorExt for Anchor<'_> {
    fn get(&self, face: &hb_font_t) -> (i32, i32) {
        let mut x = i32::from(self.x);
        let mut y = i32::from(self.y);

        if self.x_device.is_some() || self.y_device.is_some() {
            let (ppem_x, ppem_y) = face.pixels_per_em().unwrap_or((0, 0));
            let coords = face.ttfp_face.variation_coordinates().len();

            if let Some(device) = self.x_device {
                if ppem_x != 0 || coords != 0 {
                    x += device.get_x_delta(face).unwrap_or(0);
                }
            }

            if let Some(device) = self.y_device {
                if ppem_y != 0 || coords != 0 {
                    y += device.get_y_delta(face).unwrap_or(0);
                }
            }
        }

        (x, y)
    }
}

impl<'a> LayoutTable for PositioningTable<'a> {
    const INDEX: TableIndex = TableIndex::GPOS;
    const IN_PLACE: bool = true;

    type Lookup = PositioningLookup<'a>;

    fn get_lookup(&self, index: LookupIndex) -> Option<&Self::Lookup> {
        self.lookups.get(usize::from(index))
    }
}

pub mod attach_type {
    pub const MARK: u8 = 1;
    pub const CURSIVE: u8 = 2;
}

/// Just like TryFrom<N>, but for numeric types not supported by the Rust's std.
pub(crate) trait TryNumFrom<T>: Sized {
    /// Casts between numeric types.
    fn try_num_from(_: T) -> Option<Self>;
}

impl TryNumFrom<f32> for i32 {
    #[inline]
    fn try_num_from(v: f32) -> Option<Self> {
        // Based on https://github.com/rust-num/num-traits/blob/master/src/cast.rs

        // Float as int truncates toward zero, so we want to allow values
        // in the exclusive range `(MIN-1, MAX+1)`.

        // We can't represent `MIN-1` exactly, but there's no fractional part
        // at this magnitude, so we can just use a `MIN` inclusive boundary.
        const MIN: f32 = i32::MIN as f32;
        // We can't represent `MAX` exactly, but it will round up to exactly
        // `MAX+1` (a power of two) when we cast it.
        const MAX_P1: f32 = i32::MAX as f32;
        if v >= MIN && v < MAX_P1 {
            Some(v as i32)
        } else {
            None
        }
    }
}

pub(crate) trait DeviceExt {
    fn get_x_delta(&self, face: &hb_font_t) -> Option<i32>;
    fn get_y_delta(&self, face: &hb_font_t) -> Option<i32>;
}

impl DeviceExt for Device<'_> {
    fn get_x_delta(&self, face: &hb_font_t) -> Option<i32> {
        match self {
            Device::Hinting(hinting) => hinting.x_delta(face.units_per_em, face.pixels_per_em()),
            Device::Variation(variation) => face
                .tables()
                .gdef?
                .glyph_variation_delta(
                    variation.outer_index,
                    variation.inner_index,
                    face.variation_coordinates(),
                )
                .and_then(|float| i32::try_num_from(float.round())),
        }
    }

    fn get_y_delta(&self, face: &hb_font_t) -> Option<i32> {
        match self {
            Device::Hinting(hinting) => hinting.y_delta(face.units_per_em, face.pixels_per_em()),
            Device::Variation(variation) => face
                .tables()
                .gdef?
                .glyph_variation_delta(
                    variation.outer_index,
                    variation.inner_index,
                    face.variation_coordinates(),
                )
                .and_then(|float| i32::try_num_from(float.round())),
        }
    }
}

impl Apply for PositioningSubtable<'_> {
    fn apply(&self, ctx: &mut hb_ot_apply_context_t) -> Option<()> {
        match self {
            Self::Single(t) => t.apply(ctx),
            Self::Pair(t) => t.apply(ctx),
            Self::Cursive(t) => t.apply(ctx),
            Self::MarkToBase(t) => t.apply(ctx),
            Self::MarkToLigature(t) => t.apply(ctx),
            Self::MarkToMark(t) => t.apply(ctx),
            Self::Context(t) => t.apply(ctx),
            Self::ChainContext(t) => t.apply(ctx),
        }
    }
}

fn propagate_attachment_offsets(
    pos: &mut [GlyphPosition],
    len: usize,
    i: usize,
    direction: Direction,
) {
    // Adjusts offsets of attached glyphs (both cursive and mark) to accumulate
    // offset of glyph they are attached to.
    let chain = pos[i].attach_chain();
    let kind = pos[i].attach_type();
    if chain == 0 {
        return;
    }

    pos[i].set_attach_chain(0);

    let j = (i as isize + isize::from(chain)) as _;
    if j >= len {
        return;
    }

    propagate_attachment_offsets(pos, len, j, direction);

    match kind {
        attach_type::MARK => {
            pos[i].x_offset += pos[j].x_offset;
            pos[i].y_offset += pos[j].y_offset;

            assert!(j < i);
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

    pub fn position_finish_advances(_: &hb_font_t, _: &mut hb_buffer_t) {}

    pub fn position_finish_offsets(_: &hb_font_t, buffer: &mut hb_buffer_t) {
        let len = buffer.len;
        let direction = buffer.direction;

        // Handle attachments
        if buffer.scratch_flags & HB_BUFFER_SCRATCH_FLAG_HAS_GPOS_ATTACHMENT != 0 {
            for i in 0..len {
                propagate_attachment_offsets(&mut buffer.pos, len, i, direction);
            }
        }
    }
}
