#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "XPathNSResolver")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `XPathNSResolver` dictionary.\n\n*This API requires the following crate features to be activated: `XPathNsResolver`*"]
    pub type XPathNsResolver;
    #[doc = "Get the `lookupNamespaceURI` field of this object.\n\n*This API requires the following crate features to be activated: `XPathNsResolver`*"]
    #[wasm_bindgen(method, getter = "lookupNamespaceURI")]
    pub fn get_lookup_namespace_uri(this: &XPathNsResolver) -> Option<::js_sys::Function>;
    #[doc = "Change the `lookupNamespaceURI` field of this object.\n\n*This API requires the following crate features to be activated: `XPathNsResolver`*"]
    #[wasm_bindgen(method, setter = "lookupNamespaceURI")]
    pub fn set_lookup_namespace_uri(this: &XPathNsResolver, val: &::js_sys::Function);
}
impl XPathNsResolver {
    #[doc = "Construct a new `XPathNSResolver`.\n\n*This API requires the following crate features to be activated: `XPathNsResolver`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_lookup_namespace_uri()` instead."]
    pub fn lookup_namespace_uri(&mut self, val: &::js_sys::Function) -> &mut Self {
        self.set_lookup_namespace_uri(val);
        self
    }
}
impl Default for XPathNsResolver {
    fn default() -> Self {
        Self::new()
    }
}
