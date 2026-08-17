#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "MediaDeviceInfo",
        extends = "::js_sys::Object",
        js_name = "InputDeviceInfo",
        typescript_type = "InputDeviceInfo"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `InputDeviceInfo` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/InputDeviceInfo)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `InputDeviceInfo`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type InputDeviceInfo;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "MediaTrackCapabilities")]
    #[wasm_bindgen(method, js_class = "InputDeviceInfo", js_name = "getCapabilities")]
    #[doc = "The `getCapabilities()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/InputDeviceInfo/getCapabilities)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `InputDeviceInfo`, `MediaTrackCapabilities`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn get_capabilities(this: &InputDeviceInfo) -> MediaTrackCapabilities;
}
