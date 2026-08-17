#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "RcwnPerfStats")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RcwnPerfStats` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RcwnPerfStats`*"]
    pub type RcwnPerfStats;
    #[doc = "Get the `avgLong` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RcwnPerfStats`*"]
    #[wasm_bindgen(method, getter = "avgLong")]
    pub fn get_avg_long(this: &RcwnPerfStats) -> Option<u32>;
    #[doc = "Change the `avgLong` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RcwnPerfStats`*"]
    #[wasm_bindgen(method, setter = "avgLong")]
    pub fn set_avg_long(this: &RcwnPerfStats, val: u32);
    #[doc = "Get the `avgShort` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RcwnPerfStats`*"]
    #[wasm_bindgen(method, getter = "avgShort")]
    pub fn get_avg_short(this: &RcwnPerfStats) -> Option<u32>;
    #[doc = "Change the `avgShort` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RcwnPerfStats`*"]
    #[wasm_bindgen(method, setter = "avgShort")]
    pub fn set_avg_short(this: &RcwnPerfStats, val: u32);
    #[doc = "Get the `stddevLong` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RcwnPerfStats`*"]
    #[wasm_bindgen(method, getter = "stddevLong")]
    pub fn get_stddev_long(this: &RcwnPerfStats) -> Option<u32>;
    #[doc = "Change the `stddevLong` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RcwnPerfStats`*"]
    #[wasm_bindgen(method, setter = "stddevLong")]
    pub fn set_stddev_long(this: &RcwnPerfStats, val: u32);
}
impl RcwnPerfStats {
    #[doc = "Construct a new `RcwnPerfStats`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RcwnPerfStats`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_avg_long()` instead."]
    pub fn avg_long(&mut self, val: u32) -> &mut Self {
        self.set_avg_long(val);
        self
    }
    #[deprecated = "Use `set_avg_short()` instead."]
    pub fn avg_short(&mut self, val: u32) -> &mut Self {
        self.set_avg_short(val);
        self
    }
    #[deprecated = "Use `set_stddev_long()` instead."]
    pub fn stddev_long(&mut self, val: u32) -> &mut Self {
        self.set_stddev_long(val);
        self
    }
}
impl Default for RcwnPerfStats {
    fn default() -> Self {
        Self::new()
    }
}
