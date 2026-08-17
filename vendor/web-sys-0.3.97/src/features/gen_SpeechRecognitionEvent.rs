#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "Event",
        extends = "::js_sys::Object",
        js_name = "SpeechRecognitionEvent",
        typescript_type = "SpeechRecognitionEvent"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SpeechRecognitionEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognitionEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionEvent`*"]
    pub type SpeechRecognitionEvent;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SpeechRecognitionEvent",
        js_name = "resultIndex"
    )]
    #[doc = "Getter for the `resultIndex` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognitionEvent/resultIndex)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionEvent`*"]
    pub fn result_index(this: &SpeechRecognitionEvent) -> u32;
    #[cfg(feature = "SpeechRecognitionResultList")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SpeechRecognitionEvent",
        js_name = "results"
    )]
    #[doc = "Getter for the `results` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognitionEvent/results)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionEvent`, `SpeechRecognitionResultList`*"]
    pub fn results(this: &SpeechRecognitionEvent) -> Option<SpeechRecognitionResultList>;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SpeechRecognitionEvent",
        js_name = "interpretation"
    )]
    #[doc = "Getter for the `interpretation` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognitionEvent/interpretation)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionEvent`*"]
    pub fn interpretation(this: &SpeechRecognitionEvent) -> ::wasm_bindgen::JsValue;
    #[cfg(feature = "Document")]
    #[wasm_bindgen(method, getter, js_class = "SpeechRecognitionEvent", js_name = "emma")]
    #[doc = "Getter for the `emma` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognitionEvent/emma)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `SpeechRecognitionEvent`*"]
    pub fn emma(this: &SpeechRecognitionEvent) -> Option<Document>;
    #[wasm_bindgen(catch, constructor, js_class = "SpeechRecognitionEvent")]
    #[doc = "The `new SpeechRecognitionEvent(..)` constructor, creating a new instance of `SpeechRecognitionEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognitionEvent/SpeechRecognitionEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionEvent`*"]
    pub fn new(type_: &str) -> Result<SpeechRecognitionEvent, JsValue>;
    #[cfg(feature = "SpeechRecognitionEventInit")]
    #[wasm_bindgen(catch, constructor, js_class = "SpeechRecognitionEvent")]
    #[doc = "The `new SpeechRecognitionEvent(..)` constructor, creating a new instance of `SpeechRecognitionEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognitionEvent/SpeechRecognitionEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionEvent`, `SpeechRecognitionEventInit`*"]
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &SpeechRecognitionEventInit,
    ) -> Result<SpeechRecognitionEvent, JsValue>;
}
