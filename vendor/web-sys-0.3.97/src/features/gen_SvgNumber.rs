#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "SVGNumber",
        typescript_type = "SVGNumber"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgNumber` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGNumber)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgNumber`*"]
    pub type SvgNumber;
    #[wasm_bindgen(method, getter, js_class = "SVGNumber", js_name = "value")]
    #[doc = "Getter for the `value` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGNumber/value)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgNumber`*"]
    pub fn value(this: &SvgNumber) -> f32;
    #[wasm_bindgen(method, setter, js_class = "SVGNumber", js_name = "value")]
    #[doc = "Setter for the `value` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGNumber/value)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgNumber`*"]
    pub fn set_value(this: &SvgNumber, value: f32);
}
