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
        js_name = "DynamicsCompressorNode",
        typescript_type = "DynamicsCompressorNode"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DynamicsCompressorNode` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DynamicsCompressorNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DynamicsCompressorNode`*"]
    pub type DynamicsCompressorNode;
    #[cfg(feature = "AudioParam")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "DynamicsCompressorNode",
        js_name = "threshold"
    )]
    #[doc = "Getter for the `threshold` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DynamicsCompressorNode/threshold)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioParam`, `DynamicsCompressorNode`*"]
    pub fn threshold(this: &DynamicsCompressorNode) -> AudioParam;
    #[cfg(feature = "AudioParam")]
    #[wasm_bindgen(method, getter, js_class = "DynamicsCompressorNode", js_name = "knee")]
    #[doc = "Getter for the `knee` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DynamicsCompressorNode/knee)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioParam`, `DynamicsCompressorNode`*"]
    pub fn knee(this: &DynamicsCompressorNode) -> AudioParam;
    #[cfg(feature = "AudioParam")]
    #[wasm_bindgen(method, getter, js_class = "DynamicsCompressorNode", js_name = "ratio")]
    #[doc = "Getter for the `ratio` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DynamicsCompressorNode/ratio)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioParam`, `DynamicsCompressorNode`*"]
    pub fn ratio(this: &DynamicsCompressorNode) -> AudioParam;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "DynamicsCompressorNode",
        js_name = "reduction"
    )]
    #[doc = "Getter for the `reduction` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DynamicsCompressorNode/reduction)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DynamicsCompressorNode`*"]
    pub fn reduction(this: &DynamicsCompressorNode) -> f32;
    #[cfg(feature = "AudioParam")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "DynamicsCompressorNode",
        js_name = "attack"
    )]
    #[doc = "Getter for the `attack` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DynamicsCompressorNode/attack)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioParam`, `DynamicsCompressorNode`*"]
    pub fn attack(this: &DynamicsCompressorNode) -> AudioParam;
    #[cfg(feature = "AudioParam")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "DynamicsCompressorNode",
        js_name = "release"
    )]
    #[doc = "Getter for the `release` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DynamicsCompressorNode/release)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioParam`, `DynamicsCompressorNode`*"]
    pub fn release(this: &DynamicsCompressorNode) -> AudioParam;
    #[cfg(feature = "BaseAudioContext")]
    #[wasm_bindgen(catch, constructor, js_class = "DynamicsCompressorNode")]
    #[doc = "The `new DynamicsCompressorNode(..)` constructor, creating a new instance of `DynamicsCompressorNode`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DynamicsCompressorNode/DynamicsCompressorNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseAudioContext`, `DynamicsCompressorNode`*"]
    pub fn new(context: &BaseAudioContext) -> Result<DynamicsCompressorNode, JsValue>;
    #[cfg(all(feature = "BaseAudioContext", feature = "DynamicsCompressorOptions",))]
    #[wasm_bindgen(catch, constructor, js_class = "DynamicsCompressorNode")]
    #[doc = "The `new DynamicsCompressorNode(..)` constructor, creating a new instance of `DynamicsCompressorNode`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DynamicsCompressorNode/DynamicsCompressorNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseAudioContext`, `DynamicsCompressorNode`, `DynamicsCompressorOptions`*"]
    pub fn new_with_options(
        context: &BaseAudioContext,
        options: &DynamicsCompressorOptions,
    ) -> Result<DynamicsCompressorNode, JsValue>;
}
