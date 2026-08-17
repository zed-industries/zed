use core::ffi::{c_int, c_ulong, c_void};
use core::num::NonZeroU32;
use core::ptr::NonNull;

/// Raw display handle for Xlib.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct XlibDisplayHandle {
    /// A pointer to an Xlib `Display`.
    ///
    /// It is strongly recommended to set this value, however it may be set to
    /// `None` to request the default display when using EGL.
    pub display: Option<NonNull<c_void>>,

    /// An X11 screen to use with this display handle.
    ///
    /// Note, that X11 could have multiple screens, however
    /// graphics APIs could work only with one screen at the time,
    /// given that multiple screens usually reside on different GPUs.
    pub screen: c_int,
}

impl XlibDisplayHandle {
    /// Create a new handle to a display.
    ///
    ///
    /// # Example
    ///
    /// ```
    /// # use core::ffi::c_void;
    /// # use core::ptr::NonNull;
    /// # use raw_window_handle::XlibDisplayHandle;
    /// #
    /// let display: NonNull<c_void>;
    /// let screen;
    /// # display = NonNull::from(&()).cast();
    /// # screen = 0;
    /// let handle = XlibDisplayHandle::new(Some(display), screen);
    /// ```
    pub fn new(display: Option<NonNull<c_void>>, screen: c_int) -> Self {
        Self { display, screen }
    }
}

/// Raw window handle for Xlib.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct XlibWindowHandle {
    /// An Xlib `Window`.
    pub window: c_ulong,
    /// An Xlib visual ID, or 0 if unknown.
    pub visual_id: c_ulong,
}

impl XlibWindowHandle {
    /// Create a new handle to a window.
    ///
    ///
    /// # Example
    ///
    /// ```
    /// # use core::ffi::c_ulong;
    /// # use raw_window_handle::XlibWindowHandle;
    /// #
    /// let window: c_ulong;
    /// # window = 0;
    /// let mut handle = XlibWindowHandle::new(window);
    /// // Optionally set the visual ID.
    /// handle.visual_id = 0;
    /// ```
    pub fn new(window: c_ulong) -> Self {
        Self {
            window,
            visual_id: 0,
        }
    }
}

/// Raw display handle for Xcb.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct XcbDisplayHandle {
    /// A pointer to an X server `xcb_connection_t`.
    ///
    /// It is strongly recommended to set this value, however it may be set to
    /// `None` to request the default display when using EGL.
    pub connection: Option<NonNull<c_void>>,

    /// An X11 screen to use with this display handle.
    ///
    /// Note, that X11 could have multiple screens, however
    /// graphics APIs could work only with one screen at the time,
    /// given that multiple screens usually reside on different GPUs.
    pub screen: c_int,
}

impl XcbDisplayHandle {
    /// Create a new handle to a connection and screen.
    ///
    ///
    /// # Example
    ///
    /// ```
    /// # use core::ffi::c_void;
    /// # use core::ptr::NonNull;
    /// # use raw_window_handle::XcbDisplayHandle;
    /// #
    /// let connection: NonNull<c_void>;
    /// let screen;
    /// # connection = NonNull::from(&()).cast();
    /// # screen = 0;
    /// let handle = XcbDisplayHandle::new(Some(connection), screen);
    /// ```
    pub fn new(connection: Option<NonNull<c_void>>, screen: c_int) -> Self {
        Self { connection, screen }
    }
}

/// Raw window handle for Xcb.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct XcbWindowHandle {
    /// An X11 `xcb_window_t`.
    pub window: NonZeroU32, // Based on xproto.h
    /// An X11 `xcb_visualid_t`.
    pub visual_id: Option<NonZeroU32>,
}

impl XcbWindowHandle {
    /// Create a new handle to a window.
    ///
    ///
    /// # Example
    ///
    /// ```
    /// # use core::num::NonZeroU32;
    /// # use raw_window_handle::XcbWindowHandle;
    /// #
    /// let window: NonZeroU32;
    /// # window = NonZeroU32::new(1).unwrap();
    /// let mut handle = XcbWindowHandle::new(window);
    /// // Optionally set the visual ID.
    /// handle.visual_id = None;
    /// ```
    pub fn new(window: NonZeroU32) -> Self {
        Self {
            window,
            visual_id: None,
        }
    }
}

