#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "FileSystemSyncAccessHandle",
        typescript_type = "FileSystemSyncAccessHandle"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `FileSystemSyncAccessHandle` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemSyncAccessHandle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemSyncAccessHandle`*"]
    pub type FileSystemSyncAccessHandle;
    #[wasm_bindgen(method, js_class = "FileSystemSyncAccessHandle")]
    #[doc = "The `close()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemSyncAccessHandle/close)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemSyncAccessHandle`*"]
    pub fn close(this: &FileSystemSyncAccessHandle);
    #[wasm_bindgen(catch, method, js_class = "FileSystemSyncAccessHandle")]
    #[doc = "The `flush()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemSyncAccessHandle/flush)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemSyncAccessHandle`*"]
    pub fn flush(this: &FileSystemSyncAccessHandle) -> Result<(), JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "FileSystemSyncAccessHandle",
        js_name = "getSize"
    )]
    #[doc = "The `getSize()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemSyncAccessHandle/getSize)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemSyncAccessHandle`*"]
    pub fn get_size(this: &FileSystemSyncAccessHandle) -> Result<f64, JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "FileSystemSyncAccessHandle",
        js_name = "read"
    )]
    #[doc = "The `read()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemSyncAccessHandle/read)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemSyncAccessHandle`*"]
    pub fn read_with_buffer_source(
        this: &FileSystemSyncAccessHandle,
        buffer: &::js_sys::Object,
    ) -> Result<f64, JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "FileSystemSyncAccessHandle",
        js_name = "read"
    )]
    #[doc = "The `read()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemSyncAccessHandle/read)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemSyncAccessHandle`*"]
    pub fn read_with_u8_array(
        this: &FileSystemSyncAccessHandle,
        buffer: &mut [u8],
    ) -> Result<f64, JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "FileSystemSyncAccessHandle",
        js_name = "read"
    )]
    #[doc = "The `read()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemSyncAccessHandle/read)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemSyncAccessHandle`*"]
    pub fn read_with_js_u8_array(
        this: &FileSystemSyncAccessHandle,
        buffer: &::js_sys::Uint8Array,
    ) -> Result<f64, JsValue>;
    #[cfg(feature = "FileSystemReadWriteOptions")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "FileSystemSyncAccessHandle",
        js_name = "read"
    )]
    #[doc = "The `read()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemSyncAccessHandle/read)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemReadWriteOptions`, `FileSystemSyncAccessHandle`*"]
    pub fn read_with_buffer_source_and_options(
        this: &FileSystemSyncAccessHandle,
        buffer: &::js_sys::Object,
        options: &FileSystemReadWriteOptions,
    ) -> Result<f64, JsValue>;
    #[cfg(feature = "FileSystemReadWriteOptions")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "FileSystemSyncAccessHandle",
        js_name = "read"
    )]
    #[doc = "The `read()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemSyncAccessHandle/read)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemReadWriteOptions`, `FileSystemSyncAccessHandle`*"]
    pub fn read_with_u8_array_and_options(
        this: &FileSystemSyncAccessHandle,
        buffer: &mut [u8],
        options: &FileSystemReadWriteOptions,
    ) -> Result<f64, JsValue>;
    #[cfg(feature = "FileSystemReadWriteOptions")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "FileSystemSyncAccessHandle",
        js_name = "read"
    )]
    #[doc = "The `read()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemSyncAccessHandle/read)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemReadWriteOptions`, `FileSystemSyncAccessHandle`*"]
    pub fn read_with_js_u8_array_and_options(
        this: &FileSystemSyncAccessHandle,
        buffer: &::js_sys::Uint8Array,
        options: &FileSystemReadWriteOptions,
    ) -> Result<f64, JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "FileSystemSyncAccessHandle",
        js_name = "truncate"
    )]
    #[doc = "The `truncate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemSyncAccessHandle/truncate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemSyncAccessHandle`*"]
    pub fn truncate_with_u32(
        this: &FileSystemSyncAccessHandle,
        new_size: u32,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "FileSystemSyncAccessHandle",
        js_name = "truncate"
    )]
    #[doc = "The `truncate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemSyncAccessHandle/truncate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemSyncAccessHandle`*"]
    pub fn truncate_with_f64(
        this: &FileSystemSyncAccessHandle,
        new_size: f64,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "FileSystemSyncAccessHandle",
        js_name = "write"
    )]
    #[doc = "The `write()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemSyncAccessHandle/write)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemSyncAccessHandle`*"]
    pub fn write_with_buffer_source(
        this: &FileSystemSyncAccessHandle,
        buffer: &::js_sys::Object,
    ) -> Result<f64, JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "FileSystemSyncAccessHandle",
        js_name = "write"
    )]
    #[doc = "The `write()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemSyncAccessHandle/write)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemSyncAccessHandle`*"]
    pub fn write_with_u8_array(
        this: &FileSystemSyncAccessHandle,
        buffer: &[u8],
    ) -> Result<f64, JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "FileSystemSyncAccessHandle",
        js_name = "write"
    )]
    #[doc = "The `write()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemSyncAccessHandle/write)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemSyncAccessHandle`*"]
    pub fn write_with_js_u8_array(
        this: &FileSystemSyncAccessHandle,
        buffer: &::js_sys::Uint8Array,
    ) -> Result<f64, JsValue>;
    #[cfg(feature = "FileSystemReadWriteOptions")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "FileSystemSyncAccessHandle",
        js_name = "write"
    )]
    #[doc = "The `write()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemSyncAccessHandle/write)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemReadWriteOptions`, `FileSystemSyncAccessHandle`*"]
    pub fn write_with_buffer_source_and_options(
        this: &FileSystemSyncAccessHandle,
        buffer: &::js_sys::Object,
        options: &FileSystemReadWriteOptions,
    ) -> Result<f64, JsValue>;
    #[cfg(feature = "FileSystemReadWriteOptions")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "FileSystemSyncAccessHandle",
        js_name = "write"
    )]
    #[doc = "The `write()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemSyncAccessHandle/write)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemReadWriteOptions`, `FileSystemSyncAccessHandle`*"]
    pub fn write_with_u8_array_and_options(
        this: &FileSystemSyncAccessHandle,
        buffer: &[u8],
        options: &FileSystemReadWriteOptions,
    ) -> Result<f64, JsValue>;
    #[cfg(feature = "FileSystemReadWriteOptions")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "FileSystemSyncAccessHandle",
        js_name = "write"
    )]
    #[doc = "The `write()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemSyncAccessHandle/write)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemReadWriteOptions`, `FileSystemSyncAccessHandle`*"]
    pub fn write_with_js_u8_array_and_options(
        this: &FileSystemSyncAccessHandle,
        buffer: &::js_sys::Uint8Array,
        options: &FileSystemReadWriteOptions,
    ) -> Result<f64, JsValue>;
}
