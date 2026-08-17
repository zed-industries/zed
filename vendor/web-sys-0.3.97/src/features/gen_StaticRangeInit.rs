#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "StaticRangeInit")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `StaticRangeInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StaticRangeInit`*"]
    pub type StaticRangeInit;
    #[cfg(feature = "Node")]
    #[doc = "Get the `endContainer` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`, `StaticRangeInit`*"]
    #[wasm_bindgen(method, getter = "endContainer")]
    pub fn get_end_container(this: &StaticRangeInit) -> Node;
    #[cfg(feature = "Node")]
    #[doc = "Change the `endContainer` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`, `StaticRangeInit`*"]
    #[wasm_bindgen(method, setter = "endContainer")]
    pub fn set_end_container(this: &StaticRangeInit, val: &Node);
    #[doc = "Get the `endOffset` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StaticRangeInit`*"]
    #[wasm_bindgen(method, getter = "endOffset")]
    pub fn get_end_offset(this: &StaticRangeInit) -> u32;
    #[doc = "Change the `endOffset` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StaticRangeInit`*"]
    #[wasm_bindgen(method, setter = "endOffset")]
    pub fn set_end_offset(this: &StaticRangeInit, val: u32);
    #[cfg(feature = "Node")]
    #[doc = "Get the `startContainer` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`, `StaticRangeInit`*"]
    #[wasm_bindgen(method, getter = "startContainer")]
    pub fn get_start_container(this: &StaticRangeInit) -> Node;
    #[cfg(feature = "Node")]
    #[doc = "Change the `startContainer` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`, `StaticRangeInit`*"]
    #[wasm_bindgen(method, setter = "startContainer")]
    pub fn set_start_container(this: &StaticRangeInit, val: &Node);
    #[doc = "Get the `startOffset` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StaticRangeInit`*"]
    #[wasm_bindgen(method, getter = "startOffset")]
    pub fn get_start_offset(this: &StaticRangeInit) -> u32;
    #[doc = "Change the `startOffset` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StaticRangeInit`*"]
    #[wasm_bindgen(method, setter = "startOffset")]
    pub fn set_start_offset(this: &StaticRangeInit, val: u32);
}
impl StaticRangeInit {
    #[cfg(feature = "Node")]
    #[doc = "Construct a new `StaticRangeInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`, `StaticRangeInit`*"]
    pub fn new(
        end_container: &Node,
        end_offset: u32,
        start_container: &Node,
        start_offset: u32,
    ) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_end_container(end_container);
        ret.set_end_offset(end_offset);
        ret.set_start_container(start_container);
        ret.set_start_offset(start_offset);
        ret
    }
    #[cfg(feature = "Node")]
    #[deprecated = "Use `set_end_container()` instead."]
    pub fn end_container(&mut self, val: &Node) -> &mut Self {
        self.set_end_container(val);
        self
    }
    #[deprecated = "Use `set_end_offset()` instead."]
    pub fn end_offset(&mut self, val: u32) -> &mut Self {
        self.set_end_offset(val);
        self
    }
    #[cfg(feature = "Node")]
    #[deprecated = "Use `set_start_container()` instead."]
    pub fn start_container(&mut self, val: &Node) -> &mut Self {
        self.set_start_container(val);
        self
    }
    #[deprecated = "Use `set_start_offset()` instead."]
    pub fn start_offset(&mut self, val: u32) -> &mut Self {
        self.set_start_offset(val);
        self
    }
}
