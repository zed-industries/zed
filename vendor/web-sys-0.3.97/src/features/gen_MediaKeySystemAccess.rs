#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "MediaKeySystemAccess",
        typescript_type = "MediaKeySystemAccess"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaKeySystemAccess` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeySystemAccess)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySystemAccess`*"]
    pub type MediaKeySystemAccess;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "MediaKeySystemAccess",
        js_name = "keySystem"
    )]
    #[doc = "Getter for the `keySystem` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeySystemAccess/keySystem)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySystemAccess`*"]
    pub fn key_system(this: &MediaKeySystemAccess) -> ::alloc::string::String;
    #[wasm_bindgen(method, js_class = "MediaKeySystemAccess", js_name = "createMediaKeys")]
    #[doc = "The `createMediaKeys()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeySystemAccess/createMediaKeys)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySystemAccess`*"]
    pub fn create_media_keys(this: &MediaKeySystemAccess) -> ::js_sys::Promise;
    #[cfg(feature = "MediaKeySystemConfiguration")]
    #[wasm_bindgen(
        method,
        js_class = "MediaKeySystemAccess",
        js_name = "getConfiguration"
    )]
    #[doc = "The `getConfiguration()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeySystemAccess/getConfiguration)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySystemAccess`, `MediaKeySystemConfiguration`*"]
    pub fn get_configuration(this: &MediaKeySystemAccess) -> MediaKeySystemConfiguration;
}
