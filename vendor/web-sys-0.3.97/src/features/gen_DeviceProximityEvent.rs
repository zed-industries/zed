#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "Event",
        extends = "::js_sys::Object",
        js_name = "DeviceProximityEvent",
        typescript_type = "DeviceProximityEvent"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DeviceProximityEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceProximityEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceProximityEvent`*"]
    pub type DeviceProximityEvent;
    #[wasm_bindgen(method, getter, js_class = "DeviceProximityEvent", js_name = "value")]
    #[doc = "Getter for the `value` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceProximityEvent/value)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceProximityEvent`*"]
    pub fn value(this: &DeviceProximityEvent) -> f64;
    #[wasm_bindgen(method, getter, js_class = "DeviceProximityEvent", js_name = "min")]
    #[doc = "Getter for the `min` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceProximityEvent/min)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceProximityEvent`*"]
    pub fn min(this: &DeviceProximityEvent) -> f64;
    #[wasm_bindgen(method, getter, js_class = "DeviceProximityEvent", js_name = "max")]
    #[doc = "Getter for the `max` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceProximityEvent/max)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceProximityEvent`*"]
    pub fn max(this: &DeviceProximityEvent) -> f64;
    #[wasm_bindgen(catch, constructor, js_class = "DeviceProximityEvent")]
    #[doc = "The `new DeviceProximityEvent(..)` constructor, creating a new instance of `DeviceProximityEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceProximityEvent/DeviceProximityEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceProximityEvent`*"]
    pub fn new(type_: &str) -> Result<DeviceProximityEvent, JsValue>;
    #[cfg(feature = "DeviceProximityEventInit")]
    #[wasm_bindgen(catch, constructor, js_class = "DeviceProximityEvent")]
    #[doc = "The `new DeviceProximityEvent(..)` constructor, creating a new instance of `DeviceProximityEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceProximityEvent/DeviceProximityEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceProximityEvent`, `DeviceProximityEventInit`*"]
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &DeviceProximityEventInit,
    ) -> Result<DeviceProximityEvent, JsValue>;
}
