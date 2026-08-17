#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "PaymentAddress",
        typescript_type = "PaymentAddress"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PaymentAddress` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentAddress)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PaymentAddress`*"]
    pub type PaymentAddress;
    #[wasm_bindgen(method, getter, js_class = "PaymentAddress", js_name = "country")]
    #[doc = "Getter for the `country` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentAddress/country)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PaymentAddress`*"]
    pub fn country(this: &PaymentAddress) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "PaymentAddress", js_name = "addressLine")]
    #[doc = "Getter for the `addressLine` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentAddress/addressLine)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PaymentAddress`*"]
    pub fn address_line(this: &PaymentAddress) -> ::js_sys::Array;
    #[wasm_bindgen(method, getter, js_class = "PaymentAddress", js_name = "region")]
    #[doc = "Getter for the `region` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentAddress/region)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PaymentAddress`*"]
    pub fn region(this: &PaymentAddress) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "PaymentAddress", js_name = "city")]
    #[doc = "Getter for the `city` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentAddress/city)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PaymentAddress`*"]
    pub fn city(this: &PaymentAddress) -> ::alloc::string::String;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "PaymentAddress",
        js_name = "dependentLocality"
    )]
    #[doc = "Getter for the `dependentLocality` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentAddress/dependentLocality)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PaymentAddress`*"]
    pub fn dependent_locality(this: &PaymentAddress) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "PaymentAddress", js_name = "postalCode")]
    #[doc = "Getter for the `postalCode` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentAddress/postalCode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PaymentAddress`*"]
    pub fn postal_code(this: &PaymentAddress) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "PaymentAddress", js_name = "sortingCode")]
    #[doc = "Getter for the `sortingCode` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentAddress/sortingCode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PaymentAddress`*"]
    pub fn sorting_code(this: &PaymentAddress) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "PaymentAddress", js_name = "languageCode")]
    #[doc = "Getter for the `languageCode` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentAddress/languageCode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PaymentAddress`*"]
    pub fn language_code(this: &PaymentAddress) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "PaymentAddress", js_name = "organization")]
    #[doc = "Getter for the `organization` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentAddress/organization)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PaymentAddress`*"]
    pub fn organization(this: &PaymentAddress) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "PaymentAddress", js_name = "recipient")]
    #[doc = "Getter for the `recipient` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentAddress/recipient)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PaymentAddress`*"]
    pub fn recipient(this: &PaymentAddress) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "PaymentAddress", js_name = "phone")]
    #[doc = "Getter for the `phone` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentAddress/phone)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PaymentAddress`*"]
    pub fn phone(this: &PaymentAddress) -> ::alloc::string::String;
    #[wasm_bindgen(method, js_class = "PaymentAddress", js_name = "toJSON")]
    #[doc = "The `toJSON()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentAddress/toJSON)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PaymentAddress`*"]
    pub fn to_json(this: &PaymentAddress) -> ::js_sys::Object;
}
