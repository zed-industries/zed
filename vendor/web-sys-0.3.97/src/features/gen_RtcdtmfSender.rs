#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "EventTarget",
        extends = "::js_sys::Object",
        js_name = "RTCDTMFSender",
        typescript_type = "RTCDTMFSender"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcdtmfSender` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDTMFSender)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcdtmfSender`*"]
    pub type RtcdtmfSender;
    #[wasm_bindgen(method, getter, js_class = "RTCDTMFSender", js_name = "ontonechange")]
    #[doc = "Getter for the `ontonechange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDTMFSender/ontonechange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcdtmfSender`*"]
    pub fn ontonechange(this: &RtcdtmfSender) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "RTCDTMFSender", js_name = "ontonechange")]
    #[doc = "Setter for the `ontonechange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDTMFSender/ontonechange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcdtmfSender`*"]
    pub fn set_ontonechange(this: &RtcdtmfSender, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "RTCDTMFSender", js_name = "toneBuffer")]
    #[doc = "Getter for the `toneBuffer` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDTMFSender/toneBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcdtmfSender`*"]
    pub fn tone_buffer(this: &RtcdtmfSender) -> ::alloc::string::String;
    #[wasm_bindgen(method, js_class = "RTCDTMFSender", js_name = "insertDTMF")]
    #[doc = "The `insertDTMF()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDTMFSender/insertDTMF)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcdtmfSender`*"]
    pub fn insert_dtmf(this: &RtcdtmfSender, tones: &str);
    #[wasm_bindgen(method, js_class = "RTCDTMFSender", js_name = "insertDTMF")]
    #[doc = "The `insertDTMF()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDTMFSender/insertDTMF)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcdtmfSender`*"]
    pub fn insert_dtmf_with_duration(this: &RtcdtmfSender, tones: &str, duration: u32);
    #[wasm_bindgen(method, js_class = "RTCDTMFSender", js_name = "insertDTMF")]
    #[doc = "The `insertDTMF()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDTMFSender/insertDTMF)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcdtmfSender`*"]
    pub fn insert_dtmf_with_duration_and_inter_tone_gap(
        this: &RtcdtmfSender,
        tones: &str,
        duration: u32,
        inter_tone_gap: u32,
    );
}
