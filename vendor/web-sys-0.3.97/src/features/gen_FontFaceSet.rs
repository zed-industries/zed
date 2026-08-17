#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "EventTarget",
        extends = "::js_sys::Object",
        js_name = "FontFaceSet",
        typescript_type = "FontFaceSet"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `FontFaceSet` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSet)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceSet`*"]
    pub type FontFaceSet;
    #[wasm_bindgen(method, getter, js_class = "FontFaceSet", js_name = "size")]
    #[doc = "Getter for the `size` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSet/size)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceSet`*"]
    pub fn size(this: &FontFaceSet) -> u32;
    #[wasm_bindgen(method, getter, js_class = "FontFaceSet", js_name = "onloading")]
    #[doc = "Getter for the `onloading` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSet/onloading)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceSet`*"]
    pub fn onloading(this: &FontFaceSet) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "FontFaceSet", js_name = "onloading")]
    #[doc = "Setter for the `onloading` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSet/onloading)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceSet`*"]
    pub fn set_onloading(this: &FontFaceSet, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "FontFaceSet", js_name = "onloadingdone")]
    #[doc = "Getter for the `onloadingdone` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSet/onloadingdone)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceSet`*"]
    pub fn onloadingdone(this: &FontFaceSet) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "FontFaceSet", js_name = "onloadingdone")]
    #[doc = "Setter for the `onloadingdone` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSet/onloadingdone)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceSet`*"]
    pub fn set_onloadingdone(this: &FontFaceSet, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "FontFaceSet", js_name = "onloadingerror")]
    #[doc = "Getter for the `onloadingerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSet/onloadingerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceSet`*"]
    pub fn onloadingerror(this: &FontFaceSet) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "FontFaceSet", js_name = "onloadingerror")]
    #[doc = "Setter for the `onloadingerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSet/onloadingerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceSet`*"]
    pub fn set_onloadingerror(this: &FontFaceSet, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(catch, method, getter, js_class = "FontFaceSet", js_name = "ready")]
    #[doc = "Getter for the `ready` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSet/ready)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceSet`*"]
    pub fn ready(this: &FontFaceSet) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(feature = "FontFaceSetLoadStatus")]
    #[wasm_bindgen(method, getter, js_class = "FontFaceSet", js_name = "status")]
    #[doc = "Getter for the `status` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSet/status)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceSet`, `FontFaceSetLoadStatus`*"]
    pub fn status(this: &FontFaceSet) -> FontFaceSetLoadStatus;
    #[cfg(feature = "FontFace")]
    #[wasm_bindgen(catch, method, js_class = "FontFaceSet")]
    #[doc = "The `add()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSet/add)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFace`, `FontFaceSet`*"]
    pub fn add(this: &FontFaceSet, font: &FontFace) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "FontFaceSet")]
    #[doc = "The `check()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSet/check)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceSet`*"]
    pub fn check(this: &FontFaceSet, font: &str) -> Result<bool, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "FontFaceSet", js_name = "check")]
    #[doc = "The `check()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSet/check)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceSet`*"]
    pub fn check_with_text(this: &FontFaceSet, font: &str, text: &str) -> Result<bool, JsValue>;
    #[wasm_bindgen(method, js_class = "FontFaceSet")]
    #[doc = "The `clear()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSet/clear)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceSet`*"]
    pub fn clear(this: &FontFaceSet);
    #[cfg(feature = "FontFace")]
    #[wasm_bindgen(method, js_class = "FontFaceSet")]
    #[doc = "The `delete()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSet/delete)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFace`, `FontFaceSet`*"]
    pub fn delete(this: &FontFaceSet, font: &FontFace) -> bool;
    #[cfg(feature = "FontFaceSetIterator")]
    #[wasm_bindgen(method, js_class = "FontFaceSet")]
    #[doc = "The `entries()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSet/entries)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceSet`, `FontFaceSetIterator`*"]
    pub fn entries(this: &FontFaceSet) -> FontFaceSetIterator;
    #[wasm_bindgen(catch, method, js_class = "FontFaceSet", js_name = "forEach")]
    #[doc = "The `forEach()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSet/forEach)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceSet`*"]
    pub fn for_each(this: &FontFaceSet, cb: &::js_sys::Function) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "FontFaceSet", js_name = "forEach")]
    #[doc = "The `forEach()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSet/forEach)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceSet`*"]
    pub fn for_each_with_this_arg(
        this: &FontFaceSet,
        cb: &::js_sys::Function,
        this_arg: &::wasm_bindgen::JsValue,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "FontFace")]
    #[wasm_bindgen(method, js_class = "FontFaceSet")]
    #[doc = "The `has()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSet/has)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFace`, `FontFaceSet`*"]
    pub fn has(this: &FontFaceSet, font: &FontFace) -> bool;
    #[wasm_bindgen(method, js_class = "FontFaceSet")]
    #[doc = "The `load()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSet/load)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceSet`*"]
    pub fn load(this: &FontFaceSet, font: &str) -> ::js_sys::Promise;
    #[wasm_bindgen(method, js_class = "FontFaceSet", js_name = "load")]
    #[doc = "The `load()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSet/load)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceSet`*"]
    pub fn load_with_text(this: &FontFaceSet, font: &str, text: &str) -> ::js_sys::Promise;
    #[cfg(feature = "FontFaceSetIterator")]
    #[wasm_bindgen(method, js_class = "FontFaceSet")]
    #[doc = "The `values()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSet/values)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceSet`, `FontFaceSetIterator`*"]
    pub fn values(this: &FontFaceSet) -> FontFaceSetIterator;
}
