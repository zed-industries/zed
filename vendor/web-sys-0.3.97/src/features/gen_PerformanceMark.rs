#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "PerformanceEntry",
        extends = "::js_sys::Object",
        js_name = "PerformanceMark",
        typescript_type = "PerformanceMark"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PerformanceMark` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceMark)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceMark`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type PerformanceMark;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, getter, js_class = "PerformanceMark", js_name = "detail")]
    #[doc = "Getter for the `detail` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceMark/detail)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceMark`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn detail(this: &PerformanceMark) -> ::wasm_bindgen::JsValue;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(catch, constructor, js_class = "PerformanceMark")]
    #[doc = "The `new PerformanceMark(..)` constructor, creating a new instance of `PerformanceMark`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceMark/PerformanceMark)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceMark`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(mark_name: &str) -> Result<PerformanceMark, JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "PerformanceMarkOptions")]
    #[wasm_bindgen(catch, constructor, js_class = "PerformanceMark")]
    #[doc = "The `new PerformanceMark(..)` constructor, creating a new instance of `PerformanceMark`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceMark/PerformanceMark)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceMark`, `PerformanceMarkOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new_with_mark_options(
        mark_name: &str,
        mark_options: &PerformanceMarkOptions,
    ) -> Result<PerformanceMark, JsValue>;
}
