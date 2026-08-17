#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "DocumentFragment",
        extends = "Node",
        extends = "EventTarget",
        extends = "::js_sys::Object",
        js_name = "ShadowRoot",
        typescript_type = "ShadowRoot"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ShadowRoot` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ShadowRoot)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ShadowRoot`*"]
    pub type ShadowRoot;
    #[cfg(feature = "ShadowRootMode")]
    #[wasm_bindgen(method, getter, js_class = "ShadowRoot", js_name = "mode")]
    #[doc = "Getter for the `mode` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ShadowRoot/mode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ShadowRoot`, `ShadowRootMode`*"]
    pub fn mode(this: &ShadowRoot) -> ShadowRootMode;
    #[cfg(feature = "Element")]
    #[wasm_bindgen(method, getter, js_class = "ShadowRoot", js_name = "host")]
    #[doc = "Getter for the `host` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ShadowRoot/host)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`, `ShadowRoot`*"]
    pub fn host(this: &ShadowRoot) -> Element;
    #[wasm_bindgen(method, getter, js_class = "ShadowRoot", js_name = "innerHTML")]
    #[doc = "Getter for the `innerHTML` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ShadowRoot/innerHTML)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ShadowRoot`*"]
    pub fn inner_html(this: &ShadowRoot) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "ShadowRoot", js_name = "innerHTML")]
    #[doc = "Setter for the `innerHTML` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ShadowRoot/innerHTML)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ShadowRoot`*"]
    pub fn set_inner_html(this: &ShadowRoot, value: &str);
    #[cfg(feature = "Element")]
    #[wasm_bindgen(method, getter, js_class = "ShadowRoot", js_name = "activeElement")]
    #[doc = "Getter for the `activeElement` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ShadowRoot/activeElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`, `ShadowRoot`*"]
    pub fn active_element(this: &ShadowRoot) -> Option<Element>;
    #[cfg(feature = "Element")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "ShadowRoot",
        js_name = "pointerLockElement"
    )]
    #[doc = "Getter for the `pointerLockElement` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ShadowRoot/pointerLockElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`, `ShadowRoot`*"]
    pub fn pointer_lock_element(this: &ShadowRoot) -> Option<Element>;
    #[cfg(feature = "Element")]
    #[wasm_bindgen(method, getter, js_class = "ShadowRoot", js_name = "fullscreenElement")]
    #[doc = "Getter for the `fullscreenElement` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ShadowRoot/fullscreenElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`, `ShadowRoot`*"]
    pub fn fullscreen_element(this: &ShadowRoot) -> Option<Element>;
    #[cfg(feature = "StyleSheetList")]
    #[wasm_bindgen(method, getter, js_class = "ShadowRoot", js_name = "styleSheets")]
    #[doc = "Getter for the `styleSheets` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ShadowRoot/styleSheets)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ShadowRoot`, `StyleSheetList`*"]
    pub fn style_sheets(this: &ShadowRoot) -> StyleSheetList;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "ShadowRoot",
        js_name = "adoptedStyleSheets"
    )]
    #[doc = "Getter for the `adoptedStyleSheets` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ShadowRoot/adoptedStyleSheets)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ShadowRoot`*"]
    pub fn adopted_style_sheets(this: &ShadowRoot) -> ::js_sys::Array;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "ShadowRoot",
        js_name = "adoptedStyleSheets"
    )]
    #[doc = "Setter for the `adoptedStyleSheets` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ShadowRoot/adoptedStyleSheets)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ShadowRoot`*"]
    pub fn set_adopted_style_sheets(this: &ShadowRoot, value: &::wasm_bindgen::JsValue);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "Element")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "ShadowRoot",
        js_name = "pictureInPictureElement"
    )]
    #[doc = "Getter for the `pictureInPictureElement` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ShadowRoot/pictureInPictureElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`, `ShadowRoot`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn picture_in_picture_element(this: &ShadowRoot) -> Option<Element>;
    #[cfg(feature = "Element")]
    #[wasm_bindgen(method, js_class = "ShadowRoot", js_name = "getElementById")]
    #[doc = "The `getElementById()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ShadowRoot/getElementById)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`, `ShadowRoot`*"]
    pub fn get_element_by_id(this: &ShadowRoot, element_id: &str) -> Option<Element>;
    #[cfg(feature = "HtmlCollection")]
    #[wasm_bindgen(method, js_class = "ShadowRoot", js_name = "getElementsByClassName")]
    #[doc = "The `getElementsByClassName()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ShadowRoot/getElementsByClassName)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlCollection`, `ShadowRoot`*"]
    pub fn get_elements_by_class_name(this: &ShadowRoot, class_names: &str) -> HtmlCollection;
    #[cfg(feature = "HtmlCollection")]
    #[wasm_bindgen(method, js_class = "ShadowRoot", js_name = "getElementsByTagName")]
    #[doc = "The `getElementsByTagName()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ShadowRoot/getElementsByTagName)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlCollection`, `ShadowRoot`*"]
    pub fn get_elements_by_tag_name(this: &ShadowRoot, local_name: &str) -> HtmlCollection;
    #[cfg(feature = "HtmlCollection")]
    #[wasm_bindgen(method, js_class = "ShadowRoot", js_name = "getElementsByTagNameNS")]
    #[doc = "The `getElementsByTagNameNS()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ShadowRoot/getElementsByTagNameNS)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlCollection`, `ShadowRoot`*"]
    pub fn get_elements_by_tag_name_ns(
        this: &ShadowRoot,
        namespace: Option<&str>,
        local_name: &str,
    ) -> HtmlCollection;
    #[cfg(feature = "Element")]
    #[wasm_bindgen(method, js_class = "ShadowRoot", js_name = "elementFromPoint")]
    #[doc = "The `elementFromPoint()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ShadowRoot/elementFromPoint)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`, `ShadowRoot`*"]
    pub fn element_from_point(this: &ShadowRoot, x: f32, y: f32) -> Option<Element>;
    #[wasm_bindgen(method, js_class = "ShadowRoot", js_name = "elementsFromPoint")]
    #[doc = "The `elementsFromPoint()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ShadowRoot/elementsFromPoint)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ShadowRoot`*"]
    pub fn elements_from_point(this: &ShadowRoot, x: f32, y: f32) -> ::js_sys::Array;
}
