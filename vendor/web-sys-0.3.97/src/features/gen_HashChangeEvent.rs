#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "Event",
        extends = "::js_sys::Object",
        js_name = "HashChangeEvent",
        typescript_type = "HashChangeEvent"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HashChangeEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HashChangeEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HashChangeEvent`*"]
    pub type HashChangeEvent;
    #[wasm_bindgen(method, getter, js_class = "HashChangeEvent", js_name = "oldURL")]
    #[doc = "Getter for the `oldURL` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HashChangeEvent/oldURL)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HashChangeEvent`*"]
    pub fn old_url(this: &HashChangeEvent) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "HashChangeEvent", js_name = "newURL")]
    #[doc = "Getter for the `newURL` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HashChangeEvent/newURL)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HashChangeEvent`*"]
    pub fn new_url(this: &HashChangeEvent) -> ::alloc::string::String;
    #[wasm_bindgen(catch, constructor, js_class = "HashChangeEvent")]
    #[doc = "The `new HashChangeEvent(..)` constructor, creating a new instance of `HashChangeEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HashChangeEvent/HashChangeEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HashChangeEvent`*"]
    pub fn new(type_: &str) -> Result<HashChangeEvent, JsValue>;
    #[cfg(feature = "HashChangeEventInit")]
    #[wasm_bindgen(catch, constructor, js_class = "HashChangeEvent")]
    #[doc = "The `new HashChangeEvent(..)` constructor, creating a new instance of `HashChangeEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HashChangeEvent/HashChangeEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HashChangeEvent`, `HashChangeEventInit`*"]
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &HashChangeEventInit,
    ) -> Result<HashChangeEvent, JsValue>;
    #[wasm_bindgen(method, js_class = "HashChangeEvent", js_name = "initHashChangeEvent")]
    #[doc = "The `initHashChangeEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HashChangeEvent/initHashChangeEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HashChangeEvent`*"]
    pub fn init_hash_change_event(this: &HashChangeEvent, type_arg: &str);
    #[wasm_bindgen(method, js_class = "HashChangeEvent", js_name = "initHashChangeEvent")]
    #[doc = "The `initHashChangeEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HashChangeEvent/initHashChangeEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HashChangeEvent`*"]
    pub fn init_hash_change_event_with_can_bubble_arg(
        this: &HashChangeEvent,
        type_arg: &str,
        can_bubble_arg: bool,
    );
    #[wasm_bindgen(method, js_class = "HashChangeEvent", js_name = "initHashChangeEvent")]
    #[doc = "The `initHashChangeEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HashChangeEvent/initHashChangeEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HashChangeEvent`*"]
    pub fn init_hash_change_event_with_can_bubble_arg_and_cancelable_arg(
        this: &HashChangeEvent,
        type_arg: &str,
        can_bubble_arg: bool,
        cancelable_arg: bool,
    );
    #[wasm_bindgen(method, js_class = "HashChangeEvent", js_name = "initHashChangeEvent")]
    #[doc = "The `initHashChangeEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HashChangeEvent/initHashChangeEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HashChangeEvent`*"]
    pub fn init_hash_change_event_with_can_bubble_arg_and_cancelable_arg_and_old_url_arg(
        this: &HashChangeEvent,
        type_arg: &str,
        can_bubble_arg: bool,
        cancelable_arg: bool,
        old_url_arg: &str,
    );
    #[wasm_bindgen(method, js_class = "HashChangeEvent", js_name = "initHashChangeEvent")]
    #[doc = "The `initHashChangeEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HashChangeEvent/initHashChangeEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HashChangeEvent`*"]
    pub fn init_hash_change_event_with_can_bubble_arg_and_cancelable_arg_and_old_url_arg_and_new_url_arg(
        this: &HashChangeEvent,
        type_arg: &str,
        can_bubble_arg: bool,
        cancelable_arg: bool,
        old_url_arg: &str,
        new_url_arg: &str,
    );
}
