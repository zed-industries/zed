#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (is_type_of = | _ | false , extends = "SvgPathSeg" , extends = "::js_sys::Object" , js_name = "SVGPathSegLinetoHorizontalAbs" , typescript_type = "SVGPathSegLinetoHorizontalAbs")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgPathSegLinetoHorizontalAbs` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegLinetoHorizontalAbs)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPathSegLinetoHorizontalAbs`*"]
    pub type SvgPathSegLinetoHorizontalAbs;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGPathSegLinetoHorizontalAbs",
        js_name = "x"
    )]
    #[doc = "Getter for the `x` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegLinetoHorizontalAbs/x)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPathSegLinetoHorizontalAbs`*"]
    pub fn x(this: &SvgPathSegLinetoHorizontalAbs) -> f32;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "SVGPathSegLinetoHorizontalAbs",
        js_name = "x"
    )]
    #[doc = "Setter for the `x` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegLinetoHorizontalAbs/x)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPathSegLinetoHorizontalAbs`*"]
    pub fn set_x(this: &SvgPathSegLinetoHorizontalAbs, value: f32);
}
