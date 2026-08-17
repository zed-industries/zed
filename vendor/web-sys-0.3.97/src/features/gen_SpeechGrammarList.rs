#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "SpeechGrammarList",
        typescript_type = "SpeechGrammarList"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SpeechGrammarList` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechGrammarList)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechGrammarList`*"]
    pub type SpeechGrammarList;
    #[wasm_bindgen(method, getter, js_class = "SpeechGrammarList", js_name = "length")]
    #[doc = "Getter for the `length` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechGrammarList/length)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechGrammarList`*"]
    pub fn length(this: &SpeechGrammarList) -> u32;
    #[wasm_bindgen(catch, constructor, js_class = "SpeechGrammarList")]
    #[doc = "The `new SpeechGrammarList(..)` constructor, creating a new instance of `SpeechGrammarList`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechGrammarList/SpeechGrammarList)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechGrammarList`*"]
    pub fn new() -> Result<SpeechGrammarList, JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "SpeechGrammarList",
        js_name = "addFromString"
    )]
    #[doc = "The `addFromString()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechGrammarList/addFromString)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechGrammarList`*"]
    pub fn add_from_string(this: &SpeechGrammarList, string: &str) -> Result<(), JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "SpeechGrammarList",
        js_name = "addFromString"
    )]
    #[doc = "The `addFromString()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechGrammarList/addFromString)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechGrammarList`*"]
    pub fn add_from_string_with_weight(
        this: &SpeechGrammarList,
        string: &str,
        weight: f32,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "SpeechGrammarList", js_name = "addFromURI")]
    #[doc = "The `addFromURI()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechGrammarList/addFromURI)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechGrammarList`*"]
    pub fn add_from_uri(this: &SpeechGrammarList, src: &str) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "SpeechGrammarList", js_name = "addFromURI")]
    #[doc = "The `addFromURI()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechGrammarList/addFromURI)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechGrammarList`*"]
    pub fn add_from_uri_with_weight(
        this: &SpeechGrammarList,
        src: &str,
        weight: f32,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "SpeechGrammar")]
    #[wasm_bindgen(catch, method, js_class = "SpeechGrammarList")]
    #[doc = "The `item()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechGrammarList/item)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechGrammar`, `SpeechGrammarList`*"]
    pub fn item(this: &SpeechGrammarList, index: u32) -> Result<SpeechGrammar, JsValue>;
    #[cfg(feature = "SpeechGrammar")]
    #[wasm_bindgen(catch, method, js_class = "SpeechGrammarList", indexing_getter)]
    #[doc = "Indexing getter. As in the literal Javascript `this[key]`."]
    #[doc = ""]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechGrammar`, `SpeechGrammarList`*"]
    pub fn get(this: &SpeechGrammarList, index: u32) -> Result<SpeechGrammar, JsValue>;
}
