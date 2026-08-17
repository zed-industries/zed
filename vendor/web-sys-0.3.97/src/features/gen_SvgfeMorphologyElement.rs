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
        js_name = "SVGFEMorphologyElement",
        typescript_type = "SVGFEMorphologyElement"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgfeMorphologyElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEMorphologyElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgfeMorphologyElement`*"]
    pub type SvgfeMorphologyElement;
    #[cfg(feature = "SvgAnimatedString")]
    #[wasm_bindgen(method, getter, js_class = "SVGFEMorphologyElement", js_name = "in1")]
    #[doc = "Getter for the `in1` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEMorphologyElement/in1)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgfeMorphologyElement`*"]
    pub fn in1(this: &SvgfeMorphologyElement) -> SvgAnimatedString;
    #[cfg(feature = "SvgAnimatedEnumeration")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGFEMorphologyElement",
        js_name = "operator"
    )]
    #[doc = "Getter for the `operator` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEMorphologyElement/operator)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedEnumeration`, `SvgfeMorphologyElement`*"]
    pub fn operator(this: &SvgfeMorphologyElement) -> SvgAnimatedEnumeration;
    #[cfg(feature = "SvgAnimatedNumber")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGFEMorphologyElement",
        js_name = "radiusX"
    )]
    #[doc = "Getter for the `radiusX` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEMorphologyElement/radiusX)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeMorphologyElement`*"]
    pub fn radius_x(this: &SvgfeMorphologyElement) -> SvgAnimatedNumber;
    #[cfg(feature = "SvgAnimatedNumber")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGFEMorphologyElement",
        js_name = "radiusY"
    )]
    #[doc = "Getter for the `radiusY` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEMorphologyElement/radiusY)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeMorphologyElement`*"]
    pub fn radius_y(this: &SvgfeMorphologyElement) -> SvgAnimatedNumber;
    #[cfg(feature = "SvgAnimatedLength")]
    #[wasm_bindgen(method, getter, js_class = "SVGFEMorphologyElement", js_name = "x")]
    #[doc = "Getter for the `x` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEMorphologyElement/x)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeMorphologyElement`*"]
    pub fn x(this: &SvgfeMorphologyElement) -> SvgAnimatedLength;
    #[cfg(feature = "SvgAnimatedLength")]
    #[wasm_bindgen(method, getter, js_class = "SVGFEMorphologyElement", js_name = "y")]
    #[doc = "Getter for the `y` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEMorphologyElement/y)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeMorphologyElement`*"]
    pub fn y(this: &SvgfeMorphologyElement) -> SvgAnimatedLength;
    #[cfg(feature = "SvgAnimatedLength")]
    #[wasm_bindgen(method, getter, js_class = "SVGFEMorphologyElement", js_name = "width")]
    #[doc = "Getter for the `width` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEMorphologyElement/width)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeMorphologyElement`*"]
    pub fn width(this: &SvgfeMorphologyElement) -> SvgAnimatedLength;
    #[cfg(feature = "SvgAnimatedLength")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGFEMorphologyElement",
        js_name = "height"
    )]
    #[doc = "Getter for the `height` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEMorphologyElement/height)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeMorphologyElement`*"]
    pub fn height(this: &SvgfeMorphologyElement) -> SvgAnimatedLength;
    #[cfg(feature = "SvgAnimatedString")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGFEMorphologyElement",
        js_name = "result"
    )]
    #[doc = "Getter for the `result` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEMorphologyElement/result)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgfeMorphologyElement`*"]
    pub fn result(this: &SvgfeMorphologyElement) -> SvgAnimatedString;
}
impl SvgfeMorphologyElement {
    #[doc = "The `SVGFEMorphologyElement.SVG_MORPHOLOGY_OPERATOR_UNKNOWN` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgfeMorphologyElement`*"]
    pub const SVG_MORPHOLOGY_OPERATOR_UNKNOWN: u16 = 0i64 as u16;
    #[doc = "The `SVGFEMorphologyElement.SVG_MORPHOLOGY_OPERATOR_ERODE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgfeMorphologyElement`*"]
    pub const SVG_MORPHOLOGY_OPERATOR_ERODE: u16 = 1u64 as u16;
    #[doc = "The `SVGFEMorphologyElement.SVG_MORPHOLOGY_OPERATOR_DILATE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgfeMorphologyElement`*"]
    pub const SVG_MORPHOLOGY_OPERATOR_DILATE: u16 = 2u64 as u16;
}
