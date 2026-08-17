#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "EffectTiming")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `EffectTiming` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EffectTiming`*"]
    pub type EffectTiming;
    #[doc = "Get the `delay` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EffectTiming`*"]
    #[wasm_bindgen(method, getter = "delay")]
    pub fn get_delay(this: &EffectTiming) -> Option<f64>;
    #[doc = "Change the `delay` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EffectTiming`*"]
    #[wasm_bindgen(method, setter = "delay")]
    pub fn set_delay(this: &EffectTiming, val: f64);
    #[cfg(feature = "PlaybackDirection")]
    #[doc = "Get the `direction` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EffectTiming`, `PlaybackDirection`*"]
    #[wasm_bindgen(method, getter = "direction")]
    pub fn get_direction(this: &EffectTiming) -> Option<PlaybackDirection>;
    #[cfg(feature = "PlaybackDirection")]
    #[doc = "Change the `direction` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EffectTiming`, `PlaybackDirection`*"]
    #[wasm_bindgen(method, setter = "direction")]
    pub fn set_direction(this: &EffectTiming, val: PlaybackDirection);
    #[doc = "Get the `duration` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EffectTiming`*"]
    #[wasm_bindgen(method, getter = "duration")]
    pub fn get_duration(this: &EffectTiming) -> ::wasm_bindgen::JsValue;
    #[doc = "Change the `duration` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EffectTiming`*"]
    #[wasm_bindgen(method, setter = "duration")]
    pub fn set_duration(this: &EffectTiming, val: &::wasm_bindgen::JsValue);
    #[doc = "Change the `duration` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EffectTiming`*"]
    #[wasm_bindgen(method, setter = "duration")]
    pub fn set_duration_f64(this: &EffectTiming, val: f64);
    #[doc = "Change the `duration` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EffectTiming`*"]
    #[wasm_bindgen(method, setter = "duration")]
    pub fn set_duration_str(this: &EffectTiming, val: &str);
    #[doc = "Get the `easing` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EffectTiming`*"]
    #[wasm_bindgen(method, getter = "easing")]
    pub fn get_easing(this: &EffectTiming) -> Option<::alloc::string::String>;
    #[doc = "Change the `easing` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EffectTiming`*"]
    #[wasm_bindgen(method, setter = "easing")]
    pub fn set_easing(this: &EffectTiming, val: &str);
    #[doc = "Get the `endDelay` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EffectTiming`*"]
    #[wasm_bindgen(method, getter = "endDelay")]
    pub fn get_end_delay(this: &EffectTiming) -> Option<f64>;
    #[doc = "Change the `endDelay` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EffectTiming`*"]
    #[wasm_bindgen(method, setter = "endDelay")]
    pub fn set_end_delay(this: &EffectTiming, val: f64);
    #[cfg(feature = "FillMode")]
    #[doc = "Get the `fill` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EffectTiming`, `FillMode`*"]
    #[wasm_bindgen(method, getter = "fill")]
    pub fn get_fill(this: &EffectTiming) -> Option<FillMode>;
    #[cfg(feature = "FillMode")]
    #[doc = "Change the `fill` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EffectTiming`, `FillMode`*"]
    #[wasm_bindgen(method, setter = "fill")]
    pub fn set_fill(this: &EffectTiming, val: FillMode);
    #[doc = "Get the `iterationStart` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EffectTiming`*"]
    #[wasm_bindgen(method, getter = "iterationStart")]
    pub fn get_iteration_start(this: &EffectTiming) -> Option<f64>;
    #[doc = "Change the `iterationStart` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EffectTiming`*"]
    #[wasm_bindgen(method, setter = "iterationStart")]
    pub fn set_iteration_start(this: &EffectTiming, val: f64);
    #[doc = "Get the `iterations` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EffectTiming`*"]
    #[wasm_bindgen(method, getter = "iterations")]
    pub fn get_iterations(this: &EffectTiming) -> Option<f64>;
    #[doc = "Change the `iterations` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EffectTiming`*"]
    #[wasm_bindgen(method, setter = "iterations")]
    pub fn set_iterations(this: &EffectTiming, val: f64);
}
impl EffectTiming {
    #[doc = "Construct a new `EffectTiming`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EffectTiming`*"]
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
}
impl Default for EffectTiming {
    fn default() -> Self {
        Self::new()
    }
}
