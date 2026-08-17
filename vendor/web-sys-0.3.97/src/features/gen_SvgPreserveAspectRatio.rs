#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "SVGPreserveAspectRatio",
        typescript_type = "SVGPreserveAspectRatio"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgPreserveAspectRatio` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPreserveAspectRatio)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPreserveAspectRatio`*"]
    pub type SvgPreserveAspectRatio;
    #[wasm_bindgen(method, getter, js_class = "SVGPreserveAspectRatio", js_name = "align")]
    #[doc = "Getter for the `align` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPreserveAspectRatio/align)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPreserveAspectRatio`*"]
    pub fn align(this: &SvgPreserveAspectRatio) -> u16;
    #[wasm_bindgen(method, setter, js_class = "SVGPreserveAspectRatio", js_name = "align")]
    #[doc = "Setter for the `align` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPreserveAspectRatio/align)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPreserveAspectRatio`*"]
    pub fn set_align(this: &SvgPreserveAspectRatio, value: u16);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGPreserveAspectRatio",
        js_name = "meetOrSlice"
    )]
    #[doc = "Getter for the `meetOrSlice` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPreserveAspectRatio/meetOrSlice)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPreserveAspectRatio`*"]
    pub fn meet_or_slice(this: &SvgPreserveAspectRatio) -> u16;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "SVGPreserveAspectRatio",
        js_name = "meetOrSlice"
    )]
    #[doc = "Setter for the `meetOrSlice` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPreserveAspectRatio/meetOrSlice)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPreserveAspectRatio`*"]
    pub fn set_meet_or_slice(this: &SvgPreserveAspectRatio, value: u16);
}
impl SvgPreserveAspectRatio {
    #[doc = "The `SVGPreserveAspectRatio.SVG_PRESERVEASPECTRATIO_UNKNOWN` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPreserveAspectRatio`*"]
    pub const SVG_PRESERVEASPECTRATIO_UNKNOWN: u16 = 0i64 as u16;
    #[doc = "The `SVGPreserveAspectRatio.SVG_PRESERVEASPECTRATIO_NONE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPreserveAspectRatio`*"]
    pub const SVG_PRESERVEASPECTRATIO_NONE: u16 = 1u64 as u16;
    #[doc = "The `SVGPreserveAspectRatio.SVG_PRESERVEASPECTRATIO_XMINYMIN` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPreserveAspectRatio`*"]
    pub const SVG_PRESERVEASPECTRATIO_XMINYMIN: u16 = 2u64 as u16;
    #[doc = "The `SVGPreserveAspectRatio.SVG_PRESERVEASPECTRATIO_XMIDYMIN` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPreserveAspectRatio`*"]
    pub const SVG_PRESERVEASPECTRATIO_XMIDYMIN: u16 = 3u64 as u16;
    #[doc = "The `SVGPreserveAspectRatio.SVG_PRESERVEASPECTRATIO_XMAXYMIN` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPreserveAspectRatio`*"]
    pub const SVG_PRESERVEASPECTRATIO_XMAXYMIN: u16 = 4u64 as u16;
    #[doc = "The `SVGPreserveAspectRatio.SVG_PRESERVEASPECTRATIO_XMINYMID` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPreserveAspectRatio`*"]
    pub const SVG_PRESERVEASPECTRATIO_XMINYMID: u16 = 5u64 as u16;
    #[doc = "The `SVGPreserveAspectRatio.SVG_PRESERVEASPECTRATIO_XMIDYMID` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPreserveAspectRatio`*"]
    pub const SVG_PRESERVEASPECTRATIO_XMIDYMID: u16 = 6u64 as u16;
    #[doc = "The `SVGPreserveAspectRatio.SVG_PRESERVEASPECTRATIO_XMAXYMID` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPreserveAspectRatio`*"]
    pub const SVG_PRESERVEASPECTRATIO_XMAXYMID: u16 = 7u64 as u16;
    #[doc = "The `SVGPreserveAspectRatio.SVG_PRESERVEASPECTRATIO_XMINYMAX` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPreserveAspectRatio`*"]
    pub const SVG_PRESERVEASPECTRATIO_XMINYMAX: u16 = 8u64 as u16;
    #[doc = "The `SVGPreserveAspectRatio.SVG_PRESERVEASPECTRATIO_XMIDYMAX` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPreserveAspectRatio`*"]
    pub const SVG_PRESERVEASPECTRATIO_XMIDYMAX: u16 = 9u64 as u16;
    #[doc = "The `SVGPreserveAspectRatio.SVG_PRESERVEASPECTRATIO_XMAXYMAX` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPreserveAspectRatio`*"]
    pub const SVG_PRESERVEASPECTRATIO_XMAXYMAX: u16 = 10u64 as u16;
    #[doc = "The `SVGPreserveAspectRatio.SVG_MEETORSLICE_UNKNOWN` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPreserveAspectRatio`*"]
    pub const SVG_MEETORSLICE_UNKNOWN: u16 = 0i64 as u16;
    #[doc = "The `SVGPreserveAspectRatio.SVG_MEETORSLICE_MEET` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPreserveAspectRatio`*"]
    pub const SVG_MEETORSLICE_MEET: u16 = 1u64 as u16;
    #[doc = "The `SVGPreserveAspectRatio.SVG_MEETORSLICE_SLICE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPreserveAspectRatio`*"]
    pub const SVG_MEETORSLICE_SLICE: u16 = 2u64 as u16;
}
