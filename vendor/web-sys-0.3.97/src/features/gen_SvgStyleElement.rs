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
        js_name = "SVGStyleElement",
        typescript_type = "SVGStyleElement"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgStyleElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGStyleElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgStyleElement`*"]
    pub type SvgStyleElement;
    #[wasm_bindgen(method, getter, js_class = "SVGStyleElement", js_name = "xmlspace")]
    #[doc = "Getter for the `xmlspace` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGStyleElement/xmlspace)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgStyleElement`*"]
    pub fn xmlspace(this: &SvgStyleElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "SVGStyleElement", js_name = "xmlspace")]
    #[doc = "Setter for the `xmlspace` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGStyleElement/xmlspace)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgStyleElement`*"]
    pub fn set_xmlspace(this: &SvgStyleElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "SVGStyleElement", js_name = "type")]
    #[doc = "Getter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGStyleElement/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgStyleElement`*"]
    pub fn type_(this: &SvgStyleElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "SVGStyleElement", js_name = "type")]
    #[doc = "Setter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGStyleElement/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgStyleElement`*"]
    pub fn set_type(this: &SvgStyleElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "SVGStyleElement", js_name = "media")]
    #[doc = "Getter for the `media` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGStyleElement/media)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgStyleElement`*"]
    pub fn media(this: &SvgStyleElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "SVGStyleElement", js_name = "media")]
    #[doc = "Setter for the `media` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGStyleElement/media)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgStyleElement`*"]
    pub fn set_media(this: &SvgStyleElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "SVGStyleElement", js_name = "title")]
    #[doc = "Getter for the `title` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGStyleElement/title)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgStyleElement`*"]
    pub fn title(this: &SvgStyleElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "SVGStyleElement", js_name = "title")]
    #[doc = "Setter for the `title` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGStyleElement/title)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgStyleElement`*"]
    pub fn set_title(this: &SvgStyleElement, value: &str);
    #[cfg(feature = "StyleSheet")]
    #[wasm_bindgen(method, getter, js_class = "SVGStyleElement", js_name = "sheet")]
    #[doc = "Getter for the `sheet` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGStyleElement/sheet)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StyleSheet`, `SvgStyleElement`*"]
    pub fn sheet(this: &SvgStyleElement) -> Option<StyleSheet>;
}
