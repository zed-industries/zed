#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "AnimationEffect",
        extends = "::js_sys::Object",
        js_name = "KeyframeEffect",
        typescript_type = "KeyframeEffect"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `KeyframeEffect` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyframeEffect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyframeEffect`*"]
    pub type KeyframeEffect;
    #[wasm_bindgen(method, getter, js_class = "KeyframeEffect", js_name = "target")]
    #[doc = "Getter for the `target` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyframeEffect/target)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyframeEffect`*"]
    pub fn target(this: &KeyframeEffect) -> Option<::js_sys::Object>;
    #[wasm_bindgen(method, setter, js_class = "KeyframeEffect", js_name = "target")]
    #[doc = "Setter for the `target` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyframeEffect/target)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyframeEffect`*"]
    #[deprecated]
    pub fn set_target(this: &KeyframeEffect, value: Option<&::js_sys::Object>);
    #[cfg(feature = "Element")]
    #[wasm_bindgen(method, setter, js_class = "KeyframeEffect", js_name = "target")]
    #[doc = "Setter for the `target` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyframeEffect/target)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`, `KeyframeEffect`*"]
    pub fn set_target_opt_element(this: &KeyframeEffect, value: Option<&Element>);
    #[cfg(feature = "CssPseudoElement")]
    #[wasm_bindgen(method, setter, js_class = "KeyframeEffect", js_name = "target")]
    #[doc = "Setter for the `target` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyframeEffect/target)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssPseudoElement`, `KeyframeEffect`*"]
    pub fn set_target_opt_css_pseudo_element(
        this: &KeyframeEffect,
        value: Option<&CssPseudoElement>,
    );
    #[cfg(feature = "IterationCompositeOperation")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "KeyframeEffect",
        js_name = "iterationComposite"
    )]
    #[doc = "Getter for the `iterationComposite` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyframeEffect/iterationComposite)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IterationCompositeOperation`, `KeyframeEffect`*"]
    pub fn iteration_composite(this: &KeyframeEffect) -> IterationCompositeOperation;
    #[cfg(feature = "IterationCompositeOperation")]
    #[wasm_bindgen(
        method,
        setter,
        js_class = "KeyframeEffect",
        js_name = "iterationComposite"
    )]
    #[doc = "Setter for the `iterationComposite` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyframeEffect/iterationComposite)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IterationCompositeOperation`, `KeyframeEffect`*"]
    pub fn set_iteration_composite(this: &KeyframeEffect, value: IterationCompositeOperation);
    #[cfg(feature = "CompositeOperation")]
    #[wasm_bindgen(method, getter, js_class = "KeyframeEffect", js_name = "composite")]
    #[doc = "Getter for the `composite` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyframeEffect/composite)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CompositeOperation`, `KeyframeEffect`*"]
    pub fn composite(this: &KeyframeEffect) -> CompositeOperation;
    #[cfg(feature = "CompositeOperation")]
    #[wasm_bindgen(method, setter, js_class = "KeyframeEffect", js_name = "composite")]
    #[doc = "Setter for the `composite` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyframeEffect/composite)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CompositeOperation`, `KeyframeEffect`*"]
    pub fn set_composite(this: &KeyframeEffect, value: CompositeOperation);
    #[cfg(feature = "Element")]
    #[wasm_bindgen(catch, constructor, js_class = "KeyframeEffect")]
    #[doc = "The `new KeyframeEffect(..)` constructor, creating a new instance of `KeyframeEffect`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyframeEffect/KeyframeEffect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`, `KeyframeEffect`*"]
    pub fn new_with_opt_element_and_keyframes(
        target: Option<&Element>,
        keyframes: Option<&::js_sys::Object>,
    ) -> Result<KeyframeEffect, JsValue>;
    #[cfg(feature = "CssPseudoElement")]
    #[wasm_bindgen(catch, constructor, js_class = "KeyframeEffect")]
    #[doc = "The `new KeyframeEffect(..)` constructor, creating a new instance of `KeyframeEffect`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyframeEffect/KeyframeEffect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssPseudoElement`, `KeyframeEffect`*"]
    pub fn new_with_opt_css_pseudo_element_and_keyframes(
        target: Option<&CssPseudoElement>,
        keyframes: Option<&::js_sys::Object>,
    ) -> Result<KeyframeEffect, JsValue>;
    #[cfg(feature = "Element")]
    #[wasm_bindgen(catch, constructor, js_class = "KeyframeEffect")]
    #[doc = "The `new KeyframeEffect(..)` constructor, creating a new instance of `KeyframeEffect`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyframeEffect/KeyframeEffect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`, `KeyframeEffect`*"]
    pub fn new_with_opt_element_and_keyframes_and_f64(
        target: Option<&Element>,
        keyframes: Option<&::js_sys::Object>,
        options: f64,
    ) -> Result<KeyframeEffect, JsValue>;
    #[cfg(feature = "CssPseudoElement")]
    #[wasm_bindgen(catch, constructor, js_class = "KeyframeEffect")]
    #[doc = "The `new KeyframeEffect(..)` constructor, creating a new instance of `KeyframeEffect`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyframeEffect/KeyframeEffect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssPseudoElement`, `KeyframeEffect`*"]
    pub fn new_with_opt_css_pseudo_element_and_keyframes_and_f64(
        target: Option<&CssPseudoElement>,
        keyframes: Option<&::js_sys::Object>,
        options: f64,
    ) -> Result<KeyframeEffect, JsValue>;
    #[cfg(all(feature = "Element", feature = "KeyframeEffectOptions",))]
    #[wasm_bindgen(catch, constructor, js_class = "KeyframeEffect")]
    #[doc = "The `new KeyframeEffect(..)` constructor, creating a new instance of `KeyframeEffect`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyframeEffect/KeyframeEffect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`, `KeyframeEffect`, `KeyframeEffectOptions`*"]
    pub fn new_with_opt_element_and_keyframes_and_keyframe_effect_options(
        target: Option<&Element>,
        keyframes: Option<&::js_sys::Object>,
        options: &KeyframeEffectOptions,
    ) -> Result<KeyframeEffect, JsValue>;
    #[cfg(all(feature = "CssPseudoElement", feature = "KeyframeEffectOptions",))]
    #[wasm_bindgen(catch, constructor, js_class = "KeyframeEffect")]
    #[doc = "The `new KeyframeEffect(..)` constructor, creating a new instance of `KeyframeEffect`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyframeEffect/KeyframeEffect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssPseudoElement`, `KeyframeEffect`, `KeyframeEffectOptions`*"]
    pub fn new_with_opt_css_pseudo_element_and_keyframes_and_keyframe_effect_options(
        target: Option<&CssPseudoElement>,
        keyframes: Option<&::js_sys::Object>,
        options: &KeyframeEffectOptions,
    ) -> Result<KeyframeEffect, JsValue>;
    #[wasm_bindgen(catch, constructor, js_class = "KeyframeEffect")]
    #[doc = "The `new KeyframeEffect(..)` constructor, creating a new instance of `KeyframeEffect`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyframeEffect/KeyframeEffect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyframeEffect`*"]
    pub fn new_with_source(source: &KeyframeEffect) -> Result<KeyframeEffect, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "KeyframeEffect", js_name = "getKeyframes")]
    #[doc = "The `getKeyframes()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyframeEffect/getKeyframes)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyframeEffect`*"]
    pub fn get_keyframes(this: &KeyframeEffect) -> Result<::js_sys::Array, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "KeyframeEffect", js_name = "setKeyframes")]
    #[doc = "The `setKeyframes()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyframeEffect/setKeyframes)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyframeEffect`*"]
    pub fn set_keyframes(
        this: &KeyframeEffect,
        keyframes: Option<&::js_sys::Object>,
    ) -> Result<(), JsValue>;
}
