#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "PaymentMethodChangeEventInit")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PaymentMethodChangeEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PaymentMethodChangeEventInit`*"]
    pub type PaymentMethodChangeEventInit;
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PaymentMethodChangeEventInit`*"]
    #[wasm_bindgen(method, getter = "bubbles")]
    pub fn get_bubbles(this: &PaymentMethodChangeEventInit) -> Option<bool>;
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PaymentMethodChangeEventInit`*"]
    #[wasm_bindgen(method, setter = "bubbles")]
    pub fn set_bubbles(this: &PaymentMethodChangeEventInit, val: bool);
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PaymentMethodChangeEventInit`*"]
    #[wasm_bindgen(method, getter = "cancelable")]
    pub fn get_cancelable(this: &PaymentMethodChangeEventInit) -> Option<bool>;
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PaymentMethodChangeEventInit`*"]
    #[wasm_bindgen(method, setter = "cancelable")]
    pub fn set_cancelable(this: &PaymentMethodChangeEventInit, val: bool);
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PaymentMethodChangeEventInit`*"]
    #[wasm_bindgen(method, getter = "composed")]
    pub fn get_composed(this: &PaymentMethodChangeEventInit) -> Option<bool>;
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PaymentMethodChangeEventInit`*"]
    #[wasm_bindgen(method, setter = "composed")]
    pub fn set_composed(this: &PaymentMethodChangeEventInit, val: bool);
    #[doc = "Get the `methodDetails` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PaymentMethodChangeEventInit`*"]
    #[wasm_bindgen(method, getter = "methodDetails")]
    pub fn get_method_details(this: &PaymentMethodChangeEventInit) -> Option<::js_sys::Object>;
    #[doc = "Change the `methodDetails` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PaymentMethodChangeEventInit`*"]
    #[wasm_bindgen(method, setter = "methodDetails")]
    pub fn set_method_details(this: &PaymentMethodChangeEventInit, val: Option<&::js_sys::Object>);
    #[doc = "Get the `methodName` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PaymentMethodChangeEventInit`*"]
    #[wasm_bindgen(method, getter = "methodName")]
    pub fn get_method_name(this: &PaymentMethodChangeEventInit) -> ::alloc::string::String;
    #[doc = "Change the `methodName` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PaymentMethodChangeEventInit`*"]
    #[wasm_bindgen(method, setter = "methodName")]
    pub fn set_method_name(this: &PaymentMethodChangeEventInit, val: &str);
}
impl PaymentMethodChangeEventInit {
    #[doc = "Construct a new `PaymentMethodChangeEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PaymentMethodChangeEventInit`*"]
    pub fn new(method_name: &str) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_method_name(method_name);
        ret
    }
    #[deprecated = "Use `set_bubbles()` instead."]
    pub fn bubbles(&mut self, val: bool) -> &mut Self {
        self.set_bubbles(val);
        self
    }
    #[deprecated = "Use `set_cancelable()` instead."]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.set_cancelable(val);
        self
    }
    #[deprecated = "Use `set_composed()` instead."]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.set_composed(val);
        self
    }
    #[deprecated = "Use `set_method_details()` instead."]
    pub fn method_details(&mut self, val: Option<&::js_sys::Object>) -> &mut Self {
        self.set_method_details(val);
        self
    }
    #[deprecated = "Use `set_method_name()` instead."]
    pub fn method_name(&mut self, val: &str) -> &mut Self {
        self.set_method_name(val);
        self
    }
}
