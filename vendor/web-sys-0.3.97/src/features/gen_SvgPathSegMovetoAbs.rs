#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (is_type_of = | _ | false , extends = "SvgPathSeg" , extends = "::js_sys::Object" , js_name = "SVGPathSegMovetoAbs" , typescript_type = "SVGPathSegMovetoAbs")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgPathSegMovetoAbs` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegMovetoAbs)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPathSegMovetoAbs`*"]
    pub type SvgPathSegMovetoAbs;
    #[wasm_bindgen(method, getter, js_class = "SVGPathSegMovetoAbs", js_name = "x")]
    #[doc = "Getter for the `x` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegMovetoAbs/x)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPathSegMovetoAbs`*"]
    pub fn x(this: &SvgPathSegMovetoAbs) -> f32;
    #[wasm_bindgen(method, setter, js_class = "SVGPathSegMovetoAbs", js_name = "x")]
    #[doc = "Setter for the `x` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegMovetoAbs/x)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPathSegMovetoAbs`*"]
    pub fn set_x(this: &SvgPathSegMovetoAbs, value: f32);
    #[wasm_bindgen(method, getter, js_class = "SVGPathSegMovetoAbs", js_name = "y")]
    #[doc = "Getter for the `y` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegMovetoAbs/y)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPathSegMovetoAbs`*"]
    pub fn y(this: &SvgPathSegMovetoAbs) -> f32;
    #[wasm_bindgen(method, setter, js_class = "SVGPathSegMovetoAbs", js_name = "y")]
    #[doc = "Setter for the `y` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegMovetoAbs/y)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPathSegMovetoAbs`*"]
    pub fn set_y(this: &SvgPathSegMovetoAbs, value: f32);
}
