#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "RTCTrackEventInit")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcTrackEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcTrackEventInit`*"]
    pub type RtcTrackEventInit;
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcTrackEventInit`*"]
    #[wasm_bindgen(method, getter = "bubbles")]
    pub fn get_bubbles(this: &RtcTrackEventInit) -> Option<bool>;
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcTrackEventInit`*"]
    #[wasm_bindgen(method, setter = "bubbles")]
    pub fn set_bubbles(this: &RtcTrackEventInit, val: bool);
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcTrackEventInit`*"]
    #[wasm_bindgen(method, getter = "cancelable")]
    pub fn get_cancelable(this: &RtcTrackEventInit) -> Option<bool>;
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcTrackEventInit`*"]
    #[wasm_bindgen(method, setter = "cancelable")]
    pub fn set_cancelable(this: &RtcTrackEventInit, val: bool);
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcTrackEventInit`*"]
    #[wasm_bindgen(method, getter = "composed")]
    pub fn get_composed(this: &RtcTrackEventInit) -> Option<bool>;
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcTrackEventInit`*"]
    #[wasm_bindgen(method, setter = "composed")]
    pub fn set_composed(this: &RtcTrackEventInit, val: bool);
    #[cfg(feature = "RtcRtpReceiver")]
    #[doc = "Get the `receiver` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpReceiver`, `RtcTrackEventInit`*"]
    #[wasm_bindgen(method, getter = "receiver")]
    pub fn get_receiver(this: &RtcTrackEventInit) -> RtcRtpReceiver;
    #[cfg(feature = "RtcRtpReceiver")]
    #[doc = "Change the `receiver` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpReceiver`, `RtcTrackEventInit`*"]
    #[wasm_bindgen(method, setter = "receiver")]
    pub fn set_receiver(this: &RtcTrackEventInit, val: &RtcRtpReceiver);
    #[doc = "Get the `streams` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcTrackEventInit`*"]
    #[wasm_bindgen(method, getter = "streams")]
    pub fn get_streams(this: &RtcTrackEventInit) -> Option<::js_sys::Array>;
    #[doc = "Change the `streams` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcTrackEventInit`*"]
    #[wasm_bindgen(method, setter = "streams")]
    pub fn set_streams(this: &RtcTrackEventInit, val: &::wasm_bindgen::JsValue);
    #[cfg(feature = "MediaStreamTrack")]
    #[doc = "Get the `track` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamTrack`, `RtcTrackEventInit`*"]
    #[wasm_bindgen(method, getter = "track")]
    pub fn get_track(this: &RtcTrackEventInit) -> MediaStreamTrack;
    #[cfg(feature = "MediaStreamTrack")]
    #[doc = "Change the `track` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamTrack`, `RtcTrackEventInit`*"]
    #[wasm_bindgen(method, setter = "track")]
    pub fn set_track(this: &RtcTrackEventInit, val: &MediaStreamTrack);
    #[cfg(feature = "RtcRtpTransceiver")]
    #[doc = "Get the `transceiver` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpTransceiver`, `RtcTrackEventInit`*"]
    #[wasm_bindgen(method, getter = "transceiver")]
    pub fn get_transceiver(this: &RtcTrackEventInit) -> RtcRtpTransceiver;
    #[cfg(feature = "RtcRtpTransceiver")]
    #[doc = "Change the `transceiver` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpTransceiver`, `RtcTrackEventInit`*"]
    #[wasm_bindgen(method, setter = "transceiver")]
    pub fn set_transceiver(this: &RtcTrackEventInit, val: &RtcRtpTransceiver);
}
impl RtcTrackEventInit {
    #[cfg(all(
        feature = "MediaStreamTrack",
        feature = "RtcRtpReceiver",
        feature = "RtcRtpTransceiver",
    ))]
    #[doc = "Construct a new `RtcTrackEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamTrack`, `RtcRtpReceiver`, `RtcRtpTransceiver`, `RtcTrackEventInit`*"]
    pub fn new(
        receiver: &RtcRtpReceiver,
        track: &MediaStreamTrack,
        transceiver: &RtcRtpTransceiver,
    ) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_receiver(receiver);
        ret.set_track(track);
        ret.set_transceiver(transceiver);
        ret
    }
    #[deprecated = "Use `set_bubbles()` instead."]
    pub fn bubbles(&mut self, val: bool) -> &mut Self {
        self.set_bubbles(val);
        self
    }
    #[deprecated = "Use `set_cancelable()` instead."]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.set_cancelable(val);
        self
    }
    #[deprecated = "Use `set_composed()` instead."]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.set_composed(val);
        self
    }
    #[cfg(feature = "RtcRtpReceiver")]
    #[deprecated = "Use `set_receiver()` instead."]
    pub fn receiver(&mut self, val: &RtcRtpReceiver) -> &mut Self {
        self.set_receiver(val);
        self
    }
    #[deprecated = "Use `set_streams()` instead."]
    pub fn streams(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_streams(val);
        self
    }
    #[cfg(feature = "MediaStreamTrack")]
    #[deprecated = "Use `set_track()` instead."]
    pub fn track(&mut self, val: &MediaStreamTrack) -> &mut Self {
        self.set_track(val);
        self
    }
    #[cfg(feature = "RtcRtpTransceiver")]
    #[deprecated = "Use `set_transceiver()` instead."]
    pub fn transceiver(&mut self, val: &RtcRtpTransceiver) -> &mut Self {
        self.set_transceiver(val);
        self
    }
}
