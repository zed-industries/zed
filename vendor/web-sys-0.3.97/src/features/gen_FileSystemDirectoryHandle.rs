#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "FileSystemHandle",
        extends = "::js_sys::Object",
        js_name = "FileSystemDirectoryHandle",
        typescript_type = "FileSystemDirectoryHandle"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `FileSystemDirectoryHandle` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryHandle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemDirectoryHandle`*"]
    pub type FileSystemDirectoryHandle;
    #[wasm_bindgen(
        method,
        js_class = "FileSystemDirectoryHandle",
        js_name = "getDirectoryHandle"
    )]
    #[doc = "The `getDirectoryHandle()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryHandle/getDirectoryHandle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemDirectoryHandle`*"]
    pub fn get_directory_handle(this: &FileSystemDirectoryHandle, name: &str) -> ::js_sys::Promise;
    #[cfg(feature = "FileSystemGetDirectoryOptions")]
    #[wasm_bindgen(
        method,
        js_class = "FileSystemDirectoryHandle",
        js_name = "getDirectoryHandle"
    )]
    #[doc = "The `getDirectoryHandle()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryHandle/getDirectoryHandle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemDirectoryHandle`, `FileSystemGetDirectoryOptions`*"]
    pub fn get_directory_handle_with_options(
        this: &FileSystemDirectoryHandle,
        name: &str,
        options: &FileSystemGetDirectoryOptions,
    ) -> ::js_sys::Promise;
    #[wasm_bindgen(
        method,
        js_class = "FileSystemDirectoryHandle",
        js_name = "getFileHandle"
    )]
    #[doc = "The `getFileHandle()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryHandle/getFileHandle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemDirectoryHandle`*"]
    pub fn get_file_handle(this: &FileSystemDirectoryHandle, name: &str) -> ::js_sys::Promise;
    #[cfg(feature = "FileSystemGetFileOptions")]
    #[wasm_bindgen(
        method,
        js_class = "FileSystemDirectoryHandle",
        js_name = "getFileHandle"
    )]
    #[doc = "The `getFileHandle()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryHandle/getFileHandle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemDirectoryHandle`, `FileSystemGetFileOptions`*"]
    pub fn get_file_handle_with_options(
        this: &FileSystemDirectoryHandle,
        name: &str,
        options: &FileSystemGetFileOptions,
    ) -> ::js_sys::Promise;
    #[wasm_bindgen(
        method,
        js_class = "FileSystemDirectoryHandle",
        js_name = "removeEntry"
    )]
    #[doc = "The `removeEntry()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryHandle/removeEntry)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemDirectoryHandle`*"]
    pub fn remove_entry(this: &FileSystemDirectoryHandle, name: &str) -> ::js_sys::Promise;
    #[cfg(feature = "FileSystemRemoveOptions")]
    #[wasm_bindgen(
        method,
        js_class = "FileSystemDirectoryHandle",
        js_name = "removeEntry"
    )]
    #[doc = "The `removeEntry()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryHandle/removeEntry)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemDirectoryHandle`, `FileSystemRemoveOptions`*"]
    pub fn remove_entry_with_options(
        this: &FileSystemDirectoryHandle,
        name: &str,
        options: &FileSystemRemoveOptions,
    ) -> ::js_sys::Promise;
    #[wasm_bindgen(method, js_class = "FileSystemDirectoryHandle")]
    #[doc = "The `resolve()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryHandle/resolve)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemDirectoryHandle`*"]
    pub fn resolve(
        this: &FileSystemDirectoryHandle,
        possible_descendant: &FileSystemHandle,
    ) -> ::js_sys::Promise;
    #[wasm_bindgen(method, js_class = "FileSystemDirectoryHandle")]
    #[doc = "The `entries()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryHandle/entries)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemDirectoryHandle`*"]
    pub fn entries(this: &FileSystemDirectoryHandle) -> ::js_sys::AsyncIterator;
    #[wasm_bindgen(method, js_class = "FileSystemDirectoryHandle")]
    #[doc = "The `keys()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryHandle/keys)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemDirectoryHandle`*"]
    pub fn keys(this: &FileSystemDirectoryHandle) -> ::js_sys::AsyncIterator;
    #[wasm_bindgen(method, js_class = "FileSystemDirectoryHandle")]
    #[doc = "The `values()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryHandle/values)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemDirectoryHandle`*"]
    pub fn values(this: &FileSystemDirectoryHandle) -> ::js_sys::AsyncIterator;
}
