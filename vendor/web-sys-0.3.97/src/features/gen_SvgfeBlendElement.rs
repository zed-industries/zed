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
        js_name = "SVGFEBlendElement",
        typescript_type = "SVGFEBlendElement"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgfeBlendElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEBlendElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgfeBlendElement`*"]
    pub type SvgfeBlendElement;
    #[cfg(feature = "SvgAnimatedString")]
    #[wasm_bindgen(method, getter, js_class = "SVGFEBlendElement", js_name = "in1")]
    #[doc = "Getter for the `in1` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEBlendElement/in1)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgfeBlendElement`*"]
    pub fn in1(this: &SvgfeBlendElement) -> SvgAnimatedString;
    #[cfg(feature = "SvgAnimatedString")]
    #[wasm_bindgen(method, getter, js_class = "SVGFEBlendElement", js_name = "in2")]
    #[doc = "Getter for the `in2` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEBlendElement/in2)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgfeBlendElement`*"]
    pub fn in2(this: &SvgfeBlendElement) -> SvgAnimatedString;
    #[cfg(feature = "SvgAnimatedEnumeration")]
    #[wasm_bindgen(method, getter, js_class = "SVGFEBlendElement", js_name = "mode")]
    #[doc = "Getter for the `mode` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEBlendElement/mode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedEnumeration`, `SvgfeBlendElement`*"]
    pub fn mode(this: &SvgfeBlendElement) -> SvgAnimatedEnumeration;
    #[cfg(feature = "SvgAnimatedLength")]
    #[wasm_bindgen(method, getter, js_class = "SVGFEBlendElement", js_name = "x")]
    #[doc = "Getter for the `x` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEBlendElement/x)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeBlendElement`*"]
    pub fn x(this: &SvgfeBlendElement) -> SvgAnimatedLength;
    #[cfg(feature = "SvgAnimatedLength")]
    #[wasm_bindgen(method, getter, js_class = "SVGFEBlendElement", js_name = "y")]
    #[doc = "Getter for the `y` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEBlendElement/y)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeBlendElement`*"]
    pub fn y(this: &SvgfeBlendElement) -> SvgAnimatedLength;
    #[cfg(feature = "SvgAnimatedLength")]
    #[wasm_bindgen(method, getter, js_class = "SVGFEBlendElement", js_name = "width")]
    #[doc = "Getter for the `width` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEBlendElement/width)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeBlendElement`*"]
    pub fn width(this: &SvgfeBlendElement) -> SvgAnimatedLength;
    #[cfg(feature = "SvgAnimatedLength")]
    #[wasm_bindgen(method, getter, js_class = "SVGFEBlendElement", js_name = "height")]
    #[doc = "Getter for the `height` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEBlendElement/height)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeBlendElement`*"]
    pub fn height(this: &SvgfeBlendElement) -> SvgAnimatedLength;
    #[cfg(feature = "SvgAnimatedString")]
    #[wasm_bindgen(method, getter, js_class = "SVGFEBlendElement", js_name = "result")]
    #[doc = "Getter for the `result` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEBlendElement/result)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgfeBlendElement`*"]
    pub fn result(this: &SvgfeBlendElement) -> SvgAnimatedString;
}
impl SvgfeBlendElement {
    #[doc = "The `SVGFEBlendElement.SVG_FEBLEND_MODE_UNKNOWN` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgfeBlendElement`*"]
    pub const SVG_FEBLEND_MODE_UNKNOWN: u16 = 0i64 as u16;
    #[doc = "The `SVGFEBlendElement.SVG_FEBLEND_MODE_NORMAL` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgfeBlendElement`*"]
    pub const SVG_FEBLEND_MODE_NORMAL: u16 = 1u64 as u16;
    #[doc = "The `SVGFEBlendElement.SVG_FEBLEND_MODE_MULTIPLY` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgfeBlendElement`*"]
    pub const SVG_FEBLEND_MODE_MULTIPLY: u16 = 2u64 as u16;
    #[doc = "The `SVGFEBlendElement.SVG_FEBLEND_MODE_SCREEN` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgfeBlendElement`*"]
    pub const SVG_FEBLEND_MODE_SCREEN: u16 = 3u64 as u16;
    #[doc = "The `SVGFEBlendElement.SVG_FEBLEND_MODE_DARKEN` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgfeBlendElement`*"]
    pub const SVG_FEBLEND_MODE_DARKEN: u16 = 4u64 as u16;
    #[doc = "The `SVGFEBlendElement.SVG_FEBLEND_MODE_LIGHTEN` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgfeBlendElement`*"]
    pub const SVG_FEBLEND_MODE_LIGHTEN: u16 = 5u64 as u16;
    #[doc = "The `SVGFEBlendElement.SVG_FEBLEND_MODE_OVERLAY` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgfeBlendElement`*"]
    pub const SVG_FEBLEND_MODE_OVERLAY: u16 = 6u64 as u16;
    #[doc = "The `SVGFEBlendElement.SVG_FEBLEND_MODE_COLOR_DODGE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgfeBlendElement`*"]
    pub const SVG_FEBLEND_MODE_COLOR_DODGE: u16 = 7u64 as u16;
    #[doc = "The `SVGFEBlendElement.SVG_FEBLEND_MODE_COLOR_BURN` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgfeBlendElement`*"]
    pub const SVG_FEBLEND_MODE_COLOR_BURN: u16 = 8u64 as u16;
    #[doc = "The `SVGFEBlendElement.SVG_FEBLEND_MODE_HARD_LIGHT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgfeBlendElement`*"]
    pub const SVG_FEBLEND_MODE_HARD_LIGHT: u16 = 9u64 as u16;
    #[doc = "The `SVGFEBlendElement.SVG_FEBLEND_MODE_SOFT_LIGHT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgfeBlendElement`*"]
    pub const SVG_FEBLEND_MODE_SOFT_LIGHT: u16 = 10u64 as u16;
    #[doc = "The `SVGFEBlendElement.SVG_FEBLEND_MODE_DIFFERENCE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgfeBlendElement`*"]
    pub const SVG_FEBLEND_MODE_DIFFERENCE: u16 = 11u64 as u16;
    #[doc = "The `SVGFEBlendElement.SVG_FEBLEND_MODE_EXCLUSION` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgfeBlendElement`*"]
    pub const SVG_FEBLEND_MODE_EXCLUSION: u16 = 12u64 as u16;
    #[doc = "The `SVGFEBlendElement.SVG_FEBLEND_MODE_HUE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgfeBlendElement`*"]
    pub const SVG_FEBLEND_MODE_HUE: u16 = 13u64 as u16;
    #[doc = "The `SVGFEBlendElement.SVG_FEBLEND_MODE_SATURATION` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgfeBlendElement`*"]
    pub const SVG_FEBLEND_MODE_SATURATION: u16 = 14u64 as u16;
    #[doc = "The `SVGFEBlendElement.SVG_FEBLEND_MODE_COLOR` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgfeBlendElement`*"]
    pub const SVG_FEBLEND_MODE_COLOR: u16 = 15u64 as u16;
    #[doc = "The `SVGFEBlendElement.SVG_FEBLEND_MODE_LUMINOSITY` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgfeBlendElement`*"]
    pub const SVG_FEBLEND_MODE_LUMINOSITY: u16 = 16u64 as u16;
}
