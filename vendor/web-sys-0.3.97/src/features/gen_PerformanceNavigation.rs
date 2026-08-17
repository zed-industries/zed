#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "PerformanceNavigation",
        typescript_type = "PerformanceNavigation"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PerformanceNavigation` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceNavigation)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceNavigation`*"]
    pub type PerformanceNavigation;
    #[wasm_bindgen(method, getter, js_class = "PerformanceNavigation", js_name = "type")]
    #[doc = "Getter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceNavigation/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceNavigation`*"]
    pub fn type_(this: &PerformanceNavigation) -> u16;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "PerformanceNavigation",
        js_name = "redirectCount"
    )]
    #[doc = "Getter for the `redirectCount` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceNavigation/redirectCount)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceNavigation`*"]
    pub fn redirect_count(this: &PerformanceNavigation) -> u16;
    #[wasm_bindgen(method, js_class = "PerformanceNavigation", js_name = "toJSON")]
    #[doc = "The `toJSON()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceNavigation/toJSON)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceNavigation`*"]
    pub fn to_json(this: &PerformanceNavigation) -> ::js_sys::Object;
}
impl PerformanceNavigation {
    #[doc = "The `PerformanceNavigation.TYPE_NAVIGATE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceNavigation`*"]
    pub const TYPE_NAVIGATE: u16 = 0i64 as u16;
    #[doc = "The `PerformanceNavigation.TYPE_RELOAD` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceNavigation`*"]
    pub const TYPE_RELOAD: u16 = 1u64 as u16;
    #[doc = "The `PerformanceNavigation.TYPE_BACK_FORWARD` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceNavigation`*"]
    pub const TYPE_BACK_FORWARD: u16 = 2u64 as u16;
    #[doc = "The `PerformanceNavigation.TYPE_RESERVED` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceNavigation`*"]
    pub const TYPE_RESERVED: u16 = 255u64 as u16;
}
