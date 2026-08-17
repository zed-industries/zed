#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "Event",
        extends = "::js_sys::Object",
        js_name = "StorageEvent",
        typescript_type = "StorageEvent"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `StorageEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StorageEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StorageEvent`*"]
    pub type StorageEvent;
    #[wasm_bindgen(method, getter, js_class = "StorageEvent", js_name = "key")]
    #[doc = "Getter for the `key` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StorageEvent/key)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StorageEvent`*"]
    pub fn key(this: &StorageEvent) -> Option<::alloc::string::String>;
    #[wasm_bindgen(method, getter, js_class = "StorageEvent", js_name = "oldValue")]
    #[doc = "Getter for the `oldValue` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StorageEvent/oldValue)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StorageEvent`*"]
    pub fn old_value(this: &StorageEvent) -> Option<::alloc::string::String>;
    #[wasm_bindgen(method, getter, js_class = "StorageEvent", js_name = "newValue")]
    #[doc = "Getter for the `newValue` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StorageEvent/newValue)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StorageEvent`*"]
    pub fn new_value(this: &StorageEvent) -> Option<::alloc::string::String>;
    #[wasm_bindgen(method, getter, js_class = "StorageEvent", js_name = "url")]
    #[doc = "Getter for the `url` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StorageEvent/url)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StorageEvent`*"]
    pub fn url(this: &StorageEvent) -> Option<::alloc::string::String>;
    #[cfg(feature = "Storage")]
    #[wasm_bindgen(method, getter, js_class = "StorageEvent", js_name = "storageArea")]
    #[doc = "Getter for the `storageArea` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StorageEvent/storageArea)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Storage`, `StorageEvent`*"]
    pub fn storage_area(this: &StorageEvent) -> Option<Storage>;
    #[wasm_bindgen(catch, constructor, js_class = "StorageEvent")]
    #[doc = "The `new StorageEvent(..)` constructor, creating a new instance of `StorageEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StorageEvent/StorageEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StorageEvent`*"]
    pub fn new(type_: &str) -> Result<StorageEvent, JsValue>;
    #[cfg(feature = "StorageEventInit")]
    #[wasm_bindgen(catch, constructor, js_class = "StorageEvent")]
    #[doc = "The `new StorageEvent(..)` constructor, creating a new instance of `StorageEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StorageEvent/StorageEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StorageEvent`, `StorageEventInit`*"]
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &StorageEventInit,
    ) -> Result<StorageEvent, JsValue>;
    #[wasm_bindgen(method, js_class = "StorageEvent", js_name = "initStorageEvent")]
    #[doc = "The `initStorageEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StorageEvent/initStorageEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StorageEvent`*"]
    pub fn init_storage_event(this: &StorageEvent, type_: &str);
    #[wasm_bindgen(method, js_class = "StorageEvent", js_name = "initStorageEvent")]
    #[doc = "The `initStorageEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StorageEvent/initStorageEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StorageEvent`*"]
    pub fn init_storage_event_with_can_bubble(this: &StorageEvent, type_: &str, can_bubble: bool);
    #[wasm_bindgen(method, js_class = "StorageEvent", js_name = "initStorageEvent")]
    #[doc = "The `initStorageEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StorageEvent/initStorageEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StorageEvent`*"]
    pub fn init_storage_event_with_can_bubble_and_cancelable(
        this: &StorageEvent,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
    );
    #[wasm_bindgen(method, js_class = "StorageEvent", js_name = "initStorageEvent")]
    #[doc = "The `initStorageEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StorageEvent/initStorageEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StorageEvent`*"]
    pub fn init_storage_event_with_can_bubble_and_cancelable_and_key(
        this: &StorageEvent,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        key: Option<&str>,
    );
    #[wasm_bindgen(method, js_class = "StorageEvent", js_name = "initStorageEvent")]
    #[doc = "The `initStorageEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StorageEvent/initStorageEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StorageEvent`*"]
    pub fn init_storage_event_with_can_bubble_and_cancelable_and_key_and_old_value(
        this: &StorageEvent,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        key: Option<&str>,
        old_value: Option<&str>,
    );
    #[wasm_bindgen(method, js_class = "StorageEvent", js_name = "initStorageEvent")]
    #[doc = "The `initStorageEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StorageEvent/initStorageEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StorageEvent`*"]
    pub fn init_storage_event_with_can_bubble_and_cancelable_and_key_and_old_value_and_new_value(
        this: &StorageEvent,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        key: Option<&str>,
        old_value: Option<&str>,
        new_value: Option<&str>,
    );
    #[wasm_bindgen(method, js_class = "StorageEvent", js_name = "initStorageEvent")]
    #[doc = "The `initStorageEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StorageEvent/initStorageEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StorageEvent`*"]
    pub fn init_storage_event_with_can_bubble_and_cancelable_and_key_and_old_value_and_new_value_and_url(
        this: &StorageEvent,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        key: Option<&str>,
        old_value: Option<&str>,
        new_value: Option<&str>,
        url: Option<&str>,
    );
    #[cfg(feature = "Storage")]
    #[wasm_bindgen(method, js_class = "StorageEvent", js_name = "initStorageEvent")]
    #[doc = "The `initStorageEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StorageEvent/initStorageEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Storage`, `StorageEvent`*"]
    pub fn init_storage_event_with_can_bubble_and_cancelable_and_key_and_old_value_and_new_value_and_url_and_storage_area(
        this: &StorageEvent,
        type_: &str,
        can_bubble: bool,
        cancelable: bool,
        key: Option<&str>,
        old_value: Option<&str>,
        new_value: Option<&str>,
        url: Option<&str>,
        storage_area: Option<&Storage>,
    );
}
