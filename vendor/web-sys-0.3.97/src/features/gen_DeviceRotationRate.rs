#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (is_type_of = | _ | false , extends = "::js_sys::Object" , js_name = "DeviceRotationRate" , typescript_type = "DeviceRotationRate")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DeviceRotationRate` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceRotationRate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceRotationRate`*"]
    pub type DeviceRotationRate;
    #[wasm_bindgen(method, getter, js_class = "DeviceRotationRate", js_name = "alpha")]
    #[doc = "Getter for the `alpha` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceRotationRate/alpha)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceRotationRate`*"]
    pub fn alpha(this: &DeviceRotationRate) -> Option<f64>;
    #[wasm_bindgen(method, getter, js_class = "DeviceRotationRate", js_name = "beta")]
    #[doc = "Getter for the `beta` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceRotationRate/beta)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceRotationRate`*"]
    pub fn beta(this: &DeviceRotationRate) -> Option<f64>;
    #[wasm_bindgen(method, getter, js_class = "DeviceRotationRate", js_name = "gamma")]
    #[doc = "Getter for the `gamma` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceRotationRate/gamma)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceRotationRate`*"]
    pub fn gamma(this: &DeviceRotationRate) -> Option<f64>;
}
