#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "KeyAlgorithm")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `KeyAlgorithm` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyAlgorithm`*"]
    pub type KeyAlgorithm;
    #[doc = "Get the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyAlgorithm`*"]
    #[wasm_bindgen(method, getter = "name")]
    pub fn get_name(this: &KeyAlgorithm) -> ::alloc::string::String;
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyAlgorithm`*"]
    #[wasm_bindgen(method, setter = "name")]
    pub fn set_name(this: &KeyAlgorithm, val: &str);
}
impl KeyAlgorithm {
    #[doc = "Construct a new `KeyAlgorithm`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyAlgorithm`*"]
    pub fn new(name: &str) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_name(name);
        ret
    }
    #[deprecated = "Use `set_name()` instead."]
    pub fn name(&mut self, val: &str) -> &mut Self {
        self.set_name(val);
        self
    }
}
