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
        js_name = "WaveShaperNode",
        typescript_type = "WaveShaperNode"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WaveShaperNode` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WaveShaperNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WaveShaperNode`*"]
    pub type WaveShaperNode;
    #[wasm_bindgen(method, getter, js_class = "WaveShaperNode", js_name = "curve")]
    #[doc = "Getter for the `curve` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WaveShaperNode/curve)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WaveShaperNode`*"]
    pub fn curve(this: &WaveShaperNode) -> Option<::alloc::vec::Vec<f32>>;
    #[wasm_bindgen(method, setter, js_class = "WaveShaperNode", js_name = "curve")]
    #[doc = "Setter for the `curve` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WaveShaperNode/curve)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WaveShaperNode`*"]
    #[deprecated]
    pub fn set_curve(this: &WaveShaperNode, value: Option<&mut [f32]>);
    #[wasm_bindgen(method, setter, js_class = "WaveShaperNode", js_name = "curve")]
    #[doc = "Setter for the `curve` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WaveShaperNode/curve)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WaveShaperNode`*"]
    pub fn set_curve_opt_f32_slice(this: &WaveShaperNode, value: Option<&mut [f32]>);
    #[wasm_bindgen(method, setter, js_class = "WaveShaperNode", js_name = "curve")]
    #[doc = "Setter for the `curve` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WaveShaperNode/curve)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WaveShaperNode`*"]
    pub fn set_curve_opt_f32_array(this: &WaveShaperNode, value: Option<&::js_sys::Float32Array>);
    #[cfg(feature = "OverSampleType")]
    #[wasm_bindgen(method, getter, js_class = "WaveShaperNode", js_name = "oversample")]
    #[doc = "Getter for the `oversample` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WaveShaperNode/oversample)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OverSampleType`, `WaveShaperNode`*"]
    pub fn oversample(this: &WaveShaperNode) -> OverSampleType;
    #[cfg(feature = "OverSampleType")]
    #[wasm_bindgen(method, setter, js_class = "WaveShaperNode", js_name = "oversample")]
    #[doc = "Setter for the `oversample` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WaveShaperNode/oversample)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OverSampleType`, `WaveShaperNode`*"]
    pub fn set_oversample(this: &WaveShaperNode, value: OverSampleType);
    #[cfg(feature = "BaseAudioContext")]
    #[wasm_bindgen(catch, constructor, js_class = "WaveShaperNode")]
    #[doc = "The `new WaveShaperNode(..)` constructor, creating a new instance of `WaveShaperNode`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WaveShaperNode/WaveShaperNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseAudioContext`, `WaveShaperNode`*"]
    pub fn new(context: &BaseAudioContext) -> Result<WaveShaperNode, JsValue>;
    #[cfg(all(feature = "BaseAudioContext", feature = "WaveShaperOptions",))]
    #[wasm_bindgen(catch, constructor, js_class = "WaveShaperNode")]
    #[doc = "The `new WaveShaperNode(..)` constructor, creating a new instance of `WaveShaperNode`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WaveShaperNode/WaveShaperNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseAudioContext`, `WaveShaperNode`, `WaveShaperOptions`*"]
    pub fn new_with_options(
        context: &BaseAudioContext,
        options: &WaveShaperOptions,
    ) -> Result<WaveShaperNode, JsValue>;
}
