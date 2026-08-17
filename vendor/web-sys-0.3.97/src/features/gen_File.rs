#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "Blob",
        extends = "::js_sys::Object",
        js_name = "File",
        typescript_type = "File"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `File` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/File)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `File`*"]
    pub type File;
    #[wasm_bindgen(method, getter, js_class = "File", js_name = "name")]
    #[doc = "Getter for the `name` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/File/name)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `File`*"]
    pub fn name(this: &File) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "File", js_name = "lastModified")]
    #[doc = "Getter for the `lastModified` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/File/lastModified)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `File`*"]
    pub fn last_modified(this: &File) -> f64;
    #[wasm_bindgen(catch, constructor, js_class = "File")]
    #[doc = "The `new File(..)` constructor, creating a new instance of `File`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/File/File)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `File`*"]
    pub fn new_with_buffer_source_sequence(
        file_bits: &::wasm_bindgen::JsValue,
        file_name: &str,
    ) -> Result<File, JsValue>;
    #[wasm_bindgen(catch, constructor, js_class = "File")]
    #[doc = "The `new File(..)` constructor, creating a new instance of `File`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/File/File)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `File`*"]
    pub fn new_with_u8_slice_sequence(
        file_bits: &::wasm_bindgen::JsValue,
        file_name: &str,
    ) -> Result<File, JsValue>;
    #[wasm_bindgen(catch, constructor, js_class = "File")]
    #[doc = "The `new File(..)` constructor, creating a new instance of `File`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/File/File)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `File`*"]
    pub fn new_with_u8_array_sequence(
        file_bits: &::wasm_bindgen::JsValue,
        file_name: &str,
    ) -> Result<File, JsValue>;
    #[wasm_bindgen(catch, constructor, js_class = "File")]
    #[doc = "The `new File(..)` constructor, creating a new instance of `File`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/File/File)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `File`*"]
    pub fn new_with_blob_sequence(
        file_bits: &::wasm_bindgen::JsValue,
        file_name: &str,
    ) -> Result<File, JsValue>;
    #[wasm_bindgen(catch, constructor, js_class = "File")]
    #[doc = "The `new File(..)` constructor, creating a new instance of `File`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/File/File)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `File`*"]
    pub fn new_with_str_sequence(
        file_bits: &::wasm_bindgen::JsValue,
        file_name: &str,
    ) -> Result<File, JsValue>;
    #[cfg(feature = "FilePropertyBag")]
    #[wasm_bindgen(catch, constructor, js_class = "File")]
    #[doc = "The `new File(..)` constructor, creating a new instance of `File`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/File/File)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `File`, `FilePropertyBag`*"]
    pub fn new_with_buffer_source_sequence_and_options(
        file_bits: &::wasm_bindgen::JsValue,
        file_name: &str,
        options: &FilePropertyBag,
    ) -> Result<File, JsValue>;
    #[cfg(feature = "FilePropertyBag")]
    #[wasm_bindgen(catch, constructor, js_class = "File")]
    #[doc = "The `new File(..)` constructor, creating a new instance of `File`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/File/File)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `File`, `FilePropertyBag`*"]
    pub fn new_with_u8_slice_sequence_and_options(
        file_bits: &::wasm_bindgen::JsValue,
        file_name: &str,
        options: &FilePropertyBag,
    ) -> Result<File, JsValue>;
    #[cfg(feature = "FilePropertyBag")]
    #[wasm_bindgen(catch, constructor, js_class = "File")]
    #[doc = "The `new File(..)` constructor, creating a new instance of `File`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/File/File)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `File`, `FilePropertyBag`*"]
    pub fn new_with_u8_array_sequence_and_options(
        file_bits: &::wasm_bindgen::JsValue,
        file_name: &str,
        options: &FilePropertyBag,
    ) -> Result<File, JsValue>;
    #[cfg(feature = "FilePropertyBag")]
    #[wasm_bindgen(catch, constructor, js_class = "File")]
    #[doc = "The `new File(..)` constructor, creating a new instance of `File`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/File/File)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `File`, `FilePropertyBag`*"]
    pub fn new_with_blob_sequence_and_options(
        file_bits: &::wasm_bindgen::JsValue,
        file_name: &str,
        options: &FilePropertyBag,
    ) -> Result<File, JsValue>;
    #[cfg(feature = "FilePropertyBag")]
    #[wasm_bindgen(catch, constructor, js_class = "File")]
    #[doc = "The `new File(..)` constructor, creating a new instance of `File`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/File/File)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `File`, `FilePropertyBag`*"]
    pub fn new_with_str_sequence_and_options(
        file_bits: &::wasm_bindgen::JsValue,
        file_name: &str,
        options: &FilePropertyBag,
    ) -> Result<File, JsValue>;
}
