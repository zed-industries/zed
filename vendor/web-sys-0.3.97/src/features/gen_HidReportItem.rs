#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "HIDReportItem")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HidReportItem` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportItem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type HidReportItem;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `hasNull` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportItem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "hasNull")]
    pub fn get_has_null(this: &HidReportItem) -> Option<bool>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `hasNull` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportItem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "hasNull")]
    pub fn set_has_null(this: &HidReportItem, val: bool);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `hasPreferredState` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportItem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "hasPreferredState")]
    pub fn get_has_preferred_state(this: &HidReportItem) -> Option<bool>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `hasPreferredState` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportItem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "hasPreferredState")]
    pub fn set_has_preferred_state(this: &HidReportItem, val: bool);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `isAbsolute` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportItem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "isAbsolute")]
    pub fn get_is_absolute(this: &HidReportItem) -> Option<bool>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `isAbsolute` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportItem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "isAbsolute")]
    pub fn set_is_absolute(this: &HidReportItem, val: bool);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `isArray` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportItem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "isArray")]
    pub fn get_is_array(this: &HidReportItem) -> Option<bool>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `isArray` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportItem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "isArray")]
    pub fn set_is_array(this: &HidReportItem, val: bool);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `isBufferedBytes` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportItem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "isBufferedBytes")]
    pub fn get_is_buffered_bytes(this: &HidReportItem) -> Option<bool>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `isBufferedBytes` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportItem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "isBufferedBytes")]
    pub fn set_is_buffered_bytes(this: &HidReportItem, val: bool);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `isConstant` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportItem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "isConstant")]
    pub fn get_is_constant(this: &HidReportItem) -> Option<bool>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `isConstant` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportItem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "isConstant")]
    pub fn set_is_constant(this: &HidReportItem, val: bool);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `isLinear` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportItem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "isLinear")]
    pub fn get_is_linear(this: &HidReportItem) -> Option<bool>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `isLinear` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportItem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "isLinear")]
    pub fn set_is_linear(this: &HidReportItem, val: bool);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `isRange` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportItem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "isRange")]
    pub fn get_is_range(this: &HidReportItem) -> Option<bool>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `isRange` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportItem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "isRange")]
    pub fn set_is_range(this: &HidReportItem, val: bool);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `isVolatile` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportItem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "isVolatile")]
    pub fn get_is_volatile(this: &HidReportItem) -> Option<bool>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `isVolatile` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportItem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "isVolatile")]
    pub fn set_is_volatile(this: &HidReportItem, val: bool);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `logicalMaximum` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportItem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "logicalMaximum")]
    pub fn get_logical_maximum(this: &HidReportItem) -> Option<i32>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `logicalMaximum` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportItem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "logicalMaximum")]
    pub fn set_logical_maximum(this: &HidReportItem, val: i32);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `logicalMinimum` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportItem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "logicalMinimum")]
    pub fn get_logical_minimum(this: &HidReportItem) -> Option<i32>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `logicalMinimum` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportItem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "logicalMinimum")]
    pub fn set_logical_minimum(this: &HidReportItem, val: i32);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `physicalMaximum` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportItem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "physicalMaximum")]
    pub fn get_physical_maximum(this: &HidReportItem) -> Option<i32>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `physicalMaximum` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportItem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "physicalMaximum")]
    pub fn set_physical_maximum(this: &HidReportItem, val: i32);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `physicalMinimum` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportItem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "physicalMinimum")]
    pub fn get_physical_minimum(this: &HidReportItem) -> Option<i32>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `physicalMinimum` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportItem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "physicalMinimum")]
    pub fn set_physical_minimum(this: &HidReportItem, val: i32);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `reportCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportItem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "reportCount")]
    pub fn get_report_count(this: &HidReportItem) -> Option<u16>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `reportCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportItem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "reportCount")]
    pub fn set_report_count(this: &HidReportItem, val: u16);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `reportSize` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportItem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "reportSize")]
    pub fn get_report_size(this: &HidReportItem) -> Option<u16>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `reportSize` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportItem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "reportSize")]
    pub fn set_report_size(this: &HidReportItem, val: u16);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `strings` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportItem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "strings")]
    pub fn get_strings(this: &HidReportItem) -> Option<::js_sys::Array<::js_sys::JsString>>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `strings` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportItem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "strings")]
    pub fn set_strings(this: &HidReportItem, val: &[::js_sys::JsString]);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `unitExponent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportItem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "unitExponent")]
    pub fn get_unit_exponent(this: &HidReportItem) -> Option<i8>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `unitExponent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportItem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "unitExponent")]
    pub fn set_unit_exponent(this: &HidReportItem, val: i8);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `unitFactorCurrentExponent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportItem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "unitFactorCurrentExponent")]
    pub fn get_unit_factor_current_exponent(this: &HidReportItem) -> Option<i8>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `unitFactorCurrentExponent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportItem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "unitFactorCurrentExponent")]
    pub fn set_unit_factor_current_exponent(this: &HidReportItem, val: i8);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `unitFactorLengthExponent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportItem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "unitFactorLengthExponent")]
    pub fn get_unit_factor_length_exponent(this: &HidReportItem) -> Option<i8>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `unitFactorLengthExponent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportItem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "unitFactorLengthExponent")]
    pub fn set_unit_factor_length_exponent(this: &HidReportItem, val: i8);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `unitFactorLuminousIntensityExponent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportItem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "unitFactorLuminousIntensityExponent")]
    pub fn get_unit_factor_luminous_intensity_exponent(this: &HidReportItem) -> Option<i8>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `unitFactorLuminousIntensityExponent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportItem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "unitFactorLuminousIntensityExponent")]
    pub fn set_unit_factor_luminous_intensity_exponent(this: &HidReportItem, val: i8);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `unitFactorMassExponent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportItem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "unitFactorMassExponent")]
    pub fn get_unit_factor_mass_exponent(this: &HidReportItem) -> Option<i8>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `unitFactorMassExponent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportItem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "unitFactorMassExponent")]
    pub fn set_unit_factor_mass_exponent(this: &HidReportItem, val: i8);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `unitFactorTemperatureExponent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportItem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "unitFactorTemperatureExponent")]
    pub fn get_unit_factor_temperature_exponent(this: &HidReportItem) -> Option<i8>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `unitFactorTemperatureExponent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportItem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "unitFactorTemperatureExponent")]
    pub fn set_unit_factor_temperature_exponent(this: &HidReportItem, val: i8);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `unitFactorTimeExponent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportItem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "unitFactorTimeExponent")]
    pub fn get_unit_factor_time_exponent(this: &HidReportItem) -> Option<i8>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `unitFactorTimeExponent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportItem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "unitFactorTimeExponent")]
    pub fn set_unit_factor_time_exponent(this: &HidReportItem, val: i8);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "HidUnitSystem")]
    #[doc = "Get the `unitSystem` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportItem`, `HidUnitSystem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "unitSystem")]
    pub fn get_unit_system(this: &HidReportItem) -> Option<HidUnitSystem>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "HidUnitSystem")]
    #[doc = "Change the `unitSystem` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportItem`, `HidUnitSystem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "unitSystem")]
    pub fn set_unit_system(this: &HidReportItem, val: HidUnitSystem);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `usageMaximum` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportItem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "usageMaximum")]
    pub fn get_usage_maximum(this: &HidReportItem) -> Option<u32>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `usageMaximum` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportItem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "usageMaximum")]
    pub fn set_usage_maximum(this: &HidReportItem, val: u32);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `usageMinimum` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportItem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "usageMinimum")]
    pub fn get_usage_minimum(this: &HidReportItem) -> Option<u32>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `usageMinimum` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportItem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "usageMinimum")]
    pub fn set_usage_minimum(this: &HidReportItem, val: u32);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `usages` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportItem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "usages")]
    pub fn get_usages(this: &HidReportItem) -> Option<::js_sys::Array<::js_sys::Number>>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `usages` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportItem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "usages")]
    pub fn set_usages(this: &HidReportItem, val: &[::js_sys::Number]);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `wrap` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportItem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "wrap")]
    pub fn get_wrap(this: &HidReportItem) -> Option<bool>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `wrap` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportItem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "wrap")]
    pub fn set_wrap(this: &HidReportItem, val: bool);
}
#[cfg(web_sys_unstable_apis)]
impl HidReportItem {
    #[doc = "Construct a new `HidReportItem`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportItem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_has_null()` instead."]
    pub fn has_null(&mut self, val: bool) -> &mut Self {
        self.set_has_null(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_has_preferred_state()` instead."]
    pub fn has_preferred_state(&mut self, val: bool) -> &mut Self {
        self.set_has_preferred_state(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_is_absolute()` instead."]
    pub fn is_absolute(&mut self, val: bool) -> &mut Self {
        self.set_is_absolute(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_is_array()` instead."]
    pub fn is_array(&mut self, val: bool) -> &mut Self {
        self.set_is_array(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_is_buffered_bytes()` instead."]
    pub fn is_buffered_bytes(&mut self, val: bool) -> &mut Self {
        self.set_is_buffered_bytes(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_is_constant()` instead."]
    pub fn is_constant(&mut self, val: bool) -> &mut Self {
        self.set_is_constant(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_is_linear()` instead."]
    pub fn is_linear(&mut self, val: bool) -> &mut Self {
        self.set_is_linear(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_is_range()` instead."]
    pub fn is_range(&mut self, val: bool) -> &mut Self {
        self.set_is_range(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_is_volatile()` instead."]
    pub fn is_volatile(&mut self, val: bool) -> &mut Self {
        self.set_is_volatile(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_logical_maximum()` instead."]
    pub fn logical_maximum(&mut self, val: i32) -> &mut Self {
        self.set_logical_maximum(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_logical_minimum()` instead."]
    pub fn logical_minimum(&mut self, val: i32) -> &mut Self {
        self.set_logical_minimum(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_physical_maximum()` instead."]
    pub fn physical_maximum(&mut self, val: i32) -> &mut Self {
        self.set_physical_maximum(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_physical_minimum()` instead."]
    pub fn physical_minimum(&mut self, val: i32) -> &mut Self {
        self.set_physical_minimum(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_report_count()` instead."]
    pub fn report_count(&mut self, val: u16) -> &mut Self {
        self.set_report_count(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_report_size()` instead."]
    pub fn report_size(&mut self, val: u16) -> &mut Self {
        self.set_report_size(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_strings()` instead."]
    pub fn strings(&mut self, val: &[::js_sys::JsString]) -> &mut Self {
        self.set_strings(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_unit_exponent()` instead."]
    pub fn unit_exponent(&mut self, val: i8) -> &mut Self {
        self.set_unit_exponent(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_unit_factor_current_exponent()` instead."]
    pub fn unit_factor_current_exponent(&mut self, val: i8) -> &mut Self {
        self.set_unit_factor_current_exponent(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_unit_factor_length_exponent()` instead."]
    pub fn unit_factor_length_exponent(&mut self, val: i8) -> &mut Self {
        self.set_unit_factor_length_exponent(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_unit_factor_luminous_intensity_exponent()` instead."]
    pub fn unit_factor_luminous_intensity_exponent(&mut self, val: i8) -> &mut Self {
        self.set_unit_factor_luminous_intensity_exponent(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_unit_factor_mass_exponent()` instead."]
    pub fn unit_factor_mass_exponent(&mut self, val: i8) -> &mut Self {
        self.set_unit_factor_mass_exponent(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_unit_factor_temperature_exponent()` instead."]
    pub fn unit_factor_temperature_exponent(&mut self, val: i8) -> &mut Self {
        self.set_unit_factor_temperature_exponent(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_unit_factor_time_exponent()` instead."]
    pub fn unit_factor_time_exponent(&mut self, val: i8) -> &mut Self {
        self.set_unit_factor_time_exponent(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "HidUnitSystem")]
    #[deprecated = "Use `set_unit_system()` instead."]
    pub fn unit_system(&mut self, val: HidUnitSystem) -> &mut Self {
        self.set_unit_system(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_usage_maximum()` instead."]
    pub fn usage_maximum(&mut self, val: u32) -> &mut Self {
        self.set_usage_maximum(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_usage_minimum()` instead."]
    pub fn usage_minimum(&mut self, val: u32) -> &mut Self {
        self.set_usage_minimum(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_usages()` instead."]
    pub fn usages(&mut self, val: &[::js_sys::Number]) -> &mut Self {
        self.set_usages(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_wrap()` instead."]
    pub fn wrap(&mut self, val: bool) -> &mut Self {
        self.set_wrap(val);
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for HidReportItem {
    fn default() -> Self {
        Self::new()
    }
}
