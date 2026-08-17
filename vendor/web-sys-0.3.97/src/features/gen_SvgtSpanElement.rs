#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "SvgTextPositioningElement",
        extends = "SvgTextContentElement",
        extends = "SvgGraphicsElement",
        extends = "SvgElement",
        extends = "Element",
        extends = "Node",
        extends = "EventTarget",
        extends = "::js_sys::Object",
        js_name = "SVGTSpanElement",
        typescript_type = "SVGTSpanElement"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgtSpanElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTSpanElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgtSpanElement`*"]
    pub type SvgtSpanElement;
}
