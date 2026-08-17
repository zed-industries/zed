use super::{Display, Target};
use anyhow::{Context as _, Result};
use windows::Win32::UI::HiDpi::{GetDpiForMonitor, GetDpiForWindow, MDT_EFFECTIVE_DPI};
use windows::Win32::{
    Foundation::{HWND, RECT},
    Graphics::Gdi::HMONITOR,
};
use windows_capture::{
    monitor::{Error as MonitorError, Monitor},
    window::Window,
};

pub fn get_all_targets() -> Result<Vec<Target>> {
    let mut targets: Vec<Target> = Vec::new();

    // Add displays to targets
    let displays = Monitor::enumerate().context("Failed to enumerate monitors")?;
    for display in displays {
        let id = display.as_raw_hmonitor() as u32;
        let title = monitor_title(&display).context("Failed to get monitor name")?;

        let target = Target::Display(super::Display {
            id,
            title,
            raw_handle: HMONITOR(display.as_raw_hmonitor()),
            width: display.width()? as u16,
            height: display.height()? as u16,
        });
        targets.push(target);
    }

    // Add windows to targets
    let windows = Window::enumerate().context("Failed to enumerate windows")?;
    for window in windows {
        let id = window.as_raw_hwnd() as u32;
        let title = window
            .title()
            .context("Window title not found")?
            .to_string();

        let target = Target::Window(super::Window {
            id,
            title,
            raw_handle: HWND(window.as_raw_hwnd()),
        });
        targets.push(target);
    }

    Ok(targets)
}

pub fn get_main_display() -> Result<Display> {
    let display = Monitor::primary().context("Failed to get primary monitor")?;
    let id = display.as_raw_hmonitor() as u32;

    Ok(Display {
        id,
        title: monitor_title(&display).context("Failed to get monitor name")?,
        raw_handle: HMONITOR(display.as_raw_hmonitor()),
        width: display.width()? as u16,
        height: display.height()? as u16,
    })
}

fn monitor_title(monitor: &Monitor) -> Result<String, MonitorError> {
    monitor
        .name()
        .or_else(|_| monitor.device_string())
        .or_else(|_| monitor.device_name())
}

// Referred to: https://github.com/tauri-apps/tao/blob/ab792dbd6c5f0a708c818b20eaff1d9a7534c7c1/src/platform_impl/windows/dpi.rs#L50
pub fn get_scale_factor(target: &Target) -> f64 {
    const BASE_DPI: u32 = 96;

    let mut dpi_x = 0;
    let mut dpi_y = 0;

    let dpi = match target {
        Target::Window(window) => unsafe { GetDpiForWindow(window.raw_handle) },
        Target::Display(display) => unsafe {
            if GetDpiForMonitor(
                display.raw_handle,
                MDT_EFFECTIVE_DPI,
                &mut dpi_x,
                &mut dpi_y,
            )
            .is_ok()
            {
                dpi_x.into()
            } else {
                BASE_DPI
            }
        },
    };

    let scale_factor = dpi as f64 / BASE_DPI as f64;
    scale_factor as f64
}

pub fn get_target_dimensions(target: &Target) -> (u64, u64) {
    match target {
        Target::Window(window) => unsafe {
            let hwnd = window.raw_handle;

            // get width and height of the window
            let mut rect = RECT::default();
            let _ = windows::Win32::UI::WindowsAndMessaging::GetWindowRect(hwnd, &mut rect);
            let width = rect.right - rect.left;
            let height = rect.bottom - rect.top;

            (width as u64, height as u64)
        },
        Target::Display(display) => {
            let monitor = Monitor::from_raw_hmonitor(display.raw_handle.0);

            (
                monitor.width().unwrap() as u64,
                monitor.height().unwrap() as u64,
            )
        }
    }
}