/// Raw display handle for Wayland.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WaylandDisplayHandle {
    /// A pointer to a `wl_display`.
    pub display: NonNull<c_void>,
}

impl WaylandDisplayHandle {
    /// Create a new display handle.
    ///
    ///
    /// # Example
    ///
    /// ```
    /// # use core::ffi::c_void;
    /// # use core::ptr::NonNull;
    /// # use raw_window_handle::WaylandDisplayHandle;
    /// #
    /// let display: NonNull<c_void>;
    /// # display = NonNull::from(&()).cast();
    /// let handle = WaylandDisplayHandle::new(display);
    /// ```
    pub fn new(display: NonNull<c_void>) -> Self {
        Self { display }
    }
}

/// Raw window handle for Wayland.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WaylandWindowHandle {
    /// A pointer to a `wl_surface`.
    pub surface: NonNull<c_void>,
}

impl WaylandWindowHandle {
    /// Create a new handle to a surface.
    ///
    ///
    /// # Example
    ///
    /// ```
    /// # use core::ffi::c_void;
    /// # use core::ptr::NonNull;
    /// # use raw_window_handle::WaylandWindowHandle;
    /// #
    /// let surface: NonNull<c_void>;
    /// # surface = NonNull::from(&()).cast();
    /// let handle = WaylandWindowHandle::new(surface);
    /// ```
    pub fn new(surface: NonNull<c_void>) -> Self {
        Self { surface }
    }
}

/// Raw display handle for the Linux Kernel Mode Set/Direct Rendering Manager.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DrmDisplayHandle {
    /// The drm file descriptor.
    // TODO: Use `std::os::fd::RawFd`?
    pub fd: i32,
}

impl DrmDisplayHandle {
    /// Create a new handle to a file descriptor.
    ///
    ///
    /// # Example
    ///
    /// ```
    /// # use raw_window_handle::DrmDisplayHandle;
    /// #
    /// let fd: i32;
    /// # fd = 0;
    /// let handle = DrmDisplayHandle::new(fd);
    /// ```
    pub fn new(fd: i32) -> Self {
        Self { fd }
    }
}

/// Raw window handle for the Linux Kernel Mode Set/Direct Rendering Manager.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DrmWindowHandle {
    /// The primary drm plane handle.
    pub plane: u32,
}

impl DrmWindowHandle {
    /// Create a new handle to a plane.
    ///
    ///
    /// # Example
    ///
    /// ```
    /// # use raw_window_handle::DrmWindowHandle;
    /// #
    /// let plane: u32;
    /// # plane = 0;
    /// let handle = DrmWindowHandle::new(plane);
    /// ```
    pub fn new(plane: u32) -> Self {
        Self { plane }
    }
}

/// Raw display handle for the Linux Generic Buffer Manager.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GbmDisplayHandle {
    /// The gbm device.
    pub gbm_device: NonNull<c_void>,
}

impl GbmDisplayHandle {
    /// Create a new handle to a device.
    ///
    ///
    /// # Example
    ///
    /// ```
    /// # use core::ffi::c_void;
    /// # use core::ptr::NonNull;
    /// # use raw_window_handle::GbmDisplayHandle;
    /// #
    /// let ptr: NonNull<c_void>;
    /// # ptr = NonNull::from(&()).cast();
    /// let handle = GbmDisplayHandle::new(ptr);
    /// ```
    pub fn new(gbm_device: NonNull<c_void>) -> Self {
        Self { gbm_device }
    }
}

/// Raw window handle for the Linux Generic Buffer Manager.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GbmWindowHandle {
    /// The gbm surface.
    pub gbm_surface: NonNull<c_void>,
}

impl GbmWindowHandle {
    /// Create a new handle to a surface.
    ///
    ///
    /// # Example
    ///
    /// ```
    /// # use core::ffi::c_void;
    /// # use core::ptr::NonNull;
    /// # use raw_window_handle::GbmWindowHandle;
    /// #
    /// let ptr: NonNull<c_void>;
    /// # ptr = NonNull::from(&()).cast();
    /// let handle = GbmWindowHandle::new(ptr);
    /// ```
    pub fn new(gbm_surface: NonNull<c_void>) -> Self {
        Self { gbm_surface }
    }
}
