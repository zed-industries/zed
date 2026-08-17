#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "GamepadHapticActuator",
        typescript_type = "GamepadHapticActuator"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GamepadHapticActuator` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadHapticActuator)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GamepadHapticActuator`*"]
    pub type GamepadHapticActuator;
    #[cfg(feature = "GamepadHapticActuatorType")]
    #[wasm_bindgen(method, getter, js_class = "GamepadHapticActuator", js_name = "type")]
    #[doc = "Getter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadHapticActuator/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GamepadHapticActuator`, `GamepadHapticActuatorType`*"]
    #[deprecated]
    pub fn type_(this: &GamepadHapticActuator) -> GamepadHapticActuatorType;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "GamepadHapticActuator",
        js_name = "effects"
    )]
    #[doc = "Getter for the `effects` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadHapticActuator/effects)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GamepadHapticActuator`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn effects(this: &GamepadHapticActuator) -> ::js_sys::Array<::js_sys::JsString>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GamepadHapticEffectType")]
    #[wasm_bindgen(method, js_class = "GamepadHapticActuator", js_name = "playEffect")]
    #[doc = "The `playEffect()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadHapticActuator/playEffect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GamepadHapticActuator`, `GamepadHapticEffectType`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn play_effect(
        this: &GamepadHapticActuator,
        type_: GamepadHapticEffectType,
    ) -> ::js_sys::Promise<::js_sys::JsString>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(
        feature = "GamepadEffectParameters",
        feature = "GamepadHapticEffectType",
    ))]
    #[wasm_bindgen(method, js_class = "GamepadHapticActuator", js_name = "playEffect")]
    #[doc = "The `playEffect()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadHapticActuator/playEffect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GamepadEffectParameters`, `GamepadHapticActuator`, `GamepadHapticEffectType`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn play_effect_with_params(
        this: &GamepadHapticActuator,
        type_: GamepadHapticEffectType,
        params: &GamepadEffectParameters,
    ) -> ::js_sys::Promise<::js_sys::JsString>;
    #[wasm_bindgen(catch, method, js_class = "GamepadHapticActuator")]
    #[doc = "The `pulse()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadHapticActuator/pulse)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GamepadHapticActuator`*"]
    pub fn pulse(
        this: &GamepadHapticActuator,
        value: f64,
        duration: f64,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, js_class = "GamepadHapticActuator")]
    #[doc = "The `reset()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadHapticActuator/reset)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GamepadHapticActuator`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn reset(this: &GamepadHapticActuator) -> ::js_sys::Promise<::js_sys::JsString>;
}
