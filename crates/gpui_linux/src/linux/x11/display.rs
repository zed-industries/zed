use std::time::Duration;

use anyhow::Context as _;
use uuid::Uuid;
use x11rb::{
    connection::Connection as _,
    protocol::randr::{self, ConnectionExt as _},
    xcb_ffi::XCBConnection,
};

use gpui::{Bounds, DisplayId, Pixels, PlatformDisplay, Point, Size, px};

#[derive(Debug)]
pub(crate) struct X11Display {
    id: DisplayId,
    /// In device pixels, relative to the X screen's root window.
    device_bounds: Bounds<i32>,
    bounds: Bounds<Pixels>,
    uuid: Uuid,
    refresh_interval: Option<Duration>,
}

impl X11Display {
    /// The monitors of an X screen, one per active RandR CRTC, with the
    /// primary monitor first. Falls back to the whole X screen, whose refresh
    /// rate is unknown, when the server doesn't support RandR 1.2.
    pub(crate) fn all(
        xcb: &XCBConnection,
        scale_factor: f32,
        x_screen_index: usize,
    ) -> anyhow::Result<Vec<Self>> {
        match Self::monitors(xcb, scale_factor, x_screen_index) {
            Ok(monitors) if !monitors.is_empty() => Ok(monitors),
            Ok(_) => Ok(vec![Self::screen(xcb, scale_factor, x_screen_index)?]),
            Err(error) => {
                log::info!("Using the whole X screen as one display: {error:#}");
                Ok(vec![Self::screen(xcb, scale_factor, x_screen_index)?])
            }
        }
    }

    /// The X screen with the given index, from its [`X11Display::id`].
    pub(crate) fn screen_index(id: DisplayId) -> usize {
        (u64::from(id) >> 32) as usize
    }

    fn screen(
        xcb: &XCBConnection,
        scale_factor: f32,
        x_screen_index: usize,
    ) -> anyhow::Result<Self> {
        let screen = xcb
            .setup()
            .roots
            .get(x_screen_index)
            .with_context(|| format!("No screen found with index {x_screen_index}"))?;
        let device_bounds = Bounds {
            origin: Point::default(),
            size: Size {
                width: i32::from(screen.width_in_pixels),
                height: i32::from(screen.height_in_pixels),
            },
        };
        Ok(Self {
            id: display_id(x_screen_index, 0),
            device_bounds,
            bounds: logical_bounds(device_bounds, scale_factor),
            uuid: Uuid::from_bytes([0; 16]),
            refresh_interval: None,
        })
    }

    fn monitors(
        xcb: &XCBConnection,
        scale_factor: f32,
        x_screen_index: usize,
    ) -> anyhow::Result<Vec<Self>> {
        let root = xcb
            .setup()
            .roots
            .get(x_screen_index)
            .with_context(|| format!("No screen found with index {x_screen_index}"))?
            .root;
        let version = xcb.randr_query_version(1, 2)?.reply()?;
        anyhow::ensure!(
            (version.major_version, version.minor_version) >= (1, 2),
            "RandR {}.{} doesn't report monitors",
            version.major_version,
            version.minor_version
        );
        let primary_output = xcb
            .randr_get_output_primary(root)?
            .reply()
            .map(|reply| reply.output)
            .unwrap_or(x11rb::NONE);
        let resources = xcb.randr_get_screen_resources_current(root)?.reply()?;
        let crtc_cookies = resources
            .crtcs
            .iter()
            .map(|crtc| {
                (
                    *crtc,
                    xcb.randr_get_crtc_info(*crtc, resources.config_timestamp),
                )
            })
            .collect::<Vec<_>>();

        let mut monitors = Vec::new();
        for (crtc, cookie) in crtc_cookies {
            let info = match cookie.map(|cookie| cookie.reply()) {
                Ok(Ok(info)) => info,
                Ok(Err(error)) => {
                    log::warn!("Failed to get RandR CRTC {crtc}: {error}");
                    continue;
                }
                Err(error) => {
                    log::warn!("Failed to get RandR CRTC {crtc}: {error}");
                    continue;
                }
            };
            if info.mode == x11rb::NONE || info.width == 0 || info.height == 0 {
                continue;
            }
            let device_bounds = Bounds {
                origin: Point {
                    x: i32::from(info.x),
                    y: i32::from(info.y),
                },
                size: Size {
                    width: i32::from(info.width),
                    height: i32::from(info.height),
                },
            };
            // Mirrored outputs on separate CRTCs show the same region; one
            // display is enough for it.
            if monitors
                .iter()
                .any(|(_, monitor): &(bool, Self)| monitor.device_bounds == device_bounds)
            {
                continue;
            }
            let output_name = info.outputs.first().and_then(|output| {
                let output_info = xcb
                    .randr_get_output_info(*output, resources.config_timestamp)
                    .ok()?
                    .reply()
                    .ok()?;
                String::from_utf8(output_info.name).ok()
            });
            let refresh_interval = resources
                .modes
                .iter()
                .find(|mode| mode.id == info.mode)
                .and_then(mode_refresh_interval);
            let is_primary = info.outputs.contains(&primary_output);
            monitors.push((
                is_primary,
                Self {
                    id: display_id(x_screen_index, crtc),
                    device_bounds,
                    bounds: logical_bounds(device_bounds, scale_factor),
                    // Output names (e.g. `DP-1`) are stable across sessions
                    // while CRTC ids aren't.
                    uuid: output_name.map_or(Uuid::from_bytes([0; 16]), |name| {
                        Uuid::new_v5(&Uuid::NAMESPACE_OID, name.as_bytes())
                    }),
                    refresh_interval,
                },
            ));
        }
        monitors.sort_by_key(|(is_primary, _)| !is_primary);
        Ok(monitors.into_iter().map(|(_, monitor)| monitor).collect())
    }
}

