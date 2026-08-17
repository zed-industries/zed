#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "RTCAnswerOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcAnswerOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcAnswerOptions`*"]
    pub type RtcAnswerOptions;
}
impl RtcAnswerOptions {
    #[doc = "Construct a new `RtcAnswerOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcAnswerOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
}
impl Default for RtcAnswerOptions {
    fn default() -> Self {
        Self::new()
    }
}
