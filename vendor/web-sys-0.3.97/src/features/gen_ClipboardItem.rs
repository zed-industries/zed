#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "ClipboardItem",
        typescript_type = "ClipboardItem"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ClipboardItem` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ClipboardItem)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ClipboardItem`*"]
    pub type ClipboardItem;
    #[cfg(feature = "PresentationStyle")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "ClipboardItem",
        js_name = "presentationStyle"
    )]
    #[doc = "Getter for the `presentationStyle` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ClipboardItem/presentationStyle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ClipboardItem`, `PresentationStyle`*"]
    pub fn presentation_style(this: &ClipboardItem) -> PresentationStyle;
    #[wasm_bindgen(method, getter, js_class = "ClipboardItem", js_name = "types")]
    #[doc = "Getter for the `types` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ClipboardItem/types)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ClipboardItem`*"]
    pub fn types(this: &ClipboardItem) -> ::js_sys::Array;
    #[wasm_bindgen(catch, constructor, js_class = "ClipboardItem")]
    #[doc = "The `new ClipboardItem(..)` constructor, creating a new instance of `ClipboardItem`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ClipboardItem/ClipboardItem)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ClipboardItem`*"]
    pub fn new_with_record_from_str_to_str_promise(
        items: &::js_sys::Object,
    ) -> Result<ClipboardItem, JsValue>;
    #[wasm_bindgen(catch, constructor, js_class = "ClipboardItem")]
    #[doc = "The `new ClipboardItem(..)` constructor, creating a new instance of `ClipboardItem`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ClipboardItem/ClipboardItem)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ClipboardItem`*"]
    pub fn new_with_record_from_str_to_blob_promise(
        items: &::js_sys::Object,
    ) -> Result<ClipboardItem, JsValue>;
    #[cfg(feature = "ClipboardItemOptions")]
    #[wasm_bindgen(catch, constructor, js_class = "ClipboardItem")]
    #[doc = "The `new ClipboardItem(..)` constructor, creating a new instance of `ClipboardItem`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ClipboardItem/ClipboardItem)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ClipboardItem`, `ClipboardItemOptions`*"]
    pub fn new_with_record_from_str_to_str_promise_and_options(
        items: &::js_sys::Object,
        options: &ClipboardItemOptions,
    ) -> Result<ClipboardItem, JsValue>;
    #[cfg(feature = "ClipboardItemOptions")]
    #[wasm_bindgen(catch, constructor, js_class = "ClipboardItem")]
    #[doc = "The `new ClipboardItem(..)` constructor, creating a new instance of `ClipboardItem`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ClipboardItem/ClipboardItem)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ClipboardItem`, `ClipboardItemOptions`*"]
    pub fn new_with_record_from_str_to_blob_promise_and_options(
        items: &::js_sys::Object,
        options: &ClipboardItemOptions,
    ) -> Result<ClipboardItem, JsValue>;
    #[wasm_bindgen(method, js_class = "ClipboardItem", js_name = "getType")]
    #[doc = "The `getType()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ClipboardItem/getType)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ClipboardItem`*"]
    pub fn get_type(this: &ClipboardItem, type_: &str) -> ::js_sys::Promise;
    #[wasm_bindgen(static_method_of = "ClipboardItem", js_class = "ClipboardItem")]
    #[doc = "The `supports()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ClipboardItem/supports_static)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ClipboardItem`*"]
    pub fn supports(type_: &str) -> bool;
}
