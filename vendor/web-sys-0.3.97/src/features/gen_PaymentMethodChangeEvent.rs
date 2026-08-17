#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "PaymentRequestUpdateEvent",
        extends = "Event",
        extends = "::js_sys::Object",
        js_name = "PaymentMethodChangeEvent",
        typescript_type = "PaymentMethodChangeEvent"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PaymentMethodChangeEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentMethodChangeEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PaymentMethodChangeEvent`*"]
    pub type PaymentMethodChangeEvent;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "PaymentMethodChangeEvent",
        js_name = "methodName"
    )]
    #[doc = "Getter for the `methodName` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentMethodChangeEvent/methodName)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PaymentMethodChangeEvent`*"]
    pub fn method_name(this: &PaymentMethodChangeEvent) -> ::alloc::string::String;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "PaymentMethodChangeEvent",
        js_name = "methodDetails"
    )]
    #[doc = "Getter for the `methodDetails` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentMethodChangeEvent/methodDetails)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PaymentMethodChangeEvent`*"]
    pub fn method_details(this: &PaymentMethodChangeEvent) -> Option<::js_sys::Object>;
    #[wasm_bindgen(catch, constructor, js_class = "PaymentMethodChangeEvent")]
    #[doc = "The `new PaymentMethodChangeEvent(..)` constructor, creating a new instance of `PaymentMethodChangeEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentMethodChangeEvent/PaymentMethodChangeEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PaymentMethodChangeEvent`*"]
    pub fn new(type_: &str) -> Result<PaymentMethodChangeEvent, JsValue>;
    #[cfg(feature = "PaymentMethodChangeEventInit")]
    #[wasm_bindgen(catch, constructor, js_class = "PaymentMethodChangeEvent")]
    #[doc = "The `new PaymentMethodChangeEvent(..)` constructor, creating a new instance of `PaymentMethodChangeEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentMethodChangeEvent/PaymentMethodChangeEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PaymentMethodChangeEvent`, `PaymentMethodChangeEventInit`*"]
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &PaymentMethodChangeEventInit,
    ) -> Result<PaymentMethodChangeEvent, JsValue>;
}
