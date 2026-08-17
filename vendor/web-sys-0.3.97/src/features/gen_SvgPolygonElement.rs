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
        js_name = "SVGPolygonElement",
        typescript_type = "SVGPolygonElement"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgPolygonElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPolygonElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPolygonElement`*"]
    pub type SvgPolygonElement;
    #[cfg(feature = "SvgPointList")]
    #[wasm_bindgen(method, getter, js_class = "SVGPolygonElement", js_name = "points")]
    #[doc = "Getter for the `points` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPolygonElement/points)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPointList`, `SvgPolygonElement`*"]
    pub fn points(this: &SvgPolygonElement) -> SvgPointList;
    #[cfg(feature = "SvgPointList")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGPolygonElement",
        js_name = "animatedPoints"
    )]
    #[doc = "Getter for the `animatedPoints` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPolygonElement/animatedPoints)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPointList`, `SvgPolygonElement`*"]
    pub fn animated_points(this: &SvgPolygonElement) -> SvgPointList;
}
