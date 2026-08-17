#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "FileSystemReadWriteOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `FileSystemReadWriteOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemReadWriteOptions`*"]
    pub type FileSystemReadWriteOptions;
    #[doc = "Get the `at` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemReadWriteOptions`*"]
    #[wasm_bindgen(method, getter = "at")]
    pub fn get_at(this: &FileSystemReadWriteOptions) -> Option<f64>;
    #[doc = "Change the `at` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemReadWriteOptions`*"]
    #[wasm_bindgen(method, setter = "at")]
    pub fn set_at(this: &FileSystemReadWriteOptions, val: f64);
    #[doc = "Change the `at` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemReadWriteOptions`*"]
    #[wasm_bindgen(method, setter = "at")]
    pub fn set_at_u32(this: &FileSystemReadWriteOptions, val: u32);
    #[doc = "Change the `at` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemReadWriteOptions`*"]
    #[wasm_bindgen(method, setter = "at")]
    pub fn set_at_f64(this: &FileSystemReadWriteOptions, val: f64);
}
impl FileSystemReadWriteOptions {
    #[doc = "Construct a new `FileSystemReadWriteOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemReadWriteOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_at()` instead."]
    pub fn at(&mut self, val: f64) -> &mut Self {
        self.set_at(val);
        self
    }
}
impl Default for FileSystemReadWriteOptions {
    fn default() -> Self {
        Self::new()
    }
}
