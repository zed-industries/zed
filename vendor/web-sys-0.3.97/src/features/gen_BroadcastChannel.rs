#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "EventTarget",
        extends = "::js_sys::Object",
        js_name = "BroadcastChannel",
        typescript_type = "BroadcastChannel"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `BroadcastChannel` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BroadcastChannel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BroadcastChannel`*"]
    pub type BroadcastChannel;
    #[wasm_bindgen(method, getter, js_class = "BroadcastChannel", js_name = "name")]
    #[doc = "Getter for the `name` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BroadcastChannel/name)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BroadcastChannel`*"]
    pub fn name(this: &BroadcastChannel) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "BroadcastChannel", js_name = "onmessage")]
    #[doc = "Getter for the `onmessage` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BroadcastChannel/onmessage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BroadcastChannel`*"]
    pub fn onmessage(this: &BroadcastChannel) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "BroadcastChannel", js_name = "onmessage")]
    #[doc = "Setter for the `onmessage` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BroadcastChannel/onmessage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BroadcastChannel`*"]
    pub fn set_onmessage(this: &BroadcastChannel, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "BroadcastChannel",
        js_name = "onmessageerror"
    )]
    #[doc = "Getter for the `onmessageerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BroadcastChannel/onmessageerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BroadcastChannel`*"]
    pub fn onmessageerror(this: &BroadcastChannel) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "BroadcastChannel",
        js_name = "onmessageerror"
    )]
    #[doc = "Setter for the `onmessageerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BroadcastChannel/onmessageerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BroadcastChannel`*"]
    pub fn set_onmessageerror(this: &BroadcastChannel, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(catch, constructor, js_class = "BroadcastChannel")]
    #[doc = "The `new BroadcastChannel(..)` constructor, creating a new instance of `BroadcastChannel`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BroadcastChannel/BroadcastChannel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BroadcastChannel`*"]
    pub fn new(channel: &str) -> Result<BroadcastChannel, JsValue>;
    #[wasm_bindgen(method, js_class = "BroadcastChannel")]
    #[doc = "The `close()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BroadcastChannel/close)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BroadcastChannel`*"]
    pub fn close(this: &BroadcastChannel);
    #[wasm_bindgen(catch, method, js_class = "BroadcastChannel", js_name = "postMessage")]
    #[doc = "The `postMessage()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BroadcastChannel/postMessage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BroadcastChannel`*"]
    pub fn post_message(
        this: &BroadcastChannel,
        message: &::wasm_bindgen::JsValue,
    ) -> Result<(), JsValue>;
}
