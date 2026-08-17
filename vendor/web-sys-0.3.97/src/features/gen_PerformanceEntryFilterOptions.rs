#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "PerformanceEntryFilterOptions"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PerformanceEntryFilterOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceEntryFilterOptions`*"]
    pub type PerformanceEntryFilterOptions;
    #[doc = "Get the `entryType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceEntryFilterOptions`*"]
    #[wasm_bindgen(method, getter = "entryType")]
    pub fn get_entry_type(this: &PerformanceEntryFilterOptions) -> Option<::alloc::string::String>;
    #[doc = "Change the `entryType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceEntryFilterOptions`*"]
    #[wasm_bindgen(method, setter = "entryType")]
    pub fn set_entry_type(this: &PerformanceEntryFilterOptions, val: &str);
    #[doc = "Get the `initiatorType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceEntryFilterOptions`*"]
    #[wasm_bindgen(method, getter = "initiatorType")]
    pub fn get_initiator_type(
        this: &PerformanceEntryFilterOptions,
    ) -> Option<::alloc::string::String>;
    #[doc = "Change the `initiatorType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceEntryFilterOptions`*"]
    #[wasm_bindgen(method, setter = "initiatorType")]
    pub fn set_initiator_type(this: &PerformanceEntryFilterOptions, val: &str);
    #[doc = "Get the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceEntryFilterOptions`*"]
    #[wasm_bindgen(method, getter = "name")]
    pub fn get_name(this: &PerformanceEntryFilterOptions) -> Option<::alloc::string::String>;
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceEntryFilterOptions`*"]
    #[wasm_bindgen(method, setter = "name")]
    pub fn set_name(this: &PerformanceEntryFilterOptions, val: &str);
}
impl PerformanceEntryFilterOptions {
    #[doc = "Construct a new `PerformanceEntryFilterOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceEntryFilterOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_entry_type()` instead."]
    pub fn entry_type(&mut self, val: &str) -> &mut Self {
        self.set_entry_type(val);
        self
    }
    #[deprecated = "Use `set_initiator_type()` instead."]
    pub fn initiator_type(&mut self, val: &str) -> &mut Self {
        self.set_initiator_type(val);
        self
    }
    #[deprecated = "Use `set_name()` instead."]
    pub fn name(&mut self, val: &str) -> &mut Self {
        self.set_name(val);
        self
    }
}
impl Default for PerformanceEntryFilterOptions {
    fn default() -> Self {
        Self::new()
    }
}
