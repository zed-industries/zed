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
        js_name = "SVGFilterElement",
        typescript_type = "SVGFilterElement"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgFilterElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFilterElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgFilterElement`*"]
    pub type SvgFilterElement;
    #[cfg(feature = "SvgAnimatedEnumeration")]
    #[wasm_bindgen(method, getter, js_class = "SVGFilterElement", js_name = "filterUnits")]
    #[doc = "Getter for the `filterUnits` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFilterElement/filterUnits)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedEnumeration`, `SvgFilterElement`*"]
    pub fn filter_units(this: &SvgFilterElement) -> SvgAnimatedEnumeration;
    #[cfg(feature = "SvgAnimatedEnumeration")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGFilterElement",
        js_name = "primitiveUnits"
    )]
    #[doc = "Getter for the `primitiveUnits` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFilterElement/primitiveUnits)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedEnumeration`, `SvgFilterElement`*"]
    pub fn primitive_units(this: &SvgFilterElement) -> SvgAnimatedEnumeration;
    #[cfg(feature = "SvgAnimatedLength")]
    #[wasm_bindgen(method, getter, js_class = "SVGFilterElement", js_name = "x")]
    #[doc = "Getter for the `x` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFilterElement/x)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgFilterElement`*"]
    pub fn x(this: &SvgFilterElement) -> SvgAnimatedLength;
    #[cfg(feature = "SvgAnimatedLength")]
    #[wasm_bindgen(method, getter, js_class = "SVGFilterElement", js_name = "y")]
    #[doc = "Getter for the `y` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFilterElement/y)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgFilterElement`*"]
    pub fn y(this: &SvgFilterElement) -> SvgAnimatedLength;
    #[cfg(feature = "SvgAnimatedLength")]
    #[wasm_bindgen(method, getter, js_class = "SVGFilterElement", js_name = "width")]
    #[doc = "Getter for the `width` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFilterElement/width)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgFilterElement`*"]
    pub fn width(this: &SvgFilterElement) -> SvgAnimatedLength;
    #[cfg(feature = "SvgAnimatedLength")]
    #[wasm_bindgen(method, getter, js_class = "SVGFilterElement", js_name = "height")]
    #[doc = "Getter for the `height` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFilterElement/height)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgFilterElement`*"]
    pub fn height(this: &SvgFilterElement) -> SvgAnimatedLength;
    #[cfg(feature = "SvgAnimatedString")]
    #[wasm_bindgen(method, getter, js_class = "SVGFilterElement", js_name = "href")]
    #[doc = "Getter for the `href` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFilterElement/href)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgFilterElement`*"]
    pub fn href(this: &SvgFilterElement) -> SvgAnimatedString;
}
