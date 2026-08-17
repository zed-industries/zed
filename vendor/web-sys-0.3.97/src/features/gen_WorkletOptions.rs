#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "WorkletOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WorkletOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkletOptions`*"]
    pub type WorkletOptions;
    #[cfg(feature = "RequestCredentials")]
    #[doc = "Get the `credentials` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RequestCredentials`, `WorkletOptions`*"]
    #[wasm_bindgen(method, getter = "credentials")]
    pub fn get_credentials(this: &WorkletOptions) -> Option<RequestCredentials>;
    #[cfg(feature = "RequestCredentials")]
    #[doc = "Change the `credentials` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RequestCredentials`, `WorkletOptions`*"]
    #[wasm_bindgen(method, setter = "credentials")]
    pub fn set_credentials(this: &WorkletOptions, val: RequestCredentials);
}
impl WorkletOptions {
    #[doc = "Construct a new `WorkletOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkletOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(feature = "RequestCredentials")]
    #[deprecated = "Use `set_credentials()` instead."]
    pub fn credentials(&mut self, val: RequestCredentials) -> &mut Self {
        self.set_credentials(val);
        self
    }
}
impl Default for WorkletOptions {
    fn default() -> Self {
        Self::new()
    }
}
