#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "DOMQuadInit")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DomQuadInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomQuadInit`*"]
    pub type DomQuadInit;
    #[cfg(feature = "DomPointInit")]
    #[doc = "Get the `p1` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPointInit`, `DomQuadInit`*"]
    #[wasm_bindgen(method, getter = "p1")]
    pub fn get_p1(this: &DomQuadInit) -> Option<DomPointInit>;
    #[cfg(feature = "DomPointInit")]
    #[doc = "Change the `p1` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPointInit`, `DomQuadInit`*"]
    #[wasm_bindgen(method, setter = "p1")]
    pub fn set_p1(this: &DomQuadInit, val: &DomPointInit);
    #[cfg(feature = "DomPointInit")]
    #[doc = "Get the `p2` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPointInit`, `DomQuadInit`*"]
    #[wasm_bindgen(method, getter = "p2")]
    pub fn get_p2(this: &DomQuadInit) -> Option<DomPointInit>;
    #[cfg(feature = "DomPointInit")]
    #[doc = "Change the `p2` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPointInit`, `DomQuadInit`*"]
    #[wasm_bindgen(method, setter = "p2")]
    pub fn set_p2(this: &DomQuadInit, val: &DomPointInit);
    #[cfg(feature = "DomPointInit")]
    #[doc = "Get the `p3` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPointInit`, `DomQuadInit`*"]
    #[wasm_bindgen(method, getter = "p3")]
    pub fn get_p3(this: &DomQuadInit) -> Option<DomPointInit>;
    #[cfg(feature = "DomPointInit")]
    #[doc = "Change the `p3` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPointInit`, `DomQuadInit`*"]
    #[wasm_bindgen(method, setter = "p3")]
    pub fn set_p3(this: &DomQuadInit, val: &DomPointInit);
    #[cfg(feature = "DomPointInit")]
    #[doc = "Get the `p4` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPointInit`, `DomQuadInit`*"]
    #[wasm_bindgen(method, getter = "p4")]
    pub fn get_p4(this: &DomQuadInit) -> Option<DomPointInit>;
    #[cfg(feature = "DomPointInit")]
    #[doc = "Change the `p4` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPointInit`, `DomQuadInit`*"]
    #[wasm_bindgen(method, setter = "p4")]
    pub fn set_p4(this: &DomQuadInit, val: &DomPointInit);
}
impl DomQuadInit {
    #[doc = "Construct a new `DomQuadInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomQuadInit`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(feature = "DomPointInit")]
    #[deprecated = "Use `set_p1()` instead."]
    pub fn p1(&mut self, val: &DomPointInit) -> &mut Self {
        self.set_p1(val);
        self
    }
    #[cfg(feature = "DomPointInit")]
    #[deprecated = "Use `set_p2()` instead."]
    pub fn p2(&mut self, val: &DomPointInit) -> &mut Self {
        self.set_p2(val);
        self
    }
    #[cfg(feature = "DomPointInit")]
    #[deprecated = "Use `set_p3()` instead."]
    pub fn p3(&mut self, val: &DomPointInit) -> &mut Self {
        self.set_p3(val);
        self
    }
    #[cfg(feature = "DomPointInit")]
    #[deprecated = "Use `set_p4()` instead."]
    pub fn p4(&mut self, val: &DomPointInit) -> &mut Self {
        self.set_p4(val);
        self
    }
}
impl Default for DomQuadInit {
    fn default() -> Self {
        Self::new()
    }
}
