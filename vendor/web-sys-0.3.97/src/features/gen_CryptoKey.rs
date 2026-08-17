#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "CryptoKey",
        typescript_type = "CryptoKey"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CryptoKey` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CryptoKey)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CryptoKey`*"]
    pub type CryptoKey;
    #[wasm_bindgen(method, getter, js_class = "CryptoKey", js_name = "type")]
    #[doc = "Getter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CryptoKey/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CryptoKey`*"]
    pub fn type_(this: &CryptoKey) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "CryptoKey", js_name = "extractable")]
    #[doc = "Getter for the `extractable` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CryptoKey/extractable)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CryptoKey`*"]
    pub fn extractable(this: &CryptoKey) -> bool;
    #[wasm_bindgen(catch, method, getter, js_class = "CryptoKey", js_name = "algorithm")]
    #[doc = "Getter for the `algorithm` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CryptoKey/algorithm)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CryptoKey`*"]
    pub fn algorithm(this: &CryptoKey) -> Result<::js_sys::Object, JsValue>;
    #[wasm_bindgen(method, getter, js_class = "CryptoKey", js_name = "usages")]
    #[doc = "Getter for the `usages` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CryptoKey/usages)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CryptoKey`*"]
    pub fn usages(this: &CryptoKey) -> ::js_sys::Array;
}
