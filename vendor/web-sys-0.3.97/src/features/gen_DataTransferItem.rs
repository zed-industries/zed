#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "DataTransferItem",
        typescript_type = "DataTransferItem"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DataTransferItem` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransferItem)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DataTransferItem`*"]
    pub type DataTransferItem;
    #[wasm_bindgen(method, getter, js_class = "DataTransferItem", js_name = "kind")]
    #[doc = "Getter for the `kind` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransferItem/kind)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DataTransferItem`*"]
    pub fn kind(this: &DataTransferItem) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "DataTransferItem", js_name = "type")]
    #[doc = "Getter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransferItem/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DataTransferItem`*"]
    pub fn type_(this: &DataTransferItem) -> ::alloc::string::String;
    #[cfg(feature = "File")]
    #[wasm_bindgen(catch, method, js_class = "DataTransferItem", js_name = "getAsFile")]
    #[doc = "The `getAsFile()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransferItem/getAsFile)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DataTransferItem`, `File`*"]
    pub fn get_as_file(this: &DataTransferItem) -> Result<Option<File>, JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "FileSystemHandle")]
    #[wasm_bindgen(
        method,
        js_class = "DataTransferItem",
        js_name = "getAsFileSystemHandle"
    )]
    #[doc = "The `getAsFileSystemHandle()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransferItem/getAsFileSystemHandle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DataTransferItem`, `FileSystemHandle`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn get_as_file_system_handle(
        this: &DataTransferItem,
    ) -> ::js_sys::Promise<::js_sys::JsOption<FileSystemHandle>>;
    #[wasm_bindgen(catch, method, js_class = "DataTransferItem", js_name = "getAsString")]
    #[doc = "The `getAsString()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransferItem/getAsString)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DataTransferItem`*"]
    pub fn get_as_string(
        this: &DataTransferItem,
        callback: Option<&::js_sys::Function>,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "FileSystemEntry")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "DataTransferItem",
        js_name = "webkitGetAsEntry"
    )]
    #[doc = "The `webkitGetAsEntry()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransferItem/webkitGetAsEntry)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DataTransferItem`, `FileSystemEntry`*"]
    pub fn webkit_get_as_entry(this: &DataTransferItem)
        -> Result<Option<FileSystemEntry>, JsValue>;
}
