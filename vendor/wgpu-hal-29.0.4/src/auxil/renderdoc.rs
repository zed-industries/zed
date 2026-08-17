//! RenderDoc integration - <https://renderdoc.org/>
#![cfg_attr(not(any(feature = "gles", feature = "vulkan")), allow(dead_code))]

use alloc::format;
use alloc::string::String;
use core::{ffi, ptr};

/// The dynamically loaded RenderDoc API function table
#[repr(C)]
#[derive(Debug)]
pub struct RenderDocApi {
    api: renderdoc_sys::RENDERDOC_API_1_4_1,
    lib: libloading::Library,
}

unsafe impl Send for RenderDocApi {}
unsafe impl Sync for RenderDocApi {}

/// RenderDoc API type
#[derive(Debug)]
pub enum RenderDoc {
    /// RenderDoc functionality is available
    Available {
        /// RenderDoc API with function pointers
        api: RenderDocApi,
    },
    /// RenderDoc functionality is _not_ available
    NotAvailable {
        /// A description why renderdoc functionality is not available
        reason: String,
    },
}

// TODO: replace with libloading API once supported
#[cfg(unix)]
const RTLD_NOLOAD: i32 = 0x4;

impl RenderDoc {
    pub unsafe fn new() -> Self {
        type GetApiFn = unsafe extern "C" fn(version: u32, out: *mut *mut ffi::c_void) -> i32;

        #[cfg(windows)]
        let renderdoc_filename = "renderdoc.dll";
        #[cfg(all(unix, not(target_os = "android")))]
        let renderdoc_filename = "librenderdoc.so";
        #[cfg(target_os = "android")]
        let renderdoc_filename = "libVkLayer_GLES_RenderDoc.so";

        #[cfg(unix)]
        let renderdoc_result: Result<libloading::Library, libloading::Error> = unsafe {
            libloading::os::unix::Library::open(
                Some(renderdoc_filename),
                libloading::os::unix::RTLD_NOW | RTLD_NOLOAD,
            )
        }
        .map(|lib| lib.into());

        #[cfg(windows)]
        let renderdoc_result: Result<libloading::Library, libloading::Error> =
            libloading::os::windows::Library::open_already_loaded(renderdoc_filename)
                .map(|lib| lib.into());

        let renderdoc_lib = match renderdoc_result {
            Ok(lib) => lib,
            Err(e) => {
                return RenderDoc::NotAvailable {
                    reason: format!(
                        "Unable to load renderdoc library '{renderdoc_filename}': {e:?}"
                    ),
                }
            }
        };

        let get_api: libloading::Symbol<GetApiFn> =
            match unsafe { renderdoc_lib.get(c"RENDERDOC_GetAPI".to_bytes()) } {
                Ok(api) => api,
                Err(e) => {
                    return RenderDoc::NotAvailable {
                        reason: format!(
                            "Unable to get RENDERDOC_GetAPI from renderdoc library '{renderdoc_filename}': {e:?}"
                        ),
                    }
                }
            };
        let mut obj = ptr::null_mut();
        match unsafe { get_api(10401, &mut obj) } {
            1 => RenderDoc::Available {
                api: RenderDocApi {
                    api: unsafe { *obj.cast::<renderdoc_sys::RENDERDOC_API_1_4_1>() },
                    lib: renderdoc_lib,
                },
            },
            return_value => RenderDoc::NotAvailable {
                reason: format!(
                    "Unable to get API from renderdoc library '{renderdoc_filename}': {return_value}"
                ),
            },
        }
    }
}

impl Default for RenderDoc {
    fn default() -> Self {
        if !cfg!(debug_assertions) {
            return RenderDoc::NotAvailable {
                reason: "RenderDoc support is only enabled with 'debug_assertions'".into(),
            };
        }
        unsafe { Self::new() }
    }
}
/// An implementation specific handle
pub type Handle = *mut ffi::c_void;

impl RenderDoc {
    /// Start a RenderDoc frame capture
    pub unsafe fn start_frame_capture(&self, device_handle: Handle, window_handle: Handle) -> bool {
        match *self {
            Self::Available { api: ref entry } => {
                unsafe { entry.api.StartFrameCapture.unwrap()(device_handle, window_handle) };
                true
            }
            Self::NotAvailable { ref reason } => {
                log::warn!("Could not start RenderDoc frame capture: {reason}");
                false
            }
        }
    }

    /// End a RenderDoc frame capture
    pub unsafe fn end_frame_capture(&self, device_handle: Handle, window_handle: Handle) {
        match *self {
            Self::Available { api: ref entry } => {
                unsafe { entry.api.EndFrameCapture.unwrap()(device_handle, window_handle) };
            }
            Self::NotAvailable { ref reason } => {
                log::warn!("Could not end RenderDoc frame capture: {reason}")
            }
        };
    }
}
