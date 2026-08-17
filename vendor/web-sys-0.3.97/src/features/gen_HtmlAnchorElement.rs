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
        js_name = "HTMLAnchorElement",
        typescript_type = "HTMLAnchorElement"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlAnchorElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    pub type HtmlAnchorElement;
    #[wasm_bindgen(method, getter, js_class = "HTMLAnchorElement", js_name = "target")]
    #[doc = "Getter for the `target` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/target)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    pub fn target(this: &HtmlAnchorElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLAnchorElement", js_name = "target")]
    #[doc = "Setter for the `target` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/target)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    pub fn set_target(this: &HtmlAnchorElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLAnchorElement", js_name = "download")]
    #[doc = "Getter for the `download` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/download)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    pub fn download(this: &HtmlAnchorElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLAnchorElement", js_name = "download")]
    #[doc = "Setter for the `download` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/download)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    pub fn set_download(this: &HtmlAnchorElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLAnchorElement", js_name = "ping")]
    #[doc = "Getter for the `ping` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/ping)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    pub fn ping(this: &HtmlAnchorElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLAnchorElement", js_name = "ping")]
    #[doc = "Setter for the `ping` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/ping)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    pub fn set_ping(this: &HtmlAnchorElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLAnchorElement", js_name = "rel")]
    #[doc = "Getter for the `rel` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/rel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    pub fn rel(this: &HtmlAnchorElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLAnchorElement", js_name = "rel")]
    #[doc = "Setter for the `rel` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/rel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    pub fn set_rel(this: &HtmlAnchorElement, value: &str);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "HTMLAnchorElement",
        js_name = "referrerPolicy"
    )]
    #[doc = "Getter for the `referrerPolicy` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/referrerPolicy)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    pub fn referrer_policy(this: &HtmlAnchorElement) -> ::alloc::string::String;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "HTMLAnchorElement",
        js_name = "referrerPolicy"
    )]
    #[doc = "Setter for the `referrerPolicy` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/referrerPolicy)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    pub fn set_referrer_policy(this: &HtmlAnchorElement, value: &str);
    #[cfg(feature = "DomTokenList")]
    #[wasm_bindgen(method, getter, js_class = "HTMLAnchorElement", js_name = "relList")]
    #[doc = "Getter for the `relList` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/relList)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomTokenList`, `HtmlAnchorElement`*"]
    pub fn rel_list(this: &HtmlAnchorElement) -> DomTokenList;
    #[wasm_bindgen(method, getter, js_class = "HTMLAnchorElement", js_name = "hreflang")]
    #[doc = "Getter for the `hreflang` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/hreflang)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    pub fn hreflang(this: &HtmlAnchorElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLAnchorElement", js_name = "hreflang")]
    #[doc = "Setter for the `hreflang` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/hreflang)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    pub fn set_hreflang(this: &HtmlAnchorElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLAnchorElement", js_name = "type")]
    #[doc = "Getter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    pub fn type_(this: &HtmlAnchorElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLAnchorElement", js_name = "type")]
    #[doc = "Setter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    pub fn set_type(this: &HtmlAnchorElement, value: &str);
    #[wasm_bindgen(
        catch,
        method,
        getter,
        js_class = "HTMLAnchorElement",
        js_name = "text"
    )]
    #[doc = "Getter for the `text` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/text)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    pub fn text(this: &HtmlAnchorElement) -> Result<::alloc::string::String, JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        setter,
        js_class = "HTMLAnchorElement",
        js_name = "text"
    )]
    #[doc = "Setter for the `text` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/text)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    pub fn set_text(this: &HtmlAnchorElement, value: &str) -> Result<(), JsValue>;
    #[wasm_bindgen(method, getter, js_class = "HTMLAnchorElement", js_name = "coords")]
    #[doc = "Getter for the `coords` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/coords)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    pub fn coords(this: &HtmlAnchorElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLAnchorElement", js_name = "coords")]
    #[doc = "Setter for the `coords` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/coords)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    pub fn set_coords(this: &HtmlAnchorElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLAnchorElement", js_name = "charset")]
    #[doc = "Getter for the `charset` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/charset)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    pub fn charset(this: &HtmlAnchorElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLAnchorElement", js_name = "charset")]
    #[doc = "Setter for the `charset` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/charset)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    pub fn set_charset(this: &HtmlAnchorElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLAnchorElement", js_name = "name")]
    #[doc = "Getter for the `name` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/name)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    pub fn name(this: &HtmlAnchorElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLAnchorElement", js_name = "name")]
    #[doc = "Setter for the `name` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/name)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    pub fn set_name(this: &HtmlAnchorElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLAnchorElement", js_name = "rev")]
    #[doc = "Getter for the `rev` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/rev)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    pub fn rev(this: &HtmlAnchorElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLAnchorElement", js_name = "rev")]
    #[doc = "Setter for the `rev` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/rev)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    pub fn set_rev(this: &HtmlAnchorElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLAnchorElement", js_name = "shape")]
    #[doc = "Getter for the `shape` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/shape)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    pub fn shape(this: &HtmlAnchorElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLAnchorElement", js_name = "shape")]
    #[doc = "Setter for the `shape` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/shape)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    pub fn set_shape(this: &HtmlAnchorElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLAnchorElement", js_name = "href")]
    #[doc = "Getter for the `href` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/href)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    pub fn href(this: &HtmlAnchorElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLAnchorElement", js_name = "href")]
    #[doc = "Setter for the `href` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/href)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    pub fn set_href(this: &HtmlAnchorElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLAnchorElement", js_name = "origin")]
    #[doc = "Getter for the `origin` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/origin)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    pub fn origin(this: &HtmlAnchorElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "HTMLAnchorElement", js_name = "protocol")]
    #[doc = "Getter for the `protocol` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/protocol)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    pub fn protocol(this: &HtmlAnchorElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLAnchorElement", js_name = "protocol")]
    #[doc = "Setter for the `protocol` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/protocol)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    pub fn set_protocol(this: &HtmlAnchorElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLAnchorElement", js_name = "username")]
    #[doc = "Getter for the `username` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/username)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    pub fn username(this: &HtmlAnchorElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLAnchorElement", js_name = "username")]
    #[doc = "Setter for the `username` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/username)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    pub fn set_username(this: &HtmlAnchorElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLAnchorElement", js_name = "password")]
    #[doc = "Getter for the `password` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/password)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    pub fn password(this: &HtmlAnchorElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLAnchorElement", js_name = "password")]
    #[doc = "Setter for the `password` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/password)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    pub fn set_password(this: &HtmlAnchorElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLAnchorElement", js_name = "host")]
    #[doc = "Getter for the `host` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/host)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    pub fn host(this: &HtmlAnchorElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLAnchorElement", js_name = "host")]
    #[doc = "Setter for the `host` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/host)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    pub fn set_host(this: &HtmlAnchorElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLAnchorElement", js_name = "hostname")]
    #[doc = "Getter for the `hostname` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/hostname)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    pub fn hostname(this: &HtmlAnchorElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLAnchorElement", js_name = "hostname")]
    #[doc = "Setter for the `hostname` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/hostname)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    pub fn set_hostname(this: &HtmlAnchorElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLAnchorElement", js_name = "port")]
    #[doc = "Getter for the `port` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/port)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    pub fn port(this: &HtmlAnchorElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLAnchorElement", js_name = "port")]
    #[doc = "Setter for the `port` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/port)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    pub fn set_port(this: &HtmlAnchorElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLAnchorElement", js_name = "pathname")]
    #[doc = "Getter for the `pathname` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/pathname)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    pub fn pathname(this: &HtmlAnchorElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLAnchorElement", js_name = "pathname")]
    #[doc = "Setter for the `pathname` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/pathname)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    pub fn set_pathname(this: &HtmlAnchorElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLAnchorElement", js_name = "search")]
    #[doc = "Getter for the `search` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/search)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    pub fn search(this: &HtmlAnchorElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLAnchorElement", js_name = "search")]
    #[doc = "Setter for the `search` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/search)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    pub fn set_search(this: &HtmlAnchorElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLAnchorElement", js_name = "hash")]
    #[doc = "Getter for the `hash` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/hash)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    pub fn hash(this: &HtmlAnchorElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLAnchorElement", js_name = "hash")]
    #[doc = "Setter for the `hash` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/hash)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    pub fn set_hash(this: &HtmlAnchorElement, value: &str);
}
