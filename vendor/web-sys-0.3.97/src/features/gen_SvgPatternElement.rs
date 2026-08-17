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
        js_name = "SVGPatternElement",
        typescript_type = "SVGPatternElement"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgPatternElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPatternElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPatternElement`*"]
    pub type SvgPatternElement;
    #[cfg(feature = "SvgAnimatedEnumeration")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGPatternElement",
        js_name = "patternUnits"
    )]
    #[doc = "Getter for the `patternUnits` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPatternElement/patternUnits)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedEnumeration`, `SvgPatternElement`*"]
    pub fn pattern_units(this: &SvgPatternElement) -> SvgAnimatedEnumeration;
    #[cfg(feature = "SvgAnimatedEnumeration")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGPatternElement",
        js_name = "patternContentUnits"
    )]
    #[doc = "Getter for the `patternContentUnits` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPatternElement/patternContentUnits)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedEnumeration`, `SvgPatternElement`*"]
    pub fn pattern_content_units(this: &SvgPatternElement) -> SvgAnimatedEnumeration;
    #[cfg(feature = "SvgAnimatedTransformList")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGPatternElement",
        js_name = "patternTransform"
    )]
    #[doc = "Getter for the `patternTransform` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPatternElement/patternTransform)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedTransformList`, `SvgPatternElement`*"]
    pub fn pattern_transform(this: &SvgPatternElement) -> SvgAnimatedTransformList;
    #[cfg(feature = "SvgAnimatedLength")]
    #[wasm_bindgen(method, getter, js_class = "SVGPatternElement", js_name = "x")]
    #[doc = "Getter for the `x` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPatternElement/x)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgPatternElement`*"]
    pub fn x(this: &SvgPatternElement) -> SvgAnimatedLength;
    #[cfg(feature = "SvgAnimatedLength")]
    #[wasm_bindgen(method, getter, js_class = "SVGPatternElement", js_name = "y")]
    #[doc = "Getter for the `y` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPatternElement/y)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgPatternElement`*"]
    pub fn y(this: &SvgPatternElement) -> SvgAnimatedLength;
    #[cfg(feature = "SvgAnimatedLength")]
    #[wasm_bindgen(method, getter, js_class = "SVGPatternElement", js_name = "width")]
    #[doc = "Getter for the `width` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPatternElement/width)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgPatternElement`*"]
    pub fn width(this: &SvgPatternElement) -> SvgAnimatedLength;
    #[cfg(feature = "SvgAnimatedLength")]
    #[wasm_bindgen(method, getter, js_class = "SVGPatternElement", js_name = "height")]
    #[doc = "Getter for the `height` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPatternElement/height)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgPatternElement`*"]
    pub fn height(this: &SvgPatternElement) -> SvgAnimatedLength;
    #[cfg(feature = "SvgAnimatedRect")]
    #[wasm_bindgen(method, getter, js_class = "SVGPatternElement", js_name = "viewBox")]
    #[doc = "Getter for the `viewBox` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPatternElement/viewBox)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedRect`, `SvgPatternElement`*"]
    pub fn view_box(this: &SvgPatternElement) -> SvgAnimatedRect;
    #[cfg(feature = "SvgAnimatedPreserveAspectRatio")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGPatternElement",
        js_name = "preserveAspectRatio"
    )]
    #[doc = "Getter for the `preserveAspectRatio` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPatternElement/preserveAspectRatio)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedPreserveAspectRatio`, `SvgPatternElement`*"]
    pub fn preserve_aspect_ratio(this: &SvgPatternElement) -> SvgAnimatedPreserveAspectRatio;
    #[cfg(feature = "SvgAnimatedString")]
    #[wasm_bindgen(method, getter, js_class = "SVGPatternElement", js_name = "href")]
    #[doc = "Getter for the `href` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPatternElement/href)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgPatternElement`*"]
    pub fn href(this: &SvgPatternElement) -> SvgAnimatedString;
}
