#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "EventTarget",
        extends = "::js_sys::Object",
        js_name = "IDBFileHandle",
        typescript_type = "IDBFileHandle"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `IdbFileHandle` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbFileHandle`*"]
    #[deprecated]
    pub type IdbFileHandle;
    #[cfg(feature = "IdbMutableFile")]
    #[wasm_bindgen(method, getter, js_class = "IDBFileHandle", js_name = "mutableFile")]
    #[doc = "Getter for the `mutableFile` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/mutableFile)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbFileHandle`, `IdbMutableFile`*"]
    #[deprecated]
    pub fn mutable_file(this: &IdbFileHandle) -> Option<IdbMutableFile>;
    #[cfg(feature = "IdbMutableFile")]
    #[wasm_bindgen(method, getter, js_class = "IDBFileHandle", js_name = "fileHandle")]
    #[doc = "Getter for the `fileHandle` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/fileHandle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbFileHandle`, `IdbMutableFile`*"]
    #[deprecated]
    pub fn file_handle(this: &IdbFileHandle) -> Option<IdbMutableFile>;
    #[wasm_bindgen(method, getter, js_class = "IDBFileHandle", js_name = "active")]
    #[doc = "Getter for the `active` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/active)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbFileHandle`*"]
    #[deprecated]
    pub fn active(this: &IdbFileHandle) -> bool;
    #[wasm_bindgen(method, getter, js_class = "IDBFileHandle", js_name = "location")]
    #[doc = "Getter for the `location` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/location)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbFileHandle`*"]
    #[deprecated]
    pub fn location(this: &IdbFileHandle) -> Option<f64>;
    #[wasm_bindgen(method, setter, js_class = "IDBFileHandle", js_name = "location")]
    #[doc = "Setter for the `location` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/location)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbFileHandle`*"]
    #[deprecated]
    pub fn set_location(this: &IdbFileHandle, value: Option<f64>);
    #[wasm_bindgen(method, setter, js_class = "IDBFileHandle", js_name = "location")]
    #[doc = "Setter for the `location` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/location)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbFileHandle`*"]
    #[deprecated]
    pub fn set_location_opt_u32(this: &IdbFileHandle, value: Option<u32>);
    #[wasm_bindgen(method, setter, js_class = "IDBFileHandle", js_name = "location")]
    #[doc = "Setter for the `location` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/location)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbFileHandle`*"]
    #[deprecated]
    pub fn set_location_opt_f64(this: &IdbFileHandle, value: Option<f64>);
    #[wasm_bindgen(method, getter, js_class = "IDBFileHandle", js_name = "oncomplete")]
    #[doc = "Getter for the `oncomplete` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/oncomplete)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbFileHandle`*"]
    #[deprecated]
    pub fn oncomplete(this: &IdbFileHandle) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "IDBFileHandle", js_name = "oncomplete")]
    #[doc = "Setter for the `oncomplete` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/oncomplete)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbFileHandle`*"]
    #[deprecated]
    pub fn set_oncomplete(this: &IdbFileHandle, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "IDBFileHandle", js_name = "onabort")]
    #[doc = "Getter for the `onabort` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/onabort)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbFileHandle`*"]
    #[deprecated]
    pub fn onabort(this: &IdbFileHandle) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "IDBFileHandle", js_name = "onabort")]
    #[doc = "Setter for the `onabort` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/onabort)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbFileHandle`*"]
    #[deprecated]
    pub fn set_onabort(this: &IdbFileHandle, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "IDBFileHandle", js_name = "onerror")]
    #[doc = "Getter for the `onerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/onerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbFileHandle`*"]
    #[deprecated]
    pub fn onerror(this: &IdbFileHandle) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "IDBFileHandle", js_name = "onerror")]
    #[doc = "Setter for the `onerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/onerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbFileHandle`*"]
    #[deprecated]
    pub fn set_onerror(this: &IdbFileHandle, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(catch, method, js_class = "IDBFileHandle")]
    #[doc = "The `abort()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/abort)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbFileHandle`*"]
    #[deprecated]
    pub fn abort(this: &IdbFileHandle) -> Result<(), JsValue>;
    #[cfg(feature = "IdbFileRequest")]
    #[wasm_bindgen(catch, method, js_class = "IDBFileHandle", js_name = "append")]
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbFileHandle`, `IdbFileRequest`*"]
    #[deprecated]
    pub fn append_with_str(
        this: &IdbFileHandle,
        value: &str,
    ) -> Result<Option<IdbFileRequest>, JsValue>;
    #[cfg(feature = "IdbFileRequest")]
    #[wasm_bindgen(catch, method, js_class = "IDBFileHandle", js_name = "append")]
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbFileHandle`, `IdbFileRequest`*"]
    #[deprecated]
    pub fn append_with_array_buffer(
        this: &IdbFileHandle,
        value: &::js_sys::ArrayBuffer,
    ) -> Result<Option<IdbFileRequest>, JsValue>;
    #[cfg(feature = "IdbFileRequest")]
    #[wasm_bindgen(catch, method, js_class = "IDBFileHandle", js_name = "append")]
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbFileHandle`, `IdbFileRequest`*"]
    #[deprecated]
    pub fn append_with_array_buffer_view(
        this: &IdbFileHandle,
        value: &::js_sys::Object,
    ) -> Result<Option<IdbFileRequest>, JsValue>;
    #[cfg(feature = "IdbFileRequest")]
    #[wasm_bindgen(catch, method, js_class = "IDBFileHandle", js_name = "append")]
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbFileHandle`, `IdbFileRequest`*"]
    #[deprecated]
    pub fn append_with_u8_array(
        this: &IdbFileHandle,
        value: &mut [u8],
    ) -> Result<Option<IdbFileRequest>, JsValue>;
    #[cfg(feature = "IdbFileRequest")]
    #[wasm_bindgen(catch, method, js_class = "IDBFileHandle", js_name = "append")]
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbFileHandle`, `IdbFileRequest`*"]
    #[deprecated]
    pub fn append_with_js_u8_array(
        this: &IdbFileHandle,
        value: &::js_sys::Uint8Array,
    ) -> Result<Option<IdbFileRequest>, JsValue>;
    #[cfg(all(feature = "Blob", feature = "IdbFileRequest",))]
    #[wasm_bindgen(catch, method, js_class = "IDBFileHandle", js_name = "append")]
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Blob`, `IdbFileHandle`, `IdbFileRequest`*"]
    #[deprecated]
    pub fn append_with_blob(
        this: &IdbFileHandle,
        value: &Blob,
    ) -> Result<Option<IdbFileRequest>, JsValue>;
    #[cfg(feature = "IdbFileRequest")]
    #[wasm_bindgen(catch, method, js_class = "IDBFileHandle")]
    #[doc = "The `flush()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/flush)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbFileHandle`, `IdbFileRequest`*"]
    #[deprecated]
    pub fn flush(this: &IdbFileHandle) -> Result<Option<IdbFileRequest>, JsValue>;
    #[cfg(feature = "IdbFileRequest")]
    #[wasm_bindgen(catch, method, js_class = "IDBFileHandle", js_name = "getMetadata")]
    #[doc = "The `getMetadata()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/getMetadata)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbFileHandle`, `IdbFileRequest`*"]
    #[deprecated]
    pub fn get_metadata(this: &IdbFileHandle) -> Result<Option<IdbFileRequest>, JsValue>;
    #[cfg(all(feature = "IdbFileMetadataParameters", feature = "IdbFileRequest",))]
    #[wasm_bindgen(catch, method, js_class = "IDBFileHandle", js_name = "getMetadata")]
    #[doc = "The `getMetadata()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/getMetadata)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbFileHandle`, `IdbFileMetadataParameters`, `IdbFileRequest`*"]
    #[deprecated]
    pub fn get_metadata_with_parameters(
        this: &IdbFileHandle,
        parameters: &IdbFileMetadataParameters,
    ) -> Result<Option<IdbFileRequest>, JsValue>;
    #[cfg(feature = "IdbFileRequest")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "IDBFileHandle",
        js_name = "readAsArrayBuffer"
    )]
    #[doc = "The `readAsArrayBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/readAsArrayBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbFileHandle`, `IdbFileRequest`*"]
    #[deprecated]
    pub fn read_as_array_buffer_with_u32(
        this: &IdbFileHandle,
        size: u32,
    ) -> Result<Option<IdbFileRequest>, JsValue>;
    #[cfg(feature = "IdbFileRequest")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "IDBFileHandle",
        js_name = "readAsArrayBuffer"
    )]
    #[doc = "The `readAsArrayBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/readAsArrayBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbFileHandle`, `IdbFileRequest`*"]
    #[deprecated]
    pub fn read_as_array_buffer_with_f64(
        this: &IdbFileHandle,
        size: f64,
    ) -> Result<Option<IdbFileRequest>, JsValue>;
    #[cfg(feature = "IdbFileRequest")]
    #[wasm_bindgen(catch, method, js_class = "IDBFileHandle", js_name = "readAsText")]
    #[doc = "The `readAsText()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/readAsText)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbFileHandle`, `IdbFileRequest`*"]
    #[deprecated]
    pub fn read_as_text_with_u32(
        this: &IdbFileHandle,
        size: u32,
    ) -> Result<Option<IdbFileRequest>, JsValue>;
    #[cfg(feature = "IdbFileRequest")]
    #[wasm_bindgen(catch, method, js_class = "IDBFileHandle", js_name = "readAsText")]
    #[doc = "The `readAsText()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/readAsText)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbFileHandle`, `IdbFileRequest`*"]
    #[deprecated]
    pub fn read_as_text_with_f64(
        this: &IdbFileHandle,
        size: f64,
    ) -> Result<Option<IdbFileRequest>, JsValue>;
    #[cfg(feature = "IdbFileRequest")]
    #[wasm_bindgen(catch, method, js_class = "IDBFileHandle", js_name = "readAsText")]
    #[doc = "The `readAsText()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/readAsText)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbFileHandle`, `IdbFileRequest`*"]
    #[deprecated]
    pub fn read_as_text_with_u32_and_encoding(
        this: &IdbFileHandle,
        size: u32,
        encoding: Option<&str>,
    ) -> Result<Option<IdbFileRequest>, JsValue>;
    #[cfg(feature = "IdbFileRequest")]
    #[wasm_bindgen(catch, method, js_class = "IDBFileHandle", js_name = "readAsText")]
    #[doc = "The `readAsText()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/readAsText)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbFileHandle`, `IdbFileRequest`*"]
    #[deprecated]
    pub fn read_as_text_with_f64_and_encoding(
        this: &IdbFileHandle,
        size: f64,
        encoding: Option<&str>,
    ) -> Result<Option<IdbFileRequest>, JsValue>;
    #[cfg(feature = "IdbFileRequest")]
    #[wasm_bindgen(catch, method, js_class = "IDBFileHandle")]
    #[doc = "The `truncate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/truncate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbFileHandle`, `IdbFileRequest`*"]
    #[deprecated]
    pub fn truncate(this: &IdbFileHandle) -> Result<Option<IdbFileRequest>, JsValue>;
    #[cfg(feature = "IdbFileRequest")]
    #[wasm_bindgen(catch, method, js_class = "IDBFileHandle", js_name = "truncate")]
    #[doc = "The `truncate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/truncate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbFileHandle`, `IdbFileRequest`*"]
    #[deprecated]
    pub fn truncate_with_u32(
        this: &IdbFileHandle,
        size: u32,
    ) -> Result<Option<IdbFileRequest>, JsValue>;
    #[cfg(feature = "IdbFileRequest")]
    #[wasm_bindgen(catch, method, js_class = "IDBFileHandle", js_name = "truncate")]
    #[doc = "The `truncate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/truncate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbFileHandle`, `IdbFileRequest`*"]
    #[deprecated]
    pub fn truncate_with_f64(
        this: &IdbFileHandle,
        size: f64,
    ) -> Result<Option<IdbFileRequest>, JsValue>;
    #[cfg(feature = "IdbFileRequest")]
    #[wasm_bindgen(catch, method, js_class = "IDBFileHandle", js_name = "write")]
    #[doc = "The `write()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/write)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbFileHandle`, `IdbFileRequest`*"]
    #[deprecated]
    pub fn write_with_str(
        this: &IdbFileHandle,
        value: &str,
    ) -> Result<Option<IdbFileRequest>, JsValue>;
    #[cfg(feature = "IdbFileRequest")]
    #[wasm_bindgen(catch, method, js_class = "IDBFileHandle", js_name = "write")]
    #[doc = "The `write()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/write)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbFileHandle`, `IdbFileRequest`*"]
    #[deprecated]
    pub fn write_with_array_buffer(
        this: &IdbFileHandle,
        value: &::js_sys::ArrayBuffer,
    ) -> Result<Option<IdbFileRequest>, JsValue>;
    #[cfg(feature = "IdbFileRequest")]
    #[wasm_bindgen(catch, method, js_class = "IDBFileHandle", js_name = "write")]
    #[doc = "The `write()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/write)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbFileHandle`, `IdbFileRequest`*"]
    #[deprecated]
    pub fn write_with_array_buffer_view(
        this: &IdbFileHandle,
        value: &::js_sys::Object,
    ) -> Result<Option<IdbFileRequest>, JsValue>;
    #[cfg(feature = "IdbFileRequest")]
    #[wasm_bindgen(catch, method, js_class = "IDBFileHandle", js_name = "write")]
    #[doc = "The `write()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/write)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbFileHandle`, `IdbFileRequest`*"]
    #[deprecated]
    pub fn write_with_u8_array(
        this: &IdbFileHandle,
        value: &[u8],
    ) -> Result<Option<IdbFileRequest>, JsValue>;
    #[cfg(feature = "IdbFileRequest")]
    #[wasm_bindgen(catch, method, js_class = "IDBFileHandle", js_name = "write")]
    #[doc = "The `write()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/write)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbFileHandle`, `IdbFileRequest`*"]
    #[deprecated]
    pub fn write_with_js_u8_array(
        this: &IdbFileHandle,
        value: &::js_sys::Uint8Array,
    ) -> Result<Option<IdbFileRequest>, JsValue>;
    #[cfg(all(feature = "Blob", feature = "IdbFileRequest",))]
    #[wasm_bindgen(catch, method, js_class = "IDBFileHandle", js_name = "write")]
    #[doc = "The `write()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/write)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Blob`, `IdbFileHandle`, `IdbFileRequest`*"]
    #[deprecated]
    pub fn write_with_blob(
        this: &IdbFileHandle,
        value: &Blob,
    ) -> Result<Option<IdbFileRequest>, JsValue>;
}
