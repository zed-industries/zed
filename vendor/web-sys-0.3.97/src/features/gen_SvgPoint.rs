#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "SVGPoint",
        typescript_type = "SVGPoint"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgPoint` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPoint)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPoint`*"]
    pub type SvgPoint;
    #[wasm_bindgen(method, getter, js_class = "SVGPoint", js_name = "x")]
    #[doc = "Getter for the `x` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPoint/x)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPoint`*"]
    pub fn x(this: &SvgPoint) -> f32;
    #[wasm_bindgen(method, setter, js_class = "SVGPoint", js_name = "x")]
    #[doc = "Setter for the `x` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPoint/x)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPoint`*"]
    pub fn set_x(this: &SvgPoint, value: f32);
    #[wasm_bindgen(method, getter, js_class = "SVGPoint", js_name = "y")]
    #[doc = "Getter for the `y` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPoint/y)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPoint`*"]
    pub fn y(this: &SvgPoint) -> f32;
    #[wasm_bindgen(method, setter, js_class = "SVGPoint", js_name = "y")]
    #[doc = "Setter for the `y` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPoint/y)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPoint`*"]
    pub fn set_y(this: &SvgPoint, value: f32);
    #[cfg(feature = "SvgMatrix")]
    #[wasm_bindgen(method, js_class = "SVGPoint", js_name = "matrixTransform")]
    #[doc = "The `matrixTransform()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPoint/matrixTransform)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgMatrix`, `SvgPoint`*"]
    pub fn matrix_transform(this: &SvgPoint, matrix: &SvgMatrix) -> SvgPoint;
}
