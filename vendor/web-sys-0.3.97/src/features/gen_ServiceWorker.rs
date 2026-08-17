#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "EventTarget",
        extends = "::js_sys::Object",
        js_name = "ServiceWorker",
        typescript_type = "ServiceWorker"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ServiceWorker` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorker)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ServiceWorker`*"]
    pub type ServiceWorker;
    #[wasm_bindgen(method, getter, js_class = "ServiceWorker", js_name = "scriptURL")]
    #[doc = "Getter for the `scriptURL` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorker/scriptURL)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ServiceWorker`*"]
    pub fn script_url(this: &ServiceWorker) -> ::alloc::string::String;
    #[cfg(feature = "ServiceWorkerState")]
    #[wasm_bindgen(method, getter, js_class = "ServiceWorker", js_name = "state")]
    #[doc = "Getter for the `state` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorker/state)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ServiceWorker`, `ServiceWorkerState`*"]
    pub fn state(this: &ServiceWorker) -> ServiceWorkerState;
    #[wasm_bindgen(method, getter, js_class = "ServiceWorker", js_name = "onstatechange")]
    #[doc = "Getter for the `onstatechange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorker/onstatechange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ServiceWorker`*"]
    pub fn onstatechange(this: &ServiceWorker) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "ServiceWorker", js_name = "onstatechange")]
    #[doc = "Setter for the `onstatechange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorker/onstatechange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ServiceWorker`*"]
    pub fn set_onstatechange(this: &ServiceWorker, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "ServiceWorker", js_name = "onerror")]
    #[doc = "Getter for the `onerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorker/onerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ServiceWorker`*"]
    pub fn onerror(this: &ServiceWorker) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "ServiceWorker", js_name = "onerror")]
    #[doc = "Setter for the `onerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorker/onerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ServiceWorker`*"]
    pub fn set_onerror(this: &ServiceWorker, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(catch, method, js_class = "ServiceWorker", js_name = "postMessage")]
    #[doc = "The `postMessage()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorker/postMessage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ServiceWorker`*"]
    pub fn post_message(
        this: &ServiceWorker,
        message: &::wasm_bindgen::JsValue,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "ServiceWorker", js_name = "postMessage")]
    #[doc = "The `postMessage()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorker/postMessage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ServiceWorker`*"]
    pub fn post_message_with_transferable(
        this: &ServiceWorker,
        message: &::wasm_bindgen::JsValue,
        transferable: &::wasm_bindgen::JsValue,
    ) -> Result<(), JsValue>;
}
