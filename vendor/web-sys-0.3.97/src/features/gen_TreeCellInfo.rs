#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "TreeCellInfo")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `TreeCellInfo` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TreeCellInfo`*"]
    pub type TreeCellInfo;
    #[doc = "Get the `childElt` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TreeCellInfo`*"]
    #[wasm_bindgen(method, getter = "childElt")]
    pub fn get_child_elt(this: &TreeCellInfo) -> Option<::alloc::string::String>;
    #[doc = "Change the `childElt` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TreeCellInfo`*"]
    #[wasm_bindgen(method, setter = "childElt")]
    pub fn set_child_elt(this: &TreeCellInfo, val: &str);
    #[doc = "Get the `row` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TreeCellInfo`*"]
    #[wasm_bindgen(method, getter = "row")]
    pub fn get_row(this: &TreeCellInfo) -> Option<i32>;
    #[doc = "Change the `row` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TreeCellInfo`*"]
    #[wasm_bindgen(method, setter = "row")]
    pub fn set_row(this: &TreeCellInfo, val: i32);
}
impl TreeCellInfo {
    #[doc = "Construct a new `TreeCellInfo`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TreeCellInfo`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_child_elt()` instead."]
    pub fn child_elt(&mut self, val: &str) -> &mut Self {
        self.set_child_elt(val);
        self
    }
    #[deprecated = "Use `set_row()` instead."]
    pub fn row(&mut self, val: i32) -> &mut Self {
        self.set_row(val);
        self
    }
}
impl Default for TreeCellInfo {
    fn default() -> Self {
        Self::new()
    }
}
