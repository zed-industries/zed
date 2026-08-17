#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "CheckerboardReport")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CheckerboardReport` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CheckerboardReport`*"]
    pub type CheckerboardReport;
    #[doc = "Get the `log` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CheckerboardReport`*"]
    #[wasm_bindgen(method, getter = "log")]
    pub fn get_log(this: &CheckerboardReport) -> Option<::alloc::string::String>;
    #[doc = "Change the `log` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CheckerboardReport`*"]
    #[wasm_bindgen(method, setter = "log")]
    pub fn set_log(this: &CheckerboardReport, val: &str);
    #[cfg(feature = "CheckerboardReason")]
    #[doc = "Get the `reason` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CheckerboardReason`, `CheckerboardReport`*"]
    #[wasm_bindgen(method, getter = "reason")]
    pub fn get_reason(this: &CheckerboardReport) -> Option<CheckerboardReason>;
    #[cfg(feature = "CheckerboardReason")]
    #[doc = "Change the `reason` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CheckerboardReason`, `CheckerboardReport`*"]
    #[wasm_bindgen(method, setter = "reason")]
    pub fn set_reason(this: &CheckerboardReport, val: CheckerboardReason);
    #[doc = "Get the `severity` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CheckerboardReport`*"]
    #[wasm_bindgen(method, getter = "severity")]
    pub fn get_severity(this: &CheckerboardReport) -> Option<u32>;
    #[doc = "Change the `severity` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CheckerboardReport`*"]
    #[wasm_bindgen(method, setter = "severity")]
    pub fn set_severity(this: &CheckerboardReport, val: u32);
    #[doc = "Get the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CheckerboardReport`*"]
    #[wasm_bindgen(method, getter = "timestamp")]
    pub fn get_timestamp(this: &CheckerboardReport) -> Option<f64>;
    #[doc = "Change the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CheckerboardReport`*"]
    #[wasm_bindgen(method, setter = "timestamp")]
    pub fn set_timestamp(this: &CheckerboardReport, val: f64);
    #[doc = "Change the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CheckerboardReport`*"]
    #[wasm_bindgen(method, setter = "timestamp")]
    pub fn set_timestamp_u32(this: &CheckerboardReport, val: u32);
    #[doc = "Change the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CheckerboardReport`*"]
    #[wasm_bindgen(method, setter = "timestamp")]
    pub fn set_timestamp_f64(this: &CheckerboardReport, val: f64);
}
impl CheckerboardReport {
    #[doc = "Construct a new `CheckerboardReport`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CheckerboardReport`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_log()` instead."]
    pub fn log(&mut self, val: &str) -> &mut Self {
        self.set_log(val);
        self
    }
    #[cfg(feature = "CheckerboardReason")]
    #[deprecated = "Use `set_reason()` instead."]
    pub fn reason(&mut self, val: CheckerboardReason) -> &mut Self {
        self.set_reason(val);
        self
    }
    #[deprecated = "Use `set_severity()` instead."]
    pub fn severity(&mut self, val: u32) -> &mut Self {
        self.set_severity(val);
        self
    }
    #[deprecated = "Use `set_timestamp()` instead."]
    pub fn timestamp(&mut self, val: f64) -> &mut Self {
        self.set_timestamp(val);
        self
    }
}
impl Default for CheckerboardReport {
    fn default() -> Self {
        Self::new()
    }
}
