#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "PermissionStatus",
        extends = "EventTarget",
        extends = "::js_sys::Object",
        js_name = "XRPermissionStatus",
        typescript_type = "XRPermissionStatus"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `XrPermissionStatus` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRPermissionStatus)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrPermissionStatus`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type XrPermissionStatus;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, getter, js_class = "XRPermissionStatus", js_name = "granted")]
    #[doc = "Getter for the `granted` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRPermissionStatus/granted)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrPermissionStatus`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn granted(this: &XrPermissionStatus) -> ::js_sys::Array;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, setter, js_class = "XRPermissionStatus", js_name = "granted")]
    #[doc = "Setter for the `granted` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRPermissionStatus/granted)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrPermissionStatus`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_granted(this: &XrPermissionStatus, value: &[::wasm_bindgen::JsValue]);
}
