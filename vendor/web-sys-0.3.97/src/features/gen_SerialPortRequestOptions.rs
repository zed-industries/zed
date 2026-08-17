#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "SerialPortRequestOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SerialPortRequestOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SerialPortRequestOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type SerialPortRequestOptions;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "SerialPortFilter")]
    #[doc = "Get the `filters` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SerialPortFilter`, `SerialPortRequestOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "filters")]
    pub fn get_filters(
        this: &SerialPortRequestOptions,
    ) -> Option<::js_sys::Array<SerialPortFilter>>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "SerialPortFilter")]
    #[doc = "Change the `filters` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SerialPortFilter`, `SerialPortRequestOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "filters")]
    pub fn set_filters(this: &SerialPortRequestOptions, val: &[SerialPortFilter]);
}
#[cfg(web_sys_unstable_apis)]
impl SerialPortRequestOptions {
    #[doc = "Construct a new `SerialPortRequestOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SerialPortRequestOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "SerialPortFilter")]
    #[deprecated = "Use `set_filters()` instead."]
    pub fn filters(&mut self, val: &[SerialPortFilter]) -> &mut Self {
        self.set_filters(val);
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for SerialPortRequestOptions {
    fn default() -> Self {
        Self::new()
    }
}
