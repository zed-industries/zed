#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "EventTarget",
        extends = "::js_sys::Object",
        js_name = "TCPSocket",
        typescript_type = "TCPSocket"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `TcpSocket` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TcpSocket`*"]
    pub type TcpSocket;
    #[wasm_bindgen(method, getter, js_class = "TCPSocket", js_name = "host")]
    #[doc = "Getter for the `host` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/host)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TcpSocket`*"]
    pub fn host(this: &TcpSocket) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "TCPSocket", js_name = "port")]
    #[doc = "Getter for the `port` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/port)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TcpSocket`*"]
    pub fn port(this: &TcpSocket) -> u16;
    #[wasm_bindgen(method, getter, js_class = "TCPSocket", js_name = "ssl")]
    #[doc = "Getter for the `ssl` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/ssl)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TcpSocket`*"]
    pub fn ssl(this: &TcpSocket) -> bool;
    #[wasm_bindgen(method, getter, js_class = "TCPSocket", js_name = "bufferedAmount")]
    #[doc = "Getter for the `bufferedAmount` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/bufferedAmount)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TcpSocket`*"]
    pub fn buffered_amount(this: &TcpSocket) -> f64;
    #[cfg(feature = "TcpReadyState")]
    #[wasm_bindgen(method, getter, js_class = "TCPSocket", js_name = "readyState")]
    #[doc = "Getter for the `readyState` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/readyState)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TcpReadyState`, `TcpSocket`*"]
    pub fn ready_state(this: &TcpSocket) -> TcpReadyState;
    #[cfg(feature = "TcpSocketBinaryType")]
    #[wasm_bindgen(method, getter, js_class = "TCPSocket", js_name = "binaryType")]
    #[doc = "Getter for the `binaryType` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/binaryType)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TcpSocket`, `TcpSocketBinaryType`*"]
    pub fn binary_type(this: &TcpSocket) -> TcpSocketBinaryType;
    #[wasm_bindgen(method, getter, js_class = "TCPSocket", js_name = "onopen")]
    #[doc = "Getter for the `onopen` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/onopen)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TcpSocket`*"]
    pub fn onopen(this: &TcpSocket) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "TCPSocket", js_name = "onopen")]
    #[doc = "Setter for the `onopen` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/onopen)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TcpSocket`*"]
    pub fn set_onopen(this: &TcpSocket, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "TCPSocket", js_name = "ondrain")]
    #[doc = "Getter for the `ondrain` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/ondrain)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TcpSocket`*"]
    pub fn ondrain(this: &TcpSocket) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "TCPSocket", js_name = "ondrain")]
    #[doc = "Setter for the `ondrain` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/ondrain)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TcpSocket`*"]
    pub fn set_ondrain(this: &TcpSocket, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "TCPSocket", js_name = "ondata")]
    #[doc = "Getter for the `ondata` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/ondata)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TcpSocket`*"]
    pub fn ondata(this: &TcpSocket) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "TCPSocket", js_name = "ondata")]
    #[doc = "Setter for the `ondata` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/ondata)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TcpSocket`*"]
    pub fn set_ondata(this: &TcpSocket, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "TCPSocket", js_name = "onerror")]
    #[doc = "Getter for the `onerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/onerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TcpSocket`*"]
    pub fn onerror(this: &TcpSocket) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "TCPSocket", js_name = "onerror")]
    #[doc = "Setter for the `onerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/onerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TcpSocket`*"]
    pub fn set_onerror(this: &TcpSocket, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "TCPSocket", js_name = "onclose")]
    #[doc = "Getter for the `onclose` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/onclose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TcpSocket`*"]
    pub fn onclose(this: &TcpSocket) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "TCPSocket", js_name = "onclose")]
    #[doc = "Setter for the `onclose` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/onclose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TcpSocket`*"]
    pub fn set_onclose(this: &TcpSocket, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(catch, constructor, js_class = "TCPSocket")]
    #[doc = "The `new TcpSocket(..)` constructor, creating a new instance of `TcpSocket`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/TCPSocket)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TcpSocket`*"]
    pub fn new(host: &str, port: u16) -> Result<TcpSocket, JsValue>;
    #[cfg(feature = "SocketOptions")]
    #[wasm_bindgen(catch, constructor, js_class = "TCPSocket")]
    #[doc = "The `new TcpSocket(..)` constructor, creating a new instance of `TcpSocket`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/TCPSocket)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketOptions`, `TcpSocket`*"]
    pub fn new_with_options(
        host: &str,
        port: u16,
        options: &SocketOptions,
    ) -> Result<TcpSocket, JsValue>;
    #[wasm_bindgen(method, js_class = "TCPSocket")]
    #[doc = "The `close()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/close)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TcpSocket`*"]
    pub fn close(this: &TcpSocket);
    #[wasm_bindgen(catch, method, js_class = "TCPSocket")]
    #[doc = "The `resume()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/resume)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TcpSocket`*"]
    pub fn resume(this: &TcpSocket) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "TCPSocket", js_name = "send")]
    #[doc = "The `send()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/send)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TcpSocket`*"]
    pub fn send_with_str(this: &TcpSocket, data: &str) -> Result<bool, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "TCPSocket", js_name = "send")]
    #[doc = "The `send()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/send)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TcpSocket`*"]
    pub fn send_with_array_buffer(
        this: &TcpSocket,
        data: &::js_sys::ArrayBuffer,
    ) -> Result<bool, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "TCPSocket", js_name = "send")]
    #[doc = "The `send()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/send)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TcpSocket`*"]
    pub fn send_with_array_buffer_and_byte_offset(
        this: &TcpSocket,
        data: &::js_sys::ArrayBuffer,
        byte_offset: u32,
    ) -> Result<bool, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "TCPSocket", js_name = "send")]
    #[doc = "The `send()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/send)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TcpSocket`*"]
    pub fn send_with_array_buffer_and_byte_offset_and_byte_length(
        this: &TcpSocket,
        data: &::js_sys::ArrayBuffer,
        byte_offset: u32,
        byte_length: u32,
    ) -> Result<bool, JsValue>;
    #[wasm_bindgen(method, js_class = "TCPSocket")]
    #[doc = "The `suspend()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/suspend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TcpSocket`*"]
    pub fn suspend(this: &TcpSocket);
    #[wasm_bindgen(catch, method, js_class = "TCPSocket", js_name = "upgradeToSecure")]
    #[doc = "The `upgradeToSecure()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/upgradeToSecure)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TcpSocket`*"]
    pub fn upgrade_to_secure(this: &TcpSocket) -> Result<(), JsValue>;
}
