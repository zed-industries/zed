#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "DOMMatrixInit")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DomMatrixInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixInit`*"]
    pub type DomMatrixInit;
    #[doc = "Get the `a` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixInit`*"]
    #[wasm_bindgen(method, getter = "a")]
    pub fn get_a(this: &DomMatrixInit) -> Option<f64>;
    #[doc = "Change the `a` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixInit`*"]
    #[wasm_bindgen(method, setter = "a")]
    pub fn set_a(this: &DomMatrixInit, val: f64);
    #[doc = "Get the `b` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixInit`*"]
    #[wasm_bindgen(method, getter = "b")]
    pub fn get_b(this: &DomMatrixInit) -> Option<f64>;
    #[doc = "Change the `b` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixInit`*"]
    #[wasm_bindgen(method, setter = "b")]
    pub fn set_b(this: &DomMatrixInit, val: f64);
    #[doc = "Get the `c` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixInit`*"]
    #[wasm_bindgen(method, getter = "c")]
    pub fn get_c(this: &DomMatrixInit) -> Option<f64>;
    #[doc = "Change the `c` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixInit`*"]
    #[wasm_bindgen(method, setter = "c")]
    pub fn set_c(this: &DomMatrixInit, val: f64);
    #[doc = "Get the `d` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixInit`*"]
    #[wasm_bindgen(method, getter = "d")]
    pub fn get_d(this: &DomMatrixInit) -> Option<f64>;
    #[doc = "Change the `d` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixInit`*"]
    #[wasm_bindgen(method, setter = "d")]
    pub fn set_d(this: &DomMatrixInit, val: f64);
    #[doc = "Get the `e` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixInit`*"]
    #[wasm_bindgen(method, getter = "e")]
    pub fn get_e(this: &DomMatrixInit) -> Option<f64>;
    #[doc = "Change the `e` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixInit`*"]
    #[wasm_bindgen(method, setter = "e")]
    pub fn set_e(this: &DomMatrixInit, val: f64);
    #[doc = "Get the `f` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixInit`*"]
    #[wasm_bindgen(method, getter = "f")]
    pub fn get_f(this: &DomMatrixInit) -> Option<f64>;
    #[doc = "Change the `f` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixInit`*"]
    #[wasm_bindgen(method, setter = "f")]
    pub fn set_f(this: &DomMatrixInit, val: f64);
    #[doc = "Get the `m11` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixInit`*"]
    #[wasm_bindgen(method, getter = "m11")]
    pub fn get_m11(this: &DomMatrixInit) -> Option<f64>;
    #[doc = "Change the `m11` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixInit`*"]
    #[wasm_bindgen(method, setter = "m11")]
    pub fn set_m11(this: &DomMatrixInit, val: f64);
    #[doc = "Get the `m12` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixInit`*"]
    #[wasm_bindgen(method, getter = "m12")]
    pub fn get_m12(this: &DomMatrixInit) -> Option<f64>;
    #[doc = "Change the `m12` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixInit`*"]
    #[wasm_bindgen(method, setter = "m12")]
    pub fn set_m12(this: &DomMatrixInit, val: f64);
    #[doc = "Get the `m21` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixInit`*"]
    #[wasm_bindgen(method, getter = "m21")]
    pub fn get_m21(this: &DomMatrixInit) -> Option<f64>;
    #[doc = "Change the `m21` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixInit`*"]
    #[wasm_bindgen(method, setter = "m21")]
    pub fn set_m21(this: &DomMatrixInit, val: f64);
    #[doc = "Get the `m22` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixInit`*"]
    #[wasm_bindgen(method, getter = "m22")]
    pub fn get_m22(this: &DomMatrixInit) -> Option<f64>;
    #[doc = "Change the `m22` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixInit`*"]
    #[wasm_bindgen(method, setter = "m22")]
    pub fn set_m22(this: &DomMatrixInit, val: f64);
    #[doc = "Get the `m41` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixInit`*"]
    #[wasm_bindgen(method, getter = "m41")]
    pub fn get_m41(this: &DomMatrixInit) -> Option<f64>;
    #[doc = "Change the `m41` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixInit`*"]
    #[wasm_bindgen(method, setter = "m41")]
    pub fn set_m41(this: &DomMatrixInit, val: f64);
    #[doc = "Get the `m42` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixInit`*"]
    #[wasm_bindgen(method, getter = "m42")]
    pub fn get_m42(this: &DomMatrixInit) -> Option<f64>;
    #[doc = "Change the `m42` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixInit`*"]
    #[wasm_bindgen(method, setter = "m42")]
    pub fn set_m42(this: &DomMatrixInit, val: f64);
    #[doc = "Get the `is2D` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixInit`*"]
    #[wasm_bindgen(method, getter = "is2D")]
    pub fn get_is_2d(this: &DomMatrixInit) -> Option<bool>;
    #[doc = "Change the `is2D` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixInit`*"]
    #[wasm_bindgen(method, setter = "is2D")]
    pub fn set_is_2d(this: &DomMatrixInit, val: bool);
    #[doc = "Get the `m13` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixInit`*"]
    #[wasm_bindgen(method, getter = "m13")]
    pub fn get_m13(this: &DomMatrixInit) -> Option<f64>;
    #[doc = "Change the `m13` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixInit`*"]
    #[wasm_bindgen(method, setter = "m13")]
    pub fn set_m13(this: &DomMatrixInit, val: f64);
    #[doc = "Get the `m14` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixInit`*"]
    #[wasm_bindgen(method, getter = "m14")]
    pub fn get_m14(this: &DomMatrixInit) -> Option<f64>;
    #[doc = "Change the `m14` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixInit`*"]
    #[wasm_bindgen(method, setter = "m14")]
    pub fn set_m14(this: &DomMatrixInit, val: f64);
    #[doc = "Get the `m23` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixInit`*"]
    #[wasm_bindgen(method, getter = "m23")]
    pub fn get_m23(this: &DomMatrixInit) -> Option<f64>;
    #[doc = "Change the `m23` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixInit`*"]
    #[wasm_bindgen(method, setter = "m23")]
    pub fn set_m23(this: &DomMatrixInit, val: f64);
    #[doc = "Get the `m24` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixInit`*"]
    #[wasm_bindgen(method, getter = "m24")]
    pub fn get_m24(this: &DomMatrixInit) -> Option<f64>;
    #[doc = "Change the `m24` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixInit`*"]
    #[wasm_bindgen(method, setter = "m24")]
    pub fn set_m24(this: &DomMatrixInit, val: f64);
    #[doc = "Get the `m31` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixInit`*"]
    #[wasm_bindgen(method, getter = "m31")]
    pub fn get_m31(this: &DomMatrixInit) -> Option<f64>;
    #[doc = "Change the `m31` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixInit`*"]
    #[wasm_bindgen(method, setter = "m31")]
    pub fn set_m31(this: &DomMatrixInit, val: f64);
    #[doc = "Get the `m32` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixInit`*"]
    #[wasm_bindgen(method, getter = "m32")]
    pub fn get_m32(this: &DomMatrixInit) -> Option<f64>;
    #[doc = "Change the `m32` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixInit`*"]
    #[wasm_bindgen(method, setter = "m32")]
    pub fn set_m32(this: &DomMatrixInit, val: f64);
    #[doc = "Get the `m33` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixInit`*"]
    #[wasm_bindgen(method, getter = "m33")]
    pub fn get_m33(this: &DomMatrixInit) -> Option<f64>;
    #[doc = "Change the `m33` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixInit`*"]
    #[wasm_bindgen(method, setter = "m33")]
    pub fn set_m33(this: &DomMatrixInit, val: f64);
    #[doc = "Get the `m34` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixInit`*"]
    #[wasm_bindgen(method, getter = "m34")]
    pub fn get_m34(this: &DomMatrixInit) -> Option<f64>;
    #[doc = "Change the `m34` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixInit`*"]
    #[wasm_bindgen(method, setter = "m34")]
    pub fn set_m34(this: &DomMatrixInit, val: f64);
    #[doc = "Get the `m43` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixInit`*"]
    #[wasm_bindgen(method, getter = "m43")]
    pub fn get_m43(this: &DomMatrixInit) -> Option<f64>;
    #[doc = "Change the `m43` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixInit`*"]
    #[wasm_bindgen(method, setter = "m43")]
    pub fn set_m43(this: &DomMatrixInit, val: f64);
    #[doc = "Get the `m44` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixInit`*"]
    #[wasm_bindgen(method, getter = "m44")]
    pub fn get_m44(this: &DomMatrixInit) -> Option<f64>;
    #[doc = "Change the `m44` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixInit`*"]
    #[wasm_bindgen(method, setter = "m44")]
    pub fn set_m44(this: &DomMatrixInit, val: f64);
}
impl DomMatrixInit {
    #[doc = "Construct a new `DomMatrixInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomMatrixInit`*"]
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
    #[deprecated = "Use `set_is_2d()` instead."]
    pub fn is_2d(&mut self, val: bool) -> &mut Self {
        self.set_is_2d(val);
        self
    }
    #[deprecated = "Use `set_m13()` instead."]
    pub fn m13(&mut self, val: f64) -> &mut Self {
        self.set_m13(val);
        self
    }
    #[deprecated = "Use `set_m14()` instead."]
    pub fn m14(&mut self, val: f64) -> &mut Self {
        self.set_m14(val);
        self
    }
    #[deprecated = "Use `set_m23()` instead."]
    pub fn m23(&mut self, val: f64) -> &mut Self {
        self.set_m23(val);
        self
    }
    #[deprecated = "Use `set_m24()` instead."]
    pub fn m24(&mut self, val: f64) -> &mut Self {
        self.set_m24(val);
        self
    }
    #[deprecated = "Use `set_m31()` instead."]
    pub fn m31(&mut self, val: f64) -> &mut Self {
        self.set_m31(val);
        self
    }
    #[deprecated = "Use `set_m32()` instead."]
    pub fn m32(&mut self, val: f64) -> &mut Self {
        self.set_m32(val);
        self
    }
    #[deprecated = "Use `set_m33()` instead."]
    pub fn m33(&mut self, val: f64) -> &mut Self {
        self.set_m33(val);
        self
    }
    #[deprecated = "Use `set_m34()` instead."]
    pub fn m34(&mut self, val: f64) -> &mut Self {
        self.set_m34(val);
        self
    }
    #[deprecated = "Use `set_m43()` instead."]
    pub fn m43(&mut self, val: f64) -> &mut Self {
        self.set_m43(val);
        self
    }
    #[deprecated = "Use `set_m44()` instead."]
    pub fn m44(&mut self, val: f64) -> &mut Self {
        self.set_m44(val);
        self
    }
}
impl Default for DomMatrixInit {
    fn default() -> Self {
        Self::new()
    }
}
