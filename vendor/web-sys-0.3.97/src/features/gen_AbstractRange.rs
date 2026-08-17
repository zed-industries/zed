#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "AbstractRange",
        typescript_type = "AbstractRange"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AbstractRange` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AbstractRange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AbstractRange`*"]
    pub type AbstractRange;
    #[cfg(feature = "Node")]
    #[wasm_bindgen(method, getter, js_class = "AbstractRange", js_name = "startContainer")]
    #[doc = "Getter for the `startContainer` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AbstractRange/startContainer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AbstractRange`, `Node`*"]
    pub fn start_container(this: &AbstractRange) -> Node;
    #[wasm_bindgen(method, getter, js_class = "AbstractRange", js_name = "startOffset")]
    #[doc = "Getter for the `startOffset` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AbstractRange/startOffset)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AbstractRange`*"]
    pub fn start_offset(this: &AbstractRange) -> u32;
    #[cfg(feature = "Node")]
    #[wasm_bindgen(method, getter, js_class = "AbstractRange", js_name = "endContainer")]
    #[doc = "Getter for the `endContainer` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AbstractRange/endContainer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AbstractRange`, `Node`*"]
    pub fn end_container(this: &AbstractRange) -> Node;
    #[wasm_bindgen(method, getter, js_class = "AbstractRange", js_name = "endOffset")]
    #[doc = "Getter for the `endOffset` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AbstractRange/endOffset)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AbstractRange`*"]
    pub fn end_offset(this: &AbstractRange) -> u32;
    #[wasm_bindgen(method, getter, js_class = "AbstractRange", js_name = "collapsed")]
    #[doc = "Getter for the `collapsed` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AbstractRange/collapsed)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AbstractRange`*"]
    pub fn collapsed(this: &AbstractRange) -> bool;
}
