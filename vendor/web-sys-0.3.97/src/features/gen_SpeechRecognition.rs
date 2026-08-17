#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "EventTarget",
        extends = "::js_sys::Object",
        js_name = "SpeechRecognition",
        typescript_type = "SpeechRecognition"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SpeechRecognition` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognition`*"]
    pub type SpeechRecognition;
    #[cfg(feature = "SpeechGrammarList")]
    #[wasm_bindgen(method, getter, js_class = "SpeechRecognition", js_name = "grammars")]
    #[doc = "Getter for the `grammars` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/grammars)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechGrammarList`, `SpeechRecognition`*"]
    pub fn grammars(this: &SpeechRecognition) -> SpeechGrammarList;
    #[cfg(feature = "SpeechGrammarList")]
    #[wasm_bindgen(method, setter, js_class = "SpeechRecognition", js_name = "grammars")]
    #[doc = "Setter for the `grammars` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/grammars)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechGrammarList`, `SpeechRecognition`*"]
    pub fn set_grammars(this: &SpeechRecognition, value: &SpeechGrammarList);
    #[wasm_bindgen(method, getter, js_class = "SpeechRecognition", js_name = "lang")]
    #[doc = "Getter for the `lang` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/lang)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognition`*"]
    pub fn lang(this: &SpeechRecognition) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "SpeechRecognition", js_name = "lang")]
    #[doc = "Setter for the `lang` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/lang)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognition`*"]
    pub fn set_lang(this: &SpeechRecognition, value: &str);
    #[wasm_bindgen(
        catch,
        method,
        getter,
        js_class = "SpeechRecognition",
        js_name = "continuous"
    )]
    #[doc = "Getter for the `continuous` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/continuous)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognition`*"]
    pub fn continuous(this: &SpeechRecognition) -> Result<bool, JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        setter,
        js_class = "SpeechRecognition",
        js_name = "continuous"
    )]
    #[doc = "Setter for the `continuous` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/continuous)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognition`*"]
    pub fn set_continuous(this: &SpeechRecognition, value: bool) -> Result<(), JsValue>;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SpeechRecognition",
        js_name = "interimResults"
    )]
    #[doc = "Getter for the `interimResults` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/interimResults)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognition`*"]
    pub fn interim_results(this: &SpeechRecognition) -> bool;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "SpeechRecognition",
        js_name = "interimResults"
    )]
    #[doc = "Setter for the `interimResults` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/interimResults)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognition`*"]
    pub fn set_interim_results(this: &SpeechRecognition, value: bool);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SpeechRecognition",
        js_name = "maxAlternatives"
    )]
    #[doc = "Getter for the `maxAlternatives` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/maxAlternatives)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognition`*"]
    pub fn max_alternatives(this: &SpeechRecognition) -> u32;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "SpeechRecognition",
        js_name = "maxAlternatives"
    )]
    #[doc = "Setter for the `maxAlternatives` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/maxAlternatives)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognition`*"]
    pub fn set_max_alternatives(this: &SpeechRecognition, value: u32);
    #[wasm_bindgen(
        catch,
        method,
        getter,
        js_class = "SpeechRecognition",
        js_name = "serviceURI"
    )]
    #[doc = "Getter for the `serviceURI` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/serviceURI)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognition`*"]
    pub fn service_uri(this: &SpeechRecognition) -> Result<::alloc::string::String, JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        setter,
        js_class = "SpeechRecognition",
        js_name = "serviceURI"
    )]
    #[doc = "Setter for the `serviceURI` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/serviceURI)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognition`*"]
    pub fn set_service_uri(this: &SpeechRecognition, value: &str) -> Result<(), JsValue>;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SpeechRecognition",
        js_name = "onaudiostart"
    )]
    #[doc = "Getter for the `onaudiostart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onaudiostart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognition`*"]
    pub fn onaudiostart(this: &SpeechRecognition) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "SpeechRecognition",
        js_name = "onaudiostart"
    )]
    #[doc = "Setter for the `onaudiostart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onaudiostart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognition`*"]
    pub fn set_onaudiostart(this: &SpeechRecognition, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SpeechRecognition",
        js_name = "onsoundstart"
    )]
    #[doc = "Getter for the `onsoundstart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onsoundstart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognition`*"]
    pub fn onsoundstart(this: &SpeechRecognition) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "SpeechRecognition",
        js_name = "onsoundstart"
    )]
    #[doc = "Setter for the `onsoundstart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onsoundstart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognition`*"]
    pub fn set_onsoundstart(this: &SpeechRecognition, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SpeechRecognition",
        js_name = "onspeechstart"
    )]
    #[doc = "Getter for the `onspeechstart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onspeechstart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognition`*"]
    pub fn onspeechstart(this: &SpeechRecognition) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "SpeechRecognition",
        js_name = "onspeechstart"
    )]
    #[doc = "Setter for the `onspeechstart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onspeechstart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognition`*"]
    pub fn set_onspeechstart(this: &SpeechRecognition, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SpeechRecognition",
        js_name = "onspeechend"
    )]
    #[doc = "Getter for the `onspeechend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onspeechend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognition`*"]
    pub fn onspeechend(this: &SpeechRecognition) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "SpeechRecognition",
        js_name = "onspeechend"
    )]
    #[doc = "Setter for the `onspeechend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onspeechend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognition`*"]
    pub fn set_onspeechend(this: &SpeechRecognition, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "SpeechRecognition", js_name = "onsoundend")]
    #[doc = "Getter for the `onsoundend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onsoundend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognition`*"]
    pub fn onsoundend(this: &SpeechRecognition) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "SpeechRecognition", js_name = "onsoundend")]
    #[doc = "Setter for the `onsoundend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onsoundend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognition`*"]
    pub fn set_onsoundend(this: &SpeechRecognition, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "SpeechRecognition", js_name = "onaudioend")]
    #[doc = "Getter for the `onaudioend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onaudioend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognition`*"]
    pub fn onaudioend(this: &SpeechRecognition) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "SpeechRecognition", js_name = "onaudioend")]
    #[doc = "Setter for the `onaudioend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onaudioend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognition`*"]
    pub fn set_onaudioend(this: &SpeechRecognition, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "SpeechRecognition", js_name = "onresult")]
    #[doc = "Getter for the `onresult` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onresult)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognition`*"]
    pub fn onresult(this: &SpeechRecognition) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "SpeechRecognition", js_name = "onresult")]
    #[doc = "Setter for the `onresult` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onresult)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognition`*"]
    pub fn set_onresult(this: &SpeechRecognition, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "SpeechRecognition", js_name = "onnomatch")]
    #[doc = "Getter for the `onnomatch` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onnomatch)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognition`*"]
    pub fn onnomatch(this: &SpeechRecognition) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "SpeechRecognition", js_name = "onnomatch")]
    #[doc = "Setter for the `onnomatch` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onnomatch)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognition`*"]
    pub fn set_onnomatch(this: &SpeechRecognition, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "SpeechRecognition", js_name = "onerror")]
    #[doc = "Getter for the `onerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognition`*"]
    pub fn onerror(this: &SpeechRecognition) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "SpeechRecognition", js_name = "onerror")]
    #[doc = "Setter for the `onerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognition`*"]
    pub fn set_onerror(this: &SpeechRecognition, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "SpeechRecognition", js_name = "onstart")]
    #[doc = "Getter for the `onstart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onstart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognition`*"]
    pub fn onstart(this: &SpeechRecognition) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "SpeechRecognition", js_name = "onstart")]
    #[doc = "Setter for the `onstart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onstart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognition`*"]
    pub fn set_onstart(this: &SpeechRecognition, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "SpeechRecognition", js_name = "onend")]
    #[doc = "Getter for the `onend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognition`*"]
    pub fn onend(this: &SpeechRecognition) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "SpeechRecognition", js_name = "onend")]
    #[doc = "Setter for the `onend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognition`*"]
    pub fn set_onend(this: &SpeechRecognition, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(catch, constructor, js_class = "SpeechRecognition")]
    #[doc = "The `new SpeechRecognition(..)` constructor, creating a new instance of `SpeechRecognition`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/SpeechRecognition)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognition`*"]
    pub fn new() -> Result<SpeechRecognition, JsValue>;
    #[wasm_bindgen(method, js_class = "SpeechRecognition")]
    #[doc = "The `abort()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/abort)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognition`*"]
    pub fn abort(this: &SpeechRecognition);
    #[wasm_bindgen(catch, method, js_class = "SpeechRecognition")]
    #[doc = "The `start()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/start)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognition`*"]
    pub fn start(this: &SpeechRecognition) -> Result<(), JsValue>;
    #[cfg(feature = "MediaStream")]
    #[wasm_bindgen(catch, method, js_class = "SpeechRecognition", js_name = "start")]
    #[doc = "The `start()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/start)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStream`, `SpeechRecognition`*"]
    pub fn start_with_stream(this: &SpeechRecognition, stream: &MediaStream)
        -> Result<(), JsValue>;
    #[wasm_bindgen(method, js_class = "SpeechRecognition")]
    #[doc = "The `stop()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/stop)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognition`*"]
    pub fn stop(this: &SpeechRecognition);
}
