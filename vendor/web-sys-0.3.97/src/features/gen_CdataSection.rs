#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "Text",
        extends = "CharacterData",
        extends = "Node",
        extends = "EventTarget",
        extends = "::js_sys::Object",
        js_name = "CDATASection",
        typescript_type = "CDATASection"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CdataSection` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CDATASection)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CdataSection`*"]
    pub type CdataSection;
}
