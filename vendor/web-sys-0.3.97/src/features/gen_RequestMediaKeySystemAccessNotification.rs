#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "RequestMediaKeySystemAccessNotification"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RequestMediaKeySystemAccessNotification` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RequestMediaKeySystemAccessNotification`*"]
    pub type RequestMediaKeySystemAccessNotification;
    #[doc = "Get the `keySystem` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RequestMediaKeySystemAccessNotification`*"]
    #[wasm_bindgen(method, getter = "keySystem")]
    pub fn get_key_system(
        this: &RequestMediaKeySystemAccessNotification,
    ) -> ::alloc::string::String;
    #[doc = "Change the `keySystem` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RequestMediaKeySystemAccessNotification`*"]
    #[wasm_bindgen(method, setter = "keySystem")]
    pub fn set_key_system(this: &RequestMediaKeySystemAccessNotification, val: &str);
    #[cfg(feature = "MediaKeySystemStatus")]
    #[doc = "Get the `status` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySystemStatus`, `RequestMediaKeySystemAccessNotification`*"]
    #[wasm_bindgen(method, getter = "status")]
    pub fn get_status(this: &RequestMediaKeySystemAccessNotification) -> MediaKeySystemStatus;
    #[cfg(feature = "MediaKeySystemStatus")]
    #[doc = "Change the `status` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySystemStatus`, `RequestMediaKeySystemAccessNotification`*"]
    #[wasm_bindgen(method, setter = "status")]
    pub fn set_status(this: &RequestMediaKeySystemAccessNotification, val: MediaKeySystemStatus);
}
impl RequestMediaKeySystemAccessNotification {
    #[cfg(feature = "MediaKeySystemStatus")]
    #[doc = "Construct a new `RequestMediaKeySystemAccessNotification`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySystemStatus`, `RequestMediaKeySystemAccessNotification`*"]
    pub fn new(key_system: &str, status: MediaKeySystemStatus) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_key_system(key_system);
        ret.set_status(status);
        ret
    }
    #[deprecated = "Use `set_key_system()` instead."]
    pub fn key_system(&mut self, val: &str) -> &mut Self {
        self.set_key_system(val);
        self
    }
    #[cfg(feature = "MediaKeySystemStatus")]
    #[deprecated = "Use `set_status()` instead."]
    pub fn status(&mut self, val: MediaKeySystemStatus) -> &mut Self {
        self.set_status(val);
        self
    }
}
