#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "Event",
        extends = "::js_sys::Object",
        js_name = "DeviceLightEvent",
        typescript_type = "DeviceLightEvent"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DeviceLightEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceLightEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceLightEvent`*"]
    pub type DeviceLightEvent;
    #[wasm_bindgen(method, getter, js_class = "DeviceLightEvent", js_name = "value")]
    #[doc = "Getter for the `value` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceLightEvent/value)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceLightEvent`*"]
    pub fn value(this: &DeviceLightEvent) -> f64;
    #[wasm_bindgen(catch, constructor, js_class = "DeviceLightEvent")]
    #[doc = "The `new DeviceLightEvent(..)` constructor, creating a new instance of `DeviceLightEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceLightEvent/DeviceLightEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceLightEvent`*"]
    pub fn new(type_: &str) -> Result<DeviceLightEvent, JsValue>;
    #[cfg(feature = "DeviceLightEventInit")]
    #[wasm_bindgen(catch, constructor, js_class = "DeviceLightEvent")]
    #[doc = "The `new DeviceLightEvent(..)` constructor, creating a new instance of `DeviceLightEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceLightEvent/DeviceLightEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceLightEvent`, `DeviceLightEventInit`*"]
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &DeviceLightEventInit,
    ) -> Result<DeviceLightEvent, JsValue>;
}
