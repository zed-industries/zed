#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "EventTarget",
        extends = "::js_sys::Object",
        js_name = "WebSocket",
        typescript_type = "WebSocket"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WebSocket` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebSocket`*"]
    pub type WebSocket;
    #[wasm_bindgen(method, getter, js_class = "WebSocket", js_name = "url")]
    #[doc = "Getter for the `url` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/url)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebSocket`*"]
    pub fn url(this: &WebSocket) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "WebSocket", js_name = "readyState")]
    #[doc = "Getter for the `readyState` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/readyState)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebSocket`*"]
    pub fn ready_state(this: &WebSocket) -> u16;
    #[wasm_bindgen(method, getter, js_class = "WebSocket", js_name = "bufferedAmount")]
    #[doc = "Getter for the `bufferedAmount` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/bufferedAmount)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebSocket`*"]
    pub fn buffered_amount(this: &WebSocket) -> u32;
    #[wasm_bindgen(method, getter, js_class = "WebSocket", js_name = "onopen")]
    #[doc = "Getter for the `onopen` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/onopen)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebSocket`*"]
    pub fn onopen(this: &WebSocket) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "WebSocket", js_name = "onopen")]
    #[doc = "Setter for the `onopen` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/onopen)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebSocket`*"]
    pub fn set_onopen(this: &WebSocket, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "WebSocket", js_name = "onerror")]
    #[doc = "Getter for the `onerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/onerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebSocket`*"]
    pub fn onerror(this: &WebSocket) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "WebSocket", js_name = "onerror")]
    #[doc = "Setter for the `onerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/onerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebSocket`*"]
    pub fn set_onerror(this: &WebSocket, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "WebSocket", js_name = "onclose")]
    #[doc = "Getter for the `onclose` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/onclose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebSocket`*"]
    pub fn onclose(this: &WebSocket) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "WebSocket", js_name = "onclose")]
    #[doc = "Setter for the `onclose` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/onclose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebSocket`*"]
    pub fn set_onclose(this: &WebSocket, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "WebSocket", js_name = "extensions")]
    #[doc = "Getter for the `extensions` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/extensions)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebSocket`*"]
    pub fn extensions(this: &WebSocket) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "WebSocket", js_name = "protocol")]
    #[doc = "Getter for the `protocol` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/protocol)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebSocket`*"]
    pub fn protocol(this: &WebSocket) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "WebSocket", js_name = "onmessage")]
    #[doc = "Getter for the `onmessage` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/onmessage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebSocket`*"]
    pub fn onmessage(this: &WebSocket) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "WebSocket", js_name = "onmessage")]
    #[doc = "Setter for the `onmessage` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/onmessage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebSocket`*"]
    pub fn set_onmessage(this: &WebSocket, value: Option<&::js_sys::Function>);
    #[cfg(feature = "BinaryType")]
    #[wasm_bindgen(method, getter, js_class = "WebSocket", js_name = "binaryType")]
    #[doc = "Getter for the `binaryType` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/binaryType)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BinaryType`, `WebSocket`*"]
    pub fn binary_type(this: &WebSocket) -> BinaryType;
    #[cfg(feature = "BinaryType")]
    #[wasm_bindgen(method, setter, js_class = "WebSocket", js_name = "binaryType")]
    #[doc = "Setter for the `binaryType` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/binaryType)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BinaryType`, `WebSocket`*"]
    pub fn set_binary_type(this: &WebSocket, value: BinaryType);
    #[wasm_bindgen(catch, constructor, js_class = "WebSocket")]
    #[doc = "The `new WebSocket(..)` constructor, creating a new instance of `WebSocket`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/WebSocket)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebSocket`*"]
    pub fn new(url: &str) -> Result<WebSocket, JsValue>;
    #[wasm_bindgen(catch, constructor, js_class = "WebSocket")]
    #[doc = "The `new WebSocket(..)` constructor, creating a new instance of `WebSocket`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/WebSocket)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebSocket`*"]
    pub fn new_with_str(url: &str, protocols: &str) -> Result<WebSocket, JsValue>;
    #[wasm_bindgen(catch, constructor, js_class = "WebSocket")]
    #[doc = "The `new WebSocket(..)` constructor, creating a new instance of `WebSocket`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/WebSocket)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebSocket`*"]
    pub fn new_with_str_sequence(
        url: &str,
        protocols: &::wasm_bindgen::JsValue,
    ) -> Result<WebSocket, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "WebSocket")]
    #[doc = "The `close()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/close)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebSocket`*"]
    pub fn close(this: &WebSocket) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "WebSocket", js_name = "close")]
    #[doc = "The `close()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/close)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebSocket`*"]
    pub fn close_with_code(this: &WebSocket, code: u16) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "WebSocket", js_name = "close")]
    #[doc = "The `close()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/close)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebSocket`*"]
    pub fn close_with_code_and_reason(
        this: &WebSocket,
        code: u16,
        reason: &str,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "WebSocket", js_name = "send")]
    #[doc = "The `send()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/send)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebSocket`*"]
    pub fn send_with_str(this: &WebSocket, data: &str) -> Result<(), JsValue>;
    #[cfg(feature = "Blob")]
    #[wasm_bindgen(catch, method, js_class = "WebSocket", js_name = "send")]
    #[doc = "The `send()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/send)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Blob`, `WebSocket`*"]
    pub fn send_with_blob(this: &WebSocket, data: &Blob) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "WebSocket", js_name = "send")]
    #[doc = "The `send()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/send)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebSocket`*"]
    pub fn send_with_array_buffer(
        this: &WebSocket,
        data: &::js_sys::ArrayBuffer,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "WebSocket", js_name = "send")]
    #[doc = "The `send()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/send)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebSocket`*"]
    pub fn send_with_array_buffer_view(
        this: &WebSocket,
        data: &::js_sys::Object,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "WebSocket", js_name = "send")]
    #[doc = "The `send()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/send)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebSocket`*"]
    pub fn send_with_u8_array(this: &WebSocket, data: &[u8]) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "WebSocket", js_name = "send")]
    #[doc = "The `send()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/send)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebSocket`*"]
    pub fn send_with_js_u8_array(
        this: &WebSocket,
        data: &::js_sys::Uint8Array,
    ) -> Result<(), JsValue>;
}
impl WebSocket {
    #[doc = "The `WebSocket.CONNECTING` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebSocket`*"]
    pub const CONNECTING: u16 = 0i64 as u16;
    #[doc = "The `WebSocket.OPEN` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebSocket`*"]
    pub const OPEN: u16 = 1u64 as u16;
    #[doc = "The `WebSocket.CLOSING` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebSocket`*"]
    pub const CLOSING: u16 = 2u64 as u16;
    #[doc = "The `WebSocket.CLOSED` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebSocket`*"]
    pub const CLOSED: u16 = 3u64 as u16;
}
