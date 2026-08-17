#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "DOMTokenList",
        typescript_type = "DOMTokenList"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DomTokenList` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomTokenList`*"]
    pub type DomTokenList;
    #[wasm_bindgen(method, getter, js_class = "DOMTokenList", js_name = "length")]
    #[doc = "Getter for the `length` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/length)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomTokenList`*"]
    pub fn length(this: &DomTokenList) -> u32;
    #[wasm_bindgen(method, getter, js_class = "DOMTokenList", js_name = "value")]
    #[doc = "Getter for the `value` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/value)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomTokenList`*"]
    pub fn value(this: &DomTokenList) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "DOMTokenList", js_name = "value")]
    #[doc = "Setter for the `value` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/value)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomTokenList`*"]
    pub fn set_value(this: &DomTokenList, value: &str);
    #[wasm_bindgen(catch, method, variadic, js_class = "DOMTokenList")]
    #[doc = "The `add()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/add)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomTokenList`*"]
    pub fn add(this: &DomTokenList, tokens: &::js_sys::Array) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "DOMTokenList", js_name = "add")]
    #[doc = "The `add()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/add)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomTokenList`*"]
    pub fn add_0(this: &DomTokenList) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "DOMTokenList", js_name = "add")]
    #[doc = "The `add()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/add)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomTokenList`*"]
    pub fn add_1(this: &DomTokenList, tokens_1: &str) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "DOMTokenList", js_name = "add")]
    #[doc = "The `add()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/add)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomTokenList`*"]
    pub fn add_2(this: &DomTokenList, tokens_1: &str, tokens_2: &str) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "DOMTokenList", js_name = "add")]
    #[doc = "The `add()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/add)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomTokenList`*"]
    pub fn add_3(
        this: &DomTokenList,
        tokens_1: &str,
        tokens_2: &str,
        tokens_3: &str,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "DOMTokenList", js_name = "add")]
    #[doc = "The `add()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/add)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomTokenList`*"]
    pub fn add_4(
        this: &DomTokenList,
        tokens_1: &str,
        tokens_2: &str,
        tokens_3: &str,
        tokens_4: &str,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "DOMTokenList", js_name = "add")]
    #[doc = "The `add()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/add)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomTokenList`*"]
    pub fn add_5(
        this: &DomTokenList,
        tokens_1: &str,
        tokens_2: &str,
        tokens_3: &str,
        tokens_4: &str,
        tokens_5: &str,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "DOMTokenList", js_name = "add")]
    #[doc = "The `add()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/add)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomTokenList`*"]
    pub fn add_6(
        this: &DomTokenList,
        tokens_1: &str,
        tokens_2: &str,
        tokens_3: &str,
        tokens_4: &str,
        tokens_5: &str,
        tokens_6: &str,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "DOMTokenList", js_name = "add")]
    #[doc = "The `add()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/add)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomTokenList`*"]
    pub fn add_7(
        this: &DomTokenList,
        tokens_1: &str,
        tokens_2: &str,
        tokens_3: &str,
        tokens_4: &str,
        tokens_5: &str,
        tokens_6: &str,
        tokens_7: &str,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(method, js_class = "DOMTokenList")]
    #[doc = "The `contains()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/contains)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomTokenList`*"]
    pub fn contains(this: &DomTokenList, token: &str) -> bool;
    #[wasm_bindgen(catch, method, js_class = "DOMTokenList", js_name = "forEach")]
    #[doc = "The `forEach()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/forEach)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomTokenList`*"]
    pub fn for_each(this: &DomTokenList, callback: &::js_sys::Function) -> Result<(), JsValue>;
    #[wasm_bindgen(method, js_class = "DOMTokenList")]
    #[doc = "The `item()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/item)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomTokenList`*"]
    pub fn item(this: &DomTokenList, index: u32) -> Option<::alloc::string::String>;
    #[wasm_bindgen(catch, method, variadic, js_class = "DOMTokenList")]
    #[doc = "The `remove()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/remove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomTokenList`*"]
    pub fn remove(this: &DomTokenList, tokens: &::js_sys::Array) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "DOMTokenList", js_name = "remove")]
    #[doc = "The `remove()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/remove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomTokenList`*"]
    pub fn remove_0(this: &DomTokenList) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "DOMTokenList", js_name = "remove")]
    #[doc = "The `remove()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/remove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomTokenList`*"]
    pub fn remove_1(this: &DomTokenList, tokens_1: &str) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "DOMTokenList", js_name = "remove")]
    #[doc = "The `remove()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/remove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomTokenList`*"]
    pub fn remove_2(this: &DomTokenList, tokens_1: &str, tokens_2: &str) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "DOMTokenList", js_name = "remove")]
    #[doc = "The `remove()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/remove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomTokenList`*"]
    pub fn remove_3(
        this: &DomTokenList,
        tokens_1: &str,
        tokens_2: &str,
        tokens_3: &str,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "DOMTokenList", js_name = "remove")]
    #[doc = "The `remove()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/remove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomTokenList`*"]
    pub fn remove_4(
        this: &DomTokenList,
        tokens_1: &str,
        tokens_2: &str,
        tokens_3: &str,
        tokens_4: &str,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "DOMTokenList", js_name = "remove")]
    #[doc = "The `remove()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/remove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomTokenList`*"]
    pub fn remove_5(
        this: &DomTokenList,
        tokens_1: &str,
        tokens_2: &str,
        tokens_3: &str,
        tokens_4: &str,
        tokens_5: &str,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "DOMTokenList", js_name = "remove")]
    #[doc = "The `remove()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/remove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomTokenList`*"]
    pub fn remove_6(
        this: &DomTokenList,
        tokens_1: &str,
        tokens_2: &str,
        tokens_3: &str,
        tokens_4: &str,
        tokens_5: &str,
        tokens_6: &str,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "DOMTokenList", js_name = "remove")]
    #[doc = "The `remove()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/remove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomTokenList`*"]
    pub fn remove_7(
        this: &DomTokenList,
        tokens_1: &str,
        tokens_2: &str,
        tokens_3: &str,
        tokens_4: &str,
        tokens_5: &str,
        tokens_6: &str,
        tokens_7: &str,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "DOMTokenList")]
    #[doc = "The `replace()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/replace)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomTokenList`*"]
    pub fn replace(this: &DomTokenList, token: &str, new_token: &str) -> Result<bool, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "DOMTokenList")]
    #[doc = "The `supports()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/supports)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomTokenList`*"]
    pub fn supports(this: &DomTokenList, token: &str) -> Result<bool, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "DOMTokenList")]
    #[doc = "The `toggle()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/toggle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomTokenList`*"]
    pub fn toggle(this: &DomTokenList, token: &str) -> Result<bool, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "DOMTokenList", js_name = "toggle")]
    #[doc = "The `toggle()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/toggle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomTokenList`*"]
    pub fn toggle_with_force(
        this: &DomTokenList,
        token: &str,
        force: bool,
    ) -> Result<bool, JsValue>;
    #[wasm_bindgen(method, js_class = "DOMTokenList", indexing_getter)]
    #[doc = "Indexing getter. As in the literal Javascript `this[key]`."]
    #[doc = ""]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomTokenList`*"]
    pub fn get(this: &DomTokenList, index: u32) -> Option<::alloc::string::String>;
    #[wasm_bindgen(method, js_class = "DOMTokenList")]
    #[doc = "The `entries()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/entries)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomTokenList`*"]
    pub fn entries(this: &DomTokenList) -> ::js_sys::Iterator;
    #[wasm_bindgen(method, js_class = "DOMTokenList")]
    #[doc = "The `keys()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/keys)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomTokenList`*"]
    pub fn keys(this: &DomTokenList) -> ::js_sys::Iterator;
    #[wasm_bindgen(method, js_class = "DOMTokenList")]
    #[doc = "The `values()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList/values)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomTokenList`*"]
    pub fn values(this: &DomTokenList) -> ::js_sys::Iterator;
}
