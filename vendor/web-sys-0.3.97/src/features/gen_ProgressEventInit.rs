#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "ProgressEventInit")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ProgressEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProgressEventInit`*"]
    pub type ProgressEventInit;
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProgressEventInit`*"]
    #[wasm_bindgen(method, getter = "bubbles")]
    pub fn get_bubbles(this: &ProgressEventInit) -> Option<bool>;
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProgressEventInit`*"]
    #[wasm_bindgen(method, setter = "bubbles")]
    pub fn set_bubbles(this: &ProgressEventInit, val: bool);
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProgressEventInit`*"]
    #[wasm_bindgen(method, getter = "cancelable")]
    pub fn get_cancelable(this: &ProgressEventInit) -> Option<bool>;
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProgressEventInit`*"]
    #[wasm_bindgen(method, setter = "cancelable")]
    pub fn set_cancelable(this: &ProgressEventInit, val: bool);
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProgressEventInit`*"]
    #[wasm_bindgen(method, getter = "composed")]
    pub fn get_composed(this: &ProgressEventInit) -> Option<bool>;
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProgressEventInit`*"]
    #[wasm_bindgen(method, setter = "composed")]
    pub fn set_composed(this: &ProgressEventInit, val: bool);
    #[doc = "Get the `lengthComputable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProgressEventInit`*"]
    #[wasm_bindgen(method, getter = "lengthComputable")]
    pub fn get_length_computable(this: &ProgressEventInit) -> Option<bool>;
    #[doc = "Change the `lengthComputable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProgressEventInit`*"]
    #[wasm_bindgen(method, setter = "lengthComputable")]
    pub fn set_length_computable(this: &ProgressEventInit, val: bool);
    #[doc = "Get the `loaded` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProgressEventInit`*"]
    #[wasm_bindgen(method, getter = "loaded")]
    pub fn get_loaded(this: &ProgressEventInit) -> Option<f64>;
    #[doc = "Change the `loaded` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProgressEventInit`*"]
    #[wasm_bindgen(method, setter = "loaded")]
    pub fn set_loaded(this: &ProgressEventInit, val: f64);
    #[doc = "Change the `loaded` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProgressEventInit`*"]
    #[wasm_bindgen(method, setter = "loaded")]
    pub fn set_loaded_u32(this: &ProgressEventInit, val: u32);
    #[doc = "Change the `loaded` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProgressEventInit`*"]
    #[wasm_bindgen(method, setter = "loaded")]
    pub fn set_loaded_f64(this: &ProgressEventInit, val: f64);
    #[doc = "Get the `total` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProgressEventInit`*"]
    #[wasm_bindgen(method, getter = "total")]
    pub fn get_total(this: &ProgressEventInit) -> Option<f64>;
    #[doc = "Change the `total` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProgressEventInit`*"]
    #[wasm_bindgen(method, setter = "total")]
    pub fn set_total(this: &ProgressEventInit, val: f64);
    #[doc = "Change the `total` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProgressEventInit`*"]
    #[wasm_bindgen(method, setter = "total")]
    pub fn set_total_u32(this: &ProgressEventInit, val: u32);
    #[doc = "Change the `total` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProgressEventInit`*"]
    #[wasm_bindgen(method, setter = "total")]
    pub fn set_total_f64(this: &ProgressEventInit, val: f64);
}
impl ProgressEventInit {
    #[doc = "Construct a new `ProgressEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProgressEventInit`*"]
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
    #[deprecated = "Use `set_length_computable()` instead."]
    pub fn length_computable(&mut self, val: bool) -> &mut Self {
        self.set_length_computable(val);
        self
    }
    #[deprecated = "Use `set_loaded()` instead."]
    pub fn loaded(&mut self, val: f64) -> &mut Self {
        self.set_loaded(val);
        self
    }
    #[deprecated = "Use `set_total()` instead."]
    pub fn total(&mut self, val: f64) -> &mut Self {
        self.set_total(val);
        self
    }
}
impl Default for ProgressEventInit {
    fn default() -> Self {
        Self::new()
    }
}
