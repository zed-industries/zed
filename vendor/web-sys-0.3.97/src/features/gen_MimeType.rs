#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "MimeType",
        typescript_type = "MimeType"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MimeType` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MimeType)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MimeType`*"]
    pub type MimeType;
    #[wasm_bindgen(method, getter, js_class = "MimeType", js_name = "description")]
    #[doc = "Getter for the `description` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MimeType/description)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MimeType`*"]
    pub fn description(this: &MimeType) -> ::alloc::string::String;
    #[cfg(feature = "Plugin")]
    #[wasm_bindgen(method, getter, js_class = "MimeType", js_name = "enabledPlugin")]
    #[doc = "Getter for the `enabledPlugin` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MimeType/enabledPlugin)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MimeType`, `Plugin`*"]
    pub fn enabled_plugin(this: &MimeType) -> Option<Plugin>;
    #[wasm_bindgen(method, getter, js_class = "MimeType", js_name = "suffixes")]
    #[doc = "Getter for the `suffixes` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MimeType/suffixes)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MimeType`*"]
    pub fn suffixes(this: &MimeType) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "MimeType", js_name = "type")]
    #[doc = "Getter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MimeType/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MimeType`*"]
    pub fn type_(this: &MimeType) -> ::alloc::string::String;
}
