#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "MessageEventInit")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MessageEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessageEventInit`*"]
    pub type MessageEventInit;
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessageEventInit`*"]
    #[wasm_bindgen(method, getter = "bubbles")]
    pub fn get_bubbles(this: &MessageEventInit) -> Option<bool>;
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessageEventInit`*"]
    #[wasm_bindgen(method, setter = "bubbles")]
    pub fn set_bubbles(this: &MessageEventInit, val: bool);
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessageEventInit`*"]
    #[wasm_bindgen(method, getter = "cancelable")]
    pub fn get_cancelable(this: &MessageEventInit) -> Option<bool>;
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessageEventInit`*"]
    #[wasm_bindgen(method, setter = "cancelable")]
    pub fn set_cancelable(this: &MessageEventInit, val: bool);
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessageEventInit`*"]
    #[wasm_bindgen(method, getter = "composed")]
    pub fn get_composed(this: &MessageEventInit) -> Option<bool>;
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessageEventInit`*"]
    #[wasm_bindgen(method, setter = "composed")]
    pub fn set_composed(this: &MessageEventInit, val: bool);
    #[doc = "Get the `data` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessageEventInit`*"]
    #[wasm_bindgen(method, getter = "data")]
    pub fn get_data(this: &MessageEventInit) -> ::wasm_bindgen::JsValue;
    #[doc = "Change the `data` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessageEventInit`*"]
    #[wasm_bindgen(method, setter = "data")]
    pub fn set_data(this: &MessageEventInit, val: &::wasm_bindgen::JsValue);
    #[doc = "Get the `lastEventId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessageEventInit`*"]
    #[wasm_bindgen(method, getter = "lastEventId")]
    pub fn get_last_event_id(this: &MessageEventInit) -> Option<::alloc::string::String>;
    #[doc = "Change the `lastEventId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessageEventInit`*"]
    #[wasm_bindgen(method, setter = "lastEventId")]
    pub fn set_last_event_id(this: &MessageEventInit, val: &str);
    #[doc = "Get the `origin` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessageEventInit`*"]
    #[wasm_bindgen(method, getter = "origin")]
    pub fn get_origin(this: &MessageEventInit) -> Option<::alloc::string::String>;
    #[doc = "Change the `origin` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessageEventInit`*"]
    #[wasm_bindgen(method, setter = "origin")]
    pub fn set_origin(this: &MessageEventInit, val: &str);
    #[doc = "Get the `ports` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessageEventInit`*"]
    #[wasm_bindgen(method, getter = "ports")]
    pub fn get_ports(this: &MessageEventInit) -> Option<::js_sys::Array>;
    #[doc = "Change the `ports` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessageEventInit`*"]
    #[wasm_bindgen(method, setter = "ports")]
    pub fn set_ports(this: &MessageEventInit, val: &::wasm_bindgen::JsValue);
    #[doc = "Get the `source` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessageEventInit`*"]
    #[wasm_bindgen(method, getter = "source")]
    pub fn get_source(this: &MessageEventInit) -> Option<::js_sys::Object>;
    #[doc = "Change the `source` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessageEventInit`*"]
    #[wasm_bindgen(method, setter = "source")]
    pub fn set_source(this: &MessageEventInit, val: Option<&::js_sys::Object>);
    #[cfg(feature = "Window")]
    #[doc = "Change the `source` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessageEventInit`, `Window`*"]
    #[wasm_bindgen(method, setter = "source")]
    pub fn set_source_opt_window(this: &MessageEventInit, val: Option<&Window>);
    #[cfg(feature = "MessagePort")]
    #[doc = "Change the `source` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessageEventInit`, `MessagePort`*"]
    #[wasm_bindgen(method, setter = "source")]
    pub fn set_source_opt_message_port(this: &MessageEventInit, val: Option<&MessagePort>);
    #[cfg(feature = "ServiceWorker")]
    #[doc = "Change the `source` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessageEventInit`, `ServiceWorker`*"]
    #[wasm_bindgen(method, setter = "source")]
    pub fn set_source_opt_service_worker(this: &MessageEventInit, val: Option<&ServiceWorker>);
}
impl MessageEventInit {
    #[doc = "Construct a new `MessageEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessageEventInit`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_bubbles()` instead."]
    pub fn bubbles(&mut self, val: bool) -> &mut Self {
        self.set_bubbles(val);
        self
    }
    #[deprecated = "Use `set_cancelable()` instead."]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.set_cancelable(val);
        self
    }
    #[deprecated = "Use `set_composed()` instead."]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.set_composed(val);
        self
    }
    #[deprecated = "Use `set_data()` instead."]
    pub fn data(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_data(val);
        self
    }
    #[deprecated = "Use `set_last_event_id()` instead."]
    pub fn last_event_id(&mut self, val: &str) -> &mut Self {
        self.set_last_event_id(val);
        self
    }
    #[deprecated = "Use `set_origin()` instead."]
    pub fn origin(&mut self, val: &str) -> &mut Self {
        self.set_origin(val);
        self
    }
    #[deprecated = "Use `set_ports()` instead."]
    pub fn ports(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_ports(val);
        self
    }
    #[deprecated = "Use `set_source()` instead."]
    pub fn source(&mut self, val: Option<&::js_sys::Object>) -> &mut Self {
        self.set_source(val);
        self
    }
}
impl Default for MessageEventInit {
    fn default() -> Self {
        Self::new()
    }
}
