#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "AnimationPropertyValueDetails"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AnimationPropertyValueDetails` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationPropertyValueDetails`*"]
    pub type AnimationPropertyValueDetails;
    #[cfg(feature = "CompositeOperation")]
    #[doc = "Get the `composite` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationPropertyValueDetails`, `CompositeOperation`*"]
    #[wasm_bindgen(method, getter = "composite")]
    pub fn get_composite(this: &AnimationPropertyValueDetails) -> CompositeOperation;
    #[cfg(feature = "CompositeOperation")]
    #[doc = "Change the `composite` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationPropertyValueDetails`, `CompositeOperation`*"]
    #[wasm_bindgen(method, setter = "composite")]
    pub fn set_composite(this: &AnimationPropertyValueDetails, val: CompositeOperation);
    #[doc = "Get the `easing` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationPropertyValueDetails`*"]
    #[wasm_bindgen(method, getter = "easing")]
    pub fn get_easing(this: &AnimationPropertyValueDetails) -> Option<::alloc::string::String>;
    #[doc = "Change the `easing` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationPropertyValueDetails`*"]
    #[wasm_bindgen(method, setter = "easing")]
    pub fn set_easing(this: &AnimationPropertyValueDetails, val: &str);
    #[doc = "Get the `offset` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationPropertyValueDetails`*"]
    #[wasm_bindgen(method, getter = "offset")]
    pub fn get_offset(this: &AnimationPropertyValueDetails) -> f64;
    #[doc = "Change the `offset` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationPropertyValueDetails`*"]
    #[wasm_bindgen(method, setter = "offset")]
    pub fn set_offset(this: &AnimationPropertyValueDetails, val: f64);
    #[doc = "Get the `value` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationPropertyValueDetails`*"]
    #[wasm_bindgen(method, getter = "value")]
    pub fn get_value(this: &AnimationPropertyValueDetails) -> Option<::alloc::string::String>;
    #[doc = "Change the `value` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationPropertyValueDetails`*"]
    #[wasm_bindgen(method, setter = "value")]
    pub fn set_value(this: &AnimationPropertyValueDetails, val: &str);
}
impl AnimationPropertyValueDetails {
    #[cfg(feature = "CompositeOperation")]
    #[doc = "Construct a new `AnimationPropertyValueDetails`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationPropertyValueDetails`, `CompositeOperation`*"]
    pub fn new(composite: CompositeOperation, offset: f64) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_composite(composite);
        ret.set_offset(offset);
        ret
    }
    #[cfg(feature = "CompositeOperation")]
    #[deprecated = "Use `set_composite()` instead."]
    pub fn composite(&mut self, val: CompositeOperation) -> &mut Self {
        self.set_composite(val);
        self
    }
    #[deprecated = "Use `set_easing()` instead."]
    pub fn easing(&mut self, val: &str) -> &mut Self {
        self.set_easing(val);
        self
    }
    #[deprecated = "Use `set_offset()` instead."]
    pub fn offset(&mut self, val: f64) -> &mut Self {
        self.set_offset(val);
        self
    }
    #[deprecated = "Use `set_value()` instead."]
    pub fn value(&mut self, val: &str) -> &mut Self {
        self.set_value(val);
        self
    }
}
