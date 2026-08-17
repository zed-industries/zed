#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "SvgGraphicsElement",
        extends = "SvgElement",
        extends = "Element",
        extends = "Node",
        extends = "EventTarget",
        extends = "::js_sys::Object",
        js_name = "SVGAElement",
        typescript_type = "SVGAElement"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgaElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgaElement`*"]
    pub type SvgaElement;
    #[cfg(feature = "SvgAnimatedString")]
    #[wasm_bindgen(method, getter, js_class = "SVGAElement", js_name = "target")]
    #[doc = "Getter for the `target` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement/target)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgaElement`*"]
    pub fn target(this: &SvgaElement) -> SvgAnimatedString;
    #[wasm_bindgen(method, getter, js_class = "SVGAElement", js_name = "download")]
    #[doc = "Getter for the `download` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement/download)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgaElement`*"]
    pub fn download(this: &SvgaElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "SVGAElement", js_name = "download")]
    #[doc = "Setter for the `download` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement/download)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgaElement`*"]
    pub fn set_download(this: &SvgaElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "SVGAElement", js_name = "ping")]
    #[doc = "Getter for the `ping` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement/ping)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgaElement`*"]
    pub fn ping(this: &SvgaElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "SVGAElement", js_name = "ping")]
    #[doc = "Setter for the `ping` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement/ping)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgaElement`*"]
    pub fn set_ping(this: &SvgaElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "SVGAElement", js_name = "rel")]
    #[doc = "Getter for the `rel` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement/rel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgaElement`*"]
    pub fn rel(this: &SvgaElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "SVGAElement", js_name = "rel")]
    #[doc = "Setter for the `rel` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement/rel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgaElement`*"]
    pub fn set_rel(this: &SvgaElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "SVGAElement", js_name = "referrerPolicy")]
    #[doc = "Getter for the `referrerPolicy` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement/referrerPolicy)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgaElement`*"]
    pub fn referrer_policy(this: &SvgaElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "SVGAElement", js_name = "referrerPolicy")]
    #[doc = "Setter for the `referrerPolicy` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement/referrerPolicy)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgaElement`*"]
    pub fn set_referrer_policy(this: &SvgaElement, value: &str);
    #[cfg(feature = "DomTokenList")]
    #[wasm_bindgen(method, getter, js_class = "SVGAElement", js_name = "relList")]
    #[doc = "Getter for the `relList` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement/relList)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomTokenList`, `SvgaElement`*"]
    pub fn rel_list(this: &SvgaElement) -> DomTokenList;
    #[wasm_bindgen(method, getter, js_class = "SVGAElement", js_name = "hreflang")]
    #[doc = "Getter for the `hreflang` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement/hreflang)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgaElement`*"]
    pub fn hreflang(this: &SvgaElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "SVGAElement", js_name = "hreflang")]
    #[doc = "Setter for the `hreflang` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement/hreflang)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgaElement`*"]
    pub fn set_hreflang(this: &SvgaElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "SVGAElement", js_name = "type")]
    #[doc = "Getter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgaElement`*"]
    pub fn type_(this: &SvgaElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "SVGAElement", js_name = "type")]
    #[doc = "Setter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgaElement`*"]
    pub fn set_type(this: &SvgaElement, value: &str);
    #[wasm_bindgen(catch, method, getter, js_class = "SVGAElement", js_name = "text")]
    #[doc = "Getter for the `text` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement/text)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgaElement`*"]
    pub fn text(this: &SvgaElement) -> Result<::alloc::string::String, JsValue>;
    #[wasm_bindgen(catch, method, setter, js_class = "SVGAElement", js_name = "text")]
    #[doc = "Setter for the `text` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement/text)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgaElement`*"]
    pub fn set_text(this: &SvgaElement, value: &str) -> Result<(), JsValue>;
    #[cfg(feature = "SvgAnimatedString")]
    #[wasm_bindgen(method, getter, js_class = "SVGAElement", js_name = "href")]
    #[doc = "Getter for the `href` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement/href)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgaElement`*"]
    pub fn href(this: &SvgaElement) -> SvgAnimatedString;
}
