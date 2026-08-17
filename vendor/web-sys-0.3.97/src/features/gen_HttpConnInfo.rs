#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "HttpConnInfo")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HttpConnInfo` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HttpConnInfo`*"]
    pub type HttpConnInfo;
    #[doc = "Get the `protocolVersion` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HttpConnInfo`*"]
    #[wasm_bindgen(method, getter = "protocolVersion")]
    pub fn get_protocol_version(this: &HttpConnInfo) -> Option<::alloc::string::String>;
    #[doc = "Change the `protocolVersion` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HttpConnInfo`*"]
    #[wasm_bindgen(method, setter = "protocolVersion")]
    pub fn set_protocol_version(this: &HttpConnInfo, val: &str);
    #[doc = "Get the `rtt` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HttpConnInfo`*"]
    #[wasm_bindgen(method, getter = "rtt")]
    pub fn get_rtt(this: &HttpConnInfo) -> Option<u32>;
    #[doc = "Change the `rtt` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HttpConnInfo`*"]
    #[wasm_bindgen(method, setter = "rtt")]
    pub fn set_rtt(this: &HttpConnInfo, val: u32);
    #[doc = "Get the `ttl` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HttpConnInfo`*"]
    #[wasm_bindgen(method, getter = "ttl")]
    pub fn get_ttl(this: &HttpConnInfo) -> Option<u32>;
    #[doc = "Change the `ttl` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HttpConnInfo`*"]
    #[wasm_bindgen(method, setter = "ttl")]
    pub fn set_ttl(this: &HttpConnInfo, val: u32);
}
impl HttpConnInfo {
    #[doc = "Construct a new `HttpConnInfo`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HttpConnInfo`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_protocol_version()` instead."]
    pub fn protocol_version(&mut self, val: &str) -> &mut Self {
        self.set_protocol_version(val);
        self
    }
    #[deprecated = "Use `set_rtt()` instead."]
    pub fn rtt(&mut self, val: u32) -> &mut Self {
        self.set_rtt(val);
        self
    }
    #[deprecated = "Use `set_ttl()` instead."]
    pub fn ttl(&mut self, val: u32) -> &mut Self {
        self.set_ttl(val);
        self
    }
}
impl Default for HttpConnInfo {
    fn default() -> Self {
        Self::new()
    }
}
