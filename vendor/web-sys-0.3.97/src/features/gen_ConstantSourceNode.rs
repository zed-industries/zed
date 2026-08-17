#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "AudioScheduledSourceNode",
        extends = "AudioNode",
        extends = "EventTarget",
        extends = "::js_sys::Object",
        js_name = "ConstantSourceNode",
        typescript_type = "ConstantSourceNode"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ConstantSourceNode` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ConstantSourceNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstantSourceNode`*"]
    pub type ConstantSourceNode;
    #[cfg(feature = "AudioParam")]
    #[wasm_bindgen(method, getter, js_class = "ConstantSourceNode", js_name = "offset")]
    #[doc = "Getter for the `offset` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ConstantSourceNode/offset)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioParam`, `ConstantSourceNode`*"]
    pub fn offset(this: &ConstantSourceNode) -> AudioParam;
    #[wasm_bindgen(method, getter, js_class = "ConstantSourceNode", js_name = "onended")]
    #[doc = "Getter for the `onended` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ConstantSourceNode/onended)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstantSourceNode`*"]
    pub fn onended(this: &ConstantSourceNode) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "ConstantSourceNode", js_name = "onended")]
    #[doc = "Setter for the `onended` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ConstantSourceNode/onended)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstantSourceNode`*"]
    pub fn set_onended(this: &ConstantSourceNode, value: Option<&::js_sys::Function>);
    #[cfg(feature = "BaseAudioContext")]
    #[wasm_bindgen(catch, constructor, js_class = "ConstantSourceNode")]
    #[doc = "The `new ConstantSourceNode(..)` constructor, creating a new instance of `ConstantSourceNode`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ConstantSourceNode/ConstantSourceNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseAudioContext`, `ConstantSourceNode`*"]
    pub fn new(context: &BaseAudioContext) -> Result<ConstantSourceNode, JsValue>;
    #[cfg(all(feature = "BaseAudioContext", feature = "ConstantSourceOptions",))]
    #[wasm_bindgen(catch, constructor, js_class = "ConstantSourceNode")]
    #[doc = "The `new ConstantSourceNode(..)` constructor, creating a new instance of `ConstantSourceNode`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ConstantSourceNode/ConstantSourceNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseAudioContext`, `ConstantSourceNode`, `ConstantSourceOptions`*"]
    pub fn new_with_options(
        context: &BaseAudioContext,
        options: &ConstantSourceOptions,
    ) -> Result<ConstantSourceNode, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "ConstantSourceNode")]
    #[doc = "The `start()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ConstantSourceNode/start)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstantSourceNode`*"]
    pub fn start(this: &ConstantSourceNode) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "ConstantSourceNode", js_name = "start")]
    #[doc = "The `start()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ConstantSourceNode/start)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstantSourceNode`*"]
    pub fn start_with_when(this: &ConstantSourceNode, when: f64) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "ConstantSourceNode")]
    #[doc = "The `stop()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ConstantSourceNode/stop)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstantSourceNode`*"]
    pub fn stop(this: &ConstantSourceNode) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "ConstantSourceNode", js_name = "stop")]
    #[doc = "The `stop()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ConstantSourceNode/stop)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstantSourceNode`*"]
    pub fn stop_with_when(this: &ConstantSourceNode, when: f64) -> Result<(), JsValue>;
}
