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
        js_name = "SVGFEDiffuseLightingElement",
        typescript_type = "SVGFEDiffuseLightingElement"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgfeDiffuseLightingElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDiffuseLightingElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgfeDiffuseLightingElement`*"]
    pub type SvgfeDiffuseLightingElement;
    #[cfg(feature = "SvgAnimatedString")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGFEDiffuseLightingElement",
        js_name = "in1"
    )]
    #[doc = "Getter for the `in1` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDiffuseLightingElement/in1)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgfeDiffuseLightingElement`*"]
    pub fn in1(this: &SvgfeDiffuseLightingElement) -> SvgAnimatedString;
    #[cfg(feature = "SvgAnimatedNumber")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGFEDiffuseLightingElement",
        js_name = "surfaceScale"
    )]
    #[doc = "Getter for the `surfaceScale` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDiffuseLightingElement/surfaceScale)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeDiffuseLightingElement`*"]
    pub fn surface_scale(this: &SvgfeDiffuseLightingElement) -> SvgAnimatedNumber;
    #[cfg(feature = "SvgAnimatedNumber")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGFEDiffuseLightingElement",
        js_name = "diffuseConstant"
    )]
    #[doc = "Getter for the `diffuseConstant` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDiffuseLightingElement/diffuseConstant)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeDiffuseLightingElement`*"]
    pub fn diffuse_constant(this: &SvgfeDiffuseLightingElement) -> SvgAnimatedNumber;
    #[cfg(feature = "SvgAnimatedNumber")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGFEDiffuseLightingElement",
        js_name = "kernelUnitLengthX"
    )]
    #[doc = "Getter for the `kernelUnitLengthX` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDiffuseLightingElement/kernelUnitLengthX)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeDiffuseLightingElement`*"]
    pub fn kernel_unit_length_x(this: &SvgfeDiffuseLightingElement) -> SvgAnimatedNumber;
    #[cfg(feature = "SvgAnimatedNumber")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGFEDiffuseLightingElement",
        js_name = "kernelUnitLengthY"
    )]
    #[doc = "Getter for the `kernelUnitLengthY` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDiffuseLightingElement/kernelUnitLengthY)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeDiffuseLightingElement`*"]
    pub fn kernel_unit_length_y(this: &SvgfeDiffuseLightingElement) -> SvgAnimatedNumber;
    #[cfg(feature = "SvgAnimatedLength")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGFEDiffuseLightingElement",
        js_name = "x"
    )]
    #[doc = "Getter for the `x` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDiffuseLightingElement/x)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeDiffuseLightingElement`*"]
    pub fn x(this: &SvgfeDiffuseLightingElement) -> SvgAnimatedLength;
    #[cfg(feature = "SvgAnimatedLength")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGFEDiffuseLightingElement",
        js_name = "y"
    )]
    #[doc = "Getter for the `y` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDiffuseLightingElement/y)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeDiffuseLightingElement`*"]
    pub fn y(this: &SvgfeDiffuseLightingElement) -> SvgAnimatedLength;
    #[cfg(feature = "SvgAnimatedLength")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGFEDiffuseLightingElement",
        js_name = "width"
    )]
    #[doc = "Getter for the `width` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDiffuseLightingElement/width)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeDiffuseLightingElement`*"]
    pub fn width(this: &SvgfeDiffuseLightingElement) -> SvgAnimatedLength;
    #[cfg(feature = "SvgAnimatedLength")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGFEDiffuseLightingElement",
        js_name = "height"
    )]
    #[doc = "Getter for the `height` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDiffuseLightingElement/height)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeDiffuseLightingElement`*"]
    pub fn height(this: &SvgfeDiffuseLightingElement) -> SvgAnimatedLength;
    #[cfg(feature = "SvgAnimatedString")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGFEDiffuseLightingElement",
        js_name = "result"
    )]
    #[doc = "Getter for the `result` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDiffuseLightingElement/result)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgfeDiffuseLightingElement`*"]
    pub fn result(this: &SvgfeDiffuseLightingElement) -> SvgAnimatedString;
}
