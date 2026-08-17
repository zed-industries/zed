#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "DOMQuadJSON")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DomQuadJson` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomQuadJson`*"]
    pub type DomQuadJson;
    #[cfg(feature = "DomPoint")]
    #[doc = "Get the `p1` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPoint`, `DomQuadJson`*"]
    #[wasm_bindgen(method, getter = "p1")]
    pub fn get_p1(this: &DomQuadJson) -> Option<DomPoint>;
    #[cfg(feature = "DomPoint")]
    #[doc = "Change the `p1` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPoint`, `DomQuadJson`*"]
    #[wasm_bindgen(method, setter = "p1")]
    pub fn set_p1(this: &DomQuadJson, val: &DomPoint);
    #[cfg(feature = "DomPoint")]
    #[doc = "Get the `p2` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPoint`, `DomQuadJson`*"]
    #[wasm_bindgen(method, getter = "p2")]
    pub fn get_p2(this: &DomQuadJson) -> Option<DomPoint>;
    #[cfg(feature = "DomPoint")]
    #[doc = "Change the `p2` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPoint`, `DomQuadJson`*"]
    #[wasm_bindgen(method, setter = "p2")]
    pub fn set_p2(this: &DomQuadJson, val: &DomPoint);
    #[cfg(feature = "DomPoint")]
    #[doc = "Get the `p3` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPoint`, `DomQuadJson`*"]
    #[wasm_bindgen(method, getter = "p3")]
    pub fn get_p3(this: &DomQuadJson) -> Option<DomPoint>;
    #[cfg(feature = "DomPoint")]
    #[doc = "Change the `p3` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPoint`, `DomQuadJson`*"]
    #[wasm_bindgen(method, setter = "p3")]
    pub fn set_p3(this: &DomQuadJson, val: &DomPoint);
    #[cfg(feature = "DomPoint")]
    #[doc = "Get the `p4` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPoint`, `DomQuadJson`*"]
    #[wasm_bindgen(method, getter = "p4")]
    pub fn get_p4(this: &DomQuadJson) -> Option<DomPoint>;
    #[cfg(feature = "DomPoint")]
    #[doc = "Change the `p4` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPoint`, `DomQuadJson`*"]
    #[wasm_bindgen(method, setter = "p4")]
    pub fn set_p4(this: &DomQuadJson, val: &DomPoint);
}
impl DomQuadJson {
    #[doc = "Construct a new `DomQuadJson`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomQuadJson`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(feature = "DomPoint")]
    #[deprecated = "Use `set_p1()` instead."]
    pub fn p1(&mut self, val: &DomPoint) -> &mut Self {
        self.set_p1(val);
        self
    }
    #[cfg(feature = "DomPoint")]
    #[deprecated = "Use `set_p2()` instead."]
    pub fn p2(&mut self, val: &DomPoint) -> &mut Self {
        self.set_p2(val);
        self
    }
    #[cfg(feature = "DomPoint")]
    #[deprecated = "Use `set_p3()` instead."]
    pub fn p3(&mut self, val: &DomPoint) -> &mut Self {
        self.set_p3(val);
        self
    }
    #[cfg(feature = "DomPoint")]
    #[deprecated = "Use `set_p4()` instead."]
    pub fn p4(&mut self, val: &DomPoint) -> &mut Self {
        self.set_p4(val);
        self
    }
}
impl Default for DomQuadJson {
    fn default() -> Self {
        Self::new()
    }
}
