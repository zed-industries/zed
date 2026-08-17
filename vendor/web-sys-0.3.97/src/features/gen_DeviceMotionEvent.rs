#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "Event",
        extends = "::js_sys::Object",
        js_name = "DeviceMotionEvent",
        typescript_type = "DeviceMotionEvent"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DeviceMotionEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceMotionEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceMotionEvent`*"]
    pub type DeviceMotionEvent;
    #[cfg(feature = "DeviceAcceleration")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "DeviceMotionEvent",
        js_name = "acceleration"
    )]
    #[doc = "Getter for the `acceleration` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceMotionEvent/acceleration)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceAcceleration`, `DeviceMotionEvent`*"]
    pub fn acceleration(this: &DeviceMotionEvent) -> Option<DeviceAcceleration>;
    #[cfg(feature = "DeviceAcceleration")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "DeviceMotionEvent",
        js_name = "accelerationIncludingGravity"
    )]
    #[doc = "Getter for the `accelerationIncludingGravity` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceMotionEvent/accelerationIncludingGravity)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceAcceleration`, `DeviceMotionEvent`*"]
    pub fn acceleration_including_gravity(this: &DeviceMotionEvent) -> Option<DeviceAcceleration>;
    #[cfg(feature = "DeviceRotationRate")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "DeviceMotionEvent",
        js_name = "rotationRate"
    )]
    #[doc = "Getter for the `rotationRate` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceMotionEvent/rotationRate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceMotionEvent`, `DeviceRotationRate`*"]
    pub fn rotation_rate(this: &DeviceMotionEvent) -> Option<DeviceRotationRate>;
    #[wasm_bindgen(method, getter, js_class = "DeviceMotionEvent", js_name = "interval")]
    #[doc = "Getter for the `interval` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceMotionEvent/interval)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceMotionEvent`*"]
    pub fn interval(this: &DeviceMotionEvent) -> Option<f64>;
    #[wasm_bindgen(catch, constructor, js_class = "DeviceMotionEvent")]
    #[doc = "The `new DeviceMotionEvent(..)` constructor, creating a new instance of `DeviceMotionEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceMotionEvent/DeviceMotionEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceMotionEvent`*"]
    pub fn new(type_: &str) -> Result<DeviceMotionEvent, JsValue>;
    #[cfg(feature = "DeviceMotionEventInit")]
    #[wasm_bindgen(catch, constructor, js_class = "DeviceMotionEvent")]
    #[doc = "The `new DeviceMotionEvent(..)` constructor, creating a new instance of `DeviceMotionEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceMotionEvent/DeviceMotionEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceMotionEvent`, `DeviceMotionEventInit`*"]
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &DeviceMotionEventInit,
    ) -> Result<DeviceMotionEvent, JsValue>;
}
