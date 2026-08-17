#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "PaymentResponse",
        typescript_type = "PaymentResponse"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PaymentResponse` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentResponse)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PaymentResponse`*"]
    pub type PaymentResponse;
    #[wasm_bindgen(method, getter, js_class = "PaymentResponse", js_name = "requestId")]
    #[doc = "Getter for the `requestId` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentResponse/requestId)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PaymentResponse`*"]
    pub fn request_id(this: &PaymentResponse) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "PaymentResponse", js_name = "methodName")]
    #[doc = "Getter for the `methodName` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentResponse/methodName)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PaymentResponse`*"]
    pub fn method_name(this: &PaymentResponse) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "PaymentResponse", js_name = "details")]
    #[doc = "Getter for the `details` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentResponse/details)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PaymentResponse`*"]
    pub fn details(this: &PaymentResponse) -> ::js_sys::Object;
    #[cfg(feature = "PaymentAddress")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "PaymentResponse",
        js_name = "shippingAddress"
    )]
    #[doc = "Getter for the `shippingAddress` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentResponse/shippingAddress)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PaymentAddress`, `PaymentResponse`*"]
    pub fn shipping_address(this: &PaymentResponse) -> Option<PaymentAddress>;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "PaymentResponse",
        js_name = "shippingOption"
    )]
    #[doc = "Getter for the `shippingOption` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentResponse/shippingOption)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PaymentResponse`*"]
    pub fn shipping_option(this: &PaymentResponse) -> Option<::alloc::string::String>;
    #[wasm_bindgen(method, getter, js_class = "PaymentResponse", js_name = "payerName")]
    #[doc = "Getter for the `payerName` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentResponse/payerName)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PaymentResponse`*"]
    pub fn payer_name(this: &PaymentResponse) -> Option<::alloc::string::String>;
    #[wasm_bindgen(method, getter, js_class = "PaymentResponse", js_name = "payerEmail")]
    #[doc = "Getter for the `payerEmail` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentResponse/payerEmail)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PaymentResponse`*"]
    pub fn payer_email(this: &PaymentResponse) -> Option<::alloc::string::String>;
    #[wasm_bindgen(method, getter, js_class = "PaymentResponse", js_name = "payerPhone")]
    #[doc = "Getter for the `payerPhone` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentResponse/payerPhone)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PaymentResponse`*"]
    pub fn payer_phone(this: &PaymentResponse) -> Option<::alloc::string::String>;
    #[wasm_bindgen(method, js_class = "PaymentResponse")]
    #[doc = "The `complete()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentResponse/complete)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PaymentResponse`*"]
    pub fn complete(this: &PaymentResponse) -> ::js_sys::Promise;
    #[cfg(feature = "PaymentComplete")]
    #[wasm_bindgen(method, js_class = "PaymentResponse", js_name = "complete")]
    #[doc = "The `complete()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentResponse/complete)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PaymentComplete`, `PaymentResponse`*"]
    pub fn complete_with_result(
        this: &PaymentResponse,
        result: PaymentComplete,
    ) -> ::js_sys::Promise;
    #[wasm_bindgen(method, js_class = "PaymentResponse", js_name = "toJSON")]
    #[doc = "The `toJSON()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentResponse/toJSON)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PaymentResponse`*"]
    pub fn to_json(this: &PaymentResponse) -> ::js_sys::Object;
}
