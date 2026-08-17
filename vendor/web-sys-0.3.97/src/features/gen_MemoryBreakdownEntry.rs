#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "MemoryBreakdownEntry")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MemoryBreakdownEntry` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MemoryBreakdownEntry`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type MemoryBreakdownEntry;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "MemoryAttribution")]
    #[doc = "Get the `attribution` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MemoryAttribution`, `MemoryBreakdownEntry`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "attribution")]
    pub fn get_attribution(
        this: &MemoryBreakdownEntry,
    ) -> Option<::js_sys::Array<MemoryAttribution>>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "MemoryAttribution")]
    #[doc = "Change the `attribution` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MemoryAttribution`, `MemoryBreakdownEntry`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "attribution")]
    pub fn set_attribution(this: &MemoryBreakdownEntry, val: &[MemoryAttribution]);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `bytes` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MemoryBreakdownEntry`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "bytes")]
    pub fn get_bytes(this: &MemoryBreakdownEntry) -> Option<f64>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `bytes` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MemoryBreakdownEntry`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "bytes")]
    pub fn set_bytes(this: &MemoryBreakdownEntry, val: u32);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `bytes` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MemoryBreakdownEntry`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "bytes")]
    pub fn set_bytes_f64(this: &MemoryBreakdownEntry, val: f64);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `types` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MemoryBreakdownEntry`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "types")]
    pub fn get_types(this: &MemoryBreakdownEntry) -> Option<::js_sys::Array<::js_sys::JsString>>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `types` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MemoryBreakdownEntry`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "types")]
    pub fn set_types(this: &MemoryBreakdownEntry, val: &[::js_sys::JsString]);
}
#[cfg(web_sys_unstable_apis)]
impl MemoryBreakdownEntry {
    #[doc = "Construct a new `MemoryBreakdownEntry`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MemoryBreakdownEntry`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "MemoryAttribution")]
    #[deprecated = "Use `set_attribution()` instead."]
    pub fn attribution(&mut self, val: &[MemoryAttribution]) -> &mut Self {
        self.set_attribution(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_bytes()` instead."]
    pub fn bytes(&mut self, val: u32) -> &mut Self {
        self.set_bytes(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_types()` instead."]
    pub fn types(&mut self, val: &[::js_sys::JsString]) -> &mut Self {
        self.set_types(val);
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for MemoryBreakdownEntry {
    fn default() -> Self {
        Self::new()
    }
}
