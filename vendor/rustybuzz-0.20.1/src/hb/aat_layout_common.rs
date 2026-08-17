use crate::hb::aat_map::range_flags_t;
use crate::hb::buffer::hb_buffer_t;
use crate::hb::face::hb_font_t;
use crate::hb::hb_mask_t;

pub struct hb_aat_apply_context_t<'a> {
    pub face: &'a hb_font_t<'a>,
    pub buffer: &'a mut hb_buffer_t,
    pub range_flags: Option<&'a mut [range_flags_t]>,
    pub subtable_flags: hb_mask_t,
}

impl<'a> hb_aat_apply_context_t<'a> {
    pub fn new(face: &'a hb_font_t<'a>, buffer: &'a mut hb_buffer_t) -> Self {
        Self {
            face,
            buffer,
            range_flags: None,
            subtable_flags: 0,
        }
    }
}
