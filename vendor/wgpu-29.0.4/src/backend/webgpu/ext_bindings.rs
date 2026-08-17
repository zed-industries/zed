//! Extension bindings for WebGPU.
//!
//! These contain ideomatic Rust extension traits for various parts of the WebGPU
//! bindings that are missing, need to be improved, or otherwise need to be different
//! from the generated web_sys bindings.

use crate::backend::webgpu::webgpu_sys;
use wasm_bindgen::prelude::*;

/// Extension trait for [`web_sys::Navigator`] and [`web_sys::WorkerNavigator`] to
/// access the `gpu` property.
pub trait NavigatorGpu {
    /// Get the `gpu` property.
    ///
    /// This is intentionally a free function, to prevent overload conflicts with
    /// the method if it is enabled in web-sys itself.
    fn gpu(navigator: &Self) -> webgpu_sys::Gpu;
}

// --- Bindings for `Navigator` ---
#[wasm_bindgen]
extern "C" {
    /// Create a fake class which we tell wasm-bindgen has access to the `gpu` property.
    #[wasm_bindgen]
    type NavigatorWithGpu;

    #[wasm_bindgen(method, getter)]
    fn gpu(ext: &NavigatorWithGpu) -> webgpu_sys::Gpu;
}

impl NavigatorGpu for web_sys::Navigator {
    fn gpu(navigator: &Self) -> webgpu_sys::Gpu {
        // Must be an unchecked ref as this class does not exist at runtime.
        let extension: &NavigatorWithGpu = navigator.unchecked_ref();
        extension.gpu()
    }
}

impl NavigatorGpu for web_sys::WorkerNavigator {
    fn gpu(navigator: &Self) -> webgpu_sys::Gpu {
        // Must be an unchecked ref as this class does not exist at runtime.
        let extension: &NavigatorWithGpu = navigator.unchecked_ref();
        extension.gpu()
    }
}
