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
        js_name = "ChannelMergerNode",
        typescript_type = "ChannelMergerNode"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ChannelMergerNode` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ChannelMergerNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelMergerNode`*"]
    pub type ChannelMergerNode;
    #[cfg(feature = "BaseAudioContext")]
    #[wasm_bindgen(catch, constructor, js_class = "ChannelMergerNode")]
    #[doc = "The `new ChannelMergerNode(..)` constructor, creating a new instance of `ChannelMergerNode`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ChannelMergerNode/ChannelMergerNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseAudioContext`, `ChannelMergerNode`*"]
    pub fn new(context: &BaseAudioContext) -> Result<ChannelMergerNode, JsValue>;
    #[cfg(all(feature = "BaseAudioContext", feature = "ChannelMergerOptions",))]
    #[wasm_bindgen(catch, constructor, js_class = "ChannelMergerNode")]
    #[doc = "The `new ChannelMergerNode(..)` constructor, creating a new instance of `ChannelMergerNode`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ChannelMergerNode/ChannelMergerNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseAudioContext`, `ChannelMergerNode`, `ChannelMergerOptions`*"]
    pub fn new_with_options(
        context: &BaseAudioContext,
        options: &ChannelMergerOptions,
    ) -> Result<ChannelMergerNode, JsValue>;
}
