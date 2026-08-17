#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "AuthenticationExtensionsLargeBlobInputs"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AuthenticationExtensionsLargeBlobInputs` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsLargeBlobInputs`*"]
    pub type AuthenticationExtensionsLargeBlobInputs;
    #[doc = "Get the `read` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsLargeBlobInputs`*"]
    #[wasm_bindgen(method, getter = "read")]
    pub fn get_read(this: &AuthenticationExtensionsLargeBlobInputs) -> Option<bool>;
    #[doc = "Change the `read` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsLargeBlobInputs`*"]
    #[wasm_bindgen(method, setter = "read")]
    pub fn set_read(this: &AuthenticationExtensionsLargeBlobInputs, val: bool);
    #[doc = "Get the `support` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsLargeBlobInputs`*"]
    #[wasm_bindgen(method, getter = "support")]
    pub fn get_support(
        this: &AuthenticationExtensionsLargeBlobInputs,
    ) -> Option<::alloc::string::String>;
    #[doc = "Change the `support` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsLargeBlobInputs`*"]
    #[wasm_bindgen(method, setter = "support")]
    pub fn set_support(this: &AuthenticationExtensionsLargeBlobInputs, val: &str);
    #[doc = "Get the `write` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsLargeBlobInputs`*"]
    #[wasm_bindgen(method, getter = "write")]
    pub fn get_write(this: &AuthenticationExtensionsLargeBlobInputs) -> Option<::js_sys::Object>;
    #[doc = "Change the `write` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsLargeBlobInputs`*"]
    #[wasm_bindgen(method, setter = "write")]
    pub fn set_write(this: &AuthenticationExtensionsLargeBlobInputs, val: &::js_sys::Object);
    #[doc = "Change the `write` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsLargeBlobInputs`*"]
    #[wasm_bindgen(method, setter = "write")]
    pub fn set_write_buffer_source(
        this: &AuthenticationExtensionsLargeBlobInputs,
        val: &::js_sys::Object,
    );
    #[doc = "Change the `write` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsLargeBlobInputs`*"]
    #[wasm_bindgen(method, setter = "write")]
    pub fn set_write_u8_slice(this: &AuthenticationExtensionsLargeBlobInputs, val: &mut [u8]);
    #[doc = "Change the `write` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsLargeBlobInputs`*"]
    #[wasm_bindgen(method, setter = "write")]
    pub fn set_write_u8_array(
        this: &AuthenticationExtensionsLargeBlobInputs,
        val: &::js_sys::Uint8Array,
    );
}
impl AuthenticationExtensionsLargeBlobInputs {
    #[doc = "Construct a new `AuthenticationExtensionsLargeBlobInputs`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsLargeBlobInputs`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_read()` instead."]
    pub fn read(&mut self, val: bool) -> &mut Self {
        self.set_read(val);
        self
    }
    #[deprecated = "Use `set_support()` instead."]
    pub fn support(&mut self, val: &str) -> &mut Self {
        self.set_support(val);
        self
    }
    #[deprecated = "Use `set_write()` instead."]
    pub fn write(&mut self, val: &::js_sys::Object) -> &mut Self {
        self.set_write(val);
        self
    }
}
impl Default for AuthenticationExtensionsLargeBlobInputs {
    fn default() -> Self {
        Self::new()
    }
}
