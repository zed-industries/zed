#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "BasicCardResponse")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `BasicCardResponse` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BasicCardResponse`*"]
    pub type BasicCardResponse;
    #[cfg(feature = "PaymentAddress")]
    #[doc = "Get the `billingAddress` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BasicCardResponse`, `PaymentAddress`*"]
    #[wasm_bindgen(method, getter = "billingAddress")]
    pub fn get_billing_address(this: &BasicCardResponse) -> Option<PaymentAddress>;
    #[cfg(feature = "PaymentAddress")]
    #[doc = "Change the `billingAddress` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BasicCardResponse`, `PaymentAddress`*"]
    #[wasm_bindgen(method, setter = "billingAddress")]
    pub fn set_billing_address(this: &BasicCardResponse, val: Option<&PaymentAddress>);
    #[doc = "Get the `cardNumber` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BasicCardResponse`*"]
    #[wasm_bindgen(method, getter = "cardNumber")]
    pub fn get_card_number(this: &BasicCardResponse) -> ::alloc::string::String;
    #[doc = "Change the `cardNumber` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BasicCardResponse`*"]
    #[wasm_bindgen(method, setter = "cardNumber")]
    pub fn set_card_number(this: &BasicCardResponse, val: &str);
    #[doc = "Get the `cardSecurityCode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BasicCardResponse`*"]
    #[wasm_bindgen(method, getter = "cardSecurityCode")]
    pub fn get_card_security_code(this: &BasicCardResponse) -> Option<::alloc::string::String>;
    #[doc = "Change the `cardSecurityCode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BasicCardResponse`*"]
    #[wasm_bindgen(method, setter = "cardSecurityCode")]
    pub fn set_card_security_code(this: &BasicCardResponse, val: &str);
    #[doc = "Get the `cardholderName` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BasicCardResponse`*"]
    #[wasm_bindgen(method, getter = "cardholderName")]
    pub fn get_cardholder_name(this: &BasicCardResponse) -> Option<::alloc::string::String>;
    #[doc = "Change the `cardholderName` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BasicCardResponse`*"]
    #[wasm_bindgen(method, setter = "cardholderName")]
    pub fn set_cardholder_name(this: &BasicCardResponse, val: &str);
    #[doc = "Get the `expiryMonth` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BasicCardResponse`*"]
    #[wasm_bindgen(method, getter = "expiryMonth")]
    pub fn get_expiry_month(this: &BasicCardResponse) -> Option<::alloc::string::String>;
    #[doc = "Change the `expiryMonth` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BasicCardResponse`*"]
    #[wasm_bindgen(method, setter = "expiryMonth")]
    pub fn set_expiry_month(this: &BasicCardResponse, val: &str);
    #[doc = "Get the `expiryYear` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BasicCardResponse`*"]
    #[wasm_bindgen(method, getter = "expiryYear")]
    pub fn get_expiry_year(this: &BasicCardResponse) -> Option<::alloc::string::String>;
    #[doc = "Change the `expiryYear` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BasicCardResponse`*"]
    #[wasm_bindgen(method, setter = "expiryYear")]
    pub fn set_expiry_year(this: &BasicCardResponse, val: &str);
}
impl BasicCardResponse {
    #[doc = "Construct a new `BasicCardResponse`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BasicCardResponse`*"]
    pub fn new(card_number: &str) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_card_number(card_number);
        ret
    }
    #[cfg(feature = "PaymentAddress")]
    #[deprecated = "Use `set_billing_address()` instead."]
    pub fn billing_address(&mut self, val: Option<&PaymentAddress>) -> &mut Self {
        self.set_billing_address(val);
        self
    }
    #[deprecated = "Use `set_card_number()` instead."]
    pub fn card_number(&mut self, val: &str) -> &mut Self {
        self.set_card_number(val);
        self
    }
    #[deprecated = "Use `set_card_security_code()` instead."]
    pub fn card_security_code(&mut self, val: &str) -> &mut Self {
        self.set_card_security_code(val);
        self
    }
    #[deprecated = "Use `set_cardholder_name()` instead."]
    pub fn cardholder_name(&mut self, val: &str) -> &mut Self {
        self.set_cardholder_name(val);
        self
    }
    #[deprecated = "Use `set_expiry_month()` instead."]
    pub fn expiry_month(&mut self, val: &str) -> &mut Self {
        self.set_expiry_month(val);
        self
    }
    #[deprecated = "Use `set_expiry_year()` instead."]
    pub fn expiry_year(&mut self, val: &str) -> &mut Self {
        self.set_expiry_year(val);
        self
    }
}
