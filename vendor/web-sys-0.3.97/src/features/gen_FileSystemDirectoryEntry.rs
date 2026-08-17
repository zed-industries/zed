#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "FileSystemEntry",
        extends = "::js_sys::Object",
        js_name = "FileSystemDirectoryEntry",
        typescript_type = "FileSystemDirectoryEntry"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `FileSystemDirectoryEntry` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryEntry)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemDirectoryEntry`*"]
    pub type FileSystemDirectoryEntry;
    #[cfg(feature = "FileSystemDirectoryReader")]
    #[wasm_bindgen(
        method,
        js_class = "FileSystemDirectoryEntry",
        js_name = "createReader"
    )]
    #[doc = "The `createReader()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryEntry/createReader)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemDirectoryEntry`, `FileSystemDirectoryReader`*"]
    pub fn create_reader(this: &FileSystemDirectoryEntry) -> FileSystemDirectoryReader;
    #[wasm_bindgen(
        method,
        js_class = "FileSystemDirectoryEntry",
        js_name = "getDirectory"
    )]
    #[doc = "The `getDirectory()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryEntry/getDirectory)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemDirectoryEntry`*"]
    pub fn get_directory(this: &FileSystemDirectoryEntry);
    #[wasm_bindgen(
        method,
        js_class = "FileSystemDirectoryEntry",
        js_name = "getDirectory"
    )]
    #[doc = "The `getDirectory()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryEntry/getDirectory)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemDirectoryEntry`*"]
    pub fn get_directory_with_path(this: &FileSystemDirectoryEntry, path: Option<&str>);
    #[cfg(feature = "FileSystemFlags")]
    #[wasm_bindgen(
        method,
        js_class = "FileSystemDirectoryEntry",
        js_name = "getDirectory"
    )]
    #[doc = "The `getDirectory()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryEntry/getDirectory)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemDirectoryEntry`, `FileSystemFlags`*"]
    pub fn get_directory_with_path_and_options(
        this: &FileSystemDirectoryEntry,
        path: Option<&str>,
        options: &FileSystemFlags,
    );
    #[cfg(feature = "FileSystemFlags")]
    #[wasm_bindgen(
        method,
        js_class = "FileSystemDirectoryEntry",
        js_name = "getDirectory"
    )]
    #[doc = "The `getDirectory()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryEntry/getDirectory)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemDirectoryEntry`, `FileSystemFlags`*"]
    pub fn get_directory_with_path_and_options_and_callback(
        this: &FileSystemDirectoryEntry,
        path: Option<&str>,
        options: &FileSystemFlags,
        success_callback: &::js_sys::Function,
    );
    #[cfg(all(feature = "FileSystemEntryCallback", feature = "FileSystemFlags",))]
    #[wasm_bindgen(
        method,
        js_class = "FileSystemDirectoryEntry",
        js_name = "getDirectory"
    )]
    #[doc = "The `getDirectory()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryEntry/getDirectory)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemDirectoryEntry`, `FileSystemEntryCallback`, `FileSystemFlags`*"]
    pub fn get_directory_with_path_and_options_and_file_system_entry_callback(
        this: &FileSystemDirectoryEntry,
        path: Option<&str>,
        options: &FileSystemFlags,
        success_callback: &FileSystemEntryCallback,
    );
    #[cfg(feature = "FileSystemFlags")]
    #[wasm_bindgen(
        method,
        js_class = "FileSystemDirectoryEntry",
        js_name = "getDirectory"
    )]
    #[doc = "The `getDirectory()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryEntry/getDirectory)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemDirectoryEntry`, `FileSystemFlags`*"]
    pub fn get_directory_with_path_and_options_and_callback_and_callback(
        this: &FileSystemDirectoryEntry,
        path: Option<&str>,
        options: &FileSystemFlags,
        success_callback: &::js_sys::Function,
        error_callback: &::js_sys::Function,
    );
    #[cfg(all(feature = "FileSystemEntryCallback", feature = "FileSystemFlags",))]
    #[wasm_bindgen(
        method,
        js_class = "FileSystemDirectoryEntry",
        js_name = "getDirectory"
    )]
    #[doc = "The `getDirectory()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryEntry/getDirectory)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemDirectoryEntry`, `FileSystemEntryCallback`, `FileSystemFlags`*"]
    pub fn get_directory_with_path_and_options_and_file_system_entry_callback_and_callback(
        this: &FileSystemDirectoryEntry,
        path: Option<&str>,
        options: &FileSystemFlags,
        success_callback: &FileSystemEntryCallback,
        error_callback: &::js_sys::Function,
    );
    #[cfg(all(feature = "ErrorCallback", feature = "FileSystemFlags",))]
    #[wasm_bindgen(
        method,
        js_class = "FileSystemDirectoryEntry",
        js_name = "getDirectory"
    )]
    #[doc = "The `getDirectory()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryEntry/getDirectory)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ErrorCallback`, `FileSystemDirectoryEntry`, `FileSystemFlags`*"]
    pub fn get_directory_with_path_and_options_and_callback_and_error_callback(
        this: &FileSystemDirectoryEntry,
        path: Option<&str>,
        options: &FileSystemFlags,
        success_callback: &::js_sys::Function,
        error_callback: &ErrorCallback,
    );
    #[cfg(all(
        feature = "ErrorCallback",
        feature = "FileSystemEntryCallback",
        feature = "FileSystemFlags",
    ))]
    #[wasm_bindgen(
        method,
        js_class = "FileSystemDirectoryEntry",
        js_name = "getDirectory"
    )]
    #[doc = "The `getDirectory()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryEntry/getDirectory)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ErrorCallback`, `FileSystemDirectoryEntry`, `FileSystemEntryCallback`, `FileSystemFlags`*"]
    pub fn get_directory_with_path_and_options_and_file_system_entry_callback_and_error_callback(
        this: &FileSystemDirectoryEntry,
        path: Option<&str>,
        options: &FileSystemFlags,
        success_callback: &FileSystemEntryCallback,
        error_callback: &ErrorCallback,
    );
    #[wasm_bindgen(method, js_class = "FileSystemDirectoryEntry", js_name = "getFile")]
    #[doc = "The `getFile()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryEntry/getFile)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemDirectoryEntry`*"]
    pub fn get_file(this: &FileSystemDirectoryEntry);
    #[wasm_bindgen(method, js_class = "FileSystemDirectoryEntry", js_name = "getFile")]
    #[doc = "The `getFile()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryEntry/getFile)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemDirectoryEntry`*"]
    pub fn get_file_with_path(this: &FileSystemDirectoryEntry, path: Option<&str>);
    #[cfg(feature = "FileSystemFlags")]
    #[wasm_bindgen(method, js_class = "FileSystemDirectoryEntry", js_name = "getFile")]
    #[doc = "The `getFile()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryEntry/getFile)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemDirectoryEntry`, `FileSystemFlags`*"]
    pub fn get_file_with_path_and_options(
        this: &FileSystemDirectoryEntry,
        path: Option<&str>,
        options: &FileSystemFlags,
    );
    #[cfg(feature = "FileSystemFlags")]
    #[wasm_bindgen(method, js_class = "FileSystemDirectoryEntry", js_name = "getFile")]
    #[doc = "The `getFile()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryEntry/getFile)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemDirectoryEntry`, `FileSystemFlags`*"]
    pub fn get_file_with_path_and_options_and_callback(
        this: &FileSystemDirectoryEntry,
        path: Option<&str>,
        options: &FileSystemFlags,
        success_callback: &::js_sys::Function,
    );
    #[cfg(all(feature = "FileSystemEntryCallback", feature = "FileSystemFlags",))]
    #[wasm_bindgen(method, js_class = "FileSystemDirectoryEntry", js_name = "getFile")]
    #[doc = "The `getFile()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryEntry/getFile)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemDirectoryEntry`, `FileSystemEntryCallback`, `FileSystemFlags`*"]
    pub fn get_file_with_path_and_options_and_file_system_entry_callback(
        this: &FileSystemDirectoryEntry,
        path: Option<&str>,
        options: &FileSystemFlags,
        success_callback: &FileSystemEntryCallback,
    );
    #[cfg(feature = "FileSystemFlags")]
    #[wasm_bindgen(method, js_class = "FileSystemDirectoryEntry", js_name = "getFile")]
    #[doc = "The `getFile()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryEntry/getFile)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemDirectoryEntry`, `FileSystemFlags`*"]
    pub fn get_file_with_path_and_options_and_callback_and_callback(
        this: &FileSystemDirectoryEntry,
        path: Option<&str>,
        options: &FileSystemFlags,
        success_callback: &::js_sys::Function,
        error_callback: &::js_sys::Function,
    );
    #[cfg(all(feature = "FileSystemEntryCallback", feature = "FileSystemFlags",))]
    #[wasm_bindgen(method, js_class = "FileSystemDirectoryEntry", js_name = "getFile")]
    #[doc = "The `getFile()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryEntry/getFile)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemDirectoryEntry`, `FileSystemEntryCallback`, `FileSystemFlags`*"]
    pub fn get_file_with_path_and_options_and_file_system_entry_callback_and_callback(
        this: &FileSystemDirectoryEntry,
        path: Option<&str>,
        options: &FileSystemFlags,
        success_callback: &FileSystemEntryCallback,
        error_callback: &::js_sys::Function,
    );
    #[cfg(all(feature = "ErrorCallback", feature = "FileSystemFlags",))]
    #[wasm_bindgen(method, js_class = "FileSystemDirectoryEntry", js_name = "getFile")]
    #[doc = "The `getFile()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryEntry/getFile)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ErrorCallback`, `FileSystemDirectoryEntry`, `FileSystemFlags`*"]
    pub fn get_file_with_path_and_options_and_callback_and_error_callback(
        this: &FileSystemDirectoryEntry,
        path: Option<&str>,
        options: &FileSystemFlags,
        success_callback: &::js_sys::Function,
        error_callback: &ErrorCallback,
    );
    #[cfg(all(
        feature = "ErrorCallback",
        feature = "FileSystemEntryCallback",
        feature = "FileSystemFlags",
    ))]
    #[wasm_bindgen(method, js_class = "FileSystemDirectoryEntry", js_name = "getFile")]
    #[doc = "The `getFile()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryEntry/getFile)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ErrorCallback`, `FileSystemDirectoryEntry`, `FileSystemEntryCallback`, `FileSystemFlags`*"]
    pub fn get_file_with_path_and_options_and_file_system_entry_callback_and_error_callback(
        this: &FileSystemDirectoryEntry,
        path: Option<&str>,
        options: &FileSystemFlags,
        success_callback: &FileSystemEntryCallback,
        error_callback: &ErrorCallback,
    );
}
