#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "CharacterData",
        extends = "Node",
        extends = "EventTarget",
        extends = "::js_sys::Object",
        js_name = "ProcessingInstruction",
        typescript_type = "ProcessingInstruction"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ProcessingInstruction` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ProcessingInstruction)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProcessingInstruction`*"]
    pub type ProcessingInstruction;
    #[wasm_bindgen(method, getter, js_class = "ProcessingInstruction", js_name = "target")]
    #[doc = "Getter for the `target` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ProcessingInstruction/target)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProcessingInstruction`*"]
    pub fn target(this: &ProcessingInstruction) -> ::alloc::string::String;
    #[cfg(feature = "StyleSheet")]
    #[wasm_bindgen(method, getter, js_class = "ProcessingInstruction", js_name = "sheet")]
    #[doc = "Getter for the `sheet` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ProcessingInstruction/sheet)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProcessingInstruction`, `StyleSheet`*"]
    pub fn sheet(this: &ProcessingInstruction) -> Option<StyleSheet>;
}
