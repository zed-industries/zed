#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "IDBVersionChangeEventInit")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `IdbVersionChangeEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbVersionChangeEventInit`*"]
    pub type IdbVersionChangeEventInit;
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbVersionChangeEventInit`*"]
    #[wasm_bindgen(method, getter = "bubbles")]
    pub fn get_bubbles(this: &IdbVersionChangeEventInit) -> Option<bool>;
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbVersionChangeEventInit`*"]
    #[wasm_bindgen(method, setter = "bubbles")]
    pub fn set_bubbles(this: &IdbVersionChangeEventInit, val: bool);
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbVersionChangeEventInit`*"]
    #[wasm_bindgen(method, getter = "cancelable")]
    pub fn get_cancelable(this: &IdbVersionChangeEventInit) -> Option<bool>;
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbVersionChangeEventInit`*"]
    #[wasm_bindgen(method, setter = "cancelable")]
    pub fn set_cancelable(this: &IdbVersionChangeEventInit, val: bool);
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbVersionChangeEventInit`*"]
    #[wasm_bindgen(method, getter = "composed")]
    pub fn get_composed(this: &IdbVersionChangeEventInit) -> Option<bool>;
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbVersionChangeEventInit`*"]
    #[wasm_bindgen(method, setter = "composed")]
    pub fn set_composed(this: &IdbVersionChangeEventInit, val: bool);
    #[doc = "Get the `newVersion` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbVersionChangeEventInit`*"]
    #[wasm_bindgen(method, getter = "newVersion")]
    pub fn get_new_version(this: &IdbVersionChangeEventInit) -> Option<f64>;
    #[doc = "Change the `newVersion` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbVersionChangeEventInit`*"]
    #[wasm_bindgen(method, setter = "newVersion")]
    pub fn set_new_version(this: &IdbVersionChangeEventInit, val: Option<f64>);
    #[doc = "Change the `newVersion` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbVersionChangeEventInit`*"]
    #[wasm_bindgen(method, setter = "newVersion")]
    pub fn set_new_version_opt_u32(this: &IdbVersionChangeEventInit, val: Option<u32>);
    #[doc = "Change the `newVersion` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbVersionChangeEventInit`*"]
    #[wasm_bindgen(method, setter = "newVersion")]
    pub fn set_new_version_opt_f64(this: &IdbVersionChangeEventInit, val: Option<f64>);
    #[doc = "Get the `oldVersion` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbVersionChangeEventInit`*"]
    #[wasm_bindgen(method, getter = "oldVersion")]
    pub fn get_old_version(this: &IdbVersionChangeEventInit) -> Option<f64>;
    #[doc = "Change the `oldVersion` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbVersionChangeEventInit`*"]
    #[wasm_bindgen(method, setter = "oldVersion")]
    pub fn set_old_version(this: &IdbVersionChangeEventInit, val: f64);
    #[doc = "Change the `oldVersion` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbVersionChangeEventInit`*"]
    #[wasm_bindgen(method, setter = "oldVersion")]
    pub fn set_old_version_u32(this: &IdbVersionChangeEventInit, val: u32);
    #[doc = "Change the `oldVersion` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbVersionChangeEventInit`*"]
    #[wasm_bindgen(method, setter = "oldVersion")]
    pub fn set_old_version_f64(this: &IdbVersionChangeEventInit, val: f64);
}
impl IdbVersionChangeEventInit {
    #[doc = "Construct a new `IdbVersionChangeEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbVersionChangeEventInit`*"]
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
    #[deprecated = "Use `set_new_version()` instead."]
    pub fn new_version(&mut self, val: Option<f64>) -> &mut Self {
        self.set_new_version(val);
        self
    }
    #[deprecated = "Use `set_old_version()` instead."]
    pub fn old_version(&mut self, val: f64) -> &mut Self {
        self.set_old_version(val);
        self
    }
}
impl Default for IdbVersionChangeEventInit {
    fn default() -> Self {
        Self::new()
    }
}
