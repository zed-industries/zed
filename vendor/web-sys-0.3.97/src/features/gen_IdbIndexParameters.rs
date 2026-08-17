#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "IDBIndexParameters")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `IdbIndexParameters` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbIndexParameters`*"]
    pub type IdbIndexParameters;
    #[doc = "Get the `locale` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbIndexParameters`*"]
    #[deprecated]
    #[wasm_bindgen(method, getter = "locale")]
    pub fn get_locale(this: &IdbIndexParameters) -> Option<::alloc::string::String>;
    #[doc = "Change the `locale` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbIndexParameters`*"]
    #[deprecated]
    #[wasm_bindgen(method, setter = "locale")]
    pub fn set_locale(this: &IdbIndexParameters, val: Option<&str>);
    #[doc = "Get the `multiEntry` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbIndexParameters`*"]
    #[wasm_bindgen(method, getter = "multiEntry")]
    pub fn get_multi_entry(this: &IdbIndexParameters) -> Option<bool>;
    #[doc = "Change the `multiEntry` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbIndexParameters`*"]
    #[wasm_bindgen(method, setter = "multiEntry")]
    pub fn set_multi_entry(this: &IdbIndexParameters, val: bool);
    #[doc = "Get the `unique` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbIndexParameters`*"]
    #[wasm_bindgen(method, getter = "unique")]
    pub fn get_unique(this: &IdbIndexParameters) -> Option<bool>;
    #[doc = "Change the `unique` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbIndexParameters`*"]
    #[wasm_bindgen(method, setter = "unique")]
    pub fn set_unique(this: &IdbIndexParameters, val: bool);
}
impl IdbIndexParameters {
    #[doc = "Construct a new `IdbIndexParameters`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbIndexParameters`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_locale()` instead."]
    pub fn locale(&mut self, val: Option<&str>) -> &mut Self {
        self.set_locale(val);
        self
    }
    #[deprecated = "Use `set_multi_entry()` instead."]
    pub fn multi_entry(&mut self, val: bool) -> &mut Self {
        self.set_multi_entry(val);
        self
    }
    #[deprecated = "Use `set_unique()` instead."]
    pub fn unique(&mut self, val: bool) -> &mut Self {
        self.set_unique(val);
        self
    }
}
impl Default for IdbIndexParameters {
    fn default() -> Self {
        Self::new()
    }
}
