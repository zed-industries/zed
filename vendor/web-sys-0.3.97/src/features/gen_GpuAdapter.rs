#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "GPUAdapter",
        typescript_type = "GPUAdapter"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GpuAdapter` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUAdapter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuAdapter`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type GpuAdapter;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuSupportedFeatures")]
    #[wasm_bindgen(method, getter, js_class = "GPUAdapter", js_name = "features")]
    #[doc = "Getter for the `features` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUAdapter/features)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuAdapter`, `GpuSupportedFeatures`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn features(this: &GpuAdapter) -> GpuSupportedFeatures;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuSupportedLimits")]
    #[wasm_bindgen(method, getter, js_class = "GPUAdapter", js_name = "limits")]
    #[doc = "Getter for the `limits` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUAdapter/limits)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuAdapter`, `GpuSupportedLimits`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn limits(this: &GpuAdapter) -> GpuSupportedLimits;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuAdapterInfo")]
    #[wasm_bindgen(method, getter, js_class = "GPUAdapter", js_name = "info")]
    #[doc = "Getter for the `info` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUAdapter/info)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuAdapter`, `GpuAdapterInfo`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn info(this: &GpuAdapter) -> GpuAdapterInfo;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuDevice")]
    #[wasm_bindgen(method, js_class = "GPUAdapter", js_name = "requestDevice")]
    #[doc = "The `requestDevice()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUAdapter/requestDevice)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuAdapter`, `GpuDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn request_device(this: &GpuAdapter) -> ::js_sys::Promise<GpuDevice>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(feature = "GpuDevice", feature = "GpuDeviceDescriptor",))]
    #[wasm_bindgen(method, js_class = "GPUAdapter", js_name = "requestDevice")]
    #[doc = "The `requestDevice()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUAdapter/requestDevice)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuAdapter`, `GpuDevice`, `GpuDeviceDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn request_device_with_descriptor(
        this: &GpuAdapter,
        descriptor: &GpuDeviceDescriptor,
    ) -> ::js_sys::Promise<GpuDevice>;
}
