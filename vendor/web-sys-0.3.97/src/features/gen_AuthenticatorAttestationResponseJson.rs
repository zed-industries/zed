#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "AuthenticatorAttestationResponseJSON"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AuthenticatorAttestationResponseJson` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticatorAttestationResponseJson`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type AuthenticatorAttestationResponseJson;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `attestationObject` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticatorAttestationResponseJson`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "attestationObject")]
    pub fn get_attestation_object(
        this: &AuthenticatorAttestationResponseJson,
    ) -> ::alloc::string::String;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `attestationObject` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticatorAttestationResponseJson`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "attestationObject")]
    pub fn set_attestation_object(this: &AuthenticatorAttestationResponseJson, val: &str);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `authenticatorData` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticatorAttestationResponseJson`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "authenticatorData")]
    pub fn get_authenticator_data(
        this: &AuthenticatorAttestationResponseJson,
    ) -> ::alloc::string::String;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `authenticatorData` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticatorAttestationResponseJson`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "authenticatorData")]
    pub fn set_authenticator_data(this: &AuthenticatorAttestationResponseJson, val: &str);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `clientDataJSON` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticatorAttestationResponseJson`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "clientDataJSON")]
    pub fn get_client_data_json(
        this: &AuthenticatorAttestationResponseJson,
    ) -> ::alloc::string::String;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `clientDataJSON` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticatorAttestationResponseJson`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "clientDataJSON")]
    pub fn set_client_data_json(this: &AuthenticatorAttestationResponseJson, val: &str);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `publicKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticatorAttestationResponseJson`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "publicKey")]
    pub fn get_public_key(
        this: &AuthenticatorAttestationResponseJson,
    ) -> Option<::alloc::string::String>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `publicKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticatorAttestationResponseJson`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "publicKey")]
    pub fn set_public_key(this: &AuthenticatorAttestationResponseJson, val: &str);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `publicKeyAlgorithm` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticatorAttestationResponseJson`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "publicKeyAlgorithm")]
    pub fn get_public_key_algorithm(this: &AuthenticatorAttestationResponseJson) -> f64;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `publicKeyAlgorithm` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticatorAttestationResponseJson`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "publicKeyAlgorithm")]
    pub fn set_public_key_algorithm(this: &AuthenticatorAttestationResponseJson, val: i32);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `publicKeyAlgorithm` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticatorAttestationResponseJson`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "publicKeyAlgorithm")]
    pub fn set_public_key_algorithm_f64(this: &AuthenticatorAttestationResponseJson, val: f64);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `transports` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticatorAttestationResponseJson`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "transports")]
    pub fn get_transports(
        this: &AuthenticatorAttestationResponseJson,
    ) -> ::js_sys::Array<::js_sys::JsString>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `transports` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticatorAttestationResponseJson`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "transports")]
    pub fn set_transports(this: &AuthenticatorAttestationResponseJson, val: &[::js_sys::JsString]);
}
#[cfg(web_sys_unstable_apis)]
impl AuthenticatorAttestationResponseJson {
    #[doc = "Construct a new `AuthenticatorAttestationResponseJson`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticatorAttestationResponseJson`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(
        attestation_object: &str,
        authenticator_data: &str,
        client_data_json: &str,
        public_key_algorithm: i32,
        transports: &[::js_sys::JsString],
    ) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_attestation_object(attestation_object);
        ret.set_authenticator_data(authenticator_data);
        ret.set_client_data_json(client_data_json);
        ret.set_public_key_algorithm(public_key_algorithm);
        ret.set_transports(transports);
        ret
    }
    #[doc = "Construct a new `AuthenticatorAttestationResponseJson`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticatorAttestationResponseJson`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new_with_f64(
        attestation_object: &str,
        authenticator_data: &str,
        client_data_json: &str,
        public_key_algorithm: f64,
        transports: &[::js_sys::JsString],
    ) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_attestation_object(attestation_object);
        ret.set_authenticator_data(authenticator_data);
        ret.set_client_data_json(client_data_json);
        ret.set_public_key_algorithm_f64(public_key_algorithm);
        ret.set_transports(transports);
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_attestation_object()` instead."]
    pub fn attestation_object(&mut self, val: &str) -> &mut Self {
        self.set_attestation_object(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_authenticator_data()` instead."]
    pub fn authenticator_data(&mut self, val: &str) -> &mut Self {
        self.set_authenticator_data(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_client_data_json()` instead."]
    pub fn client_data_json(&mut self, val: &str) -> &mut Self {
        self.set_client_data_json(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_public_key()` instead."]
    pub fn public_key(&mut self, val: &str) -> &mut Self {
        self.set_public_key(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_public_key_algorithm()` instead."]
    pub fn public_key_algorithm(&mut self, val: i32) -> &mut Self {
        self.set_public_key_algorithm(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_transports()` instead."]
    pub fn transports(&mut self, val: &[::js_sys::JsString]) -> &mut Self {
        self.set_transports(val);
        self
    }
}
