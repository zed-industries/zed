#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "EventTarget",
        typescript_type = "EventTarget"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `EventTarget` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EventTarget)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventTarget`*"]
    pub type EventTarget;
    #[wasm_bindgen(catch, constructor, js_class = "EventTarget")]
    #[doc = "The `new EventTarget(..)` constructor, creating a new instance of `EventTarget`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EventTarget/EventTarget)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventTarget`*"]
    pub fn new() -> Result<EventTarget, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "EventTarget", js_name = "addEventListener")]
    #[doc = "The `addEventListener()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EventTarget/addEventListener)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventTarget`*"]
    pub fn add_event_listener_with_callback(
        this: &EventTarget,
        type_: &str,
        listener: &::js_sys::Function,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "EventListener")]
    #[wasm_bindgen(catch, method, js_class = "EventTarget", js_name = "addEventListener")]
    #[doc = "The `addEventListener()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EventTarget/addEventListener)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventListener`, `EventTarget`*"]
    pub fn add_event_listener_with_event_listener(
        this: &EventTarget,
        type_: &str,
        listener: &EventListener,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "AddEventListenerOptions")]
    #[wasm_bindgen(catch, method, js_class = "EventTarget", js_name = "addEventListener")]
    #[doc = "The `addEventListener()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EventTarget/addEventListener)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AddEventListenerOptions`, `EventTarget`*"]
    pub fn add_event_listener_with_callback_and_add_event_listener_options(
        this: &EventTarget,
        type_: &str,
        listener: &::js_sys::Function,
        options: &AddEventListenerOptions,
    ) -> Result<(), JsValue>;
    #[cfg(all(feature = "AddEventListenerOptions", feature = "EventListener",))]
    #[wasm_bindgen(catch, method, js_class = "EventTarget", js_name = "addEventListener")]
    #[doc = "The `addEventListener()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EventTarget/addEventListener)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AddEventListenerOptions`, `EventListener`, `EventTarget`*"]
    pub fn add_event_listener_with_event_listener_and_add_event_listener_options(
        this: &EventTarget,
        type_: &str,
        listener: &EventListener,
        options: &AddEventListenerOptions,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "EventTarget", js_name = "addEventListener")]
    #[doc = "The `addEventListener()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EventTarget/addEventListener)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventTarget`*"]
    pub fn add_event_listener_with_callback_and_bool(
        this: &EventTarget,
        type_: &str,
        listener: &::js_sys::Function,
        options: bool,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "EventListener")]
    #[wasm_bindgen(catch, method, js_class = "EventTarget", js_name = "addEventListener")]
    #[doc = "The `addEventListener()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EventTarget/addEventListener)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventListener`, `EventTarget`*"]
    pub fn add_event_listener_with_event_listener_and_bool(
        this: &EventTarget,
        type_: &str,
        listener: &EventListener,
        options: bool,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "AddEventListenerOptions")]
    #[wasm_bindgen(catch, method, js_class = "EventTarget", js_name = "addEventListener")]
    #[doc = "The `addEventListener()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EventTarget/addEventListener)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AddEventListenerOptions`, `EventTarget`*"]
    pub fn add_event_listener_with_callback_and_add_event_listener_options_and_wants_untrusted(
        this: &EventTarget,
        type_: &str,
        listener: &::js_sys::Function,
        options: &AddEventListenerOptions,
        wants_untrusted: Option<bool>,
    ) -> Result<(), JsValue>;
    #[cfg(all(feature = "AddEventListenerOptions", feature = "EventListener",))]
    #[wasm_bindgen(catch, method, js_class = "EventTarget", js_name = "addEventListener")]
    #[doc = "The `addEventListener()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EventTarget/addEventListener)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AddEventListenerOptions`, `EventListener`, `EventTarget`*"]
    pub fn add_event_listener_with_event_listener_and_add_event_listener_options_and_wants_untrusted(
        this: &EventTarget,
        type_: &str,
        listener: &EventListener,
        options: &AddEventListenerOptions,
        wants_untrusted: Option<bool>,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "EventTarget", js_name = "addEventListener")]
    #[doc = "The `addEventListener()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EventTarget/addEventListener)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventTarget`*"]
    pub fn add_event_listener_with_callback_and_bool_and_wants_untrusted(
        this: &EventTarget,
        type_: &str,
        listener: &::js_sys::Function,
        options: bool,
        wants_untrusted: Option<bool>,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "EventListener")]
    #[wasm_bindgen(catch, method, js_class = "EventTarget", js_name = "addEventListener")]
    #[doc = "The `addEventListener()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EventTarget/addEventListener)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventListener`, `EventTarget`*"]
    pub fn add_event_listener_with_event_listener_and_bool_and_wants_untrusted(
        this: &EventTarget,
        type_: &str,
        listener: &EventListener,
        options: bool,
        wants_untrusted: Option<bool>,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "Event")]
    #[wasm_bindgen(catch, method, js_class = "EventTarget", js_name = "dispatchEvent")]
    #[doc = "The `dispatchEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EventTarget/dispatchEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Event`, `EventTarget`*"]
    pub fn dispatch_event(this: &EventTarget, event: &Event) -> Result<bool, JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "EventTarget",
        js_name = "removeEventListener"
    )]
    #[doc = "The `removeEventListener()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EventTarget/removeEventListener)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventTarget`*"]
    pub fn remove_event_listener_with_callback(
        this: &EventTarget,
        type_: &str,
        listener: &::js_sys::Function,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "EventListener")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "EventTarget",
        js_name = "removeEventListener"
    )]
    #[doc = "The `removeEventListener()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EventTarget/removeEventListener)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventListener`, `EventTarget`*"]
    pub fn remove_event_listener_with_event_listener(
        this: &EventTarget,
        type_: &str,
        listener: &EventListener,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "EventListenerOptions")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "EventTarget",
        js_name = "removeEventListener"
    )]
    #[doc = "The `removeEventListener()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EventTarget/removeEventListener)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventListenerOptions`, `EventTarget`*"]
    pub fn remove_event_listener_with_callback_and_event_listener_options(
        this: &EventTarget,
        type_: &str,
        listener: &::js_sys::Function,
        options: &EventListenerOptions,
    ) -> Result<(), JsValue>;
    #[cfg(all(feature = "EventListener", feature = "EventListenerOptions",))]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "EventTarget",
        js_name = "removeEventListener"
    )]
    #[doc = "The `removeEventListener()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EventTarget/removeEventListener)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventListener`, `EventListenerOptions`, `EventTarget`*"]
    pub fn remove_event_listener_with_event_listener_and_event_listener_options(
        this: &EventTarget,
        type_: &str,
        listener: &EventListener,
        options: &EventListenerOptions,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "EventTarget",
        js_name = "removeEventListener"
    )]
    #[doc = "The `removeEventListener()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EventTarget/removeEventListener)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventTarget`*"]
    pub fn remove_event_listener_with_callback_and_bool(
        this: &EventTarget,
        type_: &str,
        listener: &::js_sys::Function,
        options: bool,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "EventListener")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "EventTarget",
        js_name = "removeEventListener"
    )]
    #[doc = "The `removeEventListener()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EventTarget/removeEventListener)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventListener`, `EventTarget`*"]
    pub fn remove_event_listener_with_event_listener_and_bool(
        this: &EventTarget,
        type_: &str,
        listener: &EventListener,
        options: bool,
    ) -> Result<(), JsValue>;
}
