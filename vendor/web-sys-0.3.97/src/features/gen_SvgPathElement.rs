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
        js_name = "SVGPathElement",
        typescript_type = "SVGPathElement"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgPathElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPathElement`*"]
    pub type SvgPathElement;
    #[cfg(feature = "SvgPathSegList")]
    #[wasm_bindgen(method, getter, js_class = "SVGPathElement", js_name = "pathSegList")]
    #[doc = "Getter for the `pathSegList` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathElement/pathSegList)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPathElement`, `SvgPathSegList`*"]
    pub fn path_seg_list(this: &SvgPathElement) -> SvgPathSegList;
    #[cfg(feature = "SvgPathSegList")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGPathElement",
        js_name = "animatedPathSegList"
    )]
    #[doc = "Getter for the `animatedPathSegList` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathElement/animatedPathSegList)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPathElement`, `SvgPathSegList`*"]
    pub fn animated_path_seg_list(this: &SvgPathElement) -> SvgPathSegList;
    #[wasm_bindgen(method, js_class = "SVGPathElement", js_name = "getPathSegAtLength")]
    #[doc = "The `getPathSegAtLength()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathElement/getPathSegAtLength)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPathElement`*"]
    pub fn get_path_seg_at_length(this: &SvgPathElement, distance: f32) -> u32;
}
