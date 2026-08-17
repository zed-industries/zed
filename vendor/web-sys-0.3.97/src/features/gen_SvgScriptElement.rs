#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "SvgElement",
        extends = "Element",
        extends = "Node",
        extends = "EventTarget",
        extends = "::js_sys::Object",
        js_name = "SVGScriptElement",
        typescript_type = "SVGScriptElement"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgScriptElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGScriptElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgScriptElement`*"]
    pub type SvgScriptElement;
    #[wasm_bindgen(method, getter, js_class = "SVGScriptElement", js_name = "type")]
    #[doc = "Getter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGScriptElement/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgScriptElement`*"]
    pub fn type_(this: &SvgScriptElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "SVGScriptElement", js_name = "type")]
    #[doc = "Setter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGScriptElement/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgScriptElement`*"]
    pub fn set_type(this: &SvgScriptElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "SVGScriptElement", js_name = "crossOrigin")]
    #[doc = "Getter for the `crossOrigin` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGScriptElement/crossOrigin)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgScriptElement`*"]
    pub fn cross_origin(this: &SvgScriptElement) -> Option<::alloc::string::String>;
    #[wasm_bindgen(method, setter, js_class = "SVGScriptElement", js_name = "crossOrigin")]
    #[doc = "Setter for the `crossOrigin` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGScriptElement/crossOrigin)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgScriptElement`*"]
    pub fn set_cross_origin(this: &SvgScriptElement, value: Option<&str>);
    #[cfg(feature = "SvgAnimatedString")]
    #[wasm_bindgen(method, getter, js_class = "SVGScriptElement", js_name = "href")]
    #[doc = "Getter for the `href` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGScriptElement/href)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgScriptElement`*"]
    pub fn href(this: &SvgScriptElement) -> SvgAnimatedString;
}
