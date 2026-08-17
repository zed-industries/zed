#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "TokenBinding")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `TokenBinding` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TokenBinding`*"]
    pub type TokenBinding;
    #[doc = "Get the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TokenBinding`*"]
    #[wasm_bindgen(method, getter = "id")]
    pub fn get_id(this: &TokenBinding) -> Option<::alloc::string::String>;
    #[doc = "Change the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TokenBinding`*"]
    #[wasm_bindgen(method, setter = "id")]
    pub fn set_id(this: &TokenBinding, val: &str);
    #[doc = "Get the `status` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TokenBinding`*"]
    #[wasm_bindgen(method, getter = "status")]
    pub fn get_status(this: &TokenBinding) -> ::alloc::string::String;
    #[doc = "Change the `status` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TokenBinding`*"]
    #[wasm_bindgen(method, setter = "status")]
    pub fn set_status(this: &TokenBinding, val: &str);
}
impl TokenBinding {
    #[doc = "Construct a new `TokenBinding`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TokenBinding`*"]
    pub fn new(status: &str) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_status(status);
        ret
    }
    #[deprecated = "Use `set_id()` instead."]
    pub fn id(&mut self, val: &str) -> &mut Self {
        self.set_id(val);
        self
    }
    #[deprecated = "Use `set_status()` instead."]
    pub fn status(&mut self, val: &str) -> &mut Self {
        self.set_status(val);
        self
    }
}
