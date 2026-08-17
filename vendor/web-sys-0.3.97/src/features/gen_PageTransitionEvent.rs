#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "Event",
        extends = "::js_sys::Object",
        js_name = "PageTransitionEvent",
        typescript_type = "PageTransitionEvent"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PageTransitionEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PageTransitionEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PageTransitionEvent`*"]
    pub type PageTransitionEvent;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "PageTransitionEvent",
        js_name = "persisted"
    )]
    #[doc = "Getter for the `persisted` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PageTransitionEvent/persisted)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PageTransitionEvent`*"]
    pub fn persisted(this: &PageTransitionEvent) -> bool;
    #[wasm_bindgen(catch, constructor, js_class = "PageTransitionEvent")]
    #[doc = "The `new PageTransitionEvent(..)` constructor, creating a new instance of `PageTransitionEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PageTransitionEvent/PageTransitionEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PageTransitionEvent`*"]
    pub fn new(type_: &str) -> Result<PageTransitionEvent, JsValue>;
    #[cfg(feature = "PageTransitionEventInit")]
    #[wasm_bindgen(catch, constructor, js_class = "PageTransitionEvent")]
    #[doc = "The `new PageTransitionEvent(..)` constructor, creating a new instance of `PageTransitionEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PageTransitionEvent/PageTransitionEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PageTransitionEvent`, `PageTransitionEventInit`*"]
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &PageTransitionEventInit,
    ) -> Result<PageTransitionEvent, JsValue>;
}
