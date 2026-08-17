#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "MutationObserver",
        typescript_type = "MutationObserver"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MutationObserver` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MutationObserver)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationObserver`*"]
    pub type MutationObserver;
    #[wasm_bindgen(catch, constructor, js_class = "MutationObserver")]
    #[doc = "The `new MutationObserver(..)` constructor, creating a new instance of `MutationObserver`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MutationObserver/MutationObserver)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationObserver`*"]
    pub fn new(mutation_callback: &::js_sys::Function) -> Result<MutationObserver, JsValue>;
    #[wasm_bindgen(method, js_class = "MutationObserver")]
    #[doc = "The `disconnect()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MutationObserver/disconnect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationObserver`*"]
    pub fn disconnect(this: &MutationObserver);
    #[cfg(feature = "Node")]
    #[wasm_bindgen(catch, method, js_class = "MutationObserver")]
    #[doc = "The `observe()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MutationObserver/observe)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationObserver`, `Node`*"]
    pub fn observe(this: &MutationObserver, target: &Node) -> Result<(), JsValue>;
    #[cfg(all(feature = "MutationObserverInit", feature = "Node",))]
    #[wasm_bindgen(catch, method, js_class = "MutationObserver", js_name = "observe")]
    #[doc = "The `observe()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MutationObserver/observe)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationObserver`, `MutationObserverInit`, `Node`*"]
    pub fn observe_with_options(
        this: &MutationObserver,
        target: &Node,
        options: &MutationObserverInit,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(method, js_class = "MutationObserver", js_name = "takeRecords")]
    #[doc = "The `takeRecords()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MutationObserver/takeRecords)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationObserver`*"]
    pub fn take_records(this: &MutationObserver) -> ::js_sys::Array;
}
