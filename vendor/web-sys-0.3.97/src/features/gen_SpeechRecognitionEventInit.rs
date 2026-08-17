#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "SpeechRecognitionEventInit")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SpeechRecognitionEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionEventInit`*"]
    pub type SpeechRecognitionEventInit;
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionEventInit`*"]
    #[wasm_bindgen(method, getter = "bubbles")]
    pub fn get_bubbles(this: &SpeechRecognitionEventInit) -> Option<bool>;
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionEventInit`*"]
    #[wasm_bindgen(method, setter = "bubbles")]
    pub fn set_bubbles(this: &SpeechRecognitionEventInit, val: bool);
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionEventInit`*"]
    #[wasm_bindgen(method, getter = "cancelable")]
    pub fn get_cancelable(this: &SpeechRecognitionEventInit) -> Option<bool>;
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionEventInit`*"]
    #[wasm_bindgen(method, setter = "cancelable")]
    pub fn set_cancelable(this: &SpeechRecognitionEventInit, val: bool);
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionEventInit`*"]
    #[wasm_bindgen(method, getter = "composed")]
    pub fn get_composed(this: &SpeechRecognitionEventInit) -> Option<bool>;
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionEventInit`*"]
    #[wasm_bindgen(method, setter = "composed")]
    pub fn set_composed(this: &SpeechRecognitionEventInit, val: bool);
    #[cfg(feature = "Document")]
    #[doc = "Get the `emma` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `SpeechRecognitionEventInit`*"]
    #[wasm_bindgen(method, getter = "emma")]
    pub fn get_emma(this: &SpeechRecognitionEventInit) -> Option<Document>;
    #[cfg(feature = "Document")]
    #[doc = "Change the `emma` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `SpeechRecognitionEventInit`*"]
    #[wasm_bindgen(method, setter = "emma")]
    pub fn set_emma(this: &SpeechRecognitionEventInit, val: Option<&Document>);
    #[doc = "Get the `interpretation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionEventInit`*"]
    #[wasm_bindgen(method, getter = "interpretation")]
    pub fn get_interpretation(this: &SpeechRecognitionEventInit) -> ::wasm_bindgen::JsValue;
    #[doc = "Change the `interpretation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionEventInit`*"]
    #[wasm_bindgen(method, setter = "interpretation")]
    pub fn set_interpretation(this: &SpeechRecognitionEventInit, val: &::wasm_bindgen::JsValue);
    #[doc = "Get the `resultIndex` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionEventInit`*"]
    #[wasm_bindgen(method, getter = "resultIndex")]
    pub fn get_result_index(this: &SpeechRecognitionEventInit) -> Option<u32>;
    #[doc = "Change the `resultIndex` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionEventInit`*"]
    #[wasm_bindgen(method, setter = "resultIndex")]
    pub fn set_result_index(this: &SpeechRecognitionEventInit, val: u32);
    #[cfg(feature = "SpeechRecognitionResultList")]
    #[doc = "Get the `results` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionEventInit`, `SpeechRecognitionResultList`*"]
    #[wasm_bindgen(method, getter = "results")]
    pub fn get_results(this: &SpeechRecognitionEventInit) -> Option<SpeechRecognitionResultList>;
    #[cfg(feature = "SpeechRecognitionResultList")]
    #[doc = "Change the `results` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionEventInit`, `SpeechRecognitionResultList`*"]
    #[wasm_bindgen(method, setter = "results")]
    pub fn set_results(
        this: &SpeechRecognitionEventInit,
        val: Option<&SpeechRecognitionResultList>,
    );
}
impl SpeechRecognitionEventInit {
    #[doc = "Construct a new `SpeechRecognitionEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionEventInit`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_bubbles()` instead."]
    pub fn bubbles(&mut self, val: bool) -> &mut Self {
        self.set_bubbles(val);
        self
    }
    #[deprecated = "Use `set_cancelable()` instead."]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.set_cancelable(val);
        self
    }
    #[deprecated = "Use `set_composed()` instead."]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.set_composed(val);
        self
    }
    #[cfg(feature = "Document")]
    #[deprecated = "Use `set_emma()` instead."]
    pub fn emma(&mut self, val: Option<&Document>) -> &mut Self {
        self.set_emma(val);
        self
    }
    #[deprecated = "Use `set_interpretation()` instead."]
    pub fn interpretation(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_interpretation(val);
        self
    }
    #[deprecated = "Use `set_result_index()` instead."]
    pub fn result_index(&mut self, val: u32) -> &mut Self {
        self.set_result_index(val);
        self
    }
    #[cfg(feature = "SpeechRecognitionResultList")]
    #[deprecated = "Use `set_results()` instead."]
    pub fn results(&mut self, val: Option<&SpeechRecognitionResultList>) -> &mut Self {
        self.set_results(val);
        self
    }
}
impl Default for SpeechRecognitionEventInit {
    fn default() -> Self {
        Self::new()
    }
}
