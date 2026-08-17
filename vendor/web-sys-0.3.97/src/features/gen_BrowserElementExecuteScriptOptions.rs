#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "BrowserElementExecuteScriptOptions"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `BrowserElementExecuteScriptOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BrowserElementExecuteScriptOptions`*"]
    pub type BrowserElementExecuteScriptOptions;
    #[doc = "Get the `origin` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BrowserElementExecuteScriptOptions`*"]
    #[wasm_bindgen(method, getter = "origin")]
    pub fn get_origin(this: &BrowserElementExecuteScriptOptions)
        -> Option<::alloc::string::String>;
    #[doc = "Change the `origin` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BrowserElementExecuteScriptOptions`*"]
    #[wasm_bindgen(method, setter = "origin")]
    pub fn set_origin(this: &BrowserElementExecuteScriptOptions, val: Option<&str>);
    #[doc = "Get the `url` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BrowserElementExecuteScriptOptions`*"]
    #[wasm_bindgen(method, getter = "url")]
    pub fn get_url(this: &BrowserElementExecuteScriptOptions) -> Option<::alloc::string::String>;
    #[doc = "Change the `url` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BrowserElementExecuteScriptOptions`*"]
    #[wasm_bindgen(method, setter = "url")]
    pub fn set_url(this: &BrowserElementExecuteScriptOptions, val: Option<&str>);
}
impl BrowserElementExecuteScriptOptions {
    #[doc = "Construct a new `BrowserElementExecuteScriptOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BrowserElementExecuteScriptOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_origin()` instead."]
    pub fn origin(&mut self, val: Option<&str>) -> &mut Self {
        self.set_origin(val);
        self
    }
    #[deprecated = "Use `set_url()` instead."]
    pub fn url(&mut self, val: Option<&str>) -> &mut Self {
        self.set_url(val);
        self
    }
}
impl Default for BrowserElementExecuteScriptOptions {
    fn default() -> Self {
        Self::new()
    }
}
