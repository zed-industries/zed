#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "EventTarget",
        extends = "::js_sys::Object",
        js_name = "SourceBufferList",
        typescript_type = "SourceBufferList"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SourceBufferList` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBufferList)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SourceBufferList`*"]
    pub type SourceBufferList;
    #[wasm_bindgen(method, getter, js_class = "SourceBufferList", js_name = "length")]
    #[doc = "Getter for the `length` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBufferList/length)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SourceBufferList`*"]
    pub fn length(this: &SourceBufferList) -> u32;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SourceBufferList",
        js_name = "onaddsourcebuffer"
    )]
    #[doc = "Getter for the `onaddsourcebuffer` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBufferList/onaddsourcebuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SourceBufferList`*"]
    pub fn onaddsourcebuffer(this: &SourceBufferList) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "SourceBufferList",
        js_name = "onaddsourcebuffer"
    )]
    #[doc = "Setter for the `onaddsourcebuffer` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBufferList/onaddsourcebuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SourceBufferList`*"]
    pub fn set_onaddsourcebuffer(this: &SourceBufferList, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SourceBufferList",
        js_name = "onremovesourcebuffer"
    )]
    #[doc = "Getter for the `onremovesourcebuffer` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBufferList/onremovesourcebuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SourceBufferList`*"]
    pub fn onremovesourcebuffer(this: &SourceBufferList) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "SourceBufferList",
        js_name = "onremovesourcebuffer"
    )]
    #[doc = "Setter for the `onremovesourcebuffer` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBufferList/onremovesourcebuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SourceBufferList`*"]
    pub fn set_onremovesourcebuffer(this: &SourceBufferList, value: Option<&::js_sys::Function>);
    #[cfg(feature = "SourceBuffer")]
    #[wasm_bindgen(method, js_class = "SourceBufferList", indexing_getter)]
    #[doc = "Indexing getter. As in the literal Javascript `this[key]`."]
    #[doc = ""]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SourceBuffer`, `SourceBufferList`*"]
    pub fn get(this: &SourceBufferList, index: u32) -> Option<SourceBuffer>;
}
