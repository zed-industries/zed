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
        js_name = "StereoPannerNode",
        typescript_type = "StereoPannerNode"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `StereoPannerNode` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StereoPannerNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StereoPannerNode`*"]
    pub type StereoPannerNode;
    #[cfg(feature = "AudioParam")]
    #[wasm_bindgen(method, getter, js_class = "StereoPannerNode", js_name = "pan")]
    #[doc = "Getter for the `pan` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StereoPannerNode/pan)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioParam`, `StereoPannerNode`*"]
    pub fn pan(this: &StereoPannerNode) -> AudioParam;
    #[cfg(feature = "BaseAudioContext")]
    #[wasm_bindgen(catch, constructor, js_class = "StereoPannerNode")]
    #[doc = "The `new StereoPannerNode(..)` constructor, creating a new instance of `StereoPannerNode`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StereoPannerNode/StereoPannerNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseAudioContext`, `StereoPannerNode`*"]
    pub fn new(context: &BaseAudioContext) -> Result<StereoPannerNode, JsValue>;
    #[cfg(all(feature = "BaseAudioContext", feature = "StereoPannerOptions",))]
    #[wasm_bindgen(catch, constructor, js_class = "StereoPannerNode")]
    #[doc = "The `new StereoPannerNode(..)` constructor, creating a new instance of `StereoPannerNode`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StereoPannerNode/StereoPannerNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseAudioContext`, `StereoPannerNode`, `StereoPannerOptions`*"]
    pub fn new_with_options(
        context: &BaseAudioContext,
        options: &StereoPannerOptions,
    ) -> Result<StereoPannerNode, JsValue>;
}
