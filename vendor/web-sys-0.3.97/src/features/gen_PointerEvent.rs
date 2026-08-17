#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "MouseEvent",
        extends = "UiEvent",
        extends = "Event",
        extends = "::js_sys::Object",
        js_name = "PointerEvent",
        typescript_type = "PointerEvent"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PointerEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PointerEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEvent`*"]
    pub type PointerEvent;
    #[wasm_bindgen(method, getter, js_class = "PointerEvent", js_name = "pointerId")]
    #[doc = "Getter for the `pointerId` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PointerEvent/pointerId)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEvent`*"]
    pub fn pointer_id(this: &PointerEvent) -> i32;
    #[wasm_bindgen(method, getter, js_class = "PointerEvent", js_name = "width")]
    #[doc = "Getter for the `width` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PointerEvent/width)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEvent`*"]
    pub fn width(this: &PointerEvent) -> i32;
    #[wasm_bindgen(method, getter, js_class = "PointerEvent", js_name = "height")]
    #[doc = "Getter for the `height` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PointerEvent/height)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEvent`*"]
    pub fn height(this: &PointerEvent) -> i32;
    #[wasm_bindgen(method, getter, js_class = "PointerEvent", js_name = "pressure")]
    #[doc = "Getter for the `pressure` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PointerEvent/pressure)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEvent`*"]
    pub fn pressure(this: &PointerEvent) -> f32;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "PointerEvent",
        js_name = "tangentialPressure"
    )]
    #[doc = "Getter for the `tangentialPressure` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PointerEvent/tangentialPressure)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEvent`*"]
    pub fn tangential_pressure(this: &PointerEvent) -> f32;
    #[wasm_bindgen(method, getter, js_class = "PointerEvent", js_name = "tiltX")]
    #[doc = "Getter for the `tiltX` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PointerEvent/tiltX)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEvent`*"]
    pub fn tilt_x(this: &PointerEvent) -> i32;
    #[wasm_bindgen(method, getter, js_class = "PointerEvent", js_name = "tiltY")]
    #[doc = "Getter for the `tiltY` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PointerEvent/tiltY)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEvent`*"]
    pub fn tilt_y(this: &PointerEvent) -> i32;
    #[wasm_bindgen(method, getter, js_class = "PointerEvent", js_name = "twist")]
    #[doc = "Getter for the `twist` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PointerEvent/twist)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEvent`*"]
    pub fn twist(this: &PointerEvent) -> i32;
    #[wasm_bindgen(method, getter, js_class = "PointerEvent", js_name = "pointerType")]
    #[doc = "Getter for the `pointerType` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PointerEvent/pointerType)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEvent`*"]
    pub fn pointer_type(this: &PointerEvent) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "PointerEvent", js_name = "isPrimary")]
    #[doc = "Getter for the `isPrimary` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PointerEvent/isPrimary)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEvent`*"]
    pub fn is_primary(this: &PointerEvent) -> bool;
    #[wasm_bindgen(catch, constructor, js_class = "PointerEvent")]
    #[doc = "The `new PointerEvent(..)` constructor, creating a new instance of `PointerEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PointerEvent/PointerEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEvent`*"]
    pub fn new(type_: &str) -> Result<PointerEvent, JsValue>;
    #[cfg(feature = "PointerEventInit")]
    #[wasm_bindgen(catch, constructor, js_class = "PointerEvent")]
    #[doc = "The `new PointerEvent(..)` constructor, creating a new instance of `PointerEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PointerEvent/PointerEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEvent`, `PointerEventInit`*"]
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &PointerEventInit,
    ) -> Result<PointerEvent, JsValue>;
    #[wasm_bindgen(method, js_class = "PointerEvent", js_name = "getCoalescedEvents")]
    #[doc = "The `getCoalescedEvents()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PointerEvent/getCoalescedEvents)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEvent`*"]
    pub fn get_coalesced_events(this: &PointerEvent) -> ::js_sys::Array;
}
