#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "EventTarget",
        extends = "::js_sys::Object",
        js_name = "ScreenDetails",
        typescript_type = "ScreenDetails"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ScreenDetails` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScreenDetails)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScreenDetails`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type ScreenDetails;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "ScreenDetailed")]
    #[wasm_bindgen(method, getter, js_class = "ScreenDetails", js_name = "screens")]
    #[doc = "Getter for the `screens` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScreenDetails/screens)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScreenDetailed`, `ScreenDetails`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn screens(this: &ScreenDetails) -> ::js_sys::Array<ScreenDetailed>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "ScreenDetailed")]
    #[wasm_bindgen(method, getter, js_class = "ScreenDetails", js_name = "currentScreen")]
    #[doc = "Getter for the `currentScreen` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScreenDetails/currentScreen)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScreenDetailed`, `ScreenDetails`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn current_screen(this: &ScreenDetails) -> ScreenDetailed;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "ScreenDetails",
        js_name = "onscreenschange"
    )]
    #[doc = "Getter for the `onscreenschange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScreenDetails/onscreenschange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScreenDetails`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn onscreenschange(this: &ScreenDetails) -> Option<::js_sys::Function>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        setter,
        js_class = "ScreenDetails",
        js_name = "onscreenschange"
    )]
    #[doc = "Setter for the `onscreenschange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScreenDetails/onscreenschange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScreenDetails`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_onscreenschange(this: &ScreenDetails, value: Option<&::js_sys::Function>);
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "ScreenDetails",
        js_name = "oncurrentscreenchange"
    )]
    #[doc = "Getter for the `oncurrentscreenchange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScreenDetails/oncurrentscreenchange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScreenDetails`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn oncurrentscreenchange(this: &ScreenDetails) -> Option<::js_sys::Function>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        setter,
        js_class = "ScreenDetails",
        js_name = "oncurrentscreenchange"
    )]
    #[doc = "Setter for the `oncurrentscreenchange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScreenDetails/oncurrentscreenchange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScreenDetails`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_oncurrentscreenchange(this: &ScreenDetails, value: Option<&::js_sys::Function>);
}
