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
        js_name = "SVGViewElement",
        typescript_type = "SVGViewElement"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgViewElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGViewElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgViewElement`*"]
    pub type SvgViewElement;
    #[cfg(feature = "SvgAnimatedRect")]
    #[wasm_bindgen(method, getter, js_class = "SVGViewElement", js_name = "viewBox")]
    #[doc = "Getter for the `viewBox` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGViewElement/viewBox)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedRect`, `SvgViewElement`*"]
    pub fn view_box(this: &SvgViewElement) -> SvgAnimatedRect;
    #[cfg(feature = "SvgAnimatedPreserveAspectRatio")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGViewElement",
        js_name = "preserveAspectRatio"
    )]
    #[doc = "Getter for the `preserveAspectRatio` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGViewElement/preserveAspectRatio)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedPreserveAspectRatio`, `SvgViewElement`*"]
    pub fn preserve_aspect_ratio(this: &SvgViewElement) -> SvgAnimatedPreserveAspectRatio;
    #[wasm_bindgen(method, getter, js_class = "SVGViewElement", js_name = "zoomAndPan")]
    #[doc = "Getter for the `zoomAndPan` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGViewElement/zoomAndPan)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgViewElement`*"]
    pub fn zoom_and_pan(this: &SvgViewElement) -> u16;
    #[wasm_bindgen(method, setter, js_class = "SVGViewElement", js_name = "zoomAndPan")]
    #[doc = "Setter for the `zoomAndPan` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGViewElement/zoomAndPan)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgViewElement`*"]
    pub fn set_zoom_and_pan(this: &SvgViewElement, value: u16);
}
impl SvgViewElement {
    #[doc = "The `SVGViewElement.SVG_ZOOMANDPAN_UNKNOWN` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgViewElement`*"]
    pub const SVG_ZOOMANDPAN_UNKNOWN: u16 = 0i64 as u16;
    #[doc = "The `SVGViewElement.SVG_ZOOMANDPAN_DISABLE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgViewElement`*"]
    pub const SVG_ZOOMANDPAN_DISABLE: u16 = 1u64 as u16;
    #[doc = "The `SVGViewElement.SVG_ZOOMANDPAN_MAGNIFY` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgViewElement`*"]
    pub const SVG_ZOOMANDPAN_MAGNIFY: u16 = 2u64 as u16;
}
