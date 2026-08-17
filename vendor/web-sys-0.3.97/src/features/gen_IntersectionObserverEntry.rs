#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "IntersectionObserverEntry",
        typescript_type = "IntersectionObserverEntry"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `IntersectionObserverEntry` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IntersectionObserverEntry)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IntersectionObserverEntry`*"]
    pub type IntersectionObserverEntry;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "IntersectionObserverEntry",
        js_name = "time"
    )]
    #[doc = "Getter for the `time` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IntersectionObserverEntry/time)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IntersectionObserverEntry`*"]
    pub fn time(this: &IntersectionObserverEntry) -> f64;
    #[cfg(feature = "DomRectReadOnly")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "IntersectionObserverEntry",
        js_name = "rootBounds"
    )]
    #[doc = "Getter for the `rootBounds` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IntersectionObserverEntry/rootBounds)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRectReadOnly`, `IntersectionObserverEntry`*"]
    pub fn root_bounds(this: &IntersectionObserverEntry) -> Option<DomRectReadOnly>;
    #[cfg(feature = "DomRectReadOnly")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "IntersectionObserverEntry",
        js_name = "boundingClientRect"
    )]
    #[doc = "Getter for the `boundingClientRect` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IntersectionObserverEntry/boundingClientRect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRectReadOnly`, `IntersectionObserverEntry`*"]
    pub fn bounding_client_rect(this: &IntersectionObserverEntry) -> DomRectReadOnly;
    #[cfg(feature = "DomRectReadOnly")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "IntersectionObserverEntry",
        js_name = "intersectionRect"
    )]
    #[doc = "Getter for the `intersectionRect` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IntersectionObserverEntry/intersectionRect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRectReadOnly`, `IntersectionObserverEntry`*"]
    pub fn intersection_rect(this: &IntersectionObserverEntry) -> DomRectReadOnly;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "IntersectionObserverEntry",
        js_name = "isIntersecting"
    )]
    #[doc = "Getter for the `isIntersecting` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IntersectionObserverEntry/isIntersecting)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IntersectionObserverEntry`*"]
    pub fn is_intersecting(this: &IntersectionObserverEntry) -> bool;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "IntersectionObserverEntry",
        js_name = "intersectionRatio"
    )]
    #[doc = "Getter for the `intersectionRatio` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IntersectionObserverEntry/intersectionRatio)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IntersectionObserverEntry`*"]
    pub fn intersection_ratio(this: &IntersectionObserverEntry) -> f64;
    #[cfg(feature = "Element")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "IntersectionObserverEntry",
        js_name = "target"
    )]
    #[doc = "Getter for the `target` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IntersectionObserverEntry/target)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`, `IntersectionObserverEntry`*"]
    pub fn target(this: &IntersectionObserverEntry) -> Element;
}
