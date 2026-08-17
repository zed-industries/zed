#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "GeolocationPosition",
        typescript_type = "GeolocationPosition"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GeolocationPosition` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GeolocationPosition)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GeolocationPosition`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type GeolocationPosition;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GeolocationCoordinates")]
    #[wasm_bindgen(method, getter, js_class = "GeolocationPosition", js_name = "coords")]
    #[doc = "Getter for the `coords` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GeolocationPosition/coords)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GeolocationCoordinates`, `GeolocationPosition`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn coords(this: &GeolocationPosition) -> GeolocationCoordinates;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "GeolocationPosition",
        js_name = "timestamp"
    )]
    #[doc = "Getter for the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GeolocationPosition/timestamp)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GeolocationPosition`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn timestamp(this: &GeolocationPosition) -> f64;
}
