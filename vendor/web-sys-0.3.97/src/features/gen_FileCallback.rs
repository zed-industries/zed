#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "FileCallback")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `FileCallback` dictionary.\n\n*This API requires the following crate features to be activated: `FileCallback`*"]
    pub type FileCallback;
    #[doc = "Get the `handleEvent` field of this object.\n\n*This API requires the following crate features to be activated: `FileCallback`*"]
    #[wasm_bindgen(method, getter = "handleEvent")]
    pub fn get_handle_event(this: &FileCallback) -> Option<::js_sys::Function>;
    #[doc = "Change the `handleEvent` field of this object.\n\n*This API requires the following crate features to be activated: `FileCallback`*"]
    #[wasm_bindgen(method, setter = "handleEvent")]
    pub fn set_handle_event(this: &FileCallback, val: &::js_sys::Function);
}
impl FileCallback {
    #[doc = "Construct a new `FileCallback`.\n\n*This API requires the following crate features to be activated: `FileCallback`*"]
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
impl Default for FileCallback {
    fn default() -> Self {
        Self::new()
    }
}
