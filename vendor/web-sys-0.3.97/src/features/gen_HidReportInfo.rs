#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "HIDReportInfo")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HidReportInfo` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportInfo`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type HidReportInfo;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "HidReportItem")]
    #[doc = "Get the `items` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportInfo`, `HidReportItem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "items")]
    pub fn get_items(this: &HidReportInfo) -> Option<::js_sys::Array<HidReportItem>>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "HidReportItem")]
    #[doc = "Change the `items` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportInfo`, `HidReportItem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "items")]
    pub fn set_items(this: &HidReportInfo, val: &[HidReportItem]);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `reportId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportInfo`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "reportId")]
    pub fn get_report_id(this: &HidReportInfo) -> Option<u8>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `reportId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportInfo`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "reportId")]
    pub fn set_report_id(this: &HidReportInfo, val: u8);
}
#[cfg(web_sys_unstable_apis)]
impl HidReportInfo {
    #[doc = "Construct a new `HidReportInfo`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidReportInfo`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "HidReportItem")]
    #[deprecated = "Use `set_items()` instead."]
    pub fn items(&mut self, val: &[HidReportItem]) -> &mut Self {
        self.set_items(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_report_id()` instead."]
    pub fn report_id(&mut self, val: u8) -> &mut Self {
        self.set_report_id(val);
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for HidReportInfo {
    fn default() -> Self {
        Self::new()
    }
}
