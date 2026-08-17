#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "ScreenLuminance",
        typescript_type = "ScreenLuminance"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ScreenLuminance` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScreenLuminance)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScreenLuminance`*"]
    pub type ScreenLuminance;
    #[wasm_bindgen(method, getter, js_class = "ScreenLuminance", js_name = "min")]
    #[doc = "Getter for the `min` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScreenLuminance/min)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScreenLuminance`*"]
    pub fn min(this: &ScreenLuminance) -> f64;
    #[wasm_bindgen(method, getter, js_class = "ScreenLuminance", js_name = "max")]
    #[doc = "Getter for the `max` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScreenLuminance/max)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScreenLuminance`*"]
    pub fn max(this: &ScreenLuminance) -> f64;
    #[wasm_bindgen(method, getter, js_class = "ScreenLuminance", js_name = "maxAverage")]
    #[doc = "Getter for the `maxAverage` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScreenLuminance/maxAverage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScreenLuminance`*"]
    pub fn max_average(this: &ScreenLuminance) -> f64;
}
