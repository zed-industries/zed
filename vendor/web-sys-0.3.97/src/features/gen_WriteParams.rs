#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "WriteParams")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WriteParams` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WriteParams`*"]
    pub type WriteParams;
    #[doc = "Get the `data` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WriteParams`*"]
    #[wasm_bindgen(method, getter = "data")]
    pub fn get_data(this: &WriteParams) -> ::wasm_bindgen::JsValue;
    #[doc = "Change the `data` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WriteParams`*"]
    #[wasm_bindgen(method, setter = "data")]
    pub fn set_data(this: &WriteParams, val: &::wasm_bindgen::JsValue);
    #[doc = "Change the `data` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WriteParams`*"]
    #[wasm_bindgen(method, setter = "data")]
    pub fn set_data_opt_buffer_source(this: &WriteParams, val: Option<&::js_sys::Object>);
    #[doc = "Change the `data` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WriteParams`*"]
    #[wasm_bindgen(method, setter = "data")]
    pub fn set_data_opt_u8_slice(this: &WriteParams, val: Option<&mut [u8]>);
    #[doc = "Change the `data` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WriteParams`*"]
    #[wasm_bindgen(method, setter = "data")]
    pub fn set_data_opt_u8_array(this: &WriteParams, val: Option<&::js_sys::Uint8Array>);
    #[cfg(feature = "Blob")]
    #[doc = "Change the `data` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Blob`, `WriteParams`*"]
    #[wasm_bindgen(method, setter = "data")]
    pub fn set_data_opt_blob(this: &WriteParams, val: Option<&Blob>);
    #[doc = "Change the `data` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WriteParams`*"]
    #[wasm_bindgen(method, setter = "data")]
    pub fn set_data_opt_str(this: &WriteParams, val: Option<&str>);
    #[doc = "Get the `position` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WriteParams`*"]
    #[wasm_bindgen(method, getter = "position")]
    pub fn get_position(this: &WriteParams) -> Option<f64>;
    #[doc = "Change the `position` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WriteParams`*"]
    #[wasm_bindgen(method, setter = "position")]
    pub fn set_position(this: &WriteParams, val: Option<f64>);
    #[doc = "Change the `position` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WriteParams`*"]
    #[wasm_bindgen(method, setter = "position")]
    pub fn set_position_opt_u32(this: &WriteParams, val: Option<u32>);
    #[doc = "Change the `position` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WriteParams`*"]
    #[wasm_bindgen(method, setter = "position")]
    pub fn set_position_opt_f64(this: &WriteParams, val: Option<f64>);
    #[doc = "Get the `size` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WriteParams`*"]
    #[wasm_bindgen(method, getter = "size")]
    pub fn get_size(this: &WriteParams) -> Option<f64>;
    #[doc = "Change the `size` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WriteParams`*"]
    #[wasm_bindgen(method, setter = "size")]
    pub fn set_size(this: &WriteParams, val: Option<f64>);
    #[doc = "Change the `size` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WriteParams`*"]
    #[wasm_bindgen(method, setter = "size")]
    pub fn set_size_opt_u32(this: &WriteParams, val: Option<u32>);
    #[doc = "Change the `size` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WriteParams`*"]
    #[wasm_bindgen(method, setter = "size")]
    pub fn set_size_opt_f64(this: &WriteParams, val: Option<f64>);
    #[cfg(feature = "WriteCommandType")]
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WriteCommandType`, `WriteParams`*"]
    #[wasm_bindgen(method, getter = "type")]
    pub fn get_type(this: &WriteParams) -> WriteCommandType;
    #[cfg(feature = "WriteCommandType")]
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WriteCommandType`, `WriteParams`*"]
    #[wasm_bindgen(method, setter = "type")]
    pub fn set_type(this: &WriteParams, val: WriteCommandType);
}
impl WriteParams {
    #[cfg(feature = "WriteCommandType")]
    #[doc = "Construct a new `WriteParams`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WriteCommandType`, `WriteParams`*"]
    pub fn new(type_: WriteCommandType) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_type(type_);
        ret
    }
    #[deprecated = "Use `set_data()` instead."]
    pub fn data(&mut self, val: Option<&::wasm_bindgen::JsValue>) -> &mut Self {
        self.set_data(val.unwrap_or(&::wasm_bindgen::JsValue::NULL));
        self
    }
    #[deprecated = "Use `set_position()` instead."]
    pub fn position(&mut self, val: Option<f64>) -> &mut Self {
        self.set_position(val);
        self
    }
    #[deprecated = "Use `set_size()` instead."]
    pub fn size(&mut self, val: Option<f64>) -> &mut Self {
        self.set_size(val);
        self
    }
    #[cfg(feature = "WriteCommandType")]
    #[deprecated = "Use `set_type()` instead."]
    pub fn type_(&mut self, val: WriteCommandType) -> &mut Self {
        self.set_type(val);
        self
    }
}
