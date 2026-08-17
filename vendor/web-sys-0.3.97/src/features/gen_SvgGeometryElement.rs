#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "SvgGraphicsElement",
        extends = "SvgElement",
        extends = "Element",
        extends = "Node",
        extends = "EventTarget",
        extends = "::js_sys::Object",
        js_name = "SVGGeometryElement",
        typescript_type = "SVGGeometryElement"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgGeometryElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGGeometryElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgGeometryElement`*"]
    pub type SvgGeometryElement;
    #[cfg(feature = "SvgAnimatedNumber")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGGeometryElement",
        js_name = "pathLength"
    )]
    #[doc = "Getter for the `pathLength` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGGeometryElement/pathLength)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgGeometryElement`*"]
    pub fn path_length(this: &SvgGeometryElement) -> SvgAnimatedNumber;
    #[cfg(feature = "SvgPoint")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "SVGGeometryElement",
        js_name = "getPointAtLength"
    )]
    #[doc = "The `getPointAtLength()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGGeometryElement/getPointAtLength)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgGeometryElement`, `SvgPoint`*"]
    pub fn get_point_at_length(
        this: &SvgGeometryElement,
        distance: f32,
    ) -> Result<SvgPoint, JsValue>;
    #[wasm_bindgen(method, js_class = "SVGGeometryElement", js_name = "getTotalLength")]
    #[doc = "The `getTotalLength()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGGeometryElement/getTotalLength)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgGeometryElement`*"]
    pub fn get_total_length(this: &SvgGeometryElement) -> f32;
    #[wasm_bindgen(method, js_class = "SVGGeometryElement", js_name = "isPointInFill")]
    #[doc = "The `isPointInFill()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGGeometryElement/isPointInFill)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgGeometryElement`*"]
    pub fn is_point_in_fill(this: &SvgGeometryElement) -> bool;
    #[cfg(feature = "DomPointInit")]
    #[wasm_bindgen(method, js_class = "SVGGeometryElement", js_name = "isPointInFill")]
    #[doc = "The `isPointInFill()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGGeometryElement/isPointInFill)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPointInit`, `SvgGeometryElement`*"]
    pub fn is_point_in_fill_with_point(this: &SvgGeometryElement, point: &DomPointInit) -> bool;
    #[wasm_bindgen(method, js_class = "SVGGeometryElement", js_name = "isPointInStroke")]
    #[doc = "The `isPointInStroke()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGGeometryElement/isPointInStroke)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgGeometryElement`*"]
    pub fn is_point_in_stroke(this: &SvgGeometryElement) -> bool;
    #[cfg(feature = "DomPointInit")]
    #[wasm_bindgen(method, js_class = "SVGGeometryElement", js_name = "isPointInStroke")]
    #[doc = "The `isPointInStroke()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGGeometryElement/isPointInStroke)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPointInit`, `SvgGeometryElement`*"]
    pub fn is_point_in_stroke_with_point(this: &SvgGeometryElement, point: &DomPointInit) -> bool;
}
