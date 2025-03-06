use std::time::Duration;

use gpui::{ease_out_quint, Animation};

pub fn in_from_bottom(duration: f32) -> Animation {
    let duration = Duration::from_millis(duration as u64);
    Animation::new(duration).with_easing(ease_out_quint())
}
