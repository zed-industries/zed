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
        js_name = "SVGFEDisplacementMapElement",
        typescript_type = "SVGFEDisplacementMapElement"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgfeDisplacementMapElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDisplacementMapElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgfeDisplacementMapElement`*"]
    pub type SvgfeDisplacementMapElement;
    #[cfg(feature = "SvgAnimatedString")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGFEDisplacementMapElement",
        js_name = "in1"
    )]
    #[doc = "Getter for the `in1` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDisplacementMapElement/in1)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgfeDisplacementMapElement`*"]
    pub fn in1(this: &SvgfeDisplacementMapElement) -> SvgAnimatedString;
    #[cfg(feature = "SvgAnimatedString")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGFEDisplacementMapElement",
        js_name = "in2"
    )]
    #[doc = "Getter for the `in2` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDisplacementMapElement/in2)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgfeDisplacementMapElement`*"]
    pub fn in2(this: &SvgfeDisplacementMapElement) -> SvgAnimatedString;
    #[cfg(feature = "SvgAnimatedNumber")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGFEDisplacementMapElement",
        js_name = "scale"
    )]
    #[doc = "Getter for the `scale` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDisplacementMapElement/scale)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeDisplacementMapElement`*"]
    pub fn scale(this: &SvgfeDisplacementMapElement) -> SvgAnimatedNumber;
    #[cfg(feature = "SvgAnimatedEnumeration")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGFEDisplacementMapElement",
        js_name = "xChannelSelector"
    )]
    #[doc = "Getter for the `xChannelSelector` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDisplacementMapElement/xChannelSelector)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedEnumeration`, `SvgfeDisplacementMapElement`*"]
    pub fn x_channel_selector(this: &SvgfeDisplacementMapElement) -> SvgAnimatedEnumeration;
    #[cfg(feature = "SvgAnimatedEnumeration")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGFEDisplacementMapElement",
        js_name = "yChannelSelector"
    )]
    #[doc = "Getter for the `yChannelSelector` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDisplacementMapElement/yChannelSelector)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedEnumeration`, `SvgfeDisplacementMapElement`*"]
    pub fn y_channel_selector(this: &SvgfeDisplacementMapElement) -> SvgAnimatedEnumeration;
    #[cfg(feature = "SvgAnimatedLength")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGFEDisplacementMapElement",
        js_name = "x"
    )]
    #[doc = "Getter for the `x` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDisplacementMapElement/x)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeDisplacementMapElement`*"]
    pub fn x(this: &SvgfeDisplacementMapElement) -> SvgAnimatedLength;
    #[cfg(feature = "SvgAnimatedLength")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGFEDisplacementMapElement",
        js_name = "y"
    )]
    #[doc = "Getter for the `y` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDisplacementMapElement/y)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeDisplacementMapElement`*"]
    pub fn y(this: &SvgfeDisplacementMapElement) -> SvgAnimatedLength;
    #[cfg(feature = "SvgAnimatedLength")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGFEDisplacementMapElement",
        js_name = "width"
    )]
    #[doc = "Getter for the `width` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDisplacementMapElement/width)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeDisplacementMapElement`*"]
    pub fn width(this: &SvgfeDisplacementMapElement) -> SvgAnimatedLength;
    #[cfg(feature = "SvgAnimatedLength")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGFEDisplacementMapElement",
        js_name = "height"
    )]
    #[doc = "Getter for the `height` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDisplacementMapElement/height)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeDisplacementMapElement`*"]
    pub fn height(this: &SvgfeDisplacementMapElement) -> SvgAnimatedLength;
    #[cfg(feature = "SvgAnimatedString")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGFEDisplacementMapElement",
        js_name = "result"
    )]
    #[doc = "Getter for the `result` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDisplacementMapElement/result)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgfeDisplacementMapElement`*"]
    pub fn result(this: &SvgfeDisplacementMapElement) -> SvgAnimatedString;
}
impl SvgfeDisplacementMapElement {
    #[doc = "The `SVGFEDisplacementMapElement.SVG_CHANNEL_UNKNOWN` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgfeDisplacementMapElement`*"]
    pub const SVG_CHANNEL_UNKNOWN: u16 = 0i64 as u16;
    #[doc = "The `SVGFEDisplacementMapElement.SVG_CHANNEL_R` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgfeDisplacementMapElement`*"]
    pub const SVG_CHANNEL_R: u16 = 1u64 as u16;
    #[doc = "The `SVGFEDisplacementMapElement.SVG_CHANNEL_G` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgfeDisplacementMapElement`*"]
    pub const SVG_CHANNEL_G: u16 = 2u64 as u16;
    #[doc = "The `SVGFEDisplacementMapElement.SVG_CHANNEL_B` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgfeDisplacementMapElement`*"]
    pub const SVG_CHANNEL_B: u16 = 3u64 as u16;
    #[doc = "The `SVGFEDisplacementMapElement.SVG_CHANNEL_A` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgfeDisplacementMapElement`*"]
    pub const SVG_CHANNEL_A: u16 = 4u64 as u16;
}
