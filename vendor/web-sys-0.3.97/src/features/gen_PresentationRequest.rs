#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "EventTarget",
        extends = "::js_sys::Object",
        js_name = "PresentationRequest",
        typescript_type = "PresentationRequest"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PresentationRequest` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationRequest)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationRequest`*"]
    pub type PresentationRequest;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "PresentationRequest",
        js_name = "onconnectionavailable"
    )]
    #[doc = "Getter for the `onconnectionavailable` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationRequest/onconnectionavailable)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationRequest`*"]
    pub fn onconnectionavailable(this: &PresentationRequest) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "PresentationRequest",
        js_name = "onconnectionavailable"
    )]
    #[doc = "Setter for the `onconnectionavailable` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationRequest/onconnectionavailable)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationRequest`*"]
    pub fn set_onconnectionavailable(
        this: &PresentationRequest,
        value: Option<&::js_sys::Function>,
    );
    #[wasm_bindgen(catch, constructor, js_class = "PresentationRequest")]
    #[doc = "The `new PresentationRequest(..)` constructor, creating a new instance of `PresentationRequest`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationRequest/PresentationRequest)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationRequest`*"]
    pub fn new_with_url(url: &str) -> Result<PresentationRequest, JsValue>;
    #[wasm_bindgen(catch, constructor, js_class = "PresentationRequest")]
    #[doc = "The `new PresentationRequest(..)` constructor, creating a new instance of `PresentationRequest`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationRequest/PresentationRequest)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationRequest`*"]
    pub fn new_with_urls(urls: &::wasm_bindgen::JsValue) -> Result<PresentationRequest, JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "PresentationRequest",
        js_name = "getAvailability"
    )]
    #[doc = "The `getAvailability()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationRequest/getAvailability)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationRequest`*"]
    pub fn get_availability(this: &PresentationRequest) -> Result<::js_sys::Promise, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "PresentationRequest")]
    #[doc = "The `reconnect()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationRequest/reconnect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationRequest`*"]
    pub fn reconnect(
        this: &PresentationRequest,
        presentation_id: &str,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "PresentationRequest")]
    #[doc = "The `start()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationRequest/start)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationRequest`*"]
    pub fn start(this: &PresentationRequest) -> Result<::js_sys::Promise, JsValue>;
}
