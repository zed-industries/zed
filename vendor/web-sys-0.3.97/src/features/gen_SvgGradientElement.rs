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
        js_name = "SVGGradientElement",
        typescript_type = "SVGGradientElement"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgGradientElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGGradientElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgGradientElement`*"]
    pub type SvgGradientElement;
    #[cfg(feature = "SvgAnimatedEnumeration")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGGradientElement",
        js_name = "gradientUnits"
    )]
    #[doc = "Getter for the `gradientUnits` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGGradientElement/gradientUnits)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedEnumeration`, `SvgGradientElement`*"]
    pub fn gradient_units(this: &SvgGradientElement) -> SvgAnimatedEnumeration;
    #[cfg(feature = "SvgAnimatedTransformList")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGGradientElement",
        js_name = "gradientTransform"
    )]
    #[doc = "Getter for the `gradientTransform` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGGradientElement/gradientTransform)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedTransformList`, `SvgGradientElement`*"]
    pub fn gradient_transform(this: &SvgGradientElement) -> SvgAnimatedTransformList;
    #[cfg(feature = "SvgAnimatedEnumeration")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGGradientElement",
        js_name = "spreadMethod"
    )]
    #[doc = "Getter for the `spreadMethod` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGGradientElement/spreadMethod)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedEnumeration`, `SvgGradientElement`*"]
    pub fn spread_method(this: &SvgGradientElement) -> SvgAnimatedEnumeration;
    #[cfg(feature = "SvgAnimatedString")]
    #[wasm_bindgen(method, getter, js_class = "SVGGradientElement", js_name = "href")]
    #[doc = "Getter for the `href` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGGradientElement/href)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgGradientElement`*"]
    pub fn href(this: &SvgGradientElement) -> SvgAnimatedString;
}
impl SvgGradientElement {
    #[doc = "The `SVGGradientElement.SVG_SPREADMETHOD_UNKNOWN` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgGradientElement`*"]
    pub const SVG_SPREADMETHOD_UNKNOWN: u16 = 0i64 as u16;
    #[doc = "The `SVGGradientElement.SVG_SPREADMETHOD_PAD` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgGradientElement`*"]
    pub const SVG_SPREADMETHOD_PAD: u16 = 1u64 as u16;
    #[doc = "The `SVGGradientElement.SVG_SPREADMETHOD_REFLECT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgGradientElement`*"]
    pub const SVG_SPREADMETHOD_REFLECT: u16 = 2u64 as u16;
    #[doc = "The `SVGGradientElement.SVG_SPREADMETHOD_REPEAT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgGradientElement`*"]
    pub const SVG_SPREADMETHOD_REPEAT: u16 = 3u64 as u16;
}
