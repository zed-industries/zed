#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "FileSystemHandle",
        extends = "::js_sys::Object",
        js_name = "FileSystemFileHandle",
        typescript_type = "FileSystemFileHandle"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `FileSystemFileHandle` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemFileHandle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemFileHandle`*"]
    pub type FileSystemFileHandle;
    #[wasm_bindgen(
        method,
        js_class = "FileSystemFileHandle",
        js_name = "createSyncAccessHandle"
    )]
    #[doc = "The `createSyncAccessHandle()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemFileHandle/createSyncAccessHandle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemFileHandle`*"]
    pub fn create_sync_access_handle(this: &FileSystemFileHandle) -> ::js_sys::Promise;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(
        feature = "FileSystemSyncAccessHandle",
        feature = "FileSystemSyncAccessHandleOptions",
    ))]
    #[wasm_bindgen(
        method,
        js_class = "FileSystemFileHandle",
        js_name = "createSyncAccessHandle"
    )]
    #[doc = "The `createSyncAccessHandle()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemFileHandle/createSyncAccessHandle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemFileHandle`, `FileSystemSyncAccessHandle`, `FileSystemSyncAccessHandleOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn create_sync_access_handle_with_options(
        this: &FileSystemFileHandle,
        options: &FileSystemSyncAccessHandleOptions,
    ) -> ::js_sys::Promise<FileSystemSyncAccessHandle>;
    #[wasm_bindgen(method, js_class = "FileSystemFileHandle", js_name = "createWritable")]
    #[doc = "The `createWritable()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemFileHandle/createWritable)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemFileHandle`*"]
    pub fn create_writable(this: &FileSystemFileHandle) -> ::js_sys::Promise;
    #[cfg(feature = "FileSystemCreateWritableOptions")]
    #[wasm_bindgen(method, js_class = "FileSystemFileHandle", js_name = "createWritable")]
    #[doc = "The `createWritable()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemFileHandle/createWritable)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemCreateWritableOptions`, `FileSystemFileHandle`*"]
    pub fn create_writable_with_options(
        this: &FileSystemFileHandle,
        options: &FileSystemCreateWritableOptions,
    ) -> ::js_sys::Promise;
    #[wasm_bindgen(method, js_class = "FileSystemFileHandle", js_name = "getFile")]
    #[doc = "The `getFile()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemFileHandle/getFile)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemFileHandle`*"]
    pub fn get_file(this: &FileSystemFileHandle) -> ::js_sys::Promise;
}
