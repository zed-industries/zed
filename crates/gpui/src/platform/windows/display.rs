use itertools::Itertools;
use smallvec::SmallVec;
use std::rc::Rc;
use util::ResultExt;
use uuid::Uuid;
use windows::{
    core::*,
    Win32::{
        Foundation::*,
        Graphics::Gdi::*,
        UI::HiDpi::{GetDpiForMonitor, MDT_EFFECTIVE_DPI},
    },
};

use crate::{px, Bounds, DisplayId, Pixels, PlatformDisplay, Point, Size};

#[derive(Debug, Clone, Copy)]
pub(crate) struct WindowsDisplay {
    pub handle: HMONITOR,
    pub display_id: DisplayId,
    bounds: Bounds<Pixels>,
    uuid: Uuid,
    scale_factor: f32,
}

impl WindowsDisplay {
    pub(crate) fn new(display_id: DisplayId) -> Option<Self> {
        let screen = available_monitors().into_iter().nth(display_id.0 as _)?;
        let info = get_monitor_info(screen).log_err()?;
        let size = info.monitorInfo.rcMonitor;
        let uuid = generate_uuid(&info.szDevice);
        let scale_facotr = get_scale_factor_for_monitor(screen).log_err()?;

        Some(WindowsDisplay {
            handle: screen,
            display_id,
            bounds: Bounds {
                origin: Point {
                    x: px(size.left as f32 / scale_facotr),
                    y: px(size.top as f32 / scale_facotr),
                },
                size: Size {
                    width: px((size.right - size.left) as f32 / scale_facotr),
                    height: px((size.bottom - size.top) as f32 / scale_facotr),
                },
            },
            uuid,
            scale_factor,
        })
    }

    pub fn new_with_handle(monitor: HMONITOR) -> Self {
        let info = get_monitor_info(monitor).expect("unable to get monitor info");
        let size = info.monitorInfo.rcMonitor;
        let uuid = generate_uuid(&info.szDevice);
        let display_id = available_monitors()
            .iter()
            .position(|handle| handle.0 == monitor.0)
            .unwrap();
        let scale_factor =
            get_scale_factor_for_monitor(monitor).expect("unable to get scale factor for monitor");

        WindowsDisplay {
            handle: monitor,
            display_id: DisplayId(display_id as _),
            bounds: Bounds {
                origin: Point {
                    x: px(size.left as f32),
                    y: px(size.top as f32),
                },
                size: Size {
                    width: px((size.right - size.left) as f32),
                    height: px((size.bottom - size.top) as f32),
                },
            },
            uuid,
            scale_factor,
        }
    }

    fn new_with_handle_and_id(handle: HMONITOR, display_id: DisplayId) -> Self {
        let info = get_monitor_info(handle).expect("unable to get monitor info");
        let size = info.monitorInfo.rcMonitor;
        let uuid = generate_uuid(&info.szDevice);
        let scale_factor =
            get_scale_factor_for_monitor(handle).expect("unable to get scale factor for monitor");

        WindowsDisplay {
            handle,
            display_id,
            bounds: Bounds {
                origin: Point {
                    x: px(size.left as f32),
                    y: px(size.top as f32),
                },
                size: Size {
                    width: px((size.right - size.left) as f32),
                    height: px((size.bottom - size.top) as f32),
                },
            },
            uuid,
            scale_factor,
        }
    }

    pub fn primary_monitor() -> Option<Self> {
        // https://devblogs.microsoft.com/oldnewthing/20070809-00/?p=25643
        const POINT_ZERO: POINT = POINT { x: 0, y: 0 };
        let monitor = unsafe { MonitorFromPoint(POINT_ZERO, MONITOR_DEFAULTTOPRIMARY) };
        if monitor.is_invalid() {
            log::error!(
                "can not find the primary monitor: {}",
                std::io::Error::last_os_error()
            );
            return None;
        }
        Some(WindowsDisplay::new_with_handle(monitor))
    }

