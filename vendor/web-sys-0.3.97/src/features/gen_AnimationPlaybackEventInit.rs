#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "AnimationPlaybackEventInit")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AnimationPlaybackEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationPlaybackEventInit`*"]
    pub type AnimationPlaybackEventInit;
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationPlaybackEventInit`*"]
    #[wasm_bindgen(method, getter = "bubbles")]
    pub fn get_bubbles(this: &AnimationPlaybackEventInit) -> Option<bool>;
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationPlaybackEventInit`*"]
    #[wasm_bindgen(method, setter = "bubbles")]
    pub fn set_bubbles(this: &AnimationPlaybackEventInit, val: bool);
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationPlaybackEventInit`*"]
    #[wasm_bindgen(method, getter = "cancelable")]
    pub fn get_cancelable(this: &AnimationPlaybackEventInit) -> Option<bool>;
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationPlaybackEventInit`*"]
    #[wasm_bindgen(method, setter = "cancelable")]
    pub fn set_cancelable(this: &AnimationPlaybackEventInit, val: bool);
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationPlaybackEventInit`*"]
    #[wasm_bindgen(method, getter = "composed")]
    pub fn get_composed(this: &AnimationPlaybackEventInit) -> Option<bool>;
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationPlaybackEventInit`*"]
    #[wasm_bindgen(method, setter = "composed")]
    pub fn set_composed(this: &AnimationPlaybackEventInit, val: bool);
    #[doc = "Get the `currentTime` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationPlaybackEventInit`*"]
    #[wasm_bindgen(method, getter = "currentTime")]
    pub fn get_current_time(this: &AnimationPlaybackEventInit) -> Option<f64>;
    #[doc = "Change the `currentTime` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationPlaybackEventInit`*"]
    #[wasm_bindgen(method, setter = "currentTime")]
    pub fn set_current_time(this: &AnimationPlaybackEventInit, val: Option<f64>);
    #[doc = "Get the `timelineTime` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationPlaybackEventInit`*"]
    #[wasm_bindgen(method, getter = "timelineTime")]
    pub fn get_timeline_time(this: &AnimationPlaybackEventInit) -> Option<f64>;
    #[doc = "Change the `timelineTime` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationPlaybackEventInit`*"]
    #[wasm_bindgen(method, setter = "timelineTime")]
    pub fn set_timeline_time(this: &AnimationPlaybackEventInit, val: Option<f64>);
}
impl AnimationPlaybackEventInit {
    #[doc = "Construct a new `AnimationPlaybackEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationPlaybackEventInit`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_bubbles()` instead."]
    pub fn bubbles(&mut self, val: bool) -> &mut Self {
        self.set_bubbles(val);
        self
    }
    #[deprecated = "Use `set_cancelable()` instead."]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.set_cancelable(val);
        self
    }
    #[deprecated = "Use `set_composed()` instead."]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.set_composed(val);
        self
    }
    #[deprecated = "Use `set_current_time()` instead."]
    pub fn current_time(&mut self, val: Option<f64>) -> &mut Self {
        self.set_current_time(val);
        self
    }
    #[deprecated = "Use `set_timeline_time()` instead."]
    pub fn timeline_time(&mut self, val: Option<f64>) -> &mut Self {
        self.set_timeline_time(val);
        self
    }
}
impl Default for AnimationPlaybackEventInit {
    fn default() -> Self {
        Self::new()
    }
}
