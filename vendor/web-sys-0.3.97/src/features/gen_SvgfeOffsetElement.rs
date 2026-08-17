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
        js_name = "SVGFEOffsetElement",
        typescript_type = "SVGFEOffsetElement"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgfeOffsetElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEOffsetElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgfeOffsetElement`*"]
    pub type SvgfeOffsetElement;
    #[cfg(feature = "SvgAnimatedString")]
    #[wasm_bindgen(method, getter, js_class = "SVGFEOffsetElement", js_name = "in1")]
    #[doc = "Getter for the `in1` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEOffsetElement/in1)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgfeOffsetElement`*"]
    pub fn in1(this: &SvgfeOffsetElement) -> SvgAnimatedString;
    #[cfg(feature = "SvgAnimatedNumber")]
    #[wasm_bindgen(method, getter, js_class = "SVGFEOffsetElement", js_name = "dx")]
    #[doc = "Getter for the `dx` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEOffsetElement/dx)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeOffsetElement`*"]
    pub fn dx(this: &SvgfeOffsetElement) -> SvgAnimatedNumber;
    #[cfg(feature = "SvgAnimatedNumber")]
    #[wasm_bindgen(method, getter, js_class = "SVGFEOffsetElement", js_name = "dy")]
    #[doc = "Getter for the `dy` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEOffsetElement/dy)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeOffsetElement`*"]
    pub fn dy(this: &SvgfeOffsetElement) -> SvgAnimatedNumber;
    #[cfg(feature = "SvgAnimatedLength")]
    #[wasm_bindgen(method, getter, js_class = "SVGFEOffsetElement", js_name = "x")]
    #[doc = "Getter for the `x` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEOffsetElement/x)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeOffsetElement`*"]
    pub fn x(this: &SvgfeOffsetElement) -> SvgAnimatedLength;
    #[cfg(feature = "SvgAnimatedLength")]
    #[wasm_bindgen(method, getter, js_class = "SVGFEOffsetElement", js_name = "y")]
    #[doc = "Getter for the `y` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEOffsetElement/y)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeOffsetElement`*"]
    pub fn y(this: &SvgfeOffsetElement) -> SvgAnimatedLength;
    #[cfg(feature = "SvgAnimatedLength")]
    #[wasm_bindgen(method, getter, js_class = "SVGFEOffsetElement", js_name = "width")]
    #[doc = "Getter for the `width` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEOffsetElement/width)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeOffsetElement`*"]
    pub fn width(this: &SvgfeOffsetElement) -> SvgAnimatedLength;
    #[cfg(feature = "SvgAnimatedLength")]
    #[wasm_bindgen(method, getter, js_class = "SVGFEOffsetElement", js_name = "height")]
    #[doc = "Getter for the `height` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEOffsetElement/height)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeOffsetElement`*"]
    pub fn height(this: &SvgfeOffsetElement) -> SvgAnimatedLength;
    #[cfg(feature = "SvgAnimatedString")]
    #[wasm_bindgen(method, getter, js_class = "SVGFEOffsetElement", js_name = "result")]
    #[doc = "Getter for the `result` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEOffsetElement/result)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgfeOffsetElement`*"]
    pub fn result(this: &SvgfeOffsetElement) -> SvgAnimatedString;
}
