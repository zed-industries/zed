#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "EventSourceInit")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `EventSourceInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventSourceInit`*"]
    pub type EventSourceInit;
    #[doc = "Get the `withCredentials` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventSourceInit`*"]
    #[wasm_bindgen(method, getter = "withCredentials")]
    pub fn get_with_credentials(this: &EventSourceInit) -> Option<bool>;
    #[doc = "Change the `withCredentials` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventSourceInit`*"]
    #[wasm_bindgen(method, setter = "withCredentials")]
    pub fn set_with_credentials(this: &EventSourceInit, val: bool);
}
impl EventSourceInit {
    #[doc = "Construct a new `EventSourceInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventSourceInit`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_with_credentials()` instead."]
    pub fn with_credentials(&mut self, val: bool) -> &mut Self {
        self.set_with_credentials(val);
        self
    }
}
impl Default for EventSourceInit {
    fn default() -> Self {
        Self::new()
    }
}
