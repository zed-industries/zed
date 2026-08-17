#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "SvgGraphicsElement",
        extends = "SvgElement",
        extends = "Element",
        extends = "Node",
        extends = "EventTarget",
        extends = "::js_sys::Object",
        js_name = "SVGTextContentElement",
        typescript_type = "SVGTextContentElement"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgTextContentElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTextContentElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgTextContentElement`*"]
    pub type SvgTextContentElement;
    #[cfg(feature = "SvgAnimatedLength")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGTextContentElement",
        js_name = "textLength"
    )]
    #[doc = "Getter for the `textLength` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTextContentElement/textLength)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgTextContentElement`*"]
    pub fn text_length(this: &SvgTextContentElement) -> SvgAnimatedLength;
    #[cfg(feature = "SvgAnimatedEnumeration")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGTextContentElement",
        js_name = "lengthAdjust"
    )]
    #[doc = "Getter for the `lengthAdjust` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTextContentElement/lengthAdjust)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedEnumeration`, `SvgTextContentElement`*"]
    pub fn length_adjust(this: &SvgTextContentElement) -> SvgAnimatedEnumeration;
    #[cfg(feature = "SvgPoint")]
    #[wasm_bindgen(
        method,
        js_class = "SVGTextContentElement",
        js_name = "getCharNumAtPosition"
    )]
    #[doc = "The `getCharNumAtPosition()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTextContentElement/getCharNumAtPosition)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPoint`, `SvgTextContentElement`*"]
    pub fn get_char_num_at_position(this: &SvgTextContentElement, point: &SvgPoint) -> i32;
    #[wasm_bindgen(
        method,
        js_class = "SVGTextContentElement",
        js_name = "getComputedTextLength"
    )]
    #[doc = "The `getComputedTextLength()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTextContentElement/getComputedTextLength)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgTextContentElement`*"]
    pub fn get_computed_text_length(this: &SvgTextContentElement) -> f32;
    #[cfg(feature = "SvgPoint")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "SVGTextContentElement",
        js_name = "getEndPositionOfChar"
    )]
    #[doc = "The `getEndPositionOfChar()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTextContentElement/getEndPositionOfChar)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPoint`, `SvgTextContentElement`*"]
    pub fn get_end_position_of_char(
        this: &SvgTextContentElement,
        charnum: u32,
    ) -> Result<SvgPoint, JsValue>;
    #[cfg(feature = "SvgRect")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "SVGTextContentElement",
        js_name = "getExtentOfChar"
    )]
    #[doc = "The `getExtentOfChar()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTextContentElement/getExtentOfChar)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgRect`, `SvgTextContentElement`*"]
    pub fn get_extent_of_char(
        this: &SvgTextContentElement,
        charnum: u32,
    ) -> Result<SvgRect, JsValue>;
    #[wasm_bindgen(
        method,
        js_class = "SVGTextContentElement",
        js_name = "getNumberOfChars"
    )]
    #[doc = "The `getNumberOfChars()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTextContentElement/getNumberOfChars)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgTextContentElement`*"]
    pub fn get_number_of_chars(this: &SvgTextContentElement) -> i32;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "SVGTextContentElement",
        js_name = "getRotationOfChar"
    )]
    #[doc = "The `getRotationOfChar()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTextContentElement/getRotationOfChar)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgTextContentElement`*"]
    pub fn get_rotation_of_char(this: &SvgTextContentElement, charnum: u32)
        -> Result<f32, JsValue>;
    #[cfg(feature = "SvgPoint")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "SVGTextContentElement",
        js_name = "getStartPositionOfChar"
    )]
    #[doc = "The `getStartPositionOfChar()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTextContentElement/getStartPositionOfChar)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPoint`, `SvgTextContentElement`*"]
    pub fn get_start_position_of_char(
        this: &SvgTextContentElement,
        charnum: u32,
    ) -> Result<SvgPoint, JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "SVGTextContentElement",
        js_name = "getSubStringLength"
    )]
    #[doc = "The `getSubStringLength()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTextContentElement/getSubStringLength)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgTextContentElement`*"]
    pub fn get_sub_string_length(
        this: &SvgTextContentElement,
        charnum: u32,
        nchars: u32,
    ) -> Result<f32, JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "SVGTextContentElement",
        js_name = "selectSubString"
    )]
    #[doc = "The `selectSubString()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTextContentElement/selectSubString)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgTextContentElement`*"]
    pub fn select_sub_string(
        this: &SvgTextContentElement,
        charnum: u32,
        nchars: u32,
    ) -> Result<(), JsValue>;
}
impl SvgTextContentElement {
    #[doc = "The `SVGTextContentElement.LENGTHADJUST_UNKNOWN` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgTextContentElement`*"]
    pub const LENGTHADJUST_UNKNOWN: u16 = 0i64 as u16;
    #[doc = "The `SVGTextContentElement.LENGTHADJUST_SPACING` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgTextContentElement`*"]
    pub const LENGTHADJUST_SPACING: u16 = 1u64 as u16;
    #[doc = "The `SVGTextContentElement.LENGTHADJUST_SPACINGANDGLYPHS` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgTextContentElement`*"]
    pub const LENGTHADJUST_SPACINGANDGLYPHS: u16 = 2u64 as u16;
}
