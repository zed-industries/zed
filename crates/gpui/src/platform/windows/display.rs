use itertools::Itertools;
use smallvec::SmallVec;
use std::rc::Rc;
use uuid::Uuid;
use windows::{
    core::*,
    Win32::{Foundation::*, Graphics::Gdi::*},
};

use crate::{Bounds, DevicePixels, DisplayId, PlatformDisplay, Point, Size};

#[derive(Debug, Clone, Copy)]
pub(crate) struct WindowsDisplay {
    pub handle: HMONITOR,
    pub display_id: DisplayId,
    bounds: Bounds<DevicePixels>,
    uuid: Uuid,
}

impl WindowsDisplay {
    pub(crate) fn new(display_id: DisplayId) -> Option<Self> {
        let Some(screen) = available_monitors().into_iter().nth(display_id.0 as _) else {
            return None;
        };
        let Ok(info) = get_monitor_info(screen).inspect_err(|e| log::error!("{}", e)) else {
            return None;
        };
        let size = info.monitorInfo.rcMonitor;
        let uuid = generate_uuid(&info.szDevice);

        Some(WindowsDisplay {
            handle: screen,
            display_id,
            bounds: Bounds {
                origin: Point {
                    x: DevicePixels(size.left),
                    y: DevicePixels(size.top),
                },
                size: Size {
                    width: DevicePixels(size.right - size.left),
                    height: DevicePixels(size.bottom - size.top),
                },
            },
            uuid,
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

        WindowsDisplay {
            handle: monitor,
            display_id: DisplayId(display_id as _),
            bounds: Bounds {
                origin: Point {
                    x: DevicePixels(size.left as i32),
                    y: DevicePixels(size.top as i32),
                },
                size: Size {
                    width: DevicePixels((size.right - size.left) as i32),
                    height: DevicePixels((size.bottom - size.top) as i32),
                },
            },
            uuid,
        }
    }

    fn new_with_handle_and_id(handle: HMONITOR, display_id: DisplayId) -> Self {
        let info = get_monitor_info(handle).expect("unable to get monitor info");
        let size = info.monitorInfo.rcMonitor;
        let uuid = generate_uuid(&info.szDevice);

        WindowsDisplay {
            handle,
            display_id,
            bounds: Bounds {
                origin: Point {
                    x: DevicePixels(size.left as i32),
                    y: DevicePixels(size.top as i32),
                },
                size: Size {
                    width: DevicePixels((size.right - size.left) as i32),
                    height: DevicePixels((size.bottom - size.top) as i32),
                },
            },
            uuid,
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
}

impl PlatformDisplay for WindowsDisplay {
    fn id(&self) -> DisplayId {
        self.display_id
    }

    fn uuid(&self) -> anyhow::Result<Uuid> {
        Ok(self.uuid)
    }

    fn bounds(&self) -> Bounds<DevicePixels> {
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
        );
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
