#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "AudioNode",
        extends = "EventTarget",
        extends = "::js_sys::Object",
        js_name = "ScriptProcessorNode",
        typescript_type = "ScriptProcessorNode"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ScriptProcessorNode` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScriptProcessorNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScriptProcessorNode`*"]
    pub type ScriptProcessorNode;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "ScriptProcessorNode",
        js_name = "onaudioprocess"
    )]
    #[doc = "Getter for the `onaudioprocess` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScriptProcessorNode/onaudioprocess)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScriptProcessorNode`*"]
    pub fn onaudioprocess(this: &ScriptProcessorNode) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "ScriptProcessorNode",
        js_name = "onaudioprocess"
    )]
    #[doc = "Setter for the `onaudioprocess` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScriptProcessorNode/onaudioprocess)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScriptProcessorNode`*"]
    pub fn set_onaudioprocess(this: &ScriptProcessorNode, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "ScriptProcessorNode",
        js_name = "bufferSize"
    )]
    #[doc = "Getter for the `bufferSize` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScriptProcessorNode/bufferSize)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScriptProcessorNode`*"]
    pub fn buffer_size(this: &ScriptProcessorNode) -> i32;
}
