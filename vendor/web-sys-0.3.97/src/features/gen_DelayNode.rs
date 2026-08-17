#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "AudioNode",
        extends = "EventTarget",
        extends = "::js_sys::Object",
        js_name = "DelayNode",
        typescript_type = "DelayNode"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DelayNode` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DelayNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DelayNode`*"]
    pub type DelayNode;
    #[cfg(feature = "AudioParam")]
    #[wasm_bindgen(method, getter, js_class = "DelayNode", js_name = "delayTime")]
    #[doc = "Getter for the `delayTime` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DelayNode/delayTime)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioParam`, `DelayNode`*"]
    pub fn delay_time(this: &DelayNode) -> AudioParam;
    #[cfg(feature = "BaseAudioContext")]
    #[wasm_bindgen(catch, constructor, js_class = "DelayNode")]
    #[doc = "The `new DelayNode(..)` constructor, creating a new instance of `DelayNode`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DelayNode/DelayNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseAudioContext`, `DelayNode`*"]
    pub fn new(context: &BaseAudioContext) -> Result<DelayNode, JsValue>;
    #[cfg(all(feature = "BaseAudioContext", feature = "DelayOptions",))]
    #[wasm_bindgen(catch, constructor, js_class = "DelayNode")]
    #[doc = "The `new DelayNode(..)` constructor, creating a new instance of `DelayNode`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DelayNode/DelayNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseAudioContext`, `DelayNode`, `DelayOptions`*"]
    pub fn new_with_options(
        context: &BaseAudioContext,
        options: &DelayOptions,
    ) -> Result<DelayNode, JsValue>;
}
