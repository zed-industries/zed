#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "EventTarget",
        extends = "::js_sys::Object",
        js_name = "VisualViewport",
        typescript_type = "VisualViewport"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `VisualViewport` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VisualViewport)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VisualViewport`*"]
    pub type VisualViewport;
    #[wasm_bindgen(method, getter, js_class = "VisualViewport", js_name = "offsetLeft")]
    #[doc = "Getter for the `offsetLeft` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VisualViewport/offsetLeft)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VisualViewport`*"]
    pub fn offset_left(this: &VisualViewport) -> f64;
    #[wasm_bindgen(method, getter, js_class = "VisualViewport", js_name = "offsetTop")]
    #[doc = "Getter for the `offsetTop` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VisualViewport/offsetTop)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VisualViewport`*"]
    pub fn offset_top(this: &VisualViewport) -> f64;
    #[wasm_bindgen(method, getter, js_class = "VisualViewport", js_name = "pageLeft")]
    #[doc = "Getter for the `pageLeft` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VisualViewport/pageLeft)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VisualViewport`*"]
    pub fn page_left(this: &VisualViewport) -> f64;
    #[wasm_bindgen(method, getter, js_class = "VisualViewport", js_name = "pageTop")]
    #[doc = "Getter for the `pageTop` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VisualViewport/pageTop)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VisualViewport`*"]
    pub fn page_top(this: &VisualViewport) -> f64;
    #[wasm_bindgen(method, getter, js_class = "VisualViewport", js_name = "width")]
    #[doc = "Getter for the `width` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VisualViewport/width)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VisualViewport`*"]
    pub fn width(this: &VisualViewport) -> f64;
    #[wasm_bindgen(method, getter, js_class = "VisualViewport", js_name = "height")]
    #[doc = "Getter for the `height` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VisualViewport/height)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VisualViewport`*"]
    pub fn height(this: &VisualViewport) -> f64;
    #[wasm_bindgen(method, getter, js_class = "VisualViewport", js_name = "scale")]
    #[doc = "Getter for the `scale` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VisualViewport/scale)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VisualViewport`*"]
    pub fn scale(this: &VisualViewport) -> f64;
    #[wasm_bindgen(method, getter, js_class = "VisualViewport", js_name = "onresize")]
    #[doc = "Getter for the `onresize` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VisualViewport/onresize)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VisualViewport`*"]
    pub fn onresize(this: &VisualViewport) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "VisualViewport", js_name = "onresize")]
    #[doc = "Setter for the `onresize` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VisualViewport/onresize)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VisualViewport`*"]
    pub fn set_onresize(this: &VisualViewport, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "VisualViewport", js_name = "onscroll")]
    #[doc = "Getter for the `onscroll` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VisualViewport/onscroll)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VisualViewport`*"]
    pub fn onscroll(this: &VisualViewport) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "VisualViewport", js_name = "onscroll")]
    #[doc = "Setter for the `onscroll` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VisualViewport/onscroll)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VisualViewport`*"]
    pub fn set_onscroll(this: &VisualViewport, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "VisualViewport", js_name = "onscrollend")]
    #[doc = "Getter for the `onscrollend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VisualViewport/onscrollend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VisualViewport`*"]
    pub fn onscrollend(this: &VisualViewport) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "VisualViewport", js_name = "onscrollend")]
    #[doc = "Setter for the `onscrollend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VisualViewport/onscrollend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VisualViewport`*"]
    pub fn set_onscrollend(this: &VisualViewport, value: Option<&::js_sys::Function>);
}
