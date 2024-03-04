use windows::Win32::{
    Graphics::Gdi::{GetMonitorInfoW, HMONITOR, MONITORINFO, MONITORINFOEXW},
    UI::HiDpi::{GetDpiForMonitor, MDT_EFFECTIVE_DPI},
};

use crate::Size;

pub struct MonitorHandle(HMONITOR);

impl MonitorHandle {
    pub fn new(hmonitor: HMONITOR) -> Self {
        MonitorHandle(hmonitor)
    }

    pub fn size(&self) -> Size<i32> {
        let info = get_monitor_info(self.0).unwrap();
        let size = info.monitorInfo.rcMonitor;

        Size {
            width: size.right - size.left,
            height: size.top - size.bottom,
        }
    }

    pub fn scale_factor(&self) -> f32 {
        (self.get_dpi() as f32) / 96.0
    }

    fn get_dpi(&self) -> u32 {
        unsafe {
            let mut dpi_x = 0;
            let mut dpi_y = 0;
            if GetDpiForMonitor(self.0, MDT_EFFECTIVE_DPI, &mut dpi_x, &mut dpi_y).is_ok() {
                dpi_x
            } else {
                96
            }
        }
    }
}

fn get_monitor_info(hmonitor: HMONITOR) -> Result<MONITORINFOEXW, std::io::Error> {
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
        Err(std::io::Error::last_os_error())
    }
}
