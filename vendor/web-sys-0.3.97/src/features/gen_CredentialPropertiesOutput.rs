#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "CredentialPropertiesOutput")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CredentialPropertiesOutput` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CredentialPropertiesOutput`*"]
    pub type CredentialPropertiesOutput;
    #[doc = "Get the `rk` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CredentialPropertiesOutput`*"]
    #[wasm_bindgen(method, getter = "rk")]
    pub fn get_rk(this: &CredentialPropertiesOutput) -> Option<bool>;
    #[doc = "Change the `rk` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CredentialPropertiesOutput`*"]
    #[wasm_bindgen(method, setter = "rk")]
    pub fn set_rk(this: &CredentialPropertiesOutput, val: bool);
}
impl CredentialPropertiesOutput {
    #[doc = "Construct a new `CredentialPropertiesOutput`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CredentialPropertiesOutput`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_rk()` instead."]
    pub fn rk(&mut self, val: bool) -> &mut Self {
        self.set_rk(val);
        self
    }
}
impl Default for CredentialPropertiesOutput {
    fn default() -> Self {
        Self::new()
    }
}
