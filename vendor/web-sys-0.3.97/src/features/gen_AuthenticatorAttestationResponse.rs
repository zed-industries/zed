#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "AuthenticatorResponse",
        extends = "::js_sys::Object",
        js_name = "AuthenticatorAttestationResponse",
        typescript_type = "AuthenticatorAttestationResponse"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AuthenticatorAttestationResponse` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AuthenticatorAttestationResponse)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticatorAttestationResponse`*"]
    pub type AuthenticatorAttestationResponse;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "AuthenticatorAttestationResponse",
        js_name = "attestationObject"
    )]
    #[doc = "Getter for the `attestationObject` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AuthenticatorAttestationResponse/attestationObject)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticatorAttestationResponse`*"]
    pub fn attestation_object(this: &AuthenticatorAttestationResponse) -> ::js_sys::ArrayBuffer;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "AuthenticatorAttestationResponse",
        js_name = "getAuthenticatorData"
    )]
    #[doc = "The `getAuthenticatorData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AuthenticatorAttestationResponse/getAuthenticatorData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticatorAttestationResponse`*"]
    pub fn get_authenticator_data(
        this: &AuthenticatorAttestationResponse,
    ) -> Result<::js_sys::ArrayBuffer, JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "AuthenticatorAttestationResponse",
        js_name = "getPublicKey"
    )]
    #[doc = "The `getPublicKey()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AuthenticatorAttestationResponse/getPublicKey)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticatorAttestationResponse`*"]
    pub fn get_public_key(
        this: &AuthenticatorAttestationResponse,
    ) -> Result<Option<::js_sys::ArrayBuffer>, JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "AuthenticatorAttestationResponse",
        js_name = "getPublicKeyAlgorithm"
    )]
    #[doc = "The `getPublicKeyAlgorithm()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AuthenticatorAttestationResponse/getPublicKeyAlgorithm)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticatorAttestationResponse`*"]
    pub fn get_public_key_algorithm(
        this: &AuthenticatorAttestationResponse,
    ) -> Result<i32, JsValue>;
    #[wasm_bindgen(
        method,
        js_class = "AuthenticatorAttestationResponse",
        js_name = "getTransports"
    )]
    #[doc = "The `getTransports()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AuthenticatorAttestationResponse/getTransports)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticatorAttestationResponse`*"]
    pub fn get_transports(this: &AuthenticatorAttestationResponse) -> ::js_sys::Array;
}
