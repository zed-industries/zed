#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "HtmlElement",
        extends = "Element",
        extends = "Node",
        extends = "EventTarget",
        extends = "::js_sys::Object",
        js_name = "HTMLScriptElement",
        typescript_type = "HTMLScriptElement"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlScriptElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLScriptElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlScriptElement`*"]
    pub type HtmlScriptElement;
    #[wasm_bindgen(method, getter, js_class = "HTMLScriptElement", js_name = "src")]
    #[doc = "Getter for the `src` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLScriptElement/src)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlScriptElement`*"]
    pub fn src(this: &HtmlScriptElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLScriptElement", js_name = "src")]
    #[doc = "Setter for the `src` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLScriptElement/src)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlScriptElement`*"]
    pub fn set_src(this: &HtmlScriptElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLScriptElement", js_name = "type")]
    #[doc = "Getter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLScriptElement/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlScriptElement`*"]
    pub fn type_(this: &HtmlScriptElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLScriptElement", js_name = "type")]
    #[doc = "Setter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLScriptElement/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlScriptElement`*"]
    pub fn set_type(this: &HtmlScriptElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLScriptElement", js_name = "noModule")]
    #[doc = "Getter for the `noModule` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLScriptElement/noModule)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlScriptElement`*"]
    pub fn no_module(this: &HtmlScriptElement) -> bool;
    #[wasm_bindgen(method, setter, js_class = "HTMLScriptElement", js_name = "noModule")]
    #[doc = "Setter for the `noModule` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLScriptElement/noModule)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlScriptElement`*"]
    pub fn set_no_module(this: &HtmlScriptElement, value: bool);
    #[wasm_bindgen(method, getter, js_class = "HTMLScriptElement", js_name = "charset")]
    #[doc = "Getter for the `charset` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLScriptElement/charset)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlScriptElement`*"]
    pub fn charset(this: &HtmlScriptElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLScriptElement", js_name = "charset")]
    #[doc = "Setter for the `charset` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLScriptElement/charset)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlScriptElement`*"]
    pub fn set_charset(this: &HtmlScriptElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLScriptElement", js_name = "async")]
    #[doc = "Getter for the `async` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLScriptElement/async)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlScriptElement`*"]
    pub fn r#async(this: &HtmlScriptElement) -> bool;
    #[wasm_bindgen(method, setter, js_class = "HTMLScriptElement", js_name = "async")]
    #[doc = "Setter for the `async` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLScriptElement/async)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlScriptElement`*"]
    pub fn set_async(this: &HtmlScriptElement, value: bool);
    #[wasm_bindgen(method, getter, js_class = "HTMLScriptElement", js_name = "defer")]
    #[doc = "Getter for the `defer` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLScriptElement/defer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlScriptElement`*"]
    pub fn defer(this: &HtmlScriptElement) -> bool;
    #[wasm_bindgen(method, setter, js_class = "HTMLScriptElement", js_name = "defer")]
    #[doc = "Setter for the `defer` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLScriptElement/defer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlScriptElement`*"]
    pub fn set_defer(this: &HtmlScriptElement, value: bool);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "HTMLScriptElement",
        js_name = "crossOrigin"
    )]
    #[doc = "Getter for the `crossOrigin` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLScriptElement/crossOrigin)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlScriptElement`*"]
    pub fn cross_origin(this: &HtmlScriptElement) -> Option<::alloc::string::String>;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "HTMLScriptElement",
        js_name = "crossOrigin"
    )]
    #[doc = "Setter for the `crossOrigin` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLScriptElement/crossOrigin)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlScriptElement`*"]
    pub fn set_cross_origin(this: &HtmlScriptElement, value: Option<&str>);
    #[wasm_bindgen(
        catch,
        method,
        getter,
        js_class = "HTMLScriptElement",
        js_name = "text"
    )]
    #[doc = "Getter for the `text` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLScriptElement/text)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlScriptElement`*"]
    pub fn text(this: &HtmlScriptElement) -> Result<::alloc::string::String, JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        setter,
        js_class = "HTMLScriptElement",
        js_name = "text"
    )]
    #[doc = "Setter for the `text` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLScriptElement/text)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlScriptElement`*"]
    pub fn set_text(this: &HtmlScriptElement, value: &str) -> Result<(), JsValue>;
    #[wasm_bindgen(method, getter, js_class = "HTMLScriptElement", js_name = "event")]
    #[doc = "Getter for the `event` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLScriptElement/event)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlScriptElement`*"]
    pub fn event(this: &HtmlScriptElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLScriptElement", js_name = "event")]
    #[doc = "Setter for the `event` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLScriptElement/event)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlScriptElement`*"]
    pub fn set_event(this: &HtmlScriptElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLScriptElement", js_name = "htmlFor")]
    #[doc = "Getter for the `htmlFor` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLScriptElement/htmlFor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlScriptElement`*"]
    pub fn html_for(this: &HtmlScriptElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLScriptElement", js_name = "htmlFor")]
    #[doc = "Setter for the `htmlFor` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLScriptElement/htmlFor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlScriptElement`*"]
    pub fn set_html_for(this: &HtmlScriptElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLScriptElement", js_name = "integrity")]
    #[doc = "Getter for the `integrity` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLScriptElement/integrity)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlScriptElement`*"]
    pub fn integrity(this: &HtmlScriptElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLScriptElement", js_name = "integrity")]
    #[doc = "Setter for the `integrity` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLScriptElement/integrity)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlScriptElement`*"]
    pub fn set_integrity(this: &HtmlScriptElement, value: &str);
}
