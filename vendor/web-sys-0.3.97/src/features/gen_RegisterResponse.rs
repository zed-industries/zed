#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "RegisterResponse")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RegisterResponse` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RegisterResponse`*"]
    pub type RegisterResponse;
    #[doc = "Get the `clientData` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RegisterResponse`*"]
    #[wasm_bindgen(method, getter = "clientData")]
    pub fn get_client_data(this: &RegisterResponse) -> Option<::alloc::string::String>;
    #[doc = "Change the `clientData` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RegisterResponse`*"]
    #[wasm_bindgen(method, setter = "clientData")]
    pub fn set_client_data(this: &RegisterResponse, val: &str);
    #[doc = "Get the `errorCode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RegisterResponse`*"]
    #[wasm_bindgen(method, getter = "errorCode")]
    pub fn get_error_code(this: &RegisterResponse) -> Option<u16>;
    #[doc = "Change the `errorCode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RegisterResponse`*"]
    #[wasm_bindgen(method, setter = "errorCode")]
    pub fn set_error_code(this: &RegisterResponse, val: Option<u16>);
    #[doc = "Get the `errorMessage` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RegisterResponse`*"]
    #[wasm_bindgen(method, getter = "errorMessage")]
    pub fn get_error_message(this: &RegisterResponse) -> Option<::alloc::string::String>;
    #[doc = "Change the `errorMessage` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RegisterResponse`*"]
    #[wasm_bindgen(method, setter = "errorMessage")]
    pub fn set_error_message(this: &RegisterResponse, val: Option<&str>);
    #[doc = "Get the `registrationData` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RegisterResponse`*"]
    #[wasm_bindgen(method, getter = "registrationData")]
    pub fn get_registration_data(this: &RegisterResponse) -> Option<::alloc::string::String>;
    #[doc = "Change the `registrationData` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RegisterResponse`*"]
    #[wasm_bindgen(method, setter = "registrationData")]
    pub fn set_registration_data(this: &RegisterResponse, val: &str);
    #[doc = "Get the `version` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RegisterResponse`*"]
    #[wasm_bindgen(method, getter = "version")]
    pub fn get_version(this: &RegisterResponse) -> Option<::alloc::string::String>;
    #[doc = "Change the `version` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RegisterResponse`*"]
    #[wasm_bindgen(method, setter = "version")]
    pub fn set_version(this: &RegisterResponse, val: &str);
}
impl RegisterResponse {
    #[doc = "Construct a new `RegisterResponse`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RegisterResponse`*"]
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
    #[deprecated = "Use `set_registration_data()` instead."]
    pub fn registration_data(&mut self, val: &str) -> &mut Self {
        self.set_registration_data(val);
        self
    }
    #[deprecated = "Use `set_version()` instead."]
    pub fn version(&mut self, val: &str) -> &mut Self {
        self.set_version(val);
        self
    }
}
impl Default for RegisterResponse {
    fn default() -> Self {
        Self::new()
    }
}
