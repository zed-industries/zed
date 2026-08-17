#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "AudioParamMap",
        typescript_type = "AudioParamMap"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AudioParamMap` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioParamMap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioParamMap`*"]
    pub type AudioParamMap;
    #[wasm_bindgen(method, getter, js_class = "AudioParamMap", js_name = "size")]
    #[doc = "Getter for the `size` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioParamMap/size)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioParamMap`*"]
    pub fn size(this: &AudioParamMap) -> u32;
    #[wasm_bindgen(catch, method, js_class = "AudioParamMap", js_name = "forEach")]
    #[doc = "The `forEach()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioParamMap/forEach)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioParamMap`*"]
    pub fn for_each(this: &AudioParamMap, callback: &::js_sys::Function) -> Result<(), JsValue>;
    #[cfg(feature = "AudioParam")]
    #[wasm_bindgen(method, js_class = "AudioParamMap")]
    #[doc = "The `get()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioParamMap/get)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioParam`, `AudioParamMap`*"]
    pub fn get(this: &AudioParamMap, key: &str) -> Option<AudioParam>;
    #[wasm_bindgen(method, js_class = "AudioParamMap")]
    #[doc = "The `has()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioParamMap/has)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioParamMap`*"]
    pub fn has(this: &AudioParamMap, key: &str) -> bool;
    #[wasm_bindgen(method, js_class = "AudioParamMap")]
    #[doc = "The `entries()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioParamMap/entries)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioParamMap`*"]
    pub fn entries(this: &AudioParamMap) -> ::js_sys::Iterator;
    #[wasm_bindgen(method, js_class = "AudioParamMap")]
    #[doc = "The `keys()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioParamMap/keys)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioParamMap`*"]
    pub fn keys(this: &AudioParamMap) -> ::js_sys::Iterator;
    #[wasm_bindgen(method, js_class = "AudioParamMap")]
    #[doc = "The `values()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioParamMap/values)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioParamMap`*"]
    pub fn values(this: &AudioParamMap) -> ::js_sys::Iterator;
}
