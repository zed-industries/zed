#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "UnderlyingSource")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `UnderlyingSource` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UnderlyingSource`*"]
    pub type UnderlyingSource;
    #[doc = "Get the `autoAllocateChunkSize` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UnderlyingSource`*"]
    #[wasm_bindgen(method, getter = "autoAllocateChunkSize")]
    pub fn get_auto_allocate_chunk_size(this: &UnderlyingSource) -> Option<f64>;
    #[doc = "Change the `autoAllocateChunkSize` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UnderlyingSource`*"]
    #[wasm_bindgen(method, setter = "autoAllocateChunkSize")]
    pub fn set_auto_allocate_chunk_size(this: &UnderlyingSource, val: f64);
    #[doc = "Change the `autoAllocateChunkSize` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UnderlyingSource`*"]
    #[wasm_bindgen(method, setter = "autoAllocateChunkSize")]
    pub fn set_auto_allocate_chunk_size_u32(this: &UnderlyingSource, val: u32);
    #[doc = "Change the `autoAllocateChunkSize` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UnderlyingSource`*"]
    #[wasm_bindgen(method, setter = "autoAllocateChunkSize")]
    pub fn set_auto_allocate_chunk_size_f64(this: &UnderlyingSource, val: f64);
    #[doc = "Get the `cancel` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UnderlyingSource`*"]
    #[wasm_bindgen(method, getter = "cancel")]
    pub fn get_cancel(this: &UnderlyingSource) -> Option<::js_sys::Function>;
    #[doc = "Change the `cancel` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UnderlyingSource`*"]
    #[wasm_bindgen(method, setter = "cancel")]
    pub fn set_cancel(this: &UnderlyingSource, val: &::js_sys::Function);
    #[doc = "Get the `pull` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UnderlyingSource`*"]
    #[wasm_bindgen(method, getter = "pull")]
    pub fn get_pull(this: &UnderlyingSource) -> Option<::js_sys::Function>;
    #[doc = "Change the `pull` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UnderlyingSource`*"]
    #[wasm_bindgen(method, setter = "pull")]
    pub fn set_pull(this: &UnderlyingSource, val: &::js_sys::Function);
    #[doc = "Get the `start` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UnderlyingSource`*"]
    #[wasm_bindgen(method, getter = "start")]
    pub fn get_start(this: &UnderlyingSource) -> Option<::js_sys::Function>;
    #[doc = "Change the `start` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UnderlyingSource`*"]
    #[wasm_bindgen(method, setter = "start")]
    pub fn set_start(this: &UnderlyingSource, val: &::js_sys::Function);
    #[cfg(feature = "ReadableStreamType")]
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStreamType`, `UnderlyingSource`*"]
    #[wasm_bindgen(method, getter = "type")]
    pub fn get_type(this: &UnderlyingSource) -> Option<ReadableStreamType>;
    #[cfg(feature = "ReadableStreamType")]
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStreamType`, `UnderlyingSource`*"]
    #[wasm_bindgen(method, setter = "type")]
    pub fn set_type(this: &UnderlyingSource, val: ReadableStreamType);
}
impl UnderlyingSource {
    #[doc = "Construct a new `UnderlyingSource`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UnderlyingSource`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_auto_allocate_chunk_size()` instead."]
    pub fn auto_allocate_chunk_size(&mut self, val: f64) -> &mut Self {
        self.set_auto_allocate_chunk_size(val);
        self
    }
    #[deprecated = "Use `set_cancel()` instead."]
    pub fn cancel(&mut self, val: &::js_sys::Function) -> &mut Self {
        self.set_cancel(val);
        self
    }
    #[deprecated = "Use `set_pull()` instead."]
    pub fn pull(&mut self, val: &::js_sys::Function) -> &mut Self {
        self.set_pull(val);
        self
    }
    #[deprecated = "Use `set_start()` instead."]
    pub fn start(&mut self, val: &::js_sys::Function) -> &mut Self {
        self.set_start(val);
        self
    }
    #[cfg(feature = "ReadableStreamType")]
    #[deprecated = "Use `set_type()` instead."]
    pub fn type_(&mut self, val: ReadableStreamType) -> &mut Self {
        self.set_type(val);
        self
    }
}
impl Default for UnderlyingSource {
    fn default() -> Self {
        Self::new()
    }
}
