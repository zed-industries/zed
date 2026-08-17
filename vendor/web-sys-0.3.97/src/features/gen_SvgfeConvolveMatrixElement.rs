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
        js_name = "SVGFEConvolveMatrixElement",
        typescript_type = "SVGFEConvolveMatrixElement"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgfeConvolveMatrixElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEConvolveMatrixElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgfeConvolveMatrixElement`*"]
    pub type SvgfeConvolveMatrixElement;
    #[cfg(feature = "SvgAnimatedString")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGFEConvolveMatrixElement",
        js_name = "in1"
    )]
    #[doc = "Getter for the `in1` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEConvolveMatrixElement/in1)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgfeConvolveMatrixElement`*"]
    pub fn in1(this: &SvgfeConvolveMatrixElement) -> SvgAnimatedString;
    #[cfg(feature = "SvgAnimatedInteger")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGFEConvolveMatrixElement",
        js_name = "orderX"
    )]
    #[doc = "Getter for the `orderX` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEConvolveMatrixElement/orderX)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedInteger`, `SvgfeConvolveMatrixElement`*"]
    pub fn order_x(this: &SvgfeConvolveMatrixElement) -> SvgAnimatedInteger;
    #[cfg(feature = "SvgAnimatedInteger")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGFEConvolveMatrixElement",
        js_name = "orderY"
    )]
    #[doc = "Getter for the `orderY` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEConvolveMatrixElement/orderY)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedInteger`, `SvgfeConvolveMatrixElement`*"]
    pub fn order_y(this: &SvgfeConvolveMatrixElement) -> SvgAnimatedInteger;
    #[cfg(feature = "SvgAnimatedNumberList")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGFEConvolveMatrixElement",
        js_name = "kernelMatrix"
    )]
    #[doc = "Getter for the `kernelMatrix` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEConvolveMatrixElement/kernelMatrix)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedNumberList`, `SvgfeConvolveMatrixElement`*"]
    pub fn kernel_matrix(this: &SvgfeConvolveMatrixElement) -> SvgAnimatedNumberList;
    #[cfg(feature = "SvgAnimatedNumber")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGFEConvolveMatrixElement",
        js_name = "divisor"
    )]
    #[doc = "Getter for the `divisor` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEConvolveMatrixElement/divisor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeConvolveMatrixElement`*"]
    pub fn divisor(this: &SvgfeConvolveMatrixElement) -> SvgAnimatedNumber;
    #[cfg(feature = "SvgAnimatedNumber")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGFEConvolveMatrixElement",
        js_name = "bias"
    )]
    #[doc = "Getter for the `bias` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEConvolveMatrixElement/bias)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeConvolveMatrixElement`*"]
    pub fn bias(this: &SvgfeConvolveMatrixElement) -> SvgAnimatedNumber;
    #[cfg(feature = "SvgAnimatedInteger")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGFEConvolveMatrixElement",
        js_name = "targetX"
    )]
    #[doc = "Getter for the `targetX` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEConvolveMatrixElement/targetX)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedInteger`, `SvgfeConvolveMatrixElement`*"]
    pub fn target_x(this: &SvgfeConvolveMatrixElement) -> SvgAnimatedInteger;
    #[cfg(feature = "SvgAnimatedInteger")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGFEConvolveMatrixElement",
        js_name = "targetY"
    )]
    #[doc = "Getter for the `targetY` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEConvolveMatrixElement/targetY)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedInteger`, `SvgfeConvolveMatrixElement`*"]
    pub fn target_y(this: &SvgfeConvolveMatrixElement) -> SvgAnimatedInteger;
    #[cfg(feature = "SvgAnimatedEnumeration")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGFEConvolveMatrixElement",
        js_name = "edgeMode"
    )]
    #[doc = "Getter for the `edgeMode` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEConvolveMatrixElement/edgeMode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedEnumeration`, `SvgfeConvolveMatrixElement`*"]
    pub fn edge_mode(this: &SvgfeConvolveMatrixElement) -> SvgAnimatedEnumeration;
    #[cfg(feature = "SvgAnimatedNumber")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGFEConvolveMatrixElement",
        js_name = "kernelUnitLengthX"
    )]
    #[doc = "Getter for the `kernelUnitLengthX` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEConvolveMatrixElement/kernelUnitLengthX)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeConvolveMatrixElement`*"]
    pub fn kernel_unit_length_x(this: &SvgfeConvolveMatrixElement) -> SvgAnimatedNumber;
    #[cfg(feature = "SvgAnimatedNumber")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGFEConvolveMatrixElement",
        js_name = "kernelUnitLengthY"
    )]
    #[doc = "Getter for the `kernelUnitLengthY` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEConvolveMatrixElement/kernelUnitLengthY)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeConvolveMatrixElement`*"]
    pub fn kernel_unit_length_y(this: &SvgfeConvolveMatrixElement) -> SvgAnimatedNumber;
    #[cfg(feature = "SvgAnimatedBoolean")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGFEConvolveMatrixElement",
        js_name = "preserveAlpha"
    )]
    #[doc = "Getter for the `preserveAlpha` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEConvolveMatrixElement/preserveAlpha)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedBoolean`, `SvgfeConvolveMatrixElement`*"]
    pub fn preserve_alpha(this: &SvgfeConvolveMatrixElement) -> SvgAnimatedBoolean;
    #[cfg(feature = "SvgAnimatedLength")]
    #[wasm_bindgen(method, getter, js_class = "SVGFEConvolveMatrixElement", js_name = "x")]
    #[doc = "Getter for the `x` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEConvolveMatrixElement/x)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeConvolveMatrixElement`*"]
    pub fn x(this: &SvgfeConvolveMatrixElement) -> SvgAnimatedLength;
    #[cfg(feature = "SvgAnimatedLength")]
    #[wasm_bindgen(method, getter, js_class = "SVGFEConvolveMatrixElement", js_name = "y")]
    #[doc = "Getter for the `y` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEConvolveMatrixElement/y)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeConvolveMatrixElement`*"]
    pub fn y(this: &SvgfeConvolveMatrixElement) -> SvgAnimatedLength;
    #[cfg(feature = "SvgAnimatedLength")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGFEConvolveMatrixElement",
        js_name = "width"
    )]
    #[doc = "Getter for the `width` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEConvolveMatrixElement/width)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeConvolveMatrixElement`*"]
    pub fn width(this: &SvgfeConvolveMatrixElement) -> SvgAnimatedLength;
    #[cfg(feature = "SvgAnimatedLength")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGFEConvolveMatrixElement",
        js_name = "height"
    )]
    #[doc = "Getter for the `height` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEConvolveMatrixElement/height)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeConvolveMatrixElement`*"]
    pub fn height(this: &SvgfeConvolveMatrixElement) -> SvgAnimatedLength;
    #[cfg(feature = "SvgAnimatedString")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGFEConvolveMatrixElement",
        js_name = "result"
    )]
    #[doc = "Getter for the `result` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEConvolveMatrixElement/result)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgfeConvolveMatrixElement`*"]
    pub fn result(this: &SvgfeConvolveMatrixElement) -> SvgAnimatedString;
}
impl SvgfeConvolveMatrixElement {
    #[doc = "The `SVGFEConvolveMatrixElement.SVG_EDGEMODE_UNKNOWN` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgfeConvolveMatrixElement`*"]
    pub const SVG_EDGEMODE_UNKNOWN: u16 = 0i64 as u16;
    #[doc = "The `SVGFEConvolveMatrixElement.SVG_EDGEMODE_DUPLICATE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgfeConvolveMatrixElement`*"]
    pub const SVG_EDGEMODE_DUPLICATE: u16 = 1u64 as u16;
    #[doc = "The `SVGFEConvolveMatrixElement.SVG_EDGEMODE_WRAP` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgfeConvolveMatrixElement`*"]
    pub const SVG_EDGEMODE_WRAP: u16 = 2u64 as u16;
    #[doc = "The `SVGFEConvolveMatrixElement.SVG_EDGEMODE_NONE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgfeConvolveMatrixElement`*"]
    pub const SVG_EDGEMODE_NONE: u16 = 3u64 as u16;
}
