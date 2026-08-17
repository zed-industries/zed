#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "FileSystemEntryCallback")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `FileSystemEntryCallback` dictionary.\n\n*This API requires the following crate features to be activated: `FileSystemEntryCallback`*"]
    pub type FileSystemEntryCallback;
    #[doc = "Get the `handleEvent` field of this object.\n\n*This API requires the following crate features to be activated: `FileSystemEntryCallback`*"]
    #[wasm_bindgen(method, getter = "handleEvent")]
    pub fn get_handle_event(this: &FileSystemEntryCallback) -> Option<::js_sys::Function>;
    #[doc = "Change the `handleEvent` field of this object.\n\n*This API requires the following crate features to be activated: `FileSystemEntryCallback`*"]
    #[wasm_bindgen(method, setter = "handleEvent")]
    pub fn set_handle_event(this: &FileSystemEntryCallback, val: &::js_sys::Function);
}
impl FileSystemEntryCallback {
    #[doc = "Construct a new `FileSystemEntryCallback`.\n\n*This API requires the following crate features to be activated: `FileSystemEntryCallback`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_handle_event()` instead."]
    pub fn handle_event(&mut self, val: &::js_sys::Function) -> &mut Self {
        self.set_handle_event(val);
        self
    }
}
impl Default for FileSystemEntryCallback {
    fn default() -> Self {
        Self::new()
    }
}
