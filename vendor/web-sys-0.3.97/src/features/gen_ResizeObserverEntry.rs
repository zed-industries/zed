#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "ResizeObserverEntry",
        typescript_type = "ResizeObserverEntry"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ResizeObserverEntry` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ResizeObserverEntry)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ResizeObserverEntry`*"]
    pub type ResizeObserverEntry;
    #[cfg(feature = "Element")]
    #[wasm_bindgen(method, getter, js_class = "ResizeObserverEntry", js_name = "target")]
    #[doc = "Getter for the `target` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ResizeObserverEntry/target)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`, `ResizeObserverEntry`*"]
    pub fn target(this: &ResizeObserverEntry) -> Element;
    #[cfg(feature = "DomRectReadOnly")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "ResizeObserverEntry",
        js_name = "contentRect"
    )]
    #[doc = "Getter for the `contentRect` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ResizeObserverEntry/contentRect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRectReadOnly`, `ResizeObserverEntry`*"]
    pub fn content_rect(this: &ResizeObserverEntry) -> DomRectReadOnly;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "ResizeObserverEntry",
        js_name = "borderBoxSize"
    )]
    #[doc = "Getter for the `borderBoxSize` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ResizeObserverEntry/borderBoxSize)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ResizeObserverEntry`*"]
    pub fn border_box_size(this: &ResizeObserverEntry) -> ::js_sys::Array;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "ResizeObserverEntry",
        js_name = "contentBoxSize"
    )]
    #[doc = "Getter for the `contentBoxSize` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ResizeObserverEntry/contentBoxSize)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ResizeObserverEntry`*"]
    pub fn content_box_size(this: &ResizeObserverEntry) -> ::js_sys::Array;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "ResizeObserverEntry",
        js_name = "devicePixelContentBoxSize"
    )]
    #[doc = "Getter for the `devicePixelContentBoxSize` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ResizeObserverEntry/devicePixelContentBoxSize)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ResizeObserverEntry`*"]
    pub fn device_pixel_content_box_size(this: &ResizeObserverEntry) -> ::js_sys::Array;
}
