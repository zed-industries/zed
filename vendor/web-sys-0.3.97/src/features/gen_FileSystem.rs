#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "FileSystem",
        typescript_type = "FileSystem"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `FileSystem` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystem)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystem`*"]
    pub type FileSystem;
    #[wasm_bindgen(method, getter, js_class = "FileSystem", js_name = "name")]
    #[doc = "Getter for the `name` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystem/name)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystem`*"]
    pub fn name(this: &FileSystem) -> ::alloc::string::String;
    #[cfg(feature = "FileSystemDirectoryEntry")]
    #[wasm_bindgen(method, getter, js_class = "FileSystem", js_name = "root")]
    #[doc = "Getter for the `root` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystem/root)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystem`, `FileSystemDirectoryEntry`*"]
    pub fn root(this: &FileSystem) -> FileSystemDirectoryEntry;
}
