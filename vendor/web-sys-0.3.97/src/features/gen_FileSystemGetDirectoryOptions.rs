#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "FileSystemGetDirectoryOptions"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `FileSystemGetDirectoryOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemGetDirectoryOptions`*"]
    pub type FileSystemGetDirectoryOptions;
    #[doc = "Get the `create` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemGetDirectoryOptions`*"]
    #[wasm_bindgen(method, getter = "create")]
    pub fn get_create(this: &FileSystemGetDirectoryOptions) -> Option<bool>;
    #[doc = "Change the `create` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemGetDirectoryOptions`*"]
    #[wasm_bindgen(method, setter = "create")]
    pub fn set_create(this: &FileSystemGetDirectoryOptions, val: bool);
}
impl FileSystemGetDirectoryOptions {
    #[doc = "Construct a new `FileSystemGetDirectoryOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemGetDirectoryOptions`*"]
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
}
impl Default for FileSystemGetDirectoryOptions {
    fn default() -> Self {
        Self::new()
    }
}
