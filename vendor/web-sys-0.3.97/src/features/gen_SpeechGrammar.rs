#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "SpeechGrammar",
        typescript_type = "SpeechGrammar"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SpeechGrammar` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechGrammar)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechGrammar`*"]
    pub type SpeechGrammar;
    #[wasm_bindgen(catch, method, getter, js_class = "SpeechGrammar", js_name = "src")]
    #[doc = "Getter for the `src` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechGrammar/src)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechGrammar`*"]
    pub fn src(this: &SpeechGrammar) -> Result<::alloc::string::String, JsValue>;
    #[wasm_bindgen(catch, method, setter, js_class = "SpeechGrammar", js_name = "src")]
    #[doc = "Setter for the `src` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechGrammar/src)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechGrammar`*"]
    pub fn set_src(this: &SpeechGrammar, value: &str) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, getter, js_class = "SpeechGrammar", js_name = "weight")]
    #[doc = "Getter for the `weight` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechGrammar/weight)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechGrammar`*"]
    pub fn weight(this: &SpeechGrammar) -> Result<f32, JsValue>;
    #[wasm_bindgen(catch, method, setter, js_class = "SpeechGrammar", js_name = "weight")]
    #[doc = "Setter for the `weight` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechGrammar/weight)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechGrammar`*"]
    pub fn set_weight(this: &SpeechGrammar, value: f32) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, constructor, js_class = "SpeechGrammar")]
    #[doc = "The `new SpeechGrammar(..)` constructor, creating a new instance of `SpeechGrammar`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechGrammar/SpeechGrammar)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechGrammar`*"]
    pub fn new() -> Result<SpeechGrammar, JsValue>;
}
