#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "FileSystemRemoveOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `FileSystemRemoveOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemRemoveOptions`*"]
    pub type FileSystemRemoveOptions;
    #[doc = "Get the `recursive` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemRemoveOptions`*"]
    #[wasm_bindgen(method, getter = "recursive")]
    pub fn get_recursive(this: &FileSystemRemoveOptions) -> Option<bool>;
    #[doc = "Change the `recursive` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemRemoveOptions`*"]
    #[wasm_bindgen(method, setter = "recursive")]
    pub fn set_recursive(this: &FileSystemRemoveOptions, val: bool);
}
impl FileSystemRemoveOptions {
    #[doc = "Construct a new `FileSystemRemoveOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemRemoveOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_recursive()` instead."]
    pub fn recursive(&mut self, val: bool) -> &mut Self {
        self.set_recursive(val);
        self
    }
}
impl Default for FileSystemRemoveOptions {
    fn default() -> Self {
        Self::new()
    }
}
