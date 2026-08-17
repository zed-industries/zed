#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (is_type_of = | _ | false , extends = "::js_sys::Object" , js_name = "Coordinates" , typescript_type = "Coordinates")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Coordinates` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Coordinates)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Coordinates`*"]
    pub type Coordinates;
    #[wasm_bindgen(method, getter, js_class = "Coordinates", js_name = "latitude")]
    #[doc = "Getter for the `latitude` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Coordinates/latitude)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Coordinates`*"]
    pub fn latitude(this: &Coordinates) -> f64;
    #[wasm_bindgen(method, getter, js_class = "Coordinates", js_name = "longitude")]
    #[doc = "Getter for the `longitude` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Coordinates/longitude)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Coordinates`*"]
    pub fn longitude(this: &Coordinates) -> f64;
    #[wasm_bindgen(method, getter, js_class = "Coordinates", js_name = "altitude")]
    #[doc = "Getter for the `altitude` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Coordinates/altitude)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Coordinates`*"]
    pub fn altitude(this: &Coordinates) -> Option<f64>;
    #[wasm_bindgen(method, getter, js_class = "Coordinates", js_name = "accuracy")]
    #[doc = "Getter for the `accuracy` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Coordinates/accuracy)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Coordinates`*"]
    pub fn accuracy(this: &Coordinates) -> f64;
    #[wasm_bindgen(method, getter, js_class = "Coordinates", js_name = "altitudeAccuracy")]
    #[doc = "Getter for the `altitudeAccuracy` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Coordinates/altitudeAccuracy)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Coordinates`*"]
    pub fn altitude_accuracy(this: &Coordinates) -> Option<f64>;
    #[wasm_bindgen(method, getter, js_class = "Coordinates", js_name = "heading")]
    #[doc = "Getter for the `heading` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Coordinates/heading)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Coordinates`*"]
    pub fn heading(this: &Coordinates) -> Option<f64>;
    #[wasm_bindgen(method, getter, js_class = "Coordinates", js_name = "speed")]
    #[doc = "Getter for the `speed` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Coordinates/speed)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Coordinates`*"]
    pub fn speed(this: &Coordinates) -> Option<f64>;
}
