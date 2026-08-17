#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "Crypto",
        typescript_type = "Crypto"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Crypto` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Crypto)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Crypto`*"]
    pub type Crypto;
    #[cfg(feature = "SubtleCrypto")]
    #[wasm_bindgen(method, getter, js_class = "Crypto", js_name = "subtle")]
    #[doc = "Getter for the `subtle` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Crypto/subtle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Crypto`, `SubtleCrypto`*"]
    pub fn subtle(this: &Crypto) -> SubtleCrypto;
    #[wasm_bindgen(catch, method, js_class = "Crypto", js_name = "getRandomValues")]
    #[doc = "The `getRandomValues()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Crypto/getRandomValues)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Crypto`*"]
    pub fn get_random_values_with_array_buffer_view(
        this: &Crypto,
        array: &::js_sys::Object,
    ) -> Result<::js_sys::Object, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Crypto", js_name = "getRandomValues")]
    #[doc = "The `getRandomValues()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Crypto/getRandomValues)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Crypto`*"]
    pub fn get_random_values_with_u8_array(
        this: &Crypto,
        array: &mut [u8],
    ) -> Result<::js_sys::Object, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Crypto", js_name = "getRandomValues")]
    #[doc = "The `getRandomValues()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Crypto/getRandomValues)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Crypto`*"]
    pub fn get_random_values_with_js_u8_array(
        this: &Crypto,
        array: &::js_sys::Uint8Array,
    ) -> Result<::js_sys::Object, JsValue>;
    #[wasm_bindgen(method, js_class = "Crypto", js_name = "randomUUID")]
    #[doc = "The `randomUUID()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Crypto/randomUUID)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Crypto`*"]
    pub fn random_uuid(this: &Crypto) -> ::alloc::string::String;
}
