#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "CheckerboardReportService",
        typescript_type = "CheckerboardReportService"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CheckerboardReportService` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CheckerboardReportService)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CheckerboardReportService`*"]
    pub type CheckerboardReportService;
    #[wasm_bindgen(catch, constructor, js_class = "CheckerboardReportService")]
    #[doc = "The `new CheckerboardReportService(..)` constructor, creating a new instance of `CheckerboardReportService`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CheckerboardReportService/CheckerboardReportService)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CheckerboardReportService`*"]
    pub fn new() -> Result<CheckerboardReportService, JsValue>;
    #[wasm_bindgen(
        method,
        js_class = "CheckerboardReportService",
        js_name = "flushActiveReports"
    )]
    #[doc = "The `flushActiveReports()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CheckerboardReportService/flushActiveReports)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CheckerboardReportService`*"]
    pub fn flush_active_reports(this: &CheckerboardReportService);
    #[wasm_bindgen(method, js_class = "CheckerboardReportService", js_name = "getReports")]
    #[doc = "The `getReports()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CheckerboardReportService/getReports)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CheckerboardReportService`*"]
    pub fn get_reports(this: &CheckerboardReportService) -> ::js_sys::Array;
    #[wasm_bindgen(
        method,
        js_class = "CheckerboardReportService",
        js_name = "isRecordingEnabled"
    )]
    #[doc = "The `isRecordingEnabled()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CheckerboardReportService/isRecordingEnabled)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CheckerboardReportService`*"]
    pub fn is_recording_enabled(this: &CheckerboardReportService) -> bool;
    #[wasm_bindgen(
        method,
        js_class = "CheckerboardReportService",
        js_name = "setRecordingEnabled"
    )]
    #[doc = "The `setRecordingEnabled()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CheckerboardReportService/setRecordingEnabled)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CheckerboardReportService`*"]
    pub fn set_recording_enabled(this: &CheckerboardReportService, a_enabled: bool);
}
