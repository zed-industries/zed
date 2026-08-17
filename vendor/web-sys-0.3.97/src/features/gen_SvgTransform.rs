#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "SVGTransform",
        typescript_type = "SVGTransform"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgTransform` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransform)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgTransform`*"]
    pub type SvgTransform;
    #[wasm_bindgen(method, getter, js_class = "SVGTransform", js_name = "type")]
    #[doc = "Getter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransform/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgTransform`*"]
    pub fn type_(this: &SvgTransform) -> u16;
    #[cfg(feature = "SvgMatrix")]
    #[wasm_bindgen(method, getter, js_class = "SVGTransform", js_name = "matrix")]
    #[doc = "Getter for the `matrix` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransform/matrix)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgMatrix`, `SvgTransform`*"]
    pub fn matrix(this: &SvgTransform) -> SvgMatrix;
    #[wasm_bindgen(method, getter, js_class = "SVGTransform", js_name = "angle")]
    #[doc = "Getter for the `angle` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransform/angle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgTransform`*"]
    pub fn angle(this: &SvgTransform) -> f32;
    #[cfg(feature = "SvgMatrix")]
    #[wasm_bindgen(catch, method, js_class = "SVGTransform", js_name = "setMatrix")]
    #[doc = "The `setMatrix()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransform/setMatrix)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgMatrix`, `SvgTransform`*"]
    pub fn set_matrix(this: &SvgTransform, matrix: &SvgMatrix) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "SVGTransform", js_name = "setRotate")]
    #[doc = "The `setRotate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransform/setRotate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgTransform`*"]
    pub fn set_rotate(this: &SvgTransform, angle: f32, cx: f32, cy: f32) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "SVGTransform", js_name = "setScale")]
    #[doc = "The `setScale()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransform/setScale)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgTransform`*"]
    pub fn set_scale(this: &SvgTransform, sx: f32, sy: f32) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "SVGTransform", js_name = "setSkewX")]
    #[doc = "The `setSkewX()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransform/setSkewX)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgTransform`*"]
    pub fn set_skew_x(this: &SvgTransform, angle: f32) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "SVGTransform", js_name = "setSkewY")]
    #[doc = "The `setSkewY()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransform/setSkewY)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgTransform`*"]
    pub fn set_skew_y(this: &SvgTransform, angle: f32) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "SVGTransform", js_name = "setTranslate")]
    #[doc = "The `setTranslate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransform/setTranslate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgTransform`*"]
    pub fn set_translate(this: &SvgTransform, tx: f32, ty: f32) -> Result<(), JsValue>;
}
impl SvgTransform {
    #[doc = "The `SVGTransform.SVG_TRANSFORM_UNKNOWN` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgTransform`*"]
    pub const SVG_TRANSFORM_UNKNOWN: u16 = 0i64 as u16;
    #[doc = "The `SVGTransform.SVG_TRANSFORM_MATRIX` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgTransform`*"]
    pub const SVG_TRANSFORM_MATRIX: u16 = 1u64 as u16;
    #[doc = "The `SVGTransform.SVG_TRANSFORM_TRANSLATE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgTransform`*"]
    pub const SVG_TRANSFORM_TRANSLATE: u16 = 2u64 as u16;
    #[doc = "The `SVGTransform.SVG_TRANSFORM_SCALE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgTransform`*"]
    pub const SVG_TRANSFORM_SCALE: u16 = 3u64 as u16;
    #[doc = "The `SVGTransform.SVG_TRANSFORM_ROTATE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgTransform`*"]
    pub const SVG_TRANSFORM_ROTATE: u16 = 4u64 as u16;
    #[doc = "The `SVGTransform.SVG_TRANSFORM_SKEWX` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgTransform`*"]
    pub const SVG_TRANSFORM_SKEWX: u16 = 5u64 as u16;
    #[doc = "The `SVGTransform.SVG_TRANSFORM_SKEWY` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgTransform`*"]
    pub const SVG_TRANSFORM_SKEWY: u16 = 6u64 as u16;
}
