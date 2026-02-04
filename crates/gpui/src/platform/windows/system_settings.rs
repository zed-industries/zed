use std::{
    cell::Cell,
    ffi::{c_uint, c_void},
};

use ::util::ResultExt;
use windows::Win32::UI::{
    Shell::{ABM_GETSTATE, ABM_GETTASKBARPOS, ABS_AUTOHIDE, APPBARDATA, SHAppBarMessage},
    WindowsAndMessaging::{
        SPI_GETWHEELSCROLLCHARS, SPI_GETWHEELSCROLLLINES, SPI_SETWORKAREA,
        SYSTEM_PARAMETERS_INFO_ACTION, SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS, SystemParametersInfoW,
    },
};

use crate::*;

use super::WindowsDisplay;

/// Windows settings pulled from SystemParametersInfo
/// https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-systemparametersinfow
#[derive(Default, Debug, Clone)]
pub(crate) struct WindowsSystemSettings {
    pub(crate) mouse_wheel_settings: MouseWheelSettings,
    pub(crate) auto_hide_taskbar_position: Cell<Option<AutoHideTaskbarPosition>>,
}

#[derive(Default, Debug, Clone)]
pub(crate) struct MouseWheelSettings {
    /// SEE: SPI_GETWHEELSCROLLCHARS
    pub(crate) wheel_scroll_chars: Cell<u32>,
    /// SEE: SPI_GETWHEELSCROLLLINES
    pub(crate) wheel_scroll_lines: Cell<u32>,
}

impl WindowsSystemSettings {
    pub(crate) fn new(display: WindowsDisplay) -> Self {
        let mut settings = Self::default();
        settings.init(display);
        settings
    }

    fn init(&mut self, display: WindowsDisplay) {
        self.mouse_wheel_settings.update();
        self.auto_hide_taskbar_position
            .set(AutoHideTaskbarPosition::new(display).log_err().flatten());
    }

    pub(crate) fn update(&self, display: WindowsDisplay, wparam: usize) {
        match SYSTEM_PARAMETERS_INFO_ACTION(wparam as u32) {
            SPI_SETWORKAREA => self.update_taskbar_position(display),
            SPI_GETWHEELSCROLLLINES | SPI_GETWHEELSCROLLCHARS => self.update_mouse_wheel_settings(),
            _ => {}
        }
    }

    fn update_mouse_wheel_settings(&self) {
        self.mouse_wheel_settings.update();
    }

    fn update_taskbar_position(&self, display: WindowsDisplay) {
        self.auto_hide_taskbar_position
            .set(AutoHideTaskbarPosition::new(display).log_err().flatten());
    }
}

impl MouseWheelSettings {
    fn update(&self) {
        self.update_wheel_scroll_chars();
        self.update_wheel_scroll_lines();
    }

    fn update_wheel_scroll_chars(&self) {
        let mut value = c_uint::default();
        let result = unsafe {
            SystemParametersInfoW(
                SPI_GETWHEELSCROLLCHARS,
                0,
                Some((&mut value) as *mut c_uint as *mut c_void),
                SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS::default(),
            )
        };

        if result.log_err() != None && self.wheel_scroll_chars.get() != value {
            self.wheel_scroll_chars.set(value);
        }
    }

    fn update_wheel_scroll_lines(&self) {
        let mut value = c_uint::default();
        let result = unsafe {
            SystemParametersInfoW(
                SPI_GETWHEELSCROLLLINES,
                0,
                Some((&mut value) as *mut c_uint as *mut c_void),
                SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS::default(),
            )
        };

        if result.log_err() != None && self.wheel_scroll_lines.get() != value {
            self.wheel_scroll_lines.set(value);
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub(crate) enum AutoHideTaskbarPosition {
    Left,
    Right,
    Top,
    #[default]
    Bottom,
}

impl AutoHideTaskbarPosition {
    fn new(display: WindowsDisplay) -> anyhow::Result<Option<Self>> {
        if !check_auto_hide_taskbar_enable() {
            // If auto hide taskbar is not enable, we do nothing in this case.
            return Ok(None);
        }
        let mut info = APPBARDATA {
            cbSize: std::mem::size_of::<APPBARDATA>() as u32,
            ..Default::default()
        };
        let ret = unsafe { SHAppBarMessage(ABM_GETTASKBARPOS, &mut info) };
        if ret == 0 {
            anyhow::bail!(
                "Unable to retrieve taskbar position: {}",
                std::io::Error::last_os_error()
            );
        }
        let taskbar_bounds: Bounds<DevicePixels> = Bounds::new(
            point(info.rc.left.into(), info.rc.top.into()),
            size(
                (info.rc.right - info.rc.left).into(),
                (info.rc.bottom - info.rc.top).into(),
            ),
        );
        let display_bounds = display.physical_bounds();
        if display_bounds.intersect(&taskbar_bounds) != taskbar_bounds {
            // This case indicates that taskbar is not on the current monitor.
            return Ok(None);
        }
        if taskbar_bounds.bottom() == display_bounds.bottom()
            && taskbar_bounds.right() == display_bounds.right()
        {
            if taskbar_bounds.size.height < display_bounds.size.height
                && taskbar_bounds.size.width == display_bounds.size.width
            {
                return Ok(Some(Self::Bottom));
            }
            if taskbar_bounds.size.width < display_bounds.size.width
                && taskbar_bounds.size.height == display_bounds.size.height
            {
                return Ok(Some(Self::Right));
            }
            log::error!(
                "Unrecognized taskbar bounds {:?} give display bounds {:?}",
                taskbar_bounds,
                display_bounds
            );
            return Ok(None);
        }
        if taskbar_bounds.top() == display_bounds.top()
            && taskbar_bounds.left() == display_bounds.left()
        {
            if taskbar_bounds.size.height < display_bounds.size.height
                && taskbar_bounds.size.width == display_bounds.size.width
            {
                return Ok(Some(Self::Top));
            }
            if taskbar_bounds.size.width < display_bounds.size.width
                && taskbar_bounds.size.height == display_bounds.size.height
            {
                return Ok(Some(Self::Left));
            }
            log::error!(
                "Unrecognized taskbar bounds {:?} give display bounds {:?}",
                taskbar_bounds,
                display_bounds
            );
            return Ok(None);
        }
        log::error!(
            "Unrecognized taskbar bounds {:?} give display bounds {:?}",
            taskbar_bounds,
            display_bounds
        );
        Ok(None)
    }
}

/// Check if auto hide taskbar is enable or not.
fn check_auto_hide_taskbar_enable() -> bool {
    let mut info = APPBARDATA {
        cbSize: std::mem::size_of::<APPBARDATA>() as u32,
        ..Default::default()
    };
    let ret = unsafe { SHAppBarMessage(ABM_GETSTATE, &mut info) } as u32;
    ret == ABS_AUTOHIDE
}
