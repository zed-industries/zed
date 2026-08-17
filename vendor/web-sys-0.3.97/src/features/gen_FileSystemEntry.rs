#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "FileSystemEntry",
        typescript_type = "FileSystemEntry"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `FileSystemEntry` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemEntry)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemEntry`*"]
    pub type FileSystemEntry;
    #[wasm_bindgen(method, getter, js_class = "FileSystemEntry", js_name = "isFile")]
    #[doc = "Getter for the `isFile` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemEntry/isFile)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemEntry`*"]
    pub fn is_file(this: &FileSystemEntry) -> bool;
    #[wasm_bindgen(method, getter, js_class = "FileSystemEntry", js_name = "isDirectory")]
    #[doc = "Getter for the `isDirectory` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemEntry/isDirectory)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemEntry`*"]
    pub fn is_directory(this: &FileSystemEntry) -> bool;
    #[wasm_bindgen(method, getter, js_class = "FileSystemEntry", js_name = "name")]
    #[doc = "Getter for the `name` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemEntry/name)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemEntry`*"]
    pub fn name(this: &FileSystemEntry) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "FileSystemEntry", js_name = "fullPath")]
    #[doc = "Getter for the `fullPath` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemEntry/fullPath)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemEntry`*"]
    pub fn full_path(this: &FileSystemEntry) -> ::alloc::string::String;
    #[cfg(feature = "FileSystem")]
    #[wasm_bindgen(method, getter, js_class = "FileSystemEntry", js_name = "filesystem")]
    #[doc = "Getter for the `filesystem` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemEntry/filesystem)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystem`, `FileSystemEntry`*"]
    pub fn filesystem(this: &FileSystemEntry) -> FileSystem;
    #[wasm_bindgen(method, js_class = "FileSystemEntry", js_name = "getParent")]
    #[doc = "The `getParent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemEntry/getParent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemEntry`*"]
    pub fn get_parent(this: &FileSystemEntry);
    #[wasm_bindgen(method, js_class = "FileSystemEntry", js_name = "getParent")]
    #[doc = "The `getParent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemEntry/getParent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemEntry`*"]
    pub fn get_parent_with_callback(this: &FileSystemEntry, success_callback: &::js_sys::Function);
    #[cfg(feature = "FileSystemEntryCallback")]
    #[wasm_bindgen(method, js_class = "FileSystemEntry", js_name = "getParent")]
    #[doc = "The `getParent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemEntry/getParent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemEntry`, `FileSystemEntryCallback`*"]
    pub fn get_parent_with_file_system_entry_callback(
        this: &FileSystemEntry,
        success_callback: &FileSystemEntryCallback,
    );
    #[wasm_bindgen(method, js_class = "FileSystemEntry", js_name = "getParent")]
    #[doc = "The `getParent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemEntry/getParent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemEntry`*"]
    pub fn get_parent_with_callback_and_callback(
        this: &FileSystemEntry,
        success_callback: &::js_sys::Function,
        error_callback: &::js_sys::Function,
    );
    #[cfg(feature = "FileSystemEntryCallback")]
    #[wasm_bindgen(method, js_class = "FileSystemEntry", js_name = "getParent")]
    #[doc = "The `getParent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemEntry/getParent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemEntry`, `FileSystemEntryCallback`*"]
    pub fn get_parent_with_file_system_entry_callback_and_callback(
        this: &FileSystemEntry,
        success_callback: &FileSystemEntryCallback,
        error_callback: &::js_sys::Function,
    );
    #[cfg(feature = "ErrorCallback")]
    #[wasm_bindgen(method, js_class = "FileSystemEntry", js_name = "getParent")]
    #[doc = "The `getParent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemEntry/getParent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ErrorCallback`, `FileSystemEntry`*"]
    pub fn get_parent_with_callback_and_error_callback(
        this: &FileSystemEntry,
        success_callback: &::js_sys::Function,
        error_callback: &ErrorCallback,
    );
    #[cfg(all(feature = "ErrorCallback", feature = "FileSystemEntryCallback",))]
    #[wasm_bindgen(method, js_class = "FileSystemEntry", js_name = "getParent")]
    #[doc = "The `getParent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemEntry/getParent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ErrorCallback`, `FileSystemEntry`, `FileSystemEntryCallback`*"]
    pub fn get_parent_with_file_system_entry_callback_and_error_callback(
        this: &FileSystemEntry,
        success_callback: &FileSystemEntryCallback,
        error_callback: &ErrorCallback,
    );
}
