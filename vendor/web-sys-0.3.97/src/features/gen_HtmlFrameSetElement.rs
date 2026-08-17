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
        js_name = "HTMLFrameSetElement",
        typescript_type = "HTMLFrameSetElement"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlFrameSetElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFrameSetElement`*"]
    pub type HtmlFrameSetElement;
    #[wasm_bindgen(method, getter, js_class = "HTMLFrameSetElement", js_name = "cols")]
    #[doc = "Getter for the `cols` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/cols)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFrameSetElement`*"]
    pub fn cols(this: &HtmlFrameSetElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLFrameSetElement", js_name = "cols")]
    #[doc = "Setter for the `cols` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/cols)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFrameSetElement`*"]
    pub fn set_cols(this: &HtmlFrameSetElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLFrameSetElement", js_name = "rows")]
    #[doc = "Getter for the `rows` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/rows)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFrameSetElement`*"]
    pub fn rows(this: &HtmlFrameSetElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLFrameSetElement", js_name = "rows")]
    #[doc = "Setter for the `rows` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/rows)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFrameSetElement`*"]
    pub fn set_rows(this: &HtmlFrameSetElement, value: &str);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "HTMLFrameSetElement",
        js_name = "onafterprint"
    )]
    #[doc = "Getter for the `onafterprint` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/onafterprint)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFrameSetElement`*"]
    pub fn onafterprint(this: &HtmlFrameSetElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "HTMLFrameSetElement",
        js_name = "onafterprint"
    )]
    #[doc = "Setter for the `onafterprint` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/onafterprint)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFrameSetElement`*"]
    pub fn set_onafterprint(this: &HtmlFrameSetElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "HTMLFrameSetElement",
        js_name = "onbeforeprint"
    )]
    #[doc = "Getter for the `onbeforeprint` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/onbeforeprint)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFrameSetElement`*"]
    pub fn onbeforeprint(this: &HtmlFrameSetElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "HTMLFrameSetElement",
        js_name = "onbeforeprint"
    )]
    #[doc = "Setter for the `onbeforeprint` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/onbeforeprint)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFrameSetElement`*"]
    pub fn set_onbeforeprint(this: &HtmlFrameSetElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "HTMLFrameSetElement",
        js_name = "onbeforeunload"
    )]
    #[doc = "Getter for the `onbeforeunload` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/onbeforeunload)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFrameSetElement`*"]
    pub fn onbeforeunload(this: &HtmlFrameSetElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "HTMLFrameSetElement",
        js_name = "onbeforeunload"
    )]
    #[doc = "Setter for the `onbeforeunload` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/onbeforeunload)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFrameSetElement`*"]
    pub fn set_onbeforeunload(this: &HtmlFrameSetElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "HTMLFrameSetElement",
        js_name = "onhashchange"
    )]
    #[doc = "Getter for the `onhashchange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/onhashchange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFrameSetElement`*"]
    pub fn onhashchange(this: &HtmlFrameSetElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "HTMLFrameSetElement",
        js_name = "onhashchange"
    )]
    #[doc = "Setter for the `onhashchange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/onhashchange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFrameSetElement`*"]
    pub fn set_onhashchange(this: &HtmlFrameSetElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "HTMLFrameSetElement",
        js_name = "onlanguagechange"
    )]
    #[doc = "Getter for the `onlanguagechange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/onlanguagechange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFrameSetElement`*"]
    pub fn onlanguagechange(this: &HtmlFrameSetElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "HTMLFrameSetElement",
        js_name = "onlanguagechange"
    )]
    #[doc = "Setter for the `onlanguagechange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/onlanguagechange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFrameSetElement`*"]
    pub fn set_onlanguagechange(this: &HtmlFrameSetElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "HTMLFrameSetElement",
        js_name = "onmessage"
    )]
    #[doc = "Getter for the `onmessage` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/onmessage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFrameSetElement`*"]
    pub fn onmessage(this: &HtmlFrameSetElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "HTMLFrameSetElement",
        js_name = "onmessage"
    )]
    #[doc = "Setter for the `onmessage` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/onmessage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFrameSetElement`*"]
    pub fn set_onmessage(this: &HtmlFrameSetElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "HTMLFrameSetElement",
        js_name = "onmessageerror"
    )]
    #[doc = "Getter for the `onmessageerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/onmessageerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFrameSetElement`*"]
    pub fn onmessageerror(this: &HtmlFrameSetElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "HTMLFrameSetElement",
        js_name = "onmessageerror"
    )]
    #[doc = "Setter for the `onmessageerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/onmessageerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFrameSetElement`*"]
    pub fn set_onmessageerror(this: &HtmlFrameSetElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "HTMLFrameSetElement",
        js_name = "onoffline"
    )]
    #[doc = "Getter for the `onoffline` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/onoffline)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFrameSetElement`*"]
    pub fn onoffline(this: &HtmlFrameSetElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "HTMLFrameSetElement",
        js_name = "onoffline"
    )]
    #[doc = "Setter for the `onoffline` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/onoffline)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFrameSetElement`*"]
    pub fn set_onoffline(this: &HtmlFrameSetElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "HTMLFrameSetElement", js_name = "ononline")]
    #[doc = "Getter for the `ononline` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/ononline)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFrameSetElement`*"]
    pub fn ononline(this: &HtmlFrameSetElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "HTMLFrameSetElement", js_name = "ononline")]
    #[doc = "Setter for the `ononline` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/ononline)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFrameSetElement`*"]
    pub fn set_ononline(this: &HtmlFrameSetElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "HTMLFrameSetElement",
        js_name = "onpagehide"
    )]
    #[doc = "Getter for the `onpagehide` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/onpagehide)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFrameSetElement`*"]
    pub fn onpagehide(this: &HtmlFrameSetElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "HTMLFrameSetElement",
        js_name = "onpagehide"
    )]
    #[doc = "Setter for the `onpagehide` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/onpagehide)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFrameSetElement`*"]
    pub fn set_onpagehide(this: &HtmlFrameSetElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "HTMLFrameSetElement",
        js_name = "onpageshow"
    )]
    #[doc = "Getter for the `onpageshow` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/onpageshow)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFrameSetElement`*"]
    pub fn onpageshow(this: &HtmlFrameSetElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "HTMLFrameSetElement",
        js_name = "onpageshow"
    )]
    #[doc = "Setter for the `onpageshow` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/onpageshow)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFrameSetElement`*"]
    pub fn set_onpageshow(this: &HtmlFrameSetElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "HTMLFrameSetElement",
        js_name = "onpopstate"
    )]
    #[doc = "Getter for the `onpopstate` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/onpopstate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFrameSetElement`*"]
    pub fn onpopstate(this: &HtmlFrameSetElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "HTMLFrameSetElement",
        js_name = "onpopstate"
    )]
    #[doc = "Setter for the `onpopstate` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/onpopstate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFrameSetElement`*"]
    pub fn set_onpopstate(this: &HtmlFrameSetElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "HTMLFrameSetElement",
        js_name = "onstorage"
    )]
    #[doc = "Getter for the `onstorage` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/onstorage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFrameSetElement`*"]
    pub fn onstorage(this: &HtmlFrameSetElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "HTMLFrameSetElement",
        js_name = "onstorage"
    )]
    #[doc = "Setter for the `onstorage` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/onstorage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFrameSetElement`*"]
    pub fn set_onstorage(this: &HtmlFrameSetElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "HTMLFrameSetElement", js_name = "onunload")]
    #[doc = "Getter for the `onunload` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/onunload)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFrameSetElement`*"]
    pub fn onunload(this: &HtmlFrameSetElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "HTMLFrameSetElement", js_name = "onunload")]
    #[doc = "Setter for the `onunload` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/onunload)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFrameSetElement`*"]
    pub fn set_onunload(this: &HtmlFrameSetElement, value: Option<&::js_sys::Function>);
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "HTMLFrameSetElement",
        js_name = "ongamepadconnected"
    )]
    #[doc = "Getter for the `ongamepadconnected` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/ongamepadconnected)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFrameSetElement`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn ongamepadconnected(this: &HtmlFrameSetElement) -> Option<::js_sys::Function>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        setter,
        js_class = "HTMLFrameSetElement",
        js_name = "ongamepadconnected"
    )]
    #[doc = "Setter for the `ongamepadconnected` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/ongamepadconnected)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFrameSetElement`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_ongamepadconnected(this: &HtmlFrameSetElement, value: Option<&::js_sys::Function>);
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "HTMLFrameSetElement",
        js_name = "ongamepaddisconnected"
    )]
    #[doc = "Getter for the `ongamepaddisconnected` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/ongamepaddisconnected)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFrameSetElement`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn ongamepaddisconnected(this: &HtmlFrameSetElement) -> Option<::js_sys::Function>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        setter,
        js_class = "HTMLFrameSetElement",
        js_name = "ongamepaddisconnected"
    )]
    #[doc = "Setter for the `ongamepaddisconnected` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/ongamepaddisconnected)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFrameSetElement`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_ongamepaddisconnected(
        this: &HtmlFrameSetElement,
        value: Option<&::js_sys::Function>,
    );
}
