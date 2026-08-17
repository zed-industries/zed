#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "DeviceRotationRateInit")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DeviceRotationRateInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceRotationRateInit`*"]
    pub type DeviceRotationRateInit;
    #[doc = "Get the `alpha` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceRotationRateInit`*"]
    #[wasm_bindgen(method, getter = "alpha")]
    pub fn get_alpha(this: &DeviceRotationRateInit) -> Option<f64>;
    #[doc = "Change the `alpha` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceRotationRateInit`*"]
    #[wasm_bindgen(method, setter = "alpha")]
    pub fn set_alpha(this: &DeviceRotationRateInit, val: Option<f64>);
    #[doc = "Get the `beta` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceRotationRateInit`*"]
    #[wasm_bindgen(method, getter = "beta")]
    pub fn get_beta(this: &DeviceRotationRateInit) -> Option<f64>;
    #[doc = "Change the `beta` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceRotationRateInit`*"]
    #[wasm_bindgen(method, setter = "beta")]
    pub fn set_beta(this: &DeviceRotationRateInit, val: Option<f64>);
    #[doc = "Get the `gamma` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceRotationRateInit`*"]
    #[wasm_bindgen(method, getter = "gamma")]
    pub fn get_gamma(this: &DeviceRotationRateInit) -> Option<f64>;
    #[doc = "Change the `gamma` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceRotationRateInit`*"]
    #[wasm_bindgen(method, setter = "gamma")]
    pub fn set_gamma(this: &DeviceRotationRateInit, val: Option<f64>);
}
impl DeviceRotationRateInit {
    #[doc = "Construct a new `DeviceRotationRateInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceRotationRateInit`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_alpha()` instead."]
    pub fn alpha(&mut self, val: Option<f64>) -> &mut Self {
        self.set_alpha(val);
        self
    }
    #[deprecated = "Use `set_beta()` instead."]
    pub fn beta(&mut self, val: Option<f64>) -> &mut Self {
        self.set_beta(val);
        self
    }
    #[deprecated = "Use `set_gamma()` instead."]
    pub fn gamma(&mut self, val: Option<f64>) -> &mut Self {
        self.set_gamma(val);
        self
    }
}
impl Default for DeviceRotationRateInit {
    fn default() -> Self {
        Self::new()
    }
}
