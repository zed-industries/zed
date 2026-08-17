#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "FileSystemHandle",
        typescript_type = "FileSystemHandle"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `FileSystemHandle` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemHandle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemHandle`*"]
    pub type FileSystemHandle;
    #[cfg(feature = "FileSystemHandleKind")]
    #[wasm_bindgen(method, getter, js_class = "FileSystemHandle", js_name = "kind")]
    #[doc = "Getter for the `kind` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemHandle/kind)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemHandle`, `FileSystemHandleKind`*"]
    pub fn kind(this: &FileSystemHandle) -> FileSystemHandleKind;
    #[wasm_bindgen(method, getter, js_class = "FileSystemHandle", js_name = "name")]
    #[doc = "Getter for the `name` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemHandle/name)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemHandle`*"]
    pub fn name(this: &FileSystemHandle) -> ::alloc::string::String;
    #[wasm_bindgen(method, js_class = "FileSystemHandle", js_name = "isSameEntry")]
    #[doc = "The `isSameEntry()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemHandle/isSameEntry)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemHandle`*"]
    pub fn is_same_entry(this: &FileSystemHandle, other: &FileSystemHandle) -> ::js_sys::Promise;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, js_class = "FileSystemHandle", js_name = "queryPermission")]
    #[doc = "The `queryPermission()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemHandle/queryPermission)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemHandle`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn query_permission(this: &FileSystemHandle) -> ::js_sys::Promise<::js_sys::JsString>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "FileSystemHandlePermissionDescriptor")]
    #[wasm_bindgen(method, js_class = "FileSystemHandle", js_name = "queryPermission")]
    #[doc = "The `queryPermission()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemHandle/queryPermission)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemHandle`, `FileSystemHandlePermissionDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn query_permission_with_descriptor(
        this: &FileSystemHandle,
        descriptor: &FileSystemHandlePermissionDescriptor,
    ) -> ::js_sys::Promise<::js_sys::JsString>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, js_class = "FileSystemHandle", js_name = "requestPermission")]
    #[doc = "The `requestPermission()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemHandle/requestPermission)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemHandle`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn request_permission(this: &FileSystemHandle) -> ::js_sys::Promise<::js_sys::JsString>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "FileSystemHandlePermissionDescriptor")]
    #[wasm_bindgen(method, js_class = "FileSystemHandle", js_name = "requestPermission")]
    #[doc = "The `requestPermission()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemHandle/requestPermission)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemHandle`, `FileSystemHandlePermissionDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn request_permission_with_descriptor(
        this: &FileSystemHandle,
        descriptor: &FileSystemHandlePermissionDescriptor,
    ) -> ::js_sys::Promise<::js_sys::JsString>;
}
