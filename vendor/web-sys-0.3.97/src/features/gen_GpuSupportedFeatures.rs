#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "GPUSupportedFeatures",
        typescript_type = "GPUSupportedFeatures"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GpuSupportedFeatures` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUSupportedFeatures)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuSupportedFeatures`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type GpuSupportedFeatures;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, getter, js_class = "GPUSupportedFeatures", js_name = "size")]
    #[doc = "Getter for the `size` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUSupportedFeatures/size)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuSupportedFeatures`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn size(this: &GpuSupportedFeatures) -> u32;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(catch, method, js_class = "GPUSupportedFeatures", js_name = "forEach")]
    #[doc = "The `forEach()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUSupportedFeatures/forEach)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuSupportedFeatures`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn for_each(
        this: &GpuSupportedFeatures,
        callback: &::js_sys::Function<fn(::js_sys::JsString) -> ::js_sys::Undefined>,
    ) -> Result<(), JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, js_class = "GPUSupportedFeatures")]
    #[doc = "The `has()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUSupportedFeatures/has)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuSupportedFeatures`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn has(this: &GpuSupportedFeatures, value: &str) -> bool;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, js_class = "GPUSupportedFeatures")]
    #[doc = "The `entries()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUSupportedFeatures/entries)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuSupportedFeatures`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn entries(
        this: &GpuSupportedFeatures,
    ) -> ::js_sys::Iterator<::js_sys::ArrayTuple<(::js_sys::JsString, ::js_sys::JsString)>>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, js_class = "GPUSupportedFeatures")]
    #[doc = "The `keys()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUSupportedFeatures/keys)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuSupportedFeatures`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn keys(this: &GpuSupportedFeatures) -> ::js_sys::Iterator<::js_sys::JsString>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, js_class = "GPUSupportedFeatures")]
    #[doc = "The `values()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUSupportedFeatures/values)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuSupportedFeatures`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn values(this: &GpuSupportedFeatures) -> ::js_sys::Iterator<::js_sys::JsString>;
}
