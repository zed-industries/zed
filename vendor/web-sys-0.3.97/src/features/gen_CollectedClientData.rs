#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "CollectedClientData")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CollectedClientData` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CollectedClientData`*"]
    pub type CollectedClientData;
    #[doc = "Get the `challenge` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CollectedClientData`*"]
    #[wasm_bindgen(method, getter = "challenge")]
    pub fn get_challenge(this: &CollectedClientData) -> ::alloc::string::String;
    #[doc = "Change the `challenge` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CollectedClientData`*"]
    #[wasm_bindgen(method, setter = "challenge")]
    pub fn set_challenge(this: &CollectedClientData, val: &str);
    #[cfg(feature = "AuthenticationExtensionsClientInputs")]
    #[doc = "Get the `clientExtensions` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientInputs`, `CollectedClientData`*"]
    #[deprecated]
    #[wasm_bindgen(method, getter = "clientExtensions")]
    pub fn get_client_extensions(
        this: &CollectedClientData,
    ) -> Option<AuthenticationExtensionsClientInputs>;
    #[cfg(feature = "AuthenticationExtensionsClientInputs")]
    #[doc = "Change the `clientExtensions` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientInputs`, `CollectedClientData`*"]
    #[deprecated]
    #[wasm_bindgen(method, setter = "clientExtensions")]
    pub fn set_client_extensions(
        this: &CollectedClientData,
        val: &AuthenticationExtensionsClientInputs,
    );
    #[doc = "Get the `crossOrigin` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CollectedClientData`*"]
    #[wasm_bindgen(method, getter = "crossOrigin")]
    pub fn get_cross_origin(this: &CollectedClientData) -> Option<bool>;
    #[doc = "Change the `crossOrigin` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CollectedClientData`*"]
    #[wasm_bindgen(method, setter = "crossOrigin")]
    pub fn set_cross_origin(this: &CollectedClientData, val: bool);
    #[doc = "Get the `hashAlgorithm` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CollectedClientData`*"]
    #[deprecated]
    #[wasm_bindgen(method, getter = "hashAlgorithm")]
    pub fn get_hash_algorithm(this: &CollectedClientData) -> ::alloc::string::String;
    #[doc = "Change the `hashAlgorithm` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CollectedClientData`*"]
    #[deprecated]
    #[wasm_bindgen(method, setter = "hashAlgorithm")]
    pub fn set_hash_algorithm(this: &CollectedClientData, val: &str);
    #[doc = "Get the `origin` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CollectedClientData`*"]
    #[wasm_bindgen(method, getter = "origin")]
    pub fn get_origin(this: &CollectedClientData) -> ::alloc::string::String;
    #[doc = "Change the `origin` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CollectedClientData`*"]
    #[wasm_bindgen(method, setter = "origin")]
    pub fn set_origin(this: &CollectedClientData, val: &str);
    #[cfg(feature = "TokenBinding")]
    #[doc = "Get the `tokenBinding` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CollectedClientData`, `TokenBinding`*"]
    #[wasm_bindgen(method, getter = "tokenBinding")]
    pub fn get_token_binding(this: &CollectedClientData) -> Option<TokenBinding>;
    #[cfg(feature = "TokenBinding")]
    #[doc = "Change the `tokenBinding` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CollectedClientData`, `TokenBinding`*"]
    #[wasm_bindgen(method, setter = "tokenBinding")]
    pub fn set_token_binding(this: &CollectedClientData, val: &TokenBinding);
    #[doc = "Get the `tokenBindingId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CollectedClientData`*"]
    #[deprecated]
    #[wasm_bindgen(method, getter = "tokenBindingId")]
    pub fn get_token_binding_id(this: &CollectedClientData) -> Option<::alloc::string::String>;
    #[doc = "Change the `tokenBindingId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CollectedClientData`*"]
    #[deprecated]
    #[wasm_bindgen(method, setter = "tokenBindingId")]
    pub fn set_token_binding_id(this: &CollectedClientData, val: &str);
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CollectedClientData`*"]
    #[wasm_bindgen(method, getter = "type")]
    pub fn get_type(this: &CollectedClientData) -> ::alloc::string::String;
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CollectedClientData`*"]
    #[wasm_bindgen(method, setter = "type")]
    pub fn set_type(this: &CollectedClientData, val: &str);
}
impl CollectedClientData {
    #[doc = "Construct a new `CollectedClientData`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CollectedClientData`*"]
    pub fn new(challenge: &str, hash_algorithm: &str, origin: &str, type_: &str) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_challenge(challenge);
        ret.set_hash_algorithm(hash_algorithm);
        ret.set_origin(origin);
        ret.set_type(type_);
        ret
    }
    #[deprecated = "Use `set_challenge()` instead."]
    pub fn challenge(&mut self, val: &str) -> &mut Self {
        self.set_challenge(val);
        self
    }
    #[cfg(feature = "AuthenticationExtensionsClientInputs")]
    #[deprecated = "Use `set_client_extensions()` instead."]
    pub fn client_extensions(&mut self, val: &AuthenticationExtensionsClientInputs) -> &mut Self {
        self.set_client_extensions(val);
        self
    }
    #[deprecated = "Use `set_cross_origin()` instead."]
    pub fn cross_origin(&mut self, val: bool) -> &mut Self {
        self.set_cross_origin(val);
        self
    }
    #[deprecated = "Use `set_hash_algorithm()` instead."]
    pub fn hash_algorithm(&mut self, val: &str) -> &mut Self {
        self.set_hash_algorithm(val);
        self
    }
    #[deprecated = "Use `set_origin()` instead."]
    pub fn origin(&mut self, val: &str) -> &mut Self {
        self.set_origin(val);
        self
    }
    #[cfg(feature = "TokenBinding")]
    #[deprecated = "Use `set_token_binding()` instead."]
    pub fn token_binding(&mut self, val: &TokenBinding) -> &mut Self {
        self.set_token_binding(val);
        self
    }
    #[deprecated = "Use `set_token_binding_id()` instead."]
    pub fn token_binding_id(&mut self, val: &str) -> &mut Self {
        self.set_token_binding_id(val);
        self
    }
    #[deprecated = "Use `set_type()` instead."]
    pub fn type_(&mut self, val: &str) -> &mut Self {
        self.set_type(val);
        self
    }
}
