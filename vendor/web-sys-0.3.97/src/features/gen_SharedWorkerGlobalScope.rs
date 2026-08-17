#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "WorkerGlobalScope",
        extends = "EventTarget",
        extends = "::js_sys::Object",
        js_name = "SharedWorkerGlobalScope",
        typescript_type = "SharedWorkerGlobalScope"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SharedWorkerGlobalScope` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SharedWorkerGlobalScope)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SharedWorkerGlobalScope`*"]
    pub type SharedWorkerGlobalScope;
    #[wasm_bindgen(method, getter, js_class = "SharedWorkerGlobalScope", js_name = "name")]
    #[doc = "Getter for the `name` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SharedWorkerGlobalScope/name)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SharedWorkerGlobalScope`*"]
    pub fn name(this: &SharedWorkerGlobalScope) -> ::alloc::string::String;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SharedWorkerGlobalScope",
        js_name = "onconnect"
    )]
    #[doc = "Getter for the `onconnect` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SharedWorkerGlobalScope/onconnect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SharedWorkerGlobalScope`*"]
    pub fn onconnect(this: &SharedWorkerGlobalScope) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "SharedWorkerGlobalScope",
        js_name = "onconnect"
    )]
    #[doc = "Setter for the `onconnect` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SharedWorkerGlobalScope/onconnect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SharedWorkerGlobalScope`*"]
    pub fn set_onconnect(this: &SharedWorkerGlobalScope, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, js_class = "SharedWorkerGlobalScope")]
    #[doc = "The `close()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SharedWorkerGlobalScope/close)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SharedWorkerGlobalScope`*"]
    pub fn close(this: &SharedWorkerGlobalScope);
}
