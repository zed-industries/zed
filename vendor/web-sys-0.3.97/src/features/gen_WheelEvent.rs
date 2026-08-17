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
        js_name = "WheelEvent",
        typescript_type = "WheelEvent"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WheelEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WheelEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WheelEvent`*"]
    pub type WheelEvent;
    #[wasm_bindgen(method, getter, js_class = "WheelEvent", js_name = "deltaX")]
    #[doc = "Getter for the `deltaX` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WheelEvent/deltaX)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WheelEvent`*"]
    pub fn delta_x(this: &WheelEvent) -> f64;
    #[wasm_bindgen(method, getter, js_class = "WheelEvent", js_name = "deltaY")]
    #[doc = "Getter for the `deltaY` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WheelEvent/deltaY)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WheelEvent`*"]
    pub fn delta_y(this: &WheelEvent) -> f64;
    #[wasm_bindgen(method, getter, js_class = "WheelEvent", js_name = "deltaZ")]
    #[doc = "Getter for the `deltaZ` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WheelEvent/deltaZ)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WheelEvent`*"]
    pub fn delta_z(this: &WheelEvent) -> f64;
    #[wasm_bindgen(method, getter, js_class = "WheelEvent", js_name = "deltaMode")]
    #[doc = "Getter for the `deltaMode` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WheelEvent/deltaMode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WheelEvent`*"]
    pub fn delta_mode(this: &WheelEvent) -> u32;
    #[wasm_bindgen(catch, constructor, js_class = "WheelEvent")]
    #[doc = "The `new WheelEvent(..)` constructor, creating a new instance of `WheelEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WheelEvent/WheelEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WheelEvent`*"]
    pub fn new(type_: &str) -> Result<WheelEvent, JsValue>;
    #[cfg(feature = "WheelEventInit")]
    #[wasm_bindgen(catch, constructor, js_class = "WheelEvent")]
    #[doc = "The `new WheelEvent(..)` constructor, creating a new instance of `WheelEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WheelEvent/WheelEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WheelEvent`, `WheelEventInit`*"]
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &WheelEventInit,
    ) -> Result<WheelEvent, JsValue>;
}
impl WheelEvent {
    #[doc = "The `WheelEvent.DOM_DELTA_PIXEL` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WheelEvent`*"]
    pub const DOM_DELTA_PIXEL: u32 = 0u64 as u32;
    #[doc = "The `WheelEvent.DOM_DELTA_LINE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WheelEvent`*"]
    pub const DOM_DELTA_LINE: u32 = 1u64 as u32;
    #[doc = "The `WheelEvent.DOM_DELTA_PAGE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WheelEvent`*"]
    pub const DOM_DELTA_PAGE: u32 = 2u64 as u32;
}
