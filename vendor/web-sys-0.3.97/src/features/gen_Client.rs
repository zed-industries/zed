#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "Client",
        typescript_type = "Client"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Client` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Client)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Client`*"]
    pub type Client;
    #[wasm_bindgen(method, getter, js_class = "Client", js_name = "url")]
    #[doc = "Getter for the `url` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Client/url)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Client`*"]
    pub fn url(this: &Client) -> ::alloc::string::String;
    #[cfg(feature = "FrameType")]
    #[wasm_bindgen(method, getter, js_class = "Client", js_name = "frameType")]
    #[doc = "Getter for the `frameType` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Client/frameType)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Client`, `FrameType`*"]
    pub fn frame_type(this: &Client) -> FrameType;
    #[cfg(feature = "ClientType")]
    #[wasm_bindgen(method, getter, js_class = "Client", js_name = "type")]
    #[doc = "Getter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Client/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Client`, `ClientType`*"]
    pub fn type_(this: &Client) -> ClientType;
    #[wasm_bindgen(method, getter, js_class = "Client", js_name = "id")]
    #[doc = "Getter for the `id` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Client/id)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Client`*"]
    pub fn id(this: &Client) -> ::alloc::string::String;
    #[wasm_bindgen(catch, method, js_class = "Client", js_name = "postMessage")]
    #[doc = "The `postMessage()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Client/postMessage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Client`*"]
    pub fn post_message(this: &Client, message: &::wasm_bindgen::JsValue) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Client", js_name = "postMessage")]
    #[doc = "The `postMessage()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Client/postMessage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Client`*"]
    pub fn post_message_with_transfer(
        this: &Client,
        message: &::wasm_bindgen::JsValue,
        transfer: &::wasm_bindgen::JsValue,
    ) -> Result<(), JsValue>;
}
