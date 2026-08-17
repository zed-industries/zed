#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (is_type_of = | _ | false , extends = "::js_sys::Object" , js_name = "Position" , typescript_type = "Position")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Position` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Position)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Position`*"]
    pub type Position;
    #[cfg(feature = "Coordinates")]
    #[wasm_bindgen(method, getter, js_class = "Position", js_name = "coords")]
    #[doc = "Getter for the `coords` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Position/coords)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Coordinates`, `Position`*"]
    pub fn coords(this: &Position) -> Coordinates;
    #[wasm_bindgen(method, getter, js_class = "Position", js_name = "timestamp")]
    #[doc = "Getter for the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Position/timestamp)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Position`*"]
    pub fn timestamp(this: &Position) -> f64;
}
