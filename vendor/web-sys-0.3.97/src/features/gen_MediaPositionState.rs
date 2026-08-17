#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "MediaPositionState")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaPositionState` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaPositionState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type MediaPositionState;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `duration` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaPositionState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "duration")]
    pub fn get_duration(this: &MediaPositionState) -> Option<f64>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `duration` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaPositionState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "duration")]
    pub fn set_duration(this: &MediaPositionState, val: f64);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `playbackRate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaPositionState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "playbackRate")]
    pub fn get_playback_rate(this: &MediaPositionState) -> Option<f64>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `playbackRate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaPositionState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "playbackRate")]
    pub fn set_playback_rate(this: &MediaPositionState, val: f64);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `position` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaPositionState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "position")]
    pub fn get_position(this: &MediaPositionState) -> Option<f64>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `position` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaPositionState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "position")]
    pub fn set_position(this: &MediaPositionState, val: f64);
}
#[cfg(web_sys_unstable_apis)]
impl MediaPositionState {
    #[doc = "Construct a new `MediaPositionState`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaPositionState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_duration()` instead."]
    pub fn duration(&mut self, val: f64) -> &mut Self {
        self.set_duration(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_playback_rate()` instead."]
    pub fn playback_rate(&mut self, val: f64) -> &mut Self {
        self.set_playback_rate(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_position()` instead."]
    pub fn position(&mut self, val: f64) -> &mut Self {
        self.set_position(val);
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for MediaPositionState {
    fn default() -> Self {
        Self::new()
    }
}
