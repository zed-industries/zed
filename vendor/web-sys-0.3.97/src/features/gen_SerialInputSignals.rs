#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "SerialInputSignals")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SerialInputSignals` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SerialInputSignals`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type SerialInputSignals;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `clearToSend` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SerialInputSignals`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "clearToSend")]
    pub fn get_clear_to_send(this: &SerialInputSignals) -> bool;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `clearToSend` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SerialInputSignals`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "clearToSend")]
    pub fn set_clear_to_send(this: &SerialInputSignals, val: bool);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `dataCarrierDetect` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SerialInputSignals`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "dataCarrierDetect")]
    pub fn get_data_carrier_detect(this: &SerialInputSignals) -> bool;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `dataCarrierDetect` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SerialInputSignals`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "dataCarrierDetect")]
    pub fn set_data_carrier_detect(this: &SerialInputSignals, val: bool);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `dataSetReady` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SerialInputSignals`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "dataSetReady")]
    pub fn get_data_set_ready(this: &SerialInputSignals) -> bool;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `dataSetReady` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SerialInputSignals`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "dataSetReady")]
    pub fn set_data_set_ready(this: &SerialInputSignals, val: bool);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `ringIndicator` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SerialInputSignals`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "ringIndicator")]
    pub fn get_ring_indicator(this: &SerialInputSignals) -> bool;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `ringIndicator` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SerialInputSignals`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "ringIndicator")]
    pub fn set_ring_indicator(this: &SerialInputSignals, val: bool);
}
#[cfg(web_sys_unstable_apis)]
impl SerialInputSignals {
    #[doc = "Construct a new `SerialInputSignals`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SerialInputSignals`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(
        clear_to_send: bool,
        data_carrier_detect: bool,
        data_set_ready: bool,
        ring_indicator: bool,
    ) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_clear_to_send(clear_to_send);
        ret.set_data_carrier_detect(data_carrier_detect);
        ret.set_data_set_ready(data_set_ready);
        ret.set_ring_indicator(ring_indicator);
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_clear_to_send()` instead."]
    pub fn clear_to_send(&mut self, val: bool) -> &mut Self {
        self.set_clear_to_send(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_data_carrier_detect()` instead."]
    pub fn data_carrier_detect(&mut self, val: bool) -> &mut Self {
        self.set_data_carrier_detect(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_data_set_ready()` instead."]
    pub fn data_set_ready(&mut self, val: bool) -> &mut Self {
        self.set_data_set_ready(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_ring_indicator()` instead."]
    pub fn ring_indicator(&mut self, val: bool) -> &mut Self {
        self.set_ring_indicator(val);
        self
    }
}
