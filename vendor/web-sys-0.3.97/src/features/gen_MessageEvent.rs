#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "Event",
        extends = "::js_sys::Object",
        js_name = "MessageEvent",
        typescript_type = "MessageEvent"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MessageEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessageEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessageEvent`*"]
    pub type MessageEvent;
    #[wasm_bindgen(method, getter, js_class = "MessageEvent", js_name = "data")]
    #[doc = "Getter for the `data` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessageEvent/data)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessageEvent`*"]
    pub fn data(this: &MessageEvent) -> ::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, getter, js_class = "MessageEvent", js_name = "origin")]
    #[doc = "Getter for the `origin` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessageEvent/origin)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessageEvent`*"]
    pub fn origin(this: &MessageEvent) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "MessageEvent", js_name = "lastEventId")]
    #[doc = "Getter for the `lastEventId` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessageEvent/lastEventId)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessageEvent`*"]
    pub fn last_event_id(this: &MessageEvent) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "MessageEvent", js_name = "source")]
    #[doc = "Getter for the `source` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessageEvent/source)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessageEvent`*"]
    pub fn source(this: &MessageEvent) -> Option<::js_sys::Object>;
    #[wasm_bindgen(method, getter, js_class = "MessageEvent", js_name = "ports")]
    #[doc = "Getter for the `ports` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessageEvent/ports)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessageEvent`*"]
    pub fn ports(this: &MessageEvent) -> ::js_sys::Array;
    #[wasm_bindgen(catch, constructor, js_class = "MessageEvent")]
    #[doc = "The `new MessageEvent(..)` constructor, creating a new instance of `MessageEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessageEvent/MessageEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessageEvent`*"]
    pub fn new(type_: &str) -> Result<MessageEvent, JsValue>;
    #[cfg(feature = "MessageEventInit")]
    #[wasm_bindgen(catch, constructor, js_class = "MessageEvent")]
    #[doc = "The `new MessageEvent(..)` constructor, creating a new instance of `MessageEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessageEvent/MessageEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessageEvent`, `MessageEventInit`*"]
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &MessageEventInit,
    ) -> Result<MessageEvent, JsValue>;
    #[wasm_bindgen(method, js_class = "MessageEvent", js_name = "initMessageEvent")]
    #[doc = "The `initMessageEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessageEvent/initMessageEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessageEvent`*"]
    pub fn init_message_event(this: &MessageEvent, type_: &str);
    #[wasm_bindgen(method, js_class = "MessageEvent", js_name = "initMessageEvent")]
    #[doc = "The `initMessageEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessageEvent/initMessageEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessageEvent`*"]
    pub fn init_message_event_with_bubbles(this: &MessageEvent, type_: &str, bubbles: bool);
    #[wasm_bindgen(method, js_class = "MessageEvent", js_name = "initMessageEvent")]
    #[doc = "The `initMessageEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessageEvent/initMessageEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessageEvent`*"]
    pub fn init_message_event_with_bubbles_and_cancelable(
        this: &MessageEvent,
        type_: &str,
        bubbles: bool,
        cancelable: bool,
    );
    #[wasm_bindgen(method, js_class = "MessageEvent", js_name = "initMessageEvent")]
    #[doc = "The `initMessageEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessageEvent/initMessageEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessageEvent`*"]
    pub fn init_message_event_with_bubbles_and_cancelable_and_data(
        this: &MessageEvent,
        type_: &str,
        bubbles: bool,
        cancelable: bool,
        data: &::wasm_bindgen::JsValue,
    );
    #[wasm_bindgen(method, js_class = "MessageEvent", js_name = "initMessageEvent")]
    #[doc = "The `initMessageEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessageEvent/initMessageEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessageEvent`*"]
    pub fn init_message_event_with_bubbles_and_cancelable_and_data_and_origin(
        this: &MessageEvent,
        type_: &str,
        bubbles: bool,
        cancelable: bool,
        data: &::wasm_bindgen::JsValue,
        origin: &str,
    );
    #[wasm_bindgen(method, js_class = "MessageEvent", js_name = "initMessageEvent")]
    #[doc = "The `initMessageEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessageEvent/initMessageEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessageEvent`*"]
    pub fn init_message_event_with_bubbles_and_cancelable_and_data_and_origin_and_last_event_id(
        this: &MessageEvent,
        type_: &str,
        bubbles: bool,
        cancelable: bool,
        data: &::wasm_bindgen::JsValue,
        origin: &str,
        last_event_id: &str,
    );
    #[cfg(feature = "Window")]
    #[wasm_bindgen(method, js_class = "MessageEvent", js_name = "initMessageEvent")]
    #[doc = "The `initMessageEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessageEvent/initMessageEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessageEvent`, `Window`*"]
    pub fn init_message_event_with_bubbles_and_cancelable_and_data_and_origin_and_last_event_id_and_opt_window(
        this: &MessageEvent,
        type_: &str,
        bubbles: bool,
        cancelable: bool,
        data: &::wasm_bindgen::JsValue,
        origin: &str,
        last_event_id: &str,
        source: Option<&Window>,
    );
    #[cfg(feature = "MessagePort")]
    #[wasm_bindgen(method, js_class = "MessageEvent", js_name = "initMessageEvent")]
    #[doc = "The `initMessageEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessageEvent/initMessageEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessageEvent`, `MessagePort`*"]
    pub fn init_message_event_with_bubbles_and_cancelable_and_data_and_origin_and_last_event_id_and_opt_message_port(
        this: &MessageEvent,
        type_: &str,
        bubbles: bool,
        cancelable: bool,
        data: &::wasm_bindgen::JsValue,
        origin: &str,
        last_event_id: &str,
        source: Option<&MessagePort>,
    );
    #[cfg(feature = "ServiceWorker")]
    #[wasm_bindgen(method, js_class = "MessageEvent", js_name = "initMessageEvent")]
    #[doc = "The `initMessageEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessageEvent/initMessageEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessageEvent`, `ServiceWorker`*"]
    pub fn init_message_event_with_bubbles_and_cancelable_and_data_and_origin_and_last_event_id_and_opt_service_worker(
        this: &MessageEvent,
        type_: &str,
        bubbles: bool,
        cancelable: bool,
        data: &::wasm_bindgen::JsValue,
        origin: &str,
        last_event_id: &str,
        source: Option<&ServiceWorker>,
    );
    #[cfg(feature = "Window")]
    #[wasm_bindgen(method, js_class = "MessageEvent", js_name = "initMessageEvent")]
    #[doc = "The `initMessageEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessageEvent/initMessageEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessageEvent`, `Window`*"]
    pub fn init_message_event_with_bubbles_and_cancelable_and_data_and_origin_and_last_event_id_and_opt_window_and_ports(
        this: &MessageEvent,
        type_: &str,
        bubbles: bool,
        cancelable: bool,
        data: &::wasm_bindgen::JsValue,
        origin: &str,
        last_event_id: &str,
        source: Option<&Window>,
        ports: &::wasm_bindgen::JsValue,
    );
    #[cfg(feature = "MessagePort")]
    #[wasm_bindgen(method, js_class = "MessageEvent", js_name = "initMessageEvent")]
    #[doc = "The `initMessageEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessageEvent/initMessageEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessageEvent`, `MessagePort`*"]
    pub fn init_message_event_with_bubbles_and_cancelable_and_data_and_origin_and_last_event_id_and_opt_message_port_and_ports(
        this: &MessageEvent,
        type_: &str,
        bubbles: bool,
        cancelable: bool,
        data: &::wasm_bindgen::JsValue,
        origin: &str,
        last_event_id: &str,
        source: Option<&MessagePort>,
        ports: &::wasm_bindgen::JsValue,
    );
    #[cfg(feature = "ServiceWorker")]
    #[wasm_bindgen(method, js_class = "MessageEvent", js_name = "initMessageEvent")]
    #[doc = "The `initMessageEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessageEvent/initMessageEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessageEvent`, `ServiceWorker`*"]
    pub fn init_message_event_with_bubbles_and_cancelable_and_data_and_origin_and_last_event_id_and_opt_service_worker_and_ports(
        this: &MessageEvent,
        type_: &str,
        bubbles: bool,
        cancelable: bool,
        data: &::wasm_bindgen::JsValue,
        origin: &str,
        last_event_id: &str,
        source: Option<&ServiceWorker>,
        ports: &::wasm_bindgen::JsValue,
    );
}
