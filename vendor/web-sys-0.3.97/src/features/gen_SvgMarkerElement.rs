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
        js_name = "SVGMarkerElement",
        typescript_type = "SVGMarkerElement"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgMarkerElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMarkerElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgMarkerElement`*"]
    pub type SvgMarkerElement;
    #[cfg(feature = "SvgAnimatedLength")]
    #[wasm_bindgen(method, getter, js_class = "SVGMarkerElement", js_name = "refX")]
    #[doc = "Getter for the `refX` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMarkerElement/refX)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgMarkerElement`*"]
    pub fn ref_x(this: &SvgMarkerElement) -> SvgAnimatedLength;
    #[cfg(feature = "SvgAnimatedLength")]
    #[wasm_bindgen(method, getter, js_class = "SVGMarkerElement", js_name = "refY")]
    #[doc = "Getter for the `refY` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMarkerElement/refY)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgMarkerElement`*"]
    pub fn ref_y(this: &SvgMarkerElement) -> SvgAnimatedLength;
    #[cfg(feature = "SvgAnimatedEnumeration")]
    #[wasm_bindgen(method, getter, js_class = "SVGMarkerElement", js_name = "markerUnits")]
    #[doc = "Getter for the `markerUnits` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMarkerElement/markerUnits)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedEnumeration`, `SvgMarkerElement`*"]
    pub fn marker_units(this: &SvgMarkerElement) -> SvgAnimatedEnumeration;
    #[cfg(feature = "SvgAnimatedLength")]
    #[wasm_bindgen(method, getter, js_class = "SVGMarkerElement", js_name = "markerWidth")]
    #[doc = "Getter for the `markerWidth` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMarkerElement/markerWidth)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgMarkerElement`*"]
    pub fn marker_width(this: &SvgMarkerElement) -> SvgAnimatedLength;
    #[cfg(feature = "SvgAnimatedLength")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGMarkerElement",
        js_name = "markerHeight"
    )]
    #[doc = "Getter for the `markerHeight` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMarkerElement/markerHeight)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgMarkerElement`*"]
    pub fn marker_height(this: &SvgMarkerElement) -> SvgAnimatedLength;
    #[cfg(feature = "SvgAnimatedEnumeration")]
    #[wasm_bindgen(method, getter, js_class = "SVGMarkerElement", js_name = "orientType")]
    #[doc = "Getter for the `orientType` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMarkerElement/orientType)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedEnumeration`, `SvgMarkerElement`*"]
    pub fn orient_type(this: &SvgMarkerElement) -> SvgAnimatedEnumeration;
    #[cfg(feature = "SvgAnimatedAngle")]
    #[wasm_bindgen(method, getter, js_class = "SVGMarkerElement", js_name = "orientAngle")]
    #[doc = "Getter for the `orientAngle` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMarkerElement/orientAngle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedAngle`, `SvgMarkerElement`*"]
    pub fn orient_angle(this: &SvgMarkerElement) -> SvgAnimatedAngle;
    #[cfg(feature = "SvgAnimatedRect")]
    #[wasm_bindgen(method, getter, js_class = "SVGMarkerElement", js_name = "viewBox")]
    #[doc = "Getter for the `viewBox` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMarkerElement/viewBox)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedRect`, `SvgMarkerElement`*"]
    pub fn view_box(this: &SvgMarkerElement) -> SvgAnimatedRect;
    #[cfg(feature = "SvgAnimatedPreserveAspectRatio")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGMarkerElement",
        js_name = "preserveAspectRatio"
    )]
    #[doc = "Getter for the `preserveAspectRatio` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMarkerElement/preserveAspectRatio)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedPreserveAspectRatio`, `SvgMarkerElement`*"]
    pub fn preserve_aspect_ratio(this: &SvgMarkerElement) -> SvgAnimatedPreserveAspectRatio;
    #[cfg(feature = "SvgAngle")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "SVGMarkerElement",
        js_name = "setOrientToAngle"
    )]
    #[doc = "The `setOrientToAngle()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMarkerElement/setOrientToAngle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAngle`, `SvgMarkerElement`*"]
    pub fn set_orient_to_angle(this: &SvgMarkerElement, angle: &SvgAngle) -> Result<(), JsValue>;
    #[wasm_bindgen(method, js_class = "SVGMarkerElement", js_name = "setOrientToAuto")]
    #[doc = "The `setOrientToAuto()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMarkerElement/setOrientToAuto)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgMarkerElement`*"]
    pub fn set_orient_to_auto(this: &SvgMarkerElement);
}
impl SvgMarkerElement {
    #[doc = "The `SVGMarkerElement.SVG_MARKERUNITS_UNKNOWN` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgMarkerElement`*"]
    pub const SVG_MARKERUNITS_UNKNOWN: u16 = 0i64 as u16;
    #[doc = "The `SVGMarkerElement.SVG_MARKERUNITS_USERSPACEONUSE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgMarkerElement`*"]
    pub const SVG_MARKERUNITS_USERSPACEONUSE: u16 = 1u64 as u16;
    #[doc = "The `SVGMarkerElement.SVG_MARKERUNITS_STROKEWIDTH` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgMarkerElement`*"]
    pub const SVG_MARKERUNITS_STROKEWIDTH: u16 = 2u64 as u16;
    #[doc = "The `SVGMarkerElement.SVG_MARKER_ORIENT_UNKNOWN` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgMarkerElement`*"]
    pub const SVG_MARKER_ORIENT_UNKNOWN: u16 = 0i64 as u16;
    #[doc = "The `SVGMarkerElement.SVG_MARKER_ORIENT_AUTO` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgMarkerElement`*"]
    pub const SVG_MARKER_ORIENT_AUTO: u16 = 1u64 as u16;
    #[doc = "The `SVGMarkerElement.SVG_MARKER_ORIENT_ANGLE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgMarkerElement`*"]
    pub const SVG_MARKER_ORIENT_ANGLE: u16 = 2u64 as u16;
}
