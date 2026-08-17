#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "EventTarget",
        extends = "::js_sys::Object",
        js_name = "FetchObserver",
        typescript_type = "FetchObserver"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `FetchObserver` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FetchObserver)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FetchObserver`*"]
    pub type FetchObserver;
    #[cfg(feature = "FetchState")]
    #[wasm_bindgen(method, getter, js_class = "FetchObserver", js_name = "state")]
    #[doc = "Getter for the `state` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FetchObserver/state)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FetchObserver`, `FetchState`*"]
    pub fn state(this: &FetchObserver) -> FetchState;
    #[wasm_bindgen(method, getter, js_class = "FetchObserver", js_name = "onstatechange")]
    #[doc = "Getter for the `onstatechange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FetchObserver/onstatechange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FetchObserver`*"]
    pub fn onstatechange(this: &FetchObserver) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "FetchObserver", js_name = "onstatechange")]
    #[doc = "Setter for the `onstatechange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FetchObserver/onstatechange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FetchObserver`*"]
    pub fn set_onstatechange(this: &FetchObserver, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "FetchObserver",
        js_name = "onrequestprogress"
    )]
    #[doc = "Getter for the `onrequestprogress` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FetchObserver/onrequestprogress)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FetchObserver`*"]
    pub fn onrequestprogress(this: &FetchObserver) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "FetchObserver",
        js_name = "onrequestprogress"
    )]
    #[doc = "Setter for the `onrequestprogress` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FetchObserver/onrequestprogress)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FetchObserver`*"]
    pub fn set_onrequestprogress(this: &FetchObserver, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "FetchObserver",
        js_name = "onresponseprogress"
    )]
    #[doc = "Getter for the `onresponseprogress` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FetchObserver/onresponseprogress)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FetchObserver`*"]
    pub fn onresponseprogress(this: &FetchObserver) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "FetchObserver",
        js_name = "onresponseprogress"
    )]
    #[doc = "Setter for the `onresponseprogress` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FetchObserver/onresponseprogress)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FetchObserver`*"]
    pub fn set_onresponseprogress(this: &FetchObserver, value: Option<&::js_sys::Function>);
}
