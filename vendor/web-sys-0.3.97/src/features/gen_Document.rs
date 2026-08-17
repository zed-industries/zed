#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "Node",
        extends = "EventTarget",
        extends = "::js_sys::Object",
        js_name = "Document",
        typescript_type = "Document"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Document` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub type Document;
    #[cfg(feature = "DomImplementation")]
    #[wasm_bindgen(
        catch,
        method,
        getter,
        js_class = "Document",
        js_name = "implementation"
    )]
    #[doc = "Getter for the `implementation` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/implementation)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `DomImplementation`*"]
    pub fn implementation(this: &Document) -> Result<DomImplementation, JsValue>;
    #[wasm_bindgen(catch, method, getter, js_class = "Document", js_name = "URL")]
    #[doc = "Getter for the `URL` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/URL)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn url(this: &Document) -> Result<::alloc::string::String, JsValue>;
    #[wasm_bindgen(catch, method, getter, js_class = "Document", js_name = "documentURI")]
    #[doc = "Getter for the `documentURI` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/documentURI)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn document_uri(this: &Document) -> Result<::alloc::string::String, JsValue>;
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "compatMode")]
    #[doc = "Getter for the `compatMode` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/compatMode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn compat_mode(this: &Document) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "characterSet")]
    #[doc = "Getter for the `characterSet` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/characterSet)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn character_set(this: &Document) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "charset")]
    #[doc = "Getter for the `charset` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/charset)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn charset(this: &Document) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "inputEncoding")]
    #[doc = "Getter for the `inputEncoding` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/inputEncoding)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn input_encoding(this: &Document) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "contentType")]
    #[doc = "Getter for the `contentType` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/contentType)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn content_type(this: &Document) -> ::alloc::string::String;
    #[cfg(feature = "DocumentType")]
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "doctype")]
    #[doc = "Getter for the `doctype` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/doctype)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `DocumentType`*"]
    pub fn doctype(this: &Document) -> Option<DocumentType>;
    #[cfg(feature = "Element")]
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "documentElement")]
    #[doc = "Getter for the `documentElement` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/documentElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `Element`*"]
    pub fn document_element(this: &Document) -> Option<Element>;
    #[cfg(feature = "Location")]
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "location")]
    #[doc = "Getter for the `location` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/location)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `Location`*"]
    pub fn location(this: &Document) -> Option<Location>;
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "referrer")]
    #[doc = "Getter for the `referrer` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/referrer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn referrer(this: &Document) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "lastModified")]
    #[doc = "Getter for the `lastModified` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/lastModified)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn last_modified(this: &Document) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "readyState")]
    #[doc = "Getter for the `readyState` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/readyState)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn ready_state(this: &Document) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "title")]
    #[doc = "Getter for the `title` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/title)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn title(this: &Document) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "Document", js_name = "title")]
    #[doc = "Setter for the `title` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/title)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn set_title(this: &Document, value: &str);
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "dir")]
    #[doc = "Getter for the `dir` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/dir)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn dir(this: &Document) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "Document", js_name = "dir")]
    #[doc = "Setter for the `dir` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/dir)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn set_dir(this: &Document, value: &str);
    #[cfg(feature = "HtmlElement")]
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "body")]
    #[doc = "Getter for the `body` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/body)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `HtmlElement`*"]
    pub fn body(this: &Document) -> Option<HtmlElement>;
    #[cfg(feature = "HtmlElement")]
    #[wasm_bindgen(method, setter, js_class = "Document", js_name = "body")]
    #[doc = "Setter for the `body` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/body)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `HtmlElement`*"]
    pub fn set_body(this: &Document, value: Option<&HtmlElement>);
    #[cfg(feature = "HtmlHeadElement")]
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "head")]
    #[doc = "Getter for the `head` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/head)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `HtmlHeadElement`*"]
    pub fn head(this: &Document) -> Option<HtmlHeadElement>;
    #[cfg(feature = "HtmlCollection")]
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "images")]
    #[doc = "Getter for the `images` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/images)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `HtmlCollection`*"]
    pub fn images(this: &Document) -> HtmlCollection;
    #[cfg(feature = "HtmlCollection")]
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "embeds")]
    #[doc = "Getter for the `embeds` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/embeds)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `HtmlCollection`*"]
    pub fn embeds(this: &Document) -> HtmlCollection;
    #[cfg(feature = "HtmlCollection")]
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "plugins")]
    #[doc = "Getter for the `plugins` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/plugins)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `HtmlCollection`*"]
    pub fn plugins(this: &Document) -> HtmlCollection;
    #[cfg(feature = "HtmlCollection")]
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "links")]
    #[doc = "Getter for the `links` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/links)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `HtmlCollection`*"]
    pub fn links(this: &Document) -> HtmlCollection;
    #[cfg(feature = "HtmlCollection")]
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "forms")]
    #[doc = "Getter for the `forms` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/forms)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `HtmlCollection`*"]
    pub fn forms(this: &Document) -> HtmlCollection;
    #[cfg(feature = "HtmlCollection")]
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "scripts")]
    #[doc = "Getter for the `scripts` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/scripts)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `HtmlCollection`*"]
    pub fn scripts(this: &Document) -> HtmlCollection;
    #[cfg(feature = "Window")]
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "defaultView")]
    #[doc = "Getter for the `defaultView` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/defaultView)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `Window`*"]
    pub fn default_view(this: &Document) -> Option<Window>;
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "onreadystatechange")]
    #[doc = "Getter for the `onreadystatechange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onreadystatechange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn onreadystatechange(this: &Document) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Document", js_name = "onreadystatechange")]
    #[doc = "Setter for the `onreadystatechange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onreadystatechange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn set_onreadystatechange(this: &Document, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "Document",
        js_name = "onbeforescriptexecute"
    )]
    #[doc = "Getter for the `onbeforescriptexecute` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onbeforescriptexecute)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn onbeforescriptexecute(this: &Document) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "Document",
        js_name = "onbeforescriptexecute"
    )]
    #[doc = "Setter for the `onbeforescriptexecute` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onbeforescriptexecute)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn set_onbeforescriptexecute(this: &Document, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "Document",
        js_name = "onafterscriptexecute"
    )]
    #[doc = "Getter for the `onafterscriptexecute` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onafterscriptexecute)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn onafterscriptexecute(this: &Document) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "Document",
        js_name = "onafterscriptexecute"
    )]
    #[doc = "Setter for the `onafterscriptexecute` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onafterscriptexecute)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn set_onafterscriptexecute(this: &Document, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "onselectionchange")]
    #[doc = "Getter for the `onselectionchange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onselectionchange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn onselectionchange(this: &Document) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Document", js_name = "onselectionchange")]
    #[doc = "Setter for the `onselectionchange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onselectionchange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn set_onselectionchange(this: &Document, value: Option<&::js_sys::Function>);
    #[cfg(feature = "Element")]
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "currentScript")]
    #[doc = "Getter for the `currentScript` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/currentScript)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `Element`*"]
    pub fn current_script(this: &Document) -> Option<Element>;
    #[cfg(feature = "HtmlCollection")]
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "anchors")]
    #[doc = "Getter for the `anchors` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/anchors)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `HtmlCollection`*"]
    pub fn anchors(this: &Document) -> HtmlCollection;
    #[cfg(feature = "HtmlCollection")]
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "applets")]
    #[doc = "Getter for the `applets` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/applets)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `HtmlCollection`*"]
    pub fn applets(this: &Document) -> HtmlCollection;
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "fullscreen")]
    #[doc = "Getter for the `fullscreen` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/fullscreen)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn fullscreen(this: &Document) -> bool;
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "fullscreenEnabled")]
    #[doc = "Getter for the `fullscreenEnabled` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/fullscreenEnabled)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn fullscreen_enabled(this: &Document) -> bool;
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "onfullscreenchange")]
    #[doc = "Getter for the `onfullscreenchange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onfullscreenchange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn onfullscreenchange(this: &Document) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Document", js_name = "onfullscreenchange")]
    #[doc = "Setter for the `onfullscreenchange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onfullscreenchange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn set_onfullscreenchange(this: &Document, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "onfullscreenerror")]
    #[doc = "Getter for the `onfullscreenerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onfullscreenerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn onfullscreenerror(this: &Document) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Document", js_name = "onfullscreenerror")]
    #[doc = "Setter for the `onfullscreenerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onfullscreenerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn set_onfullscreenerror(this: &Document, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "onpointerlockchange")]
    #[doc = "Getter for the `onpointerlockchange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onpointerlockchange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn onpointerlockchange(this: &Document) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Document", js_name = "onpointerlockchange")]
    #[doc = "Setter for the `onpointerlockchange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onpointerlockchange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn set_onpointerlockchange(this: &Document, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "onpointerlockerror")]
    #[doc = "Getter for the `onpointerlockerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onpointerlockerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn onpointerlockerror(this: &Document) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Document", js_name = "onpointerlockerror")]
    #[doc = "Setter for the `onpointerlockerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onpointerlockerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn set_onpointerlockerror(this: &Document, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "hidden")]
    #[doc = "Getter for the `hidden` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/hidden)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn hidden(this: &Document) -> bool;
    #[cfg(feature = "VisibilityState")]
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "visibilityState")]
    #[doc = "Getter for the `visibilityState` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/visibilityState)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `VisibilityState`*"]
    pub fn visibility_state(this: &Document) -> VisibilityState;
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "onvisibilitychange")]
    #[doc = "Getter for the `onvisibilitychange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onvisibilitychange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn onvisibilitychange(this: &Document) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Document", js_name = "onvisibilitychange")]
    #[doc = "Setter for the `onvisibilitychange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onvisibilitychange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn set_onvisibilitychange(this: &Document, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "Document",
        js_name = "selectedStyleSheetSet"
    )]
    #[doc = "Getter for the `selectedStyleSheetSet` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/selectedStyleSheetSet)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn selected_style_sheet_set(this: &Document) -> Option<::alloc::string::String>;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "Document",
        js_name = "selectedStyleSheetSet"
    )]
    #[doc = "Setter for the `selectedStyleSheetSet` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/selectedStyleSheetSet)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn set_selected_style_sheet_set(this: &Document, value: Option<&str>);
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "lastStyleSheetSet")]
    #[doc = "Getter for the `lastStyleSheetSet` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/lastStyleSheetSet)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn last_style_sheet_set(this: &Document) -> Option<::alloc::string::String>;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "Document",
        js_name = "preferredStyleSheetSet"
    )]
    #[doc = "Getter for the `preferredStyleSheetSet` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/preferredStyleSheetSet)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn preferred_style_sheet_set(this: &Document) -> Option<::alloc::string::String>;
    #[cfg(feature = "DomStringList")]
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "styleSheetSets")]
    #[doc = "Getter for the `styleSheetSets` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/styleSheetSets)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `DomStringList`*"]
    pub fn style_sheet_sets(this: &Document) -> DomStringList;
    #[cfg(feature = "Element")]
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "scrollingElement")]
    #[doc = "Getter for the `scrollingElement` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/scrollingElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `Element`*"]
    pub fn scrolling_element(this: &Document) -> Option<Element>;
    #[cfg(feature = "DocumentTimeline")]
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "timeline")]
    #[doc = "Getter for the `timeline` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/timeline)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `DocumentTimeline`*"]
    pub fn timeline(this: &Document) -> DocumentTimeline;
    #[cfg(feature = "SvgsvgElement")]
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "rootElement")]
    #[doc = "Getter for the `rootElement` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/rootElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `SvgsvgElement`*"]
    pub fn root_element(this: &Document) -> Option<SvgsvgElement>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "Document",
        js_name = "pictureInPictureEnabled"
    )]
    #[doc = "Getter for the `pictureInPictureEnabled` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/pictureInPictureEnabled)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn picture_in_picture_enabled(this: &Document) -> bool;
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "oncopy")]
    #[doc = "Getter for the `oncopy` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/oncopy)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn oncopy(this: &Document) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Document", js_name = "oncopy")]
    #[doc = "Setter for the `oncopy` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/oncopy)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn set_oncopy(this: &Document, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "oncut")]
    #[doc = "Getter for the `oncut` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/oncut)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn oncut(this: &Document) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Document", js_name = "oncut")]
    #[doc = "Setter for the `oncut` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/oncut)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn set_oncut(this: &Document, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "onpaste")]
    #[doc = "Getter for the `onpaste` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onpaste)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn onpaste(this: &Document) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Document", js_name = "onpaste")]
    #[doc = "Setter for the `onpaste` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onpaste)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn set_onpaste(this: &Document, value: Option<&::js_sys::Function>);
    #[cfg(feature = "Element")]
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "activeElement")]
    #[doc = "Getter for the `activeElement` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/activeElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `Element`*"]
    pub fn active_element(this: &Document) -> Option<Element>;
    #[cfg(feature = "Element")]
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "pointerLockElement")]
    #[doc = "Getter for the `pointerLockElement` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/pointerLockElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `Element`*"]
    pub fn pointer_lock_element(this: &Document) -> Option<Element>;
    #[cfg(feature = "Element")]
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "fullscreenElement")]
    #[doc = "Getter for the `fullscreenElement` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/fullscreenElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `Element`*"]
    pub fn fullscreen_element(this: &Document) -> Option<Element>;
    #[cfg(feature = "StyleSheetList")]
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "styleSheets")]
    #[doc = "Getter for the `styleSheets` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/styleSheets)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `StyleSheetList`*"]
    pub fn style_sheets(this: &Document) -> StyleSheetList;
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "adoptedStyleSheets")]
    #[doc = "Getter for the `adoptedStyleSheets` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/adoptedStyleSheets)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn adopted_style_sheets(this: &Document) -> ::js_sys::Array;
    #[wasm_bindgen(method, setter, js_class = "Document", js_name = "adoptedStyleSheets")]
    #[doc = "Setter for the `adoptedStyleSheets` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/adoptedStyleSheets)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn set_adopted_style_sheets(this: &Document, value: &::wasm_bindgen::JsValue);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "Element")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "Document",
        js_name = "pictureInPictureElement"
    )]
    #[doc = "Getter for the `pictureInPictureElement` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/pictureInPictureElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `Element`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn picture_in_picture_element(this: &Document) -> Option<Element>;
    #[cfg(feature = "FontFaceSet")]
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "fonts")]
    #[doc = "Getter for the `fonts` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/fonts)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `FontFaceSet`*"]
    pub fn fonts(this: &Document) -> FontFaceSet;
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "onabort")]
    #[doc = "Getter for the `onabort` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onabort)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn onabort(this: &Document) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Document", js_name = "onabort")]
    #[doc = "Setter for the `onabort` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onabort)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn set_onabort(this: &Document, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "onblur")]
    #[doc = "Getter for the `onblur` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onblur)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn onblur(this: &Document) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Document", js_name = "onblur")]
    #[doc = "Setter for the `onblur` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onblur)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn set_onblur(this: &Document, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "onfocus")]
    #[doc = "Getter for the `onfocus` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onfocus)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn onfocus(this: &Document) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Document", js_name = "onfocus")]
    #[doc = "Setter for the `onfocus` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onfocus)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn set_onfocus(this: &Document, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "oncancel")]
    #[doc = "Getter for the `oncancel` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/oncancel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn oncancel(this: &Document) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Document", js_name = "oncancel")]
    #[doc = "Setter for the `oncancel` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/oncancel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn set_oncancel(this: &Document, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "onauxclick")]
    #[doc = "Getter for the `onauxclick` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onauxclick)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn onauxclick(this: &Document) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Document", js_name = "onauxclick")]
    #[doc = "Setter for the `onauxclick` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onauxclick)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn set_onauxclick(this: &Document, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "onbeforetoggle")]
    #[doc = "Getter for the `onbeforetoggle` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onbeforetoggle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn onbeforetoggle(this: &Document) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Document", js_name = "onbeforetoggle")]
    #[doc = "Setter for the `onbeforetoggle` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onbeforetoggle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn set_onbeforetoggle(this: &Document, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "oncanplay")]
    #[doc = "Getter for the `oncanplay` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/oncanplay)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn oncanplay(this: &Document) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Document", js_name = "oncanplay")]
    #[doc = "Setter for the `oncanplay` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/oncanplay)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn set_oncanplay(this: &Document, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "oncanplaythrough")]
    #[doc = "Getter for the `oncanplaythrough` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/oncanplaythrough)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn oncanplaythrough(this: &Document) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Document", js_name = "oncanplaythrough")]
    #[doc = "Setter for the `oncanplaythrough` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/oncanplaythrough)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn set_oncanplaythrough(this: &Document, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "onchange")]
    #[doc = "Getter for the `onchange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onchange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn onchange(this: &Document) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Document", js_name = "onchange")]
    #[doc = "Setter for the `onchange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onchange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn set_onchange(this: &Document, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "onclick")]
    #[doc = "Getter for the `onclick` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onclick)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn onclick(this: &Document) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Document", js_name = "onclick")]
    #[doc = "Setter for the `onclick` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onclick)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn set_onclick(this: &Document, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "onclose")]
    #[doc = "Getter for the `onclose` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onclose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn onclose(this: &Document) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Document", js_name = "onclose")]
    #[doc = "Setter for the `onclose` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onclose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn set_onclose(this: &Document, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "oncontextmenu")]
    #[doc = "Getter for the `oncontextmenu` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/oncontextmenu)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn oncontextmenu(this: &Document) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Document", js_name = "oncontextmenu")]
    #[doc = "Setter for the `oncontextmenu` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/oncontextmenu)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn set_oncontextmenu(this: &Document, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "ondblclick")]
    #[doc = "Getter for the `ondblclick` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ondblclick)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn ondblclick(this: &Document) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Document", js_name = "ondblclick")]
    #[doc = "Setter for the `ondblclick` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ondblclick)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn set_ondblclick(this: &Document, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "ondrag")]
    #[doc = "Getter for the `ondrag` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ondrag)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn ondrag(this: &Document) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Document", js_name = "ondrag")]
    #[doc = "Setter for the `ondrag` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ondrag)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn set_ondrag(this: &Document, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "ondragend")]
    #[doc = "Getter for the `ondragend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ondragend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn ondragend(this: &Document) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Document", js_name = "ondragend")]
    #[doc = "Setter for the `ondragend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ondragend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn set_ondragend(this: &Document, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "ondragenter")]
    #[doc = "Getter for the `ondragenter` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ondragenter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn ondragenter(this: &Document) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Document", js_name = "ondragenter")]
    #[doc = "Setter for the `ondragenter` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ondragenter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn set_ondragenter(this: &Document, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "ondragexit")]
    #[doc = "Getter for the `ondragexit` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ondragexit)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn ondragexit(this: &Document) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Document", js_name = "ondragexit")]
    #[doc = "Setter for the `ondragexit` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ondragexit)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn set_ondragexit(this: &Document, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "ondragleave")]
    #[doc = "Getter for the `ondragleave` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ondragleave)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn ondragleave(this: &Document) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Document", js_name = "ondragleave")]
    #[doc = "Setter for the `ondragleave` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ondragleave)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn set_ondragleave(this: &Document, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "ondragover")]
    #[doc = "Getter for the `ondragover` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ondragover)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn ondragover(this: &Document) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Document", js_name = "ondragover")]
    #[doc = "Setter for the `ondragover` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ondragover)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn set_ondragover(this: &Document, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "ondragstart")]
    #[doc = "Getter for the `ondragstart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ondragstart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn ondragstart(this: &Document) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Document", js_name = "ondragstart")]
    #[doc = "Setter for the `ondragstart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ondragstart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn set_ondragstart(this: &Document, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "ondrop")]
    #[doc = "Getter for the `ondrop` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ondrop)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn ondrop(this: &Document) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Document", js_name = "ondrop")]
    #[doc = "Setter for the `ondrop` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ondrop)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn set_ondrop(this: &Document, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "ondurationchange")]
    #[doc = "Getter for the `ondurationchange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ondurationchange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn ondurationchange(this: &Document) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Document", js_name = "ondurationchange")]
    #[doc = "Setter for the `ondurationchange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ondurationchange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn set_ondurationchange(this: &Document, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "onemptied")]
    #[doc = "Getter for the `onemptied` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onemptied)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn onemptied(this: &Document) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Document", js_name = "onemptied")]
    #[doc = "Setter for the `onemptied` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onemptied)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn set_onemptied(this: &Document, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "onended")]
    #[doc = "Getter for the `onended` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onended)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn onended(this: &Document) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Document", js_name = "onended")]
    #[doc = "Setter for the `onended` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onended)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn set_onended(this: &Document, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "oninput")]
    #[doc = "Getter for the `oninput` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/oninput)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn oninput(this: &Document) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Document", js_name = "oninput")]
    #[doc = "Setter for the `oninput` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/oninput)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn set_oninput(this: &Document, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "onbeforeinput")]
    #[doc = "Getter for the `onbeforeinput` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onbeforeinput)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn onbeforeinput(this: &Document) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Document", js_name = "onbeforeinput")]
    #[doc = "Setter for the `onbeforeinput` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onbeforeinput)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn set_onbeforeinput(this: &Document, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "oninvalid")]
    #[doc = "Getter for the `oninvalid` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/oninvalid)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn oninvalid(this: &Document) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Document", js_name = "oninvalid")]
    #[doc = "Setter for the `oninvalid` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/oninvalid)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn set_oninvalid(this: &Document, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "onkeydown")]
    #[doc = "Getter for the `onkeydown` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onkeydown)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn onkeydown(this: &Document) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Document", js_name = "onkeydown")]
    #[doc = "Setter for the `onkeydown` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onkeydown)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn set_onkeydown(this: &Document, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "onkeypress")]
    #[doc = "Getter for the `onkeypress` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onkeypress)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn onkeypress(this: &Document) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Document", js_name = "onkeypress")]
    #[doc = "Setter for the `onkeypress` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onkeypress)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn set_onkeypress(this: &Document, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "onkeyup")]
    #[doc = "Getter for the `onkeyup` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onkeyup)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn onkeyup(this: &Document) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Document", js_name = "onkeyup")]
    #[doc = "Setter for the `onkeyup` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onkeyup)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn set_onkeyup(this: &Document, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "onload")]
    #[doc = "Getter for the `onload` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onload)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn onload(this: &Document) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Document", js_name = "onload")]
    #[doc = "Setter for the `onload` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onload)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn set_onload(this: &Document, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "onloadeddata")]
    #[doc = "Getter for the `onloadeddata` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onloadeddata)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn onloadeddata(this: &Document) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Document", js_name = "onloadeddata")]
    #[doc = "Setter for the `onloadeddata` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onloadeddata)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn set_onloadeddata(this: &Document, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "onloadedmetadata")]
    #[doc = "Getter for the `onloadedmetadata` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onloadedmetadata)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn onloadedmetadata(this: &Document) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Document", js_name = "onloadedmetadata")]
    #[doc = "Setter for the `onloadedmetadata` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onloadedmetadata)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn set_onloadedmetadata(this: &Document, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "onloadend")]
    #[doc = "Getter for the `onloadend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onloadend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn onloadend(this: &Document) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Document", js_name = "onloadend")]
    #[doc = "Setter for the `onloadend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onloadend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn set_onloadend(this: &Document, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "onloadstart")]
    #[doc = "Getter for the `onloadstart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onloadstart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn onloadstart(this: &Document) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Document", js_name = "onloadstart")]
    #[doc = "Setter for the `onloadstart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onloadstart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn set_onloadstart(this: &Document, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "onmousedown")]
    #[doc = "Getter for the `onmousedown` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onmousedown)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn onmousedown(this: &Document) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Document", js_name = "onmousedown")]
    #[doc = "Setter for the `onmousedown` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onmousedown)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn set_onmousedown(this: &Document, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "onmouseenter")]
    #[doc = "Getter for the `onmouseenter` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onmouseenter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn onmouseenter(this: &Document) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Document", js_name = "onmouseenter")]
    #[doc = "Setter for the `onmouseenter` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onmouseenter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn set_onmouseenter(this: &Document, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "onmouseleave")]
    #[doc = "Getter for the `onmouseleave` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onmouseleave)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn onmouseleave(this: &Document) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Document", js_name = "onmouseleave")]
    #[doc = "Setter for the `onmouseleave` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onmouseleave)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn set_onmouseleave(this: &Document, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "onmousemove")]
    #[doc = "Getter for the `onmousemove` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onmousemove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn onmousemove(this: &Document) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Document", js_name = "onmousemove")]
    #[doc = "Setter for the `onmousemove` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onmousemove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn set_onmousemove(this: &Document, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "onmouseout")]
    #[doc = "Getter for the `onmouseout` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onmouseout)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn onmouseout(this: &Document) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Document", js_name = "onmouseout")]
    #[doc = "Setter for the `onmouseout` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onmouseout)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn set_onmouseout(this: &Document, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "onmouseover")]
    #[doc = "Getter for the `onmouseover` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onmouseover)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn onmouseover(this: &Document) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Document", js_name = "onmouseover")]
    #[doc = "Setter for the `onmouseover` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onmouseover)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn set_onmouseover(this: &Document, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "onmouseup")]
    #[doc = "Getter for the `onmouseup` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onmouseup)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn onmouseup(this: &Document) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Document", js_name = "onmouseup")]
    #[doc = "Setter for the `onmouseup` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onmouseup)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn set_onmouseup(this: &Document, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "onwheel")]
    #[doc = "Getter for the `onwheel` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onwheel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn onwheel(this: &Document) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Document", js_name = "onwheel")]
    #[doc = "Setter for the `onwheel` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onwheel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn set_onwheel(this: &Document, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "onpause")]
    #[doc = "Getter for the `onpause` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onpause)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn onpause(this: &Document) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Document", js_name = "onpause")]
    #[doc = "Setter for the `onpause` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onpause)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn set_onpause(this: &Document, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "onplay")]
    #[doc = "Getter for the `onplay` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onplay)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn onplay(this: &Document) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Document", js_name = "onplay")]
    #[doc = "Setter for the `onplay` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onplay)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn set_onplay(this: &Document, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "onplaying")]
    #[doc = "Getter for the `onplaying` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onplaying)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn onplaying(this: &Document) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Document", js_name = "onplaying")]
    #[doc = "Setter for the `onplaying` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onplaying)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn set_onplaying(this: &Document, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "onprogress")]
    #[doc = "Getter for the `onprogress` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onprogress)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn onprogress(this: &Document) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Document", js_name = "onprogress")]
    #[doc = "Setter for the `onprogress` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onprogress)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn set_onprogress(this: &Document, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "onratechange")]
    #[doc = "Getter for the `onratechange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onratechange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn onratechange(this: &Document) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Document", js_name = "onratechange")]
    #[doc = "Setter for the `onratechange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onratechange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn set_onratechange(this: &Document, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "onreset")]
    #[doc = "Getter for the `onreset` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onreset)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn onreset(this: &Document) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Document", js_name = "onreset")]
    #[doc = "Setter for the `onreset` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onreset)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn set_onreset(this: &Document, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "onresize")]
    #[doc = "Getter for the `onresize` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onresize)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn onresize(this: &Document) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Document", js_name = "onresize")]
    #[doc = "Setter for the `onresize` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onresize)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn set_onresize(this: &Document, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "onscroll")]
    #[doc = "Getter for the `onscroll` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onscroll)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn onscroll(this: &Document) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Document", js_name = "onscroll")]
    #[doc = "Setter for the `onscroll` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onscroll)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn set_onscroll(this: &Document, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "onseeked")]
    #[doc = "Getter for the `onseeked` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onseeked)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn onseeked(this: &Document) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Document", js_name = "onseeked")]
    #[doc = "Setter for the `onseeked` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onseeked)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn set_onseeked(this: &Document, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "onseeking")]
    #[doc = "Getter for the `onseeking` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onseeking)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn onseeking(this: &Document) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Document", js_name = "onseeking")]
    #[doc = "Setter for the `onseeking` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onseeking)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn set_onseeking(this: &Document, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "onselect")]
    #[doc = "Getter for the `onselect` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onselect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn onselect(this: &Document) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Document", js_name = "onselect")]
    #[doc = "Setter for the `onselect` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onselect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn set_onselect(this: &Document, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "onshow")]
    #[doc = "Getter for the `onshow` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onshow)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn onshow(this: &Document) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Document", js_name = "onshow")]
    #[doc = "Setter for the `onshow` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onshow)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn set_onshow(this: &Document, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "onstalled")]
    #[doc = "Getter for the `onstalled` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onstalled)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn onstalled(this: &Document) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Document", js_name = "onstalled")]
    #[doc = "Setter for the `onstalled` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onstalled)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn set_onstalled(this: &Document, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "onsubmit")]
    #[doc = "Getter for the `onsubmit` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onsubmit)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn onsubmit(this: &Document) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Document", js_name = "onsubmit")]
    #[doc = "Setter for the `onsubmit` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onsubmit)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn set_onsubmit(this: &Document, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "onsuspend")]
    #[doc = "Getter for the `onsuspend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onsuspend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn onsuspend(this: &Document) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Document", js_name = "onsuspend")]
    #[doc = "Setter for the `onsuspend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onsuspend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn set_onsuspend(this: &Document, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "ontimeupdate")]
    #[doc = "Getter for the `ontimeupdate` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ontimeupdate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn ontimeupdate(this: &Document) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Document", js_name = "ontimeupdate")]
    #[doc = "Setter for the `ontimeupdate` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ontimeupdate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn set_ontimeupdate(this: &Document, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "onvolumechange")]
    #[doc = "Getter for the `onvolumechange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onvolumechange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn onvolumechange(this: &Document) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Document", js_name = "onvolumechange")]
    #[doc = "Setter for the `onvolumechange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onvolumechange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn set_onvolumechange(this: &Document, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "onwaiting")]
    #[doc = "Getter for the `onwaiting` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onwaiting)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn onwaiting(this: &Document) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Document", js_name = "onwaiting")]
    #[doc = "Setter for the `onwaiting` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onwaiting)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn set_onwaiting(this: &Document, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "onselectstart")]
    #[doc = "Getter for the `onselectstart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onselectstart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn onselectstart(this: &Document) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Document", js_name = "onselectstart")]
    #[doc = "Setter for the `onselectstart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onselectstart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn set_onselectstart(this: &Document, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "ontoggle")]
    #[doc = "Getter for the `ontoggle` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ontoggle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn ontoggle(this: &Document) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Document", js_name = "ontoggle")]
    #[doc = "Setter for the `ontoggle` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ontoggle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn set_ontoggle(this: &Document, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "onpointercancel")]
    #[doc = "Getter for the `onpointercancel` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onpointercancel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn onpointercancel(this: &Document) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Document", js_name = "onpointercancel")]
    #[doc = "Setter for the `onpointercancel` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onpointercancel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn set_onpointercancel(this: &Document, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "onpointerdown")]
    #[doc = "Getter for the `onpointerdown` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onpointerdown)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn onpointerdown(this: &Document) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Document", js_name = "onpointerdown")]
    #[doc = "Setter for the `onpointerdown` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onpointerdown)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn set_onpointerdown(this: &Document, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "onpointerup")]
    #[doc = "Getter for the `onpointerup` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onpointerup)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn onpointerup(this: &Document) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Document", js_name = "onpointerup")]
    #[doc = "Setter for the `onpointerup` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onpointerup)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn set_onpointerup(this: &Document, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "onpointermove")]
    #[doc = "Getter for the `onpointermove` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onpointermove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn onpointermove(this: &Document) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Document", js_name = "onpointermove")]
    #[doc = "Setter for the `onpointermove` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onpointermove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn set_onpointermove(this: &Document, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "onpointerout")]
    #[doc = "Getter for the `onpointerout` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onpointerout)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn onpointerout(this: &Document) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Document", js_name = "onpointerout")]
    #[doc = "Setter for the `onpointerout` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onpointerout)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn set_onpointerout(this: &Document, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "onpointerover")]
    #[doc = "Getter for the `onpointerover` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onpointerover)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn onpointerover(this: &Document) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Document", js_name = "onpointerover")]
    #[doc = "Setter for the `onpointerover` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onpointerover)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn set_onpointerover(this: &Document, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "onpointerenter")]
    #[doc = "Getter for the `onpointerenter` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onpointerenter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn onpointerenter(this: &Document) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Document", js_name = "onpointerenter")]
    #[doc = "Setter for the `onpointerenter` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onpointerenter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn set_onpointerenter(this: &Document, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "onpointerleave")]
    #[doc = "Getter for the `onpointerleave` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onpointerleave)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn onpointerleave(this: &Document) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Document", js_name = "onpointerleave")]
    #[doc = "Setter for the `onpointerleave` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onpointerleave)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn set_onpointerleave(this: &Document, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "ongotpointercapture")]
    #[doc = "Getter for the `ongotpointercapture` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ongotpointercapture)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn ongotpointercapture(this: &Document) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Document", js_name = "ongotpointercapture")]
    #[doc = "Setter for the `ongotpointercapture` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ongotpointercapture)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn set_ongotpointercapture(this: &Document, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "Document",
        js_name = "onlostpointercapture"
    )]
    #[doc = "Getter for the `onlostpointercapture` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onlostpointercapture)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn onlostpointercapture(this: &Document) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "Document",
        js_name = "onlostpointercapture"
    )]
    #[doc = "Setter for the `onlostpointercapture` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onlostpointercapture)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn set_onlostpointercapture(this: &Document, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "onanimationcancel")]
    #[doc = "Getter for the `onanimationcancel` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onanimationcancel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn onanimationcancel(this: &Document) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Document", js_name = "onanimationcancel")]
    #[doc = "Setter for the `onanimationcancel` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onanimationcancel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn set_onanimationcancel(this: &Document, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "onanimationend")]
    #[doc = "Getter for the `onanimationend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onanimationend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn onanimationend(this: &Document) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Document", js_name = "onanimationend")]
    #[doc = "Setter for the `onanimationend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onanimationend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn set_onanimationend(this: &Document, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "Document",
        js_name = "onanimationiteration"
    )]
    #[doc = "Getter for the `onanimationiteration` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onanimationiteration)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn onanimationiteration(this: &Document) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "Document",
        js_name = "onanimationiteration"
    )]
    #[doc = "Setter for the `onanimationiteration` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onanimationiteration)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn set_onanimationiteration(this: &Document, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "onanimationstart")]
    #[doc = "Getter for the `onanimationstart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onanimationstart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn onanimationstart(this: &Document) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Document", js_name = "onanimationstart")]
    #[doc = "Setter for the `onanimationstart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onanimationstart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn set_onanimationstart(this: &Document, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "ontransitioncancel")]
    #[doc = "Getter for the `ontransitioncancel` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ontransitioncancel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn ontransitioncancel(this: &Document) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Document", js_name = "ontransitioncancel")]
    #[doc = "Setter for the `ontransitioncancel` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ontransitioncancel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn set_ontransitioncancel(this: &Document, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "ontransitionend")]
    #[doc = "Getter for the `ontransitionend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ontransitionend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn ontransitionend(this: &Document) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Document", js_name = "ontransitionend")]
    #[doc = "Setter for the `ontransitionend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ontransitionend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn set_ontransitionend(this: &Document, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "ontransitionrun")]
    #[doc = "Getter for the `ontransitionrun` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ontransitionrun)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn ontransitionrun(this: &Document) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Document", js_name = "ontransitionrun")]
    #[doc = "Setter for the `ontransitionrun` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ontransitionrun)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn set_ontransitionrun(this: &Document, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "ontransitionstart")]
    #[doc = "Getter for the `ontransitionstart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ontransitionstart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn ontransitionstart(this: &Document) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Document", js_name = "ontransitionstart")]
    #[doc = "Setter for the `ontransitionstart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ontransitionstart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn set_ontransitionstart(this: &Document, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "Document",
        js_name = "onwebkitanimationend"
    )]
    #[doc = "Getter for the `onwebkitanimationend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onwebkitanimationend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn onwebkitanimationend(this: &Document) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "Document",
        js_name = "onwebkitanimationend"
    )]
    #[doc = "Setter for the `onwebkitanimationend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onwebkitanimationend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn set_onwebkitanimationend(this: &Document, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "Document",
        js_name = "onwebkitanimationiteration"
    )]
    #[doc = "Getter for the `onwebkitanimationiteration` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onwebkitanimationiteration)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn onwebkitanimationiteration(this: &Document) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "Document",
        js_name = "onwebkitanimationiteration"
    )]
    #[doc = "Setter for the `onwebkitanimationiteration` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onwebkitanimationiteration)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn set_onwebkitanimationiteration(this: &Document, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "Document",
        js_name = "onwebkitanimationstart"
    )]
    #[doc = "Getter for the `onwebkitanimationstart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onwebkitanimationstart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn onwebkitanimationstart(this: &Document) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "Document",
        js_name = "onwebkitanimationstart"
    )]
    #[doc = "Setter for the `onwebkitanimationstart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onwebkitanimationstart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn set_onwebkitanimationstart(this: &Document, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "Document",
        js_name = "onwebkittransitionend"
    )]
    #[doc = "Getter for the `onwebkittransitionend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onwebkittransitionend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn onwebkittransitionend(this: &Document) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "Document",
        js_name = "onwebkittransitionend"
    )]
    #[doc = "Setter for the `onwebkittransitionend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onwebkittransitionend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn set_onwebkittransitionend(this: &Document, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "onerror")]
    #[doc = "Getter for the `onerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn onerror(this: &Document) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Document", js_name = "onerror")]
    #[doc = "Setter for the `onerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn set_onerror(this: &Document, value: Option<&::js_sys::Function>);
    #[cfg(feature = "HtmlCollection")]
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "children")]
    #[doc = "Getter for the `children` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/children)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `HtmlCollection`*"]
    pub fn children(this: &Document) -> HtmlCollection;
    #[cfg(feature = "Element")]
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "firstElementChild")]
    #[doc = "Getter for the `firstElementChild` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/firstElementChild)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `Element`*"]
    pub fn first_element_child(this: &Document) -> Option<Element>;
    #[cfg(feature = "Element")]
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "lastElementChild")]
    #[doc = "Getter for the `lastElementChild` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/lastElementChild)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `Element`*"]
    pub fn last_element_child(this: &Document) -> Option<Element>;
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "childElementCount")]
    #[doc = "Getter for the `childElementCount` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/childElementCount)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn child_element_count(this: &Document) -> u32;
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "ontouchstart")]
    #[doc = "Getter for the `ontouchstart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ontouchstart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn ontouchstart(this: &Document) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Document", js_name = "ontouchstart")]
    #[doc = "Setter for the `ontouchstart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ontouchstart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn set_ontouchstart(this: &Document, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "ontouchend")]
    #[doc = "Getter for the `ontouchend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ontouchend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn ontouchend(this: &Document) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Document", js_name = "ontouchend")]
    #[doc = "Setter for the `ontouchend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ontouchend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn set_ontouchend(this: &Document, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "ontouchmove")]
    #[doc = "Getter for the `ontouchmove` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ontouchmove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn ontouchmove(this: &Document) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Document", js_name = "ontouchmove")]
    #[doc = "Setter for the `ontouchmove` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ontouchmove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn set_ontouchmove(this: &Document, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Document", js_name = "ontouchcancel")]
    #[doc = "Getter for the `ontouchcancel` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ontouchcancel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn ontouchcancel(this: &Document) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Document", js_name = "ontouchcancel")]
    #[doc = "Setter for the `ontouchcancel` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ontouchcancel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn set_ontouchcancel(this: &Document, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(catch, constructor, js_class = "Document")]
    #[doc = "The `new Document(..)` constructor, creating a new instance of `Document`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/Document)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn new() -> Result<Document, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Document", js_name = "adoptNode")]
    #[doc = "The `adoptNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/adoptNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn adopt_node(this: &Document, node: &Node) -> Result<Node, JsValue>;
    #[cfg(feature = "CaretPosition")]
    #[wasm_bindgen(method, js_class = "Document", js_name = "caretPositionFromPoint")]
    #[doc = "The `caretPositionFromPoint()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/caretPositionFromPoint)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CaretPosition`, `Document`*"]
    pub fn caret_position_from_point(this: &Document, x: f32, y: f32) -> Option<CaretPosition>;
    #[cfg(feature = "Attr")]
    #[wasm_bindgen(catch, method, js_class = "Document", js_name = "createAttribute")]
    #[doc = "The `createAttribute()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/createAttribute)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Attr`, `Document`*"]
    pub fn create_attribute(this: &Document, name: &str) -> Result<Attr, JsValue>;
    #[cfg(feature = "Attr")]
    #[wasm_bindgen(catch, method, js_class = "Document", js_name = "createAttributeNS")]
    #[doc = "The `createAttributeNS()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/createAttributeNS)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Attr`, `Document`*"]
    pub fn create_attribute_ns(
        this: &Document,
        namespace: Option<&str>,
        name: &str,
    ) -> Result<Attr, JsValue>;
    #[cfg(feature = "CdataSection")]
    #[wasm_bindgen(catch, method, js_class = "Document", js_name = "createCDATASection")]
    #[doc = "The `createCDATASection()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/createCDATASection)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CdataSection`, `Document`*"]
    pub fn create_cdata_section(this: &Document, data: &str) -> Result<CdataSection, JsValue>;
    #[cfg(feature = "Comment")]
    #[wasm_bindgen(method, js_class = "Document", js_name = "createComment")]
    #[doc = "The `createComment()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/createComment)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Comment`, `Document`*"]
    pub fn create_comment(this: &Document, data: &str) -> Comment;
    #[cfg(feature = "DocumentFragment")]
    #[wasm_bindgen(method, js_class = "Document", js_name = "createDocumentFragment")]
    #[doc = "The `createDocumentFragment()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/createDocumentFragment)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `DocumentFragment`*"]
    pub fn create_document_fragment(this: &Document) -> DocumentFragment;
    #[cfg(feature = "Element")]
    #[wasm_bindgen(catch, method, js_class = "Document", js_name = "createElement")]
    #[doc = "The `createElement()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/createElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `Element`*"]
    pub fn create_element(this: &Document, local_name: &str) -> Result<Element, JsValue>;
    #[cfg(all(feature = "Element", feature = "ElementCreationOptions",))]
    #[wasm_bindgen(catch, method, js_class = "Document", js_name = "createElement")]
    #[doc = "The `createElement()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/createElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `Element`, `ElementCreationOptions`*"]
    pub fn create_element_with_element_creation_options(
        this: &Document,
        local_name: &str,
        options: &ElementCreationOptions,
    ) -> Result<Element, JsValue>;
    #[cfg(feature = "Element")]
    #[wasm_bindgen(catch, method, js_class = "Document", js_name = "createElement")]
    #[doc = "The `createElement()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/createElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `Element`*"]
    pub fn create_element_with_str(
        this: &Document,
        local_name: &str,
        options: &str,
    ) -> Result<Element, JsValue>;
    #[cfg(feature = "Element")]
    #[wasm_bindgen(catch, method, js_class = "Document", js_name = "createElementNS")]
    #[doc = "The `createElementNS()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/createElementNS)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `Element`*"]
    pub fn create_element_ns(
        this: &Document,
        namespace: Option<&str>,
        qualified_name: &str,
    ) -> Result<Element, JsValue>;
    #[cfg(all(feature = "Element", feature = "ElementCreationOptions",))]
    #[wasm_bindgen(catch, method, js_class = "Document", js_name = "createElementNS")]
    #[doc = "The `createElementNS()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/createElementNS)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `Element`, `ElementCreationOptions`*"]
    pub fn create_element_ns_with_element_creation_options(
        this: &Document,
        namespace: Option<&str>,
        qualified_name: &str,
        options: &ElementCreationOptions,
    ) -> Result<Element, JsValue>;
    #[cfg(feature = "Element")]
    #[wasm_bindgen(catch, method, js_class = "Document", js_name = "createElementNS")]
    #[doc = "The `createElementNS()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/createElementNS)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `Element`*"]
    pub fn create_element_ns_with_str(
        this: &Document,
        namespace: Option<&str>,
        qualified_name: &str,
        options: &str,
    ) -> Result<Element, JsValue>;
    #[cfg(feature = "Event")]
    #[wasm_bindgen(catch, method, js_class = "Document", js_name = "createEvent")]
    #[doc = "The `createEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/createEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `Event`*"]
    pub fn create_event(this: &Document, interface: &str) -> Result<Event, JsValue>;
    #[cfg(feature = "NodeIterator")]
    #[wasm_bindgen(catch, method, js_class = "Document", js_name = "createNodeIterator")]
    #[doc = "The `createNodeIterator()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/createNodeIterator)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `NodeIterator`*"]
    pub fn create_node_iterator(this: &Document, root: &Node) -> Result<NodeIterator, JsValue>;
    #[cfg(feature = "NodeIterator")]
    #[wasm_bindgen(catch, method, js_class = "Document", js_name = "createNodeIterator")]
    #[doc = "The `createNodeIterator()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/createNodeIterator)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `NodeIterator`*"]
    pub fn create_node_iterator_with_what_to_show(
        this: &Document,
        root: &Node,
        what_to_show: u32,
    ) -> Result<NodeIterator, JsValue>;
    #[cfg(all(feature = "NodeFilter", feature = "NodeIterator",))]
    #[wasm_bindgen(catch, method, js_class = "Document", js_name = "createNodeIterator")]
    #[doc = "The `createNodeIterator()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/createNodeIterator)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `NodeFilter`, `NodeIterator`*"]
    pub fn create_node_iterator_with_what_to_show_and_filter(
        this: &Document,
        root: &Node,
        what_to_show: u32,
        filter: Option<&NodeFilter>,
    ) -> Result<NodeIterator, JsValue>;
    #[cfg(feature = "ProcessingInstruction")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "Document",
        js_name = "createProcessingInstruction"
    )]
    #[doc = "The `createProcessingInstruction()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/createProcessingInstruction)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `ProcessingInstruction`*"]
    pub fn create_processing_instruction(
        this: &Document,
        target: &str,
        data: &str,
    ) -> Result<ProcessingInstruction, JsValue>;
    #[cfg(feature = "Range")]
    #[wasm_bindgen(catch, method, js_class = "Document", js_name = "createRange")]
    #[doc = "The `createRange()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/createRange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `Range`*"]
    pub fn create_range(this: &Document) -> Result<Range, JsValue>;
    #[cfg(feature = "Text")]
    #[wasm_bindgen(method, js_class = "Document", js_name = "createTextNode")]
    #[doc = "The `createTextNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/createTextNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `Text`*"]
    pub fn create_text_node(this: &Document, data: &str) -> Text;
    #[cfg(feature = "TreeWalker")]
    #[wasm_bindgen(catch, method, js_class = "Document", js_name = "createTreeWalker")]
    #[doc = "The `createTreeWalker()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/createTreeWalker)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `TreeWalker`*"]
    pub fn create_tree_walker(this: &Document, root: &Node) -> Result<TreeWalker, JsValue>;
    #[cfg(feature = "TreeWalker")]
    #[wasm_bindgen(catch, method, js_class = "Document", js_name = "createTreeWalker")]
    #[doc = "The `createTreeWalker()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/createTreeWalker)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `TreeWalker`*"]
    pub fn create_tree_walker_with_what_to_show(
        this: &Document,
        root: &Node,
        what_to_show: u32,
    ) -> Result<TreeWalker, JsValue>;
    #[cfg(all(feature = "NodeFilter", feature = "TreeWalker",))]
    #[wasm_bindgen(catch, method, js_class = "Document", js_name = "createTreeWalker")]
    #[doc = "The `createTreeWalker()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/createTreeWalker)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `NodeFilter`, `TreeWalker`*"]
    pub fn create_tree_walker_with_what_to_show_and_filter(
        this: &Document,
        root: &Node,
        what_to_show: u32,
        filter: Option<&NodeFilter>,
    ) -> Result<TreeWalker, JsValue>;
    #[wasm_bindgen(method, js_class = "Document", js_name = "enableStyleSheetsForSet")]
    #[doc = "The `enableStyleSheetsForSet()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/enableStyleSheetsForSet)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn enable_style_sheets_for_set(this: &Document, name: Option<&str>);
    #[wasm_bindgen(method, js_class = "Document", js_name = "exitFullscreen")]
    #[doc = "The `exitFullscreen()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/exitFullscreen)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn exit_fullscreen(this: &Document);
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, js_class = "Document", js_name = "exitPictureInPicture")]
    #[doc = "The `exitPictureInPicture()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/exitPictureInPicture)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn exit_picture_in_picture(this: &Document) -> ::js_sys::Promise<::js_sys::Undefined>;
    #[wasm_bindgen(method, js_class = "Document", js_name = "exitPointerLock")]
    #[doc = "The `exitPointerLock()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/exitPointerLock)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn exit_pointer_lock(this: &Document);
    #[wasm_bindgen(method, js_class = "Document", js_name = "getAnimations")]
    #[doc = "The `getAnimations()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/getAnimations)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn get_animations(this: &Document) -> ::js_sys::Array;
    #[cfg(feature = "Element")]
    #[wasm_bindgen(method, js_class = "Document", js_name = "getElementById")]
    #[doc = "The `getElementById()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/getElementById)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `Element`*"]
    pub fn get_element_by_id(this: &Document, element_id: &str) -> Option<Element>;
    #[cfg(feature = "HtmlCollection")]
    #[wasm_bindgen(method, js_class = "Document", js_name = "getElementsByClassName")]
    #[doc = "The `getElementsByClassName()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/getElementsByClassName)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `HtmlCollection`*"]
    pub fn get_elements_by_class_name(this: &Document, class_names: &str) -> HtmlCollection;
    #[cfg(feature = "NodeList")]
    #[wasm_bindgen(method, js_class = "Document", js_name = "getElementsByName")]
    #[doc = "The `getElementsByName()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/getElementsByName)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `NodeList`*"]
    pub fn get_elements_by_name(this: &Document, element_name: &str) -> NodeList;
    #[cfg(feature = "HtmlCollection")]
    #[wasm_bindgen(method, js_class = "Document", js_name = "getElementsByTagName")]
    #[doc = "The `getElementsByTagName()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/getElementsByTagName)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `HtmlCollection`*"]
    pub fn get_elements_by_tag_name(this: &Document, local_name: &str) -> HtmlCollection;
    #[cfg(feature = "HtmlCollection")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "Document",
        js_name = "getElementsByTagNameNS"
    )]
    #[doc = "The `getElementsByTagNameNS()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/getElementsByTagNameNS)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `HtmlCollection`*"]
    pub fn get_elements_by_tag_name_ns(
        this: &Document,
        namespace: Option<&str>,
        local_name: &str,
    ) -> Result<HtmlCollection, JsValue>;
    #[cfg(feature = "Selection")]
    #[wasm_bindgen(catch, method, js_class = "Document", js_name = "getSelection")]
    #[doc = "The `getSelection()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/getSelection)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `Selection`*"]
    pub fn get_selection(this: &Document) -> Result<Option<Selection>, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Document", js_name = "hasFocus")]
    #[doc = "The `hasFocus()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/hasFocus)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn has_focus(this: &Document) -> Result<bool, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Document", js_name = "importNode")]
    #[doc = "The `importNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/importNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn import_node(this: &Document, node: &Node) -> Result<Node, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Document", js_name = "importNode")]
    #[doc = "The `importNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/importNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn import_node_with_deep(this: &Document, node: &Node, deep: bool)
        -> Result<Node, JsValue>;
    #[cfg(feature = "Element")]
    #[wasm_bindgen(catch, method, js_class = "Document", js_name = "querySelector")]
    #[doc = "The `querySelector()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/querySelector)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `Element`*"]
    pub fn query_selector(this: &Document, selectors: &str) -> Result<Option<Element>, JsValue>;
    #[cfg(feature = "NodeList")]
    #[wasm_bindgen(catch, method, js_class = "Document", js_name = "querySelectorAll")]
    #[doc = "The `querySelectorAll()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/querySelectorAll)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `NodeList`*"]
    pub fn query_selector_all(this: &Document, selectors: &str) -> Result<NodeList, JsValue>;
    #[wasm_bindgen(method, js_class = "Document", js_name = "releaseCapture")]
    #[doc = "The `releaseCapture()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/releaseCapture)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn release_capture(this: &Document);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "ViewTransition")]
    #[wasm_bindgen(catch, method, js_class = "Document", js_name = "startViewTransition")]
    #[doc = "The `startViewTransition()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/startViewTransition)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `ViewTransition`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn start_view_transition(this: &Document) -> Result<ViewTransition, JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "ViewTransition")]
    #[wasm_bindgen(catch, method, js_class = "Document", js_name = "startViewTransition")]
    #[doc = "The `startViewTransition()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/startViewTransition)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `ViewTransition`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn start_view_transition_with_update_callback(
        this: &Document,
        update_callback: Option<&::js_sys::Function<fn() -> ::js_sys::Promise>>,
    ) -> Result<ViewTransition, JsValue>;
    #[cfg(feature = "Element")]
    #[wasm_bindgen(method, js_class = "Document", js_name = "elementFromPoint")]
    #[doc = "The `elementFromPoint()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/elementFromPoint)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `Element`*"]
    pub fn element_from_point(this: &Document, x: f32, y: f32) -> Option<Element>;
    #[wasm_bindgen(method, js_class = "Document", js_name = "elementsFromPoint")]
    #[doc = "The `elementsFromPoint()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/elementsFromPoint)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn elements_from_point(this: &Document, x: f32, y: f32) -> ::js_sys::Array;
    #[cfg(all(feature = "DomPoint", feature = "DomPointInit", feature = "Text",))]
    #[wasm_bindgen(catch, method, js_class = "Document", js_name = "convertPointFromNode")]
    #[doc = "The `convertPointFromNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/convertPointFromNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `DomPoint`, `DomPointInit`, `Text`*"]
    pub fn convert_point_from_node_with_text(
        this: &Document,
        point: &DomPointInit,
        from: &Text,
    ) -> Result<DomPoint, JsValue>;
    #[cfg(all(feature = "DomPoint", feature = "DomPointInit", feature = "Element",))]
    #[wasm_bindgen(catch, method, js_class = "Document", js_name = "convertPointFromNode")]
    #[doc = "The `convertPointFromNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/convertPointFromNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `DomPoint`, `DomPointInit`, `Element`*"]
    pub fn convert_point_from_node_with_element(
        this: &Document,
        point: &DomPointInit,
        from: &Element,
    ) -> Result<DomPoint, JsValue>;
    #[cfg(all(feature = "DomPoint", feature = "DomPointInit",))]
    #[wasm_bindgen(catch, method, js_class = "Document", js_name = "convertPointFromNode")]
    #[doc = "The `convertPointFromNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/convertPointFromNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `DomPoint`, `DomPointInit`*"]
    pub fn convert_point_from_node_with_document(
        this: &Document,
        point: &DomPointInit,
        from: &Document,
    ) -> Result<DomPoint, JsValue>;
    #[cfg(all(
        feature = "ConvertCoordinateOptions",
        feature = "DomPoint",
        feature = "DomPointInit",
        feature = "Text",
    ))]
    #[wasm_bindgen(catch, method, js_class = "Document", js_name = "convertPointFromNode")]
    #[doc = "The `convertPointFromNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/convertPointFromNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConvertCoordinateOptions`, `Document`, `DomPoint`, `DomPointInit`, `Text`*"]
    pub fn convert_point_from_node_with_text_and_options(
        this: &Document,
        point: &DomPointInit,
        from: &Text,
        options: &ConvertCoordinateOptions,
    ) -> Result<DomPoint, JsValue>;
    #[cfg(all(
        feature = "ConvertCoordinateOptions",
        feature = "DomPoint",
        feature = "DomPointInit",
        feature = "Element",
    ))]
    #[wasm_bindgen(catch, method, js_class = "Document", js_name = "convertPointFromNode")]
    #[doc = "The `convertPointFromNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/convertPointFromNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConvertCoordinateOptions`, `Document`, `DomPoint`, `DomPointInit`, `Element`*"]
    pub fn convert_point_from_node_with_element_and_options(
        this: &Document,
        point: &DomPointInit,
        from: &Element,
        options: &ConvertCoordinateOptions,
    ) -> Result<DomPoint, JsValue>;
    #[cfg(all(
        feature = "ConvertCoordinateOptions",
        feature = "DomPoint",
        feature = "DomPointInit",
    ))]
    #[wasm_bindgen(catch, method, js_class = "Document", js_name = "convertPointFromNode")]
    #[doc = "The `convertPointFromNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/convertPointFromNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConvertCoordinateOptions`, `Document`, `DomPoint`, `DomPointInit`*"]
    pub fn convert_point_from_node_with_document_and_options(
        this: &Document,
        point: &DomPointInit,
        from: &Document,
        options: &ConvertCoordinateOptions,
    ) -> Result<DomPoint, JsValue>;
    #[cfg(all(feature = "DomQuad", feature = "Text",))]
    #[wasm_bindgen(catch, method, js_class = "Document", js_name = "convertQuadFromNode")]
    #[doc = "The `convertQuadFromNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/convertQuadFromNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `DomQuad`, `Text`*"]
    pub fn convert_quad_from_node_with_text(
        this: &Document,
        quad: &DomQuad,
        from: &Text,
    ) -> Result<DomQuad, JsValue>;
    #[cfg(all(feature = "DomQuad", feature = "Element",))]
    #[wasm_bindgen(catch, method, js_class = "Document", js_name = "convertQuadFromNode")]
    #[doc = "The `convertQuadFromNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/convertQuadFromNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `DomQuad`, `Element`*"]
    pub fn convert_quad_from_node_with_element(
        this: &Document,
        quad: &DomQuad,
        from: &Element,
    ) -> Result<DomQuad, JsValue>;
    #[cfg(feature = "DomQuad")]
    #[wasm_bindgen(catch, method, js_class = "Document", js_name = "convertQuadFromNode")]
    #[doc = "The `convertQuadFromNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/convertQuadFromNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `DomQuad`*"]
    pub fn convert_quad_from_node_with_document(
        this: &Document,
        quad: &DomQuad,
        from: &Document,
    ) -> Result<DomQuad, JsValue>;
    #[cfg(all(
        feature = "ConvertCoordinateOptions",
        feature = "DomQuad",
        feature = "Text",
    ))]
    #[wasm_bindgen(catch, method, js_class = "Document", js_name = "convertQuadFromNode")]
    #[doc = "The `convertQuadFromNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/convertQuadFromNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConvertCoordinateOptions`, `Document`, `DomQuad`, `Text`*"]
    pub fn convert_quad_from_node_with_text_and_options(
        this: &Document,
        quad: &DomQuad,
        from: &Text,
        options: &ConvertCoordinateOptions,
    ) -> Result<DomQuad, JsValue>;
    #[cfg(all(
        feature = "ConvertCoordinateOptions",
        feature = "DomQuad",
        feature = "Element",
    ))]
    #[wasm_bindgen(catch, method, js_class = "Document", js_name = "convertQuadFromNode")]
    #[doc = "The `convertQuadFromNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/convertQuadFromNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConvertCoordinateOptions`, `Document`, `DomQuad`, `Element`*"]
    pub fn convert_quad_from_node_with_element_and_options(
        this: &Document,
        quad: &DomQuad,
        from: &Element,
        options: &ConvertCoordinateOptions,
    ) -> Result<DomQuad, JsValue>;
    #[cfg(all(feature = "ConvertCoordinateOptions", feature = "DomQuad",))]
    #[wasm_bindgen(catch, method, js_class = "Document", js_name = "convertQuadFromNode")]
    #[doc = "The `convertQuadFromNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/convertQuadFromNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConvertCoordinateOptions`, `Document`, `DomQuad`*"]
    pub fn convert_quad_from_node_with_document_and_options(
        this: &Document,
        quad: &DomQuad,
        from: &Document,
        options: &ConvertCoordinateOptions,
    ) -> Result<DomQuad, JsValue>;
    #[cfg(all(feature = "DomQuad", feature = "DomRectReadOnly", feature = "Text",))]
    #[wasm_bindgen(catch, method, js_class = "Document", js_name = "convertRectFromNode")]
    #[doc = "The `convertRectFromNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/convertRectFromNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `DomQuad`, `DomRectReadOnly`, `Text`*"]
    pub fn convert_rect_from_node_with_text(
        this: &Document,
        rect: &DomRectReadOnly,
        from: &Text,
    ) -> Result<DomQuad, JsValue>;
    #[cfg(all(feature = "DomQuad", feature = "DomRectReadOnly", feature = "Element",))]
    #[wasm_bindgen(catch, method, js_class = "Document", js_name = "convertRectFromNode")]
    #[doc = "The `convertRectFromNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/convertRectFromNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `DomQuad`, `DomRectReadOnly`, `Element`*"]
    pub fn convert_rect_from_node_with_element(
        this: &Document,
        rect: &DomRectReadOnly,
        from: &Element,
    ) -> Result<DomQuad, JsValue>;
    #[cfg(all(feature = "DomQuad", feature = "DomRectReadOnly",))]
    #[wasm_bindgen(catch, method, js_class = "Document", js_name = "convertRectFromNode")]
    #[doc = "The `convertRectFromNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/convertRectFromNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `DomQuad`, `DomRectReadOnly`*"]
    pub fn convert_rect_from_node_with_document(
        this: &Document,
        rect: &DomRectReadOnly,
        from: &Document,
    ) -> Result<DomQuad, JsValue>;
    #[cfg(all(
        feature = "ConvertCoordinateOptions",
        feature = "DomQuad",
        feature = "DomRectReadOnly",
        feature = "Text",
    ))]
    #[wasm_bindgen(catch, method, js_class = "Document", js_name = "convertRectFromNode")]
    #[doc = "The `convertRectFromNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/convertRectFromNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConvertCoordinateOptions`, `Document`, `DomQuad`, `DomRectReadOnly`, `Text`*"]
    pub fn convert_rect_from_node_with_text_and_options(
        this: &Document,
        rect: &DomRectReadOnly,
        from: &Text,
        options: &ConvertCoordinateOptions,
    ) -> Result<DomQuad, JsValue>;
    #[cfg(all(
        feature = "ConvertCoordinateOptions",
        feature = "DomQuad",
        feature = "DomRectReadOnly",
        feature = "Element",
    ))]
    #[wasm_bindgen(catch, method, js_class = "Document", js_name = "convertRectFromNode")]
    #[doc = "The `convertRectFromNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/convertRectFromNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConvertCoordinateOptions`, `Document`, `DomQuad`, `DomRectReadOnly`, `Element`*"]
    pub fn convert_rect_from_node_with_element_and_options(
        this: &Document,
        rect: &DomRectReadOnly,
        from: &Element,
        options: &ConvertCoordinateOptions,
    ) -> Result<DomQuad, JsValue>;
    #[cfg(all(
        feature = "ConvertCoordinateOptions",
        feature = "DomQuad",
        feature = "DomRectReadOnly",
    ))]
    #[wasm_bindgen(catch, method, js_class = "Document", js_name = "convertRectFromNode")]
    #[doc = "The `convertRectFromNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/convertRectFromNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConvertCoordinateOptions`, `Document`, `DomQuad`, `DomRectReadOnly`*"]
    pub fn convert_rect_from_node_with_document_and_options(
        this: &Document,
        rect: &DomRectReadOnly,
        from: &Document,
        options: &ConvertCoordinateOptions,
    ) -> Result<DomQuad, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Document", js_name = "getBoxQuads")]
    #[doc = "The `getBoxQuads()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/getBoxQuads)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn get_box_quads(this: &Document) -> Result<::js_sys::Array, JsValue>;
    #[cfg(feature = "BoxQuadOptions")]
    #[wasm_bindgen(catch, method, js_class = "Document", js_name = "getBoxQuads")]
    #[doc = "The `getBoxQuads()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/getBoxQuads)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BoxQuadOptions`, `Document`*"]
    pub fn get_box_quads_with_options(
        this: &Document,
        options: &BoxQuadOptions,
    ) -> Result<::js_sys::Array, JsValue>;
    #[wasm_bindgen(catch, method, variadic, js_class = "Document", js_name = "append")]
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn append_with_node(this: &Document, nodes: &::js_sys::Array) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Document", js_name = "append")]
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn append_with_node_0(this: &Document) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Document", js_name = "append")]
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn append_with_node_1(this: &Document, nodes_1: &Node) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Document", js_name = "append")]
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn append_with_node_2(
        this: &Document,
        nodes_1: &Node,
        nodes_2: &Node,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Document", js_name = "append")]
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn append_with_node_3(
        this: &Document,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Document", js_name = "append")]
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn append_with_node_4(
        this: &Document,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Document", js_name = "append")]
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn append_with_node_5(
        this: &Document,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Document", js_name = "append")]
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn append_with_node_6(
        this: &Document,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
        nodes_6: &Node,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Document", js_name = "append")]
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn append_with_node_7(
        this: &Document,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
        nodes_6: &Node,
        nodes_7: &Node,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, variadic, js_class = "Document", js_name = "append")]
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn append_with_str(this: &Document, nodes: &::js_sys::Array) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Document", js_name = "append")]
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn append_with_str_0(this: &Document) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Document", js_name = "append")]
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn append_with_str_1(this: &Document, nodes_1: &str) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Document", js_name = "append")]
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn append_with_str_2(this: &Document, nodes_1: &str, nodes_2: &str) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Document", js_name = "append")]
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn append_with_str_3(
        this: &Document,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Document", js_name = "append")]
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn append_with_str_4(
        this: &Document,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Document", js_name = "append")]
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn append_with_str_5(
        this: &Document,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Document", js_name = "append")]
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn append_with_str_6(
        this: &Document,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
        nodes_6: &str,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Document", js_name = "append")]
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn append_with_str_7(
        this: &Document,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
        nodes_6: &str,
        nodes_7: &str,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, variadic, js_class = "Document", js_name = "prepend")]
    #[doc = "The `prepend()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/prepend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn prepend_with_node(this: &Document, nodes: &::js_sys::Array) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Document", js_name = "prepend")]
    #[doc = "The `prepend()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/prepend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn prepend_with_node_0(this: &Document) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Document", js_name = "prepend")]
    #[doc = "The `prepend()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/prepend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn prepend_with_node_1(this: &Document, nodes_1: &Node) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Document", js_name = "prepend")]
    #[doc = "The `prepend()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/prepend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn prepend_with_node_2(
        this: &Document,
        nodes_1: &Node,
        nodes_2: &Node,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Document", js_name = "prepend")]
    #[doc = "The `prepend()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/prepend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn prepend_with_node_3(
        this: &Document,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Document", js_name = "prepend")]
    #[doc = "The `prepend()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/prepend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn prepend_with_node_4(
        this: &Document,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Document", js_name = "prepend")]
    #[doc = "The `prepend()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/prepend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn prepend_with_node_5(
        this: &Document,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Document", js_name = "prepend")]
    #[doc = "The `prepend()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/prepend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn prepend_with_node_6(
        this: &Document,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
        nodes_6: &Node,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Document", js_name = "prepend")]
    #[doc = "The `prepend()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/prepend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn prepend_with_node_7(
        this: &Document,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
        nodes_6: &Node,
        nodes_7: &Node,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, variadic, js_class = "Document", js_name = "prepend")]
    #[doc = "The `prepend()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/prepend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn prepend_with_str(this: &Document, nodes: &::js_sys::Array) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Document", js_name = "prepend")]
    #[doc = "The `prepend()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/prepend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn prepend_with_str_0(this: &Document) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Document", js_name = "prepend")]
    #[doc = "The `prepend()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/prepend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn prepend_with_str_1(this: &Document, nodes_1: &str) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Document", js_name = "prepend")]
    #[doc = "The `prepend()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/prepend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn prepend_with_str_2(this: &Document, nodes_1: &str, nodes_2: &str)
        -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Document", js_name = "prepend")]
    #[doc = "The `prepend()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/prepend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn prepend_with_str_3(
        this: &Document,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Document", js_name = "prepend")]
    #[doc = "The `prepend()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/prepend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn prepend_with_str_4(
        this: &Document,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Document", js_name = "prepend")]
    #[doc = "The `prepend()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/prepend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn prepend_with_str_5(
        this: &Document,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Document", js_name = "prepend")]
    #[doc = "The `prepend()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/prepend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn prepend_with_str_6(
        this: &Document,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
        nodes_6: &str,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Document", js_name = "prepend")]
    #[doc = "The `prepend()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/prepend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn prepend_with_str_7(
        this: &Document,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
        nodes_6: &str,
        nodes_7: &str,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(method, variadic, js_class = "Document", js_name = "replaceChildren")]
    #[doc = "The `replaceChildren()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/replaceChildren)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn replace_children_with_node(this: &Document, nodes: &::js_sys::Array);
    #[wasm_bindgen(method, js_class = "Document", js_name = "replaceChildren")]
    #[doc = "The `replaceChildren()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/replaceChildren)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn replace_children_with_node_0(this: &Document);
    #[wasm_bindgen(method, js_class = "Document", js_name = "replaceChildren")]
    #[doc = "The `replaceChildren()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/replaceChildren)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn replace_children_with_node_1(this: &Document, nodes_1: &Node);
    #[wasm_bindgen(method, js_class = "Document", js_name = "replaceChildren")]
    #[doc = "The `replaceChildren()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/replaceChildren)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn replace_children_with_node_2(this: &Document, nodes_1: &Node, nodes_2: &Node);
    #[wasm_bindgen(method, js_class = "Document", js_name = "replaceChildren")]
    #[doc = "The `replaceChildren()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/replaceChildren)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn replace_children_with_node_3(
        this: &Document,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
    );
    #[wasm_bindgen(method, js_class = "Document", js_name = "replaceChildren")]
    #[doc = "The `replaceChildren()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/replaceChildren)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn replace_children_with_node_4(
        this: &Document,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
    );
    #[wasm_bindgen(method, js_class = "Document", js_name = "replaceChildren")]
    #[doc = "The `replaceChildren()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/replaceChildren)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn replace_children_with_node_5(
        this: &Document,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
    );
    #[wasm_bindgen(method, js_class = "Document", js_name = "replaceChildren")]
    #[doc = "The `replaceChildren()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/replaceChildren)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn replace_children_with_node_6(
        this: &Document,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
        nodes_6: &Node,
    );
    #[wasm_bindgen(method, js_class = "Document", js_name = "replaceChildren")]
    #[doc = "The `replaceChildren()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/replaceChildren)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn replace_children_with_node_7(
        this: &Document,
        nodes_1: &Node,
        nodes_2: &Node,
        nodes_3: &Node,
        nodes_4: &Node,
        nodes_5: &Node,
        nodes_6: &Node,
        nodes_7: &Node,
    );
    #[wasm_bindgen(method, variadic, js_class = "Document", js_name = "replaceChildren")]
    #[doc = "The `replaceChildren()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/replaceChildren)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn replace_children_with_str(this: &Document, nodes: &::js_sys::Array);
    #[wasm_bindgen(method, js_class = "Document", js_name = "replaceChildren")]
    #[doc = "The `replaceChildren()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/replaceChildren)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn replace_children_with_str_0(this: &Document);
    #[wasm_bindgen(method, js_class = "Document", js_name = "replaceChildren")]
    #[doc = "The `replaceChildren()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/replaceChildren)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn replace_children_with_str_1(this: &Document, nodes_1: &str);
    #[wasm_bindgen(method, js_class = "Document", js_name = "replaceChildren")]
    #[doc = "The `replaceChildren()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/replaceChildren)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn replace_children_with_str_2(this: &Document, nodes_1: &str, nodes_2: &str);
    #[wasm_bindgen(method, js_class = "Document", js_name = "replaceChildren")]
    #[doc = "The `replaceChildren()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/replaceChildren)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn replace_children_with_str_3(
        this: &Document,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
    );
    #[wasm_bindgen(method, js_class = "Document", js_name = "replaceChildren")]
    #[doc = "The `replaceChildren()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/replaceChildren)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn replace_children_with_str_4(
        this: &Document,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
    );
    #[wasm_bindgen(method, js_class = "Document", js_name = "replaceChildren")]
    #[doc = "The `replaceChildren()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/replaceChildren)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn replace_children_with_str_5(
        this: &Document,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
    );
    #[wasm_bindgen(method, js_class = "Document", js_name = "replaceChildren")]
    #[doc = "The `replaceChildren()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/replaceChildren)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn replace_children_with_str_6(
        this: &Document,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
        nodes_6: &str,
    );
    #[wasm_bindgen(method, js_class = "Document", js_name = "replaceChildren")]
    #[doc = "The `replaceChildren()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/replaceChildren)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn replace_children_with_str_7(
        this: &Document,
        nodes_1: &str,
        nodes_2: &str,
        nodes_3: &str,
        nodes_4: &str,
        nodes_5: &str,
        nodes_6: &str,
        nodes_7: &str,
    );
    #[cfg(feature = "XPathExpression")]
    #[wasm_bindgen(catch, method, js_class = "Document", js_name = "createExpression")]
    #[doc = "The `createExpression()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/createExpression)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `XPathExpression`*"]
    pub fn create_expression(this: &Document, expression: &str)
        -> Result<XPathExpression, JsValue>;
    #[cfg(feature = "XPathExpression")]
    #[wasm_bindgen(catch, method, js_class = "Document", js_name = "createExpression")]
    #[doc = "The `createExpression()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/createExpression)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `XPathExpression`*"]
    pub fn create_expression_with_opt_callback(
        this: &Document,
        expression: &str,
        resolver: Option<&::js_sys::Function>,
    ) -> Result<XPathExpression, JsValue>;
    #[cfg(all(feature = "XPathExpression", feature = "XPathNsResolver",))]
    #[wasm_bindgen(catch, method, js_class = "Document", js_name = "createExpression")]
    #[doc = "The `createExpression()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/createExpression)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `XPathExpression`, `XPathNsResolver`*"]
    pub fn create_expression_with_opt_x_path_ns_resolver(
        this: &Document,
        expression: &str,
        resolver: Option<&XPathNsResolver>,
    ) -> Result<XPathExpression, JsValue>;
    #[wasm_bindgen(method, js_class = "Document", js_name = "createNSResolver")]
    #[doc = "The `createNSResolver()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/createNSResolver)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    pub fn create_ns_resolver(this: &Document, node_resolver: &Node) -> Node;
    #[cfg(feature = "XPathResult")]
    #[wasm_bindgen(catch, method, js_class = "Document")]
    #[doc = "The `evaluate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/evaluate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `XPathResult`*"]
    pub fn evaluate(
        this: &Document,
        expression: &str,
        context_node: &Node,
    ) -> Result<XPathResult, JsValue>;
    #[cfg(feature = "XPathResult")]
    #[wasm_bindgen(catch, method, js_class = "Document", js_name = "evaluate")]
    #[doc = "The `evaluate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/evaluate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `XPathResult`*"]
    pub fn evaluate_with_opt_callback(
        this: &Document,
        expression: &str,
        context_node: &Node,
        resolver: Option<&::js_sys::Function>,
    ) -> Result<XPathResult, JsValue>;
    #[cfg(all(feature = "XPathNsResolver", feature = "XPathResult",))]
    #[wasm_bindgen(catch, method, js_class = "Document", js_name = "evaluate")]
    #[doc = "The `evaluate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/evaluate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `XPathNsResolver`, `XPathResult`*"]
    pub fn evaluate_with_opt_x_path_ns_resolver(
        this: &Document,
        expression: &str,
        context_node: &Node,
        resolver: Option<&XPathNsResolver>,
    ) -> Result<XPathResult, JsValue>;
    #[cfg(feature = "XPathResult")]
    #[wasm_bindgen(catch, method, js_class = "Document", js_name = "evaluate")]
    #[doc = "The `evaluate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/evaluate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `XPathResult`*"]
    pub fn evaluate_with_opt_callback_and_type(
        this: &Document,
        expression: &str,
        context_node: &Node,
        resolver: Option<&::js_sys::Function>,
        type_: u16,
    ) -> Result<XPathResult, JsValue>;
    #[cfg(all(feature = "XPathNsResolver", feature = "XPathResult",))]
    #[wasm_bindgen(catch, method, js_class = "Document", js_name = "evaluate")]
    #[doc = "The `evaluate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/evaluate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `XPathNsResolver`, `XPathResult`*"]
    pub fn evaluate_with_opt_x_path_ns_resolver_and_type(
        this: &Document,
        expression: &str,
        context_node: &Node,
        resolver: Option<&XPathNsResolver>,
        type_: u16,
    ) -> Result<XPathResult, JsValue>;
    #[cfg(feature = "XPathResult")]
    #[wasm_bindgen(catch, method, js_class = "Document", js_name = "evaluate")]
    #[doc = "The `evaluate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/evaluate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `XPathResult`*"]
    pub fn evaluate_with_opt_callback_and_type_and_result(
        this: &Document,
        expression: &str,
        context_node: &Node,
        resolver: Option<&::js_sys::Function>,
        type_: u16,
        result: Option<&::js_sys::Object>,
    ) -> Result<XPathResult, JsValue>;
    #[cfg(all(feature = "XPathNsResolver", feature = "XPathResult",))]
    #[wasm_bindgen(catch, method, js_class = "Document", js_name = "evaluate")]
    #[doc = "The `evaluate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/evaluate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `XPathNsResolver`, `XPathResult`*"]
    pub fn evaluate_with_opt_x_path_ns_resolver_and_type_and_result(
        this: &Document,
        expression: &str,
        context_node: &Node,
        resolver: Option<&XPathNsResolver>,
        type_: u16,
        result: Option<&::js_sys::Object>,
    ) -> Result<XPathResult, JsValue>;
}
