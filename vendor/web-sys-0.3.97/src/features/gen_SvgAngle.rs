#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "SVGAngle",
        typescript_type = "SVGAngle"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgAngle` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAngle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAngle`*"]
    pub type SvgAngle;
    #[wasm_bindgen(method, getter, js_class = "SVGAngle", js_name = "unitType")]
    #[doc = "Getter for the `unitType` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAngle/unitType)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAngle`*"]
    pub fn unit_type(this: &SvgAngle) -> u16;
    #[wasm_bindgen(method, getter, js_class = "SVGAngle", js_name = "value")]
    #[doc = "Getter for the `value` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAngle/value)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAngle`*"]
    pub fn value(this: &SvgAngle) -> f32;
    #[wasm_bindgen(method, setter, js_class = "SVGAngle", js_name = "value")]
    #[doc = "Setter for the `value` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAngle/value)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAngle`*"]
    pub fn set_value(this: &SvgAngle, value: f32);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGAngle",
        js_name = "valueInSpecifiedUnits"
    )]
    #[doc = "Getter for the `valueInSpecifiedUnits` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAngle/valueInSpecifiedUnits)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAngle`*"]
    pub fn value_in_specified_units(this: &SvgAngle) -> f32;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "SVGAngle",
        js_name = "valueInSpecifiedUnits"
    )]
    #[doc = "Setter for the `valueInSpecifiedUnits` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAngle/valueInSpecifiedUnits)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAngle`*"]
    pub fn set_value_in_specified_units(this: &SvgAngle, value: f32);
    #[wasm_bindgen(method, getter, js_class = "SVGAngle", js_name = "valueAsString")]
    #[doc = "Getter for the `valueAsString` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAngle/valueAsString)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAngle`*"]
    pub fn value_as_string(this: &SvgAngle) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "SVGAngle", js_name = "valueAsString")]
    #[doc = "Setter for the `valueAsString` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAngle/valueAsString)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAngle`*"]
    pub fn set_value_as_string(this: &SvgAngle, value: &str);
    #[wasm_bindgen(
        catch,
        method,
        js_class = "SVGAngle",
        js_name = "convertToSpecifiedUnits"
    )]
    #[doc = "The `convertToSpecifiedUnits()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAngle/convertToSpecifiedUnits)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAngle`*"]
    pub fn convert_to_specified_units(this: &SvgAngle, unit_type: u16) -> Result<(), JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "SVGAngle",
        js_name = "newValueSpecifiedUnits"
    )]
    #[doc = "The `newValueSpecifiedUnits()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAngle/newValueSpecifiedUnits)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAngle`*"]
    pub fn new_value_specified_units(
        this: &SvgAngle,
        unit_type: u16,
        value_in_specified_units: f32,
    ) -> Result<(), JsValue>;
}
impl SvgAngle {
    #[doc = "The `SVGAngle.SVG_ANGLETYPE_UNKNOWN` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAngle`*"]
    pub const SVG_ANGLETYPE_UNKNOWN: u16 = 0i64 as u16;
    #[doc = "The `SVGAngle.SVG_ANGLETYPE_UNSPECIFIED` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAngle`*"]
    pub const SVG_ANGLETYPE_UNSPECIFIED: u16 = 1u64 as u16;
    #[doc = "The `SVGAngle.SVG_ANGLETYPE_DEG` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAngle`*"]
    pub const SVG_ANGLETYPE_DEG: u16 = 2u64 as u16;
    #[doc = "The `SVGAngle.SVG_ANGLETYPE_RAD` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAngle`*"]
    pub const SVG_ANGLETYPE_RAD: u16 = 3u64 as u16;
    #[doc = "The `SVGAngle.SVG_ANGLETYPE_GRAD` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAngle`*"]
    pub const SVG_ANGLETYPE_GRAD: u16 = 4u64 as u16;
}
