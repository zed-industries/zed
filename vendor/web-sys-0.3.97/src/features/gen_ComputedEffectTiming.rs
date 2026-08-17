#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "ComputedEffectTiming")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ComputedEffectTiming` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ComputedEffectTiming`*"]
    pub type ComputedEffectTiming;
    #[doc = "Get the `delay` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ComputedEffectTiming`*"]
    #[wasm_bindgen(method, getter = "delay")]
    pub fn get_delay(this: &ComputedEffectTiming) -> Option<f64>;
    #[doc = "Change the `delay` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ComputedEffectTiming`*"]
    #[wasm_bindgen(method, setter = "delay")]
    pub fn set_delay(this: &ComputedEffectTiming, val: f64);
    #[cfg(feature = "PlaybackDirection")]
    #[doc = "Get the `direction` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ComputedEffectTiming`, `PlaybackDirection`*"]
    #[wasm_bindgen(method, getter = "direction")]
    pub fn get_direction(this: &ComputedEffectTiming) -> Option<PlaybackDirection>;
    #[cfg(feature = "PlaybackDirection")]
    #[doc = "Change the `direction` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ComputedEffectTiming`, `PlaybackDirection`*"]
    #[wasm_bindgen(method, setter = "direction")]
    pub fn set_direction(this: &ComputedEffectTiming, val: PlaybackDirection);
    #[doc = "Get the `duration` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ComputedEffectTiming`*"]
    #[wasm_bindgen(method, getter = "duration")]
    pub fn get_duration(this: &ComputedEffectTiming) -> ::wasm_bindgen::JsValue;
    #[doc = "Change the `duration` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ComputedEffectTiming`*"]
    #[wasm_bindgen(method, setter = "duration")]
    pub fn set_duration(this: &ComputedEffectTiming, val: &::wasm_bindgen::JsValue);
    #[doc = "Change the `duration` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ComputedEffectTiming`*"]
    #[wasm_bindgen(method, setter = "duration")]
    pub fn set_duration_f64(this: &ComputedEffectTiming, val: f64);
    #[doc = "Change the `duration` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ComputedEffectTiming`*"]
    #[wasm_bindgen(method, setter = "duration")]
    pub fn set_duration_str(this: &ComputedEffectTiming, val: &str);
    #[doc = "Get the `easing` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ComputedEffectTiming`*"]
    #[wasm_bindgen(method, getter = "easing")]
    pub fn get_easing(this: &ComputedEffectTiming) -> Option<::alloc::string::String>;
    #[doc = "Change the `easing` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ComputedEffectTiming`*"]
    #[wasm_bindgen(method, setter = "easing")]
    pub fn set_easing(this: &ComputedEffectTiming, val: &str);
    #[doc = "Get the `endDelay` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ComputedEffectTiming`*"]
    #[wasm_bindgen(method, getter = "endDelay")]
    pub fn get_end_delay(this: &ComputedEffectTiming) -> Option<f64>;
    #[doc = "Change the `endDelay` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ComputedEffectTiming`*"]
    #[wasm_bindgen(method, setter = "endDelay")]
    pub fn set_end_delay(this: &ComputedEffectTiming, val: f64);
    #[cfg(feature = "FillMode")]
    #[doc = "Get the `fill` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ComputedEffectTiming`, `FillMode`*"]
    #[wasm_bindgen(method, getter = "fill")]
    pub fn get_fill(this: &ComputedEffectTiming) -> Option<FillMode>;
    #[cfg(feature = "FillMode")]
    #[doc = "Change the `fill` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ComputedEffectTiming`, `FillMode`*"]
    #[wasm_bindgen(method, setter = "fill")]
    pub fn set_fill(this: &ComputedEffectTiming, val: FillMode);
    #[doc = "Get the `iterationStart` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ComputedEffectTiming`*"]
    #[wasm_bindgen(method, getter = "iterationStart")]
    pub fn get_iteration_start(this: &ComputedEffectTiming) -> Option<f64>;
    #[doc = "Change the `iterationStart` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ComputedEffectTiming`*"]
    #[wasm_bindgen(method, setter = "iterationStart")]
    pub fn set_iteration_start(this: &ComputedEffectTiming, val: f64);
    #[doc = "Get the `iterations` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ComputedEffectTiming`*"]
    #[wasm_bindgen(method, getter = "iterations")]
    pub fn get_iterations(this: &ComputedEffectTiming) -> Option<f64>;
    #[doc = "Change the `iterations` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ComputedEffectTiming`*"]
    #[wasm_bindgen(method, setter = "iterations")]
    pub fn set_iterations(this: &ComputedEffectTiming, val: f64);
    #[doc = "Get the `activeDuration` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ComputedEffectTiming`*"]
    #[wasm_bindgen(method, getter = "activeDuration")]
    pub fn get_active_duration(this: &ComputedEffectTiming) -> Option<f64>;
    #[doc = "Change the `activeDuration` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ComputedEffectTiming`*"]
    #[wasm_bindgen(method, setter = "activeDuration")]
    pub fn set_active_duration(this: &ComputedEffectTiming, val: f64);
    #[doc = "Get the `currentIteration` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ComputedEffectTiming`*"]
    #[wasm_bindgen(method, getter = "currentIteration")]
    pub fn get_current_iteration(this: &ComputedEffectTiming) -> Option<f64>;
    #[doc = "Change the `currentIteration` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ComputedEffectTiming`*"]
    #[wasm_bindgen(method, setter = "currentIteration")]
    pub fn set_current_iteration(this: &ComputedEffectTiming, val: Option<f64>);
    #[doc = "Get the `endTime` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ComputedEffectTiming`*"]
    #[wasm_bindgen(method, getter = "endTime")]
    pub fn get_end_time(this: &ComputedEffectTiming) -> Option<f64>;
    #[doc = "Change the `endTime` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ComputedEffectTiming`*"]
    #[wasm_bindgen(method, setter = "endTime")]
    pub fn set_end_time(this: &ComputedEffectTiming, val: f64);
    #[doc = "Get the `localTime` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ComputedEffectTiming`*"]
    #[wasm_bindgen(method, getter = "localTime")]
    pub fn get_local_time(this: &ComputedEffectTiming) -> Option<f64>;
    #[doc = "Change the `localTime` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ComputedEffectTiming`*"]
    #[wasm_bindgen(method, setter = "localTime")]
    pub fn set_local_time(this: &ComputedEffectTiming, val: Option<f64>);
    #[doc = "Get the `progress` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ComputedEffectTiming`*"]
    #[wasm_bindgen(method, getter = "progress")]
    pub fn get_progress(this: &ComputedEffectTiming) -> Option<f64>;
    #[doc = "Change the `progress` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ComputedEffectTiming`*"]
    #[wasm_bindgen(method, setter = "progress")]
    pub fn set_progress(this: &ComputedEffectTiming, val: Option<f64>);
}
impl ComputedEffectTiming {
    #[doc = "Construct a new `ComputedEffectTiming`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ComputedEffectTiming`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_delay()` instead."]
    pub fn delay(&mut self, val: f64) -> &mut Self {
        self.set_delay(val);
        self
    }
    #[cfg(feature = "PlaybackDirection")]
    #[deprecated = "Use `set_direction()` instead."]
    pub fn direction(&mut self, val: PlaybackDirection) -> &mut Self {
        self.set_direction(val);
        self
    }
    #[deprecated = "Use `set_duration()` instead."]
    pub fn duration(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_duration(val);
        self
    }
    #[deprecated = "Use `set_easing()` instead."]
    pub fn easing(&mut self, val: &str) -> &mut Self {
        self.set_easing(val);
        self
    }
    #[deprecated = "Use `set_end_delay()` instead."]
    pub fn end_delay(&mut self, val: f64) -> &mut Self {
        self.set_end_delay(val);
        self
    }
    #[cfg(feature = "FillMode")]
    #[deprecated = "Use `set_fill()` instead."]
    pub fn fill(&mut self, val: FillMode) -> &mut Self {
        self.set_fill(val);
        self
    }
    #[deprecated = "Use `set_iteration_start()` instead."]
    pub fn iteration_start(&mut self, val: f64) -> &mut Self {
        self.set_iteration_start(val);
        self
    }
    #[deprecated = "Use `set_iterations()` instead."]
    pub fn iterations(&mut self, val: f64) -> &mut Self {
        self.set_iterations(val);
        self
    }
    #[deprecated = "Use `set_active_duration()` instead."]
    pub fn active_duration(&mut self, val: f64) -> &mut Self {
        self.set_active_duration(val);
        self
    }
    #[deprecated = "Use `set_current_iteration()` instead."]
    pub fn current_iteration(&mut self, val: Option<f64>) -> &mut Self {
        self.set_current_iteration(val);
        self
    }
    #[deprecated = "Use `set_end_time()` instead."]
    pub fn end_time(&mut self, val: f64) -> &mut Self {
        self.set_end_time(val);
        self
    }
    #[deprecated = "Use `set_local_time()` instead."]
    pub fn local_time(&mut self, val: Option<f64>) -> &mut Self {
        self.set_local_time(val);
        self
    }
    #[deprecated = "Use `set_progress()` instead."]
    pub fn progress(&mut self, val: Option<f64>) -> &mut Self {
        self.set_progress(val);
        self
    }
}
impl Default for ComputedEffectTiming {
    fn default() -> Self {
        Self::new()
    }
}
