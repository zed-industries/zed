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
        js_name = "SVGMaskElement",
        typescript_type = "SVGMaskElement"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgMaskElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMaskElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgMaskElement`*"]
    pub type SvgMaskElement;
    #[cfg(feature = "SvgAnimatedEnumeration")]
    #[wasm_bindgen(method, getter, js_class = "SVGMaskElement", js_name = "maskUnits")]
    #[doc = "Getter for the `maskUnits` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMaskElement/maskUnits)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedEnumeration`, `SvgMaskElement`*"]
    pub fn mask_units(this: &SvgMaskElement) -> SvgAnimatedEnumeration;
    #[cfg(feature = "SvgAnimatedEnumeration")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGMaskElement",
        js_name = "maskContentUnits"
    )]
    #[doc = "Getter for the `maskContentUnits` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMaskElement/maskContentUnits)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedEnumeration`, `SvgMaskElement`*"]
    pub fn mask_content_units(this: &SvgMaskElement) -> SvgAnimatedEnumeration;
    #[cfg(feature = "SvgAnimatedLength")]
    #[wasm_bindgen(method, getter, js_class = "SVGMaskElement", js_name = "x")]
    #[doc = "Getter for the `x` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMaskElement/x)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgMaskElement`*"]
    pub fn x(this: &SvgMaskElement) -> SvgAnimatedLength;
    #[cfg(feature = "SvgAnimatedLength")]
    #[wasm_bindgen(method, getter, js_class = "SVGMaskElement", js_name = "y")]
    #[doc = "Getter for the `y` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMaskElement/y)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgMaskElement`*"]
    pub fn y(this: &SvgMaskElement) -> SvgAnimatedLength;
    #[cfg(feature = "SvgAnimatedLength")]
    #[wasm_bindgen(method, getter, js_class = "SVGMaskElement", js_name = "width")]
    #[doc = "Getter for the `width` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMaskElement/width)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgMaskElement`*"]
    pub fn width(this: &SvgMaskElement) -> SvgAnimatedLength;
    #[cfg(feature = "SvgAnimatedLength")]
    #[wasm_bindgen(method, getter, js_class = "SVGMaskElement", js_name = "height")]
    #[doc = "Getter for the `height` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMaskElement/height)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgMaskElement`*"]
    pub fn height(this: &SvgMaskElement) -> SvgAnimatedLength;
}
impl SvgMaskElement {
    #[doc = "The `SVGMaskElement.SVG_MASKTYPE_LUMINANCE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgMaskElement`*"]
    pub const SVG_MASKTYPE_LUMINANCE: u16 = 0i64 as u16;
    #[doc = "The `SVGMaskElement.SVG_MASKTYPE_ALPHA` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgMaskElement`*"]
    pub const SVG_MASKTYPE_ALPHA: u16 = 1u64 as u16;
}
