use std::cell::Cell;
use util::ResultExt;

use windows::Win32::{Foundation::*, System::Threading::*, UI::WindowsAndMessaging::*};

use crate::{GlobalPixels, Pixels, Point, Size};

type PhysicalPoint = Point<GlobalPixels>;
pub(crate) type LogicalPoint = Point<Pixels>;
type PhysicalSize = Size<GlobalPixels>;
type LogicalSize = Size<Pixels>;

#[derive(Debug, Default)]
pub(crate) struct DisplaySize {
    origin: Cell<PhysicalPoint>,
    physical_size: Cell<PhysicalSize>,
    logical_size: Cell<LogicalSize>,
    scale_factor: Cell<f32>,
}

impl DisplaySize {
    /// `x` and `y` should be screen coordinate based
    /// `width` and `height` should be physical pixels
    pub fn new(x: i32, y: i32, width: u32, height: u32, scale_factor: f32) -> Self {
        DisplaySize {
            origin: Cell::new(PhysicalPoint::new_physical(x, y)),
            physical_size: Cell::new(PhysicalSize::new_physical(width, height)),
            logical_size: Cell::new(LogicalSize::new_logical(width, height, scale_factor)),
            scale_factor: Cell::new(scale_factor),
        }
    }

    /// `width` and `height` should be physical pixels
    pub fn update_size(&self, width: u32, height: u32, scale_factor: f32) {
        self.physical_size
            .set(PhysicalSize::new_physical(width, height));
        self.logical_size
            .set(LogicalSize::new_logical(width, height, scale_factor));
    }

    /// used with `BladeRenderer::update_drawable_size`
    pub fn drawable_size(&self) -> Size<f64> {
        self.physical_size.get().to_drawable_size()
    }

    pub fn get_origin(&self) -> PhysicalPoint {
        self.origin.get()
    }

    /// `x` and `y` should be screen coordinate based
    pub fn set_origin(&self, x: i32, y: i32) {
        self.origin.set(PhysicalPoint::new_physical(x, y));
    }

    pub fn get_physical_size(&self) -> PhysicalSize {
        self.physical_size.get()
    }

    pub fn get_logical_size(&self) -> LogicalSize {
        self.logical_size.get()
    }

    pub fn get_scale_factor(&self) -> f32 {
        self.scale_factor.get()
    }

    pub fn set_scale_factor(&self, scale_factor: f32) {
        self.scale_factor.set(scale_factor);
    }
}

impl PhysicalPoint {
    /// get `PhysicalPoint` with given `x` and `y`
    ///
    /// `x` and 'y` here should be physical pixels.
    pub fn new_physical(x: i32, y: i32) -> Self {
        Point {
            x: GlobalPixels(x as f32),
            y: GlobalPixels(y as f32),
        }
    }
}

impl LogicalPoint {
    // get `LogicalPoint` with given `x` and `y`
    ///
    /// `x` and 'y` here should be physical pixels, `scale_factor` should be generated from
    /// the same monitor
    pub fn new_logical(x: i32, y: i32, scale_factor: f32) -> Self {
        Point {
            x: Pixels(x as f32 / scale_factor),
            y: Pixels(y as f32 / scale_factor),
        }
    }
}

impl PhysicalSize {
    /// get `PhysicalSize` with given `width` and `height`
    ///
    /// `width` and 'height` here should be physical pixels.
    pub fn new_physical(width: u32, height: u32) -> Self {
        Size {
            width: GlobalPixels(width as f32),
            height: GlobalPixels(height as f32),
        }
    }

    /// get the physical size of the app's drawable area,
    /// used with `BladeRenderer::update_drawable_size`
    pub fn to_drawable_size(&self) -> Size<f64> {
        Size {
            width: self.width.0 as f64,
            height: self.height.0 as f64,
        }
    }
}

impl LogicalSize {
    /// get `LogicalSize` with given `width` and `height`
    ///
    /// `width` and 'height` here should be physical pixels, `scale_factor` should be generated from
    /// the same monitor
    pub fn new_logical(width: u32, height: u32, scale_factor: f32) -> Self {
        Size {
            width: Pixels(width as f32 / scale_factor),
            height: Pixels(height as f32 / scale_factor),
        }
    }
}

pub(crate) trait HiLoWord {
    fn hiword(&self) -> u16;
    fn loword(&self) -> u16;
    fn signed_hiword(&self) -> i16;
    fn signed_loword(&self) -> i16;
}

impl HiLoWord for WPARAM {
    fn hiword(&self) -> u16 {
        ((self.0 >> 16) & 0xFFFF) as u16
    }

    fn loword(&self) -> u16 {
        (self.0 & 0xFFFF) as u16
    }

    fn signed_hiword(&self) -> i16 {
        ((self.0 >> 16) & 0xFFFF) as i16
    }

    fn signed_loword(&self) -> i16 {
        (self.0 & 0xFFFF) as i16
    }
}

impl HiLoWord for LPARAM {
    fn hiword(&self) -> u16 {
        ((self.0 >> 16) & 0xFFFF) as u16
    }

    fn loword(&self) -> u16 {
        (self.0 & 0xFFFF) as u16
    }

    fn signed_hiword(&self) -> i16 {
        ((self.0 >> 16) & 0xFFFF) as i16
    }

    fn signed_loword(&self) -> i16 {
        (self.0 & 0xFFFF) as i16
    }
}

pub(crate) unsafe fn get_window_long(hwnd: HWND, nindex: WINDOW_LONG_PTR_INDEX) -> isize {
    #[cfg(target_pointer_width = "64")]
    unsafe {
        GetWindowLongPtrW(hwnd, nindex)
    }
    #[cfg(target_pointer_width = "32")]
    unsafe {
        GetWindowLongW(hwnd, nindex) as isize
    }
}

pub(crate) unsafe fn set_window_long(
    hwnd: HWND,
    nindex: WINDOW_LONG_PTR_INDEX,
    dwnewlong: isize,
) -> isize {
    #[cfg(target_pointer_width = "64")]
    unsafe {
        SetWindowLongPtrW(hwnd, nindex, dwnewlong)
    }
    #[cfg(target_pointer_width = "32")]
    unsafe {
        SetWindowLongW(hwnd, nindex, dwnewlong as i32) as isize
    }
}

pub(crate) struct OwnedHandle(HANDLE);

impl OwnedHandle {
    pub(crate) fn new(handle: HANDLE) -> Self {
        Self(handle)
    }

    #[inline(always)]
    pub(crate) fn to_raw(&self) -> HANDLE {
        self.0
    }
}

impl Drop for OwnedHandle {
    fn drop(&mut self) {
        if !self.0.is_invalid() {
            unsafe { CloseHandle(self.0) }.log_err();
        }
    }
}

pub(crate) fn create_event() -> windows::core::Result<OwnedHandle> {
    Ok(OwnedHandle::new(unsafe {
        CreateEventW(None, false, false, None)?
    }))
}

pub(crate) fn windows_credentials_target_name(url: &str) -> String {
    format!("zed:url={}", url)
}
