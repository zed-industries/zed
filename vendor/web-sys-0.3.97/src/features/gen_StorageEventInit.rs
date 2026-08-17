#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "StorageEventInit")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `StorageEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StorageEventInit`*"]
    pub type StorageEventInit;
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StorageEventInit`*"]
    #[wasm_bindgen(method, getter = "bubbles")]
    pub fn get_bubbles(this: &StorageEventInit) -> Option<bool>;
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StorageEventInit`*"]
    #[wasm_bindgen(method, setter = "bubbles")]
    pub fn set_bubbles(this: &StorageEventInit, val: bool);
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StorageEventInit`*"]
    #[wasm_bindgen(method, getter = "cancelable")]
    pub fn get_cancelable(this: &StorageEventInit) -> Option<bool>;
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StorageEventInit`*"]
    #[wasm_bindgen(method, setter = "cancelable")]
    pub fn set_cancelable(this: &StorageEventInit, val: bool);
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StorageEventInit`*"]
    #[wasm_bindgen(method, getter = "composed")]
    pub fn get_composed(this: &StorageEventInit) -> Option<bool>;
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StorageEventInit`*"]
    #[wasm_bindgen(method, setter = "composed")]
    pub fn set_composed(this: &StorageEventInit, val: bool);
    #[doc = "Get the `key` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StorageEventInit`*"]
    #[wasm_bindgen(method, getter = "key")]
    pub fn get_key(this: &StorageEventInit) -> Option<::alloc::string::String>;
    #[doc = "Change the `key` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StorageEventInit`*"]
    #[wasm_bindgen(method, setter = "key")]
    pub fn set_key(this: &StorageEventInit, val: Option<&str>);
    #[doc = "Get the `newValue` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StorageEventInit`*"]
    #[wasm_bindgen(method, getter = "newValue")]
    pub fn get_new_value(this: &StorageEventInit) -> Option<::alloc::string::String>;
    #[doc = "Change the `newValue` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StorageEventInit`*"]
    #[wasm_bindgen(method, setter = "newValue")]
    pub fn set_new_value(this: &StorageEventInit, val: Option<&str>);
    #[doc = "Get the `oldValue` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StorageEventInit`*"]
    #[wasm_bindgen(method, getter = "oldValue")]
    pub fn get_old_value(this: &StorageEventInit) -> Option<::alloc::string::String>;
    #[doc = "Change the `oldValue` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StorageEventInit`*"]
    #[wasm_bindgen(method, setter = "oldValue")]
    pub fn set_old_value(this: &StorageEventInit, val: Option<&str>);
    #[cfg(feature = "Storage")]
    #[doc = "Get the `storageArea` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Storage`, `StorageEventInit`*"]
    #[wasm_bindgen(method, getter = "storageArea")]
    pub fn get_storage_area(this: &StorageEventInit) -> Option<Storage>;
    #[cfg(feature = "Storage")]
    #[doc = "Change the `storageArea` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Storage`, `StorageEventInit`*"]
    #[wasm_bindgen(method, setter = "storageArea")]
    pub fn set_storage_area(this: &StorageEventInit, val: Option<&Storage>);
    #[doc = "Get the `url` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StorageEventInit`*"]
    #[wasm_bindgen(method, getter = "url")]
    pub fn get_url(this: &StorageEventInit) -> Option<::alloc::string::String>;
    #[doc = "Change the `url` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StorageEventInit`*"]
    #[wasm_bindgen(method, setter = "url")]
    pub fn set_url(this: &StorageEventInit, val: &str);
}
impl StorageEventInit {
    #[doc = "Construct a new `StorageEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StorageEventInit`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_bubbles()` instead."]
    pub fn bubbles(&mut self, val: bool) -> &mut Self {
        self.set_bubbles(val);
        self
    }
    #[deprecated = "Use `set_cancelable()` instead."]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.set_cancelable(val);
        self
    }
    #[deprecated = "Use `set_composed()` instead."]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.set_composed(val);
        self
    }
    #[deprecated = "Use `set_key()` instead."]
    pub fn key(&mut self, val: Option<&str>) -> &mut Self {
        self.set_key(val);
        self
    }
    #[deprecated = "Use `set_new_value()` instead."]
    pub fn new_value(&mut self, val: Option<&str>) -> &mut Self {
        self.set_new_value(val);
        self
    }
    #[deprecated = "Use `set_old_value()` instead."]
    pub fn old_value(&mut self, val: Option<&str>) -> &mut Self {
        self.set_old_value(val);
        self
    }
    #[cfg(feature = "Storage")]
    #[deprecated = "Use `set_storage_area()` instead."]
    pub fn storage_area(&mut self, val: Option<&Storage>) -> &mut Self {
        self.set_storage_area(val);
        self
    }
    #[deprecated = "Use `set_url()` instead."]
    pub fn url(&mut self, val: &str) -> &mut Self {
        self.set_url(val);
        self
    }
}
impl Default for StorageEventInit {
    fn default() -> Self {
        Self::new()
    }
}
