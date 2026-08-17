#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "EventListenerOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `EventListenerOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventListenerOptions`*"]
    pub type EventListenerOptions;
    #[doc = "Get the `capture` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventListenerOptions`*"]
    #[wasm_bindgen(method, getter = "capture")]
    pub fn get_capture(this: &EventListenerOptions) -> Option<bool>;
    #[doc = "Change the `capture` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventListenerOptions`*"]
    #[wasm_bindgen(method, setter = "capture")]
    pub fn set_capture(this: &EventListenerOptions, val: bool);
}
impl EventListenerOptions {
    #[doc = "Construct a new `EventListenerOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventListenerOptions`*"]
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
}
impl Default for EventListenerOptions {
    fn default() -> Self {
        Self::new()
    }
}
