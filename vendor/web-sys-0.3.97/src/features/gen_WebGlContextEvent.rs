#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "Event",
        extends = "::js_sys::Object",
        js_name = "WebGLContextEvent",
        typescript_type = "WebGLContextEvent"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WebGlContextEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLContextEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlContextEvent`*"]
    pub type WebGlContextEvent;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "WebGLContextEvent",
        js_name = "statusMessage"
    )]
    #[doc = "Getter for the `statusMessage` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLContextEvent/statusMessage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlContextEvent`*"]
    pub fn status_message(this: &WebGlContextEvent) -> ::alloc::string::String;
    #[wasm_bindgen(catch, constructor, js_class = "WebGLContextEvent")]
    #[doc = "The `new WebGlContextEvent(..)` constructor, creating a new instance of `WebGlContextEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLContextEvent/WebGLContextEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlContextEvent`*"]
    pub fn new(type_: &str) -> Result<WebGlContextEvent, JsValue>;
    #[cfg(feature = "WebGlContextEventInit")]
    #[wasm_bindgen(catch, constructor, js_class = "WebGLContextEvent")]
    #[doc = "The `new WebGlContextEvent(..)` constructor, creating a new instance of `WebGlContextEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLContextEvent/WebGLContextEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlContextEvent`, `WebGlContextEventInit`*"]
    pub fn new_with_event_init(
        type_: &str,
        event_init: &WebGlContextEventInit,
    ) -> Result<WebGlContextEvent, JsValue>;
}
