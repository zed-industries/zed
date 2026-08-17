#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "CSSStyleDeclaration",
        typescript_type = "CSSStyleDeclaration"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CssStyleDeclaration` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleDeclaration)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssStyleDeclaration`*"]
    pub type CssStyleDeclaration;
    #[wasm_bindgen(method, getter, js_class = "CSSStyleDeclaration", js_name = "cssText")]
    #[doc = "Getter for the `cssText` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleDeclaration/cssText)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssStyleDeclaration`*"]
    pub fn css_text(this: &CssStyleDeclaration) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "CSSStyleDeclaration", js_name = "cssText")]
    #[doc = "Setter for the `cssText` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleDeclaration/cssText)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssStyleDeclaration`*"]
    pub fn set_css_text(this: &CssStyleDeclaration, value: &str);
    #[wasm_bindgen(method, getter, js_class = "CSSStyleDeclaration", js_name = "length")]
    #[doc = "Getter for the `length` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleDeclaration/length)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssStyleDeclaration`*"]
    pub fn length(this: &CssStyleDeclaration) -> u32;
    #[cfg(feature = "CssRule")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "CSSStyleDeclaration",
        js_name = "parentRule"
    )]
    #[doc = "Getter for the `parentRule` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleDeclaration/parentRule)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssRule`, `CssStyleDeclaration`*"]
    pub fn parent_rule(this: &CssStyleDeclaration) -> Option<CssRule>;
    #[wasm_bindgen(
        method,
        js_class = "CSSStyleDeclaration",
        js_name = "getPropertyPriority"
    )]
    #[doc = "The `getPropertyPriority()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleDeclaration/getPropertyPriority)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssStyleDeclaration`*"]
    pub fn get_property_priority(
        this: &CssStyleDeclaration,
        property: &str,
    ) -> ::alloc::string::String;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "CSSStyleDeclaration",
        js_name = "getPropertyValue"
    )]
    #[doc = "The `getPropertyValue()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleDeclaration/getPropertyValue)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssStyleDeclaration`*"]
    pub fn get_property_value(
        this: &CssStyleDeclaration,
        property: &str,
    ) -> Result<::alloc::string::String, JsValue>;
    #[wasm_bindgen(method, js_class = "CSSStyleDeclaration")]
    #[doc = "The `item()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleDeclaration/item)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssStyleDeclaration`*"]
    pub fn item(this: &CssStyleDeclaration, index: u32) -> ::alloc::string::String;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "CSSStyleDeclaration",
        js_name = "removeProperty"
    )]
    #[doc = "The `removeProperty()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleDeclaration/removeProperty)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssStyleDeclaration`*"]
    pub fn remove_property(
        this: &CssStyleDeclaration,
        property: &str,
    ) -> Result<::alloc::string::String, JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "CSSStyleDeclaration",
        js_name = "setProperty"
    )]
    #[doc = "The `setProperty()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleDeclaration/setProperty)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssStyleDeclaration`*"]
    pub fn set_property(
        this: &CssStyleDeclaration,
        property: &str,
        value: &str,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "CSSStyleDeclaration",
        js_name = "setProperty"
    )]
    #[doc = "The `setProperty()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleDeclaration/setProperty)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssStyleDeclaration`*"]
    pub fn set_property_with_priority(
        this: &CssStyleDeclaration,
        property: &str,
        value: &str,
        priority: &str,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(method, js_class = "CSSStyleDeclaration", indexing_getter)]
    #[doc = "Indexing getter. As in the literal Javascript `this[key]`."]
    #[doc = ""]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssStyleDeclaration`*"]
    pub fn get(this: &CssStyleDeclaration, index: u32) -> Option<::alloc::string::String>;
}
