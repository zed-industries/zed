#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "SvgComponentTransferFunctionElement",
        extends = "SvgElement",
        extends = "Element",
        extends = "Node",
        extends = "EventTarget",
        extends = "::js_sys::Object",
        js_name = "SVGFEFuncGElement",
        typescript_type = "SVGFEFuncGElement"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgfeFuncGElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEFuncGElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgfeFuncGElement`*"]
    pub type SvgfeFuncGElement;
}
