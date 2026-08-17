#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "ConnStatusDict")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ConnStatusDict` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConnStatusDict`*"]
    pub type ConnStatusDict;
    #[doc = "Get the `status` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConnStatusDict`*"]
    #[wasm_bindgen(method, getter = "status")]
    pub fn get_status(this: &ConnStatusDict) -> Option<::alloc::string::String>;
    #[doc = "Change the `status` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConnStatusDict`*"]
    #[wasm_bindgen(method, setter = "status")]
    pub fn set_status(this: &ConnStatusDict, val: &str);
}
impl ConnStatusDict {
    #[doc = "Construct a new `ConnStatusDict`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConnStatusDict`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_status()` instead."]
    pub fn status(&mut self, val: &str) -> &mut Self {
        self.set_status(val);
        self
    }
}
impl Default for ConnStatusDict {
    fn default() -> Self {
        Self::new()
    }
}
