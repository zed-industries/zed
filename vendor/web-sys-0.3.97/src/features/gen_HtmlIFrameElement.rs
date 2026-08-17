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
        js_name = "HTMLIFrameElement",
        typescript_type = "HTMLIFrameElement"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlIFrameElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlIFrameElement`*"]
    pub type HtmlIFrameElement;
    #[wasm_bindgen(method, getter, js_class = "HTMLIFrameElement", js_name = "src")]
    #[doc = "Getter for the `src` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/src)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlIFrameElement`*"]
    pub fn src(this: &HtmlIFrameElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLIFrameElement", js_name = "src")]
    #[doc = "Setter for the `src` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/src)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlIFrameElement`*"]
    pub fn set_src(this: &HtmlIFrameElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLIFrameElement", js_name = "srcdoc")]
    #[doc = "Getter for the `srcdoc` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/srcdoc)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlIFrameElement`*"]
    pub fn srcdoc(this: &HtmlIFrameElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLIFrameElement", js_name = "srcdoc")]
    #[doc = "Setter for the `srcdoc` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/srcdoc)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlIFrameElement`*"]
    pub fn set_srcdoc(this: &HtmlIFrameElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLIFrameElement", js_name = "name")]
    #[doc = "Getter for the `name` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/name)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlIFrameElement`*"]
    pub fn name(this: &HtmlIFrameElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLIFrameElement", js_name = "name")]
    #[doc = "Setter for the `name` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/name)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlIFrameElement`*"]
    pub fn set_name(this: &HtmlIFrameElement, value: &str);
    #[cfg(feature = "DomTokenList")]
    #[wasm_bindgen(method, getter, js_class = "HTMLIFrameElement", js_name = "sandbox")]
    #[doc = "Getter for the `sandbox` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/sandbox)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomTokenList`, `HtmlIFrameElement`*"]
    pub fn sandbox(this: &HtmlIFrameElement) -> DomTokenList;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "HTMLIFrameElement",
        js_name = "allowFullscreen"
    )]
    #[doc = "Getter for the `allowFullscreen` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/allowFullscreen)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlIFrameElement`*"]
    pub fn allow_fullscreen(this: &HtmlIFrameElement) -> bool;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "HTMLIFrameElement",
        js_name = "allowFullscreen"
    )]
    #[doc = "Setter for the `allowFullscreen` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/allowFullscreen)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlIFrameElement`*"]
    pub fn set_allow_fullscreen(this: &HtmlIFrameElement, value: bool);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "HTMLIFrameElement",
        js_name = "allowPaymentRequest"
    )]
    #[doc = "Getter for the `allowPaymentRequest` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/allowPaymentRequest)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlIFrameElement`*"]
    pub fn allow_payment_request(this: &HtmlIFrameElement) -> bool;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "HTMLIFrameElement",
        js_name = "allowPaymentRequest"
    )]
    #[doc = "Setter for the `allowPaymentRequest` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/allowPaymentRequest)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlIFrameElement`*"]
    pub fn set_allow_payment_request(this: &HtmlIFrameElement, value: bool);
    #[wasm_bindgen(method, getter, js_class = "HTMLIFrameElement", js_name = "width")]
    #[doc = "Getter for the `width` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/width)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlIFrameElement`*"]
    pub fn width(this: &HtmlIFrameElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLIFrameElement", js_name = "width")]
    #[doc = "Setter for the `width` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/width)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlIFrameElement`*"]
    pub fn set_width(this: &HtmlIFrameElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLIFrameElement", js_name = "height")]
    #[doc = "Getter for the `height` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/height)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlIFrameElement`*"]
    pub fn height(this: &HtmlIFrameElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLIFrameElement", js_name = "height")]
    #[doc = "Setter for the `height` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/height)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlIFrameElement`*"]
    pub fn set_height(this: &HtmlIFrameElement, value: &str);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "HTMLIFrameElement",
        js_name = "referrerPolicy"
    )]
    #[doc = "Getter for the `referrerPolicy` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/referrerPolicy)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlIFrameElement`*"]
    pub fn referrer_policy(this: &HtmlIFrameElement) -> ::alloc::string::String;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "HTMLIFrameElement",
        js_name = "referrerPolicy"
    )]
    #[doc = "Setter for the `referrerPolicy` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/referrerPolicy)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlIFrameElement`*"]
    pub fn set_referrer_policy(this: &HtmlIFrameElement, value: &str);
    #[cfg(feature = "Document")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "HTMLIFrameElement",
        js_name = "contentDocument"
    )]
    #[doc = "Getter for the `contentDocument` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/contentDocument)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `HtmlIFrameElement`*"]
    pub fn content_document(this: &HtmlIFrameElement) -> Option<Document>;
    #[cfg(feature = "Window")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "HTMLIFrameElement",
        js_name = "contentWindow"
    )]
    #[doc = "Getter for the `contentWindow` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/contentWindow)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlIFrameElement`, `Window`*"]
    pub fn content_window(this: &HtmlIFrameElement) -> Option<Window>;
    #[wasm_bindgen(method, getter, js_class = "HTMLIFrameElement", js_name = "align")]
    #[doc = "Getter for the `align` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/align)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlIFrameElement`*"]
    pub fn align(this: &HtmlIFrameElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLIFrameElement", js_name = "align")]
    #[doc = "Setter for the `align` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/align)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlIFrameElement`*"]
    pub fn set_align(this: &HtmlIFrameElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLIFrameElement", js_name = "scrolling")]
    #[doc = "Getter for the `scrolling` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/scrolling)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlIFrameElement`*"]
    pub fn scrolling(this: &HtmlIFrameElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLIFrameElement", js_name = "scrolling")]
    #[doc = "Setter for the `scrolling` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/scrolling)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlIFrameElement`*"]
    pub fn set_scrolling(this: &HtmlIFrameElement, value: &str);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "HTMLIFrameElement",
        js_name = "frameBorder"
    )]
    #[doc = "Getter for the `frameBorder` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/frameBorder)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlIFrameElement`*"]
    pub fn frame_border(this: &HtmlIFrameElement) -> ::alloc::string::String;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "HTMLIFrameElement",
        js_name = "frameBorder"
    )]
    #[doc = "Setter for the `frameBorder` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/frameBorder)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlIFrameElement`*"]
    pub fn set_frame_border(this: &HtmlIFrameElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLIFrameElement", js_name = "longDesc")]
    #[doc = "Getter for the `longDesc` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/longDesc)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlIFrameElement`*"]
    pub fn long_desc(this: &HtmlIFrameElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLIFrameElement", js_name = "longDesc")]
    #[doc = "Setter for the `longDesc` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/longDesc)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlIFrameElement`*"]
    pub fn set_long_desc(this: &HtmlIFrameElement, value: &str);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "HTMLIFrameElement",
        js_name = "marginHeight"
    )]
    #[doc = "Getter for the `marginHeight` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/marginHeight)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlIFrameElement`*"]
    pub fn margin_height(this: &HtmlIFrameElement) -> ::alloc::string::String;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "HTMLIFrameElement",
        js_name = "marginHeight"
    )]
    #[doc = "Setter for the `marginHeight` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/marginHeight)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlIFrameElement`*"]
    pub fn set_margin_height(this: &HtmlIFrameElement, value: &str);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "HTMLIFrameElement",
        js_name = "marginWidth"
    )]
    #[doc = "Getter for the `marginWidth` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/marginWidth)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlIFrameElement`*"]
    pub fn margin_width(this: &HtmlIFrameElement) -> ::alloc::string::String;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "HTMLIFrameElement",
        js_name = "marginWidth"
    )]
    #[doc = "Setter for the `marginWidth` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/marginWidth)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlIFrameElement`*"]
    pub fn set_margin_width(this: &HtmlIFrameElement, value: &str);
    #[cfg(feature = "Document")]
    #[wasm_bindgen(method, js_class = "HTMLIFrameElement", js_name = "getSVGDocument")]
    #[doc = "The `getSVGDocument()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/getSVGDocument)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `HtmlIFrameElement`*"]
    pub fn get_svg_document(this: &HtmlIFrameElement) -> Option<Document>;
}
