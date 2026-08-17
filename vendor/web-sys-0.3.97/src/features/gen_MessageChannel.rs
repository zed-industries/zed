#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "MessageChannel",
        typescript_type = "MessageChannel"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MessageChannel` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessageChannel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessageChannel`*"]
    pub type MessageChannel;
    #[cfg(feature = "MessagePort")]
    #[wasm_bindgen(method, getter, js_class = "MessageChannel", js_name = "port1")]
    #[doc = "Getter for the `port1` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessageChannel/port1)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessageChannel`, `MessagePort`*"]
    pub fn port1(this: &MessageChannel) -> MessagePort;
    #[cfg(feature = "MessagePort")]
    #[wasm_bindgen(method, getter, js_class = "MessageChannel", js_name = "port2")]
    #[doc = "Getter for the `port2` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessageChannel/port2)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessageChannel`, `MessagePort`*"]
    pub fn port2(this: &MessageChannel) -> MessagePort;
    #[wasm_bindgen(catch, constructor, js_class = "MessageChannel")]
    #[doc = "The `new MessageChannel(..)` constructor, creating a new instance of `MessageChannel`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessageChannel/MessageChannel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessageChannel`*"]
    pub fn new() -> Result<MessageChannel, JsValue>;
}
