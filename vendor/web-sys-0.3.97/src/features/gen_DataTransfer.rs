#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "DataTransfer",
        typescript_type = "DataTransfer"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DataTransfer` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransfer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DataTransfer`*"]
    pub type DataTransfer;
    #[wasm_bindgen(method, getter, js_class = "DataTransfer", js_name = "dropEffect")]
    #[doc = "Getter for the `dropEffect` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransfer/dropEffect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DataTransfer`*"]
    pub fn drop_effect(this: &DataTransfer) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "DataTransfer", js_name = "dropEffect")]
    #[doc = "Setter for the `dropEffect` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransfer/dropEffect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DataTransfer`*"]
    pub fn set_drop_effect(this: &DataTransfer, value: &str);
    #[wasm_bindgen(method, getter, js_class = "DataTransfer", js_name = "effectAllowed")]
    #[doc = "Getter for the `effectAllowed` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransfer/effectAllowed)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DataTransfer`*"]
    pub fn effect_allowed(this: &DataTransfer) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "DataTransfer", js_name = "effectAllowed")]
    #[doc = "Setter for the `effectAllowed` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransfer/effectAllowed)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DataTransfer`*"]
    pub fn set_effect_allowed(this: &DataTransfer, value: &str);
    #[cfg(feature = "DataTransferItemList")]
    #[wasm_bindgen(method, getter, js_class = "DataTransfer", js_name = "items")]
    #[doc = "Getter for the `items` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransfer/items)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DataTransfer`, `DataTransferItemList`*"]
    pub fn items(this: &DataTransfer) -> DataTransferItemList;
    #[wasm_bindgen(method, getter, js_class = "DataTransfer", js_name = "types")]
    #[doc = "Getter for the `types` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransfer/types)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DataTransfer`*"]
    pub fn types(this: &DataTransfer) -> ::js_sys::Array;
    #[cfg(feature = "FileList")]
    #[wasm_bindgen(method, getter, js_class = "DataTransfer", js_name = "files")]
    #[doc = "Getter for the `files` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransfer/files)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DataTransfer`, `FileList`*"]
    pub fn files(this: &DataTransfer) -> Option<FileList>;
    #[wasm_bindgen(catch, constructor, js_class = "DataTransfer")]
    #[doc = "The `new DataTransfer(..)` constructor, creating a new instance of `DataTransfer`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransfer/DataTransfer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DataTransfer`*"]
    pub fn new() -> Result<DataTransfer, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "DataTransfer", js_name = "clearData")]
    #[doc = "The `clearData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransfer/clearData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DataTransfer`*"]
    pub fn clear_data(this: &DataTransfer) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "DataTransfer", js_name = "clearData")]
    #[doc = "The `clearData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransfer/clearData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DataTransfer`*"]
    pub fn clear_data_with_format(this: &DataTransfer, format: &str) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "DataTransfer", js_name = "getData")]
    #[doc = "The `getData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransfer/getData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DataTransfer`*"]
    pub fn get_data(this: &DataTransfer, format: &str) -> Result<::alloc::string::String, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "DataTransfer", js_name = "getFiles")]
    #[doc = "The `getFiles()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransfer/getFiles)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DataTransfer`*"]
    pub fn get_files(this: &DataTransfer) -> Result<::js_sys::Promise, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "DataTransfer", js_name = "getFiles")]
    #[doc = "The `getFiles()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransfer/getFiles)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DataTransfer`*"]
    pub fn get_files_with_recursive_flag(
        this: &DataTransfer,
        recursive_flag: bool,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "DataTransfer",
        js_name = "getFilesAndDirectories"
    )]
    #[doc = "The `getFilesAndDirectories()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransfer/getFilesAndDirectories)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DataTransfer`*"]
    pub fn get_files_and_directories(this: &DataTransfer) -> Result<::js_sys::Promise, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "DataTransfer", js_name = "setData")]
    #[doc = "The `setData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransfer/setData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DataTransfer`*"]
    pub fn set_data(this: &DataTransfer, format: &str, data: &str) -> Result<(), JsValue>;
    #[cfg(feature = "Element")]
    #[wasm_bindgen(method, js_class = "DataTransfer", js_name = "setDragImage")]
    #[doc = "The `setDragImage()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransfer/setDragImage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DataTransfer`, `Element`*"]
    pub fn set_drag_image(this: &DataTransfer, image: &Element, x: i32, y: i32);
}
