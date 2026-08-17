#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "OfflineAudioCompletionEventInit"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `OfflineAudioCompletionEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineAudioCompletionEventInit`*"]
    pub type OfflineAudioCompletionEventInit;
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineAudioCompletionEventInit`*"]
    #[wasm_bindgen(method, getter = "bubbles")]
    pub fn get_bubbles(this: &OfflineAudioCompletionEventInit) -> Option<bool>;
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineAudioCompletionEventInit`*"]
    #[wasm_bindgen(method, setter = "bubbles")]
    pub fn set_bubbles(this: &OfflineAudioCompletionEventInit, val: bool);
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineAudioCompletionEventInit`*"]
    #[wasm_bindgen(method, getter = "cancelable")]
    pub fn get_cancelable(this: &OfflineAudioCompletionEventInit) -> Option<bool>;
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineAudioCompletionEventInit`*"]
    #[wasm_bindgen(method, setter = "cancelable")]
    pub fn set_cancelable(this: &OfflineAudioCompletionEventInit, val: bool);
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineAudioCompletionEventInit`*"]
    #[wasm_bindgen(method, getter = "composed")]
    pub fn get_composed(this: &OfflineAudioCompletionEventInit) -> Option<bool>;
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineAudioCompletionEventInit`*"]
    #[wasm_bindgen(method, setter = "composed")]
    pub fn set_composed(this: &OfflineAudioCompletionEventInit, val: bool);
    #[cfg(feature = "AudioBuffer")]
    #[doc = "Get the `renderedBuffer` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBuffer`, `OfflineAudioCompletionEventInit`*"]
    #[wasm_bindgen(method, getter = "renderedBuffer")]
    pub fn get_rendered_buffer(this: &OfflineAudioCompletionEventInit) -> AudioBuffer;
    #[cfg(feature = "AudioBuffer")]
    #[doc = "Change the `renderedBuffer` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBuffer`, `OfflineAudioCompletionEventInit`*"]
    #[wasm_bindgen(method, setter = "renderedBuffer")]
    pub fn set_rendered_buffer(this: &OfflineAudioCompletionEventInit, val: &AudioBuffer);
}
impl OfflineAudioCompletionEventInit {
    #[cfg(feature = "AudioBuffer")]
    #[doc = "Construct a new `OfflineAudioCompletionEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBuffer`, `OfflineAudioCompletionEventInit`*"]
    pub fn new(rendered_buffer: &AudioBuffer) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_rendered_buffer(rendered_buffer);
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
    #[cfg(feature = "AudioBuffer")]
    #[deprecated = "Use `set_rendered_buffer()` instead."]
    pub fn rendered_buffer(&mut self, val: &AudioBuffer) -> &mut Self {
        self.set_rendered_buffer(val);
        self
    }
}
