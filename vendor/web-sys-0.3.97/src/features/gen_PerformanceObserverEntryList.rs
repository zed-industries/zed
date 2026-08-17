#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "PerformanceObserverEntryList",
        typescript_type = "PerformanceObserverEntryList"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PerformanceObserverEntryList` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceObserverEntryList)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceObserverEntryList`*"]
    pub type PerformanceObserverEntryList;
    #[wasm_bindgen(
        method,
        js_class = "PerformanceObserverEntryList",
        js_name = "getEntries"
    )]
    #[doc = "The `getEntries()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceObserverEntryList/getEntries)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceObserverEntryList`*"]
    pub fn get_entries(this: &PerformanceObserverEntryList) -> ::js_sys::Array;
    #[cfg(feature = "PerformanceEntryFilterOptions")]
    #[wasm_bindgen(
        method,
        js_class = "PerformanceObserverEntryList",
        js_name = "getEntries"
    )]
    #[doc = "The `getEntries()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceObserverEntryList/getEntries)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceEntryFilterOptions`, `PerformanceObserverEntryList`*"]
    pub fn get_entries_with_filter(
        this: &PerformanceObserverEntryList,
        filter: &PerformanceEntryFilterOptions,
    ) -> ::js_sys::Array;
    #[wasm_bindgen(
        method,
        js_class = "PerformanceObserverEntryList",
        js_name = "getEntriesByName"
    )]
    #[doc = "The `getEntriesByName()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceObserverEntryList/getEntriesByName)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceObserverEntryList`*"]
    pub fn get_entries_by_name(this: &PerformanceObserverEntryList, name: &str) -> ::js_sys::Array;
    #[wasm_bindgen(
        method,
        js_class = "PerformanceObserverEntryList",
        js_name = "getEntriesByName"
    )]
    #[doc = "The `getEntriesByName()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceObserverEntryList/getEntriesByName)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceObserverEntryList`*"]
    pub fn get_entries_by_name_with_entry_type(
        this: &PerformanceObserverEntryList,
        name: &str,
        entry_type: &str,
    ) -> ::js_sys::Array;
    #[wasm_bindgen(
        method,
        js_class = "PerformanceObserverEntryList",
        js_name = "getEntriesByType"
    )]
    #[doc = "The `getEntriesByType()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceObserverEntryList/getEntriesByType)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceObserverEntryList`*"]
    pub fn get_entries_by_type(
        this: &PerformanceObserverEntryList,
        entry_type: &str,
    ) -> ::js_sys::Array;
}
