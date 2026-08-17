#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "Event",
        extends = "::js_sys::Object",
        js_name = "SpeechSynthesisEvent",
        typescript_type = "SpeechSynthesisEvent"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SpeechSynthesisEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisEvent`*"]
    pub type SpeechSynthesisEvent;
    #[cfg(feature = "SpeechSynthesisUtterance")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SpeechSynthesisEvent",
        js_name = "utterance"
    )]
    #[doc = "Getter for the `utterance` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisEvent/utterance)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisEvent`, `SpeechSynthesisUtterance`*"]
    pub fn utterance(this: &SpeechSynthesisEvent) -> SpeechSynthesisUtterance;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SpeechSynthesisEvent",
        js_name = "charIndex"
    )]
    #[doc = "Getter for the `charIndex` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisEvent/charIndex)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisEvent`*"]
    pub fn char_index(this: &SpeechSynthesisEvent) -> u32;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SpeechSynthesisEvent",
        js_name = "charLength"
    )]
    #[doc = "Getter for the `charLength` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisEvent/charLength)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisEvent`*"]
    pub fn char_length(this: &SpeechSynthesisEvent) -> Option<u32>;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SpeechSynthesisEvent",
        js_name = "elapsedTime"
    )]
    #[doc = "Getter for the `elapsedTime` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisEvent/elapsedTime)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisEvent`*"]
    pub fn elapsed_time(this: &SpeechSynthesisEvent) -> f32;
    #[wasm_bindgen(method, getter, js_class = "SpeechSynthesisEvent", js_name = "name")]
    #[doc = "Getter for the `name` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisEvent/name)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisEvent`*"]
    pub fn name(this: &SpeechSynthesisEvent) -> Option<::alloc::string::String>;
    #[cfg(feature = "SpeechSynthesisEventInit")]
    #[wasm_bindgen(catch, constructor, js_class = "SpeechSynthesisEvent")]
    #[doc = "The `new SpeechSynthesisEvent(..)` constructor, creating a new instance of `SpeechSynthesisEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisEvent/SpeechSynthesisEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisEvent`, `SpeechSynthesisEventInit`*"]
    pub fn new(
        type_: &str,
        event_init_dict: &SpeechSynthesisEventInit,
    ) -> Result<SpeechSynthesisEvent, JsValue>;
}
