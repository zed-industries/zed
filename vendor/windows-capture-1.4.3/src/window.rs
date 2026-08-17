use std::{ptr, string::FromUtf16Error};

use windows::{
    Graphics::Capture::GraphicsCaptureItem,
    Win32::{
        Foundation::{HWND, LPARAM, RECT, TRUE},
        Graphics::Gdi::{MONITOR_DEFAULTTONULL, MonitorFromWindow},
        System::{
            Threading::GetCurrentProcessId, WinRT::Graphics::Capture::IGraphicsCaptureItemInterop,
        },
        UI::WindowsAndMessaging::{
            EnumChildWindows, FindWindowW, GWL_EXSTYLE, GWL_STYLE, GetClientRect, GetDesktopWindow,
            GetForegroundWindow, GetWindowLongPtrW, GetWindowRect, GetWindowTextLengthW,
            GetWindowTextW, GetWindowThreadProcessId, IsWindowVisible, WS_CHILD, WS_EX_TOOLWINDOW,
        },
    },
    core::{BOOL, HSTRING},
};

use crate::monitor::Monitor;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("No active window found")]
    NoActiveWindow,
    #[error("Failed to find window with name: {0}")]
    NotFound(String),
    #[error("Failed to convert windows string from UTF-16: {0}")]
    FailedToConvertWindowsString(#[from] FromUtf16Error),
    #[error("Windows API error: {0}")]
    WindowsError(#[from] windows::core::Error),
}

/// Represents a window in the Windows operating system.
///
/// # Example
/// ```no_run
/// use windows_capture::window::Window;
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let window = Window::foreground()?;
///     println!("Foreground window title: {}", window.title()?);
///
///     Ok(())
/// }
/// ```
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct Window {
    window: HWND,
}

unsafe impl Send for Window {}

impl Window {
    /// Returns the foreground window.
    ///
    /// # Errors
    ///
    /// Returns an `Error::NoActiveWindow` if there is no active window.
    #[inline]
    pub fn foreground() -> Result<Self, Error> {
        let window = unsafe { GetForegroundWindow() };

        if window.is_invalid() {
            return Err(Error::NoActiveWindow);
        }

        Ok(Self { window })
    }

    /// Creates a `Window` instance from a window name.
    ///
    /// # Arguments
    ///
    /// * `title` - The name of the window.
    ///
    /// # Errors
    ///
    /// Returns an `Error::NotFound` if the window is not found.
    #[inline]
    pub fn from_name(title: &str) -> Result<Self, Error> {
        let hstring_title = HSTRING::from(title);
        let window = unsafe { FindWindowW(None, &hstring_title)? };

        if window.is_invalid() {
            return Err(Error::NotFound(String::from(title)));
        }

        Ok(Self { window })
    }

    /// Creates a `Window` instance from a window name substring.
    ///
    /// # Arguments
    ///
    /// * `title` - The substring to search for in window names.
    ///
    /// # Errors
    ///
    /// Returns an `Error::NotFound` if no window with a matching name substring is found.
    #[inline]
    pub fn from_contains_name(title: &str) -> Result<Self, Error> {
        let windows = Self::enumerate()?;

        let mut target_window = None;
        for window in windows {
            if window.title()?.contains(title) {
                target_window = Some(window);
                break;
            }
        }

        target_window.map_or_else(|| Err(Error::NotFound(String::from(title))), Ok)
    }

    /// Returns the title of the window.
    ///
    /// # Errors
    ///
    /// Returns an `Error` if there is an error retrieving the window title.
    #[inline]
    pub fn title(&self) -> Result<String, Error> {
        let len = unsafe { GetWindowTextLengthW(self.window) };

        let mut name = vec![0u16; usize::try_from(len).unwrap() + 1];
        if len >= 1 {
            let copied = unsafe { GetWindowTextW(self.window, &mut name) };
            if copied == 0 {
                return Ok(String::new());
            }
        }

        let name = String::from_utf16(
            &name
                .as_slice()
                .iter()
                .take_while(|ch| **ch != 0x0000)
                .copied()
                .collect::<Vec<_>>(),
        )?;

        Ok(name)
    }

    /// Returns the monitor that has the largest area of intersection with the window.
    ///
    /// Returns `None` if the window doesn't intersect with any monitor.
    #[must_use]
    #[inline]
    pub fn monitor(&self) -> Option<Monitor> {
        let window = self.window;

        let monitor = unsafe { MonitorFromWindow(window, MONITOR_DEFAULTTONULL) };

        if monitor.is_invalid() {
            None
        } else {
            Some(Monitor::from_raw_hmonitor(monitor.0))
        }
    }

    /// Returns the rectangle of the window in screen coordinates.
    ///
    /// # Errors
    ///
    /// Returns an `Error::WindowsError` if there is an error retrieving the window rectangle.
    #[inline]
    pub fn rect(&self) -> Result<RECT, Error> {
        let mut rect = RECT::default();
        let result = unsafe { GetWindowRect(self.window, &mut rect) };
        if result.is_ok() {
            Ok(rect)
        } else {
            Err(Error::WindowsError(windows::core::Error::from_win32()))
        }
    }

    /// Checks if the window is a valid window.
    ///
    /// # Returns
    ///
    /// Returns `true` if the window is valid, `false` otherwise.
    #[must_use]
    #[inline]
    pub fn is_valid(&self) -> bool {
        if !unsafe { IsWindowVisible(self.window).as_bool() } {
            return false;
        }

        let mut id = 0;
        unsafe { GetWindowThreadProcessId(self.window, Some(&mut id)) };
        if id == unsafe { GetCurrentProcessId() } {
            return false;
        }

        let mut rect = RECT::default();
        let result = unsafe { GetClientRect(self.window, &mut rect) };
        if result.is_ok() {
            let styles = unsafe { GetWindowLongPtrW(self.window, GWL_STYLE) };
            let ex_styles = unsafe { GetWindowLongPtrW(self.window, GWL_EXSTYLE) };

            if (ex_styles & isize::try_from(WS_EX_TOOLWINDOW.0).unwrap()) != 0 {
                return false;
            }
            if (styles & isize::try_from(WS_CHILD.0).unwrap()) != 0 {
                return false;
            }
        } else {
            return false;
        }

        true
    }

    /// Returns a list of all windows.
    ///
    /// # Errors
    ///
    /// Returns an `Error` if there is an error enumerating the windows.
    #[inline]
    pub fn enumerate() -> Result<Vec<Self>, Error> {
        let mut windows: Vec<Self> = Vec::new();

        unsafe {
            EnumChildWindows(
                Some(GetDesktopWindow()),
                Some(Self::enum_windows_callback),
                LPARAM(ptr::addr_of_mut!(windows) as isize),
            )
            .ok()?;
        };

        Ok(windows)
    }

    /// Creates a `Window` instance from a raw HWND.
    ///
    /// # Arguments
    ///
    /// * `hwnd` - The raw HWND.
    #[must_use]
    #[inline]
    pub const fn from_raw_hwnd(hwnd: *mut std::ffi::c_void) -> Self {
        Self { window: HWND(hwnd) }
    }

    /// Returns the raw HWND of the window.
    #[must_use]
    #[inline]
    pub const fn as_raw_hwnd(&self) -> *mut std::ffi::c_void {
        self.window.0
    }

    // Callback used for enumerating all windows.
    #[inline]
    unsafe extern "system" fn enum_windows_callback(window: HWND, vec: LPARAM) -> BOOL {
        let windows = &mut *(vec.0 as *mut Vec<Self>);

        if Self::from_raw_hwnd(window.0).is_valid() {
            windows.push(Self { window });
        }

        TRUE
    }
}

// Implements TryFrom For Window To Convert It To GraphicsCaptureItem
impl TryFrom<Window> for GraphicsCaptureItem {
    type Error = Error;

    #[inline]
    fn try_from(value: Window) -> Result<Self, Self::Error> {
        let window = HWND(value.as_raw_hwnd());

        let interop = windows::core::factory::<Self, IGraphicsCaptureItemInterop>()?;
        Ok(unsafe { interop.CreateForWindow(window)? })
    }
}
