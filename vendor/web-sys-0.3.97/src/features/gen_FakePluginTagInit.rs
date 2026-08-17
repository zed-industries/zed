#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "FakePluginTagInit")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `FakePluginTagInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FakePluginTagInit`*"]
    pub type FakePluginTagInit;
    #[doc = "Get the `description` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FakePluginTagInit`*"]
    #[wasm_bindgen(method, getter = "description")]
    pub fn get_description(this: &FakePluginTagInit) -> Option<::alloc::string::String>;
    #[doc = "Change the `description` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FakePluginTagInit`*"]
    #[wasm_bindgen(method, setter = "description")]
    pub fn set_description(this: &FakePluginTagInit, val: &str);
    #[doc = "Get the `fileName` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FakePluginTagInit`*"]
    #[wasm_bindgen(method, getter = "fileName")]
    pub fn get_file_name(this: &FakePluginTagInit) -> Option<::alloc::string::String>;
    #[doc = "Change the `fileName` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FakePluginTagInit`*"]
    #[wasm_bindgen(method, setter = "fileName")]
    pub fn set_file_name(this: &FakePluginTagInit, val: &str);
    #[doc = "Get the `fullPath` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FakePluginTagInit`*"]
    #[wasm_bindgen(method, getter = "fullPath")]
    pub fn get_full_path(this: &FakePluginTagInit) -> Option<::alloc::string::String>;
    #[doc = "Change the `fullPath` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FakePluginTagInit`*"]
    #[wasm_bindgen(method, setter = "fullPath")]
    pub fn set_full_path(this: &FakePluginTagInit, val: &str);
    #[doc = "Get the `handlerURI` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FakePluginTagInit`*"]
    #[wasm_bindgen(method, getter = "handlerURI")]
    pub fn get_handler_uri(this: &FakePluginTagInit) -> ::alloc::string::String;
    #[doc = "Change the `handlerURI` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FakePluginTagInit`*"]
    #[wasm_bindgen(method, setter = "handlerURI")]
    pub fn set_handler_uri(this: &FakePluginTagInit, val: &str);
    #[doc = "Get the `mimeEntries` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FakePluginTagInit`*"]
    #[wasm_bindgen(method, getter = "mimeEntries")]
    pub fn get_mime_entries(this: &FakePluginTagInit) -> ::js_sys::Array;
    #[doc = "Change the `mimeEntries` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FakePluginTagInit`*"]
    #[wasm_bindgen(method, setter = "mimeEntries")]
    pub fn set_mime_entries(this: &FakePluginTagInit, val: &::wasm_bindgen::JsValue);
    #[doc = "Get the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FakePluginTagInit`*"]
    #[wasm_bindgen(method, getter = "name")]
    pub fn get_name(this: &FakePluginTagInit) -> Option<::alloc::string::String>;
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FakePluginTagInit`*"]
    #[wasm_bindgen(method, setter = "name")]
    pub fn set_name(this: &FakePluginTagInit, val: &str);
    #[doc = "Get the `niceName` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FakePluginTagInit`*"]
    #[wasm_bindgen(method, getter = "niceName")]
    pub fn get_nice_name(this: &FakePluginTagInit) -> Option<::alloc::string::String>;
    #[doc = "Change the `niceName` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FakePluginTagInit`*"]
    #[wasm_bindgen(method, setter = "niceName")]
    pub fn set_nice_name(this: &FakePluginTagInit, val: &str);
    #[doc = "Get the `sandboxScript` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FakePluginTagInit`*"]
    #[wasm_bindgen(method, getter = "sandboxScript")]
    pub fn get_sandbox_script(this: &FakePluginTagInit) -> Option<::alloc::string::String>;
    #[doc = "Change the `sandboxScript` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FakePluginTagInit`*"]
    #[wasm_bindgen(method, setter = "sandboxScript")]
    pub fn set_sandbox_script(this: &FakePluginTagInit, val: &str);
    #[doc = "Get the `version` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FakePluginTagInit`*"]
    #[wasm_bindgen(method, getter = "version")]
    pub fn get_version(this: &FakePluginTagInit) -> Option<::alloc::string::String>;
    #[doc = "Change the `version` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FakePluginTagInit`*"]
    #[wasm_bindgen(method, setter = "version")]
    pub fn set_version(this: &FakePluginTagInit, val: &str);
}
impl FakePluginTagInit {
    #[doc = "Construct a new `FakePluginTagInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FakePluginTagInit`*"]
    pub fn new(handler_uri: &str, mime_entries: &::wasm_bindgen::JsValue) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_handler_uri(handler_uri);
        ret.set_mime_entries(mime_entries);
        ret
    }
    #[deprecated = "Use `set_description()` instead."]
    pub fn description(&mut self, val: &str) -> &mut Self {
        self.set_description(val);
        self
    }
    #[deprecated = "Use `set_file_name()` instead."]
    pub fn file_name(&mut self, val: &str) -> &mut Self {
        self.set_file_name(val);
        self
    }
    #[deprecated = "Use `set_full_path()` instead."]
    pub fn full_path(&mut self, val: &str) -> &mut Self {
        self.set_full_path(val);
        self
    }
    #[deprecated = "Use `set_handler_uri()` instead."]
    pub fn handler_uri(&mut self, val: &str) -> &mut Self {
        self.set_handler_uri(val);
        self
    }
    #[deprecated = "Use `set_mime_entries()` instead."]
    pub fn mime_entries(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_mime_entries(val);
        self
    }
    #[deprecated = "Use `set_name()` instead."]
    pub fn name(&mut self, val: &str) -> &mut Self {
        self.set_name(val);
        self
    }
    #[deprecated = "Use `set_nice_name()` instead."]
    pub fn nice_name(&mut self, val: &str) -> &mut Self {
        self.set_nice_name(val);
        self
    }
    #[deprecated = "Use `set_sandbox_script()` instead."]
    pub fn sandbox_script(&mut self, val: &str) -> &mut Self {
        self.set_sandbox_script(val);
        self
    }
    #[deprecated = "Use `set_version()` instead."]
    pub fn version(&mut self, val: &str) -> &mut Self {
        self.set_version(val);
        self
    }
}
