#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "MediaKeysPolicy")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaKeysPolicy` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeysPolicy`*"]
    pub type MediaKeysPolicy;
    #[doc = "Get the `minHdcpVersion` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeysPolicy`*"]
    #[wasm_bindgen(method, getter = "minHdcpVersion")]
    pub fn get_min_hdcp_version(this: &MediaKeysPolicy) -> Option<::alloc::string::String>;
    #[doc = "Change the `minHdcpVersion` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeysPolicy`*"]
    #[wasm_bindgen(method, setter = "minHdcpVersion")]
    pub fn set_min_hdcp_version(this: &MediaKeysPolicy, val: &str);
}
impl MediaKeysPolicy {
    #[doc = "Construct a new `MediaKeysPolicy`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeysPolicy`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_min_hdcp_version()` instead."]
    pub fn min_hdcp_version(&mut self, val: &str) -> &mut Self {
        self.set_min_hdcp_version(val);
        self
    }
}
impl Default for MediaKeysPolicy {
    fn default() -> Self {
        Self::new()
    }
}
