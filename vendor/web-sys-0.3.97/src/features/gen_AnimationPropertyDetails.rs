#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "AnimationPropertyDetails")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AnimationPropertyDetails` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationPropertyDetails`*"]
    pub type AnimationPropertyDetails;
    #[doc = "Get the `property` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationPropertyDetails`*"]
    #[wasm_bindgen(method, getter = "property")]
    pub fn get_property(this: &AnimationPropertyDetails) -> ::alloc::string::String;
    #[doc = "Change the `property` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationPropertyDetails`*"]
    #[wasm_bindgen(method, setter = "property")]
    pub fn set_property(this: &AnimationPropertyDetails, val: &str);
    #[doc = "Get the `runningOnCompositor` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationPropertyDetails`*"]
    #[wasm_bindgen(method, getter = "runningOnCompositor")]
    pub fn get_running_on_compositor(this: &AnimationPropertyDetails) -> bool;
    #[doc = "Change the `runningOnCompositor` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationPropertyDetails`*"]
    #[wasm_bindgen(method, setter = "runningOnCompositor")]
    pub fn set_running_on_compositor(this: &AnimationPropertyDetails, val: bool);
    #[doc = "Get the `values` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationPropertyDetails`*"]
    #[wasm_bindgen(method, getter = "values")]
    pub fn get_values(this: &AnimationPropertyDetails) -> ::js_sys::Array;
    #[doc = "Change the `values` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationPropertyDetails`*"]
    #[wasm_bindgen(method, setter = "values")]
    pub fn set_values(this: &AnimationPropertyDetails, val: &::wasm_bindgen::JsValue);
    #[doc = "Get the `warning` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationPropertyDetails`*"]
    #[wasm_bindgen(method, getter = "warning")]
    pub fn get_warning(this: &AnimationPropertyDetails) -> Option<::alloc::string::String>;
    #[doc = "Change the `warning` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationPropertyDetails`*"]
    #[wasm_bindgen(method, setter = "warning")]
    pub fn set_warning(this: &AnimationPropertyDetails, val: &str);
}
impl AnimationPropertyDetails {
    #[doc = "Construct a new `AnimationPropertyDetails`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationPropertyDetails`*"]
    pub fn new(
        property: &str,
        running_on_compositor: bool,
        values: &::wasm_bindgen::JsValue,
    ) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_property(property);
        ret.set_running_on_compositor(running_on_compositor);
        ret.set_values(values);
        ret
    }
    #[deprecated = "Use `set_property()` instead."]
    pub fn property(&mut self, val: &str) -> &mut Self {
        self.set_property(val);
        self
    }
    #[deprecated = "Use `set_running_on_compositor()` instead."]
    pub fn running_on_compositor(&mut self, val: bool) -> &mut Self {
        self.set_running_on_compositor(val);
        self
    }
    #[deprecated = "Use `set_values()` instead."]
    pub fn values(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_values(val);
        self
    }
    #[deprecated = "Use `set_warning()` instead."]
    pub fn warning(&mut self, val: &str) -> &mut Self {
        self.set_warning(val);
        self
    }
}
