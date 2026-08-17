#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "DOMImplementation",
        typescript_type = "DOMImplementation"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DomImplementation` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMImplementation)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomImplementation`*"]
    pub type DomImplementation;
    #[cfg(feature = "Document")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "DOMImplementation",
        js_name = "createDocument"
    )]
    #[doc = "The `createDocument()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMImplementation/createDocument)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `DomImplementation`*"]
    pub fn create_document(
        this: &DomImplementation,
        namespace: Option<&str>,
        qualified_name: &str,
    ) -> Result<Document, JsValue>;
    #[cfg(all(feature = "Document", feature = "DocumentType",))]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "DOMImplementation",
        js_name = "createDocument"
    )]
    #[doc = "The `createDocument()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMImplementation/createDocument)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `DocumentType`, `DomImplementation`*"]
    pub fn create_document_with_doctype(
        this: &DomImplementation,
        namespace: Option<&str>,
        qualified_name: &str,
        doctype: Option<&DocumentType>,
    ) -> Result<Document, JsValue>;
    #[cfg(feature = "DocumentType")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "DOMImplementation",
        js_name = "createDocumentType"
    )]
    #[doc = "The `createDocumentType()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMImplementation/createDocumentType)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentType`, `DomImplementation`*"]
    pub fn create_document_type(
        this: &DomImplementation,
        qualified_name: &str,
        public_id: &str,
        system_id: &str,
    ) -> Result<DocumentType, JsValue>;
    #[cfg(feature = "Document")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "DOMImplementation",
        js_name = "createHTMLDocument"
    )]
    #[doc = "The `createHTMLDocument()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMImplementation/createHTMLDocument)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `DomImplementation`*"]
    pub fn create_html_document(this: &DomImplementation) -> Result<Document, JsValue>;
    #[cfg(feature = "Document")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "DOMImplementation",
        js_name = "createHTMLDocument"
    )]
    #[doc = "The `createHTMLDocument()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMImplementation/createHTMLDocument)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `DomImplementation`*"]
    pub fn create_html_document_with_title(
        this: &DomImplementation,
        title: &str,
    ) -> Result<Document, JsValue>;
    #[wasm_bindgen(method, js_class = "DOMImplementation", js_name = "hasFeature")]
    #[doc = "The `hasFeature()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMImplementation/hasFeature)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomImplementation`*"]
    pub fn has_feature(this: &DomImplementation) -> bool;
}