    /// Check if the center point of given bounds is inside this monitor
    pub fn check_given_bounds(&self, bounds: Bounds<Pixels>) -> bool {
        let center = bounds.center();
        let center = POINT {
            x: center.x.0 as i32,
            y: center.y.0 as i32,
        };
        let monitor = unsafe { MonitorFromPoint(center, MONITOR_DEFAULTTONULL) };
        if monitor.is_invalid() {
            false
        } else {
            let display = WindowsDisplay::new_with_handle(monitor);
            display.uuid == self.uuid
        }
    }

    pub fn displays() -> Vec<Rc<dyn PlatformDisplay>> {
        available_monitors()
            .into_iter()
            .enumerate()
            .map(|(id, handle)| {
                Rc::new(WindowsDisplay::new_with_handle_and_id(
                    handle,
                    DisplayId(id as _),
                )) as Rc<dyn PlatformDisplay>
            })
            .collect()
    }

    pub(crate) fn frequency(&self) -> Option<u32> {
        get_monitor_info(self.handle).ok().and_then(|info| {
            let mut devmode = DEVMODEW::default();
            unsafe {
                EnumDisplaySettingsW(
                    PCWSTR(info.szDevice.as_ptr()),
                    ENUM_CURRENT_SETTINGS,
                    &mut devmode,
                )
            }
            .as_bool()
            .then(|| devmode.dmDisplayFrequency)
        })
    }

    /// Check if this monitor is still online
    pub fn is_connected(hmonitor: HMONITOR) -> bool {
        available_monitors().iter().contains(&hmonitor)
    }
}

impl PlatformDisplay for WindowsDisplay {
    fn id(&self) -> DisplayId {
        self.display_id
    }

    fn uuid(&self) -> anyhow::Result<Uuid> {
        Ok(self.uuid)
    }

    fn bounds(&self) -> Bounds<Pixels> {
        self.bounds
    }
}

fn available_monitors() -> SmallVec<[HMONITOR; 4]> {
    let mut monitors: SmallVec<[HMONITOR; 4]> = SmallVec::new();
    unsafe {
        EnumDisplayMonitors(
            HDC::default(),
            None,
            Some(monitor_enum_proc),
            LPARAM(&mut monitors as *mut _ as _),
        )
        .ok()
        .log_err();
    }
    monitors
}

unsafe extern "system" fn monitor_enum_proc(
    hmonitor: HMONITOR,
    _hdc: HDC,
    _place: *mut RECT,
    data: LPARAM,
) -> BOOL {
    let monitors = data.0 as *mut SmallVec<[HMONITOR; 4]>;
    unsafe { (*monitors).push(hmonitor) };
    BOOL(1)
}

fn get_monitor_info(hmonitor: HMONITOR) -> anyhow::Result<MONITORINFOEXW> {
    let mut monitor_info: MONITORINFOEXW = unsafe { std::mem::zeroed() };
    monitor_info.monitorInfo.cbSize = std::mem::size_of::<MONITORINFOEXW>() as u32;
    let status = unsafe {
        GetMonitorInfoW(
            hmonitor,
            &mut monitor_info as *mut MONITORINFOEXW as *mut MONITORINFO,
        )
    };
    if status.as_bool() {
        Ok(monitor_info)
    } else {
        Err(anyhow::anyhow!(std::io::Error::last_os_error()))
    }
}

fn generate_uuid(device_name: &[u16]) -> Uuid {
    let name = device_name
        .iter()
        .flat_map(|&a| a.to_be_bytes().to_vec())
        .collect_vec();
    Uuid::new_v5(&Uuid::NAMESPACE_DNS, &name)
}

fn get_scale_factor_for_monitor(monitor: HMONITOR) -> Result<f32> {
    let mut dpi_x = 0;
    let mut dpi_y = 0;
    unsafe { GetDpiForMonitor(monitor, MDT_EFFECTIVE_DPI, &mut dpi_x, &mut dpi_y) }?;
    assert_eq!(dpi_x, dpi_y);
    Ok(dpi_x as f32 / 96.0)
}
