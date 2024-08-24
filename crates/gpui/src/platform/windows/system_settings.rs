use std::ffi::{c_uint, c_void};

use ::util::ResultExt;
use windows::Win32::UI::{
    Shell::{SHAppBarMessage, ABM_GETTASKBARPOS, APPBARDATA},
    WindowsAndMessaging::{
        SystemParametersInfoW, SPI_GETWHEELSCROLLCHARS, SPI_GETWHEELSCROLLLINES,
        SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS,
    },
};

use crate::*;

use super::WindowsDisplay;

/// Windows settings pulled from SystemParametersInfo
/// https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-systemparametersinfow
#[derive(Default, Debug, Clone, Copy)]
pub(crate) struct WindowsSystemSettings {
    pub(crate) mouse_wheel_settings: MouseWheelSettings,
    pub(crate) taskbar_position: Option<TaskbarPosition>,
}

#[derive(Default, Debug, Clone, Copy)]
pub(crate) struct MouseWheelSettings {
    /// SEE: SPI_GETWHEELSCROLLCHARS
    pub(crate) wheel_scroll_chars: u32,
    /// SEE: SPI_GETWHEELSCROLLLINES
    pub(crate) wheel_scroll_lines: u32,
}

impl WindowsSystemSettings {
    pub(crate) fn new(display: WindowsDisplay) -> Self {
        let mut settings = Self::default();
        settings.update(display);
        settings
    }

    pub(crate) fn update(&mut self, display: WindowsDisplay) {
        self.mouse_wheel_settings.update();
        self.taskbar_position = TaskbarPosition::new(display).log_err().flatten();
    }
}

impl MouseWheelSettings {
    fn update(&mut self) {
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

#[derive(Debug, Clone, Copy, Default)]
pub(crate) enum TaskbarPosition {
    Left,
    Right,
    Top,
    #[default]
    Bottom,
}

impl TaskbarPosition {
    fn new(display: WindowsDisplay) -> anyhow::Result<Option<Self>> {
        let mut position = APPBARDATA::default();
        let ret = unsafe { SHAppBarMessage(ABM_GETTASKBARPOS, &mut position) };
        if ret == 0 {
            anyhow::bail!("{}", std::io::Error::last_os_error());
        }
        let taskbar_bounds: Bounds<DevicePixels> = Bounds::new(
            point(position.rc.left.into(), position.rc.top.into()),
            size(
                (position.rc.right - position.rc.left).into(),
                (position.rc.bottom - position.rc.top).into(),
            ),
        );
        let display_bounds = display.physical_bounds();
        let intersec = display_bounds.intersect(&taskbar_bounds);
        println!("taskbar: {:?}", taskbar_bounds);
        println!("display: {:?}", display_bounds);
        println!("Intersect: {:?}", intersec);
        if display_bounds.intersect(&taskbar_bounds) != taskbar_bounds {
            return Ok(None);
        }
        println!("--> {:#?}", position.rc);
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
        return Ok(None);
    }
}
