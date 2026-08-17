#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "RTCStatsReport",
        typescript_type = "RTCStatsReport"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcStatsReport` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCStatsReport)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsReport`*"]
    pub type RtcStatsReport;
    #[wasm_bindgen(method, getter, js_class = "RTCStatsReport", js_name = "size")]
    #[doc = "Getter for the `size` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCStatsReport/size)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsReport`*"]
    pub fn size(this: &RtcStatsReport) -> u32;
    #[wasm_bindgen(catch, method, js_class = "RTCStatsReport", js_name = "forEach")]
    #[doc = "The `forEach()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCStatsReport/forEach)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsReport`*"]
    pub fn for_each(this: &RtcStatsReport, callback: &::js_sys::Function) -> Result<(), JsValue>;
    #[wasm_bindgen(method, js_class = "RTCStatsReport")]
    #[doc = "The `get()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCStatsReport/get)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsReport`*"]
    pub fn get(this: &RtcStatsReport, key: &str) -> Option<::js_sys::Object>;
    #[wasm_bindgen(method, js_class = "RTCStatsReport")]
    #[doc = "The `has()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCStatsReport/has)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsReport`*"]
    pub fn has(this: &RtcStatsReport, key: &str) -> bool;
    #[wasm_bindgen(method, js_class = "RTCStatsReport")]
    #[doc = "The `entries()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCStatsReport/entries)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsReport`*"]
    pub fn entries(this: &RtcStatsReport) -> ::js_sys::Iterator;
    #[wasm_bindgen(method, js_class = "RTCStatsReport")]
    #[doc = "The `keys()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCStatsReport/keys)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsReport`*"]
    pub fn keys(this: &RtcStatsReport) -> ::js_sys::Iterator;
    #[wasm_bindgen(method, js_class = "RTCStatsReport")]
    #[doc = "The `values()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCStatsReport/values)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsReport`*"]
    pub fn values(this: &RtcStatsReport) -> ::js_sys::Iterator;
}
