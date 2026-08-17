#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "MediaKeyStatusMap",
        typescript_type = "MediaKeyStatusMap"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaKeyStatusMap` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeyStatusMap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeyStatusMap`*"]
    pub type MediaKeyStatusMap;
    #[wasm_bindgen(method, getter, js_class = "MediaKeyStatusMap", js_name = "size")]
    #[doc = "Getter for the `size` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeyStatusMap/size)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeyStatusMap`*"]
    pub fn size(this: &MediaKeyStatusMap) -> u32;
    #[wasm_bindgen(catch, method, js_class = "MediaKeyStatusMap", js_name = "forEach")]
    #[doc = "The `forEach()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeyStatusMap/forEach)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeyStatusMap`*"]
    pub fn for_each(this: &MediaKeyStatusMap, callback: &::js_sys::Function)
        -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "MediaKeyStatusMap", js_name = "get")]
    #[doc = "The `get()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeyStatusMap/get)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeyStatusMap`*"]
    pub fn get_with_buffer_source(
        this: &MediaKeyStatusMap,
        key_id: &::js_sys::Object,
    ) -> Result<::wasm_bindgen::JsValue, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "MediaKeyStatusMap", js_name = "get")]
    #[doc = "The `get()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeyStatusMap/get)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeyStatusMap`*"]
    pub fn get_with_u8_array(
        this: &MediaKeyStatusMap,
        key_id: &mut [u8],
    ) -> Result<::wasm_bindgen::JsValue, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "MediaKeyStatusMap", js_name = "get")]
    #[doc = "The `get()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeyStatusMap/get)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeyStatusMap`*"]
    pub fn get_with_js_u8_array(
        this: &MediaKeyStatusMap,
        key_id: &::js_sys::Uint8Array,
    ) -> Result<::wasm_bindgen::JsValue, JsValue>;
    #[wasm_bindgen(method, js_class = "MediaKeyStatusMap", js_name = "has")]
    #[doc = "The `has()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeyStatusMap/has)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeyStatusMap`*"]
    pub fn has_with_buffer_source(this: &MediaKeyStatusMap, key_id: &::js_sys::Object) -> bool;
    #[wasm_bindgen(method, js_class = "MediaKeyStatusMap", js_name = "has")]
    #[doc = "The `has()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeyStatusMap/has)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeyStatusMap`*"]
    pub fn has_with_u8_array(this: &MediaKeyStatusMap, key_id: &mut [u8]) -> bool;
    #[wasm_bindgen(method, js_class = "MediaKeyStatusMap", js_name = "has")]
    #[doc = "The `has()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeyStatusMap/has)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeyStatusMap`*"]
    pub fn has_with_js_u8_array(this: &MediaKeyStatusMap, key_id: &::js_sys::Uint8Array) -> bool;
    #[wasm_bindgen(method, js_class = "MediaKeyStatusMap")]
    #[doc = "The `entries()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeyStatusMap/entries)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeyStatusMap`*"]
    pub fn entries(this: &MediaKeyStatusMap) -> ::js_sys::Iterator;
    #[wasm_bindgen(method, js_class = "MediaKeyStatusMap")]
    #[doc = "The `keys()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeyStatusMap/keys)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeyStatusMap`*"]
    pub fn keys(this: &MediaKeyStatusMap) -> ::js_sys::Iterator;
    #[wasm_bindgen(method, js_class = "MediaKeyStatusMap")]
    #[doc = "The `values()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeyStatusMap/values)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeyStatusMap`*"]
    pub fn values(this: &MediaKeyStatusMap) -> ::js_sys::Iterator;
}
