#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "AnimationEffect",
        typescript_type = "AnimationEffect"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AnimationEffect` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnimationEffect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationEffect`*"]
    pub type AnimationEffect;
    #[cfg(feature = "ComputedEffectTiming")]
    #[wasm_bindgen(method, js_class = "AnimationEffect", js_name = "getComputedTiming")]
    #[doc = "The `getComputedTiming()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnimationEffect/getComputedTiming)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationEffect`, `ComputedEffectTiming`*"]
    pub fn get_computed_timing(this: &AnimationEffect) -> ComputedEffectTiming;
    #[cfg(feature = "EffectTiming")]
    #[wasm_bindgen(method, js_class = "AnimationEffect", js_name = "getTiming")]
    #[doc = "The `getTiming()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnimationEffect/getTiming)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationEffect`, `EffectTiming`*"]
    pub fn get_timing(this: &AnimationEffect) -> EffectTiming;
    #[wasm_bindgen(catch, method, js_class = "AnimationEffect", js_name = "updateTiming")]
    #[doc = "The `updateTiming()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnimationEffect/updateTiming)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationEffect`*"]
    pub fn update_timing(this: &AnimationEffect) -> Result<(), JsValue>;
    #[cfg(feature = "OptionalEffectTiming")]
    #[wasm_bindgen(catch, method, js_class = "AnimationEffect", js_name = "updateTiming")]
    #[doc = "The `updateTiming()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnimationEffect/updateTiming)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationEffect`, `OptionalEffectTiming`*"]
    pub fn update_timing_with_timing(
        this: &AnimationEffect,
        timing: &OptionalEffectTiming,
    ) -> Result<(), JsValue>;
}
