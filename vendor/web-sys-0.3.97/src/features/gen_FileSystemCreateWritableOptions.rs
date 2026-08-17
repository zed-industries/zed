#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "FileSystemCreateWritableOptions"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `FileSystemCreateWritableOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemCreateWritableOptions`*"]
    pub type FileSystemCreateWritableOptions;
    #[doc = "Get the `keepExistingData` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemCreateWritableOptions`*"]
    #[wasm_bindgen(method, getter = "keepExistingData")]
    pub fn get_keep_existing_data(this: &FileSystemCreateWritableOptions) -> Option<bool>;
    #[doc = "Change the `keepExistingData` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemCreateWritableOptions`*"]
    #[wasm_bindgen(method, setter = "keepExistingData")]
    pub fn set_keep_existing_data(this: &FileSystemCreateWritableOptions, val: bool);
}
impl FileSystemCreateWritableOptions {
    #[doc = "Construct a new `FileSystemCreateWritableOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemCreateWritableOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_keep_existing_data()` instead."]
    pub fn keep_existing_data(&mut self, val: bool) -> &mut Self {
        self.set_keep_existing_data(val);
        self
    }
}
impl Default for FileSystemCreateWritableOptions {
    fn default() -> Self {
        Self::new()
    }
}
