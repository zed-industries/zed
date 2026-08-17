#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "FetchReadableStreamReadDataArray"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `FetchReadableStreamReadDataArray` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FetchReadableStreamReadDataArray`*"]
    pub type FetchReadableStreamReadDataArray;
    #[doc = "Get the `value` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FetchReadableStreamReadDataArray`*"]
    #[wasm_bindgen(method, getter = "value")]
    pub fn get_value(this: &FetchReadableStreamReadDataArray) -> Option<::alloc::vec::Vec<u8>>;
    #[doc = "Change the `value` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FetchReadableStreamReadDataArray`*"]
    #[wasm_bindgen(method, setter = "value")]
    pub fn set_value(this: &FetchReadableStreamReadDataArray, val: &::js_sys::Uint8Array);
    #[doc = "Change the `value` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FetchReadableStreamReadDataArray`*"]
    #[wasm_bindgen(method, setter = "value")]
    pub fn set_value_u8_slice(this: &FetchReadableStreamReadDataArray, val: &mut [u8]);
    #[doc = "Change the `value` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FetchReadableStreamReadDataArray`*"]
    #[wasm_bindgen(method, setter = "value")]
    pub fn set_value_u8_array(this: &FetchReadableStreamReadDataArray, val: &::js_sys::Uint8Array);
}
impl FetchReadableStreamReadDataArray {
    #[doc = "Construct a new `FetchReadableStreamReadDataArray`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FetchReadableStreamReadDataArray`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_value()` instead."]
    pub fn value(&mut self, val: &::js_sys::Uint8Array) -> &mut Self {
        self.set_value(val);
        self
    }
}
impl Default for FetchReadableStreamReadDataArray {
    fn default() -> Self {
        Self::new()
    }
}
