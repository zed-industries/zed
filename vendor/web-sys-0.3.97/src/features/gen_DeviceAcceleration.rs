#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (is_type_of = | _ | false , extends = "::js_sys::Object" , js_name = "DeviceAcceleration" , typescript_type = "DeviceAcceleration")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DeviceAcceleration` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceAcceleration)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceAcceleration`*"]
    pub type DeviceAcceleration;
    #[wasm_bindgen(method, getter, js_class = "DeviceAcceleration", js_name = "x")]
    #[doc = "Getter for the `x` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceAcceleration/x)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceAcceleration`*"]
    pub fn x(this: &DeviceAcceleration) -> Option<f64>;
    #[wasm_bindgen(method, getter, js_class = "DeviceAcceleration", js_name = "y")]
    #[doc = "Getter for the `y` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceAcceleration/y)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceAcceleration`*"]
    pub fn y(this: &DeviceAcceleration) -> Option<f64>;
    #[wasm_bindgen(method, getter, js_class = "DeviceAcceleration", js_name = "z")]
    #[doc = "Getter for the `z` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceAcceleration/z)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceAcceleration`*"]
    pub fn z(this: &DeviceAcceleration) -> Option<f64>;
}
