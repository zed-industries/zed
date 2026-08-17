#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "WorkerOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WorkerOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerOptions`*"]
    pub type WorkerOptions;
    #[cfg(feature = "RequestCredentials")]
    #[doc = "Get the `credentials` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RequestCredentials`, `WorkerOptions`*"]
    #[wasm_bindgen(method, getter = "credentials")]
    pub fn get_credentials(this: &WorkerOptions) -> Option<RequestCredentials>;
    #[cfg(feature = "RequestCredentials")]
    #[doc = "Change the `credentials` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RequestCredentials`, `WorkerOptions`*"]
    #[wasm_bindgen(method, setter = "credentials")]
    pub fn set_credentials(this: &WorkerOptions, val: RequestCredentials);
    #[doc = "Get the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerOptions`*"]
    #[wasm_bindgen(method, getter = "name")]
    pub fn get_name(this: &WorkerOptions) -> Option<::alloc::string::String>;
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerOptions`*"]
    #[wasm_bindgen(method, setter = "name")]
    pub fn set_name(this: &WorkerOptions, val: &str);
    #[cfg(feature = "WorkerType")]
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerOptions`, `WorkerType`*"]
    #[wasm_bindgen(method, getter = "type")]
    pub fn get_type(this: &WorkerOptions) -> Option<WorkerType>;
    #[cfg(feature = "WorkerType")]
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerOptions`, `WorkerType`*"]
    #[wasm_bindgen(method, setter = "type")]
    pub fn set_type(this: &WorkerOptions, val: WorkerType);
}
impl WorkerOptions {
    #[doc = "Construct a new `WorkerOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerOptions`*"]
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
    #[deprecated = "Use `set_name()` instead."]
    pub fn name(&mut self, val: &str) -> &mut Self {
        self.set_name(val);
        self
    }
    #[cfg(feature = "WorkerType")]
    #[deprecated = "Use `set_type()` instead."]
    pub fn type_(&mut self, val: WorkerType) -> &mut Self {
        self.set_type(val);
        self
    }
}
impl Default for WorkerOptions {
    fn default() -> Self {
        Self::new()
    }
}
