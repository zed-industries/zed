#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "EventTarget",
        extends = "::js_sys::Object",
        js_name = "SFrameTransform",
        typescript_type = "SFrameTransform"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SFrameTransform` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SFrameTransform)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SFrameTransform`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type SFrameTransform;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, getter, js_class = "SFrameTransform", js_name = "onerror")]
    #[doc = "Getter for the `onerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SFrameTransform/onerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SFrameTransform`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn onerror(this: &SFrameTransform) -> Option<::js_sys::Function>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, setter, js_class = "SFrameTransform", js_name = "onerror")]
    #[doc = "Setter for the `onerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SFrameTransform/onerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SFrameTransform`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_onerror(this: &SFrameTransform, value: Option<&::js_sys::Function>);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "ReadableStream")]
    #[wasm_bindgen(method, getter, js_class = "SFrameTransform", js_name = "readable")]
    #[doc = "Getter for the `readable` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SFrameTransform/readable)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStream`, `SFrameTransform`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn readable(this: &SFrameTransform) -> ReadableStream;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "WritableStream")]
    #[wasm_bindgen(method, getter, js_class = "SFrameTransform", js_name = "writable")]
    #[doc = "Getter for the `writable` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SFrameTransform/writable)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SFrameTransform`, `WritableStream`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn writable(this: &SFrameTransform) -> WritableStream;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(catch, constructor, js_class = "SFrameTransform")]
    #[doc = "The `new SFrameTransform(..)` constructor, creating a new instance of `SFrameTransform`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SFrameTransform/SFrameTransform)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SFrameTransform`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Result<SFrameTransform, JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "SFrameTransformOptions")]
    #[wasm_bindgen(catch, constructor, js_class = "SFrameTransform")]
    #[doc = "The `new SFrameTransform(..)` constructor, creating a new instance of `SFrameTransform`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SFrameTransform/SFrameTransform)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SFrameTransform`, `SFrameTransformOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new_with_options(options: &SFrameTransformOptions) -> Result<SFrameTransform, JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "CryptoKey")]
    #[wasm_bindgen(method, js_class = "SFrameTransform", js_name = "setEncryptionKey")]
    #[doc = "The `setEncryptionKey()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SFrameTransform/setEncryptionKey)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CryptoKey`, `SFrameTransform`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_encryption_key(
        this: &SFrameTransform,
        key: &CryptoKey,
    ) -> ::js_sys::Promise<::js_sys::Undefined>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "CryptoKey")]
    #[wasm_bindgen(method, js_class = "SFrameTransform", js_name = "setEncryptionKey")]
    #[doc = "The `setEncryptionKey()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SFrameTransform/setEncryptionKey)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CryptoKey`, `SFrameTransform`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_encryption_key_with_u32(
        this: &SFrameTransform,
        key: &CryptoKey,
        key_id: u32,
    ) -> ::js_sys::Promise<::js_sys::Undefined>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "CryptoKey")]
    #[wasm_bindgen(method, js_class = "SFrameTransform", js_name = "setEncryptionKey")]
    #[doc = "The `setEncryptionKey()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SFrameTransform/setEncryptionKey)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CryptoKey`, `SFrameTransform`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_encryption_key_with_f64(
        this: &SFrameTransform,
        key: &CryptoKey,
        key_id: f64,
    ) -> ::js_sys::Promise<::js_sys::Undefined>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "CryptoKey")]
    #[wasm_bindgen(method, js_class = "SFrameTransform", js_name = "setEncryptionKey")]
    #[doc = "The `setEncryptionKey()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SFrameTransform/setEncryptionKey)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CryptoKey`, `SFrameTransform`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_encryption_key_with_big_int(
        this: &SFrameTransform,
        key: &CryptoKey,
        key_id: &::js_sys::BigInt,
    ) -> ::js_sys::Promise<::js_sys::Undefined>;
}
