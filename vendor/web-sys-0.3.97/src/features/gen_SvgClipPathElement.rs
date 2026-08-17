#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "SvgElement",
        extends = "Element",
        extends = "Node",
        extends = "EventTarget",
        extends = "::js_sys::Object",
        js_name = "SVGClipPathElement",
        typescript_type = "SVGClipPathElement"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgClipPathElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGClipPathElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgClipPathElement`*"]
    pub type SvgClipPathElement;
    #[cfg(feature = "SvgAnimatedEnumeration")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGClipPathElement",
        js_name = "clipPathUnits"
    )]
    #[doc = "Getter for the `clipPathUnits` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGClipPathElement/clipPathUnits)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedEnumeration`, `SvgClipPathElement`*"]
    pub fn clip_path_units(this: &SvgClipPathElement) -> SvgAnimatedEnumeration;
    #[cfg(feature = "SvgAnimatedTransformList")]
    #[wasm_bindgen(method, getter, js_class = "SVGClipPathElement", js_name = "transform")]
    #[doc = "Getter for the `transform` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGClipPathElement/transform)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedTransformList`, `SvgClipPathElement`*"]
    pub fn transform(this: &SvgClipPathElement) -> SvgAnimatedTransformList;
}
