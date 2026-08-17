#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "FileReaderSync",
        typescript_type = "FileReaderSync"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `FileReaderSync` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReaderSync)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileReaderSync`*"]
    pub type FileReaderSync;
    #[wasm_bindgen(catch, constructor, js_class = "FileReaderSync")]
    #[doc = "The `new FileReaderSync(..)` constructor, creating a new instance of `FileReaderSync`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReaderSync/FileReaderSync)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileReaderSync`*"]
    pub fn new() -> Result<FileReaderSync, JsValue>;
    #[cfg(feature = "Blob")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "FileReaderSync",
        js_name = "readAsArrayBuffer"
    )]
    #[doc = "The `readAsArrayBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReaderSync/readAsArrayBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Blob`, `FileReaderSync`*"]
    pub fn read_as_array_buffer(
        this: &FileReaderSync,
        blob: &Blob,
    ) -> Result<::js_sys::ArrayBuffer, JsValue>;
    #[cfg(feature = "Blob")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "FileReaderSync",
        js_name = "readAsBinaryString"
    )]
    #[doc = "The `readAsBinaryString()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReaderSync/readAsBinaryString)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Blob`, `FileReaderSync`*"]
    pub fn read_as_binary_string(
        this: &FileReaderSync,
        blob: &Blob,
    ) -> Result<::alloc::string::String, JsValue>;
    #[cfg(feature = "Blob")]
    #[wasm_bindgen(catch, method, js_class = "FileReaderSync", js_name = "readAsDataURL")]
    #[doc = "The `readAsDataURL()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReaderSync/readAsDataURL)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Blob`, `FileReaderSync`*"]
    pub fn read_as_data_url(
        this: &FileReaderSync,
        blob: &Blob,
    ) -> Result<::alloc::string::String, JsValue>;
    #[cfg(feature = "Blob")]
    #[wasm_bindgen(catch, method, js_class = "FileReaderSync", js_name = "readAsText")]
    #[doc = "The `readAsText()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReaderSync/readAsText)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Blob`, `FileReaderSync`*"]
    pub fn read_as_text(
        this: &FileReaderSync,
        blob: &Blob,
    ) -> Result<::alloc::string::String, JsValue>;
    #[cfg(feature = "Blob")]
    #[wasm_bindgen(catch, method, js_class = "FileReaderSync", js_name = "readAsText")]
    #[doc = "The `readAsText()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReaderSync/readAsText)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Blob`, `FileReaderSync`*"]
    pub fn read_as_text_with_encoding(
        this: &FileReaderSync,
        blob: &Blob,
        encoding: &str,
    ) -> Result<::alloc::string::String, JsValue>;
}
