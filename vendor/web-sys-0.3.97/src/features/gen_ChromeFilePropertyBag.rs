#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "ChromeFilePropertyBag")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ChromeFilePropertyBag` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChromeFilePropertyBag`*"]
    pub type ChromeFilePropertyBag;
    #[doc = "Get the `lastModified` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChromeFilePropertyBag`*"]
    #[wasm_bindgen(method, getter = "lastModified")]
    pub fn get_last_modified(this: &ChromeFilePropertyBag) -> Option<f64>;
    #[doc = "Change the `lastModified` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChromeFilePropertyBag`*"]
    #[wasm_bindgen(method, setter = "lastModified")]
    pub fn set_last_modified(this: &ChromeFilePropertyBag, val: f64);
    #[doc = "Change the `lastModified` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChromeFilePropertyBag`*"]
    #[wasm_bindgen(method, setter = "lastModified")]
    pub fn set_last_modified_i32(this: &ChromeFilePropertyBag, val: i32);
    #[doc = "Change the `lastModified` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChromeFilePropertyBag`*"]
    #[wasm_bindgen(method, setter = "lastModified")]
    pub fn set_last_modified_f64(this: &ChromeFilePropertyBag, val: f64);
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChromeFilePropertyBag`*"]
    #[wasm_bindgen(method, getter = "type")]
    pub fn get_type(this: &ChromeFilePropertyBag) -> Option<::alloc::string::String>;
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChromeFilePropertyBag`*"]
    #[wasm_bindgen(method, setter = "type")]
    pub fn set_type(this: &ChromeFilePropertyBag, val: &str);
    #[doc = "Get the `existenceCheck` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChromeFilePropertyBag`*"]
    #[wasm_bindgen(method, getter = "existenceCheck")]
    pub fn get_existence_check(this: &ChromeFilePropertyBag) -> Option<bool>;
    #[doc = "Change the `existenceCheck` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChromeFilePropertyBag`*"]
    #[wasm_bindgen(method, setter = "existenceCheck")]
    pub fn set_existence_check(this: &ChromeFilePropertyBag, val: bool);
    #[doc = "Get the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChromeFilePropertyBag`*"]
    #[wasm_bindgen(method, getter = "name")]
    pub fn get_name(this: &ChromeFilePropertyBag) -> Option<::alloc::string::String>;
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChromeFilePropertyBag`*"]
    #[wasm_bindgen(method, setter = "name")]
    pub fn set_name(this: &ChromeFilePropertyBag, val: &str);
}
impl ChromeFilePropertyBag {
    #[doc = "Construct a new `ChromeFilePropertyBag`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChromeFilePropertyBag`*"]
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
    #[deprecated = "Use `set_existence_check()` instead."]
    pub fn existence_check(&mut self, val: bool) -> &mut Self {
        self.set_existence_check(val);
        self
    }
    #[deprecated = "Use `set_name()` instead."]
    pub fn name(&mut self, val: &str) -> &mut Self {
        self.set_name(val);
        self
    }
}
impl Default for ChromeFilePropertyBag {
    fn default() -> Self {
        Self::new()
    }
}
