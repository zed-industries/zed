#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "GamepadTouch",
        typescript_type = "GamepadTouch"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GamepadTouch` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadTouch)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GamepadTouch`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type GamepadTouch;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, getter, js_class = "GamepadTouch", js_name = "touchId")]
    #[doc = "Getter for the `touchId` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadTouch/touchId)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GamepadTouch`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn touch_id(this: &GamepadTouch) -> u32;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, getter, js_class = "GamepadTouch", js_name = "surfaceId")]
    #[doc = "Getter for the `surfaceId` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadTouch/surfaceId)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GamepadTouch`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn surface_id(this: &GamepadTouch) -> u8;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, getter, js_class = "GamepadTouch", js_name = "position")]
    #[doc = "Getter for the `position` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadTouch/position)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GamepadTouch`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn position(this: &GamepadTouch) -> ::alloc::vec::Vec<f32>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "GamepadTouch",
        js_name = "surfaceDimensions"
    )]
    #[doc = "Getter for the `surfaceDimensions` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadTouch/surfaceDimensions)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GamepadTouch`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn surface_dimensions(this: &GamepadTouch) -> Option<::alloc::vec::Vec<u32>>;
}
