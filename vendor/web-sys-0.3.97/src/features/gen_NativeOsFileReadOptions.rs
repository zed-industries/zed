#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "NativeOSFileReadOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `NativeOsFileReadOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NativeOsFileReadOptions`*"]
    pub type NativeOsFileReadOptions;
    #[doc = "Get the `bytes` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NativeOsFileReadOptions`*"]
    #[wasm_bindgen(method, getter = "bytes")]
    pub fn get_bytes(this: &NativeOsFileReadOptions) -> Option<f64>;
    #[doc = "Change the `bytes` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NativeOsFileReadOptions`*"]
    #[wasm_bindgen(method, setter = "bytes")]
    pub fn set_bytes(this: &NativeOsFileReadOptions, val: Option<f64>);
    #[doc = "Change the `bytes` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NativeOsFileReadOptions`*"]
    #[wasm_bindgen(method, setter = "bytes")]
    pub fn set_bytes_opt_u32(this: &NativeOsFileReadOptions, val: Option<u32>);
    #[doc = "Change the `bytes` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NativeOsFileReadOptions`*"]
    #[wasm_bindgen(method, setter = "bytes")]
    pub fn set_bytes_opt_f64(this: &NativeOsFileReadOptions, val: Option<f64>);
    #[doc = "Get the `encoding` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NativeOsFileReadOptions`*"]
    #[wasm_bindgen(method, getter = "encoding")]
    pub fn get_encoding(this: &NativeOsFileReadOptions) -> Option<::alloc::string::String>;
    #[doc = "Change the `encoding` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NativeOsFileReadOptions`*"]
    #[wasm_bindgen(method, setter = "encoding")]
    pub fn set_encoding(this: &NativeOsFileReadOptions, val: Option<&str>);
}
impl NativeOsFileReadOptions {
    #[doc = "Construct a new `NativeOsFileReadOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NativeOsFileReadOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_bytes()` instead."]
    pub fn bytes(&mut self, val: Option<f64>) -> &mut Self {
        self.set_bytes(val);
        self
    }
    #[deprecated = "Use `set_encoding()` instead."]
    pub fn encoding(&mut self, val: Option<&str>) -> &mut Self {
        self.set_encoding(val);
        self
    }
}
impl Default for NativeOsFileReadOptions {
    fn default() -> Self {
        Self::new()
    }
}
