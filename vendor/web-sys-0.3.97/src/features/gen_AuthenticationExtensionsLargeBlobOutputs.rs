#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "AuthenticationExtensionsLargeBlobOutputs"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AuthenticationExtensionsLargeBlobOutputs` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsLargeBlobOutputs`*"]
    pub type AuthenticationExtensionsLargeBlobOutputs;
    #[doc = "Get the `blob` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsLargeBlobOutputs`*"]
    #[wasm_bindgen(method, getter = "blob")]
    pub fn get_blob(
        this: &AuthenticationExtensionsLargeBlobOutputs,
    ) -> Option<::js_sys::ArrayBuffer>;
    #[doc = "Change the `blob` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsLargeBlobOutputs`*"]
    #[wasm_bindgen(method, setter = "blob")]
    pub fn set_blob(this: &AuthenticationExtensionsLargeBlobOutputs, val: &::js_sys::ArrayBuffer);
    #[doc = "Get the `supported` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsLargeBlobOutputs`*"]
    #[wasm_bindgen(method, getter = "supported")]
    pub fn get_supported(this: &AuthenticationExtensionsLargeBlobOutputs) -> Option<bool>;
    #[doc = "Change the `supported` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsLargeBlobOutputs`*"]
    #[wasm_bindgen(method, setter = "supported")]
    pub fn set_supported(this: &AuthenticationExtensionsLargeBlobOutputs, val: bool);
    #[doc = "Get the `written` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsLargeBlobOutputs`*"]
    #[wasm_bindgen(method, getter = "written")]
    pub fn get_written(this: &AuthenticationExtensionsLargeBlobOutputs) -> Option<bool>;
    #[doc = "Change the `written` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsLargeBlobOutputs`*"]
    #[wasm_bindgen(method, setter = "written")]
    pub fn set_written(this: &AuthenticationExtensionsLargeBlobOutputs, val: bool);
}
impl AuthenticationExtensionsLargeBlobOutputs {
    #[doc = "Construct a new `AuthenticationExtensionsLargeBlobOutputs`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsLargeBlobOutputs`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_blob()` instead."]
    pub fn blob(&mut self, val: &::js_sys::ArrayBuffer) -> &mut Self {
        self.set_blob(val);
        self
    }
    #[deprecated = "Use `set_supported()` instead."]
    pub fn supported(&mut self, val: bool) -> &mut Self {
        self.set_supported(val);
        self
    }
    #[deprecated = "Use `set_written()` instead."]
    pub fn written(&mut self, val: bool) -> &mut Self {
        self.set_written(val);
        self
    }
}
impl Default for AuthenticationExtensionsLargeBlobOutputs {
    fn default() -> Self {
        Self::new()
    }
}
