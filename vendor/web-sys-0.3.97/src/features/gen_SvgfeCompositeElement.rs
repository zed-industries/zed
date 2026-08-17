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
        js_name = "SVGFECompositeElement",
        typescript_type = "SVGFECompositeElement"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgfeCompositeElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFECompositeElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgfeCompositeElement`*"]
    pub type SvgfeCompositeElement;
    #[cfg(feature = "SvgAnimatedString")]
    #[wasm_bindgen(method, getter, js_class = "SVGFECompositeElement", js_name = "in1")]
    #[doc = "Getter for the `in1` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFECompositeElement/in1)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgfeCompositeElement`*"]
    pub fn in1(this: &SvgfeCompositeElement) -> SvgAnimatedString;
    #[cfg(feature = "SvgAnimatedString")]
    #[wasm_bindgen(method, getter, js_class = "SVGFECompositeElement", js_name = "in2")]
    #[doc = "Getter for the `in2` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFECompositeElement/in2)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgfeCompositeElement`*"]
    pub fn in2(this: &SvgfeCompositeElement) -> SvgAnimatedString;
    #[cfg(feature = "SvgAnimatedEnumeration")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGFECompositeElement",
        js_name = "operator"
    )]
    #[doc = "Getter for the `operator` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFECompositeElement/operator)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedEnumeration`, `SvgfeCompositeElement`*"]
    pub fn operator(this: &SvgfeCompositeElement) -> SvgAnimatedEnumeration;
    #[cfg(feature = "SvgAnimatedNumber")]
    #[wasm_bindgen(method, getter, js_class = "SVGFECompositeElement", js_name = "k1")]
    #[doc = "Getter for the `k1` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFECompositeElement/k1)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeCompositeElement`*"]
    pub fn k1(this: &SvgfeCompositeElement) -> SvgAnimatedNumber;
    #[cfg(feature = "SvgAnimatedNumber")]
    #[wasm_bindgen(method, getter, js_class = "SVGFECompositeElement", js_name = "k2")]
    #[doc = "Getter for the `k2` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFECompositeElement/k2)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeCompositeElement`*"]
    pub fn k2(this: &SvgfeCompositeElement) -> SvgAnimatedNumber;
    #[cfg(feature = "SvgAnimatedNumber")]
    #[wasm_bindgen(method, getter, js_class = "SVGFECompositeElement", js_name = "k3")]
    #[doc = "Getter for the `k3` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFECompositeElement/k3)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeCompositeElement`*"]
    pub fn k3(this: &SvgfeCompositeElement) -> SvgAnimatedNumber;
    #[cfg(feature = "SvgAnimatedNumber")]
    #[wasm_bindgen(method, getter, js_class = "SVGFECompositeElement", js_name = "k4")]
    #[doc = "Getter for the `k4` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFECompositeElement/k4)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeCompositeElement`*"]
    pub fn k4(this: &SvgfeCompositeElement) -> SvgAnimatedNumber;
    #[cfg(feature = "SvgAnimatedLength")]
    #[wasm_bindgen(method, getter, js_class = "SVGFECompositeElement", js_name = "x")]
    #[doc = "Getter for the `x` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFECompositeElement/x)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeCompositeElement`*"]
    pub fn x(this: &SvgfeCompositeElement) -> SvgAnimatedLength;
    #[cfg(feature = "SvgAnimatedLength")]
    #[wasm_bindgen(method, getter, js_class = "SVGFECompositeElement", js_name = "y")]
    #[doc = "Getter for the `y` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFECompositeElement/y)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeCompositeElement`*"]
    pub fn y(this: &SvgfeCompositeElement) -> SvgAnimatedLength;
    #[cfg(feature = "SvgAnimatedLength")]
    #[wasm_bindgen(method, getter, js_class = "SVGFECompositeElement", js_name = "width")]
    #[doc = "Getter for the `width` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFECompositeElement/width)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeCompositeElement`*"]
    pub fn width(this: &SvgfeCompositeElement) -> SvgAnimatedLength;
    #[cfg(feature = "SvgAnimatedLength")]
    #[wasm_bindgen(method, getter, js_class = "SVGFECompositeElement", js_name = "height")]
    #[doc = "Getter for the `height` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFECompositeElement/height)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeCompositeElement`*"]
    pub fn height(this: &SvgfeCompositeElement) -> SvgAnimatedLength;
    #[cfg(feature = "SvgAnimatedString")]
    #[wasm_bindgen(method, getter, js_class = "SVGFECompositeElement", js_name = "result")]
    #[doc = "Getter for the `result` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFECompositeElement/result)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgfeCompositeElement`*"]
    pub fn result(this: &SvgfeCompositeElement) -> SvgAnimatedString;
}
impl SvgfeCompositeElement {
    #[doc = "The `SVGFECompositeElement.SVG_FECOMPOSITE_OPERATOR_UNKNOWN` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgfeCompositeElement`*"]
    pub const SVG_FECOMPOSITE_OPERATOR_UNKNOWN: u16 = 0i64 as u16;
    #[doc = "The `SVGFECompositeElement.SVG_FECOMPOSITE_OPERATOR_OVER` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgfeCompositeElement`*"]
    pub const SVG_FECOMPOSITE_OPERATOR_OVER: u16 = 1u64 as u16;
    #[doc = "The `SVGFECompositeElement.SVG_FECOMPOSITE_OPERATOR_IN` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgfeCompositeElement`*"]
    pub const SVG_FECOMPOSITE_OPERATOR_IN: u16 = 2u64 as u16;
    #[doc = "The `SVGFECompositeElement.SVG_FECOMPOSITE_OPERATOR_OUT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgfeCompositeElement`*"]
    pub const SVG_FECOMPOSITE_OPERATOR_OUT: u16 = 3u64 as u16;
    #[doc = "The `SVGFECompositeElement.SVG_FECOMPOSITE_OPERATOR_ATOP` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgfeCompositeElement`*"]
    pub const SVG_FECOMPOSITE_OPERATOR_ATOP: u16 = 4u64 as u16;
    #[doc = "The `SVGFECompositeElement.SVG_FECOMPOSITE_OPERATOR_XOR` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgfeCompositeElement`*"]
    pub const SVG_FECOMPOSITE_OPERATOR_XOR: u16 = 5u64 as u16;
    #[doc = "The `SVGFECompositeElement.SVG_FECOMPOSITE_OPERATOR_ARITHMETIC` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgfeCompositeElement`*"]
    pub const SVG_FECOMPOSITE_OPERATOR_ARITHMETIC: u16 = 6u64 as u16;
}
