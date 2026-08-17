#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "HIDCollectionInfo")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HidCollectionInfo` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidCollectionInfo`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type HidCollectionInfo;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `children` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidCollectionInfo`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "children")]
    pub fn get_children(this: &HidCollectionInfo) -> Option<::js_sys::Array<HidCollectionInfo>>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `children` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidCollectionInfo`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "children")]
    pub fn set_children(this: &HidCollectionInfo, val: &[HidCollectionInfo]);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "HidReportInfo")]
    #[doc = "Get the `featureReports` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidCollectionInfo`, `HidReportInfo`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "featureReports")]
    pub fn get_feature_reports(this: &HidCollectionInfo) -> Option<::js_sys::Array<HidReportInfo>>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "HidReportInfo")]
    #[doc = "Change the `featureReports` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidCollectionInfo`, `HidReportInfo`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "featureReports")]
    pub fn set_feature_reports(this: &HidCollectionInfo, val: &[HidReportInfo]);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "HidReportInfo")]
    #[doc = "Get the `inputReports` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidCollectionInfo`, `HidReportInfo`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "inputReports")]
    pub fn get_input_reports(this: &HidCollectionInfo) -> Option<::js_sys::Array<HidReportInfo>>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "HidReportInfo")]
    #[doc = "Change the `inputReports` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidCollectionInfo`, `HidReportInfo`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "inputReports")]
    pub fn set_input_reports(this: &HidCollectionInfo, val: &[HidReportInfo]);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "HidReportInfo")]
    #[doc = "Get the `outputReports` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidCollectionInfo`, `HidReportInfo`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "outputReports")]
    pub fn get_output_reports(this: &HidCollectionInfo) -> Option<::js_sys::Array<HidReportInfo>>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "HidReportInfo")]
    #[doc = "Change the `outputReports` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidCollectionInfo`, `HidReportInfo`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "outputReports")]
    pub fn set_output_reports(this: &HidCollectionInfo, val: &[HidReportInfo]);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidCollectionInfo`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "type")]
    pub fn get_type(this: &HidCollectionInfo) -> Option<u8>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidCollectionInfo`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "type")]
    pub fn set_type(this: &HidCollectionInfo, val: u8);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `usage` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidCollectionInfo`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "usage")]
    pub fn get_usage(this: &HidCollectionInfo) -> Option<u16>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `usage` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidCollectionInfo`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "usage")]
    pub fn set_usage(this: &HidCollectionInfo, val: u16);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `usagePage` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidCollectionInfo`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "usagePage")]
    pub fn get_usage_page(this: &HidCollectionInfo) -> Option<u16>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `usagePage` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidCollectionInfo`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "usagePage")]
    pub fn set_usage_page(this: &HidCollectionInfo, val: u16);
}
#[cfg(web_sys_unstable_apis)]
impl HidCollectionInfo {
    #[doc = "Construct a new `HidCollectionInfo`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidCollectionInfo`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_children()` instead."]
    pub fn children(&mut self, val: &[HidCollectionInfo]) -> &mut Self {
        self.set_children(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "HidReportInfo")]
    #[deprecated = "Use `set_feature_reports()` instead."]
    pub fn feature_reports(&mut self, val: &[HidReportInfo]) -> &mut Self {
        self.set_feature_reports(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "HidReportInfo")]
    #[deprecated = "Use `set_input_reports()` instead."]
    pub fn input_reports(&mut self, val: &[HidReportInfo]) -> &mut Self {
        self.set_input_reports(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "HidReportInfo")]
    #[deprecated = "Use `set_output_reports()` instead."]
    pub fn output_reports(&mut self, val: &[HidReportInfo]) -> &mut Self {
        self.set_output_reports(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_type()` instead."]
    pub fn type_(&mut self, val: u8) -> &mut Self {
        self.set_type(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_usage()` instead."]
    pub fn usage(&mut self, val: u16) -> &mut Self {
        self.set_usage(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_usage_page()` instead."]
    pub fn usage_page(&mut self, val: u16) -> &mut Self {
        self.set_usage_page(val);
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for HidCollectionInfo {
    fn default() -> Self {
        Self::new()
    }
}
