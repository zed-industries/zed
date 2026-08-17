#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "PublicKeyCredentialDescriptor"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PublicKeyCredentialDescriptor` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialDescriptor`*"]
    pub type PublicKeyCredentialDescriptor;
    #[doc = "Get the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialDescriptor`*"]
    #[wasm_bindgen(method, getter = "id")]
    pub fn get_id(this: &PublicKeyCredentialDescriptor) -> ::js_sys::Object;
    #[doc = "Change the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialDescriptor`*"]
    #[wasm_bindgen(method, setter = "id")]
    pub fn set_id(this: &PublicKeyCredentialDescriptor, val: &::js_sys::Object);
    #[doc = "Change the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialDescriptor`*"]
    #[wasm_bindgen(method, setter = "id")]
    pub fn set_id_buffer_source(this: &PublicKeyCredentialDescriptor, val: &::js_sys::Object);
    #[doc = "Change the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialDescriptor`*"]
    #[wasm_bindgen(method, setter = "id")]
    pub fn set_id_u8_slice(this: &PublicKeyCredentialDescriptor, val: &mut [u8]);
    #[doc = "Change the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialDescriptor`*"]
    #[wasm_bindgen(method, setter = "id")]
    pub fn set_id_u8_array(this: &PublicKeyCredentialDescriptor, val: &::js_sys::Uint8Array);
    #[doc = "Get the `transports` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialDescriptor`*"]
    #[wasm_bindgen(method, getter = "transports")]
    pub fn get_transports(this: &PublicKeyCredentialDescriptor) -> Option<::js_sys::Array>;
    #[doc = "Change the `transports` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialDescriptor`*"]
    #[wasm_bindgen(method, setter = "transports")]
    pub fn set_transports(this: &PublicKeyCredentialDescriptor, val: &::wasm_bindgen::JsValue);
    #[cfg(feature = "PublicKeyCredentialType")]
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialDescriptor`, `PublicKeyCredentialType`*"]
    #[wasm_bindgen(method, getter = "type")]
    pub fn get_type(this: &PublicKeyCredentialDescriptor) -> PublicKeyCredentialType;
    #[cfg(feature = "PublicKeyCredentialType")]
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialDescriptor`, `PublicKeyCredentialType`*"]
    #[wasm_bindgen(method, setter = "type")]
    pub fn set_type(this: &PublicKeyCredentialDescriptor, val: PublicKeyCredentialType);
}
impl PublicKeyCredentialDescriptor {
    #[cfg(feature = "PublicKeyCredentialType")]
    #[doc = "Construct a new `PublicKeyCredentialDescriptor`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialDescriptor`, `PublicKeyCredentialType`*"]
    pub fn new(id: &::js_sys::Object, type_: PublicKeyCredentialType) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_id(id);
        ret.set_type(type_);
        ret
    }
    #[cfg(feature = "PublicKeyCredentialType")]
    #[doc = "Construct a new `PublicKeyCredentialDescriptor`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialDescriptor`, `PublicKeyCredentialType`*"]
    pub fn new_with_u8_slice(id: &mut [u8], type_: PublicKeyCredentialType) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_id_u8_slice(id);
        ret.set_type(type_);
        ret
    }
    #[cfg(feature = "PublicKeyCredentialType")]
    #[doc = "Construct a new `PublicKeyCredentialDescriptor`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialDescriptor`, `PublicKeyCredentialType`*"]
    pub fn new_with_u8_array(id: &::js_sys::Uint8Array, type_: PublicKeyCredentialType) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_id_u8_array(id);
        ret.set_type(type_);
        ret
    }
    #[deprecated = "Use `set_id()` instead."]
    pub fn id(&mut self, val: &::js_sys::Object) -> &mut Self {
        self.set_id(val);
        self
    }
    #[deprecated = "Use `set_transports()` instead."]
    pub fn transports(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_transports(val);
        self
    }
    #[cfg(feature = "PublicKeyCredentialType")]
    #[deprecated = "Use `set_type()` instead."]
    pub fn type_(&mut self, val: PublicKeyCredentialType) -> &mut Self {
        self.set_type(val);
        self
    }
}
