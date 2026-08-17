#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "WGSLLanguageFeatures",
        typescript_type = "WGSLLanguageFeatures"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WgslLanguageFeatures` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WGSLLanguageFeatures)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WgslLanguageFeatures`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type WgslLanguageFeatures;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, getter, js_class = "WGSLLanguageFeatures", js_name = "size")]
    #[doc = "Getter for the `size` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WGSLLanguageFeatures/size)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WgslLanguageFeatures`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn size(this: &WgslLanguageFeatures) -> u32;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(catch, method, js_class = "WGSLLanguageFeatures", js_name = "forEach")]
    #[doc = "The `forEach()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WGSLLanguageFeatures/forEach)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WgslLanguageFeatures`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn for_each(
        this: &WgslLanguageFeatures,
        callback: &::js_sys::Function<fn(::js_sys::JsString) -> ::js_sys::Undefined>,
    ) -> Result<(), JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, js_class = "WGSLLanguageFeatures")]
    #[doc = "The `has()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WGSLLanguageFeatures/has)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WgslLanguageFeatures`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn has(this: &WgslLanguageFeatures, value: &str) -> bool;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, js_class = "WGSLLanguageFeatures")]
    #[doc = "The `entries()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WGSLLanguageFeatures/entries)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WgslLanguageFeatures`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn entries(
        this: &WgslLanguageFeatures,
    ) -> ::js_sys::Iterator<::js_sys::ArrayTuple<(::js_sys::JsString, ::js_sys::JsString)>>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, js_class = "WGSLLanguageFeatures")]
    #[doc = "The `keys()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WGSLLanguageFeatures/keys)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WgslLanguageFeatures`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn keys(this: &WgslLanguageFeatures) -> ::js_sys::Iterator<::js_sys::JsString>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, js_class = "WGSLLanguageFeatures")]
    #[doc = "The `values()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WGSLLanguageFeatures/values)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WgslLanguageFeatures`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn values(this: &WgslLanguageFeatures) -> ::js_sys::Iterator<::js_sys::JsString>;
}
