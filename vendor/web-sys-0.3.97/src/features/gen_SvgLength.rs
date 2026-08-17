#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "SVGLength",
        typescript_type = "SVGLength"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgLength` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGLength)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgLength`*"]
    pub type SvgLength;
    #[wasm_bindgen(method, getter, js_class = "SVGLength", js_name = "unitType")]
    #[doc = "Getter for the `unitType` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGLength/unitType)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgLength`*"]
    pub fn unit_type(this: &SvgLength) -> u16;
    #[wasm_bindgen(catch, method, getter, js_class = "SVGLength", js_name = "value")]
    #[doc = "Getter for the `value` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGLength/value)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgLength`*"]
    pub fn value(this: &SvgLength) -> Result<f32, JsValue>;
    #[wasm_bindgen(catch, method, setter, js_class = "SVGLength", js_name = "value")]
    #[doc = "Setter for the `value` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGLength/value)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgLength`*"]
    pub fn set_value(this: &SvgLength, value: f32) -> Result<(), JsValue>;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGLength",
        js_name = "valueInSpecifiedUnits"
    )]
    #[doc = "Getter for the `valueInSpecifiedUnits` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGLength/valueInSpecifiedUnits)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgLength`*"]
    pub fn value_in_specified_units(this: &SvgLength) -> f32;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "SVGLength",
        js_name = "valueInSpecifiedUnits"
    )]
    #[doc = "Setter for the `valueInSpecifiedUnits` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGLength/valueInSpecifiedUnits)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgLength`*"]
    pub fn set_value_in_specified_units(this: &SvgLength, value: f32);
    #[wasm_bindgen(method, getter, js_class = "SVGLength", js_name = "valueAsString")]
    #[doc = "Getter for the `valueAsString` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGLength/valueAsString)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgLength`*"]
    pub fn value_as_string(this: &SvgLength) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "SVGLength", js_name = "valueAsString")]
    #[doc = "Setter for the `valueAsString` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGLength/valueAsString)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgLength`*"]
    pub fn set_value_as_string(this: &SvgLength, value: &str);
    #[wasm_bindgen(
        catch,
        method,
        js_class = "SVGLength",
        js_name = "convertToSpecifiedUnits"
    )]
    #[doc = "The `convertToSpecifiedUnits()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGLength/convertToSpecifiedUnits)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgLength`*"]
    pub fn convert_to_specified_units(this: &SvgLength, unit_type: u16) -> Result<(), JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "SVGLength",
        js_name = "newValueSpecifiedUnits"
    )]
    #[doc = "The `newValueSpecifiedUnits()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGLength/newValueSpecifiedUnits)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgLength`*"]
    pub fn new_value_specified_units(
        this: &SvgLength,
        unit_type: u16,
        value_in_specified_units: f32,
    ) -> Result<(), JsValue>;
}
impl SvgLength {
    #[doc = "The `SVGLength.SVG_LENGTHTYPE_UNKNOWN` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgLength`*"]
    pub const SVG_LENGTHTYPE_UNKNOWN: u16 = 0i64 as u16;
    #[doc = "The `SVGLength.SVG_LENGTHTYPE_NUMBER` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgLength`*"]
    pub const SVG_LENGTHTYPE_NUMBER: u16 = 1u64 as u16;
    #[doc = "The `SVGLength.SVG_LENGTHTYPE_PERCENTAGE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgLength`*"]
    pub const SVG_LENGTHTYPE_PERCENTAGE: u16 = 2u64 as u16;
    #[doc = "The `SVGLength.SVG_LENGTHTYPE_EMS` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgLength`*"]
    pub const SVG_LENGTHTYPE_EMS: u16 = 3u64 as u16;
    #[doc = "The `SVGLength.SVG_LENGTHTYPE_EXS` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgLength`*"]
    pub const SVG_LENGTHTYPE_EXS: u16 = 4u64 as u16;
    #[doc = "The `SVGLength.SVG_LENGTHTYPE_PX` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgLength`*"]
    pub const SVG_LENGTHTYPE_PX: u16 = 5u64 as u16;
    #[doc = "The `SVGLength.SVG_LENGTHTYPE_CM` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgLength`*"]
    pub const SVG_LENGTHTYPE_CM: u16 = 6u64 as u16;
    #[doc = "The `SVGLength.SVG_LENGTHTYPE_MM` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgLength`*"]
    pub const SVG_LENGTHTYPE_MM: u16 = 7u64 as u16;
    #[doc = "The `SVGLength.SVG_LENGTHTYPE_IN` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgLength`*"]
    pub const SVG_LENGTHTYPE_IN: u16 = 8u64 as u16;
    #[doc = "The `SVGLength.SVG_LENGTHTYPE_PT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgLength`*"]
    pub const SVG_LENGTHTYPE_PT: u16 = 9u64 as u16;
    #[doc = "The `SVGLength.SVG_LENGTHTYPE_PC` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgLength`*"]
    pub const SVG_LENGTHTYPE_PC: u16 = 10u64 as u16;
}
