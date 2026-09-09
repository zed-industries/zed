use std::{sync::OnceLock, time::Duration};

use crate::Damage;

pub(crate) fn log_frame(
    lower: Duration,
    bin: Duration,
    damage_tracking: Duration,
    raster: Duration,
    operation_count: usize,
    damage: &Damage,
) {
    static ENABLED: OnceLock<bool> = OnceLock::new();
    if !ENABLED.get_or_init(|| {
        std::env::var("GPUI_SOFTWARE_STATS").is_ok_and(|value| value == "1" || value == "true")
    }) {
        return;
    }
    let damaged_pixels = damage
        .rects
        .iter()
        .map(|rect| {
            u64::try_from(rect.size.width.0.max(0)).unwrap_or_default()
                * u64::try_from(rect.size.height.0.max(0)).unwrap_or_default()
        })
        .sum::<u64>();
    log::info!(
        "gpui_software: ops={operation_count} damaged_pixels={damaged_pixels} lower={lower:?} bin={bin:?} damage={damage_tracking:?} raster={raster:?}"
    );
}
