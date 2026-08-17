#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "DateTimeValue")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DateTimeValue` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DateTimeValue`*"]
    pub type DateTimeValue;
    #[doc = "Get the `day` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DateTimeValue`*"]
    #[wasm_bindgen(method, getter = "day")]
    pub fn get_day(this: &DateTimeValue) -> Option<i32>;
    #[doc = "Change the `day` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DateTimeValue`*"]
    #[wasm_bindgen(method, setter = "day")]
    pub fn set_day(this: &DateTimeValue, val: i32);
    #[doc = "Get the `hour` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DateTimeValue`*"]
    #[wasm_bindgen(method, getter = "hour")]
    pub fn get_hour(this: &DateTimeValue) -> Option<i32>;
    #[doc = "Change the `hour` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DateTimeValue`*"]
    #[wasm_bindgen(method, setter = "hour")]
    pub fn set_hour(this: &DateTimeValue, val: i32);
    #[doc = "Get the `minute` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DateTimeValue`*"]
    #[wasm_bindgen(method, getter = "minute")]
    pub fn get_minute(this: &DateTimeValue) -> Option<i32>;
    #[doc = "Change the `minute` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DateTimeValue`*"]
    #[wasm_bindgen(method, setter = "minute")]
    pub fn set_minute(this: &DateTimeValue, val: i32);
    #[doc = "Get the `month` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DateTimeValue`*"]
    #[wasm_bindgen(method, getter = "month")]
    pub fn get_month(this: &DateTimeValue) -> Option<i32>;
    #[doc = "Change the `month` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DateTimeValue`*"]
    #[wasm_bindgen(method, setter = "month")]
    pub fn set_month(this: &DateTimeValue, val: i32);
    #[doc = "Get the `year` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DateTimeValue`*"]
    #[wasm_bindgen(method, getter = "year")]
    pub fn get_year(this: &DateTimeValue) -> Option<i32>;
    #[doc = "Change the `year` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DateTimeValue`*"]
    #[wasm_bindgen(method, setter = "year")]
    pub fn set_year(this: &DateTimeValue, val: i32);
}
impl DateTimeValue {
    #[doc = "Construct a new `DateTimeValue`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DateTimeValue`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_day()` instead."]
    pub fn day(&mut self, val: i32) -> &mut Self {
        self.set_day(val);
        self
    }
    #[deprecated = "Use `set_hour()` instead."]
    pub fn hour(&mut self, val: i32) -> &mut Self {
        self.set_hour(val);
        self
    }
    #[deprecated = "Use `set_minute()` instead."]
    pub fn minute(&mut self, val: i32) -> &mut Self {
        self.set_minute(val);
        self
    }
    #[deprecated = "Use `set_month()` instead."]
    pub fn month(&mut self, val: i32) -> &mut Self {
        self.set_month(val);
        self
    }
    #[deprecated = "Use `set_year()` instead."]
    pub fn year(&mut self, val: i32) -> &mut Self {
        self.set_year(val);
        self
    }
}
impl Default for DateTimeValue {
    fn default() -> Self {
        Self::new()
    }
}
