#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "RTCDataChannelInit")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcDataChannelInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannelInit`*"]
    pub type RtcDataChannelInit;
    #[doc = "Get the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannelInit`*"]
    #[wasm_bindgen(method, getter = "id")]
    pub fn get_id(this: &RtcDataChannelInit) -> Option<u16>;
    #[doc = "Change the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannelInit`*"]
    #[wasm_bindgen(method, setter = "id")]
    pub fn set_id(this: &RtcDataChannelInit, val: u16);
    #[doc = "Get the `maxPacketLifeTime` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannelInit`*"]
    #[wasm_bindgen(method, getter = "maxPacketLifeTime")]
    pub fn get_max_packet_life_time(this: &RtcDataChannelInit) -> Option<u16>;
    #[doc = "Change the `maxPacketLifeTime` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannelInit`*"]
    #[wasm_bindgen(method, setter = "maxPacketLifeTime")]
    pub fn set_max_packet_life_time(this: &RtcDataChannelInit, val: u16);
    #[doc = "Get the `maxRetransmitTime` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannelInit`*"]
    #[wasm_bindgen(method, getter = "maxRetransmitTime")]
    pub fn get_max_retransmit_time(this: &RtcDataChannelInit) -> Option<u16>;
    #[doc = "Change the `maxRetransmitTime` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannelInit`*"]
    #[wasm_bindgen(method, setter = "maxRetransmitTime")]
    pub fn set_max_retransmit_time(this: &RtcDataChannelInit, val: u16);
    #[doc = "Get the `maxRetransmits` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannelInit`*"]
    #[wasm_bindgen(method, getter = "maxRetransmits")]
    pub fn get_max_retransmits(this: &RtcDataChannelInit) -> Option<u16>;
    #[doc = "Change the `maxRetransmits` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannelInit`*"]
    #[wasm_bindgen(method, setter = "maxRetransmits")]
    pub fn set_max_retransmits(this: &RtcDataChannelInit, val: u16);
    #[doc = "Get the `negotiated` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannelInit`*"]
    #[wasm_bindgen(method, getter = "negotiated")]
    pub fn get_negotiated(this: &RtcDataChannelInit) -> Option<bool>;
    #[doc = "Change the `negotiated` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannelInit`*"]
    #[wasm_bindgen(method, setter = "negotiated")]
    pub fn set_negotiated(this: &RtcDataChannelInit, val: bool);
    #[doc = "Get the `ordered` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannelInit`*"]
    #[wasm_bindgen(method, getter = "ordered")]
    pub fn get_ordered(this: &RtcDataChannelInit) -> Option<bool>;
    #[doc = "Change the `ordered` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannelInit`*"]
    #[wasm_bindgen(method, setter = "ordered")]
    pub fn set_ordered(this: &RtcDataChannelInit, val: bool);
    #[doc = "Get the `protocol` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannelInit`*"]
    #[wasm_bindgen(method, getter = "protocol")]
    pub fn get_protocol(this: &RtcDataChannelInit) -> Option<::alloc::string::String>;
    #[doc = "Change the `protocol` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannelInit`*"]
    #[wasm_bindgen(method, setter = "protocol")]
    pub fn set_protocol(this: &RtcDataChannelInit, val: &str);
}
impl RtcDataChannelInit {
    #[doc = "Construct a new `RtcDataChannelInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannelInit`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_id()` instead."]
    pub fn id(&mut self, val: u16) -> &mut Self {
        self.set_id(val);
        self
    }
    #[deprecated = "Use `set_max_packet_life_time()` instead."]
    pub fn max_packet_life_time(&mut self, val: u16) -> &mut Self {
        self.set_max_packet_life_time(val);
        self
    }
    #[deprecated = "Use `set_max_retransmit_time()` instead."]
    pub fn max_retransmit_time(&mut self, val: u16) -> &mut Self {
        self.set_max_retransmit_time(val);
        self
    }
    #[deprecated = "Use `set_max_retransmits()` instead."]
    pub fn max_retransmits(&mut self, val: u16) -> &mut Self {
        self.set_max_retransmits(val);
        self
    }
    #[deprecated = "Use `set_negotiated()` instead."]
    pub fn negotiated(&mut self, val: bool) -> &mut Self {
        self.set_negotiated(val);
        self
    }
    #[deprecated = "Use `set_ordered()` instead."]
    pub fn ordered(&mut self, val: bool) -> &mut Self {
        self.set_ordered(val);
        self
    }
    #[deprecated = "Use `set_protocol()` instead."]
    pub fn protocol(&mut self, val: &str) -> &mut Self {
        self.set_protocol(val);
        self
    }
}
impl Default for RtcDataChannelInit {
    fn default() -> Self {
        Self::new()
    }
}
