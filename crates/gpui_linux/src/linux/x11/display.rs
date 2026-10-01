use std::time::Duration;

use anyhow::Context as _;
use uuid::Uuid;
use x11rb::{
    connection::Connection as _,
    protocol::{randr, randr::ConnectionExt as _, xproto},
    xcb_ffi::XCBConnection,
};

use gpui::{
    Bounds, DisplayId, DisplayPower, DisplayState, Pixels, PlatformDisplay, Point, Size, px,
};

#[derive(Debug, Clone)]
pub(crate) struct X11Display {
    x_screen_index: usize,
    bounds: Bounds<Pixels>,
    uuid: Uuid,
    /// The refresh interval of the monitor a window is on. An X screen spans
    /// all monitors, so this is only known for a window's display.
    pub(crate) refresh_interval: Option<Duration>,
}

impl X11Display {
    pub(crate) fn new(
        xcb: &XCBConnection,
        scale_factor: f32,
        x_screen_index: usize,
    ) -> anyhow::Result<Self> {
        let screen = xcb
            .setup()
            .roots
            .get(x_screen_index)
            .with_context(|| format!("No screen found with index {x_screen_index}"))?;
        Ok(Self {
            x_screen_index,
            bounds: Bounds {
                origin: Default::default(),
                size: Size {
                    width: px(screen.width_in_pixels as f32 / scale_factor),
                    height: px(screen.height_in_pixels as f32 / scale_factor),
                },
            },
            uuid: Uuid::from_bytes([0; 16]),
            refresh_interval: None,
        })
    }
}

impl PlatformDisplay for X11Display {
    fn id(&self) -> DisplayId {
        DisplayId::new(self.x_screen_index as u64)
    }

    fn uuid(&self) -> anyhow::Result<Uuid> {
        Ok(self.uuid)
    }

    fn bounds(&self) -> Bounds<Pixels> {
        self.bounds
    }

    // DPMS reports power only when polled, so it isn't tracked.
    fn state(&self) -> DisplayState {
        DisplayState {
            refresh_interval: self.refresh_interval,
            power: DisplayPower::Unknown,
        }
    }
}

/// The area and refresh interval of an enabled RandR CRTC, i.e. a monitor.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct CrtcMode {
    /// Position and size in root window pixels.
    pub bounds: Bounds<i32>,
    pub refresh_interval: Option<Duration>,
}

impl CrtcMode {
    /// The refresh interval of the monitor containing `point`, in root window
    /// pixels.
    pub(crate) fn refresh_interval_at(crtcs: &[CrtcMode], point: Point<i32>) -> Option<Duration> {
        crtcs
            .iter()
            .find(|crtc| crtc.bounds.contains(&point))
            .and_then(|crtc| crtc.refresh_interval)
    }
}

/// Reads the enabled CRTCs of the screen `root` belongs to.
pub(crate) fn query_crtc_modes(
    xcb: &XCBConnection,
    root: xproto::Window,
) -> anyhow::Result<Vec<CrtcMode>> {
    let resources = xcb
        .randr_get_screen_resources_current(root)?
        .reply()
        .context("querying RandR screen resources")?;
    let cookies = resources
        .crtcs
        .iter()
        .map(|crtc| xcb.randr_get_crtc_info(*crtc, resources.config_timestamp))
        .collect::<Result<Vec<_>, _>>()?;
    let mut crtcs = Vec::with_capacity(cookies.len());
    for cookie in cookies {
        let info = cookie.reply().context("querying RandR CRTC")?;
        if info.mode == 0 {
            continue;
        }
        let refresh_interval = resources
            .modes
            .iter()
            .find(|mode| mode.id == info.mode)
            .and_then(mode_refresh_interval);
        crtcs.push(CrtcMode {
            bounds: Bounds {
                origin: Point::new(i32::from(info.x), i32::from(info.y)),
                size: Size::new(i32::from(info.width), i32::from(info.height)),
            },
            refresh_interval,
        });
    }
    Ok(crtcs)
}

/// The refresh interval of a RandR mode, or `None` if its timings are unset.
pub(crate) fn mode_refresh_interval(mode: &randr::ModeInfo) -> Option<Duration> {
    let mut lines_per_frame = f64::from(mode.vtotal);
    if mode.mode_flags.contains(randr::ModeFlag::DOUBLE_SCAN) {
        lines_per_frame *= 2.0;
    }
    // An interlaced mode refreshes each field, half of the lines.
    if mode.mode_flags.contains(randr::ModeFlag::INTERLACE) {
        lines_per_frame /= 2.0;
    }
    DisplayState::refresh_interval_from_hz(
        f64::from(mode.dot_clock) / (f64::from(mode.htotal) * lines_per_frame),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mode(
        dot_clock: u32,
        htotal: u16,
        vtotal: u16,
        mode_flags: randr::ModeFlag,
    ) -> randr::ModeInfo {
        randr::ModeInfo {
            id: 1,
            width: 1920,
            height: 1080,
            dot_clock,
            hsync_start: 0,
            hsync_end: 0,
            htotal,
            hskew: 0,
            vsync_start: 0,
            vsync_end: 0,
            vtotal,
            name_len: 0,
            mode_flags,
        }
    }

    #[test]
    fn test_mode_refresh_interval() {
        let micros = |mode: randr::ModeInfo| {
            mode_refresh_interval(&mode).map(|interval| interval.as_micros())
        };
        // CEA-861 1080p60 and 1080i60 timings.
        assert_eq!(
            micros(mode(148_500_000, 2200, 1125, randr::ModeFlag::default())),
            Some(16_666)
        );
        assert_eq!(
            micros(mode(74_250_000, 2200, 1125, randr::ModeFlag::INTERLACE)),
            Some(16_666)
        );
        assert_eq!(
            micros(mode(148_500_000, 2200, 1125, randr::ModeFlag::DOUBLE_SCAN)),
            Some(33_333)
        );
        assert_eq!(
            micros(mode(0, 2200, 1125, randr::ModeFlag::default())),
            None
        );
        assert_eq!(
            micros(mode(148_500_000, 0, 1125, randr::ModeFlag::default())),
            None
        );
    }

    #[test]
    fn test_refresh_interval_at() {
        let crtcs = [
            CrtcMode {
                bounds: Bounds::new(Point::new(0, 0), Size::new(1920, 1080)),
                refresh_interval: Some(Duration::from_secs(1) / 60),
            },
            CrtcMode {
                bounds: Bounds::new(Point::new(1920, 0), Size::new(2560, 1440)),
                refresh_interval: Some(Duration::from_secs(1) / 144),
            },
        ];
        assert_eq!(
            CrtcMode::refresh_interval_at(&crtcs, Point::new(100, 100)),
            Some(Duration::from_secs(1) / 60)
        );
        assert_eq!(
            CrtcMode::refresh_interval_at(&crtcs, Point::new(2000, 100)),
            Some(Duration::from_secs(1) / 144)
        );
        assert_eq!(
            CrtcMode::refresh_interval_at(&crtcs, Point::new(100, 1200)),
            None
        );
    }
}
