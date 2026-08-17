#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "RsaOtherPrimesInfo")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RsaOtherPrimesInfo` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RsaOtherPrimesInfo`*"]
    pub type RsaOtherPrimesInfo;
    #[doc = "Get the `d` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RsaOtherPrimesInfo`*"]
    #[wasm_bindgen(method, getter = "d")]
    pub fn get_d(this: &RsaOtherPrimesInfo) -> ::alloc::string::String;
    #[doc = "Change the `d` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RsaOtherPrimesInfo`*"]
    #[wasm_bindgen(method, setter = "d")]
    pub fn set_d(this: &RsaOtherPrimesInfo, val: &str);
    #[doc = "Get the `r` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RsaOtherPrimesInfo`*"]
    #[wasm_bindgen(method, getter = "r")]
    pub fn get_r(this: &RsaOtherPrimesInfo) -> ::alloc::string::String;
    #[doc = "Change the `r` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RsaOtherPrimesInfo`*"]
    #[wasm_bindgen(method, setter = "r")]
    pub fn set_r(this: &RsaOtherPrimesInfo, val: &str);
    #[doc = "Get the `t` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RsaOtherPrimesInfo`*"]
    #[wasm_bindgen(method, getter = "t")]
    pub fn get_t(this: &RsaOtherPrimesInfo) -> ::alloc::string::String;
    #[doc = "Change the `t` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RsaOtherPrimesInfo`*"]
    #[wasm_bindgen(method, setter = "t")]
    pub fn set_t(this: &RsaOtherPrimesInfo, val: &str);
}
impl RsaOtherPrimesInfo {
    #[doc = "Construct a new `RsaOtherPrimesInfo`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RsaOtherPrimesInfo`*"]
    pub fn new(d: &str, r: &str, t: &str) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_d(d);
        ret.set_r(r);
        ret.set_t(t);
        ret
    }
    #[deprecated = "Use `set_d()` instead."]
    pub fn d(&mut self, val: &str) -> &mut Self {
        self.set_d(val);
        self
    }
    #[deprecated = "Use `set_r()` instead."]
    pub fn r(&mut self, val: &str) -> &mut Self {
        self.set_r(val);
        self
    }
    #[deprecated = "Use `set_t()` instead."]
    pub fn t(&mut self, val: &str) -> &mut Self {
        self.set_t(val);
        self
    }
}
