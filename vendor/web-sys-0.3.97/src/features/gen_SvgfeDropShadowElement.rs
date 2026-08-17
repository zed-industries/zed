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
        js_name = "SVGFEDropShadowElement",
        typescript_type = "SVGFEDropShadowElement"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgfeDropShadowElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDropShadowElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgfeDropShadowElement`*"]
    pub type SvgfeDropShadowElement;
    #[cfg(feature = "SvgAnimatedString")]
    #[wasm_bindgen(method, getter, js_class = "SVGFEDropShadowElement", js_name = "in1")]
    #[doc = "Getter for the `in1` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDropShadowElement/in1)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgfeDropShadowElement`*"]
    pub fn in1(this: &SvgfeDropShadowElement) -> SvgAnimatedString;
    #[cfg(feature = "SvgAnimatedNumber")]
    #[wasm_bindgen(method, getter, js_class = "SVGFEDropShadowElement", js_name = "dx")]
    #[doc = "Getter for the `dx` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDropShadowElement/dx)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeDropShadowElement`*"]
    pub fn dx(this: &SvgfeDropShadowElement) -> SvgAnimatedNumber;
    #[cfg(feature = "SvgAnimatedNumber")]
    #[wasm_bindgen(method, getter, js_class = "SVGFEDropShadowElement", js_name = "dy")]
    #[doc = "Getter for the `dy` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDropShadowElement/dy)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeDropShadowElement`*"]
    pub fn dy(this: &SvgfeDropShadowElement) -> SvgAnimatedNumber;
    #[cfg(feature = "SvgAnimatedNumber")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGFEDropShadowElement",
        js_name = "stdDeviationX"
    )]
    #[doc = "Getter for the `stdDeviationX` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDropShadowElement/stdDeviationX)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeDropShadowElement`*"]
    pub fn std_deviation_x(this: &SvgfeDropShadowElement) -> SvgAnimatedNumber;
    #[cfg(feature = "SvgAnimatedNumber")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGFEDropShadowElement",
        js_name = "stdDeviationY"
    )]
    #[doc = "Getter for the `stdDeviationY` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDropShadowElement/stdDeviationY)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeDropShadowElement`*"]
    pub fn std_deviation_y(this: &SvgfeDropShadowElement) -> SvgAnimatedNumber;
    #[cfg(feature = "SvgAnimatedLength")]
    #[wasm_bindgen(method, getter, js_class = "SVGFEDropShadowElement", js_name = "x")]
    #[doc = "Getter for the `x` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDropShadowElement/x)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeDropShadowElement`*"]
    pub fn x(this: &SvgfeDropShadowElement) -> SvgAnimatedLength;
    #[cfg(feature = "SvgAnimatedLength")]
    #[wasm_bindgen(method, getter, js_class = "SVGFEDropShadowElement", js_name = "y")]
    #[doc = "Getter for the `y` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDropShadowElement/y)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeDropShadowElement`*"]
    pub fn y(this: &SvgfeDropShadowElement) -> SvgAnimatedLength;
    #[cfg(feature = "SvgAnimatedLength")]
    #[wasm_bindgen(method, getter, js_class = "SVGFEDropShadowElement", js_name = "width")]
    #[doc = "Getter for the `width` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDropShadowElement/width)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeDropShadowElement`*"]
    pub fn width(this: &SvgfeDropShadowElement) -> SvgAnimatedLength;
    #[cfg(feature = "SvgAnimatedLength")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGFEDropShadowElement",
        js_name = "height"
    )]
    #[doc = "Getter for the `height` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDropShadowElement/height)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeDropShadowElement`*"]
    pub fn height(this: &SvgfeDropShadowElement) -> SvgAnimatedLength;
    #[cfg(feature = "SvgAnimatedString")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGFEDropShadowElement",
        js_name = "result"
    )]
    #[doc = "Getter for the `result` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDropShadowElement/result)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgfeDropShadowElement`*"]
    pub fn result(this: &SvgfeDropShadowElement) -> SvgAnimatedString;
    #[wasm_bindgen(
        method,
        js_class = "SVGFEDropShadowElement",
        js_name = "setStdDeviation"
    )]
    #[doc = "The `setStdDeviation()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDropShadowElement/setStdDeviation)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgfeDropShadowElement`*"]
    pub fn set_std_deviation(
        this: &SvgfeDropShadowElement,
        std_deviation_x: f32,
        std_deviation_y: f32,
    );
}
