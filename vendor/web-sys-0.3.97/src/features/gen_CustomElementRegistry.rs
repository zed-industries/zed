#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "CustomElementRegistry",
        typescript_type = "CustomElementRegistry"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CustomElementRegistry` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CustomElementRegistry)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CustomElementRegistry`*"]
    pub type CustomElementRegistry;
    #[wasm_bindgen(catch, method, js_class = "CustomElementRegistry")]
    #[doc = "The `define()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CustomElementRegistry/define)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CustomElementRegistry`*"]
    pub fn define(
        this: &CustomElementRegistry,
        name: &str,
        function_constructor: &::js_sys::Function,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "ElementDefinitionOptions")]
    #[wasm_bindgen(catch, method, js_class = "CustomElementRegistry", js_name = "define")]
    #[doc = "The `define()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CustomElementRegistry/define)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CustomElementRegistry`, `ElementDefinitionOptions`*"]
    pub fn define_with_options(
        this: &CustomElementRegistry,
        name: &str,
        function_constructor: &::js_sys::Function,
        options: &ElementDefinitionOptions,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(method, js_class = "CustomElementRegistry")]
    #[doc = "The `get()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CustomElementRegistry/get)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CustomElementRegistry`*"]
    pub fn get(this: &CustomElementRegistry, name: &str) -> ::wasm_bindgen::JsValue;
    #[cfg(feature = "Node")]
    #[wasm_bindgen(method, js_class = "CustomElementRegistry")]
    #[doc = "The `upgrade()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CustomElementRegistry/upgrade)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CustomElementRegistry`, `Node`*"]
    pub fn upgrade(this: &CustomElementRegistry, root: &Node);
    #[wasm_bindgen(
        catch,
        method,
        js_class = "CustomElementRegistry",
        js_name = "whenDefined"
    )]
    #[doc = "The `whenDefined()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CustomElementRegistry/whenDefined)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CustomElementRegistry`*"]
    pub fn when_defined(
        this: &CustomElementRegistry,
        name: &str,
    ) -> Result<::js_sys::Promise, JsValue>;
}
