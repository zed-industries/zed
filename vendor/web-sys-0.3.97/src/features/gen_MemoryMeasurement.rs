#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "MemoryMeasurement")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MemoryMeasurement` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MemoryMeasurement`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type MemoryMeasurement;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "MemoryBreakdownEntry")]
    #[doc = "Get the `breakdown` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MemoryBreakdownEntry`, `MemoryMeasurement`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "breakdown")]
    pub fn get_breakdown(this: &MemoryMeasurement)
        -> Option<::js_sys::Array<MemoryBreakdownEntry>>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "MemoryBreakdownEntry")]
    #[doc = "Change the `breakdown` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MemoryBreakdownEntry`, `MemoryMeasurement`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "breakdown")]
    pub fn set_breakdown(this: &MemoryMeasurement, val: &[MemoryBreakdownEntry]);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `bytes` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MemoryMeasurement`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "bytes")]
    pub fn get_bytes(this: &MemoryMeasurement) -> Option<f64>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `bytes` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MemoryMeasurement`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "bytes")]
    pub fn set_bytes(this: &MemoryMeasurement, val: u32);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `bytes` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MemoryMeasurement`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "bytes")]
    pub fn set_bytes_f64(this: &MemoryMeasurement, val: f64);
}
#[cfg(web_sys_unstable_apis)]
impl MemoryMeasurement {
    #[doc = "Construct a new `MemoryMeasurement`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MemoryMeasurement`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "MemoryBreakdownEntry")]
    #[deprecated = "Use `set_breakdown()` instead."]
    pub fn breakdown(&mut self, val: &[MemoryBreakdownEntry]) -> &mut Self {
        self.set_breakdown(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_bytes()` instead."]
    pub fn bytes(&mut self, val: u32) -> &mut Self {
        self.set_bytes(val);
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for MemoryMeasurement {
    fn default() -> Self {
        Self::new()
    }
}
