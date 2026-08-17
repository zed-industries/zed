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
        js_name = "SVGFEGaussianBlurElement",
        typescript_type = "SVGFEGaussianBlurElement"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgfeGaussianBlurElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEGaussianBlurElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgfeGaussianBlurElement`*"]
    pub type SvgfeGaussianBlurElement;
    #[cfg(feature = "SvgAnimatedString")]
    #[wasm_bindgen(method, getter, js_class = "SVGFEGaussianBlurElement", js_name = "in1")]
    #[doc = "Getter for the `in1` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEGaussianBlurElement/in1)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgfeGaussianBlurElement`*"]
    pub fn in1(this: &SvgfeGaussianBlurElement) -> SvgAnimatedString;
    #[cfg(feature = "SvgAnimatedNumber")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGFEGaussianBlurElement",
        js_name = "stdDeviationX"
    )]
    #[doc = "Getter for the `stdDeviationX` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEGaussianBlurElement/stdDeviationX)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeGaussianBlurElement`*"]
    pub fn std_deviation_x(this: &SvgfeGaussianBlurElement) -> SvgAnimatedNumber;
    #[cfg(feature = "SvgAnimatedNumber")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGFEGaussianBlurElement",
        js_name = "stdDeviationY"
    )]
    #[doc = "Getter for the `stdDeviationY` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEGaussianBlurElement/stdDeviationY)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeGaussianBlurElement`*"]
    pub fn std_deviation_y(this: &SvgfeGaussianBlurElement) -> SvgAnimatedNumber;
    #[cfg(feature = "SvgAnimatedLength")]
    #[wasm_bindgen(method, getter, js_class = "SVGFEGaussianBlurElement", js_name = "x")]
    #[doc = "Getter for the `x` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEGaussianBlurElement/x)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeGaussianBlurElement`*"]
    pub fn x(this: &SvgfeGaussianBlurElement) -> SvgAnimatedLength;
    #[cfg(feature = "SvgAnimatedLength")]
    #[wasm_bindgen(method, getter, js_class = "SVGFEGaussianBlurElement", js_name = "y")]
    #[doc = "Getter for the `y` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEGaussianBlurElement/y)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeGaussianBlurElement`*"]
    pub fn y(this: &SvgfeGaussianBlurElement) -> SvgAnimatedLength;
    #[cfg(feature = "SvgAnimatedLength")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGFEGaussianBlurElement",
        js_name = "width"
    )]
    #[doc = "Getter for the `width` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEGaussianBlurElement/width)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeGaussianBlurElement`*"]
    pub fn width(this: &SvgfeGaussianBlurElement) -> SvgAnimatedLength;
    #[cfg(feature = "SvgAnimatedLength")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGFEGaussianBlurElement",
        js_name = "height"
    )]
    #[doc = "Getter for the `height` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEGaussianBlurElement/height)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeGaussianBlurElement`*"]
    pub fn height(this: &SvgfeGaussianBlurElement) -> SvgAnimatedLength;
    #[cfg(feature = "SvgAnimatedString")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGFEGaussianBlurElement",
        js_name = "result"
    )]
    #[doc = "Getter for the `result` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEGaussianBlurElement/result)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgfeGaussianBlurElement`*"]
    pub fn result(this: &SvgfeGaussianBlurElement) -> SvgAnimatedString;
    #[wasm_bindgen(
        method,
        js_class = "SVGFEGaussianBlurElement",
        js_name = "setStdDeviation"
    )]
    #[doc = "The `setStdDeviation()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEGaussianBlurElement/setStdDeviation)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgfeGaussianBlurElement`*"]
    pub fn set_std_deviation(
        this: &SvgfeGaussianBlurElement,
        std_deviation_x: f32,
        std_deviation_y: f32,
    );
}
