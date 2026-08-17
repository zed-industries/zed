#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "IDBObjectStoreParameters")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `IdbObjectStoreParameters` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbObjectStoreParameters`*"]
    pub type IdbObjectStoreParameters;
    #[doc = "Get the `autoIncrement` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbObjectStoreParameters`*"]
    #[wasm_bindgen(method, getter = "autoIncrement")]
    pub fn get_auto_increment(this: &IdbObjectStoreParameters) -> Option<bool>;
    #[doc = "Change the `autoIncrement` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbObjectStoreParameters`*"]
    #[wasm_bindgen(method, setter = "autoIncrement")]
    pub fn set_auto_increment(this: &IdbObjectStoreParameters, val: bool);
    #[doc = "Get the `keyPath` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbObjectStoreParameters`*"]
    #[wasm_bindgen(method, getter = "keyPath")]
    pub fn get_key_path(this: &IdbObjectStoreParameters) -> ::wasm_bindgen::JsValue;
    #[doc = "Change the `keyPath` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbObjectStoreParameters`*"]
    #[wasm_bindgen(method, setter = "keyPath")]
    pub fn set_key_path(this: &IdbObjectStoreParameters, val: &::wasm_bindgen::JsValue);
    #[doc = "Change the `keyPath` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbObjectStoreParameters`*"]
    #[wasm_bindgen(method, setter = "keyPath")]
    pub fn set_key_path_opt_str(this: &IdbObjectStoreParameters, val: Option<&str>);
    #[doc = "Change the `keyPath` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbObjectStoreParameters`*"]
    #[wasm_bindgen(method, setter = "keyPath")]
    pub fn set_key_path_opt_str_sequence(
        this: &IdbObjectStoreParameters,
        val: Option<&::wasm_bindgen::JsValue>,
    );
}
impl IdbObjectStoreParameters {
    #[doc = "Construct a new `IdbObjectStoreParameters`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbObjectStoreParameters`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_auto_increment()` instead."]
    pub fn auto_increment(&mut self, val: bool) -> &mut Self {
        self.set_auto_increment(val);
        self
    }
    #[deprecated = "Use `set_key_path()` instead."]
    pub fn key_path(&mut self, val: Option<&::wasm_bindgen::JsValue>) -> &mut Self {
        self.set_key_path(val.unwrap_or(&::wasm_bindgen::JsValue::NULL));
        self
    }
}
impl Default for IdbObjectStoreParameters {
    fn default() -> Self {
        Self::new()
    }
}
