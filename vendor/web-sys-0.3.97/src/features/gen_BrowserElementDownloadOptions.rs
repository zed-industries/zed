#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "BrowserElementDownloadOptions"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `BrowserElementDownloadOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BrowserElementDownloadOptions`*"]
    pub type BrowserElementDownloadOptions;
    #[doc = "Get the `filename` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BrowserElementDownloadOptions`*"]
    #[wasm_bindgen(method, getter = "filename")]
    pub fn get_filename(this: &BrowserElementDownloadOptions) -> Option<::alloc::string::String>;
    #[doc = "Change the `filename` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BrowserElementDownloadOptions`*"]
    #[wasm_bindgen(method, setter = "filename")]
    pub fn set_filename(this: &BrowserElementDownloadOptions, val: Option<&str>);
    #[doc = "Get the `referrer` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BrowserElementDownloadOptions`*"]
    #[wasm_bindgen(method, getter = "referrer")]
    pub fn get_referrer(this: &BrowserElementDownloadOptions) -> Option<::alloc::string::String>;
    #[doc = "Change the `referrer` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BrowserElementDownloadOptions`*"]
    #[wasm_bindgen(method, setter = "referrer")]
    pub fn set_referrer(this: &BrowserElementDownloadOptions, val: Option<&str>);
}
impl BrowserElementDownloadOptions {
    #[doc = "Construct a new `BrowserElementDownloadOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BrowserElementDownloadOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_filename()` instead."]
    pub fn filename(&mut self, val: Option<&str>) -> &mut Self {
        self.set_filename(val);
        self
    }
    #[deprecated = "Use `set_referrer()` instead."]
    pub fn referrer(&mut self, val: Option<&str>) -> &mut Self {
        self.set_referrer(val);
        self
    }
}
impl Default for BrowserElementDownloadOptions {
    fn default() -> Self {
        Self::new()
    }
}
