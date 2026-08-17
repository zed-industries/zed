#[cfg(not(feature = "std"))]
use core_maths::CoreFloat;

use super::buffer::hb_buffer_t;
use super::hb_font_t;
use super::ot_shape_plan::hb_ot_shape_plan_t;

pub fn apply(plan: &hb_ot_shape_plan_t, face: &hb_font_t, buffer: &mut hb_buffer_t) -> Option<()> {
    let trak_mask = plan.trak_mask;

    let ptem = face.points_per_em?;
    if ptem <= 0.0 {
        return None;
    }

    let trak = face.tables().trak?;

    if !buffer.have_positions {
        buffer.clear_positions();
    }

    if buffer.direction.is_horizontal() {
        let tracking = trak.hor_tracking(ptem)?;
        let advance_to_add = tracking;
        let offset_to_add = tracking / 2;
        foreach_grapheme!(buffer, start, end, {
            if buffer.info[start].mask & trak_mask != 0 {
                buffer.pos[start].x_advance += advance_to_add;
                buffer.pos[start].x_offset += offset_to_add;
            }
        });
    } else {
        let tracking = trak.ver_tracking(ptem)?;
        let advance_to_add = tracking;
        let offset_to_add = tracking / 2;
        foreach_grapheme!(buffer, start, end, {
            if buffer.info[start].mask & trak_mask != 0 {
                buffer.pos[start].y_advance += advance_to_add;
                buffer.pos[start].y_offset += offset_to_add;
            }
        });
    }

    Some(())
}

trait TrackTableExt {
    fn hor_tracking(&self, ptem: f32) -> Option<i32>;
    fn ver_tracking(&self, ptem: f32) -> Option<i32>;
}

impl TrackTableExt for ttf_parser::trak::Table<'_> {
    fn hor_tracking(&self, ptem: f32) -> Option<i32> {
        self.horizontal.tracking(ptem)
    }

    fn ver_tracking(&self, ptem: f32) -> Option<i32> {
        self.vertical.tracking(ptem)
    }
}

trait TrackTableDataExt {
    fn tracking(&self, ptem: f32) -> Option<i32>;
    fn interpolate_at(
        &self,
        idx: u16,
        target_size: f32,
        track: &ttf_parser::trak::Track,
    ) -> Option<f32>;
}

impl TrackTableDataExt for ttf_parser::trak::TrackData<'_> {
    fn tracking(&self, ptem: f32) -> Option<i32> {
        // Choose track.
        let track = self.tracks.into_iter().find(|t| t.value == 0.0)?;

        // Choose size.
        if self.sizes.is_empty() {
            return None;
        }

        let mut idx = self
            .sizes
            .into_iter()
            .position(|s| s.0 >= ptem)
            .unwrap_or(self.sizes.len() as usize - 1);

        idx = idx.saturating_sub(1);

        self.interpolate_at(idx as u16, ptem, &track)
            .map(|n| n.round() as i32)
    }

    fn interpolate_at(
        &self,
        idx: u16,
        target_size: f32,
        track: &ttf_parser::trak::Track,
    ) -> Option<f32> {
        debug_assert!(idx < self.sizes.len() - 1);

        let s0 = self.sizes.get(idx)?.0;
        let s1 = self.sizes.get(idx + 1)?.0;

        let t = if s0 == s1 {
            0.0
        } else {
            (target_size - s0) / (s1 - s0)
        };

        let n =
            t * (track.values.get(idx + 1)? as f32) + (1.0 - t) * (track.values.get(idx)? as f32);

        Some(n)
    }
}
