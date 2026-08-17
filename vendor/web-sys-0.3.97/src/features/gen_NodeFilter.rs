#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "NodeFilter")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `NodeFilter` dictionary.\n\n*This API requires the following crate features to be activated: `NodeFilter`*"]
    pub type NodeFilter;
    #[doc = "Get the `acceptNode` field of this object.\n\n*This API requires the following crate features to be activated: `NodeFilter`*"]
    #[wasm_bindgen(method, getter = "acceptNode")]
    pub fn get_accept_node(this: &NodeFilter) -> Option<::js_sys::Function>;
    #[doc = "Change the `acceptNode` field of this object.\n\n*This API requires the following crate features to be activated: `NodeFilter`*"]
    #[wasm_bindgen(method, setter = "acceptNode")]
    pub fn set_accept_node(this: &NodeFilter, val: &::js_sys::Function);
}
impl NodeFilter {
    #[doc = "Construct a new `NodeFilter`.\n\n*This API requires the following crate features to be activated: `NodeFilter`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_accept_node()` instead."]
    pub fn accept_node(&mut self, val: &::js_sys::Function) -> &mut Self {
        self.set_accept_node(val);
        self
    }
}
impl Default for NodeFilter {
    fn default() -> Self {
        Self::new()
    }
}
