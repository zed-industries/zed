#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "Event",
        extends = "::js_sys::Object",
        js_name = "AnimationEvent",
        typescript_type = "AnimationEvent"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AnimationEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnimationEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationEvent`*"]
    pub type AnimationEvent;
    #[wasm_bindgen(method, getter, js_class = "AnimationEvent", js_name = "animationName")]
    #[doc = "Getter for the `animationName` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnimationEvent/animationName)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationEvent`*"]
    pub fn animation_name(this: &AnimationEvent) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "AnimationEvent", js_name = "elapsedTime")]
    #[doc = "Getter for the `elapsedTime` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnimationEvent/elapsedTime)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationEvent`*"]
    pub fn elapsed_time(this: &AnimationEvent) -> f32;
    #[wasm_bindgen(method, getter, js_class = "AnimationEvent", js_name = "pseudoElement")]
    #[doc = "Getter for the `pseudoElement` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnimationEvent/pseudoElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationEvent`*"]
    pub fn pseudo_element(this: &AnimationEvent) -> ::alloc::string::String;
    #[wasm_bindgen(catch, constructor, js_class = "AnimationEvent")]
    #[doc = "The `new AnimationEvent(..)` constructor, creating a new instance of `AnimationEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnimationEvent/AnimationEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationEvent`*"]
    pub fn new(type_: &str) -> Result<AnimationEvent, JsValue>;
    #[cfg(feature = "AnimationEventInit")]
    #[wasm_bindgen(catch, constructor, js_class = "AnimationEvent")]
    #[doc = "The `new AnimationEvent(..)` constructor, creating a new instance of `AnimationEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnimationEvent/AnimationEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationEvent`, `AnimationEventInit`*"]
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &AnimationEventInit,
    ) -> Result<AnimationEvent, JsValue>;
}
