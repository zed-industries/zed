#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "GeolocationPositionError",
        typescript_type = "GeolocationPositionError"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GeolocationPositionError` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GeolocationPositionError)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GeolocationPositionError`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type GeolocationPositionError;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "GeolocationPositionError",
        js_name = "code"
    )]
    #[doc = "Getter for the `code` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GeolocationPositionError/code)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GeolocationPositionError`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn code(this: &GeolocationPositionError) -> u16;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "GeolocationPositionError",
        js_name = "message"
    )]
    #[doc = "Getter for the `message` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GeolocationPositionError/message)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GeolocationPositionError`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn message(this: &GeolocationPositionError) -> ::alloc::string::String;
}
#[cfg(web_sys_unstable_apis)]
impl GeolocationPositionError {
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `GeolocationPositionError.PERMISSION_DENIED` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GeolocationPositionError`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub const PERMISSION_DENIED: u16 = 1u64 as u16;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `GeolocationPositionError.POSITION_UNAVAILABLE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GeolocationPositionError`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub const POSITION_UNAVAILABLE: u16 = 2u64 as u16;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `GeolocationPositionError.TIMEOUT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GeolocationPositionError`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub const TIMEOUT: u16 = 3u64 as u16;
}
