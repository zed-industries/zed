#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (is_type_of = | _ | false , extends = "SvgPathSeg" , extends = "::js_sys::Object" , js_name = "SVGPathSegMovetoRel" , typescript_type = "SVGPathSegMovetoRel")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgPathSegMovetoRel` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegMovetoRel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPathSegMovetoRel`*"]
    pub type SvgPathSegMovetoRel;
    #[wasm_bindgen(method, getter, js_class = "SVGPathSegMovetoRel", js_name = "x")]
    #[doc = "Getter for the `x` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegMovetoRel/x)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPathSegMovetoRel`*"]
    pub fn x(this: &SvgPathSegMovetoRel) -> f32;
    #[wasm_bindgen(method, setter, js_class = "SVGPathSegMovetoRel", js_name = "x")]
    #[doc = "Setter for the `x` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegMovetoRel/x)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPathSegMovetoRel`*"]
    pub fn set_x(this: &SvgPathSegMovetoRel, value: f32);
    #[wasm_bindgen(method, getter, js_class = "SVGPathSegMovetoRel", js_name = "y")]
    #[doc = "Getter for the `y` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegMovetoRel/y)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPathSegMovetoRel`*"]
    pub fn y(this: &SvgPathSegMovetoRel) -> f32;
    #[wasm_bindgen(method, setter, js_class = "SVGPathSegMovetoRel", js_name = "y")]
    #[doc = "Setter for the `y` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegMovetoRel/y)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPathSegMovetoRel`*"]
    pub fn set_y(this: &SvgPathSegMovetoRel, value: f32);
}
