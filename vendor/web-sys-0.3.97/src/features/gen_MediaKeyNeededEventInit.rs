#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "MediaKeyNeededEventInit")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaKeyNeededEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeyNeededEventInit`*"]
    pub type MediaKeyNeededEventInit;
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeyNeededEventInit`*"]
    #[wasm_bindgen(method, getter = "bubbles")]
    pub fn get_bubbles(this: &MediaKeyNeededEventInit) -> Option<bool>;
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeyNeededEventInit`*"]
    #[wasm_bindgen(method, setter = "bubbles")]
    pub fn set_bubbles(this: &MediaKeyNeededEventInit, val: bool);
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeyNeededEventInit`*"]
    #[wasm_bindgen(method, getter = "cancelable")]
    pub fn get_cancelable(this: &MediaKeyNeededEventInit) -> Option<bool>;
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeyNeededEventInit`*"]
    #[wasm_bindgen(method, setter = "cancelable")]
    pub fn set_cancelable(this: &MediaKeyNeededEventInit, val: bool);
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeyNeededEventInit`*"]
    #[wasm_bindgen(method, getter = "composed")]
    pub fn get_composed(this: &MediaKeyNeededEventInit) -> Option<bool>;
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeyNeededEventInit`*"]
    #[wasm_bindgen(method, setter = "composed")]
    pub fn set_composed(this: &MediaKeyNeededEventInit, val: bool);
    #[doc = "Get the `initData` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeyNeededEventInit`*"]
    #[wasm_bindgen(method, getter = "initData")]
    pub fn get_init_data(this: &MediaKeyNeededEventInit) -> Option<::js_sys::ArrayBuffer>;
    #[doc = "Change the `initData` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeyNeededEventInit`*"]
    #[wasm_bindgen(method, setter = "initData")]
    pub fn set_init_data(this: &MediaKeyNeededEventInit, val: Option<&::js_sys::ArrayBuffer>);
    #[doc = "Get the `initDataType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeyNeededEventInit`*"]
    #[wasm_bindgen(method, getter = "initDataType")]
    pub fn get_init_data_type(this: &MediaKeyNeededEventInit) -> Option<::alloc::string::String>;
    #[doc = "Change the `initDataType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeyNeededEventInit`*"]
    #[wasm_bindgen(method, setter = "initDataType")]
    pub fn set_init_data_type(this: &MediaKeyNeededEventInit, val: &str);
}
impl MediaKeyNeededEventInit {
    #[doc = "Construct a new `MediaKeyNeededEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeyNeededEventInit`*"]
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
    #[deprecated = "Use `set_init_data()` instead."]
    pub fn init_data(&mut self, val: Option<&::js_sys::ArrayBuffer>) -> &mut Self {
        self.set_init_data(val);
        self
    }
    #[deprecated = "Use `set_init_data_type()` instead."]
    pub fn init_data_type(&mut self, val: &str) -> &mut Self {
        self.set_init_data_type(val);
        self
    }
}
impl Default for MediaKeyNeededEventInit {
    fn default() -> Self {
        Self::new()
    }
}
