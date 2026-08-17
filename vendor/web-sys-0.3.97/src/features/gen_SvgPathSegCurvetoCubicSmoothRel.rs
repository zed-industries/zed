#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (is_type_of = | _ | false , extends = "SvgPathSeg" , extends = "::js_sys::Object" , js_name = "SVGPathSegCurvetoCubicSmoothRel" , typescript_type = "SVGPathSegCurvetoCubicSmoothRel")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgPathSegCurvetoCubicSmoothRel` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoCubicSmoothRel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPathSegCurvetoCubicSmoothRel`*"]
    pub type SvgPathSegCurvetoCubicSmoothRel;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGPathSegCurvetoCubicSmoothRel",
        js_name = "x"
    )]
    #[doc = "Getter for the `x` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoCubicSmoothRel/x)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPathSegCurvetoCubicSmoothRel`*"]
    pub fn x(this: &SvgPathSegCurvetoCubicSmoothRel) -> f32;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "SVGPathSegCurvetoCubicSmoothRel",
        js_name = "x"
    )]
    #[doc = "Setter for the `x` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoCubicSmoothRel/x)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPathSegCurvetoCubicSmoothRel`*"]
    pub fn set_x(this: &SvgPathSegCurvetoCubicSmoothRel, value: f32);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGPathSegCurvetoCubicSmoothRel",
        js_name = "y"
    )]
    #[doc = "Getter for the `y` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoCubicSmoothRel/y)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPathSegCurvetoCubicSmoothRel`*"]
    pub fn y(this: &SvgPathSegCurvetoCubicSmoothRel) -> f32;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "SVGPathSegCurvetoCubicSmoothRel",
        js_name = "y"
    )]
    #[doc = "Setter for the `y` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoCubicSmoothRel/y)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPathSegCurvetoCubicSmoothRel`*"]
    pub fn set_y(this: &SvgPathSegCurvetoCubicSmoothRel, value: f32);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGPathSegCurvetoCubicSmoothRel",
        js_name = "x2"
    )]
    #[doc = "Getter for the `x2` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoCubicSmoothRel/x2)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPathSegCurvetoCubicSmoothRel`*"]
    pub fn x2(this: &SvgPathSegCurvetoCubicSmoothRel) -> f32;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "SVGPathSegCurvetoCubicSmoothRel",
        js_name = "x2"
    )]
    #[doc = "Setter for the `x2` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoCubicSmoothRel/x2)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPathSegCurvetoCubicSmoothRel`*"]
    pub fn set_x2(this: &SvgPathSegCurvetoCubicSmoothRel, value: f32);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGPathSegCurvetoCubicSmoothRel",
        js_name = "y2"
    )]
    #[doc = "Getter for the `y2` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoCubicSmoothRel/y2)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPathSegCurvetoCubicSmoothRel`*"]
    pub fn y2(this: &SvgPathSegCurvetoCubicSmoothRel) -> f32;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "SVGPathSegCurvetoCubicSmoothRel",
        js_name = "y2"
    )]
    #[doc = "Setter for the `y2` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoCubicSmoothRel/y2)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPathSegCurvetoCubicSmoothRel`*"]
    pub fn set_y2(this: &SvgPathSegCurvetoCubicSmoothRel, value: f32);
}
