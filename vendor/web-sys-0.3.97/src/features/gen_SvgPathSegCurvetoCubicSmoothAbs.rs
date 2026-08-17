#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (is_type_of = | _ | false , extends = "SvgPathSeg" , extends = "::js_sys::Object" , js_name = "SVGPathSegCurvetoCubicSmoothAbs" , typescript_type = "SVGPathSegCurvetoCubicSmoothAbs")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgPathSegCurvetoCubicSmoothAbs` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoCubicSmoothAbs)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPathSegCurvetoCubicSmoothAbs`*"]
    pub type SvgPathSegCurvetoCubicSmoothAbs;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGPathSegCurvetoCubicSmoothAbs",
        js_name = "x"
    )]
    #[doc = "Getter for the `x` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoCubicSmoothAbs/x)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPathSegCurvetoCubicSmoothAbs`*"]
    pub fn x(this: &SvgPathSegCurvetoCubicSmoothAbs) -> f32;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "SVGPathSegCurvetoCubicSmoothAbs",
        js_name = "x"
    )]
    #[doc = "Setter for the `x` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoCubicSmoothAbs/x)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPathSegCurvetoCubicSmoothAbs`*"]
    pub fn set_x(this: &SvgPathSegCurvetoCubicSmoothAbs, value: f32);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGPathSegCurvetoCubicSmoothAbs",
        js_name = "y"
    )]
    #[doc = "Getter for the `y` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoCubicSmoothAbs/y)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPathSegCurvetoCubicSmoothAbs`*"]
    pub fn y(this: &SvgPathSegCurvetoCubicSmoothAbs) -> f32;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "SVGPathSegCurvetoCubicSmoothAbs",
        js_name = "y"
    )]
    #[doc = "Setter for the `y` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoCubicSmoothAbs/y)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPathSegCurvetoCubicSmoothAbs`*"]
    pub fn set_y(this: &SvgPathSegCurvetoCubicSmoothAbs, value: f32);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGPathSegCurvetoCubicSmoothAbs",
        js_name = "x2"
    )]
    #[doc = "Getter for the `x2` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoCubicSmoothAbs/x2)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPathSegCurvetoCubicSmoothAbs`*"]
    pub fn x2(this: &SvgPathSegCurvetoCubicSmoothAbs) -> f32;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "SVGPathSegCurvetoCubicSmoothAbs",
        js_name = "x2"
    )]
    #[doc = "Setter for the `x2` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoCubicSmoothAbs/x2)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPathSegCurvetoCubicSmoothAbs`*"]
    pub fn set_x2(this: &SvgPathSegCurvetoCubicSmoothAbs, value: f32);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGPathSegCurvetoCubicSmoothAbs",
        js_name = "y2"
    )]
    #[doc = "Getter for the `y2` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoCubicSmoothAbs/y2)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPathSegCurvetoCubicSmoothAbs`*"]
    pub fn y2(this: &SvgPathSegCurvetoCubicSmoothAbs) -> f32;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "SVGPathSegCurvetoCubicSmoothAbs",
        js_name = "y2"
    )]
    #[doc = "Setter for the `y2` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegCurvetoCubicSmoothAbs/y2)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPathSegCurvetoCubicSmoothAbs`*"]
    pub fn set_y2(this: &SvgPathSegCurvetoCubicSmoothAbs, value: f32);
}
