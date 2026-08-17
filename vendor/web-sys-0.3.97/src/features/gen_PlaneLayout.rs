#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "PlaneLayout")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PlaneLayout` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PlaneLayout`*"]
    pub type PlaneLayout;
    #[doc = "Get the `offset` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PlaneLayout`*"]
    #[wasm_bindgen(method, getter = "offset")]
    pub fn get_offset(this: &PlaneLayout) -> u32;
    #[doc = "Change the `offset` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PlaneLayout`*"]
    #[wasm_bindgen(method, setter = "offset")]
    pub fn set_offset(this: &PlaneLayout, val: u32);
    #[doc = "Get the `stride` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PlaneLayout`*"]
    #[wasm_bindgen(method, getter = "stride")]
    pub fn get_stride(this: &PlaneLayout) -> u32;
    #[doc = "Change the `stride` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PlaneLayout`*"]
    #[wasm_bindgen(method, setter = "stride")]
    pub fn set_stride(this: &PlaneLayout, val: u32);
}
impl PlaneLayout {
    #[doc = "Construct a new `PlaneLayout`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PlaneLayout`*"]
    pub fn new(offset: u32, stride: u32) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_offset(offset);
        ret.set_stride(stride);
        ret
    }
    #[deprecated = "Use `set_offset()` instead."]
    pub fn offset(&mut self, val: u32) -> &mut Self {
        self.set_offset(val);
        self
    }
    #[deprecated = "Use `set_stride()` instead."]
    pub fn stride(&mut self, val: u32) -> &mut Self {
        self.set_stride(val);
        self
    }
}
