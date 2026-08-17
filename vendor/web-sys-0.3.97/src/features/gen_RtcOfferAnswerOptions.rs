#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "RTCOfferAnswerOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcOfferAnswerOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOfferAnswerOptions`*"]
    pub type RtcOfferAnswerOptions;
}
impl RtcOfferAnswerOptions {
    #[doc = "Construct a new `RtcOfferAnswerOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOfferAnswerOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
}
impl Default for RtcOfferAnswerOptions {
    fn default() -> Self {
        Self::new()
    }
}
