#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "Credential",
        extends = "::js_sys::Object",
        js_name = "PublicKeyCredential",
        typescript_type = "PublicKeyCredential"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PublicKeyCredential` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PublicKeyCredential)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredential`*"]
    pub type PublicKeyCredential;
    #[wasm_bindgen(method, getter, js_class = "PublicKeyCredential", js_name = "rawId")]
    #[doc = "Getter for the `rawId` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PublicKeyCredential/rawId)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredential`*"]
    pub fn raw_id(this: &PublicKeyCredential) -> ::js_sys::ArrayBuffer;
    #[cfg(feature = "AuthenticatorResponse")]
    #[wasm_bindgen(method, getter, js_class = "PublicKeyCredential", js_name = "response")]
    #[doc = "Getter for the `response` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PublicKeyCredential/response)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticatorResponse`, `PublicKeyCredential`*"]
    pub fn response(this: &PublicKeyCredential) -> AuthenticatorResponse;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "PublicKeyCredential",
        js_name = "authenticatorAttachment"
    )]
    #[doc = "Getter for the `authenticatorAttachment` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PublicKeyCredential/authenticatorAttachment)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredential`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn authenticator_attachment(this: &PublicKeyCredential) -> Option<::alloc::string::String>;
    #[cfg(feature = "AuthenticationExtensionsClientOutputs")]
    #[wasm_bindgen(
        method,
        js_class = "PublicKeyCredential",
        js_name = "getClientExtensionResults"
    )]
    #[doc = "The `getClientExtensionResults()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PublicKeyCredential/getClientExtensionResults)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientOutputs`, `PublicKeyCredential`*"]
    pub fn get_client_extension_results(
        this: &PublicKeyCredential,
    ) -> AuthenticationExtensionsClientOutputs;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        static_method_of = "PublicKeyCredential",
        js_class = "PublicKeyCredential",
        js_name = "isConditionalMediationAvailable"
    )]
    #[doc = "The `isConditionalMediationAvailable()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PublicKeyCredential/isConditionalMediationAvailable_static)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredential`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn is_conditional_mediation_available() -> ::js_sys::Promise<::js_sys::Boolean>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        static_method_of = "PublicKeyCredential",
        js_class = "PublicKeyCredential",
        js_name = "isPasskeyPlatformAuthenticatorAvailable"
    )]
    #[doc = "The `isPasskeyPlatformAuthenticatorAvailable()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PublicKeyCredential/isPasskeyPlatformAuthenticatorAvailable_static)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredential`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn is_passkey_platform_authenticator_available() -> ::js_sys::Promise<::js_sys::Boolean>;
    #[wasm_bindgen(
        static_method_of = "PublicKeyCredential",
        js_class = "PublicKeyCredential",
        js_name = "isUserVerifyingPlatformAuthenticatorAvailable"
    )]
    #[doc = "The `isUserVerifyingPlatformAuthenticatorAvailable()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PublicKeyCredential/isUserVerifyingPlatformAuthenticatorAvailable_static)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredential`*"]
    pub fn is_user_verifying_platform_authenticator_available() -> ::js_sys::Promise;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(
        feature = "PublicKeyCredentialCreationOptions",
        feature = "PublicKeyCredentialCreationOptionsJson",
    ))]
    #[wasm_bindgen(
        catch,
        static_method_of = "PublicKeyCredential",
        js_class = "PublicKeyCredential",
        js_name = "parseCreationOptionsFromJSON"
    )]
    #[doc = "The `parseCreationOptionsFromJSON()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PublicKeyCredential/parseCreationOptionsFromJSON_static)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredential`, `PublicKeyCredentialCreationOptions`, `PublicKeyCredentialCreationOptionsJson`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn parse_creation_options_from_json(
        options: &PublicKeyCredentialCreationOptionsJson,
    ) -> Result<PublicKeyCredentialCreationOptions, JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(
        feature = "PublicKeyCredentialRequestOptions",
        feature = "PublicKeyCredentialRequestOptionsJson",
    ))]
    #[wasm_bindgen(
        catch,
        static_method_of = "PublicKeyCredential",
        js_class = "PublicKeyCredential",
        js_name = "parseRequestOptionsFromJSON"
    )]
    #[doc = "The `parseRequestOptionsFromJSON()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PublicKeyCredential/parseRequestOptionsFromJSON_static)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredential`, `PublicKeyCredentialRequestOptions`, `PublicKeyCredentialRequestOptionsJson`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn parse_request_options_from_json(
        options: &PublicKeyCredentialRequestOptionsJson,
    ) -> Result<PublicKeyCredentialRequestOptions, JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(catch, method, js_class = "PublicKeyCredential", js_name = "toJSON")]
    #[doc = "The `toJSON()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PublicKeyCredential/toJSON)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredential`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn to_json(this: &PublicKeyCredential) -> Result<::wasm_bindgen::JsValue, JsValue>;
}
