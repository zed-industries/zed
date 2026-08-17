#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "ConsoleInstanceOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ConsoleInstanceOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleInstanceOptions`*"]
    pub type ConsoleInstanceOptions;
    #[doc = "Get the `consoleID` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleInstanceOptions`*"]
    #[wasm_bindgen(method, getter = "consoleID")]
    pub fn get_console_id(this: &ConsoleInstanceOptions) -> Option<::alloc::string::String>;
    #[doc = "Change the `consoleID` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleInstanceOptions`*"]
    #[wasm_bindgen(method, setter = "consoleID")]
    pub fn set_console_id(this: &ConsoleInstanceOptions, val: &str);
    #[doc = "Get the `dump` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleInstanceOptions`*"]
    #[wasm_bindgen(method, getter = "dump")]
    pub fn get_dump(this: &ConsoleInstanceOptions) -> Option<::js_sys::Function>;
    #[doc = "Change the `dump` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleInstanceOptions`*"]
    #[wasm_bindgen(method, setter = "dump")]
    pub fn set_dump(this: &ConsoleInstanceOptions, val: &::js_sys::Function);
    #[doc = "Get the `innerID` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleInstanceOptions`*"]
    #[wasm_bindgen(method, getter = "innerID")]
    pub fn get_inner_id(this: &ConsoleInstanceOptions) -> Option<::alloc::string::String>;
    #[doc = "Change the `innerID` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleInstanceOptions`*"]
    #[wasm_bindgen(method, setter = "innerID")]
    pub fn set_inner_id(this: &ConsoleInstanceOptions, val: &str);
    #[cfg(feature = "ConsoleLogLevel")]
    #[doc = "Get the `maxLogLevel` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleInstanceOptions`, `ConsoleLogLevel`*"]
    #[wasm_bindgen(method, getter = "maxLogLevel")]
    pub fn get_max_log_level(this: &ConsoleInstanceOptions) -> Option<ConsoleLogLevel>;
    #[cfg(feature = "ConsoleLogLevel")]
    #[doc = "Change the `maxLogLevel` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleInstanceOptions`, `ConsoleLogLevel`*"]
    #[wasm_bindgen(method, setter = "maxLogLevel")]
    pub fn set_max_log_level(this: &ConsoleInstanceOptions, val: ConsoleLogLevel);
    #[doc = "Get the `maxLogLevelPref` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleInstanceOptions`*"]
    #[wasm_bindgen(method, getter = "maxLogLevelPref")]
    pub fn get_max_log_level_pref(this: &ConsoleInstanceOptions)
        -> Option<::alloc::string::String>;
    #[doc = "Change the `maxLogLevelPref` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleInstanceOptions`*"]
    #[wasm_bindgen(method, setter = "maxLogLevelPref")]
    pub fn set_max_log_level_pref(this: &ConsoleInstanceOptions, val: &str);
    #[doc = "Get the `prefix` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleInstanceOptions`*"]
    #[wasm_bindgen(method, getter = "prefix")]
    pub fn get_prefix(this: &ConsoleInstanceOptions) -> Option<::alloc::string::String>;
    #[doc = "Change the `prefix` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleInstanceOptions`*"]
    #[wasm_bindgen(method, setter = "prefix")]
    pub fn set_prefix(this: &ConsoleInstanceOptions, val: &str);
}
impl ConsoleInstanceOptions {
    #[doc = "Construct a new `ConsoleInstanceOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleInstanceOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_console_id()` instead."]
    pub fn console_id(&mut self, val: &str) -> &mut Self {
        self.set_console_id(val);
        self
    }
    #[deprecated = "Use `set_dump()` instead."]
    pub fn dump(&mut self, val: &::js_sys::Function) -> &mut Self {
        self.set_dump(val);
        self
    }
    #[deprecated = "Use `set_inner_id()` instead."]
    pub fn inner_id(&mut self, val: &str) -> &mut Self {
        self.set_inner_id(val);
        self
    }
    #[cfg(feature = "ConsoleLogLevel")]
    #[deprecated = "Use `set_max_log_level()` instead."]
    pub fn max_log_level(&mut self, val: ConsoleLogLevel) -> &mut Self {
        self.set_max_log_level(val);
        self
    }
    #[deprecated = "Use `set_max_log_level_pref()` instead."]
    pub fn max_log_level_pref(&mut self, val: &str) -> &mut Self {
        self.set_max_log_level_pref(val);
        self
    }
    #[deprecated = "Use `set_prefix()` instead."]
    pub fn prefix(&mut self, val: &str) -> &mut Self {
        self.set_prefix(val);
        self
    }
}
impl Default for ConsoleInstanceOptions {
    fn default() -> Self {
        Self::new()
    }
}
