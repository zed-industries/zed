#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "EventTarget",
        extends = "::js_sys::Object",
        js_name = "PresentationConnection",
        typescript_type = "PresentationConnection"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PresentationConnection` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnection)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnection`*"]
    pub type PresentationConnection;
    #[wasm_bindgen(method, getter, js_class = "PresentationConnection", js_name = "id")]
    #[doc = "Getter for the `id` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnection/id)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnection`*"]
    pub fn id(this: &PresentationConnection) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "PresentationConnection", js_name = "url")]
    #[doc = "Getter for the `url` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnection/url)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnection`*"]
    pub fn url(this: &PresentationConnection) -> ::alloc::string::String;
    #[cfg(feature = "PresentationConnectionState")]
    #[wasm_bindgen(method, getter, js_class = "PresentationConnection", js_name = "state")]
    #[doc = "Getter for the `state` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnection/state)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnection`, `PresentationConnectionState`*"]
    pub fn state(this: &PresentationConnection) -> PresentationConnectionState;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "PresentationConnection",
        js_name = "onconnect"
    )]
    #[doc = "Getter for the `onconnect` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnection/onconnect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnection`*"]
    pub fn onconnect(this: &PresentationConnection) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "PresentationConnection",
        js_name = "onconnect"
    )]
    #[doc = "Setter for the `onconnect` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnection/onconnect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnection`*"]
    pub fn set_onconnect(this: &PresentationConnection, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "PresentationConnection",
        js_name = "onclose"
    )]
    #[doc = "Getter for the `onclose` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnection/onclose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnection`*"]
    pub fn onclose(this: &PresentationConnection) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "PresentationConnection",
        js_name = "onclose"
    )]
    #[doc = "Setter for the `onclose` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnection/onclose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnection`*"]
    pub fn set_onclose(this: &PresentationConnection, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "PresentationConnection",
        js_name = "onterminate"
    )]
    #[doc = "Getter for the `onterminate` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnection/onterminate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnection`*"]
    pub fn onterminate(this: &PresentationConnection) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "PresentationConnection",
        js_name = "onterminate"
    )]
    #[doc = "Setter for the `onterminate` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnection/onterminate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnection`*"]
    pub fn set_onterminate(this: &PresentationConnection, value: Option<&::js_sys::Function>);
    #[cfg(feature = "PresentationConnectionBinaryType")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "PresentationConnection",
        js_name = "binaryType"
    )]
    #[doc = "Getter for the `binaryType` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnection/binaryType)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnection`, `PresentationConnectionBinaryType`*"]
    pub fn binary_type(this: &PresentationConnection) -> PresentationConnectionBinaryType;
    #[cfg(feature = "PresentationConnectionBinaryType")]
    #[wasm_bindgen(
        method,
        setter,
        js_class = "PresentationConnection",
        js_name = "binaryType"
    )]
    #[doc = "Setter for the `binaryType` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnection/binaryType)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnection`, `PresentationConnectionBinaryType`*"]
    pub fn set_binary_type(this: &PresentationConnection, value: PresentationConnectionBinaryType);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "PresentationConnection",
        js_name = "onmessage"
    )]
    #[doc = "Getter for the `onmessage` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnection/onmessage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnection`*"]
    pub fn onmessage(this: &PresentationConnection) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "PresentationConnection",
        js_name = "onmessage"
    )]
    #[doc = "Setter for the `onmessage` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnection/onmessage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnection`*"]
    pub fn set_onmessage(this: &PresentationConnection, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(catch, method, js_class = "PresentationConnection")]
    #[doc = "The `close()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnection/close)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnection`*"]
    pub fn close(this: &PresentationConnection) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "PresentationConnection", js_name = "send")]
    #[doc = "The `send()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnection/send)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnection`*"]
    pub fn send_with_str(this: &PresentationConnection, data: &str) -> Result<(), JsValue>;
    #[cfg(feature = "Blob")]
    #[wasm_bindgen(catch, method, js_class = "PresentationConnection", js_name = "send")]
    #[doc = "The `send()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnection/send)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Blob`, `PresentationConnection`*"]
    pub fn send_with_blob(this: &PresentationConnection, data: &Blob) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "PresentationConnection", js_name = "send")]
    #[doc = "The `send()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnection/send)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnection`*"]
    pub fn send_with_array_buffer(
        this: &PresentationConnection,
        data: &::js_sys::ArrayBuffer,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "PresentationConnection", js_name = "send")]
    #[doc = "The `send()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnection/send)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnection`*"]
    pub fn send_with_array_buffer_view(
        this: &PresentationConnection,
        data: &::js_sys::Object,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "PresentationConnection", js_name = "send")]
    #[doc = "The `send()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnection/send)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnection`*"]
    pub fn send_with_u8_array(this: &PresentationConnection, data: &[u8]) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "PresentationConnection", js_name = "send")]
    #[doc = "The `send()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnection/send)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnection`*"]
    pub fn send_with_js_u8_array(
        this: &PresentationConnection,
        data: &::js_sys::Uint8Array,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "PresentationConnection")]
    #[doc = "The `terminate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnection/terminate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnection`*"]
    pub fn terminate(this: &PresentationConnection) -> Result<(), JsValue>;
}
