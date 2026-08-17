#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "Event",
        extends = "::js_sys::Object",
        js_name = "PopStateEvent",
        typescript_type = "PopStateEvent"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PopStateEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PopStateEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PopStateEvent`*"]
    pub type PopStateEvent;
    #[wasm_bindgen(method, getter, js_class = "PopStateEvent", js_name = "state")]
    #[doc = "Getter for the `state` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PopStateEvent/state)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PopStateEvent`*"]
    pub fn state(this: &PopStateEvent) -> ::wasm_bindgen::JsValue;
    #[wasm_bindgen(catch, constructor, js_class = "PopStateEvent")]
    #[doc = "The `new PopStateEvent(..)` constructor, creating a new instance of `PopStateEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PopStateEvent/PopStateEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PopStateEvent`*"]
    pub fn new(type_: &str) -> Result<PopStateEvent, JsValue>;
    #[cfg(feature = "PopStateEventInit")]
    #[wasm_bindgen(catch, constructor, js_class = "PopStateEvent")]
    #[doc = "The `new PopStateEvent(..)` constructor, creating a new instance of `PopStateEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PopStateEvent/PopStateEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PopStateEvent`, `PopStateEventInit`*"]
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &PopStateEventInit,
    ) -> Result<PopStateEvent, JsValue>;
}
