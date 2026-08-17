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
        js_name = "HTMLTrackElement",
        typescript_type = "HTMLTrackElement"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlTrackElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTrackElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTrackElement`*"]
    pub type HtmlTrackElement;
    #[wasm_bindgen(method, getter, js_class = "HTMLTrackElement", js_name = "kind")]
    #[doc = "Getter for the `kind` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTrackElement/kind)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTrackElement`*"]
    pub fn kind(this: &HtmlTrackElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLTrackElement", js_name = "kind")]
    #[doc = "Setter for the `kind` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTrackElement/kind)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTrackElement`*"]
    pub fn set_kind(this: &HtmlTrackElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLTrackElement", js_name = "src")]
    #[doc = "Getter for the `src` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTrackElement/src)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTrackElement`*"]
    pub fn src(this: &HtmlTrackElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLTrackElement", js_name = "src")]
    #[doc = "Setter for the `src` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTrackElement/src)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTrackElement`*"]
    pub fn set_src(this: &HtmlTrackElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLTrackElement", js_name = "srclang")]
    #[doc = "Getter for the `srclang` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTrackElement/srclang)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTrackElement`*"]
    pub fn srclang(this: &HtmlTrackElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLTrackElement", js_name = "srclang")]
    #[doc = "Setter for the `srclang` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTrackElement/srclang)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTrackElement`*"]
    pub fn set_srclang(this: &HtmlTrackElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLTrackElement", js_name = "label")]
    #[doc = "Getter for the `label` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTrackElement/label)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTrackElement`*"]
    pub fn label(this: &HtmlTrackElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLTrackElement", js_name = "label")]
    #[doc = "Setter for the `label` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTrackElement/label)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTrackElement`*"]
    pub fn set_label(this: &HtmlTrackElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLTrackElement", js_name = "default")]
    #[doc = "Getter for the `default` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTrackElement/default)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTrackElement`*"]
    pub fn default(this: &HtmlTrackElement) -> bool;
    #[wasm_bindgen(method, setter, js_class = "HTMLTrackElement", js_name = "default")]
    #[doc = "Setter for the `default` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTrackElement/default)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTrackElement`*"]
    pub fn set_default(this: &HtmlTrackElement, value: bool);
    #[wasm_bindgen(method, getter, js_class = "HTMLTrackElement", js_name = "readyState")]
    #[doc = "Getter for the `readyState` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTrackElement/readyState)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTrackElement`*"]
    pub fn ready_state(this: &HtmlTrackElement) -> u16;
    #[cfg(feature = "TextTrack")]
    #[wasm_bindgen(method, getter, js_class = "HTMLTrackElement", js_name = "track")]
    #[doc = "Getter for the `track` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTrackElement/track)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTrackElement`, `TextTrack`*"]
    pub fn track(this: &HtmlTrackElement) -> Option<TextTrack>;
}
impl HtmlTrackElement {
    #[doc = "The `HTMLTrackElement.NONE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTrackElement`*"]
    pub const NONE: u16 = 0i64 as u16;
    #[doc = "The `HTMLTrackElement.LOADING` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTrackElement`*"]
    pub const LOADING: u16 = 1u64 as u16;
    #[doc = "The `HTMLTrackElement.LOADED` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTrackElement`*"]
    pub const LOADED: u16 = 2u64 as u16;
    #[doc = "The `HTMLTrackElement.ERROR` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTrackElement`*"]
    pub const ERROR: u16 = 3u64 as u16;
}
