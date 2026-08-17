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
        js_name = "SVGFETurbulenceElement",
        typescript_type = "SVGFETurbulenceElement"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgfeTurbulenceElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFETurbulenceElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgfeTurbulenceElement`*"]
    pub type SvgfeTurbulenceElement;
    #[cfg(feature = "SvgAnimatedNumber")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGFETurbulenceElement",
        js_name = "baseFrequencyX"
    )]
    #[doc = "Getter for the `baseFrequencyX` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFETurbulenceElement/baseFrequencyX)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeTurbulenceElement`*"]
    pub fn base_frequency_x(this: &SvgfeTurbulenceElement) -> SvgAnimatedNumber;
    #[cfg(feature = "SvgAnimatedNumber")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGFETurbulenceElement",
        js_name = "baseFrequencyY"
    )]
    #[doc = "Getter for the `baseFrequencyY` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFETurbulenceElement/baseFrequencyY)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeTurbulenceElement`*"]
    pub fn base_frequency_y(this: &SvgfeTurbulenceElement) -> SvgAnimatedNumber;
    #[cfg(feature = "SvgAnimatedInteger")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGFETurbulenceElement",
        js_name = "numOctaves"
    )]
    #[doc = "Getter for the `numOctaves` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFETurbulenceElement/numOctaves)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedInteger`, `SvgfeTurbulenceElement`*"]
    pub fn num_octaves(this: &SvgfeTurbulenceElement) -> SvgAnimatedInteger;
    #[cfg(feature = "SvgAnimatedNumber")]
    #[wasm_bindgen(method, getter, js_class = "SVGFETurbulenceElement", js_name = "seed")]
    #[doc = "Getter for the `seed` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFETurbulenceElement/seed)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeTurbulenceElement`*"]
    pub fn seed(this: &SvgfeTurbulenceElement) -> SvgAnimatedNumber;
    #[cfg(feature = "SvgAnimatedEnumeration")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGFETurbulenceElement",
        js_name = "stitchTiles"
    )]
    #[doc = "Getter for the `stitchTiles` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFETurbulenceElement/stitchTiles)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedEnumeration`, `SvgfeTurbulenceElement`*"]
    pub fn stitch_tiles(this: &SvgfeTurbulenceElement) -> SvgAnimatedEnumeration;
    #[cfg(feature = "SvgAnimatedEnumeration")]
    #[wasm_bindgen(method, getter, js_class = "SVGFETurbulenceElement", js_name = "type")]
    #[doc = "Getter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFETurbulenceElement/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedEnumeration`, `SvgfeTurbulenceElement`*"]
    pub fn type_(this: &SvgfeTurbulenceElement) -> SvgAnimatedEnumeration;
    #[cfg(feature = "SvgAnimatedLength")]
    #[wasm_bindgen(method, getter, js_class = "SVGFETurbulenceElement", js_name = "x")]
    #[doc = "Getter for the `x` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFETurbulenceElement/x)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeTurbulenceElement`*"]
    pub fn x(this: &SvgfeTurbulenceElement) -> SvgAnimatedLength;
    #[cfg(feature = "SvgAnimatedLength")]
    #[wasm_bindgen(method, getter, js_class = "SVGFETurbulenceElement", js_name = "y")]
    #[doc = "Getter for the `y` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFETurbulenceElement/y)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeTurbulenceElement`*"]
    pub fn y(this: &SvgfeTurbulenceElement) -> SvgAnimatedLength;
    #[cfg(feature = "SvgAnimatedLength")]
    #[wasm_bindgen(method, getter, js_class = "SVGFETurbulenceElement", js_name = "width")]
    #[doc = "Getter for the `width` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFETurbulenceElement/width)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeTurbulenceElement`*"]
    pub fn width(this: &SvgfeTurbulenceElement) -> SvgAnimatedLength;
    #[cfg(feature = "SvgAnimatedLength")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGFETurbulenceElement",
        js_name = "height"
    )]
    #[doc = "Getter for the `height` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFETurbulenceElement/height)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeTurbulenceElement`*"]
    pub fn height(this: &SvgfeTurbulenceElement) -> SvgAnimatedLength;
    #[cfg(feature = "SvgAnimatedString")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGFETurbulenceElement",
        js_name = "result"
    )]
    #[doc = "Getter for the `result` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFETurbulenceElement/result)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgfeTurbulenceElement`*"]
    pub fn result(this: &SvgfeTurbulenceElement) -> SvgAnimatedString;
}
impl SvgfeTurbulenceElement {
    #[doc = "The `SVGFETurbulenceElement.SVG_TURBULENCE_TYPE_UNKNOWN` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgfeTurbulenceElement`*"]
    pub const SVG_TURBULENCE_TYPE_UNKNOWN: u16 = 0i64 as u16;
    #[doc = "The `SVGFETurbulenceElement.SVG_TURBULENCE_TYPE_FRACTALNOISE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgfeTurbulenceElement`*"]
    pub const SVG_TURBULENCE_TYPE_FRACTALNOISE: u16 = 1u64 as u16;
    #[doc = "The `SVGFETurbulenceElement.SVG_TURBULENCE_TYPE_TURBULENCE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgfeTurbulenceElement`*"]
    pub const SVG_TURBULENCE_TYPE_TURBULENCE: u16 = 2u64 as u16;
    #[doc = "The `SVGFETurbulenceElement.SVG_STITCHTYPE_UNKNOWN` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgfeTurbulenceElement`*"]
    pub const SVG_STITCHTYPE_UNKNOWN: u16 = 0i64 as u16;
    #[doc = "The `SVGFETurbulenceElement.SVG_STITCHTYPE_STITCH` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgfeTurbulenceElement`*"]
    pub const SVG_STITCHTYPE_STITCH: u16 = 1u64 as u16;
    #[doc = "The `SVGFETurbulenceElement.SVG_STITCHTYPE_NOSTITCH` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgfeTurbulenceElement`*"]
    pub const SVG_STITCHTYPE_NOSTITCH: u16 = 2u64 as u16;
}
