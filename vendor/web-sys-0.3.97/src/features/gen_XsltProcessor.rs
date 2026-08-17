#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "XSLTProcessor",
        typescript_type = "XSLTProcessor"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `XsltProcessor` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XSLTProcessor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XsltProcessor`*"]
    pub type XsltProcessor;
    #[wasm_bindgen(catch, constructor, js_class = "XSLTProcessor")]
    #[doc = "The `new XsltProcessor(..)` constructor, creating a new instance of `XsltProcessor`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XSLTProcessor/XSLTProcessor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XsltProcessor`*"]
    pub fn new() -> Result<XsltProcessor, JsValue>;
    #[wasm_bindgen(method, js_class = "XSLTProcessor", js_name = "clearParameters")]
    #[doc = "The `clearParameters()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XSLTProcessor/clearParameters)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XsltProcessor`*"]
    pub fn clear_parameters(this: &XsltProcessor);
    #[cfg(feature = "Node")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "XSLTProcessor",
        js_name = "importStylesheet"
    )]
    #[doc = "The `importStylesheet()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XSLTProcessor/importStylesheet)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`, `XsltProcessor`*"]
    pub fn import_stylesheet(this: &XsltProcessor, style: &Node) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "XSLTProcessor", js_name = "removeParameter")]
    #[doc = "The `removeParameter()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XSLTProcessor/removeParameter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XsltProcessor`*"]
    pub fn remove_parameter(
        this: &XsltProcessor,
        namespace_uri: &str,
        local_name: &str,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(method, js_class = "XSLTProcessor")]
    #[doc = "The `reset()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XSLTProcessor/reset)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XsltProcessor`*"]
    pub fn reset(this: &XsltProcessor);
    #[wasm_bindgen(catch, method, js_class = "XSLTProcessor", js_name = "setParameter")]
    #[doc = "The `setParameter()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XSLTProcessor/setParameter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XsltProcessor`*"]
    pub fn set_parameter(
        this: &XsltProcessor,
        namespace_uri: &str,
        local_name: &str,
        value: &::wasm_bindgen::JsValue,
    ) -> Result<(), JsValue>;
    #[cfg(all(feature = "Document", feature = "Node",))]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "XSLTProcessor",
        js_name = "transformToDocument"
    )]
    #[doc = "The `transformToDocument()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XSLTProcessor/transformToDocument)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `Node`, `XsltProcessor`*"]
    pub fn transform_to_document(this: &XsltProcessor, source: &Node) -> Result<Document, JsValue>;
    #[cfg(all(feature = "Document", feature = "DocumentFragment", feature = "Node",))]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "XSLTProcessor",
        js_name = "transformToFragment"
    )]
    #[doc = "The `transformToFragment()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XSLTProcessor/transformToFragment)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `DocumentFragment`, `Node`, `XsltProcessor`*"]
    pub fn transform_to_fragment(
        this: &XsltProcessor,
        source: &Node,
        output: &Document,
    ) -> Result<DocumentFragment, JsValue>;
}
