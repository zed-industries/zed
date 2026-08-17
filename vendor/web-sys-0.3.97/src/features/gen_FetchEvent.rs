#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "ExtendableEvent",
        extends = "Event",
        extends = "::js_sys::Object",
        js_name = "FetchEvent",
        typescript_type = "FetchEvent"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `FetchEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FetchEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FetchEvent`*"]
    pub type FetchEvent;
    #[cfg(feature = "Request")]
    #[wasm_bindgen(method, getter, js_class = "FetchEvent", js_name = "request")]
    #[doc = "Getter for the `request` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FetchEvent/request)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FetchEvent`, `Request`*"]
    pub fn request(this: &FetchEvent) -> Request;
    #[wasm_bindgen(method, getter, js_class = "FetchEvent", js_name = "clientId")]
    #[doc = "Getter for the `clientId` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FetchEvent/clientId)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FetchEvent`*"]
    pub fn client_id(this: &FetchEvent) -> Option<::alloc::string::String>;
    #[wasm_bindgen(method, getter, js_class = "FetchEvent", js_name = "isReload")]
    #[doc = "Getter for the `isReload` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FetchEvent/isReload)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FetchEvent`*"]
    pub fn is_reload(this: &FetchEvent) -> bool;
    #[cfg(feature = "FetchEventInit")]
    #[wasm_bindgen(catch, constructor, js_class = "FetchEvent")]
    #[doc = "The `new FetchEvent(..)` constructor, creating a new instance of `FetchEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FetchEvent/FetchEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FetchEvent`, `FetchEventInit`*"]
    pub fn new(type_: &str, event_init_dict: &FetchEventInit) -> Result<FetchEvent, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "FetchEvent", js_name = "respondWith")]
    #[doc = "The `respondWith()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FetchEvent/respondWith)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FetchEvent`*"]
    pub fn respond_with(this: &FetchEvent, r: &::js_sys::Promise) -> Result<(), JsValue>;
}
