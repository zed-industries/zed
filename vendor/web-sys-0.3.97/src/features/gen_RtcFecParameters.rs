#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "RTCFecParameters")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcFecParameters` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcFecParameters`*"]
    pub type RtcFecParameters;
    #[doc = "Get the `ssrc` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcFecParameters`*"]
    #[wasm_bindgen(method, getter = "ssrc")]
    pub fn get_ssrc(this: &RtcFecParameters) -> Option<u32>;
    #[doc = "Change the `ssrc` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcFecParameters`*"]
    #[wasm_bindgen(method, setter = "ssrc")]
    pub fn set_ssrc(this: &RtcFecParameters, val: u32);
}
impl RtcFecParameters {
    #[doc = "Construct a new `RtcFecParameters`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcFecParameters`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_ssrc()` instead."]
    pub fn ssrc(&mut self, val: u32) -> &mut Self {
        self.set_ssrc(val);
        self
    }
}
impl Default for RtcFecParameters {
    fn default() -> Self {
        Self::new()
    }
}
