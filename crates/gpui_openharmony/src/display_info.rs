//! Display properties via the OpenHarmony DisplayManager NDK.
//!
//! Queries the default display's width, height, density DPI, and rotation
//! through `OH_NativeDisplayManager` APIs.  Falls back to surface-provided
//! dimensions when the service is unavailable.

use log::warn;

// ── OHOS NDK FFI ──────────────────────────────────────────────────────────────

#[allow(non_camel_case_types)]
type NativeDisplayManager_ErrorCode = i32;

const DISPLAY_MANAGER_OK: NativeDisplayManager_ErrorCode = 0;

#[allow(non_camel_case_types)]
#[repr(C)]
struct NativeDisplayManager_DisplayInfo {
    id: u32,
    name: [std::ffi::c_char; 33], // OH_DISPLAY_NAME_LENGTH + 1
    is_alive: bool,
    width: i32,
    height: i32,
    physical_width: i32,
    physical_height: i32,
    refresh_rate: u32,
    available_width: u32,
    available_height: u32,
    density_dpi: f32,
    density_pixels: f32,
    scaled_density: f32,
    x_dpi: f32,
    y_dpi: f32,
    rotation: i32, // NativeDisplayManager_Rotation
    state: i32,    // NativeDisplayManager_DisplayState
    orientation: i32, // NativeDisplayManager_Orientation
}

// Provided by libnative_display_manager.so (build.rs emits the link directive).
#[link(name = "native_display_manager", kind = "dylib")]
#[allow(non_snake_case, non_camel_case_types)]
unsafe extern "C" {
    fn OH_NativeDisplayManager_GetDefaultDisplayWidth(display_width: *mut i32) -> i32;
    fn OH_NativeDisplayManager_GetDefaultDisplayHeight(display_height: *mut i32) -> i32;
    fn OH_NativeDisplayManager_GetDefaultDisplayDensityInfo(density: *mut f32) -> i32;
    fn OH_NativeDisplayManager_GetDefaultDisplayInfo(
        display_info: *mut NativeDisplayManager_DisplayInfo,
    ) -> i32;
}

// ── Public types ──────────────────────────────────────────────────────────────

/// Display properties queried from the OHOS DisplayManager service.
#[derive(Clone, Debug)]
pub struct DisplayProperties {
    /// Display width in physical pixels.
    pub width: u32,
    /// Display height in physical pixels.
    pub height: u32,
    /// Display density in DPI (typically 160, 240, 320, 480).
    pub density_dpi: f32,
    /// Current rotation (0, 90, 180, 270).
    pub rotation_degrees: u32,
}

impl DisplayProperties {
    /// Scale factor relative to a 160 DPI baseline.
    pub fn scale_factor(&self) -> f32 {
        self.density_dpi / 160.0
    }
}

// ── Public API ────────────────────────────────────────────────────────────────

/// Queries the default display from the OHOS DisplayManager service.
///
/// Returns `None` if the service is unavailable or the query fails
/// (e.g., running on a non-OHOS host or before the display service starts).
pub fn query_default_display() -> Option<DisplayProperties> {
    unsafe {
        let mut info = std::mem::MaybeUninit::<NativeDisplayManager_DisplayInfo>::uninit();
        let rc = OH_NativeDisplayManager_GetDefaultDisplayInfo(info.as_mut_ptr());
        if rc != DISPLAY_MANAGER_OK {
            warn!("OH_NativeDisplayManager_GetDefaultDisplayInfo failed: {rc}");
            return query_fallback();
        }
        let info = info.assume_init();
        Some(DisplayProperties {
            width: info.width.max(0) as u32,
            height: info.height.max(0) as u32,
            density_dpi: info.density_dpi,
            rotation_degrees: match info.rotation {
                1 => 90,
                2 => 180,
                3 => 270,
                _ => 0,
            },
        })
    }
}

fn query_fallback() -> Option<DisplayProperties> {
    unsafe {
        let mut width: i32 = 0;
        let mut height: i32 = 0;
        let mut density: f32 = 0.0;

        let rw = OH_NativeDisplayManager_GetDefaultDisplayWidth(&mut width as *mut i32);
        let rh = OH_NativeDisplayManager_GetDefaultDisplayHeight(&mut height as *mut i32);
        let rd = OH_NativeDisplayManager_GetDefaultDisplayDensityInfo(&mut density as *mut f32);

        if rw != DISPLAY_MANAGER_OK || rh != DISPLAY_MANAGER_OK || width <= 0 || height <= 0 {
            return None;
        }

        if rd != DISPLAY_MANAGER_OK || density <= 0.0 {
            density = 320.0;
        }

        Some(DisplayProperties {
            width: width as u32,
            height: height as u32,
            density_dpi: density,
            rotation_degrees: 0,
        })
    }
}
