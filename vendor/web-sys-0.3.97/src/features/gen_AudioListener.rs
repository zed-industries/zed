#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "AudioListener",
        typescript_type = "AudioListener"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AudioListener` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioListener)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioListener`*"]
    pub type AudioListener;
    #[wasm_bindgen(method, getter, js_class = "AudioListener", js_name = "dopplerFactor")]
    #[doc = "Getter for the `dopplerFactor` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioListener/dopplerFactor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioListener`*"]
    pub fn doppler_factor(this: &AudioListener) -> f64;
    #[wasm_bindgen(method, setter, js_class = "AudioListener", js_name = "dopplerFactor")]
    #[doc = "Setter for the `dopplerFactor` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioListener/dopplerFactor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioListener`*"]
    pub fn set_doppler_factor(this: &AudioListener, value: f64);
    #[wasm_bindgen(method, getter, js_class = "AudioListener", js_name = "speedOfSound")]
    #[doc = "Getter for the `speedOfSound` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioListener/speedOfSound)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioListener`*"]
    pub fn speed_of_sound(this: &AudioListener) -> f64;
    #[wasm_bindgen(method, setter, js_class = "AudioListener", js_name = "speedOfSound")]
    #[doc = "Setter for the `speedOfSound` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioListener/speedOfSound)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioListener`*"]
    pub fn set_speed_of_sound(this: &AudioListener, value: f64);
    #[wasm_bindgen(method, js_class = "AudioListener", js_name = "setOrientation")]
    #[doc = "The `setOrientation()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioListener/setOrientation)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioListener`*"]
    pub fn set_orientation(
        this: &AudioListener,
        x: f64,
        y: f64,
        z: f64,
        x_up: f64,
        y_up: f64,
        z_up: f64,
    );
    #[wasm_bindgen(method, js_class = "AudioListener", js_name = "setPosition")]
    #[doc = "The `setPosition()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioListener/setPosition)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioListener`*"]
    pub fn set_position(this: &AudioListener, x: f64, y: f64, z: f64);
    #[wasm_bindgen(method, js_class = "AudioListener", js_name = "setVelocity")]
    #[doc = "The `setVelocity()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioListener/setVelocity)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioListener`*"]
    pub fn set_velocity(this: &AudioListener, x: f64, y: f64, z: f64);
}
