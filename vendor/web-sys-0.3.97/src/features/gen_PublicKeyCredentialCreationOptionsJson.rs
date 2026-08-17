#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "PublicKeyCredentialCreationOptionsJSON"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PublicKeyCredentialCreationOptionsJson` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialCreationOptionsJson`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type PublicKeyCredentialCreationOptionsJson;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `attestation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialCreationOptionsJson`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "attestation")]
    pub fn get_attestation(
        this: &PublicKeyCredentialCreationOptionsJson,
    ) -> Option<::alloc::string::String>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `attestation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialCreationOptionsJson`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "attestation")]
    pub fn set_attestation(this: &PublicKeyCredentialCreationOptionsJson, val: &str);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `attestationFormats` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialCreationOptionsJson`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "attestationFormats")]
    pub fn get_attestation_formats(
        this: &PublicKeyCredentialCreationOptionsJson,
    ) -> Option<::js_sys::Array<::js_sys::JsString>>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `attestationFormats` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialCreationOptionsJson`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "attestationFormats")]
    pub fn set_attestation_formats(
        this: &PublicKeyCredentialCreationOptionsJson,
        val: &[::js_sys::JsString],
    );
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "AuthenticatorSelectionCriteria")]
    #[doc = "Get the `authenticatorSelection` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticatorSelectionCriteria`, `PublicKeyCredentialCreationOptionsJson`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "authenticatorSelection")]
    pub fn get_authenticator_selection(
        this: &PublicKeyCredentialCreationOptionsJson,
    ) -> Option<AuthenticatorSelectionCriteria>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "AuthenticatorSelectionCriteria")]
    #[doc = "Change the `authenticatorSelection` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticatorSelectionCriteria`, `PublicKeyCredentialCreationOptionsJson`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "authenticatorSelection")]
    pub fn set_authenticator_selection(
        this: &PublicKeyCredentialCreationOptionsJson,
        val: &AuthenticatorSelectionCriteria,
    );
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `challenge` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialCreationOptionsJson`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "challenge")]
    pub fn get_challenge(this: &PublicKeyCredentialCreationOptionsJson) -> ::alloc::string::String;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `challenge` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialCreationOptionsJson`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "challenge")]
    pub fn set_challenge(this: &PublicKeyCredentialCreationOptionsJson, val: &str);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "PublicKeyCredentialDescriptorJson")]
    #[doc = "Get the `excludeCredentials` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialCreationOptionsJson`, `PublicKeyCredentialDescriptorJson`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "excludeCredentials")]
    pub fn get_exclude_credentials(
        this: &PublicKeyCredentialCreationOptionsJson,
    ) -> Option<::js_sys::Array<PublicKeyCredentialDescriptorJson>>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "PublicKeyCredentialDescriptorJson")]
    #[doc = "Change the `excludeCredentials` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialCreationOptionsJson`, `PublicKeyCredentialDescriptorJson`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "excludeCredentials")]
    pub fn set_exclude_credentials(
        this: &PublicKeyCredentialCreationOptionsJson,
        val: &[PublicKeyCredentialDescriptorJson],
    );
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "AuthenticationExtensionsClientInputsJson")]
    #[doc = "Get the `extensions` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientInputsJson`, `PublicKeyCredentialCreationOptionsJson`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "extensions")]
    pub fn get_extensions(
        this: &PublicKeyCredentialCreationOptionsJson,
    ) -> Option<AuthenticationExtensionsClientInputsJson>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "AuthenticationExtensionsClientInputsJson")]
    #[doc = "Change the `extensions` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientInputsJson`, `PublicKeyCredentialCreationOptionsJson`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "extensions")]
    pub fn set_extensions(
        this: &PublicKeyCredentialCreationOptionsJson,
        val: &AuthenticationExtensionsClientInputsJson,
    );
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `hints` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialCreationOptionsJson`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "hints")]
    pub fn get_hints(
        this: &PublicKeyCredentialCreationOptionsJson,
    ) -> Option<::js_sys::Array<::js_sys::JsString>>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `hints` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialCreationOptionsJson`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "hints")]
    pub fn set_hints(this: &PublicKeyCredentialCreationOptionsJson, val: &[::js_sys::JsString]);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "PublicKeyCredentialParameters")]
    #[doc = "Get the `pubKeyCredParams` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialCreationOptionsJson`, `PublicKeyCredentialParameters`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "pubKeyCredParams")]
    pub fn get_pub_key_cred_params(
        this: &PublicKeyCredentialCreationOptionsJson,
    ) -> ::js_sys::Array<PublicKeyCredentialParameters>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "PublicKeyCredentialParameters")]
    #[doc = "Change the `pubKeyCredParams` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialCreationOptionsJson`, `PublicKeyCredentialParameters`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "pubKeyCredParams")]
    pub fn set_pub_key_cred_params(
        this: &PublicKeyCredentialCreationOptionsJson,
        val: &[PublicKeyCredentialParameters],
    );
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "PublicKeyCredentialRpEntity")]
    #[doc = "Get the `rp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialCreationOptionsJson`, `PublicKeyCredentialRpEntity`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "rp")]
    pub fn get_rp(this: &PublicKeyCredentialCreationOptionsJson) -> PublicKeyCredentialRpEntity;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "PublicKeyCredentialRpEntity")]
    #[doc = "Change the `rp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialCreationOptionsJson`, `PublicKeyCredentialRpEntity`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "rp")]
    pub fn set_rp(this: &PublicKeyCredentialCreationOptionsJson, val: &PublicKeyCredentialRpEntity);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `timeout` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialCreationOptionsJson`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "timeout")]
    pub fn get_timeout(this: &PublicKeyCredentialCreationOptionsJson) -> Option<u32>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `timeout` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialCreationOptionsJson`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "timeout")]
    pub fn set_timeout(this: &PublicKeyCredentialCreationOptionsJson, val: u32);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "PublicKeyCredentialUserEntityJson")]
    #[doc = "Get the `user` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialCreationOptionsJson`, `PublicKeyCredentialUserEntityJson`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "user")]
    pub fn get_user(
        this: &PublicKeyCredentialCreationOptionsJson,
    ) -> PublicKeyCredentialUserEntityJson;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "PublicKeyCredentialUserEntityJson")]
    #[doc = "Change the `user` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialCreationOptionsJson`, `PublicKeyCredentialUserEntityJson`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "user")]
    pub fn set_user(
        this: &PublicKeyCredentialCreationOptionsJson,
        val: &PublicKeyCredentialUserEntityJson,
    );
}
#[cfg(web_sys_unstable_apis)]
impl PublicKeyCredentialCreationOptionsJson {
    #[cfg(all(
        feature = "PublicKeyCredentialParameters",
        feature = "PublicKeyCredentialRpEntity",
        feature = "PublicKeyCredentialUserEntityJson",
    ))]
    #[doc = "Construct a new `PublicKeyCredentialCreationOptionsJson`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialCreationOptionsJson`, `PublicKeyCredentialParameters`, `PublicKeyCredentialRpEntity`, `PublicKeyCredentialUserEntityJson`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(
        challenge: &str,
        pub_key_cred_params: &[PublicKeyCredentialParameters],
        rp: &PublicKeyCredentialRpEntity,
        user: &PublicKeyCredentialUserEntityJson,
    ) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_challenge(challenge);
        ret.set_pub_key_cred_params(pub_key_cred_params);
        ret.set_rp(rp);
        ret.set_user(user);
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_attestation()` instead."]
    pub fn attestation(&mut self, val: &str) -> &mut Self {
        self.set_attestation(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_attestation_formats()` instead."]
    pub fn attestation_formats(&mut self, val: &[::js_sys::JsString]) -> &mut Self {
        self.set_attestation_formats(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "AuthenticatorSelectionCriteria")]
    #[deprecated = "Use `set_authenticator_selection()` instead."]
    pub fn authenticator_selection(&mut self, val: &AuthenticatorSelectionCriteria) -> &mut Self {
        self.set_authenticator_selection(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_challenge()` instead."]
    pub fn challenge(&mut self, val: &str) -> &mut Self {
        self.set_challenge(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "PublicKeyCredentialDescriptorJson")]
    #[deprecated = "Use `set_exclude_credentials()` instead."]
    pub fn exclude_credentials(&mut self, val: &[PublicKeyCredentialDescriptorJson]) -> &mut Self {
        self.set_exclude_credentials(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "AuthenticationExtensionsClientInputsJson")]
    #[deprecated = "Use `set_extensions()` instead."]
    pub fn extensions(&mut self, val: &AuthenticationExtensionsClientInputsJson) -> &mut Self {
        self.set_extensions(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_hints()` instead."]
    pub fn hints(&mut self, val: &[::js_sys::JsString]) -> &mut Self {
        self.set_hints(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "PublicKeyCredentialParameters")]
    #[deprecated = "Use `set_pub_key_cred_params()` instead."]
    pub fn pub_key_cred_params(&mut self, val: &[PublicKeyCredentialParameters]) -> &mut Self {
        self.set_pub_key_cred_params(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "PublicKeyCredentialRpEntity")]
    #[deprecated = "Use `set_rp()` instead."]
    pub fn rp(&mut self, val: &PublicKeyCredentialRpEntity) -> &mut Self {
        self.set_rp(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_timeout()` instead."]
    pub fn timeout(&mut self, val: u32) -> &mut Self {
        self.set_timeout(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "PublicKeyCredentialUserEntityJson")]
    #[deprecated = "Use `set_user()` instead."]
    pub fn user(&mut self, val: &PublicKeyCredentialUserEntityJson) -> &mut Self {
        self.set_user(val);
        self
    }
}
