#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "SpeechRecognitionAlternative",
        typescript_type = "SpeechRecognitionAlternative"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SpeechRecognitionAlternative` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognitionAlternative)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionAlternative`*"]
    pub type SpeechRecognitionAlternative;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SpeechRecognitionAlternative",
        js_name = "transcript"
    )]
    #[doc = "Getter for the `transcript` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognitionAlternative/transcript)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionAlternative`*"]
    pub fn transcript(this: &SpeechRecognitionAlternative) -> ::alloc::string::String;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SpeechRecognitionAlternative",
        js_name = "confidence"
    )]
    #[doc = "Getter for the `confidence` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognitionAlternative/confidence)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionAlternative`*"]
    pub fn confidence(this: &SpeechRecognitionAlternative) -> f32;
}
