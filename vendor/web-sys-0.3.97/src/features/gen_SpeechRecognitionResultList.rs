#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "SpeechRecognitionResultList",
        typescript_type = "SpeechRecognitionResultList"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SpeechRecognitionResultList` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognitionResultList)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionResultList`*"]
    pub type SpeechRecognitionResultList;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SpeechRecognitionResultList",
        js_name = "length"
    )]
    #[doc = "Getter for the `length` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognitionResultList/length)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionResultList`*"]
    pub fn length(this: &SpeechRecognitionResultList) -> u32;
    #[cfg(feature = "SpeechRecognitionResult")]
    #[wasm_bindgen(method, js_class = "SpeechRecognitionResultList")]
    #[doc = "The `item()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognitionResultList/item)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionResult`, `SpeechRecognitionResultList`*"]
    pub fn item(this: &SpeechRecognitionResultList, index: u32) -> SpeechRecognitionResult;
    #[cfg(feature = "SpeechRecognitionResult")]
    #[wasm_bindgen(method, js_class = "SpeechRecognitionResultList", indexing_getter)]
    #[doc = "Indexing getter. As in the literal Javascript `this[key]`."]
    #[doc = ""]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionResult`, `SpeechRecognitionResultList`*"]
    pub fn get(this: &SpeechRecognitionResultList, index: u32) -> Option<SpeechRecognitionResult>;
}
