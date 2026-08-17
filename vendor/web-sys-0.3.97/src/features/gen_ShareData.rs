#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "ShareData")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ShareData` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ShareData`*"]
    pub type ShareData;
    #[doc = "Get the `files` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ShareData`*"]
    #[wasm_bindgen(method, getter = "files")]
    pub fn get_files(this: &ShareData) -> Option<::js_sys::Array>;
    #[doc = "Change the `files` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ShareData`*"]
    #[wasm_bindgen(method, setter = "files")]
    pub fn set_files(this: &ShareData, val: &::wasm_bindgen::JsValue);
    #[doc = "Get the `text` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ShareData`*"]
    #[wasm_bindgen(method, getter = "text")]
    pub fn get_text(this: &ShareData) -> Option<::alloc::string::String>;
    #[doc = "Change the `text` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ShareData`*"]
    #[wasm_bindgen(method, setter = "text")]
    pub fn set_text(this: &ShareData, val: &str);
    #[doc = "Get the `title` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ShareData`*"]
    #[wasm_bindgen(method, getter = "title")]
    pub fn get_title(this: &ShareData) -> Option<::alloc::string::String>;
    #[doc = "Change the `title` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ShareData`*"]
    #[wasm_bindgen(method, setter = "title")]
    pub fn set_title(this: &ShareData, val: &str);
    #[doc = "Get the `url` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ShareData`*"]
    #[wasm_bindgen(method, getter = "url")]
    pub fn get_url(this: &ShareData) -> Option<::alloc::string::String>;
    #[doc = "Change the `url` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ShareData`*"]
    #[wasm_bindgen(method, setter = "url")]
    pub fn set_url(this: &ShareData, val: &str);
}
impl ShareData {
    #[doc = "Construct a new `ShareData`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ShareData`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_files()` instead."]
    pub fn files(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_files(val);
        self
    }
    #[deprecated = "Use `set_text()` instead."]
    pub fn text(&mut self, val: &str) -> &mut Self {
        self.set_text(val);
        self
    }
    #[deprecated = "Use `set_title()` instead."]
    pub fn title(&mut self, val: &str) -> &mut Self {
        self.set_title(val);
        self
    }
    #[deprecated = "Use `set_url()` instead."]
    pub fn url(&mut self, val: &str) -> &mut Self {
        self.set_url(val);
        self
    }
}
impl Default for ShareData {
    fn default() -> Self {
        Self::new()
    }
}
