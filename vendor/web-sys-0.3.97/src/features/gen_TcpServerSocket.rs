#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "EventTarget",
        extends = "::js_sys::Object",
        js_name = "TCPServerSocket",
        typescript_type = "TCPServerSocket"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `TcpServerSocket` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPServerSocket)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TcpServerSocket`*"]
    pub type TcpServerSocket;
    #[wasm_bindgen(method, getter, js_class = "TCPServerSocket", js_name = "localPort")]
    #[doc = "Getter for the `localPort` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPServerSocket/localPort)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TcpServerSocket`*"]
    pub fn local_port(this: &TcpServerSocket) -> u16;
    #[wasm_bindgen(method, getter, js_class = "TCPServerSocket", js_name = "onconnect")]
    #[doc = "Getter for the `onconnect` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPServerSocket/onconnect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TcpServerSocket`*"]
    pub fn onconnect(this: &TcpServerSocket) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "TCPServerSocket", js_name = "onconnect")]
    #[doc = "Setter for the `onconnect` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPServerSocket/onconnect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TcpServerSocket`*"]
    pub fn set_onconnect(this: &TcpServerSocket, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "TCPServerSocket", js_name = "onerror")]
    #[doc = "Getter for the `onerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPServerSocket/onerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TcpServerSocket`*"]
    pub fn onerror(this: &TcpServerSocket) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "TCPServerSocket", js_name = "onerror")]
    #[doc = "Setter for the `onerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPServerSocket/onerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TcpServerSocket`*"]
    pub fn set_onerror(this: &TcpServerSocket, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(catch, constructor, js_class = "TCPServerSocket")]
    #[doc = "The `new TcpServerSocket(..)` constructor, creating a new instance of `TcpServerSocket`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPServerSocket/TCPServerSocket)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TcpServerSocket`*"]
    pub fn new(port: u16) -> Result<TcpServerSocket, JsValue>;
    #[cfg(feature = "ServerSocketOptions")]
    #[wasm_bindgen(catch, constructor, js_class = "TCPServerSocket")]
    #[doc = "The `new TcpServerSocket(..)` constructor, creating a new instance of `TcpServerSocket`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPServerSocket/TCPServerSocket)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ServerSocketOptions`, `TcpServerSocket`*"]
    pub fn new_with_options(
        port: u16,
        options: &ServerSocketOptions,
    ) -> Result<TcpServerSocket, JsValue>;
    #[cfg(feature = "ServerSocketOptions")]
    #[wasm_bindgen(catch, constructor, js_class = "TCPServerSocket")]
    #[doc = "The `new TcpServerSocket(..)` constructor, creating a new instance of `TcpServerSocket`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPServerSocket/TCPServerSocket)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ServerSocketOptions`, `TcpServerSocket`*"]
    pub fn new_with_options_and_backlog(
        port: u16,
        options: &ServerSocketOptions,
        backlog: u16,
    ) -> Result<TcpServerSocket, JsValue>;
    #[wasm_bindgen(method, js_class = "TCPServerSocket")]
    #[doc = "The `close()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPServerSocket/close)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TcpServerSocket`*"]
    pub fn close(this: &TcpServerSocket);
}
