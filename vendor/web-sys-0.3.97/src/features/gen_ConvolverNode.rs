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
        js_name = "ConvolverNode",
        typescript_type = "ConvolverNode"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ConvolverNode` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ConvolverNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConvolverNode`*"]
    pub type ConvolverNode;
    #[cfg(feature = "AudioBuffer")]
    #[wasm_bindgen(method, getter, js_class = "ConvolverNode", js_name = "buffer")]
    #[doc = "Getter for the `buffer` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ConvolverNode/buffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBuffer`, `ConvolverNode`*"]
    pub fn buffer(this: &ConvolverNode) -> Option<AudioBuffer>;
    #[cfg(feature = "AudioBuffer")]
    #[wasm_bindgen(method, setter, js_class = "ConvolverNode", js_name = "buffer")]
    #[doc = "Setter for the `buffer` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ConvolverNode/buffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBuffer`, `ConvolverNode`*"]
    pub fn set_buffer(this: &ConvolverNode, value: Option<&AudioBuffer>);
    #[wasm_bindgen(method, getter, js_class = "ConvolverNode", js_name = "normalize")]
    #[doc = "Getter for the `normalize` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ConvolverNode/normalize)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConvolverNode`*"]
    pub fn normalize(this: &ConvolverNode) -> bool;
    #[wasm_bindgen(method, setter, js_class = "ConvolverNode", js_name = "normalize")]
    #[doc = "Setter for the `normalize` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ConvolverNode/normalize)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConvolverNode`*"]
    pub fn set_normalize(this: &ConvolverNode, value: bool);
    #[cfg(feature = "BaseAudioContext")]
    #[wasm_bindgen(catch, constructor, js_class = "ConvolverNode")]
    #[doc = "The `new ConvolverNode(..)` constructor, creating a new instance of `ConvolverNode`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ConvolverNode/ConvolverNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseAudioContext`, `ConvolverNode`*"]
    pub fn new(context: &BaseAudioContext) -> Result<ConvolverNode, JsValue>;
    #[cfg(all(feature = "BaseAudioContext", feature = "ConvolverOptions",))]
    #[wasm_bindgen(catch, constructor, js_class = "ConvolverNode")]
    #[doc = "The `new ConvolverNode(..)` constructor, creating a new instance of `ConvolverNode`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ConvolverNode/ConvolverNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseAudioContext`, `ConvolverNode`, `ConvolverOptions`*"]
    pub fn new_with_options(
        context: &BaseAudioContext,
        options: &ConvolverOptions,
    ) -> Result<ConvolverNode, JsValue>;
}
