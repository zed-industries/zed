#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "RTCRtcpParameters")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcRtcpParameters` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtcpParameters`*"]
    pub type RtcRtcpParameters;
    #[doc = "Get the `cname` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtcpParameters`*"]
    #[wasm_bindgen(method, getter = "cname")]
    pub fn get_cname(this: &RtcRtcpParameters) -> Option<::alloc::string::String>;
    #[doc = "Change the `cname` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtcpParameters`*"]
    #[wasm_bindgen(method, setter = "cname")]
    pub fn set_cname(this: &RtcRtcpParameters, val: &str);
    #[doc = "Get the `reducedSize` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtcpParameters`*"]
    #[wasm_bindgen(method, getter = "reducedSize")]
    pub fn get_reduced_size(this: &RtcRtcpParameters) -> Option<bool>;
    #[doc = "Change the `reducedSize` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtcpParameters`*"]
    #[wasm_bindgen(method, setter = "reducedSize")]
    pub fn set_reduced_size(this: &RtcRtcpParameters, val: bool);
}
impl RtcRtcpParameters {
    #[doc = "Construct a new `RtcRtcpParameters`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtcpParameters`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_cname()` instead."]
    pub fn cname(&mut self, val: &str) -> &mut Self {
        self.set_cname(val);
        self
    }
    #[deprecated = "Use `set_reduced_size()` instead."]
    pub fn reduced_size(&mut self, val: bool) -> &mut Self {
        self.set_reduced_size(val);
        self
    }
}
impl Default for RtcRtcpParameters {
    fn default() -> Self {
        Self::new()
    }
}
