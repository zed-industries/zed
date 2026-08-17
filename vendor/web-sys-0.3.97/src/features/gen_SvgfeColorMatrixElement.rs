#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "SvgElement",
        extends = "Element",
        extends = "Node",
        extends = "EventTarget",
        extends = "::js_sys::Object",
        js_name = "SVGFEColorMatrixElement",
        typescript_type = "SVGFEColorMatrixElement"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgfeColorMatrixElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEColorMatrixElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgfeColorMatrixElement`*"]
    pub type SvgfeColorMatrixElement;
    #[cfg(feature = "SvgAnimatedString")]
    #[wasm_bindgen(method, getter, js_class = "SVGFEColorMatrixElement", js_name = "in1")]
    #[doc = "Getter for the `in1` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEColorMatrixElement/in1)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgfeColorMatrixElement`*"]
    pub fn in1(this: &SvgfeColorMatrixElement) -> SvgAnimatedString;
    #[cfg(feature = "SvgAnimatedEnumeration")]
    #[wasm_bindgen(method, getter, js_class = "SVGFEColorMatrixElement", js_name = "type")]
    #[doc = "Getter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEColorMatrixElement/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedEnumeration`, `SvgfeColorMatrixElement`*"]
    pub fn type_(this: &SvgfeColorMatrixElement) -> SvgAnimatedEnumeration;
    #[cfg(feature = "SvgAnimatedNumberList")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGFEColorMatrixElement",
        js_name = "values"
    )]
    #[doc = "Getter for the `values` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEColorMatrixElement/values)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedNumberList`, `SvgfeColorMatrixElement`*"]
    pub fn values(this: &SvgfeColorMatrixElement) -> SvgAnimatedNumberList;
    #[cfg(feature = "SvgAnimatedLength")]
    #[wasm_bindgen(method, getter, js_class = "SVGFEColorMatrixElement", js_name = "x")]
    #[doc = "Getter for the `x` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEColorMatrixElement/x)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeColorMatrixElement`*"]
    pub fn x(this: &SvgfeColorMatrixElement) -> SvgAnimatedLength;
    #[cfg(feature = "SvgAnimatedLength")]
    #[wasm_bindgen(method, getter, js_class = "SVGFEColorMatrixElement", js_name = "y")]
    #[doc = "Getter for the `y` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEColorMatrixElement/y)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeColorMatrixElement`*"]
    pub fn y(this: &SvgfeColorMatrixElement) -> SvgAnimatedLength;
    #[cfg(feature = "SvgAnimatedLength")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGFEColorMatrixElement",
        js_name = "width"
    )]
    #[doc = "Getter for the `width` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEColorMatrixElement/width)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeColorMatrixElement`*"]
    pub fn width(this: &SvgfeColorMatrixElement) -> SvgAnimatedLength;
    #[cfg(feature = "SvgAnimatedLength")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGFEColorMatrixElement",
        js_name = "height"
    )]
    #[doc = "Getter for the `height` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEColorMatrixElement/height)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeColorMatrixElement`*"]
    pub fn height(this: &SvgfeColorMatrixElement) -> SvgAnimatedLength;
    #[cfg(feature = "SvgAnimatedString")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGFEColorMatrixElement",
        js_name = "result"
    )]
    #[doc = "Getter for the `result` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEColorMatrixElement/result)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgfeColorMatrixElement`*"]
    pub fn result(this: &SvgfeColorMatrixElement) -> SvgAnimatedString;
}
impl SvgfeColorMatrixElement {
    #[doc = "The `SVGFEColorMatrixElement.SVG_FECOLORMATRIX_TYPE_UNKNOWN` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgfeColorMatrixElement`*"]
    pub const SVG_FECOLORMATRIX_TYPE_UNKNOWN: u16 = 0i64 as u16;
    #[doc = "The `SVGFEColorMatrixElement.SVG_FECOLORMATRIX_TYPE_MATRIX` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgfeColorMatrixElement`*"]
    pub const SVG_FECOLORMATRIX_TYPE_MATRIX: u16 = 1u64 as u16;
    #[doc = "The `SVGFEColorMatrixElement.SVG_FECOLORMATRIX_TYPE_SATURATE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgfeColorMatrixElement`*"]
    pub const SVG_FECOLORMATRIX_TYPE_SATURATE: u16 = 2u64 as u16;
    #[doc = "The `SVGFEColorMatrixElement.SVG_FECOLORMATRIX_TYPE_HUEROTATE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgfeColorMatrixElement`*"]
    pub const SVG_FECOLORMATRIX_TYPE_HUEROTATE: u16 = 3u64 as u16;
    #[doc = "The `SVGFEColorMatrixElement.SVG_FECOLORMATRIX_TYPE_LUMINANCETOALPHA` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgfeColorMatrixElement`*"]
    pub const SVG_FECOLORMATRIX_TYPE_LUMINANCETOALPHA: u16 = 4u64 as u16;
}
