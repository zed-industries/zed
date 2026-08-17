#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "SubtleCrypto",
        typescript_type = "SubtleCrypto"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SubtleCrypto` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SubtleCrypto`*"]
    pub type SubtleCrypto;
    #[cfg(feature = "CryptoKey")]
    #[wasm_bindgen(catch, method, js_class = "SubtleCrypto", js_name = "decrypt")]
    #[doc = "The `decrypt()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/decrypt)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*"]
    pub fn decrypt_with_object_and_buffer_source(
        this: &SubtleCrypto,
        algorithm: &::js_sys::Object,
        key: &CryptoKey,
        data: &::js_sys::Object,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(feature = "CryptoKey")]
    #[wasm_bindgen(catch, method, js_class = "SubtleCrypto", js_name = "decrypt")]
    #[doc = "The `decrypt()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/decrypt)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*"]
    pub fn decrypt_with_str_and_buffer_source(
        this: &SubtleCrypto,
        algorithm: &str,
        key: &CryptoKey,
        data: &::js_sys::Object,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(feature = "CryptoKey")]
    #[wasm_bindgen(catch, method, js_class = "SubtleCrypto", js_name = "decrypt")]
    #[doc = "The `decrypt()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/decrypt)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*"]
    pub fn decrypt_with_object_and_u8_array(
        this: &SubtleCrypto,
        algorithm: &::js_sys::Object,
        key: &CryptoKey,
        data: &[u8],
    ) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(feature = "CryptoKey")]
    #[wasm_bindgen(catch, method, js_class = "SubtleCrypto", js_name = "decrypt")]
    #[doc = "The `decrypt()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/decrypt)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*"]
    pub fn decrypt_with_str_and_u8_array(
        this: &SubtleCrypto,
        algorithm: &str,
        key: &CryptoKey,
        data: &[u8],
    ) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(feature = "CryptoKey")]
    #[wasm_bindgen(catch, method, js_class = "SubtleCrypto", js_name = "decrypt")]
    #[doc = "The `decrypt()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/decrypt)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*"]
    pub fn decrypt_with_object_and_js_u8_array(
        this: &SubtleCrypto,
        algorithm: &::js_sys::Object,
        key: &CryptoKey,
        data: &::js_sys::Uint8Array,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(feature = "CryptoKey")]
    #[wasm_bindgen(catch, method, js_class = "SubtleCrypto", js_name = "decrypt")]
    #[doc = "The `decrypt()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/decrypt)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*"]
    pub fn decrypt_with_str_and_js_u8_array(
        this: &SubtleCrypto,
        algorithm: &str,
        key: &CryptoKey,
        data: &::js_sys::Uint8Array,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(feature = "CryptoKey")]
    #[wasm_bindgen(catch, method, js_class = "SubtleCrypto", js_name = "deriveBits")]
    #[doc = "The `deriveBits()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/deriveBits)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*"]
    pub fn derive_bits_with_object(
        this: &SubtleCrypto,
        algorithm: &::js_sys::Object,
        base_key: &CryptoKey,
        length: u32,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(feature = "CryptoKey")]
    #[wasm_bindgen(catch, method, js_class = "SubtleCrypto", js_name = "deriveBits")]
    #[doc = "The `deriveBits()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/deriveBits)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*"]
    pub fn derive_bits_with_str(
        this: &SubtleCrypto,
        algorithm: &str,
        base_key: &CryptoKey,
        length: u32,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(feature = "CryptoKey")]
    #[wasm_bindgen(catch, method, js_class = "SubtleCrypto", js_name = "deriveKey")]
    #[doc = "The `deriveKey()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/deriveKey)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*"]
    pub fn derive_key_with_object_and_object(
        this: &SubtleCrypto,
        algorithm: &::js_sys::Object,
        base_key: &CryptoKey,
        derived_key_type: &::js_sys::Object,
        extractable: bool,
        key_usages: &::wasm_bindgen::JsValue,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(feature = "CryptoKey")]
    #[wasm_bindgen(catch, method, js_class = "SubtleCrypto", js_name = "deriveKey")]
    #[doc = "The `deriveKey()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/deriveKey)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*"]
    pub fn derive_key_with_str_and_object(
        this: &SubtleCrypto,
        algorithm: &str,
        base_key: &CryptoKey,
        derived_key_type: &::js_sys::Object,
        extractable: bool,
        key_usages: &::wasm_bindgen::JsValue,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(feature = "CryptoKey")]
    #[wasm_bindgen(catch, method, js_class = "SubtleCrypto", js_name = "deriveKey")]
    #[doc = "The `deriveKey()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/deriveKey)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*"]
    pub fn derive_key_with_object_and_str(
        this: &SubtleCrypto,
        algorithm: &::js_sys::Object,
        base_key: &CryptoKey,
        derived_key_type: &str,
        extractable: bool,
        key_usages: &::wasm_bindgen::JsValue,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(feature = "CryptoKey")]
    #[wasm_bindgen(catch, method, js_class = "SubtleCrypto", js_name = "deriveKey")]
    #[doc = "The `deriveKey()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/deriveKey)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*"]
    pub fn derive_key_with_str_and_str(
        this: &SubtleCrypto,
        algorithm: &str,
        base_key: &CryptoKey,
        derived_key_type: &str,
        extractable: bool,
        key_usages: &::wasm_bindgen::JsValue,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "SubtleCrypto", js_name = "digest")]
    #[doc = "The `digest()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/digest)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SubtleCrypto`*"]
    pub fn digest_with_object_and_buffer_source(
        this: &SubtleCrypto,
        algorithm: &::js_sys::Object,
        data: &::js_sys::Object,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "SubtleCrypto", js_name = "digest")]
    #[doc = "The `digest()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/digest)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SubtleCrypto`*"]
    pub fn digest_with_str_and_buffer_source(
        this: &SubtleCrypto,
        algorithm: &str,
        data: &::js_sys::Object,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "SubtleCrypto", js_name = "digest")]
    #[doc = "The `digest()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/digest)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SubtleCrypto`*"]
    pub fn digest_with_object_and_u8_array(
        this: &SubtleCrypto,
        algorithm: &::js_sys::Object,
        data: &[u8],
    ) -> Result<::js_sys::Promise, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "SubtleCrypto", js_name = "digest")]
    #[doc = "The `digest()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/digest)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SubtleCrypto`*"]
    pub fn digest_with_str_and_u8_array(
        this: &SubtleCrypto,
        algorithm: &str,
        data: &[u8],
    ) -> Result<::js_sys::Promise, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "SubtleCrypto", js_name = "digest")]
    #[doc = "The `digest()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/digest)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SubtleCrypto`*"]
    pub fn digest_with_object_and_js_u8_array(
        this: &SubtleCrypto,
        algorithm: &::js_sys::Object,
        data: &::js_sys::Uint8Array,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "SubtleCrypto", js_name = "digest")]
    #[doc = "The `digest()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/digest)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SubtleCrypto`*"]
    pub fn digest_with_str_and_js_u8_array(
        this: &SubtleCrypto,
        algorithm: &str,
        data: &::js_sys::Uint8Array,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(feature = "CryptoKey")]
    #[wasm_bindgen(catch, method, js_class = "SubtleCrypto", js_name = "encrypt")]
    #[doc = "The `encrypt()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/encrypt)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*"]
    pub fn encrypt_with_object_and_buffer_source(
        this: &SubtleCrypto,
        algorithm: &::js_sys::Object,
        key: &CryptoKey,
        data: &::js_sys::Object,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(feature = "CryptoKey")]
    #[wasm_bindgen(catch, method, js_class = "SubtleCrypto", js_name = "encrypt")]
    #[doc = "The `encrypt()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/encrypt)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*"]
    pub fn encrypt_with_str_and_buffer_source(
        this: &SubtleCrypto,
        algorithm: &str,
        key: &CryptoKey,
        data: &::js_sys::Object,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(feature = "CryptoKey")]
    #[wasm_bindgen(catch, method, js_class = "SubtleCrypto", js_name = "encrypt")]
    #[doc = "The `encrypt()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/encrypt)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*"]
    pub fn encrypt_with_object_and_u8_array(
        this: &SubtleCrypto,
        algorithm: &::js_sys::Object,
        key: &CryptoKey,
        data: &[u8],
    ) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(feature = "CryptoKey")]
    #[wasm_bindgen(catch, method, js_class = "SubtleCrypto", js_name = "encrypt")]
    #[doc = "The `encrypt()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/encrypt)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*"]
    pub fn encrypt_with_str_and_u8_array(
        this: &SubtleCrypto,
        algorithm: &str,
        key: &CryptoKey,
        data: &[u8],
    ) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(feature = "CryptoKey")]
    #[wasm_bindgen(catch, method, js_class = "SubtleCrypto", js_name = "encrypt")]
    #[doc = "The `encrypt()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/encrypt)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*"]
    pub fn encrypt_with_object_and_js_u8_array(
        this: &SubtleCrypto,
        algorithm: &::js_sys::Object,
        key: &CryptoKey,
        data: &::js_sys::Uint8Array,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(feature = "CryptoKey")]
    #[wasm_bindgen(catch, method, js_class = "SubtleCrypto", js_name = "encrypt")]
    #[doc = "The `encrypt()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/encrypt)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*"]
    pub fn encrypt_with_str_and_js_u8_array(
        this: &SubtleCrypto,
        algorithm: &str,
        key: &CryptoKey,
        data: &::js_sys::Uint8Array,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(feature = "CryptoKey")]
    #[wasm_bindgen(catch, method, js_class = "SubtleCrypto", js_name = "exportKey")]
    #[doc = "The `exportKey()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/exportKey)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*"]
    pub fn export_key(
        this: &SubtleCrypto,
        format: &str,
        key: &CryptoKey,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "SubtleCrypto", js_name = "generateKey")]
    #[doc = "The `generateKey()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/generateKey)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SubtleCrypto`*"]
    pub fn generate_key_with_object(
        this: &SubtleCrypto,
        algorithm: &::js_sys::Object,
        extractable: bool,
        key_usages: &::wasm_bindgen::JsValue,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "SubtleCrypto", js_name = "generateKey")]
    #[doc = "The `generateKey()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/generateKey)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SubtleCrypto`*"]
    pub fn generate_key_with_str(
        this: &SubtleCrypto,
        algorithm: &str,
        extractable: bool,
        key_usages: &::wasm_bindgen::JsValue,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "SubtleCrypto", js_name = "importKey")]
    #[doc = "The `importKey()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/importKey)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SubtleCrypto`*"]
    pub fn import_key_with_object(
        this: &SubtleCrypto,
        format: &str,
        key_data: &::js_sys::Object,
        algorithm: &::js_sys::Object,
        extractable: bool,
        key_usages: &::wasm_bindgen::JsValue,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "SubtleCrypto", js_name = "importKey")]
    #[doc = "The `importKey()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/importKey)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SubtleCrypto`*"]
    pub fn import_key_with_str(
        this: &SubtleCrypto,
        format: &str,
        key_data: &::js_sys::Object,
        algorithm: &str,
        extractable: bool,
        key_usages: &::wasm_bindgen::JsValue,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(feature = "CryptoKey")]
    #[wasm_bindgen(catch, method, js_class = "SubtleCrypto", js_name = "sign")]
    #[doc = "The `sign()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/sign)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*"]
    pub fn sign_with_object_and_buffer_source(
        this: &SubtleCrypto,
        algorithm: &::js_sys::Object,
        key: &CryptoKey,
        data: &::js_sys::Object,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(feature = "CryptoKey")]
    #[wasm_bindgen(catch, method, js_class = "SubtleCrypto", js_name = "sign")]
    #[doc = "The `sign()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/sign)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*"]
    pub fn sign_with_str_and_buffer_source(
        this: &SubtleCrypto,
        algorithm: &str,
        key: &CryptoKey,
        data: &::js_sys::Object,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(feature = "CryptoKey")]
    #[wasm_bindgen(catch, method, js_class = "SubtleCrypto", js_name = "sign")]
    #[doc = "The `sign()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/sign)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*"]
    pub fn sign_with_object_and_u8_array(
        this: &SubtleCrypto,
        algorithm: &::js_sys::Object,
        key: &CryptoKey,
        data: &[u8],
    ) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(feature = "CryptoKey")]
    #[wasm_bindgen(catch, method, js_class = "SubtleCrypto", js_name = "sign")]
    #[doc = "The `sign()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/sign)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*"]
    pub fn sign_with_str_and_u8_array(
        this: &SubtleCrypto,
        algorithm: &str,
        key: &CryptoKey,
        data: &[u8],
    ) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(feature = "CryptoKey")]
    #[wasm_bindgen(catch, method, js_class = "SubtleCrypto", js_name = "sign")]
    #[doc = "The `sign()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/sign)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*"]
    pub fn sign_with_object_and_js_u8_array(
        this: &SubtleCrypto,
        algorithm: &::js_sys::Object,
        key: &CryptoKey,
        data: &::js_sys::Uint8Array,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(feature = "CryptoKey")]
    #[wasm_bindgen(catch, method, js_class = "SubtleCrypto", js_name = "sign")]
    #[doc = "The `sign()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/sign)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*"]
    pub fn sign_with_str_and_js_u8_array(
        this: &SubtleCrypto,
        algorithm: &str,
        key: &CryptoKey,
        data: &::js_sys::Uint8Array,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(feature = "CryptoKey")]
    #[wasm_bindgen(catch, method, js_class = "SubtleCrypto", js_name = "unwrapKey")]
    #[doc = "The `unwrapKey()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/unwrapKey)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*"]
    pub fn unwrap_key_with_buffer_source_and_object_and_object(
        this: &SubtleCrypto,
        format: &str,
        wrapped_key: &::js_sys::Object,
        unwrapping_key: &CryptoKey,
        unwrap_algorithm: &::js_sys::Object,
        unwrapped_key_algorithm: &::js_sys::Object,
        extractable: bool,
        key_usages: &::wasm_bindgen::JsValue,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(feature = "CryptoKey")]
    #[wasm_bindgen(catch, method, js_class = "SubtleCrypto", js_name = "unwrapKey")]
    #[doc = "The `unwrapKey()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/unwrapKey)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*"]
    pub fn unwrap_key_with_u8_array_and_object_and_object(
        this: &SubtleCrypto,
        format: &str,
        wrapped_key: &[u8],
        unwrapping_key: &CryptoKey,
        unwrap_algorithm: &::js_sys::Object,
        unwrapped_key_algorithm: &::js_sys::Object,
        extractable: bool,
        key_usages: &::wasm_bindgen::JsValue,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(feature = "CryptoKey")]
    #[wasm_bindgen(catch, method, js_class = "SubtleCrypto", js_name = "unwrapKey")]
    #[doc = "The `unwrapKey()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/unwrapKey)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*"]
    pub fn unwrap_key_with_js_u8_array_and_object_and_object(
        this: &SubtleCrypto,
        format: &str,
        wrapped_key: &::js_sys::Uint8Array,
        unwrapping_key: &CryptoKey,
        unwrap_algorithm: &::js_sys::Object,
        unwrapped_key_algorithm: &::js_sys::Object,
        extractable: bool,
        key_usages: &::wasm_bindgen::JsValue,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(feature = "CryptoKey")]
    #[wasm_bindgen(catch, method, js_class = "SubtleCrypto", js_name = "unwrapKey")]
    #[doc = "The `unwrapKey()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/unwrapKey)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*"]
    pub fn unwrap_key_with_buffer_source_and_str_and_object(
        this: &SubtleCrypto,
        format: &str,
        wrapped_key: &::js_sys::Object,
        unwrapping_key: &CryptoKey,
        unwrap_algorithm: &str,
        unwrapped_key_algorithm: &::js_sys::Object,
        extractable: bool,
        key_usages: &::wasm_bindgen::JsValue,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(feature = "CryptoKey")]
    #[wasm_bindgen(catch, method, js_class = "SubtleCrypto", js_name = "unwrapKey")]
    #[doc = "The `unwrapKey()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/unwrapKey)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*"]
    pub fn unwrap_key_with_u8_array_and_str_and_object(
        this: &SubtleCrypto,
        format: &str,
        wrapped_key: &[u8],
        unwrapping_key: &CryptoKey,
        unwrap_algorithm: &str,
        unwrapped_key_algorithm: &::js_sys::Object,
        extractable: bool,
        key_usages: &::wasm_bindgen::JsValue,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(feature = "CryptoKey")]
    #[wasm_bindgen(catch, method, js_class = "SubtleCrypto", js_name = "unwrapKey")]
    #[doc = "The `unwrapKey()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/unwrapKey)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*"]
    pub fn unwrap_key_with_js_u8_array_and_str_and_object(
        this: &SubtleCrypto,
        format: &str,
        wrapped_key: &::js_sys::Uint8Array,
        unwrapping_key: &CryptoKey,
        unwrap_algorithm: &str,
        unwrapped_key_algorithm: &::js_sys::Object,
        extractable: bool,
        key_usages: &::wasm_bindgen::JsValue,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(feature = "CryptoKey")]
    #[wasm_bindgen(catch, method, js_class = "SubtleCrypto", js_name = "unwrapKey")]
    #[doc = "The `unwrapKey()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/unwrapKey)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*"]
    pub fn unwrap_key_with_buffer_source_and_object_and_str(
        this: &SubtleCrypto,
        format: &str,
        wrapped_key: &::js_sys::Object,
        unwrapping_key: &CryptoKey,
        unwrap_algorithm: &::js_sys::Object,
        unwrapped_key_algorithm: &str,
        extractable: bool,
        key_usages: &::wasm_bindgen::JsValue,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(feature = "CryptoKey")]
    #[wasm_bindgen(catch, method, js_class = "SubtleCrypto", js_name = "unwrapKey")]
    #[doc = "The `unwrapKey()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/unwrapKey)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*"]
    pub fn unwrap_key_with_u8_array_and_object_and_str(
        this: &SubtleCrypto,
        format: &str,
        wrapped_key: &[u8],
        unwrapping_key: &CryptoKey,
        unwrap_algorithm: &::js_sys::Object,
        unwrapped_key_algorithm: &str,
        extractable: bool,
        key_usages: &::wasm_bindgen::JsValue,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(feature = "CryptoKey")]
    #[wasm_bindgen(catch, method, js_class = "SubtleCrypto", js_name = "unwrapKey")]
    #[doc = "The `unwrapKey()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/unwrapKey)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*"]
    pub fn unwrap_key_with_js_u8_array_and_object_and_str(
        this: &SubtleCrypto,
        format: &str,
        wrapped_key: &::js_sys::Uint8Array,
        unwrapping_key: &CryptoKey,
        unwrap_algorithm: &::js_sys::Object,
        unwrapped_key_algorithm: &str,
        extractable: bool,
        key_usages: &::wasm_bindgen::JsValue,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(feature = "CryptoKey")]
    #[wasm_bindgen(catch, method, js_class = "SubtleCrypto", js_name = "unwrapKey")]
    #[doc = "The `unwrapKey()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/unwrapKey)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*"]
    pub fn unwrap_key_with_buffer_source_and_str_and_str(
        this: &SubtleCrypto,
        format: &str,
        wrapped_key: &::js_sys::Object,
        unwrapping_key: &CryptoKey,
        unwrap_algorithm: &str,
        unwrapped_key_algorithm: &str,
        extractable: bool,
        key_usages: &::wasm_bindgen::JsValue,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(feature = "CryptoKey")]
    #[wasm_bindgen(catch, method, js_class = "SubtleCrypto", js_name = "unwrapKey")]
    #[doc = "The `unwrapKey()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/unwrapKey)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*"]
    pub fn unwrap_key_with_u8_array_and_str_and_str(
        this: &SubtleCrypto,
        format: &str,
        wrapped_key: &[u8],
        unwrapping_key: &CryptoKey,
        unwrap_algorithm: &str,
        unwrapped_key_algorithm: &str,
        extractable: bool,
        key_usages: &::wasm_bindgen::JsValue,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(feature = "CryptoKey")]
    #[wasm_bindgen(catch, method, js_class = "SubtleCrypto", js_name = "unwrapKey")]
    #[doc = "The `unwrapKey()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/unwrapKey)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*"]
    pub fn unwrap_key_with_js_u8_array_and_str_and_str(
        this: &SubtleCrypto,
        format: &str,
        wrapped_key: &::js_sys::Uint8Array,
        unwrapping_key: &CryptoKey,
        unwrap_algorithm: &str,
        unwrapped_key_algorithm: &str,
        extractable: bool,
        key_usages: &::wasm_bindgen::JsValue,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(feature = "CryptoKey")]
    #[wasm_bindgen(catch, method, js_class = "SubtleCrypto", js_name = "verify")]
    #[doc = "The `verify()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/verify)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*"]
    pub fn verify_with_object_and_buffer_source_and_buffer_source(
        this: &SubtleCrypto,
        algorithm: &::js_sys::Object,
        key: &CryptoKey,
        signature: &::js_sys::Object,
        data: &::js_sys::Object,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(feature = "CryptoKey")]
    #[wasm_bindgen(catch, method, js_class = "SubtleCrypto", js_name = "verify")]
    #[doc = "The `verify()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/verify)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*"]
    pub fn verify_with_str_and_buffer_source_and_buffer_source(
        this: &SubtleCrypto,
        algorithm: &str,
        key: &CryptoKey,
        signature: &::js_sys::Object,
        data: &::js_sys::Object,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(feature = "CryptoKey")]
    #[wasm_bindgen(catch, method, js_class = "SubtleCrypto", js_name = "verify")]
    #[doc = "The `verify()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/verify)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*"]
    pub fn verify_with_object_and_u8_array_and_buffer_source(
        this: &SubtleCrypto,
        algorithm: &::js_sys::Object,
        key: &CryptoKey,
        signature: &[u8],
        data: &::js_sys::Object,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(feature = "CryptoKey")]
    #[wasm_bindgen(catch, method, js_class = "SubtleCrypto", js_name = "verify")]
    #[doc = "The `verify()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/verify)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*"]
    pub fn verify_with_str_and_u8_array_and_buffer_source(
        this: &SubtleCrypto,
        algorithm: &str,
        key: &CryptoKey,
        signature: &[u8],
        data: &::js_sys::Object,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(feature = "CryptoKey")]
    #[wasm_bindgen(catch, method, js_class = "SubtleCrypto", js_name = "verify")]
    #[doc = "The `verify()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/verify)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*"]
    pub fn verify_with_object_and_js_u8_array_and_buffer_source(
        this: &SubtleCrypto,
        algorithm: &::js_sys::Object,
        key: &CryptoKey,
        signature: &::js_sys::Uint8Array,
        data: &::js_sys::Object,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(feature = "CryptoKey")]
    #[wasm_bindgen(catch, method, js_class = "SubtleCrypto", js_name = "verify")]
    #[doc = "The `verify()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/verify)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*"]
    pub fn verify_with_str_and_js_u8_array_and_buffer_source(
        this: &SubtleCrypto,
        algorithm: &str,
        key: &CryptoKey,
        signature: &::js_sys::Uint8Array,
        data: &::js_sys::Object,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(feature = "CryptoKey")]
    #[wasm_bindgen(catch, method, js_class = "SubtleCrypto", js_name = "verify")]
    #[doc = "The `verify()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/verify)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*"]
    pub fn verify_with_object_and_buffer_source_and_u8_array(
        this: &SubtleCrypto,
        algorithm: &::js_sys::Object,
        key: &CryptoKey,
        signature: &::js_sys::Object,
        data: &[u8],
    ) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(feature = "CryptoKey")]
    #[wasm_bindgen(catch, method, js_class = "SubtleCrypto", js_name = "verify")]
    #[doc = "The `verify()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/verify)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*"]
    pub fn verify_with_str_and_buffer_source_and_u8_array(
        this: &SubtleCrypto,
        algorithm: &str,
        key: &CryptoKey,
        signature: &::js_sys::Object,
        data: &[u8],
    ) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(feature = "CryptoKey")]
    #[wasm_bindgen(catch, method, js_class = "SubtleCrypto", js_name = "verify")]
    #[doc = "The `verify()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/verify)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*"]
    pub fn verify_with_object_and_u8_array_and_u8_array(
        this: &SubtleCrypto,
        algorithm: &::js_sys::Object,
        key: &CryptoKey,
        signature: &[u8],
        data: &[u8],
    ) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(feature = "CryptoKey")]
    #[wasm_bindgen(catch, method, js_class = "SubtleCrypto", js_name = "verify")]
    #[doc = "The `verify()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/verify)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*"]
    pub fn verify_with_str_and_u8_array_and_u8_array(
        this: &SubtleCrypto,
        algorithm: &str,
        key: &CryptoKey,
        signature: &[u8],
        data: &[u8],
    ) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(feature = "CryptoKey")]
    #[wasm_bindgen(catch, method, js_class = "SubtleCrypto", js_name = "verify")]
    #[doc = "The `verify()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/verify)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*"]
    pub fn verify_with_object_and_u8_array_and_u8_slice(
        this: &SubtleCrypto,
        algorithm: &::js_sys::Object,
        key: &CryptoKey,
        signature: &::js_sys::Uint8Array,
        data: &[u8],
    ) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(feature = "CryptoKey")]
    #[wasm_bindgen(catch, method, js_class = "SubtleCrypto", js_name = "verify")]
    #[doc = "The `verify()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/verify)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*"]
    pub fn verify_with_str_and_u8_array_and_u8_slice(
        this: &SubtleCrypto,
        algorithm: &str,
        key: &CryptoKey,
        signature: &::js_sys::Uint8Array,
        data: &[u8],
    ) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(feature = "CryptoKey")]
    #[wasm_bindgen(catch, method, js_class = "SubtleCrypto", js_name = "verify")]
    #[doc = "The `verify()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/verify)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*"]
    pub fn verify_with_object_and_buffer_source_and_js_u8_array(
        this: &SubtleCrypto,
        algorithm: &::js_sys::Object,
        key: &CryptoKey,
        signature: &::js_sys::Object,
        data: &::js_sys::Uint8Array,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(feature = "CryptoKey")]
    #[wasm_bindgen(catch, method, js_class = "SubtleCrypto", js_name = "verify")]
    #[doc = "The `verify()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/verify)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*"]
    pub fn verify_with_str_and_buffer_source_and_js_u8_array(
        this: &SubtleCrypto,
        algorithm: &str,
        key: &CryptoKey,
        signature: &::js_sys::Object,
        data: &::js_sys::Uint8Array,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(feature = "CryptoKey")]
    #[wasm_bindgen(catch, method, js_class = "SubtleCrypto", js_name = "verify")]
    #[doc = "The `verify()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/verify)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*"]
    pub fn verify_with_object_and_u8_slice_and_u8_array(
        this: &SubtleCrypto,
        algorithm: &::js_sys::Object,
        key: &CryptoKey,
        signature: &[u8],
        data: &::js_sys::Uint8Array,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(feature = "CryptoKey")]
    #[wasm_bindgen(catch, method, js_class = "SubtleCrypto", js_name = "verify")]
    #[doc = "The `verify()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/verify)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*"]
    pub fn verify_with_str_and_u8_slice_and_u8_array(
        this: &SubtleCrypto,
        algorithm: &str,
        key: &CryptoKey,
        signature: &[u8],
        data: &::js_sys::Uint8Array,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(feature = "CryptoKey")]
    #[wasm_bindgen(catch, method, js_class = "SubtleCrypto", js_name = "verify")]
    #[doc = "The `verify()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/verify)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*"]
    pub fn verify_with_object_and_js_u8_array_and_js_u8_array(
        this: &SubtleCrypto,
        algorithm: &::js_sys::Object,
        key: &CryptoKey,
        signature: &::js_sys::Uint8Array,
        data: &::js_sys::Uint8Array,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(feature = "CryptoKey")]
    #[wasm_bindgen(catch, method, js_class = "SubtleCrypto", js_name = "verify")]
    #[doc = "The `verify()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/verify)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*"]
    pub fn verify_with_str_and_js_u8_array_and_js_u8_array(
        this: &SubtleCrypto,
        algorithm: &str,
        key: &CryptoKey,
        signature: &::js_sys::Uint8Array,
        data: &::js_sys::Uint8Array,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(feature = "CryptoKey")]
    #[wasm_bindgen(catch, method, js_class = "SubtleCrypto", js_name = "wrapKey")]
    #[doc = "The `wrapKey()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/wrapKey)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*"]
    pub fn wrap_key_with_object(
        this: &SubtleCrypto,
        format: &str,
        key: &CryptoKey,
        wrapping_key: &CryptoKey,
        wrap_algorithm: &::js_sys::Object,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(feature = "CryptoKey")]
    #[wasm_bindgen(catch, method, js_class = "SubtleCrypto", js_name = "wrapKey")]
    #[doc = "The `wrapKey()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/wrapKey)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CryptoKey`, `SubtleCrypto`*"]
    pub fn wrap_key_with_str(
        this: &SubtleCrypto,
        format: &str,
        key: &CryptoKey,
        wrapping_key: &CryptoKey,
        wrap_algorithm: &str,
    ) -> Result<::js_sys::Promise, JsValue>;
}
