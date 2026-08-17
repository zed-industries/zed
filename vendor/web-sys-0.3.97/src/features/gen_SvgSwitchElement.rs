#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "SvgGraphicsElement",
        extends = "SvgElement",
        extends = "Element",
        extends = "Node",
        extends = "EventTarget",
        extends = "::js_sys::Object",
        js_name = "SVGSwitchElement",
        typescript_type = "SVGSwitchElement"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgSwitchElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSwitchElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgSwitchElement`*"]
    pub type SvgSwitchElement;
}
