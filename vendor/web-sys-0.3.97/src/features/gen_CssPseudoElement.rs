#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "CSSPseudoElement",
        typescript_type = "CSSPseudoElement"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CssPseudoElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSPseudoElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssPseudoElement`*"]
    pub type CssPseudoElement;
    #[wasm_bindgen(method, getter, js_class = "CSSPseudoElement", js_name = "type")]
    #[doc = "Getter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSPseudoElement/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssPseudoElement`*"]
    pub fn type_(this: &CssPseudoElement) -> ::alloc::string::String;
    #[cfg(feature = "Element")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "CSSPseudoElement",
        js_name = "parentElement"
    )]
    #[doc = "Getter for the `parentElement` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSPseudoElement/parentElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssPseudoElement`, `Element`*"]
    pub fn parent_element(this: &CssPseudoElement) -> Element;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "Animation")]
    #[wasm_bindgen(method, js_class = "CSSPseudoElement")]
    #[doc = "The `animate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSPseudoElement/animate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Animation`, `CssPseudoElement`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn animate(this: &CssPseudoElement, keyframes: Option<&::js_sys::Object>) -> Animation;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "Animation")]
    #[wasm_bindgen(method, js_class = "CSSPseudoElement", js_name = "animate")]
    #[doc = "The `animate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSPseudoElement/animate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Animation`, `CssPseudoElement`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn animate_with_f64(
        this: &CssPseudoElement,
        keyframes: Option<&::js_sys::Object>,
        options: f64,
    ) -> Animation;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(feature = "Animation", feature = "KeyframeAnimationOptions",))]
    #[wasm_bindgen(method, js_class = "CSSPseudoElement", js_name = "animate")]
    #[doc = "The `animate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSPseudoElement/animate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Animation`, `CssPseudoElement`, `KeyframeAnimationOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn animate_with_keyframe_animation_options(
        this: &CssPseudoElement,
        keyframes: Option<&::js_sys::Object>,
        options: &KeyframeAnimationOptions,
    ) -> Animation;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "Animation")]
    #[wasm_bindgen(method, js_class = "CSSPseudoElement", js_name = "getAnimations")]
    #[doc = "The `getAnimations()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSPseudoElement/getAnimations)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Animation`, `CssPseudoElement`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn get_animations(this: &CssPseudoElement) -> ::js_sys::Array<Animation>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(feature = "Animation", feature = "GetAnimationsOptions",))]
    #[wasm_bindgen(method, js_class = "CSSPseudoElement", js_name = "getAnimations")]
    #[doc = "The `getAnimations()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSPseudoElement/getAnimations)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Animation`, `CssPseudoElement`, `GetAnimationsOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn get_animations_with_options(
        this: &CssPseudoElement,
        options: &GetAnimationsOptions,
    ) -> ::js_sys::Array<Animation>;
}
