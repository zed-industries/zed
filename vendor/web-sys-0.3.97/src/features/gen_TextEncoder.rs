#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "TextEncoder",
        typescript_type = "TextEncoder"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `TextEncoder` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextEncoder)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextEncoder`*"]
    pub type TextEncoder;
    #[wasm_bindgen(method, getter, js_class = "TextEncoder", js_name = "encoding")]
    #[doc = "Getter for the `encoding` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextEncoder/encoding)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextEncoder`*"]
    pub fn encoding(this: &TextEncoder) -> ::alloc::string::String;
    #[wasm_bindgen(catch, constructor, js_class = "TextEncoder")]
    #[doc = "The `new TextEncoder(..)` constructor, creating a new instance of `TextEncoder`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextEncoder/TextEncoder)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextEncoder`*"]
    pub fn new() -> Result<TextEncoder, JsValue>;
    #[wasm_bindgen(method, js_class = "TextEncoder")]
    #[doc = "The `encode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextEncoder/encode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextEncoder`*"]
    pub fn encode(this: &TextEncoder) -> ::alloc::vec::Vec<u8>;
    #[wasm_bindgen(method, js_class = "TextEncoder", js_name = "encode")]
    #[doc = "The `encode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextEncoder/encode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextEncoder`*"]
    pub fn encode_with_input(this: &TextEncoder, input: &str) -> ::alloc::vec::Vec<u8>;
}
