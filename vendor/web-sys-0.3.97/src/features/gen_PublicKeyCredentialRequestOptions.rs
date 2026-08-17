#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "PublicKeyCredentialRequestOptions"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PublicKeyCredentialRequestOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialRequestOptions`*"]
    pub type PublicKeyCredentialRequestOptions;
    #[doc = "Get the `allowCredentials` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialRequestOptions`*"]
    #[wasm_bindgen(method, getter = "allowCredentials")]
    pub fn get_allow_credentials(
        this: &PublicKeyCredentialRequestOptions,
    ) -> Option<::js_sys::Array>;
    #[doc = "Change the `allowCredentials` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialRequestOptions`*"]
    #[wasm_bindgen(method, setter = "allowCredentials")]
    pub fn set_allow_credentials(
        this: &PublicKeyCredentialRequestOptions,
        val: &::wasm_bindgen::JsValue,
    );
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `attestation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialRequestOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "attestation")]
    pub fn get_attestation(
        this: &PublicKeyCredentialRequestOptions,
    ) -> Option<::alloc::string::String>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `attestation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialRequestOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "attestation")]
    pub fn set_attestation(this: &PublicKeyCredentialRequestOptions, val: &str);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `attestationFormats` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialRequestOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "attestationFormats")]
    pub fn get_attestation_formats(
        this: &PublicKeyCredentialRequestOptions,
    ) -> Option<::js_sys::Array<::js_sys::JsString>>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `attestationFormats` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialRequestOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "attestationFormats")]
    pub fn set_attestation_formats(
        this: &PublicKeyCredentialRequestOptions,
        val: &[::js_sys::JsString],
    );
    #[doc = "Get the `challenge` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialRequestOptions`*"]
    #[wasm_bindgen(method, getter = "challenge")]
    pub fn get_challenge(this: &PublicKeyCredentialRequestOptions) -> ::js_sys::Object;
    #[doc = "Change the `challenge` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialRequestOptions`*"]
    #[wasm_bindgen(method, setter = "challenge")]
    pub fn set_challenge(this: &PublicKeyCredentialRequestOptions, val: &::js_sys::Object);
    #[doc = "Change the `challenge` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialRequestOptions`*"]
    #[wasm_bindgen(method, setter = "challenge")]
    pub fn set_challenge_buffer_source(
        this: &PublicKeyCredentialRequestOptions,
        val: &::js_sys::Object,
    );
    #[doc = "Change the `challenge` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialRequestOptions`*"]
    #[wasm_bindgen(method, setter = "challenge")]
    pub fn set_challenge_u8_slice(this: &PublicKeyCredentialRequestOptions, val: &mut [u8]);
    #[doc = "Change the `challenge` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialRequestOptions`*"]
    #[wasm_bindgen(method, setter = "challenge")]
    pub fn set_challenge_u8_array(
        this: &PublicKeyCredentialRequestOptions,
        val: &::js_sys::Uint8Array,
    );
    #[cfg(feature = "AuthenticationExtensionsClientInputs")]
    #[doc = "Get the `extensions` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientInputs`, `PublicKeyCredentialRequestOptions`*"]
    #[wasm_bindgen(method, getter = "extensions")]
    pub fn get_extensions(
        this: &PublicKeyCredentialRequestOptions,
    ) -> Option<AuthenticationExtensionsClientInputs>;
    #[cfg(feature = "AuthenticationExtensionsClientInputs")]
    #[doc = "Change the `extensions` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientInputs`, `PublicKeyCredentialRequestOptions`*"]
    #[wasm_bindgen(method, setter = "extensions")]
    pub fn set_extensions(
        this: &PublicKeyCredentialRequestOptions,
        val: &AuthenticationExtensionsClientInputs,
    );
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `hints` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialRequestOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "hints")]
    pub fn get_hints(
        this: &PublicKeyCredentialRequestOptions,
    ) -> Option<::js_sys::Array<::js_sys::JsString>>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `hints` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialRequestOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "hints")]
    pub fn set_hints(this: &PublicKeyCredentialRequestOptions, val: &[::js_sys::JsString]);
    #[doc = "Get the `rpId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialRequestOptions`*"]
    #[wasm_bindgen(method, getter = "rpId")]
    pub fn get_rp_id(this: &PublicKeyCredentialRequestOptions) -> Option<::alloc::string::String>;
    #[doc = "Change the `rpId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialRequestOptions`*"]
    #[wasm_bindgen(method, setter = "rpId")]
    pub fn set_rp_id(this: &PublicKeyCredentialRequestOptions, val: &str);
    #[doc = "Get the `timeout` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialRequestOptions`*"]
    #[wasm_bindgen(method, getter = "timeout")]
    pub fn get_timeout(this: &PublicKeyCredentialRequestOptions) -> Option<u32>;
    #[doc = "Change the `timeout` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialRequestOptions`*"]
    #[wasm_bindgen(method, setter = "timeout")]
    pub fn set_timeout(this: &PublicKeyCredentialRequestOptions, val: u32);
    #[cfg(feature = "UserVerificationRequirement")]
    #[doc = "Get the `userVerification` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialRequestOptions`, `UserVerificationRequirement`*"]
    #[wasm_bindgen(method, getter = "userVerification")]
    pub fn get_user_verification(
        this: &PublicKeyCredentialRequestOptions,
    ) -> Option<UserVerificationRequirement>;
    #[cfg(feature = "UserVerificationRequirement")]
    #[doc = "Change the `userVerification` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialRequestOptions`, `UserVerificationRequirement`*"]
    #[wasm_bindgen(method, setter = "userVerification")]
    pub fn set_user_verification(
        this: &PublicKeyCredentialRequestOptions,
        val: UserVerificationRequirement,
    );
}
impl PublicKeyCredentialRequestOptions {
    #[doc = "Construct a new `PublicKeyCredentialRequestOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialRequestOptions`*"]
    pub fn new(challenge: &::js_sys::Object) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_challenge(challenge);
        ret
    }
    #[doc = "Construct a new `PublicKeyCredentialRequestOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialRequestOptions`*"]
    pub fn new_with_u8_slice(challenge: &mut [u8]) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_challenge_u8_slice(challenge);
        ret
    }
    #[doc = "Construct a new `PublicKeyCredentialRequestOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialRequestOptions`*"]
    pub fn new_with_u8_array(challenge: &::js_sys::Uint8Array) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_challenge_u8_array(challenge);
        ret
    }
    #[deprecated = "Use `set_allow_credentials()` instead."]
    pub fn allow_credentials(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_allow_credentials(val);
        self
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
    #[deprecated = "Use `set_challenge()` instead."]
    pub fn challenge(&mut self, val: &::js_sys::Object) -> &mut Self {
        self.set_challenge(val);
        self
    }
    #[cfg(feature = "AuthenticationExtensionsClientInputs")]
    #[deprecated = "Use `set_extensions()` instead."]
    pub fn extensions(&mut self, val: &AuthenticationExtensionsClientInputs) -> &mut Self {
        self.set_extensions(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_hints()` instead."]
    pub fn hints(&mut self, val: &[::js_sys::JsString]) -> &mut Self {
        self.set_hints(val);
        self
    }
    #[deprecated = "Use `set_rp_id()` instead."]
    pub fn rp_id(&mut self, val: &str) -> &mut Self {
        self.set_rp_id(val);
        self
    }
    #[deprecated = "Use `set_timeout()` instead."]
    pub fn timeout(&mut self, val: u32) -> &mut Self {
        self.set_timeout(val);
        self
    }
    #[cfg(feature = "UserVerificationRequirement")]
    #[deprecated = "Use `set_user_verification()` instead."]
    pub fn user_verification(&mut self, val: UserVerificationRequirement) -> &mut Self {
        self.set_user_verification(val);
        self
    }
}
