#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "HttpConnDict")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HttpConnDict` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HttpConnDict`*"]
    pub type HttpConnDict;
    #[doc = "Get the `connections` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HttpConnDict`*"]
    #[wasm_bindgen(method, getter = "connections")]
    pub fn get_connections(this: &HttpConnDict) -> Option<::js_sys::Array>;
    #[doc = "Change the `connections` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HttpConnDict`*"]
    #[wasm_bindgen(method, setter = "connections")]
    pub fn set_connections(this: &HttpConnDict, val: &::wasm_bindgen::JsValue);
}
impl HttpConnDict {
    #[doc = "Construct a new `HttpConnDict`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HttpConnDict`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_connections()` instead."]
    pub fn connections(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_connections(val);
        self
    }
}
impl Default for HttpConnDict {
    fn default() -> Self {
        Self::new()
    }
}
