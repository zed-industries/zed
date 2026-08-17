#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "SvcOutputMetadata")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvcOutputMetadata` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvcOutputMetadata`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type SvcOutputMetadata;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `temporalLayerId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvcOutputMetadata`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "temporalLayerId")]
    pub fn get_temporal_layer_id(this: &SvcOutputMetadata) -> Option<u32>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `temporalLayerId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvcOutputMetadata`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "temporalLayerId")]
    pub fn set_temporal_layer_id(this: &SvcOutputMetadata, val: u32);
}
#[cfg(web_sys_unstable_apis)]
impl SvcOutputMetadata {
    #[doc = "Construct a new `SvcOutputMetadata`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvcOutputMetadata`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_temporal_layer_id()` instead."]
    pub fn temporal_layer_id(&mut self, val: u32) -> &mut Self {
        self.set_temporal_layer_id(val);
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for SvcOutputMetadata {
    fn default() -> Self {
        Self::new()
    }
}
