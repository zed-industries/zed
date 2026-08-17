#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "CredentialsContainer",
        typescript_type = "CredentialsContainer"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CredentialsContainer` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CredentialsContainer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CredentialsContainer`*"]
    pub type CredentialsContainer;
    #[wasm_bindgen(catch, method, js_class = "CredentialsContainer")]
    #[doc = "The `create()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CredentialsContainer/create)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CredentialsContainer`*"]
    pub fn create(this: &CredentialsContainer) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(feature = "CredentialCreationOptions")]
    #[wasm_bindgen(catch, method, js_class = "CredentialsContainer", js_name = "create")]
    #[doc = "The `create()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CredentialsContainer/create)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CredentialCreationOptions`, `CredentialsContainer`*"]
    pub fn create_with_options(
        this: &CredentialsContainer,
        options: &CredentialCreationOptions,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "CredentialsContainer")]
    #[doc = "The `get()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CredentialsContainer/get)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CredentialsContainer`*"]
    pub fn get(this: &CredentialsContainer) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(feature = "CredentialRequestOptions")]
    #[wasm_bindgen(catch, method, js_class = "CredentialsContainer", js_name = "get")]
    #[doc = "The `get()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CredentialsContainer/get)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CredentialRequestOptions`, `CredentialsContainer`*"]
    pub fn get_with_options(
        this: &CredentialsContainer,
        options: &CredentialRequestOptions,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "CredentialsContainer",
        js_name = "preventSilentAccess"
    )]
    #[doc = "The `preventSilentAccess()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CredentialsContainer/preventSilentAccess)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CredentialsContainer`*"]
    pub fn prevent_silent_access(this: &CredentialsContainer)
        -> Result<::js_sys::Promise, JsValue>;
    #[cfg(feature = "Credential")]
    #[wasm_bindgen(catch, method, js_class = "CredentialsContainer")]
    #[doc = "The `store()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CredentialsContainer/store)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Credential`, `CredentialsContainer`*"]
    pub fn store(
        this: &CredentialsContainer,
        credential: &Credential,
    ) -> Result<::js_sys::Promise, JsValue>;
}
