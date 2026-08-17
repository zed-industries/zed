#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "AddEventListenerOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AddEventListenerOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AddEventListenerOptions`*"]
    pub type AddEventListenerOptions;
    #[doc = "Get the `capture` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AddEventListenerOptions`*"]
    #[wasm_bindgen(method, getter = "capture")]
    pub fn get_capture(this: &AddEventListenerOptions) -> Option<bool>;
    #[doc = "Change the `capture` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AddEventListenerOptions`*"]
    #[wasm_bindgen(method, setter = "capture")]
    pub fn set_capture(this: &AddEventListenerOptions, val: bool);
    #[doc = "Get the `once` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AddEventListenerOptions`*"]
    #[wasm_bindgen(method, getter = "once")]
    pub fn get_once(this: &AddEventListenerOptions) -> Option<bool>;
    #[doc = "Change the `once` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AddEventListenerOptions`*"]
    #[wasm_bindgen(method, setter = "once")]
    pub fn set_once(this: &AddEventListenerOptions, val: bool);
    #[doc = "Get the `passive` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AddEventListenerOptions`*"]
    #[wasm_bindgen(method, getter = "passive")]
    pub fn get_passive(this: &AddEventListenerOptions) -> Option<bool>;
    #[doc = "Change the `passive` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AddEventListenerOptions`*"]
    #[wasm_bindgen(method, setter = "passive")]
    pub fn set_passive(this: &AddEventListenerOptions, val: bool);
    #[cfg(feature = "AbortSignal")]
    #[doc = "Get the `signal` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AbortSignal`, `AddEventListenerOptions`*"]
    #[wasm_bindgen(method, getter = "signal")]
    pub fn get_signal(this: &AddEventListenerOptions) -> Option<AbortSignal>;
    #[cfg(feature = "AbortSignal")]
    #[doc = "Change the `signal` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AbortSignal`, `AddEventListenerOptions`*"]
    #[wasm_bindgen(method, setter = "signal")]
    pub fn set_signal(this: &AddEventListenerOptions, val: &AbortSignal);
}
impl AddEventListenerOptions {
    #[doc = "Construct a new `AddEventListenerOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AddEventListenerOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_capture()` instead."]
    pub fn capture(&mut self, val: bool) -> &mut Self {
        self.set_capture(val);
        self
    }
    #[deprecated = "Use `set_once()` instead."]
    pub fn once(&mut self, val: bool) -> &mut Self {
        self.set_once(val);
        self
    }
    #[deprecated = "Use `set_passive()` instead."]
    pub fn passive(&mut self, val: bool) -> &mut Self {
        self.set_passive(val);
        self
    }
    #[cfg(feature = "AbortSignal")]
    #[deprecated = "Use `set_signal()` instead."]
    pub fn signal(&mut self, val: &AbortSignal) -> &mut Self {
        self.set_signal(val);
        self
    }
}
impl Default for AddEventListenerOptions {
    fn default() -> Self {
        Self::new()
    }
}
