#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "DOMMatrix2DInit")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DomMatrix2dInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix2dInit`*"]
    pub type DomMatrix2dInit;
    #[doc = "Get the `a` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix2dInit`*"]
    #[wasm_bindgen(method, getter = "a")]
    pub fn get_a(this: &DomMatrix2dInit) -> Option<f64>;
    #[doc = "Change the `a` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix2dInit`*"]
    #[wasm_bindgen(method, setter = "a")]
    pub fn set_a(this: &DomMatrix2dInit, val: f64);
    #[doc = "Get the `b` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix2dInit`*"]
    #[wasm_bindgen(method, getter = "b")]
    pub fn get_b(this: &DomMatrix2dInit) -> Option<f64>;
    #[doc = "Change the `b` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix2dInit`*"]
    #[wasm_bindgen(method, setter = "b")]
    pub fn set_b(this: &DomMatrix2dInit, val: f64);
    #[doc = "Get the `c` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix2dInit`*"]
    #[wasm_bindgen(method, getter = "c")]
    pub fn get_c(this: &DomMatrix2dInit) -> Option<f64>;
    #[doc = "Change the `c` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix2dInit`*"]
    #[wasm_bindgen(method, setter = "c")]
    pub fn set_c(this: &DomMatrix2dInit, val: f64);
    #[doc = "Get the `d` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix2dInit`*"]
    #[wasm_bindgen(method, getter = "d")]
    pub fn get_d(this: &DomMatrix2dInit) -> Option<f64>;
    #[doc = "Change the `d` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix2dInit`*"]
    #[wasm_bindgen(method, setter = "d")]
    pub fn set_d(this: &DomMatrix2dInit, val: f64);
    #[doc = "Get the `e` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix2dInit`*"]
    #[wasm_bindgen(method, getter = "e")]
    pub fn get_e(this: &DomMatrix2dInit) -> Option<f64>;
    #[doc = "Change the `e` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix2dInit`*"]
    #[wasm_bindgen(method, setter = "e")]
    pub fn set_e(this: &DomMatrix2dInit, val: f64);
    #[doc = "Get the `f` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix2dInit`*"]
    #[wasm_bindgen(method, getter = "f")]
    pub fn get_f(this: &DomMatrix2dInit) -> Option<f64>;
    #[doc = "Change the `f` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix2dInit`*"]
    #[wasm_bindgen(method, setter = "f")]
    pub fn set_f(this: &DomMatrix2dInit, val: f64);
    #[doc = "Get the `m11` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix2dInit`*"]
    #[wasm_bindgen(method, getter = "m11")]
    pub fn get_m11(this: &DomMatrix2dInit) -> Option<f64>;
    #[doc = "Change the `m11` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix2dInit`*"]
    #[wasm_bindgen(method, setter = "m11")]
    pub fn set_m11(this: &DomMatrix2dInit, val: f64);
    #[doc = "Get the `m12` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix2dInit`*"]
    #[wasm_bindgen(method, getter = "m12")]
    pub fn get_m12(this: &DomMatrix2dInit) -> Option<f64>;
    #[doc = "Change the `m12` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix2dInit`*"]
    #[wasm_bindgen(method, setter = "m12")]
    pub fn set_m12(this: &DomMatrix2dInit, val: f64);
    #[doc = "Get the `m21` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix2dInit`*"]
    #[wasm_bindgen(method, getter = "m21")]
    pub fn get_m21(this: &DomMatrix2dInit) -> Option<f64>;
    #[doc = "Change the `m21` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix2dInit`*"]
    #[wasm_bindgen(method, setter = "m21")]
    pub fn set_m21(this: &DomMatrix2dInit, val: f64);
    #[doc = "Get the `m22` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix2dInit`*"]
    #[wasm_bindgen(method, getter = "m22")]
    pub fn get_m22(this: &DomMatrix2dInit) -> Option<f64>;
    #[doc = "Change the `m22` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix2dInit`*"]
    #[wasm_bindgen(method, setter = "m22")]
    pub fn set_m22(this: &DomMatrix2dInit, val: f64);
    #[doc = "Get the `m41` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix2dInit`*"]
    #[wasm_bindgen(method, getter = "m41")]
    pub fn get_m41(this: &DomMatrix2dInit) -> Option<f64>;
    #[doc = "Change the `m41` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix2dInit`*"]
    #[wasm_bindgen(method, setter = "m41")]
    pub fn set_m41(this: &DomMatrix2dInit, val: f64);
    #[doc = "Get the `m42` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix2dInit`*"]
    #[wasm_bindgen(method, getter = "m42")]
    pub fn get_m42(this: &DomMatrix2dInit) -> Option<f64>;
    #[doc = "Change the `m42` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix2dInit`*"]
    #[wasm_bindgen(method, setter = "m42")]
    pub fn set_m42(this: &DomMatrix2dInit, val: f64);
}
impl DomMatrix2dInit {
    #[doc = "Construct a new `DomMatrix2dInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrix2dInit`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_a()` instead."]
    pub fn a(&mut self, val: f64) -> &mut Self {
        self.set_a(val);
        self
    }
    #[deprecated = "Use `set_b()` instead."]
    pub fn b(&mut self, val: f64) -> &mut Self {
        self.set_b(val);
        self
    }
    #[deprecated = "Use `set_c()` instead."]
    pub fn c(&mut self, val: f64) -> &mut Self {
        self.set_c(val);
        self
    }
    #[deprecated = "Use `set_d()` instead."]
    pub fn d(&mut self, val: f64) -> &mut Self {
        self.set_d(val);
        self
    }
    #[deprecated = "Use `set_e()` instead."]
    pub fn e(&mut self, val: f64) -> &mut Self {
        self.set_e(val);
        self
    }
    #[deprecated = "Use `set_f()` instead."]
    pub fn f(&mut self, val: f64) -> &mut Self {
        self.set_f(val);
        self
    }
    #[deprecated = "Use `set_m11()` instead."]
    pub fn m11(&mut self, val: f64) -> &mut Self {
        self.set_m11(val);
        self
    }
    #[deprecated = "Use `set_m12()` instead."]
    pub fn m12(&mut self, val: f64) -> &mut Self {
        self.set_m12(val);
        self
    }
    #[deprecated = "Use `set_m21()` instead."]
    pub fn m21(&mut self, val: f64) -> &mut Self {
        self.set_m21(val);
        self
    }
    #[deprecated = "Use `set_m22()` instead."]
    pub fn m22(&mut self, val: f64) -> &mut Self {
        self.set_m22(val);
        self
    }
    #[deprecated = "Use `set_m41()` instead."]
    pub fn m41(&mut self, val: f64) -> &mut Self {
        self.set_m41(val);
        self
    }
    #[deprecated = "Use `set_m42()` instead."]
    pub fn m42(&mut self, val: f64) -> &mut Self {
        self.set_m42(val);
        self
    }
}
impl Default for DomMatrix2dInit {
    fn default() -> Self {
        Self::new()
    }
}
