#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (is_type_of = | _ | false , extends = "SvgPathSeg" , extends = "::js_sys::Object" , js_name = "SVGPathSegLinetoVerticalAbs" , typescript_type = "SVGPathSegLinetoVerticalAbs")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgPathSegLinetoVerticalAbs` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegLinetoVerticalAbs)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPathSegLinetoVerticalAbs`*"]
    pub type SvgPathSegLinetoVerticalAbs;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGPathSegLinetoVerticalAbs",
        js_name = "y"
    )]
    #[doc = "Getter for the `y` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegLinetoVerticalAbs/y)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPathSegLinetoVerticalAbs`*"]
    pub fn y(this: &SvgPathSegLinetoVerticalAbs) -> f32;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "SVGPathSegLinetoVerticalAbs",
        js_name = "y"
    )]
    #[doc = "Setter for the `y` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegLinetoVerticalAbs/y)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPathSegLinetoVerticalAbs`*"]
    pub fn set_y(this: &SvgPathSegLinetoVerticalAbs, value: f32);
}
