#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "GPUExternalTexture",
        typescript_type = "GPUExternalTexture"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GpuExternalTexture` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUExternalTexture)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuExternalTexture`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type GpuExternalTexture;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, getter, js_class = "GPUExternalTexture", js_name = "label")]
    #[doc = "Getter for the `label` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUExternalTexture/label)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuExternalTexture`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn label(this: &GpuExternalTexture) -> ::alloc::string::String;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, setter, js_class = "GPUExternalTexture", js_name = "label")]
    #[doc = "Setter for the `label` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUExternalTexture/label)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuExternalTexture`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_label(this: &GpuExternalTexture, value: &str);
}
