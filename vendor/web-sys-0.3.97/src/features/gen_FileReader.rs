#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "EventTarget",
        extends = "::js_sys::Object",
        js_name = "FileReader",
        typescript_type = "FileReader"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `FileReader` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReader)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileReader`*"]
    pub type FileReader;
    #[wasm_bindgen(method, getter, js_class = "FileReader", js_name = "readyState")]
    #[doc = "Getter for the `readyState` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/readyState)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileReader`*"]
    pub fn ready_state(this: &FileReader) -> u16;
    #[wasm_bindgen(catch, method, getter, js_class = "FileReader", js_name = "result")]
    #[doc = "Getter for the `result` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/result)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileReader`*"]
    pub fn result(this: &FileReader) -> Result<::wasm_bindgen::JsValue, JsValue>;
    #[cfg(feature = "DomException")]
    #[wasm_bindgen(method, getter, js_class = "FileReader", js_name = "error")]
    #[doc = "Getter for the `error` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/error)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomException`, `FileReader`*"]
    pub fn error(this: &FileReader) -> Option<DomException>;
    #[wasm_bindgen(method, getter, js_class = "FileReader", js_name = "onloadstart")]
    #[doc = "Getter for the `onloadstart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/onloadstart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileReader`*"]
    pub fn onloadstart(this: &FileReader) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "FileReader", js_name = "onloadstart")]
    #[doc = "Setter for the `onloadstart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/onloadstart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileReader`*"]
    pub fn set_onloadstart(this: &FileReader, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "FileReader", js_name = "onprogress")]
    #[doc = "Getter for the `onprogress` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/onprogress)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileReader`*"]
    pub fn onprogress(this: &FileReader) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "FileReader", js_name = "onprogress")]
    #[doc = "Setter for the `onprogress` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/onprogress)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileReader`*"]
    pub fn set_onprogress(this: &FileReader, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "FileReader", js_name = "onload")]
    #[doc = "Getter for the `onload` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/onload)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileReader`*"]
    pub fn onload(this: &FileReader) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "FileReader", js_name = "onload")]
    #[doc = "Setter for the `onload` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/onload)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileReader`*"]
    pub fn set_onload(this: &FileReader, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "FileReader", js_name = "onabort")]
    #[doc = "Getter for the `onabort` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/onabort)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileReader`*"]
    pub fn onabort(this: &FileReader) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "FileReader", js_name = "onabort")]
    #[doc = "Setter for the `onabort` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/onabort)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileReader`*"]
    pub fn set_onabort(this: &FileReader, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "FileReader", js_name = "onerror")]
    #[doc = "Getter for the `onerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/onerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileReader`*"]
    pub fn onerror(this: &FileReader) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "FileReader", js_name = "onerror")]
    #[doc = "Setter for the `onerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/onerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileReader`*"]
    pub fn set_onerror(this: &FileReader, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "FileReader", js_name = "onloadend")]
    #[doc = "Getter for the `onloadend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/onloadend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileReader`*"]
    pub fn onloadend(this: &FileReader) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "FileReader", js_name = "onloadend")]
    #[doc = "Setter for the `onloadend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/onloadend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileReader`*"]
    pub fn set_onloadend(this: &FileReader, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(catch, constructor, js_class = "FileReader")]
    #[doc = "The `new FileReader(..)` constructor, creating a new instance of `FileReader`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/FileReader)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileReader`*"]
    pub fn new() -> Result<FileReader, JsValue>;
    #[wasm_bindgen(method, js_class = "FileReader")]
    #[doc = "The `abort()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/abort)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileReader`*"]
    pub fn abort(this: &FileReader);
    #[cfg(feature = "Blob")]
    #[wasm_bindgen(catch, method, js_class = "FileReader", js_name = "readAsArrayBuffer")]
    #[doc = "The `readAsArrayBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/readAsArrayBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Blob`, `FileReader`*"]
    pub fn read_as_array_buffer(this: &FileReader, blob: &Blob) -> Result<(), JsValue>;
    #[cfg(feature = "Blob")]
    #[wasm_bindgen(catch, method, js_class = "FileReader", js_name = "readAsBinaryString")]
    #[doc = "The `readAsBinaryString()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/readAsBinaryString)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Blob`, `FileReader`*"]
    pub fn read_as_binary_string(this: &FileReader, filedata: &Blob) -> Result<(), JsValue>;
    #[cfg(feature = "Blob")]
    #[wasm_bindgen(catch, method, js_class = "FileReader", js_name = "readAsDataURL")]
    #[doc = "The `readAsDataURL()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/readAsDataURL)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Blob`, `FileReader`*"]
    pub fn read_as_data_url(this: &FileReader, blob: &Blob) -> Result<(), JsValue>;
    #[cfg(feature = "Blob")]
    #[wasm_bindgen(catch, method, js_class = "FileReader", js_name = "readAsText")]
    #[doc = "The `readAsText()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/readAsText)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Blob`, `FileReader`*"]
    pub fn read_as_text(this: &FileReader, blob: &Blob) -> Result<(), JsValue>;
    #[cfg(feature = "Blob")]
    #[wasm_bindgen(catch, method, js_class = "FileReader", js_name = "readAsText")]
    #[doc = "The `readAsText()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/readAsText)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Blob`, `FileReader`*"]
    pub fn read_as_text_with_label(
        this: &FileReader,
        blob: &Blob,
        label: &str,
    ) -> Result<(), JsValue>;
}
impl FileReader {
    #[doc = "The `FileReader.EMPTY` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileReader`*"]
    pub const EMPTY: u16 = 0i64 as u16;
    #[doc = "The `FileReader.LOADING` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileReader`*"]
    pub const LOADING: u16 = 1u64 as u16;
    #[doc = "The `FileReader.DONE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileReader`*"]
    pub const DONE: u16 = 2u64 as u16;
}
