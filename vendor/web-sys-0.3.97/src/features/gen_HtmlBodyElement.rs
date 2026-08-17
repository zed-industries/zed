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
        js_name = "HTMLBodyElement",
        typescript_type = "HTMLBodyElement"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlBodyElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
    pub type HtmlBodyElement;
    #[wasm_bindgen(method, getter, js_class = "HTMLBodyElement", js_name = "text")]
    #[doc = "Getter for the `text` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/text)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
    pub fn text(this: &HtmlBodyElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLBodyElement", js_name = "text")]
    #[doc = "Setter for the `text` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/text)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
    pub fn set_text(this: &HtmlBodyElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLBodyElement", js_name = "link")]
    #[doc = "Getter for the `link` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/link)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
    pub fn link(this: &HtmlBodyElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLBodyElement", js_name = "link")]
    #[doc = "Setter for the `link` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/link)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
    pub fn set_link(this: &HtmlBodyElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLBodyElement", js_name = "vLink")]
    #[doc = "Getter for the `vLink` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/vLink)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
    pub fn v_link(this: &HtmlBodyElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLBodyElement", js_name = "vLink")]
    #[doc = "Setter for the `vLink` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/vLink)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
    pub fn set_v_link(this: &HtmlBodyElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLBodyElement", js_name = "aLink")]
    #[doc = "Getter for the `aLink` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/aLink)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
    pub fn a_link(this: &HtmlBodyElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLBodyElement", js_name = "aLink")]
    #[doc = "Setter for the `aLink` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/aLink)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
    pub fn set_a_link(this: &HtmlBodyElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLBodyElement", js_name = "bgColor")]
    #[doc = "Getter for the `bgColor` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/bgColor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
    pub fn bg_color(this: &HtmlBodyElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLBodyElement", js_name = "bgColor")]
    #[doc = "Setter for the `bgColor` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/bgColor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
    pub fn set_bg_color(this: &HtmlBodyElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLBodyElement", js_name = "background")]
    #[doc = "Getter for the `background` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/background)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
    pub fn background(this: &HtmlBodyElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLBodyElement", js_name = "background")]
    #[doc = "Setter for the `background` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/background)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
    pub fn set_background(this: &HtmlBodyElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLBodyElement", js_name = "onafterprint")]
    #[doc = "Getter for the `onafterprint` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/onafterprint)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
    pub fn onafterprint(this: &HtmlBodyElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "HTMLBodyElement", js_name = "onafterprint")]
    #[doc = "Setter for the `onafterprint` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/onafterprint)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
    pub fn set_onafterprint(this: &HtmlBodyElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "HTMLBodyElement",
        js_name = "onbeforeprint"
    )]
    #[doc = "Getter for the `onbeforeprint` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/onbeforeprint)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
    pub fn onbeforeprint(this: &HtmlBodyElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "HTMLBodyElement",
        js_name = "onbeforeprint"
    )]
    #[doc = "Setter for the `onbeforeprint` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/onbeforeprint)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
    pub fn set_onbeforeprint(this: &HtmlBodyElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "HTMLBodyElement",
        js_name = "onbeforeunload"
    )]
    #[doc = "Getter for the `onbeforeunload` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/onbeforeunload)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
    pub fn onbeforeunload(this: &HtmlBodyElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "HTMLBodyElement",
        js_name = "onbeforeunload"
    )]
    #[doc = "Setter for the `onbeforeunload` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/onbeforeunload)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
    pub fn set_onbeforeunload(this: &HtmlBodyElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "HTMLBodyElement", js_name = "onhashchange")]
    #[doc = "Getter for the `onhashchange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/onhashchange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
    pub fn onhashchange(this: &HtmlBodyElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "HTMLBodyElement", js_name = "onhashchange")]
    #[doc = "Setter for the `onhashchange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/onhashchange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
    pub fn set_onhashchange(this: &HtmlBodyElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "HTMLBodyElement",
        js_name = "onlanguagechange"
    )]
    #[doc = "Getter for the `onlanguagechange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/onlanguagechange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
    pub fn onlanguagechange(this: &HtmlBodyElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "HTMLBodyElement",
        js_name = "onlanguagechange"
    )]
    #[doc = "Setter for the `onlanguagechange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/onlanguagechange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
    pub fn set_onlanguagechange(this: &HtmlBodyElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "HTMLBodyElement", js_name = "onmessage")]
    #[doc = "Getter for the `onmessage` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/onmessage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
    pub fn onmessage(this: &HtmlBodyElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "HTMLBodyElement", js_name = "onmessage")]
    #[doc = "Setter for the `onmessage` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/onmessage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
    pub fn set_onmessage(this: &HtmlBodyElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "HTMLBodyElement",
        js_name = "onmessageerror"
    )]
    #[doc = "Getter for the `onmessageerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/onmessageerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
    pub fn onmessageerror(this: &HtmlBodyElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "HTMLBodyElement",
        js_name = "onmessageerror"
    )]
    #[doc = "Setter for the `onmessageerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/onmessageerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
    pub fn set_onmessageerror(this: &HtmlBodyElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "HTMLBodyElement", js_name = "onoffline")]
    #[doc = "Getter for the `onoffline` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/onoffline)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
    pub fn onoffline(this: &HtmlBodyElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "HTMLBodyElement", js_name = "onoffline")]
    #[doc = "Setter for the `onoffline` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/onoffline)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
    pub fn set_onoffline(this: &HtmlBodyElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "HTMLBodyElement", js_name = "ononline")]
    #[doc = "Getter for the `ononline` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/ononline)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
    pub fn ononline(this: &HtmlBodyElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "HTMLBodyElement", js_name = "ononline")]
    #[doc = "Setter for the `ononline` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/ononline)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
    pub fn set_ononline(this: &HtmlBodyElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "HTMLBodyElement", js_name = "onpagehide")]
    #[doc = "Getter for the `onpagehide` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/onpagehide)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
    pub fn onpagehide(this: &HtmlBodyElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "HTMLBodyElement", js_name = "onpagehide")]
    #[doc = "Setter for the `onpagehide` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/onpagehide)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
    pub fn set_onpagehide(this: &HtmlBodyElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "HTMLBodyElement", js_name = "onpageshow")]
    #[doc = "Getter for the `onpageshow` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/onpageshow)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
    pub fn onpageshow(this: &HtmlBodyElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "HTMLBodyElement", js_name = "onpageshow")]
    #[doc = "Setter for the `onpageshow` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/onpageshow)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
    pub fn set_onpageshow(this: &HtmlBodyElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "HTMLBodyElement", js_name = "onpopstate")]
    #[doc = "Getter for the `onpopstate` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/onpopstate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
    pub fn onpopstate(this: &HtmlBodyElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "HTMLBodyElement", js_name = "onpopstate")]
    #[doc = "Setter for the `onpopstate` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/onpopstate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
    pub fn set_onpopstate(this: &HtmlBodyElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "HTMLBodyElement", js_name = "onstorage")]
    #[doc = "Getter for the `onstorage` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/onstorage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
    pub fn onstorage(this: &HtmlBodyElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "HTMLBodyElement", js_name = "onstorage")]
    #[doc = "Setter for the `onstorage` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/onstorage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
    pub fn set_onstorage(this: &HtmlBodyElement, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "HTMLBodyElement", js_name = "onunload")]
    #[doc = "Getter for the `onunload` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/onunload)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
    pub fn onunload(this: &HtmlBodyElement) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "HTMLBodyElement", js_name = "onunload")]
    #[doc = "Setter for the `onunload` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/onunload)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
    pub fn set_onunload(this: &HtmlBodyElement, value: Option<&::js_sys::Function>);
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "HTMLBodyElement",
        js_name = "ongamepadconnected"
    )]
    #[doc = "Getter for the `ongamepadconnected` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/ongamepadconnected)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn ongamepadconnected(this: &HtmlBodyElement) -> Option<::js_sys::Function>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        setter,
        js_class = "HTMLBodyElement",
        js_name = "ongamepadconnected"
    )]
    #[doc = "Setter for the `ongamepadconnected` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/ongamepadconnected)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_ongamepadconnected(this: &HtmlBodyElement, value: Option<&::js_sys::Function>);
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "HTMLBodyElement",
        js_name = "ongamepaddisconnected"
    )]
    #[doc = "Getter for the `ongamepaddisconnected` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/ongamepaddisconnected)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn ongamepaddisconnected(this: &HtmlBodyElement) -> Option<::js_sys::Function>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        setter,
        js_class = "HTMLBodyElement",
        js_name = "ongamepaddisconnected"
    )]
    #[doc = "Setter for the `ongamepaddisconnected` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/ongamepaddisconnected)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_ongamepaddisconnected(this: &HtmlBodyElement, value: Option<&::js_sys::Function>);
}
