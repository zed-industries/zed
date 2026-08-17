#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "Screen",
        extends = "EventTarget",
        extends = "::js_sys::Object",
        js_name = "ScreenDetailed",
        typescript_type = "ScreenDetailed"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ScreenDetailed` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScreenDetailed)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScreenDetailed`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type ScreenDetailed;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, getter, js_class = "ScreenDetailed", js_name = "availLeft")]
    #[doc = "Getter for the `availLeft` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScreenDetailed/availLeft)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScreenDetailed`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn avail_left(this: &ScreenDetailed) -> i32;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, getter, js_class = "ScreenDetailed", js_name = "availTop")]
    #[doc = "Getter for the `availTop` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScreenDetailed/availTop)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScreenDetailed`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn avail_top(this: &ScreenDetailed) -> i32;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, getter, js_class = "ScreenDetailed", js_name = "left")]
    #[doc = "Getter for the `left` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScreenDetailed/left)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScreenDetailed`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn left(this: &ScreenDetailed) -> i32;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, getter, js_class = "ScreenDetailed", js_name = "top")]
    #[doc = "Getter for the `top` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScreenDetailed/top)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScreenDetailed`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn top(this: &ScreenDetailed) -> i32;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, getter, js_class = "ScreenDetailed", js_name = "isPrimary")]
    #[doc = "Getter for the `isPrimary` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScreenDetailed/isPrimary)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScreenDetailed`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn is_primary(this: &ScreenDetailed) -> bool;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, getter, js_class = "ScreenDetailed", js_name = "isInternal")]
    #[doc = "Getter for the `isInternal` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScreenDetailed/isInternal)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScreenDetailed`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn is_internal(this: &ScreenDetailed) -> bool;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "ScreenDetailed",
        js_name = "devicePixelRatio"
    )]
    #[doc = "Getter for the `devicePixelRatio` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScreenDetailed/devicePixelRatio)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScreenDetailed`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn device_pixel_ratio(this: &ScreenDetailed) -> f32;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, getter, js_class = "ScreenDetailed", js_name = "label")]
    #[doc = "Getter for the `label` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScreenDetailed/label)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScreenDetailed`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn label(this: &ScreenDetailed) -> ::alloc::string::String;
}
