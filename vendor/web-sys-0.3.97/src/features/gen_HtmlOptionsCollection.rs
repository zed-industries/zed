#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "HtmlCollection",
        extends = "::js_sys::Object",
        js_name = "HTMLOptionsCollection",
        typescript_type = "HTMLOptionsCollection"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlOptionsCollection` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionsCollection)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlOptionsCollection`*"]
    pub type HtmlOptionsCollection;
    #[wasm_bindgen(method, getter, js_class = "HTMLOptionsCollection", js_name = "length")]
    #[doc = "Getter for the `length` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionsCollection/length)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlOptionsCollection`*"]
    pub fn length(this: &HtmlOptionsCollection) -> u32;
    #[wasm_bindgen(method, setter, js_class = "HTMLOptionsCollection", js_name = "length")]
    #[doc = "Setter for the `length` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionsCollection/length)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlOptionsCollection`*"]
    pub fn set_length(this: &HtmlOptionsCollection, value: u32);
    #[wasm_bindgen(
        catch,
        method,
        getter,
        js_class = "HTMLOptionsCollection",
        js_name = "selectedIndex"
    )]
    #[doc = "Getter for the `selectedIndex` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionsCollection/selectedIndex)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlOptionsCollection`*"]
    pub fn selected_index(this: &HtmlOptionsCollection) -> Result<i32, JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        setter,
        js_class = "HTMLOptionsCollection",
        js_name = "selectedIndex"
    )]
    #[doc = "Setter for the `selectedIndex` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionsCollection/selectedIndex)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlOptionsCollection`*"]
    pub fn set_selected_index(this: &HtmlOptionsCollection, value: i32) -> Result<(), JsValue>;
    #[cfg(feature = "HtmlOptionElement")]
    #[wasm_bindgen(catch, method, js_class = "HTMLOptionsCollection", js_name = "add")]
    #[doc = "The `add()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionsCollection/add)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlOptionElement`, `HtmlOptionsCollection`*"]
    pub fn add_with_html_option_element(
        this: &HtmlOptionsCollection,
        element: &HtmlOptionElement,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "HtmlOptGroupElement")]
    #[wasm_bindgen(catch, method, js_class = "HTMLOptionsCollection", js_name = "add")]
    #[doc = "The `add()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionsCollection/add)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlOptGroupElement`, `HtmlOptionsCollection`*"]
    pub fn add_with_html_opt_group_element(
        this: &HtmlOptionsCollection,
        element: &HtmlOptGroupElement,
    ) -> Result<(), JsValue>;
    #[cfg(all(feature = "HtmlElement", feature = "HtmlOptionElement",))]
    #[wasm_bindgen(catch, method, js_class = "HTMLOptionsCollection", js_name = "add")]
    #[doc = "The `add()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionsCollection/add)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlElement`, `HtmlOptionElement`, `HtmlOptionsCollection`*"]
    pub fn add_with_html_option_element_and_opt_html_element(
        this: &HtmlOptionsCollection,
        element: &HtmlOptionElement,
        before: Option<&HtmlElement>,
    ) -> Result<(), JsValue>;
    #[cfg(all(feature = "HtmlElement", feature = "HtmlOptGroupElement",))]
    #[wasm_bindgen(catch, method, js_class = "HTMLOptionsCollection", js_name = "add")]
    #[doc = "The `add()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionsCollection/add)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlElement`, `HtmlOptGroupElement`, `HtmlOptionsCollection`*"]
    pub fn add_with_html_opt_group_element_and_opt_html_element(
        this: &HtmlOptionsCollection,
        element: &HtmlOptGroupElement,
        before: Option<&HtmlElement>,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "HtmlOptionElement")]
    #[wasm_bindgen(catch, method, js_class = "HTMLOptionsCollection", js_name = "add")]
    #[doc = "The `add()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionsCollection/add)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlOptionElement`, `HtmlOptionsCollection`*"]
    pub fn add_with_html_option_element_and_opt_i32(
        this: &HtmlOptionsCollection,
        element: &HtmlOptionElement,
        before: Option<i32>,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "HtmlOptGroupElement")]
    #[wasm_bindgen(catch, method, js_class = "HTMLOptionsCollection", js_name = "add")]
    #[doc = "The `add()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionsCollection/add)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlOptGroupElement`, `HtmlOptionsCollection`*"]
    pub fn add_with_html_opt_group_element_and_opt_i32(
        this: &HtmlOptionsCollection,
        element: &HtmlOptGroupElement,
        before: Option<i32>,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "HTMLOptionsCollection")]
    #[doc = "The `remove()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionsCollection/remove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlOptionsCollection`*"]
    pub fn remove(this: &HtmlOptionsCollection, index: i32) -> Result<(), JsValue>;
    #[cfg(feature = "HtmlOptionElement")]
    #[wasm_bindgen(catch, method, js_class = "HTMLOptionsCollection", indexing_setter)]
    #[doc = "Indexing setter. As in the literal Javascript `this[key] = value`."]
    #[doc = ""]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlOptionElement`, `HtmlOptionsCollection`*"]
    pub fn set(
        this: &HtmlOptionsCollection,
        index: u32,
        option: Option<&HtmlOptionElement>,
    ) -> Result<(), JsValue>;
}
