#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "Event",
        extends = "::js_sys::Object",
        js_name = "TCPSocketErrorEvent",
        typescript_type = "TCPSocketErrorEvent"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `TcpSocketErrorEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocketErrorEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TcpSocketErrorEvent`*"]
    pub type TcpSocketErrorEvent;
    #[wasm_bindgen(method, getter, js_class = "TCPSocketErrorEvent", js_name = "name")]
    #[doc = "Getter for the `name` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocketErrorEvent/name)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TcpSocketErrorEvent`*"]
    pub fn name(this: &TcpSocketErrorEvent) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "TCPSocketErrorEvent", js_name = "message")]
    #[doc = "Getter for the `message` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocketErrorEvent/message)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TcpSocketErrorEvent`*"]
    pub fn message(this: &TcpSocketErrorEvent) -> ::alloc::string::String;
    #[wasm_bindgen(catch, constructor, js_class = "TCPSocketErrorEvent")]
    #[doc = "The `new TcpSocketErrorEvent(..)` constructor, creating a new instance of `TcpSocketErrorEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocketErrorEvent/TCPSocketErrorEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TcpSocketErrorEvent`*"]
    pub fn new(type_: &str) -> Result<TcpSocketErrorEvent, JsValue>;
    #[cfg(feature = "TcpSocketErrorEventInit")]
    #[wasm_bindgen(catch, constructor, js_class = "TCPSocketErrorEvent")]
    #[doc = "The `new TcpSocketErrorEvent(..)` constructor, creating a new instance of `TcpSocketErrorEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocketErrorEvent/TCPSocketErrorEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TcpSocketErrorEvent`, `TcpSocketErrorEventInit`*"]
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &TcpSocketErrorEventInit,
    ) -> Result<TcpSocketErrorEvent, JsValue>;
}
