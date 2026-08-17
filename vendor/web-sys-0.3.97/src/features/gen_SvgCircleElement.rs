#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "SvgGeometryElement",
        extends = "SvgGraphicsElement",
        extends = "SvgElement",
        extends = "Element",
        extends = "Node",
        extends = "EventTarget",
        extends = "::js_sys::Object",
        js_name = "SVGCircleElement",
        typescript_type = "SVGCircleElement"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgCircleElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGCircleElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgCircleElement`*"]
    pub type SvgCircleElement;
    #[cfg(feature = "SvgAnimatedLength")]
    #[wasm_bindgen(method, getter, js_class = "SVGCircleElement", js_name = "cx")]
    #[doc = "Getter for the `cx` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGCircleElement/cx)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgCircleElement`*"]
    pub fn cx(this: &SvgCircleElement) -> SvgAnimatedLength;
    #[cfg(feature = "SvgAnimatedLength")]
    #[wasm_bindgen(method, getter, js_class = "SVGCircleElement", js_name = "cy")]
    #[doc = "Getter for the `cy` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGCircleElement/cy)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgCircleElement`*"]
    pub fn cy(this: &SvgCircleElement) -> SvgAnimatedLength;
    #[cfg(feature = "SvgAnimatedLength")]
    #[wasm_bindgen(method, getter, js_class = "SVGCircleElement", js_name = "r")]
    #[doc = "Getter for the `r` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGCircleElement/r)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgCircleElement`*"]
    pub fn r(this: &SvgCircleElement) -> SvgAnimatedLength;
}
