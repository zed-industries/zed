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
        js_name = "ChannelSplitterNode",
        typescript_type = "ChannelSplitterNode"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ChannelSplitterNode` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ChannelSplitterNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelSplitterNode`*"]
    pub type ChannelSplitterNode;
    #[cfg(feature = "BaseAudioContext")]
    #[wasm_bindgen(catch, constructor, js_class = "ChannelSplitterNode")]
    #[doc = "The `new ChannelSplitterNode(..)` constructor, creating a new instance of `ChannelSplitterNode`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ChannelSplitterNode/ChannelSplitterNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseAudioContext`, `ChannelSplitterNode`*"]
    pub fn new(context: &BaseAudioContext) -> Result<ChannelSplitterNode, JsValue>;
    #[cfg(all(feature = "BaseAudioContext", feature = "ChannelSplitterOptions",))]
    #[wasm_bindgen(catch, constructor, js_class = "ChannelSplitterNode")]
    #[doc = "The `new ChannelSplitterNode(..)` constructor, creating a new instance of `ChannelSplitterNode`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ChannelSplitterNode/ChannelSplitterNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseAudioContext`, `ChannelSplitterNode`, `ChannelSplitterOptions`*"]
    pub fn new_with_options(
        context: &BaseAudioContext,
        options: &ChannelSplitterOptions,
    ) -> Result<ChannelSplitterNode, JsValue>;
}
