#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "WebTransportDatagramStats")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WebTransportDatagramStats` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportDatagramStats`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type WebTransportDatagramStats;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `droppedIncoming` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportDatagramStats`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "droppedIncoming")]
    pub fn get_dropped_incoming(this: &WebTransportDatagramStats) -> Option<f64>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `droppedIncoming` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportDatagramStats`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "droppedIncoming")]
    pub fn set_dropped_incoming(this: &WebTransportDatagramStats, val: u32);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `droppedIncoming` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportDatagramStats`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "droppedIncoming")]
    pub fn set_dropped_incoming_f64(this: &WebTransportDatagramStats, val: f64);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `expiredOutgoing` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportDatagramStats`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "expiredOutgoing")]
    pub fn get_expired_outgoing(this: &WebTransportDatagramStats) -> Option<f64>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `expiredOutgoing` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportDatagramStats`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "expiredOutgoing")]
    pub fn set_expired_outgoing(this: &WebTransportDatagramStats, val: u32);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `expiredOutgoing` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportDatagramStats`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "expiredOutgoing")]
    pub fn set_expired_outgoing_f64(this: &WebTransportDatagramStats, val: f64);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `lostOutgoing` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportDatagramStats`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "lostOutgoing")]
    pub fn get_lost_outgoing(this: &WebTransportDatagramStats) -> Option<f64>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `lostOutgoing` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportDatagramStats`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "lostOutgoing")]
    pub fn set_lost_outgoing(this: &WebTransportDatagramStats, val: u32);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `lostOutgoing` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportDatagramStats`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "lostOutgoing")]
    pub fn set_lost_outgoing_f64(this: &WebTransportDatagramStats, val: f64);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportDatagramStats`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "timestamp")]
    pub fn get_timestamp(this: &WebTransportDatagramStats) -> Option<f64>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportDatagramStats`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "timestamp")]
    pub fn set_timestamp(this: &WebTransportDatagramStats, val: f64);
}
#[cfg(web_sys_unstable_apis)]
impl WebTransportDatagramStats {
    #[doc = "Construct a new `WebTransportDatagramStats`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportDatagramStats`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_dropped_incoming()` instead."]
    pub fn dropped_incoming(&mut self, val: u32) -> &mut Self {
        self.set_dropped_incoming(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_expired_outgoing()` instead."]
    pub fn expired_outgoing(&mut self, val: u32) -> &mut Self {
        self.set_expired_outgoing(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_lost_outgoing()` instead."]
    pub fn lost_outgoing(&mut self, val: u32) -> &mut Self {
        self.set_lost_outgoing(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_timestamp()` instead."]
    pub fn timestamp(&mut self, val: f64) -> &mut Self {
        self.set_timestamp(val);
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for WebTransportDatagramStats {
    fn default() -> Self {
        Self::new()
    }
}
