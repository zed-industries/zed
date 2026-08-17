#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "FileSystemFlags")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `FileSystemFlags` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemFlags`*"]
    pub type FileSystemFlags;
    #[doc = "Get the `create` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemFlags`*"]
    #[wasm_bindgen(method, getter = "create")]
    pub fn get_create(this: &FileSystemFlags) -> Option<bool>;
    #[doc = "Change the `create` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemFlags`*"]
    #[wasm_bindgen(method, setter = "create")]
    pub fn set_create(this: &FileSystemFlags, val: bool);
    #[doc = "Get the `exclusive` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemFlags`*"]
    #[wasm_bindgen(method, getter = "exclusive")]
    pub fn get_exclusive(this: &FileSystemFlags) -> Option<bool>;
    #[doc = "Change the `exclusive` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemFlags`*"]
    #[wasm_bindgen(method, setter = "exclusive")]
    pub fn set_exclusive(this: &FileSystemFlags, val: bool);
}
impl FileSystemFlags {
    #[doc = "Construct a new `FileSystemFlags`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemFlags`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_create()` instead."]
    pub fn create(&mut self, val: bool) -> &mut Self {
        self.set_create(val);
        self
    }
    #[deprecated = "Use `set_exclusive()` instead."]
    pub fn exclusive(&mut self, val: bool) -> &mut Self {
        self.set_exclusive(val);
        self
    }
}
impl Default for FileSystemFlags {
    fn default() -> Self {
        Self::new()
    }
}
