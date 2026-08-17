#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "RegistrationOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RegistrationOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RegistrationOptions`*"]
    pub type RegistrationOptions;
    #[doc = "Get the `scope` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RegistrationOptions`*"]
    #[wasm_bindgen(method, getter = "scope")]
    pub fn get_scope(this: &RegistrationOptions) -> Option<::alloc::string::String>;
    #[doc = "Change the `scope` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RegistrationOptions`*"]
    #[wasm_bindgen(method, setter = "scope")]
    pub fn set_scope(this: &RegistrationOptions, val: &str);
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RegistrationOptions`*"]
    #[wasm_bindgen(method, getter = "type")]
    pub fn get_type(this: &RegistrationOptions) -> Option<::alloc::string::String>;
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RegistrationOptions`*"]
    #[wasm_bindgen(method, setter = "type")]
    pub fn set_type(this: &RegistrationOptions, val: &str);
    #[cfg(feature = "ServiceWorkerUpdateViaCache")]
    #[doc = "Get the `updateViaCache` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RegistrationOptions`, `ServiceWorkerUpdateViaCache`*"]
    #[wasm_bindgen(method, getter = "updateViaCache")]
    pub fn get_update_via_cache(this: &RegistrationOptions) -> Option<ServiceWorkerUpdateViaCache>;
    #[cfg(feature = "ServiceWorkerUpdateViaCache")]
    #[doc = "Change the `updateViaCache` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RegistrationOptions`, `ServiceWorkerUpdateViaCache`*"]
    #[wasm_bindgen(method, setter = "updateViaCache")]
    pub fn set_update_via_cache(this: &RegistrationOptions, val: ServiceWorkerUpdateViaCache);
}
impl RegistrationOptions {
    #[doc = "Construct a new `RegistrationOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RegistrationOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_scope()` instead."]
    pub fn scope(&mut self, val: &str) -> &mut Self {
        self.set_scope(val);
        self
    }
    #[deprecated = "Use `set_type()` instead."]
    pub fn type_(&mut self, val: &str) -> &mut Self {
        self.set_type(val);
        self
    }
    #[cfg(feature = "ServiceWorkerUpdateViaCache")]
    #[deprecated = "Use `set_update_via_cache()` instead."]
    pub fn update_via_cache(&mut self, val: ServiceWorkerUpdateViaCache) -> &mut Self {
        self.set_update_via_cache(val);
        self
    }
}
impl Default for RegistrationOptions {
    fn default() -> Self {
        Self::new()
    }
}
