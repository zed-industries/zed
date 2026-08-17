#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "MediaKeys",
        typescript_type = "MediaKeys"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaKeys` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeys)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeys`*"]
    pub type MediaKeys;
    #[wasm_bindgen(method, getter, js_class = "MediaKeys", js_name = "keySystem")]
    #[doc = "Getter for the `keySystem` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeys/keySystem)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeys`*"]
    pub fn key_system(this: &MediaKeys) -> ::alloc::string::String;
    #[cfg(feature = "MediaKeySession")]
    #[wasm_bindgen(catch, method, js_class = "MediaKeys", js_name = "createSession")]
    #[doc = "The `createSession()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeys/createSession)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySession`, `MediaKeys`*"]
    pub fn create_session(this: &MediaKeys) -> Result<MediaKeySession, JsValue>;
    #[cfg(all(feature = "MediaKeySession", feature = "MediaKeySessionType",))]
    #[wasm_bindgen(catch, method, js_class = "MediaKeys", js_name = "createSession")]
    #[doc = "The `createSession()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeys/createSession)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySession`, `MediaKeySessionType`, `MediaKeys`*"]
    pub fn create_session_with_session_type(
        this: &MediaKeys,
        session_type: MediaKeySessionType,
    ) -> Result<MediaKeySession, JsValue>;
    #[wasm_bindgen(method, js_class = "MediaKeys", js_name = "getStatusForPolicy")]
    #[doc = "The `getStatusForPolicy()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeys/getStatusForPolicy)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeys`*"]
    pub fn get_status_for_policy(this: &MediaKeys) -> ::js_sys::Promise;
    #[cfg(feature = "MediaKeysPolicy")]
    #[wasm_bindgen(method, js_class = "MediaKeys", js_name = "getStatusForPolicy")]
    #[doc = "The `getStatusForPolicy()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeys/getStatusForPolicy)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeys`, `MediaKeysPolicy`*"]
    pub fn get_status_for_policy_with_policy(
        this: &MediaKeys,
        policy: &MediaKeysPolicy,
    ) -> ::js_sys::Promise;
    #[wasm_bindgen(method, js_class = "MediaKeys", js_name = "setServerCertificate")]
    #[doc = "The `setServerCertificate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeys/setServerCertificate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeys`*"]
    pub fn set_server_certificate_with_buffer_source(
        this: &MediaKeys,
        server_certificate: &::js_sys::Object,
    ) -> ::js_sys::Promise;
    #[wasm_bindgen(method, js_class = "MediaKeys", js_name = "setServerCertificate")]
    #[doc = "The `setServerCertificate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeys/setServerCertificate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeys`*"]
    pub fn set_server_certificate_with_u8_array(
        this: &MediaKeys,
        server_certificate: &mut [u8],
    ) -> ::js_sys::Promise;
    #[wasm_bindgen(method, js_class = "MediaKeys", js_name = "setServerCertificate")]
    #[doc = "The `setServerCertificate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeys/setServerCertificate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeys`*"]
    pub fn set_server_certificate_with_js_u8_array(
        this: &MediaKeys,
        server_certificate: &::js_sys::Uint8Array,
    ) -> ::js_sys::Promise;
}
