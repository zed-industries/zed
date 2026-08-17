#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "RcwnStatus")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RcwnStatus` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RcwnStatus`*"]
    pub type RcwnStatus;
    #[doc = "Get the `cacheNotSlowCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RcwnStatus`*"]
    #[wasm_bindgen(method, getter = "cacheNotSlowCount")]
    pub fn get_cache_not_slow_count(this: &RcwnStatus) -> Option<u32>;
    #[doc = "Change the `cacheNotSlowCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RcwnStatus`*"]
    #[wasm_bindgen(method, setter = "cacheNotSlowCount")]
    pub fn set_cache_not_slow_count(this: &RcwnStatus, val: u32);
    #[doc = "Get the `cacheSlowCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RcwnStatus`*"]
    #[wasm_bindgen(method, getter = "cacheSlowCount")]
    pub fn get_cache_slow_count(this: &RcwnStatus) -> Option<u32>;
    #[doc = "Change the `cacheSlowCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RcwnStatus`*"]
    #[wasm_bindgen(method, setter = "cacheSlowCount")]
    pub fn set_cache_slow_count(this: &RcwnStatus, val: u32);
    #[doc = "Get the `perfStats` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RcwnStatus`*"]
    #[wasm_bindgen(method, getter = "perfStats")]
    pub fn get_perf_stats(this: &RcwnStatus) -> Option<::js_sys::Array>;
    #[doc = "Change the `perfStats` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RcwnStatus`*"]
    #[wasm_bindgen(method, setter = "perfStats")]
    pub fn set_perf_stats(this: &RcwnStatus, val: &::wasm_bindgen::JsValue);
    #[doc = "Get the `rcwnCacheWonCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RcwnStatus`*"]
    #[wasm_bindgen(method, getter = "rcwnCacheWonCount")]
    pub fn get_rcwn_cache_won_count(this: &RcwnStatus) -> Option<u32>;
    #[doc = "Change the `rcwnCacheWonCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RcwnStatus`*"]
    #[wasm_bindgen(method, setter = "rcwnCacheWonCount")]
    pub fn set_rcwn_cache_won_count(this: &RcwnStatus, val: u32);
    #[doc = "Get the `rcwnNetWonCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RcwnStatus`*"]
    #[wasm_bindgen(method, getter = "rcwnNetWonCount")]
    pub fn get_rcwn_net_won_count(this: &RcwnStatus) -> Option<u32>;
    #[doc = "Change the `rcwnNetWonCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RcwnStatus`*"]
    #[wasm_bindgen(method, setter = "rcwnNetWonCount")]
    pub fn set_rcwn_net_won_count(this: &RcwnStatus, val: u32);
    #[doc = "Get the `totalNetworkRequests` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RcwnStatus`*"]
    #[wasm_bindgen(method, getter = "totalNetworkRequests")]
    pub fn get_total_network_requests(this: &RcwnStatus) -> Option<u32>;
    #[doc = "Change the `totalNetworkRequests` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RcwnStatus`*"]
    #[wasm_bindgen(method, setter = "totalNetworkRequests")]
    pub fn set_total_network_requests(this: &RcwnStatus, val: u32);
}
impl RcwnStatus {
    #[doc = "Construct a new `RcwnStatus`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RcwnStatus`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_cache_not_slow_count()` instead."]
    pub fn cache_not_slow_count(&mut self, val: u32) -> &mut Self {
        self.set_cache_not_slow_count(val);
        self
    }
    #[deprecated = "Use `set_cache_slow_count()` instead."]
    pub fn cache_slow_count(&mut self, val: u32) -> &mut Self {
        self.set_cache_slow_count(val);
        self
    }
    #[deprecated = "Use `set_perf_stats()` instead."]
    pub fn perf_stats(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_perf_stats(val);
        self
    }
    #[deprecated = "Use `set_rcwn_cache_won_count()` instead."]
    pub fn rcwn_cache_won_count(&mut self, val: u32) -> &mut Self {
        self.set_rcwn_cache_won_count(val);
        self
    }
    #[deprecated = "Use `set_rcwn_net_won_count()` instead."]
    pub fn rcwn_net_won_count(&mut self, val: u32) -> &mut Self {
        self.set_rcwn_net_won_count(val);
        self
    }
    #[deprecated = "Use `set_total_network_requests()` instead."]
    pub fn total_network_requests(&mut self, val: u32) -> &mut Self {
        self.set_total_network_requests(val);
        self
    }
}
impl Default for RcwnStatus {
    fn default() -> Self {
        Self::new()
    }
}
