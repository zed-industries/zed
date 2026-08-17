#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "GeolocationCoordinates",
        typescript_type = "GeolocationCoordinates"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GeolocationCoordinates` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GeolocationCoordinates)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GeolocationCoordinates`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type GeolocationCoordinates;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "GeolocationCoordinates",
        js_name = "accuracy"
    )]
    #[doc = "Getter for the `accuracy` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GeolocationCoordinates/accuracy)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GeolocationCoordinates`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn accuracy(this: &GeolocationCoordinates) -> f64;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "GeolocationCoordinates",
        js_name = "latitude"
    )]
    #[doc = "Getter for the `latitude` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GeolocationCoordinates/latitude)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GeolocationCoordinates`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn latitude(this: &GeolocationCoordinates) -> f64;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "GeolocationCoordinates",
        js_name = "longitude"
    )]
    #[doc = "Getter for the `longitude` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GeolocationCoordinates/longitude)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GeolocationCoordinates`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn longitude(this: &GeolocationCoordinates) -> f64;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "GeolocationCoordinates",
        js_name = "altitude"
    )]
    #[doc = "Getter for the `altitude` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GeolocationCoordinates/altitude)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GeolocationCoordinates`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn altitude(this: &GeolocationCoordinates) -> Option<f64>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "GeolocationCoordinates",
        js_name = "altitudeAccuracy"
    )]
    #[doc = "Getter for the `altitudeAccuracy` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GeolocationCoordinates/altitudeAccuracy)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GeolocationCoordinates`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn altitude_accuracy(this: &GeolocationCoordinates) -> Option<f64>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "GeolocationCoordinates",
        js_name = "heading"
    )]
    #[doc = "Getter for the `heading` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GeolocationCoordinates/heading)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GeolocationCoordinates`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn heading(this: &GeolocationCoordinates) -> Option<f64>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, getter, js_class = "GeolocationCoordinates", js_name = "speed")]
    #[doc = "Getter for the `speed` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GeolocationCoordinates/speed)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GeolocationCoordinates`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn speed(this: &GeolocationCoordinates) -> Option<f64>;
}
