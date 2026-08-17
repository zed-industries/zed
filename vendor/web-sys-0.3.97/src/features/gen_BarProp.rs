#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "BarProp",
        typescript_type = "BarProp"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `BarProp` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BarProp)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BarProp`*"]
    pub type BarProp;
    #[wasm_bindgen(catch, method, getter, js_class = "BarProp", js_name = "visible")]
    #[doc = "Getter for the `visible` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BarProp/visible)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BarProp`*"]
    pub fn visible(this: &BarProp) -> Result<bool, JsValue>;
    #[wasm_bindgen(catch, method, setter, js_class = "BarProp", js_name = "visible")]
    #[doc = "Setter for the `visible` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BarProp/visible)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BarProp`*"]
    pub fn set_visible(this: &BarProp, value: bool) -> Result<(), JsValue>;
}
