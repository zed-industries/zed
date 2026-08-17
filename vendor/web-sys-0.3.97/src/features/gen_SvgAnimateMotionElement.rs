#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "SvgAnimationElement",
        extends = "SvgElement",
        extends = "Element",
        extends = "Node",
        extends = "EventTarget",
        extends = "::js_sys::Object",
        js_name = "SVGAnimateMotionElement",
        typescript_type = "SVGAnimateMotionElement"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgAnimateMotionElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimateMotionElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimateMotionElement`*"]
    pub type SvgAnimateMotionElement;
}
