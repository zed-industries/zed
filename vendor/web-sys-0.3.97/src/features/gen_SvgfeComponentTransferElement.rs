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
        js_name = "SVGFEComponentTransferElement",
        typescript_type = "SVGFEComponentTransferElement"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgfeComponentTransferElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEComponentTransferElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgfeComponentTransferElement`*"]
    pub type SvgfeComponentTransferElement;
    #[cfg(feature = "SvgAnimatedString")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGFEComponentTransferElement",
        js_name = "in1"
    )]
    #[doc = "Getter for the `in1` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEComponentTransferElement/in1)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgfeComponentTransferElement`*"]
    pub fn in1(this: &SvgfeComponentTransferElement) -> SvgAnimatedString;
    #[cfg(feature = "SvgAnimatedLength")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGFEComponentTransferElement",
        js_name = "x"
    )]
    #[doc = "Getter for the `x` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEComponentTransferElement/x)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeComponentTransferElement`*"]
    pub fn x(this: &SvgfeComponentTransferElement) -> SvgAnimatedLength;
    #[cfg(feature = "SvgAnimatedLength")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGFEComponentTransferElement",
        js_name = "y"
    )]
    #[doc = "Getter for the `y` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEComponentTransferElement/y)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeComponentTransferElement`*"]
    pub fn y(this: &SvgfeComponentTransferElement) -> SvgAnimatedLength;
    #[cfg(feature = "SvgAnimatedLength")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGFEComponentTransferElement",
        js_name = "width"
    )]
    #[doc = "Getter for the `width` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEComponentTransferElement/width)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeComponentTransferElement`*"]
    pub fn width(this: &SvgfeComponentTransferElement) -> SvgAnimatedLength;
    #[cfg(feature = "SvgAnimatedLength")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGFEComponentTransferElement",
        js_name = "height"
    )]
    #[doc = "Getter for the `height` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEComponentTransferElement/height)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeComponentTransferElement`*"]
    pub fn height(this: &SvgfeComponentTransferElement) -> SvgAnimatedLength;
    #[cfg(feature = "SvgAnimatedString")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGFEComponentTransferElement",
        js_name = "result"
    )]
    #[doc = "Getter for the `result` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEComponentTransferElement/result)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgfeComponentTransferElement`*"]
    pub fn result(this: &SvgfeComponentTransferElement) -> SvgAnimatedString;
}
