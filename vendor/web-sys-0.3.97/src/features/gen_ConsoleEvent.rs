#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "ConsoleEvent")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ConsoleEvent` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleEvent`*"]
    pub type ConsoleEvent;
    #[doc = "Get the `ID` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleEvent`*"]
    #[wasm_bindgen(method, getter = "ID")]
    pub fn get_id(this: &ConsoleEvent) -> ::wasm_bindgen::JsValue;
    #[doc = "Change the `ID` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleEvent`*"]
    #[wasm_bindgen(method, setter = "ID")]
    pub fn set_id(this: &ConsoleEvent, val: &::wasm_bindgen::JsValue);
    #[doc = "Change the `ID` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleEvent`*"]
    #[wasm_bindgen(method, setter = "ID")]
    pub fn set_id_u32(this: &ConsoleEvent, val: u32);
    #[doc = "Change the `ID` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleEvent`*"]
    #[wasm_bindgen(method, setter = "ID")]
    pub fn set_id_f64(this: &ConsoleEvent, val: f64);
    #[doc = "Change the `ID` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleEvent`*"]
    #[wasm_bindgen(method, setter = "ID")]
    pub fn set_id_str(this: &ConsoleEvent, val: &str);
    #[doc = "Get the `addonId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleEvent`*"]
    #[wasm_bindgen(method, getter = "addonId")]
    pub fn get_addon_id(this: &ConsoleEvent) -> Option<::alloc::string::String>;
    #[doc = "Change the `addonId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleEvent`*"]
    #[wasm_bindgen(method, setter = "addonId")]
    pub fn set_addon_id(this: &ConsoleEvent, val: &str);
    #[doc = "Get the `arguments` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleEvent`*"]
    #[wasm_bindgen(method, getter = "arguments")]
    pub fn get_arguments(this: &ConsoleEvent) -> Option<::js_sys::Array>;
    #[doc = "Change the `arguments` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleEvent`*"]
    #[wasm_bindgen(method, setter = "arguments")]
    pub fn set_arguments(this: &ConsoleEvent, val: &::wasm_bindgen::JsValue);
    #[doc = "Get the `columnNumber` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleEvent`*"]
    #[wasm_bindgen(method, getter = "columnNumber")]
    pub fn get_column_number(this: &ConsoleEvent) -> Option<u32>;
    #[doc = "Change the `columnNumber` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleEvent`*"]
    #[wasm_bindgen(method, setter = "columnNumber")]
    pub fn set_column_number(this: &ConsoleEvent, val: u32);
    #[doc = "Get the `consoleID` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleEvent`*"]
    #[wasm_bindgen(method, getter = "consoleID")]
    pub fn get_console_id(this: &ConsoleEvent) -> Option<::alloc::string::String>;
    #[doc = "Change the `consoleID` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleEvent`*"]
    #[wasm_bindgen(method, setter = "consoleID")]
    pub fn set_console_id(this: &ConsoleEvent, val: &str);
    #[doc = "Get the `counter` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleEvent`*"]
    #[wasm_bindgen(method, getter = "counter")]
    pub fn get_counter(this: &ConsoleEvent) -> ::wasm_bindgen::JsValue;
    #[doc = "Change the `counter` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleEvent`*"]
    #[wasm_bindgen(method, setter = "counter")]
    pub fn set_counter(this: &ConsoleEvent, val: &::wasm_bindgen::JsValue);
    #[doc = "Get the `filename` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleEvent`*"]
    #[wasm_bindgen(method, getter = "filename")]
    pub fn get_filename(this: &ConsoleEvent) -> Option<::alloc::string::String>;
    #[doc = "Change the `filename` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleEvent`*"]
    #[wasm_bindgen(method, setter = "filename")]
    pub fn set_filename(this: &ConsoleEvent, val: &str);
    #[doc = "Get the `functionName` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleEvent`*"]
    #[wasm_bindgen(method, getter = "functionName")]
    pub fn get_function_name(this: &ConsoleEvent) -> Option<::alloc::string::String>;
    #[doc = "Change the `functionName` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleEvent`*"]
    #[wasm_bindgen(method, setter = "functionName")]
    pub fn set_function_name(this: &ConsoleEvent, val: &str);
    #[doc = "Get the `groupName` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleEvent`*"]
    #[wasm_bindgen(method, getter = "groupName")]
    pub fn get_group_name(this: &ConsoleEvent) -> Option<::alloc::string::String>;
    #[doc = "Change the `groupName` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleEvent`*"]
    #[wasm_bindgen(method, setter = "groupName")]
    pub fn set_group_name(this: &ConsoleEvent, val: &str);
    #[doc = "Get the `innerID` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleEvent`*"]
    #[wasm_bindgen(method, getter = "innerID")]
    pub fn get_inner_id(this: &ConsoleEvent) -> ::wasm_bindgen::JsValue;
    #[doc = "Change the `innerID` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleEvent`*"]
    #[wasm_bindgen(method, setter = "innerID")]
    pub fn set_inner_id(this: &ConsoleEvent, val: &::wasm_bindgen::JsValue);
    #[doc = "Change the `innerID` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleEvent`*"]
    #[wasm_bindgen(method, setter = "innerID")]
    pub fn set_inner_id_u32(this: &ConsoleEvent, val: u32);
    #[doc = "Change the `innerID` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleEvent`*"]
    #[wasm_bindgen(method, setter = "innerID")]
    pub fn set_inner_id_f64(this: &ConsoleEvent, val: f64);
    #[doc = "Change the `innerID` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleEvent`*"]
    #[wasm_bindgen(method, setter = "innerID")]
    pub fn set_inner_id_str(this: &ConsoleEvent, val: &str);
    #[doc = "Get the `level` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleEvent`*"]
    #[wasm_bindgen(method, getter = "level")]
    pub fn get_level(this: &ConsoleEvent) -> Option<::alloc::string::String>;
    #[doc = "Change the `level` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleEvent`*"]
    #[wasm_bindgen(method, setter = "level")]
    pub fn set_level(this: &ConsoleEvent, val: &str);
    #[doc = "Get the `lineNumber` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleEvent`*"]
    #[wasm_bindgen(method, getter = "lineNumber")]
    pub fn get_line_number(this: &ConsoleEvent) -> Option<u32>;
    #[doc = "Change the `lineNumber` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleEvent`*"]
    #[wasm_bindgen(method, setter = "lineNumber")]
    pub fn set_line_number(this: &ConsoleEvent, val: u32);
    #[doc = "Get the `prefix` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleEvent`*"]
    #[wasm_bindgen(method, getter = "prefix")]
    pub fn get_prefix(this: &ConsoleEvent) -> Option<::alloc::string::String>;
    #[doc = "Change the `prefix` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleEvent`*"]
    #[wasm_bindgen(method, setter = "prefix")]
    pub fn set_prefix(this: &ConsoleEvent, val: &str);
    #[doc = "Get the `private` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleEvent`*"]
    #[wasm_bindgen(method, getter = "private")]
    pub fn get_private(this: &ConsoleEvent) -> Option<bool>;
    #[doc = "Change the `private` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleEvent`*"]
    #[wasm_bindgen(method, setter = "private")]
    pub fn set_private(this: &ConsoleEvent, val: bool);
    #[doc = "Get the `styles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleEvent`*"]
    #[wasm_bindgen(method, getter = "styles")]
    pub fn get_styles(this: &ConsoleEvent) -> Option<::js_sys::Array>;
    #[doc = "Change the `styles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleEvent`*"]
    #[wasm_bindgen(method, setter = "styles")]
    pub fn set_styles(this: &ConsoleEvent, val: &::wasm_bindgen::JsValue);
    #[doc = "Get the `timeStamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleEvent`*"]
    #[wasm_bindgen(method, getter = "timeStamp")]
    pub fn get_time_stamp(this: &ConsoleEvent) -> Option<f64>;
    #[doc = "Change the `timeStamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleEvent`*"]
    #[wasm_bindgen(method, setter = "timeStamp")]
    pub fn set_time_stamp(this: &ConsoleEvent, val: f64);
    #[doc = "Get the `timer` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleEvent`*"]
    #[wasm_bindgen(method, getter = "timer")]
    pub fn get_timer(this: &ConsoleEvent) -> ::wasm_bindgen::JsValue;
    #[doc = "Change the `timer` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleEvent`*"]
    #[wasm_bindgen(method, setter = "timer")]
    pub fn set_timer(this: &ConsoleEvent, val: &::wasm_bindgen::JsValue);
}
impl ConsoleEvent {
    #[doc = "Construct a new `ConsoleEvent`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleEvent`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_id()` instead."]
    pub fn id(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_id(val);
        self
    }
    #[deprecated = "Use `set_addon_id()` instead."]
    pub fn addon_id(&mut self, val: &str) -> &mut Self {
        self.set_addon_id(val);
        self
    }
    #[deprecated = "Use `set_arguments()` instead."]
    pub fn arguments(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_arguments(val);
        self
    }
    #[deprecated = "Use `set_column_number()` instead."]
    pub fn column_number(&mut self, val: u32) -> &mut Self {
        self.set_column_number(val);
        self
    }
    #[deprecated = "Use `set_console_id()` instead."]
    pub fn console_id(&mut self, val: &str) -> &mut Self {
        self.set_console_id(val);
        self
    }
    #[deprecated = "Use `set_counter()` instead."]
    pub fn counter(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_counter(val);
        self
    }
    #[deprecated = "Use `set_filename()` instead."]
    pub fn filename(&mut self, val: &str) -> &mut Self {
        self.set_filename(val);
        self
    }
    #[deprecated = "Use `set_function_name()` instead."]
    pub fn function_name(&mut self, val: &str) -> &mut Self {
        self.set_function_name(val);
        self
    }
    #[deprecated = "Use `set_group_name()` instead."]
    pub fn group_name(&mut self, val: &str) -> &mut Self {
        self.set_group_name(val);
        self
    }
    #[deprecated = "Use `set_inner_id()` instead."]
    pub fn inner_id(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_inner_id(val);
        self
    }
    #[deprecated = "Use `set_level()` instead."]
    pub fn level(&mut self, val: &str) -> &mut Self {
        self.set_level(val);
        self
    }
    #[deprecated = "Use `set_line_number()` instead."]
    pub fn line_number(&mut self, val: u32) -> &mut Self {
        self.set_line_number(val);
        self
    }
    #[deprecated = "Use `set_prefix()` instead."]
    pub fn prefix(&mut self, val: &str) -> &mut Self {
        self.set_prefix(val);
        self
    }
    #[deprecated = "Use `set_private()` instead."]
    pub fn private(&mut self, val: bool) -> &mut Self {
        self.set_private(val);
        self
    }
    #[deprecated = "Use `set_styles()` instead."]
    pub fn styles(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_styles(val);
        self
    }
    #[deprecated = "Use `set_time_stamp()` instead."]
    pub fn time_stamp(&mut self, val: f64) -> &mut Self {
        self.set_time_stamp(val);
        self
    }
    #[deprecated = "Use `set_timer()` instead."]
    pub fn timer(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_timer(val);
        self
    }
}
impl Default for ConsoleEvent {
    fn default() -> Self {
        Self::new()
    }
}