/// The display showing the largest part of `window_bounds`, both in device
/// pixels relative to the root window, or `None` if it's on none of them.
pub(crate) fn display_for_bounds<D: AsRef<X11Display>>(
    displays: &[D],
    window_bounds: Bounds<i32>,
) -> Option<&D> {
    displays
        .iter()
        .map(|display| {
            (
                display,
                overlap_area(display.as_ref().device_bounds, window_bounds),
            )
        })
        .filter(|(_, area)| *area > 0)
        // `max_by_key` keeps the last maximum; prefer earlier displays, which
        // start with the primary one.
        .rev()
        .max_by_key(|(_, area)| *area)
        .map(|(display, _)| display)
}

fn overlap_area(a: Bounds<i32>, b: Bounds<i32>) -> i64 {
    let left = a.origin.x.max(b.origin.x);
    let top = a.origin.y.max(b.origin.y);
    let right = (a.origin.x + a.size.width).min(b.origin.x + b.size.width);
    let bottom = (a.origin.y + a.size.height).min(b.origin.y + b.size.height);
    i64::from((right - left).max(0)) * i64::from((bottom - top).max(0))
}

/// Combines the X screen and CRTC into one id, so the screen to create a
/// window on can be recovered from a display's id.
fn display_id(x_screen_index: usize, crtc: randr::Crtc) -> DisplayId {
    DisplayId::new(((x_screen_index as u64) << 32) | u64::from(crtc))
}

fn logical_bounds(device_bounds: Bounds<i32>, scale_factor: f32) -> Bounds<Pixels> {
    device_bounds.map(|value| px(value as f32 / scale_factor))
}

/// The refresh interval of a RandR mode, or `None` if its timings are unknown.
pub(crate) fn mode_refresh_interval(mode: &randr::ModeInfo) -> Option<Duration> {
    let mut pixels_per_frame = u64::from(mode.htotal) * u64::from(mode.vtotal);
    if mode.mode_flags.contains(randr::ModeFlag::DOUBLE_SCAN) {
        pixels_per_frame *= 2;
    }
    if mode.mode_flags.contains(randr::ModeFlag::INTERLACE) {
        pixels_per_frame /= 2;
    }
    if mode.dot_clock == 0 || pixels_per_frame == 0 {
        return None;
    }
    Some(Duration::from_nanos(
        pixels_per_frame * 1_000_000_000 / u64::from(mode.dot_clock),
    ))
}

impl PlatformDisplay for X11Display {
    fn refresh_interval(&self) -> Option<Duration> {
        self.refresh_interval
    }

    fn id(&self) -> DisplayId {
        self.id
    }

    fn uuid(&self) -> anyhow::Result<Uuid> {
        Ok(self.uuid)
    }

    fn bounds(&self) -> Bounds<Pixels> {
        self.bounds
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{point, size};

    fn display(x: i32, width: i32) -> X11Display {
        let device_bounds = Bounds {
            origin: point(x, 0),
            size: size(width, 1000),
        };
        X11Display {
            id: display_id(0, x as u32 + 1),
            device_bounds,
            bounds: logical_bounds(device_bounds, 1.0),
            uuid: Uuid::from_bytes([0; 16]),
            refresh_interval: None,
        }
    }

    impl AsRef<X11Display> for X11Display {
        fn as_ref(&self) -> &X11Display {
            self
        }
    }

    #[test]
    fn windows_belong_to_the_display_showing_most_of_them() {
        let displays = [display(0, 1000), display(1000, 2000)];
        let window = |x| Bounds {
            origin: point(x, 100),
            size: size(400, 400),
        };
        let id_for = |x| display_for_bounds(&displays, window(x)).map(|display| display.id);

        assert_eq!(id_for(100), Some(displays[0].id));
        assert_eq!(id_for(700), Some(displays[0].id));
        assert_eq!(id_for(900), Some(displays[1].id));
        assert_eq!(id_for(5000), None);
        // Ties go to the earlier, primary display.
        assert_eq!(id_for(800), Some(displays[0].id));
    }

    #[test]
    fn display_ids_carry_the_x_screen() {
        assert_eq!(X11Display::screen_index(display_id(0, 0x47)), 0);
        assert_eq!(X11Display::screen_index(display_id(2, 0x47)), 2);
    }

    #[test]
    fn mode_refresh_intervals_follow_the_pixel_clock() {
        let mode = |dot_clock, mode_flags| randr::ModeInfo {
            id: 1,
            width: 1920,
            height: 1080,
            dot_clock,
            hsync_start: 0,
            hsync_end: 0,
            htotal: 2200,
            hskew: 0,
            vsync_start: 0,
            vsync_end: 0,
            vtotal: 1125,
            name_len: 0,
            mode_flags,
        };
        assert_eq!(
            mode_refresh_interval(&mode(148_500_000, randr::ModeFlag::default())),
            Some(Duration::from_nanos(16_666_666))
        );
        assert_eq!(
            mode_refresh_interval(&mode(148_500_000, randr::ModeFlag::INTERLACE)),
            Some(Duration::from_nanos(8_333_333))
        );
        assert_eq!(
            mode_refresh_interval(&mode(0, randr::ModeFlag::default())),
            None
        );
    }
}
