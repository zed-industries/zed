#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "ClientQueryOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ClientQueryOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ClientQueryOptions`*"]
    pub type ClientQueryOptions;
    #[doc = "Get the `includeUncontrolled` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ClientQueryOptions`*"]
    #[wasm_bindgen(method, getter = "includeUncontrolled")]
    pub fn get_include_uncontrolled(this: &ClientQueryOptions) -> Option<bool>;
    #[doc = "Change the `includeUncontrolled` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ClientQueryOptions`*"]
    #[wasm_bindgen(method, setter = "includeUncontrolled")]
    pub fn set_include_uncontrolled(this: &ClientQueryOptions, val: bool);
    #[cfg(feature = "ClientType")]
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ClientQueryOptions`, `ClientType`*"]
    #[wasm_bindgen(method, getter = "type")]
    pub fn get_type(this: &ClientQueryOptions) -> Option<ClientType>;
    #[cfg(feature = "ClientType")]
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ClientQueryOptions`, `ClientType`*"]
    #[wasm_bindgen(method, setter = "type")]
    pub fn set_type(this: &ClientQueryOptions, val: ClientType);
}
impl ClientQueryOptions {
    #[doc = "Construct a new `ClientQueryOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ClientQueryOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_include_uncontrolled()` instead."]
    pub fn include_uncontrolled(&mut self, val: bool) -> &mut Self {
        self.set_include_uncontrolled(val);
        self
    }
    #[cfg(feature = "ClientType")]
    #[deprecated = "Use `set_type()` instead."]
    pub fn type_(&mut self, val: ClientType) -> &mut Self {
        self.set_type(val);
        self
    }
}
impl Default for ClientQueryOptions {
    fn default() -> Self {
        Self::new()
    }
}
