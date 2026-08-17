#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "Event",
        extends = "::js_sys::Object",
        js_name = "SubmitEvent",
        typescript_type = "SubmitEvent"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SubmitEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubmitEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SubmitEvent`*"]
    pub type SubmitEvent;
    #[cfg(feature = "HtmlElement")]
    #[wasm_bindgen(method, getter, js_class = "SubmitEvent", js_name = "submitter")]
    #[doc = "Getter for the `submitter` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubmitEvent/submitter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlElement`, `SubmitEvent`*"]
    pub fn submitter(this: &SubmitEvent) -> Option<HtmlElement>;
    #[wasm_bindgen(catch, constructor, js_class = "SubmitEvent")]
    #[doc = "The `new SubmitEvent(..)` constructor, creating a new instance of `SubmitEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubmitEvent/SubmitEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SubmitEvent`*"]
    pub fn new(type_: &str) -> Result<SubmitEvent, JsValue>;
    #[cfg(feature = "SubmitEventInit")]
    #[wasm_bindgen(catch, constructor, js_class = "SubmitEvent")]
    #[doc = "The `new SubmitEvent(..)` constructor, creating a new instance of `SubmitEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubmitEvent/SubmitEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SubmitEvent`, `SubmitEventInit`*"]
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &SubmitEventInit,
    ) -> Result<SubmitEvent, JsValue>;
}
