#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "Event",
        extends = "::js_sys::Object",
        js_name = "TCPServerSocketEvent",
        typescript_type = "TCPServerSocketEvent"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `TcpServerSocketEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPServerSocketEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TcpServerSocketEvent`*"]
    pub type TcpServerSocketEvent;
    #[cfg(feature = "TcpSocket")]
    #[wasm_bindgen(method, getter, js_class = "TCPServerSocketEvent", js_name = "socket")]
    #[doc = "Getter for the `socket` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPServerSocketEvent/socket)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TcpServerSocketEvent`, `TcpSocket`*"]
    pub fn socket(this: &TcpServerSocketEvent) -> TcpSocket;
    #[wasm_bindgen(catch, constructor, js_class = "TCPServerSocketEvent")]
    #[doc = "The `new TcpServerSocketEvent(..)` constructor, creating a new instance of `TcpServerSocketEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPServerSocketEvent/TCPServerSocketEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TcpServerSocketEvent`*"]
    pub fn new(type_: &str) -> Result<TcpServerSocketEvent, JsValue>;
    #[cfg(feature = "TcpServerSocketEventInit")]
    #[wasm_bindgen(catch, constructor, js_class = "TCPServerSocketEvent")]
    #[doc = "The `new TcpServerSocketEvent(..)` constructor, creating a new instance of `TcpServerSocketEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPServerSocketEvent/TCPServerSocketEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TcpServerSocketEvent`, `TcpServerSocketEventInit`*"]
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &TcpServerSocketEventInit,
    ) -> Result<TcpServerSocketEvent, JsValue>;
}
