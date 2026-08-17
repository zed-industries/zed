#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "Event",
        extends = "::js_sys::Object",
        js_name = "TCPSocketEvent",
        typescript_type = "TCPSocketEvent"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `TcpSocketEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocketEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TcpSocketEvent`*"]
    pub type TcpSocketEvent;
    #[wasm_bindgen(method, getter, js_class = "TCPSocketEvent", js_name = "data")]
    #[doc = "Getter for the `data` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocketEvent/data)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TcpSocketEvent`*"]
    pub fn data(this: &TcpSocketEvent) -> ::wasm_bindgen::JsValue;
    #[wasm_bindgen(catch, constructor, js_class = "TCPSocketEvent")]
    #[doc = "The `new TcpSocketEvent(..)` constructor, creating a new instance of `TcpSocketEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocketEvent/TCPSocketEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TcpSocketEvent`*"]
    pub fn new(type_: &str) -> Result<TcpSocketEvent, JsValue>;
    #[cfg(feature = "TcpSocketEventInit")]
    #[wasm_bindgen(catch, constructor, js_class = "TCPSocketEvent")]
    #[doc = "The `new TcpSocketEvent(..)` constructor, creating a new instance of `TcpSocketEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocketEvent/TCPSocketEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TcpSocketEvent`, `TcpSocketEventInit`*"]
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &TcpSocketEventInit,
    ) -> Result<TcpSocketEvent, JsValue>;
}
