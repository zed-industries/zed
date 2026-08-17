#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "IDBOpenDBOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `IdbOpenDbOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbOpenDbOptions`*"]
    #[deprecated]
    pub type IdbOpenDbOptions;
    #[cfg(feature = "StorageType")]
    #[doc = "Get the `storage` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbOpenDbOptions`, `StorageType`*"]
    #[deprecated]
    #[wasm_bindgen(method, getter = "storage")]
    pub fn get_storage(this: &IdbOpenDbOptions) -> Option<StorageType>;
    #[cfg(feature = "StorageType")]
    #[doc = "Change the `storage` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbOpenDbOptions`, `StorageType`*"]
    #[deprecated]
    #[wasm_bindgen(method, setter = "storage")]
    pub fn set_storage(this: &IdbOpenDbOptions, val: StorageType);
    #[doc = "Get the `version` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbOpenDbOptions`*"]
    #[deprecated]
    #[wasm_bindgen(method, getter = "version")]
    pub fn get_version(this: &IdbOpenDbOptions) -> Option<f64>;
    #[doc = "Change the `version` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbOpenDbOptions`*"]
    #[deprecated]
    #[wasm_bindgen(method, setter = "version")]
    pub fn set_version(this: &IdbOpenDbOptions, val: f64);
    #[doc = "Change the `version` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbOpenDbOptions`*"]
    #[deprecated]
    #[wasm_bindgen(method, setter = "version")]
    pub fn set_version_u32(this: &IdbOpenDbOptions, val: u32);
    #[doc = "Change the `version` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbOpenDbOptions`*"]
    #[deprecated]
    #[wasm_bindgen(method, setter = "version")]
    pub fn set_version_f64(this: &IdbOpenDbOptions, val: f64);
}
impl IdbOpenDbOptions {
    #[doc = "Construct a new `IdbOpenDbOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbOpenDbOptions`*"]
    #[deprecated]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(feature = "StorageType")]
    #[deprecated = "Use `set_storage()` instead."]
    pub fn storage(&mut self, val: StorageType) -> &mut Self {
        self.set_storage(val);
        self
    }
    #[deprecated = "Use `set_version()` instead."]
    pub fn version(&mut self, val: f64) -> &mut Self {
        self.set_version(val);
        self
    }
}
impl Default for IdbOpenDbOptions {
    fn default() -> Self {
        Self::new()
    }
}
