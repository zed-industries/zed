#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "PublicKeyCredentialUserEntity"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PublicKeyCredentialUserEntity` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialUserEntity`*"]
    pub type PublicKeyCredentialUserEntity;
    #[doc = "Get the `icon` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialUserEntity`*"]
    #[deprecated]
    #[wasm_bindgen(method, getter = "icon")]
    pub fn get_icon(this: &PublicKeyCredentialUserEntity) -> Option<::alloc::string::String>;
    #[doc = "Change the `icon` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialUserEntity`*"]
    #[deprecated]
    #[wasm_bindgen(method, setter = "icon")]
    pub fn set_icon(this: &PublicKeyCredentialUserEntity, val: &str);
    #[doc = "Get the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialUserEntity`*"]
    #[wasm_bindgen(method, getter = "name")]
    pub fn get_name(this: &PublicKeyCredentialUserEntity) -> ::alloc::string::String;
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialUserEntity`*"]
    #[wasm_bindgen(method, setter = "name")]
    pub fn set_name(this: &PublicKeyCredentialUserEntity, val: &str);
    #[doc = "Get the `displayName` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialUserEntity`*"]
    #[wasm_bindgen(method, getter = "displayName")]
    pub fn get_display_name(this: &PublicKeyCredentialUserEntity) -> ::alloc::string::String;
    #[doc = "Change the `displayName` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialUserEntity`*"]
    #[wasm_bindgen(method, setter = "displayName")]
    pub fn set_display_name(this: &PublicKeyCredentialUserEntity, val: &str);
    #[doc = "Get the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialUserEntity`*"]
    #[wasm_bindgen(method, getter = "id")]
    pub fn get_id(this: &PublicKeyCredentialUserEntity) -> ::js_sys::Object;
    #[doc = "Change the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialUserEntity`*"]
    #[wasm_bindgen(method, setter = "id")]
    pub fn set_id(this: &PublicKeyCredentialUserEntity, val: &::js_sys::Object);
    #[doc = "Change the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialUserEntity`*"]
    #[wasm_bindgen(method, setter = "id")]
    pub fn set_id_buffer_source(this: &PublicKeyCredentialUserEntity, val: &::js_sys::Object);
    #[doc = "Change the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialUserEntity`*"]
    #[wasm_bindgen(method, setter = "id")]
    pub fn set_id_u8_slice(this: &PublicKeyCredentialUserEntity, val: &mut [u8]);
    #[doc = "Change the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialUserEntity`*"]
    #[wasm_bindgen(method, setter = "id")]
    pub fn set_id_u8_array(this: &PublicKeyCredentialUserEntity, val: &::js_sys::Uint8Array);
}
impl PublicKeyCredentialUserEntity {
    #[doc = "Construct a new `PublicKeyCredentialUserEntity`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialUserEntity`*"]
    pub fn new(name: &str, display_name: &str, id: &::js_sys::Object) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_name(name);
        ret.set_display_name(display_name);
        ret.set_id(id);
        ret
    }
    #[doc = "Construct a new `PublicKeyCredentialUserEntity`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialUserEntity`*"]
    pub fn new_with_u8_slice(name: &str, display_name: &str, id: &mut [u8]) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_name(name);
        ret.set_display_name(display_name);
        ret.set_id_u8_slice(id);
        ret
    }
    #[doc = "Construct a new `PublicKeyCredentialUserEntity`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialUserEntity`*"]
    pub fn new_with_u8_array(name: &str, display_name: &str, id: &::js_sys::Uint8Array) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_name(name);
        ret.set_display_name(display_name);
        ret.set_id_u8_array(id);
        ret
    }
    #[deprecated = "Use `set_icon()` instead."]
    pub fn icon(&mut self, val: &str) -> &mut Self {
        self.set_icon(val);
        self
    }
    #[deprecated = "Use `set_name()` instead."]
    pub fn name(&mut self, val: &str) -> &mut Self {
        self.set_name(val);
        self
    }
    #[deprecated = "Use `set_display_name()` instead."]
    pub fn display_name(&mut self, val: &str) -> &mut Self {
        self.set_display_name(val);
        self
    }
    #[deprecated = "Use `set_id()` instead."]
    pub fn id(&mut self, val: &::js_sys::Object) -> &mut Self {
        self.set_id(val);
        self
    }
}
