#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "SignResponse")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SignResponse` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SignResponse`*"]
    pub type SignResponse;
    #[doc = "Get the `clientData` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SignResponse`*"]
    #[wasm_bindgen(method, getter = "clientData")]
    pub fn get_client_data(this: &SignResponse) -> Option<::alloc::string::String>;
    #[doc = "Change the `clientData` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SignResponse`*"]
    #[wasm_bindgen(method, setter = "clientData")]
    pub fn set_client_data(this: &SignResponse, val: &str);
    #[doc = "Get the `errorCode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SignResponse`*"]
    #[wasm_bindgen(method, getter = "errorCode")]
    pub fn get_error_code(this: &SignResponse) -> Option<u16>;
    #[doc = "Change the `errorCode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SignResponse`*"]
    #[wasm_bindgen(method, setter = "errorCode")]
    pub fn set_error_code(this: &SignResponse, val: Option<u16>);
    #[doc = "Get the `errorMessage` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SignResponse`*"]
    #[wasm_bindgen(method, getter = "errorMessage")]
    pub fn get_error_message(this: &SignResponse) -> Option<::alloc::string::String>;
    #[doc = "Change the `errorMessage` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SignResponse`*"]
    #[wasm_bindgen(method, setter = "errorMessage")]
    pub fn set_error_message(this: &SignResponse, val: Option<&str>);
    #[doc = "Get the `keyHandle` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SignResponse`*"]
    #[wasm_bindgen(method, getter = "keyHandle")]
    pub fn get_key_handle(this: &SignResponse) -> Option<::alloc::string::String>;
    #[doc = "Change the `keyHandle` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SignResponse`*"]
    #[wasm_bindgen(method, setter = "keyHandle")]
    pub fn set_key_handle(this: &SignResponse, val: &str);
    #[doc = "Get the `signatureData` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SignResponse`*"]
    #[wasm_bindgen(method, getter = "signatureData")]
    pub fn get_signature_data(this: &SignResponse) -> Option<::alloc::string::String>;
    #[doc = "Change the `signatureData` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SignResponse`*"]
    #[wasm_bindgen(method, setter = "signatureData")]
    pub fn set_signature_data(this: &SignResponse, val: &str);
}
impl SignResponse {
    #[doc = "Construct a new `SignResponse`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SignResponse`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_client_data()` instead."]
    pub fn client_data(&mut self, val: &str) -> &mut Self {
        self.set_client_data(val);
        self
    }
    #[deprecated = "Use `set_error_code()` instead."]
    pub fn error_code(&mut self, val: Option<u16>) -> &mut Self {
        self.set_error_code(val);
        self
    }
    #[deprecated = "Use `set_error_message()` instead."]
    pub fn error_message(&mut self, val: Option<&str>) -> &mut Self {
        self.set_error_message(val);
        self
    }
    #[deprecated = "Use `set_key_handle()` instead."]
    pub fn key_handle(&mut self, val: &str) -> &mut Self {
        self.set_key_handle(val);
        self
    }
    #[deprecated = "Use `set_signature_data()` instead."]
    pub fn signature_data(&mut self, val: &str) -> &mut Self {
        self.set_signature_data(val);
        self
    }
}
impl Default for SignResponse {
    fn default() -> Self {
        Self::new()
    }
}
