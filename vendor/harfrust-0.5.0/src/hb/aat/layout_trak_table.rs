#[cfg(not(feature = "std"))]
#[allow(unused_imports)]
use core_maths::CoreFloat as _;

use read_fonts::tables::trak::TrackTableEntry;
use read_fonts::types::{BigEndian, Fixed};
use read_fonts::FontData;

use crate::hb::{buffer::hb_buffer_t, hb_font_t, ot_shape_plan::hb_ot_shape_plan_t};

pub fn apply(_plan: &hb_ot_shape_plan_t, face: &hb_font_t, buffer: &mut hb_buffer_t) -> Option<()> {
    let trak = face.aat_tables.trak.as_ref()?;
    let mut ptem = face.points_per_em.unwrap_or(0.0);
    if ptem <= 0.0 {
        ptem = 12.0; // CoreText fallback
    }

    if !buffer.have_positions {
        buffer.clear_positions();
    }

    let advance_to_add = if buffer.direction.is_horizontal() {
        trak.get_h_tracking(ptem, 0.0)
    } else {
        trak.get_v_tracking(ptem, 0.0)
    };

    foreach_grapheme!(buffer, start, end, {
        if buffer.direction.is_horizontal() {
            buffer.pos[start].x_advance += advance_to_add;
        } else {
            buffer.pos[start].y_advance += advance_to_add;
        }
    });

    Some(())
}

trait TrakExt {
    fn get_h_tracking(&self, ptem: f32, track: f32) -> i32;
    fn get_v_tracking(&self, ptem: f32, track: f32) -> i32;
}

impl TrakExt for read_fonts::tables::trak::Trak<'_> {
    fn get_h_tracking(&self, ptem: f32, track: f32) -> i32 {
        self.horiz().transpose().ok().flatten().map_or(0, |t| {
            t.get_tracking(self.offset_data(), ptem, track).round() as i32
        })
    }

    fn get_v_tracking(&self, ptem: f32, track: f32) -> i32 {
        self.vert().transpose().ok().flatten().map_or(0, |t| {
            t.get_tracking(self.offset_data(), ptem, track).round() as i32
        })
    }
}

trait TrackDataExt {
    fn get_tracking(&self, offset_data: FontData, ptem: f32, track: f32) -> f32;
}

impl TrackDataExt for read_fonts::tables::trak::TrackData<'_> {
    fn get_tracking(&self, offset_data: FontData, ptem: f32, track: f32) -> f32 {
        let Ok(sizes) = self.size_table(offset_data) else {
            return 0.0;
        };

        if sizes.is_empty() {
            return 0.0;
        }

        let tracks = self.track_table();
        if tracks.is_empty() {
            return 0.0;
        }

        let get_value = |entry: &TrackTableEntry| {
            let Ok(values) = entry.per_size_values(offset_data, sizes.len() as u16) else {
                return 0.0;
            };
            entry.get_value(ptem, sizes, values)
        };

        if tracks.len() == 1 {
            return tracks.first().map_or(0.0, get_value);
        }

        let mut i = 0;
        let mut j = tracks.len() - 1;

        while i + 1 < tracks.len() && tracks.get(i + 1).map_or(0.0, |t| t.track().to_f32()) <= track
        {
            i += 1;
        }
        while j > 0 && tracks.get(j - 1).map_or(0.0, |t| t.track().to_f32()) >= track {
            j -= 1;
        }

        if i == j {
            return tracks.get(i).map_or(0.0, get_value);
        }

        let t0 = tracks.get(i).map_or(0.0, |t| t.track().to_f32());
        let t1 = tracks.get(j).map_or(0.0, |t| t.track().to_f32());
        let interp = if (t1 - t0).abs() < f32::EPSILON {
            0.0
        } else {
            (track - t0) / (t1 - t0)
        };

        let a = tracks.get(i).map_or(0.0, get_value);
        let b = tracks.get(j).map_or(0.0, get_value);
        a + interp * (b - a)
    }
}

trait TrackEntryExt {
    fn get_value(&self, ptem: f32, sizes: &[BigEndian<Fixed>], values: &[BigEndian<i16>]) -> f32;
}

impl TrackEntryExt for TrackTableEntry {
    fn get_value(&self, ptem: f32, sizes: &[BigEndian<Fixed>], values: &[BigEndian<i16>]) -> f32 {
        let n = sizes.len().min(values.len());
        if n == 0 {
            return 0.0;
        }

        for i in 0..n {
            let size_pt = sizes.get(i).map_or(0.0, |f| f.get().to_f32());
            if size_pt >= ptem {
                if i == 0 {
                    return values.first().map(|v| v.get() as f32).unwrap_or_default();
                }

                let s0 = sizes.get(i - 1).map_or(0.0, |f| f.get().to_f32());
                let s1 = size_pt;
                let v0 = values
                    .get(i - 1)
                    .map(|v| v.get() as f32)
                    .unwrap_or_default();
                let v1 = values.get(i).map(|v| v.get() as f32).unwrap_or_default();

                if (s1 - s0).abs() < f32::EPSILON {
                    return (v0 + v1) * 0.5;
                }

                let t = (ptem - s0) / (s1 - s0);
                return v0 + t * (v1 - v0);
            }
        }

        values
            .get(n - 1)
            .map(|v| v.get() as f32)
            .unwrap_or_default()
    }
}
