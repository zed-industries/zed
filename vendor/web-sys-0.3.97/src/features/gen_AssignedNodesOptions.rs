#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "AssignedNodesOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AssignedNodesOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AssignedNodesOptions`*"]
    pub type AssignedNodesOptions;
    #[doc = "Get the `flatten` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AssignedNodesOptions`*"]
    #[wasm_bindgen(method, getter = "flatten")]
    pub fn get_flatten(this: &AssignedNodesOptions) -> Option<bool>;
    #[doc = "Change the `flatten` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AssignedNodesOptions`*"]
    #[wasm_bindgen(method, setter = "flatten")]
    pub fn set_flatten(this: &AssignedNodesOptions, val: bool);
}
impl AssignedNodesOptions {
    #[doc = "Construct a new `AssignedNodesOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AssignedNodesOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_flatten()` instead."]
    pub fn flatten(&mut self, val: bool) -> &mut Self {
        self.set_flatten(val);
        self
    }
}
impl Default for AssignedNodesOptions {
    fn default() -> Self {
        Self::new()
    }
}
