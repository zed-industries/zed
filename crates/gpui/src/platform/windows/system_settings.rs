use std::ffi::{c_uint, c_void};

use util::ResultExt;
use windows::Win32::UI::WindowsAndMessaging::{
    SystemParametersInfoW, SPI_GETWHEELSCROLLCHARS, SPI_GETWHEELSCROLLLINES,
    SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS,
};

/// Windows settings pulled from SystemParametersInfo
/// https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-systemparametersinfow
#[derive(Default, Debug, Clone, Copy)]
pub(crate) struct WindowsSystemSettings {
    pub(crate) mouse_wheel_settings: MouseWheelSettings,
}

#[derive(Default, Debug, Clone, Copy)]
pub(crate) struct MouseWheelSettings {
    /// SEE: SPI_GETWHEELSCROLLCHARS
    pub(crate) wheel_scroll_chars: u32,
    /// SEE: SPI_GETWHEELSCROLLLINES
    pub(crate) wheel_scroll_lines: u32,
}

impl WindowsSystemSettings {
    pub(crate) fn new() -> Self {
        let mut settings = Self::default();
        settings.init();
        settings
    }

    fn init(&mut self) {
        self.mouse_wheel_settings.update();
    }
}

impl MouseWheelSettings {
    pub(crate) fn update(&mut self) {
        self.update_wheel_scroll_chars();
        self.update_wheel_scroll_lines();
    }

    fn update_wheel_scroll_chars(&mut self) {
        let mut value = c_uint::default();
        let result = unsafe {
            SystemParametersInfoW(
                SPI_GETWHEELSCROLLCHARS,
                0,
                Some((&mut value) as *mut c_uint as *mut c_void),
                SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS::default(),
            )
        };

        if result.log_err() != None && self.wheel_scroll_chars != value {
            self.wheel_scroll_chars = value;
        }
    }

    fn update_wheel_scroll_lines(&mut self) {
        let mut value = c_uint::default();
        let result = unsafe {
            SystemParametersInfoW(
                SPI_GETWHEELSCROLLLINES,
                0,
                Some((&mut value) as *mut c_uint as *mut c_void),
                SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS::default(),
            )
        };

        if result.log_err() != None && self.wheel_scroll_lines != value {
            self.wheel_scroll_lines = value;
        }
    }
}
