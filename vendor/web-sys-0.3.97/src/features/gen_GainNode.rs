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
        js_name = "GainNode",
        typescript_type = "GainNode"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GainNode` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GainNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GainNode`*"]
    pub type GainNode;
    #[cfg(feature = "AudioParam")]
    #[wasm_bindgen(method, getter, js_class = "GainNode", js_name = "gain")]
    #[doc = "Getter for the `gain` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GainNode/gain)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioParam`, `GainNode`*"]
    pub fn gain(this: &GainNode) -> AudioParam;
    #[cfg(feature = "BaseAudioContext")]
    #[wasm_bindgen(catch, constructor, js_class = "GainNode")]
    #[doc = "The `new GainNode(..)` constructor, creating a new instance of `GainNode`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GainNode/GainNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseAudioContext`, `GainNode`*"]
    pub fn new(context: &BaseAudioContext) -> Result<GainNode, JsValue>;
    #[cfg(all(feature = "BaseAudioContext", feature = "GainOptions",))]
    #[wasm_bindgen(catch, constructor, js_class = "GainNode")]
    #[doc = "The `new GainNode(..)` constructor, creating a new instance of `GainNode`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GainNode/GainNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseAudioContext`, `GainNode`, `GainOptions`*"]
    pub fn new_with_options(
        context: &BaseAudioContext,
        options: &GainOptions,
    ) -> Result<GainNode, JsValue>;
}
