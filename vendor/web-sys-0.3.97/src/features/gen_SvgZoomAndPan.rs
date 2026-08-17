#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "SVGZoomAndPan",
        typescript_type = "SVGZoomAndPan"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgZoomAndPan` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGZoomAndPan)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgZoomAndPan`*"]
    pub type SvgZoomAndPan;
    #[wasm_bindgen(method, getter, js_class = "SVGZoomAndPan", js_name = "zoomAndPan")]
    #[doc = "Getter for the `zoomAndPan` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGZoomAndPan/zoomAndPan)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgZoomAndPan`*"]
    pub fn zoom_and_pan(this: &SvgZoomAndPan) -> u16;
    #[wasm_bindgen(method, setter, js_class = "SVGZoomAndPan", js_name = "zoomAndPan")]
    #[doc = "Setter for the `zoomAndPan` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGZoomAndPan/zoomAndPan)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgZoomAndPan`*"]
    pub fn set_zoom_and_pan(this: &SvgZoomAndPan, value: u16);
}
impl SvgZoomAndPan {
    #[doc = "The `SVGZoomAndPan.SVG_ZOOMANDPAN_UNKNOWN` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgZoomAndPan`*"]
    pub const SVG_ZOOMANDPAN_UNKNOWN: u16 = 0i64 as u16;
    #[doc = "The `SVGZoomAndPan.SVG_ZOOMANDPAN_DISABLE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgZoomAndPan`*"]
    pub const SVG_ZOOMANDPAN_DISABLE: u16 = 1u64 as u16;
    #[doc = "The `SVGZoomAndPan.SVG_ZOOMANDPAN_MAGNIFY` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgZoomAndPan`*"]
    pub const SVG_ZOOMANDPAN_MAGNIFY: u16 = 2u64 as u16;
}
