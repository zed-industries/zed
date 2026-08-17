#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "Event",
        typescript_type = "Event"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Event` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Event)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Event`*"]
    pub type Event;
    #[wasm_bindgen(method, getter, js_class = "Event", js_name = "type")]
    #[doc = "Getter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Event/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Event`*"]
    pub fn type_(this: &Event) -> ::alloc::string::String;
    #[cfg(feature = "EventTarget")]
    #[wasm_bindgen(method, getter, js_class = "Event", js_name = "target")]
    #[doc = "Getter for the `target` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Event/target)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Event`, `EventTarget`*"]
    pub fn target(this: &Event) -> Option<EventTarget>;
    #[cfg(feature = "EventTarget")]
    #[wasm_bindgen(method, getter, js_class = "Event", js_name = "currentTarget")]
    #[doc = "Getter for the `currentTarget` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Event/currentTarget)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Event`, `EventTarget`*"]
    pub fn current_target(this: &Event) -> Option<EventTarget>;
    #[wasm_bindgen(method, getter, js_class = "Event", js_name = "eventPhase")]
    #[doc = "Getter for the `eventPhase` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Event/eventPhase)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Event`*"]
    pub fn event_phase(this: &Event) -> u16;
    #[wasm_bindgen(method, getter, js_class = "Event", js_name = "bubbles")]
    #[doc = "Getter for the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Event/bubbles)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Event`*"]
    pub fn bubbles(this: &Event) -> bool;
    #[wasm_bindgen(method, getter, js_class = "Event", js_name = "cancelable")]
    #[doc = "Getter for the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Event/cancelable)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Event`*"]
    pub fn cancelable(this: &Event) -> bool;
    #[wasm_bindgen(method, getter, js_class = "Event", js_name = "defaultPrevented")]
    #[doc = "Getter for the `defaultPrevented` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Event/defaultPrevented)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Event`*"]
    pub fn default_prevented(this: &Event) -> bool;
    #[wasm_bindgen(method, getter, js_class = "Event", js_name = "composed")]
    #[doc = "Getter for the `composed` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Event/composed)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Event`*"]
    pub fn composed(this: &Event) -> bool;
    #[wasm_bindgen(method, getter, js_class = "Event", js_name = "isTrusted")]
    #[doc = "Getter for the `isTrusted` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Event/isTrusted)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Event`*"]
    pub fn is_trusted(this: &Event) -> bool;
    #[wasm_bindgen(method, getter, js_class = "Event", js_name = "timeStamp")]
    #[doc = "Getter for the `timeStamp` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Event/timeStamp)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Event`*"]
    pub fn time_stamp(this: &Event) -> f64;
    #[wasm_bindgen(method, getter, js_class = "Event", js_name = "cancelBubble")]
    #[doc = "Getter for the `cancelBubble` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Event/cancelBubble)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Event`*"]
    pub fn cancel_bubble(this: &Event) -> bool;
    #[wasm_bindgen(method, setter, js_class = "Event", js_name = "cancelBubble")]
    #[doc = "Setter for the `cancelBubble` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Event/cancelBubble)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Event`*"]
    pub fn set_cancel_bubble(this: &Event, value: bool);
    #[wasm_bindgen(catch, constructor, js_class = "Event")]
    #[doc = "The `new Event(..)` constructor, creating a new instance of `Event`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Event/Event)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Event`*"]
    pub fn new(type_: &str) -> Result<Event, JsValue>;
    #[cfg(feature = "EventInit")]
    #[wasm_bindgen(catch, constructor, js_class = "Event")]
    #[doc = "The `new Event(..)` constructor, creating a new instance of `Event`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Event/Event)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Event`, `EventInit`*"]
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &EventInit,
    ) -> Result<Event, JsValue>;
    #[wasm_bindgen(method, js_class = "Event", js_name = "composedPath")]
    #[doc = "The `composedPath()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Event/composedPath)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Event`*"]
    pub fn composed_path(this: &Event) -> ::js_sys::Array;
    #[wasm_bindgen(method, js_class = "Event", js_name = "initEvent")]
    #[doc = "The `initEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Event/initEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Event`*"]
    pub fn init_event(this: &Event, type_: &str);
    #[wasm_bindgen(method, js_class = "Event", js_name = "initEvent")]
    #[doc = "The `initEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Event/initEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Event`*"]
    pub fn init_event_with_bubbles(this: &Event, type_: &str, bubbles: bool);
    #[wasm_bindgen(method, js_class = "Event", js_name = "initEvent")]
    #[doc = "The `initEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Event/initEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Event`*"]
    pub fn init_event_with_bubbles_and_cancelable(
        this: &Event,
        type_: &str,
        bubbles: bool,
        cancelable: bool,
    );
    #[wasm_bindgen(method, js_class = "Event", js_name = "preventDefault")]
    #[doc = "The `preventDefault()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Event/preventDefault)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Event`*"]
    pub fn prevent_default(this: &Event);
    #[wasm_bindgen(method, js_class = "Event", js_name = "stopImmediatePropagation")]
    #[doc = "The `stopImmediatePropagation()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Event/stopImmediatePropagation)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Event`*"]
    pub fn stop_immediate_propagation(this: &Event);
    #[wasm_bindgen(method, js_class = "Event", js_name = "stopPropagation")]
    #[doc = "The `stopPropagation()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Event/stopPropagation)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Event`*"]
    pub fn stop_propagation(this: &Event);
}
impl Event {
    #[doc = "The `Event.NONE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Event`*"]
    pub const NONE: u16 = 0i64 as u16;
    #[doc = "The `Event.CAPTURING_PHASE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Event`*"]
    pub const CAPTURING_PHASE: u16 = 1u64 as u16;
    #[doc = "The `Event.AT_TARGET` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Event`*"]
    pub const AT_TARGET: u16 = 2u64 as u16;
    #[doc = "The `Event.BUBBLING_PHASE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Event`*"]
    pub const BUBBLING_PHASE: u16 = 3u64 as u16;
}
