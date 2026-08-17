#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (is_type_of = | _ | false , extends = "::js_sys::Object" , js_name = "EXT_disjoint_timer_query" , typescript_type = "EXT_disjoint_timer_query")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ExtDisjointTimerQuery` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EXT_disjoint_timer_query)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtDisjointTimerQuery`*"]
    pub type ExtDisjointTimerQuery;
    #[cfg(feature = "WebGlQuery")]
    #[wasm_bindgen(
        method,
        js_class = "EXT_disjoint_timer_query",
        js_name = "beginQueryEXT"
    )]
    #[doc = "The `beginQueryEXT()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EXT_disjoint_timer_query/beginQueryEXT)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtDisjointTimerQuery`, `WebGlQuery`*"]
    pub fn begin_query_ext(this: &ExtDisjointTimerQuery, target: u32, query: &WebGlQuery);
    #[cfg(feature = "WebGlQuery")]
    #[wasm_bindgen(
        method,
        js_class = "EXT_disjoint_timer_query",
        js_name = "createQueryEXT"
    )]
    #[doc = "The `createQueryEXT()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EXT_disjoint_timer_query/createQueryEXT)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtDisjointTimerQuery`, `WebGlQuery`*"]
    pub fn create_query_ext(this: &ExtDisjointTimerQuery) -> Option<WebGlQuery>;
    #[cfg(feature = "WebGlQuery")]
    #[wasm_bindgen(
        method,
        js_class = "EXT_disjoint_timer_query",
        js_name = "deleteQueryEXT"
    )]
    #[doc = "The `deleteQueryEXT()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EXT_disjoint_timer_query/deleteQueryEXT)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtDisjointTimerQuery`, `WebGlQuery`*"]
    pub fn delete_query_ext(this: &ExtDisjointTimerQuery, query: Option<&WebGlQuery>);
    #[wasm_bindgen(method, js_class = "EXT_disjoint_timer_query", js_name = "endQueryEXT")]
    #[doc = "The `endQueryEXT()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EXT_disjoint_timer_query/endQueryEXT)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtDisjointTimerQuery`*"]
    pub fn end_query_ext(this: &ExtDisjointTimerQuery, target: u32);
    #[wasm_bindgen(method, js_class = "EXT_disjoint_timer_query", js_name = "getQueryEXT")]
    #[doc = "The `getQueryEXT()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EXT_disjoint_timer_query/getQueryEXT)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtDisjointTimerQuery`*"]
    pub fn get_query_ext(
        this: &ExtDisjointTimerQuery,
        target: u32,
        pname: u32,
    ) -> ::wasm_bindgen::JsValue;
    #[cfg(feature = "WebGlQuery")]
    #[wasm_bindgen(
        method,
        js_class = "EXT_disjoint_timer_query",
        js_name = "getQueryObjectEXT"
    )]
    #[doc = "The `getQueryObjectEXT()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EXT_disjoint_timer_query/getQueryObjectEXT)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtDisjointTimerQuery`, `WebGlQuery`*"]
    pub fn get_query_object_ext(
        this: &ExtDisjointTimerQuery,
        query: &WebGlQuery,
        pname: u32,
    ) -> ::wasm_bindgen::JsValue;
    #[cfg(feature = "WebGlQuery")]
    #[wasm_bindgen(method, js_class = "EXT_disjoint_timer_query", js_name = "isQueryEXT")]
    #[doc = "The `isQueryEXT()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EXT_disjoint_timer_query/isQueryEXT)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtDisjointTimerQuery`, `WebGlQuery`*"]
    pub fn is_query_ext(this: &ExtDisjointTimerQuery, query: Option<&WebGlQuery>) -> bool;
    #[cfg(feature = "WebGlQuery")]
    #[wasm_bindgen(
        method,
        js_class = "EXT_disjoint_timer_query",
        js_name = "queryCounterEXT"
    )]
    #[doc = "The `queryCounterEXT()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EXT_disjoint_timer_query/queryCounterEXT)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtDisjointTimerQuery`, `WebGlQuery`*"]
    pub fn query_counter_ext(this: &ExtDisjointTimerQuery, query: &WebGlQuery, target: u32);
}
impl ExtDisjointTimerQuery {
    #[doc = "The `EXT_disjoint_timer_query.QUERY_COUNTER_BITS_EXT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtDisjointTimerQuery`*"]
    pub const QUERY_COUNTER_BITS_EXT: u32 = 34916u64 as u32;
    #[doc = "The `EXT_disjoint_timer_query.CURRENT_QUERY_EXT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtDisjointTimerQuery`*"]
    pub const CURRENT_QUERY_EXT: u32 = 34917u64 as u32;
    #[doc = "The `EXT_disjoint_timer_query.QUERY_RESULT_EXT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtDisjointTimerQuery`*"]
    pub const QUERY_RESULT_EXT: u32 = 34918u64 as u32;
    #[doc = "The `EXT_disjoint_timer_query.QUERY_RESULT_AVAILABLE_EXT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtDisjointTimerQuery`*"]
    pub const QUERY_RESULT_AVAILABLE_EXT: u32 = 34919u64 as u32;
    #[doc = "The `EXT_disjoint_timer_query.TIME_ELAPSED_EXT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtDisjointTimerQuery`*"]
    pub const TIME_ELAPSED_EXT: u32 = 35007u64 as u32;
    #[doc = "The `EXT_disjoint_timer_query.TIMESTAMP_EXT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtDisjointTimerQuery`*"]
    pub const TIMESTAMP_EXT: u32 = 36392u64 as u32;
    #[doc = "The `EXT_disjoint_timer_query.GPU_DISJOINT_EXT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtDisjointTimerQuery`*"]
    pub const GPU_DISJOINT_EXT: u32 = 36795u64 as u32;
}
