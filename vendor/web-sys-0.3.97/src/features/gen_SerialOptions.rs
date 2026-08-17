#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "SerialOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SerialOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SerialOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type SerialOptions;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `baudRate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SerialOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "baudRate")]
    pub fn get_baud_rate(this: &SerialOptions) -> u32;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `baudRate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SerialOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "baudRate")]
    pub fn set_baud_rate(this: &SerialOptions, val: u32);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `bufferSize` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SerialOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "bufferSize")]
    pub fn get_buffer_size(this: &SerialOptions) -> Option<u32>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `bufferSize` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SerialOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "bufferSize")]
    pub fn set_buffer_size(this: &SerialOptions, val: u32);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `dataBits` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SerialOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "dataBits")]
    pub fn get_data_bits(this: &SerialOptions) -> Option<u8>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `dataBits` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SerialOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "dataBits")]
    pub fn set_data_bits(this: &SerialOptions, val: u8);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "FlowControlType")]
    #[doc = "Get the `flowControl` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FlowControlType`, `SerialOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "flowControl")]
    pub fn get_flow_control(this: &SerialOptions) -> Option<FlowControlType>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "FlowControlType")]
    #[doc = "Change the `flowControl` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FlowControlType`, `SerialOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "flowControl")]
    pub fn set_flow_control(this: &SerialOptions, val: FlowControlType);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "ParityType")]
    #[doc = "Get the `parity` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ParityType`, `SerialOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "parity")]
    pub fn get_parity(this: &SerialOptions) -> Option<ParityType>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "ParityType")]
    #[doc = "Change the `parity` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ParityType`, `SerialOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "parity")]
    pub fn set_parity(this: &SerialOptions, val: ParityType);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `stopBits` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SerialOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "stopBits")]
    pub fn get_stop_bits(this: &SerialOptions) -> Option<u8>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `stopBits` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SerialOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "stopBits")]
    pub fn set_stop_bits(this: &SerialOptions, val: u8);
}
#[cfg(web_sys_unstable_apis)]
impl SerialOptions {
    #[doc = "Construct a new `SerialOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SerialOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(baud_rate: u32) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_baud_rate(baud_rate);
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_baud_rate()` instead."]
    pub fn baud_rate(&mut self, val: u32) -> &mut Self {
        self.set_baud_rate(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_buffer_size()` instead."]
    pub fn buffer_size(&mut self, val: u32) -> &mut Self {
        self.set_buffer_size(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_data_bits()` instead."]
    pub fn data_bits(&mut self, val: u8) -> &mut Self {
        self.set_data_bits(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "FlowControlType")]
    #[deprecated = "Use `set_flow_control()` instead."]
    pub fn flow_control(&mut self, val: FlowControlType) -> &mut Self {
        self.set_flow_control(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "ParityType")]
    #[deprecated = "Use `set_parity()` instead."]
    pub fn parity(&mut self, val: ParityType) -> &mut Self {
        self.set_parity(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_stop_bits()` instead."]
    pub fn stop_bits(&mut self, val: u8) -> &mut Self {
        self.set_stop_bits(val);
        self
    }
}
