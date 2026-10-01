use std::{
    fmt::Debug,
    hash::{Hash, Hasher},
    time::Duration,
};

use anyhow::Context as _;
use uuid::Uuid;
use wayland_backend::client::ObjectId;

use gpui::{Bounds, DisplayId, DisplayPower, DisplayState, Pixels, PlatformDisplay};

#[derive(Debug, Clone)]
pub(crate) struct WaylandDisplay {
    /// The ID of the wl_output object
    pub id: ObjectId,
    pub name: Option<String>,
    pub bounds: Bounds<Pixels>,
    pub refresh_interval: Option<Duration>,
}

impl Hash for WaylandDisplay {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl PlatformDisplay for WaylandDisplay {
    fn id(&self) -> DisplayId {
        DisplayId::new(self.id.protocol_id() as u64)
    }

    fn uuid(&self) -> anyhow::Result<Uuid> {
        let name = self
            .name
            .as_ref()
            .context("Wayland display does not have a name")?;
        Ok(Uuid::new_v5(&Uuid::NAMESPACE_DNS, name.as_bytes()))
    }

    fn bounds(&self) -> Bounds<Pixels> {
        self.bounds
    }

    // Display power is only available through wlr-output-power-management,
    // whose controls are exclusive per output in wlroots: holding one would
    // stop the user's idle daemon from turning the output off.
    fn state(&self) -> DisplayState {
        DisplayState {
            refresh_interval: self.refresh_interval,
            power: DisplayPower::Unknown,
        }
    }
}

/// Converts a `wl_output.mode` refresh rate, in millihertz, to an interval.
/// Compositors send 0 when the rate doesn't apply, e.g. for virtual outputs.
pub(crate) fn refresh_interval_from_millihertz(millihertz: i32) -> Option<Duration> {
    DisplayState::refresh_interval_from_hz(f64::from(millihertz) / 1000.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_refresh_interval_from_millihertz() {
        assert_eq!(
            refresh_interval_from_millihertz(60_000).map(|interval| interval.as_micros()),
            Some(16_666)
        );
        assert_eq!(
            refresh_interval_from_millihertz(59_940).map(|interval| interval.as_micros()),
            Some(16683)
        );
        assert_eq!(
            refresh_interval_from_millihertz(143_998).map(|interval| interval.as_micros()),
            Some(6944)
        );
        assert_eq!(refresh_interval_from_millihertz(0), None);
        assert_eq!(refresh_interval_from_millihertz(-1), None);
    }
}
