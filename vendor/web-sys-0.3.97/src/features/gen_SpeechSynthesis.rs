#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "EventTarget",
        extends = "::js_sys::Object",
        js_name = "SpeechSynthesis",
        typescript_type = "SpeechSynthesis"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SpeechSynthesis` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesis)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesis`*"]
    pub type SpeechSynthesis;
    #[wasm_bindgen(method, getter, js_class = "SpeechSynthesis", js_name = "pending")]
    #[doc = "Getter for the `pending` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesis/pending)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesis`*"]
    pub fn pending(this: &SpeechSynthesis) -> bool;
    #[wasm_bindgen(method, getter, js_class = "SpeechSynthesis", js_name = "speaking")]
    #[doc = "Getter for the `speaking` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesis/speaking)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesis`*"]
    pub fn speaking(this: &SpeechSynthesis) -> bool;
    #[wasm_bindgen(method, getter, js_class = "SpeechSynthesis", js_name = "paused")]
    #[doc = "Getter for the `paused` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesis/paused)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesis`*"]
    pub fn paused(this: &SpeechSynthesis) -> bool;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SpeechSynthesis",
        js_name = "onvoiceschanged"
    )]
    #[doc = "Getter for the `onvoiceschanged` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesis/onvoiceschanged)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesis`*"]
    pub fn onvoiceschanged(this: &SpeechSynthesis) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "SpeechSynthesis",
        js_name = "onvoiceschanged"
    )]
    #[doc = "Setter for the `onvoiceschanged` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesis/onvoiceschanged)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesis`*"]
    pub fn set_onvoiceschanged(this: &SpeechSynthesis, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, js_class = "SpeechSynthesis")]
    #[doc = "The `cancel()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesis/cancel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesis`*"]
    pub fn cancel(this: &SpeechSynthesis);
    #[wasm_bindgen(method, js_class = "SpeechSynthesis", js_name = "getVoices")]
    #[doc = "The `getVoices()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesis/getVoices)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesis`*"]
    pub fn get_voices(this: &SpeechSynthesis) -> ::js_sys::Array;
    #[wasm_bindgen(method, js_class = "SpeechSynthesis")]
    #[doc = "The `pause()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesis/pause)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesis`*"]
    pub fn pause(this: &SpeechSynthesis);
    #[wasm_bindgen(method, js_class = "SpeechSynthesis")]
    #[doc = "The `resume()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesis/resume)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesis`*"]
    pub fn resume(this: &SpeechSynthesis);
    #[cfg(feature = "SpeechSynthesisUtterance")]
    #[wasm_bindgen(method, js_class = "SpeechSynthesis")]
    #[doc = "The `speak()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesis/speak)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesis`, `SpeechSynthesisUtterance`*"]
    pub fn speak(this: &SpeechSynthesis, utterance: &SpeechSynthesisUtterance);
}
