#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "FilePropertyBag")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `FilePropertyBag` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FilePropertyBag`*"]
    pub type FilePropertyBag;
    #[doc = "Get the `lastModified` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FilePropertyBag`*"]
    #[wasm_bindgen(method, getter = "lastModified")]
    pub fn get_last_modified(this: &FilePropertyBag) -> Option<f64>;
    #[doc = "Change the `lastModified` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FilePropertyBag`*"]
    #[wasm_bindgen(method, setter = "lastModified")]
    pub fn set_last_modified(this: &FilePropertyBag, val: f64);
    #[doc = "Change the `lastModified` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FilePropertyBag`*"]
    #[wasm_bindgen(method, setter = "lastModified")]
    pub fn set_last_modified_i32(this: &FilePropertyBag, val: i32);
    #[doc = "Change the `lastModified` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FilePropertyBag`*"]
    #[wasm_bindgen(method, setter = "lastModified")]
    pub fn set_last_modified_f64(this: &FilePropertyBag, val: f64);
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FilePropertyBag`*"]
    #[wasm_bindgen(method, getter = "type")]
    pub fn get_type(this: &FilePropertyBag) -> Option<::alloc::string::String>;
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FilePropertyBag`*"]
    #[wasm_bindgen(method, setter = "type")]
    pub fn set_type(this: &FilePropertyBag, val: &str);
}
impl FilePropertyBag {
    #[doc = "Construct a new `FilePropertyBag`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FilePropertyBag`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_last_modified()` instead."]
    pub fn last_modified(&mut self, val: f64) -> &mut Self {
        self.set_last_modified(val);
        self
    }
    #[deprecated = "Use `set_type()` instead."]
    pub fn type_(&mut self, val: &str) -> &mut Self {
        self.set_type(val);
        self
    }
}
impl Default for FilePropertyBag {
    fn default() -> Self {
        Self::new()
    }
}
