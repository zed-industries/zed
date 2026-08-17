#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "AudioParam",
        typescript_type = "AudioParam"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AudioParam` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioParam)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioParam`*"]
    pub type AudioParam;
    #[wasm_bindgen(method, getter, js_class = "AudioParam", js_name = "value")]
    #[doc = "Getter for the `value` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioParam/value)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioParam`*"]
    pub fn value(this: &AudioParam) -> f32;
    #[wasm_bindgen(method, setter, js_class = "AudioParam", js_name = "value")]
    #[doc = "Setter for the `value` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioParam/value)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioParam`*"]
    pub fn set_value(this: &AudioParam, value: f32);
    #[wasm_bindgen(method, getter, js_class = "AudioParam", js_name = "defaultValue")]
    #[doc = "Getter for the `defaultValue` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioParam/defaultValue)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioParam`*"]
    pub fn default_value(this: &AudioParam) -> f32;
    #[wasm_bindgen(method, getter, js_class = "AudioParam", js_name = "minValue")]
    #[doc = "Getter for the `minValue` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioParam/minValue)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioParam`*"]
    pub fn min_value(this: &AudioParam) -> f32;
    #[wasm_bindgen(method, getter, js_class = "AudioParam", js_name = "maxValue")]
    #[doc = "Getter for the `maxValue` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioParam/maxValue)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioParam`*"]
    pub fn max_value(this: &AudioParam) -> f32;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "AudioParam",
        js_name = "cancelScheduledValues"
    )]
    #[doc = "The `cancelScheduledValues()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioParam/cancelScheduledValues)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioParam`*"]
    pub fn cancel_scheduled_values(
        this: &AudioParam,
        start_time: f64,
    ) -> Result<AudioParam, JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "AudioParam",
        js_name = "exponentialRampToValueAtTime"
    )]
    #[doc = "The `exponentialRampToValueAtTime()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioParam/exponentialRampToValueAtTime)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioParam`*"]
    pub fn exponential_ramp_to_value_at_time(
        this: &AudioParam,
        value: f32,
        end_time: f64,
    ) -> Result<AudioParam, JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "AudioParam",
        js_name = "linearRampToValueAtTime"
    )]
    #[doc = "The `linearRampToValueAtTime()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioParam/linearRampToValueAtTime)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioParam`*"]
    pub fn linear_ramp_to_value_at_time(
        this: &AudioParam,
        value: f32,
        end_time: f64,
    ) -> Result<AudioParam, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "AudioParam", js_name = "setTargetAtTime")]
    #[doc = "The `setTargetAtTime()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioParam/setTargetAtTime)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioParam`*"]
    pub fn set_target_at_time(
        this: &AudioParam,
        target: f32,
        start_time: f64,
        time_constant: f64,
    ) -> Result<AudioParam, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "AudioParam", js_name = "setValueAtTime")]
    #[doc = "The `setValueAtTime()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioParam/setValueAtTime)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioParam`*"]
    pub fn set_value_at_time(
        this: &AudioParam,
        value: f32,
        start_time: f64,
    ) -> Result<AudioParam, JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "AudioParam",
        js_name = "setValueCurveAtTime"
    )]
    #[doc = "The `setValueCurveAtTime()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioParam/setValueCurveAtTime)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioParam`*"]
    pub fn set_value_curve_at_time(
        this: &AudioParam,
        values: &mut [f32],
        start_time: f64,
        duration: f64,
    ) -> Result<AudioParam, JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "AudioParam",
        js_name = "setValueCurveAtTime"
    )]
    #[doc = "The `setValueCurveAtTime()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioParam/setValueCurveAtTime)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioParam`*"]
    pub fn set_value_curve_at_time_with_f32_array(
        this: &AudioParam,
        values: &::js_sys::Float32Array,
        start_time: f64,
        duration: f64,
    ) -> Result<AudioParam, JsValue>;
}
