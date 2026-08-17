#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "EventTarget",
        extends = "::js_sys::Object",
        js_name = "EventSource",
        typescript_type = "EventSource"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `EventSource` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EventSource)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventSource`*"]
    pub type EventSource;
    #[wasm_bindgen(method, getter, js_class = "EventSource", js_name = "url")]
    #[doc = "Getter for the `url` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EventSource/url)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventSource`*"]
    pub fn url(this: &EventSource) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "EventSource", js_name = "withCredentials")]
    #[doc = "Getter for the `withCredentials` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EventSource/withCredentials)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventSource`*"]
    pub fn with_credentials(this: &EventSource) -> bool;
    #[wasm_bindgen(method, getter, js_class = "EventSource", js_name = "readyState")]
    #[doc = "Getter for the `readyState` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EventSource/readyState)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventSource`*"]
    pub fn ready_state(this: &EventSource) -> u16;
    #[wasm_bindgen(method, getter, js_class = "EventSource", js_name = "onopen")]
    #[doc = "Getter for the `onopen` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EventSource/onopen)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventSource`*"]
    pub fn onopen(this: &EventSource) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "EventSource", js_name = "onopen")]
    #[doc = "Setter for the `onopen` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EventSource/onopen)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventSource`*"]
    pub fn set_onopen(this: &EventSource, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "EventSource", js_name = "onmessage")]
    #[doc = "Getter for the `onmessage` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EventSource/onmessage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventSource`*"]
    pub fn onmessage(this: &EventSource) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "EventSource", js_name = "onmessage")]
    #[doc = "Setter for the `onmessage` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EventSource/onmessage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventSource`*"]
    pub fn set_onmessage(this: &EventSource, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "EventSource", js_name = "onerror")]
    #[doc = "Getter for the `onerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EventSource/onerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventSource`*"]
    pub fn onerror(this: &EventSource) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "EventSource", js_name = "onerror")]
    #[doc = "Setter for the `onerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EventSource/onerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventSource`*"]
    pub fn set_onerror(this: &EventSource, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(catch, constructor, js_class = "EventSource")]
    #[doc = "The `new EventSource(..)` constructor, creating a new instance of `EventSource`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EventSource/EventSource)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventSource`*"]
    pub fn new(url: &str) -> Result<EventSource, JsValue>;
    #[cfg(feature = "EventSourceInit")]
    #[wasm_bindgen(catch, constructor, js_class = "EventSource")]
    #[doc = "The `new EventSource(..)` constructor, creating a new instance of `EventSource`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EventSource/EventSource)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventSource`, `EventSourceInit`*"]
    pub fn new_with_event_source_init_dict(
        url: &str,
        event_source_init_dict: &EventSourceInit,
    ) -> Result<EventSource, JsValue>;
    #[wasm_bindgen(method, js_class = "EventSource")]
    #[doc = "The `close()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EventSource/close)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventSource`*"]
    pub fn close(this: &EventSource);
}
impl EventSource {
    #[doc = "The `EventSource.CONNECTING` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventSource`*"]
    pub const CONNECTING: u16 = 0i64 as u16;
    #[doc = "The `EventSource.OPEN` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventSource`*"]
    pub const OPEN: u16 = 1u64 as u16;
    #[doc = "The `EventSource.CLOSED` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventSource`*"]
    pub const CLOSED: u16 = 2u64 as u16;
}
