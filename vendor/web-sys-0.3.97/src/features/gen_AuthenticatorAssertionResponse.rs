#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "AuthenticatorResponse",
        extends = "::js_sys::Object",
        js_name = "AuthenticatorAssertionResponse",
        typescript_type = "AuthenticatorAssertionResponse"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AuthenticatorAssertionResponse` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AuthenticatorAssertionResponse)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticatorAssertionResponse`*"]
    pub type AuthenticatorAssertionResponse;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "AuthenticatorAssertionResponse",
        js_name = "authenticatorData"
    )]
    #[doc = "Getter for the `authenticatorData` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AuthenticatorAssertionResponse/authenticatorData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticatorAssertionResponse`*"]
    pub fn authenticator_data(this: &AuthenticatorAssertionResponse) -> ::js_sys::ArrayBuffer;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "AuthenticatorAssertionResponse",
        js_name = "signature"
    )]
    #[doc = "Getter for the `signature` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AuthenticatorAssertionResponse/signature)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticatorAssertionResponse`*"]
    pub fn signature(this: &AuthenticatorAssertionResponse) -> ::js_sys::ArrayBuffer;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "AuthenticatorAssertionResponse",
        js_name = "userHandle"
    )]
    #[doc = "Getter for the `userHandle` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AuthenticatorAssertionResponse/userHandle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticatorAssertionResponse`*"]
    pub fn user_handle(this: &AuthenticatorAssertionResponse) -> Option<::js_sys::ArrayBuffer>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "AuthenticatorAssertionResponse",
        js_name = "attestationObject"
    )]
    #[doc = "Getter for the `attestationObject` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AuthenticatorAssertionResponse/attestationObject)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticatorAssertionResponse`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn attestation_object(
        this: &AuthenticatorAssertionResponse,
    ) -> Option<::js_sys::ArrayBuffer>;
}
