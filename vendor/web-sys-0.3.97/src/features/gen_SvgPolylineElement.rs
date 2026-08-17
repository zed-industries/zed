#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "SvgGeometryElement",
        extends = "SvgGraphicsElement",
        extends = "SvgElement",
        extends = "Element",
        extends = "Node",
        extends = "EventTarget",
        extends = "::js_sys::Object",
        js_name = "SVGPolylineElement",
        typescript_type = "SVGPolylineElement"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgPolylineElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPolylineElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPolylineElement`*"]
    pub type SvgPolylineElement;
    #[cfg(feature = "SvgPointList")]
    #[wasm_bindgen(method, getter, js_class = "SVGPolylineElement", js_name = "points")]
    #[doc = "Getter for the `points` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPolylineElement/points)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPointList`, `SvgPolylineElement`*"]
    pub fn points(this: &SvgPolylineElement) -> SvgPointList;
    #[cfg(feature = "SvgPointList")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGPolylineElement",
        js_name = "animatedPoints"
    )]
    #[doc = "Getter for the `animatedPoints` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPolylineElement/animatedPoints)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPointList`, `SvgPolylineElement`*"]
    pub fn animated_points(this: &SvgPolylineElement) -> SvgPointList;
}
