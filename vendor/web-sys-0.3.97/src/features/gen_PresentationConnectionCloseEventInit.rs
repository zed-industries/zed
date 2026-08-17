#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "PresentationConnectionCloseEventInit"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PresentationConnectionCloseEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnectionCloseEventInit`*"]
    pub type PresentationConnectionCloseEventInit;
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnectionCloseEventInit`*"]
    #[wasm_bindgen(method, getter = "bubbles")]
    pub fn get_bubbles(this: &PresentationConnectionCloseEventInit) -> Option<bool>;
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnectionCloseEventInit`*"]
    #[wasm_bindgen(method, setter = "bubbles")]
    pub fn set_bubbles(this: &PresentationConnectionCloseEventInit, val: bool);
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnectionCloseEventInit`*"]
    #[wasm_bindgen(method, getter = "cancelable")]
    pub fn get_cancelable(this: &PresentationConnectionCloseEventInit) -> Option<bool>;
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnectionCloseEventInit`*"]
    #[wasm_bindgen(method, setter = "cancelable")]
    pub fn set_cancelable(this: &PresentationConnectionCloseEventInit, val: bool);
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnectionCloseEventInit`*"]
    #[wasm_bindgen(method, getter = "composed")]
    pub fn get_composed(this: &PresentationConnectionCloseEventInit) -> Option<bool>;
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnectionCloseEventInit`*"]
    #[wasm_bindgen(method, setter = "composed")]
    pub fn set_composed(this: &PresentationConnectionCloseEventInit, val: bool);
    #[doc = "Get the `message` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnectionCloseEventInit`*"]
    #[wasm_bindgen(method, getter = "message")]
    pub fn get_message(
        this: &PresentationConnectionCloseEventInit,
    ) -> Option<::alloc::string::String>;
    #[doc = "Change the `message` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnectionCloseEventInit`*"]
    #[wasm_bindgen(method, setter = "message")]
    pub fn set_message(this: &PresentationConnectionCloseEventInit, val: &str);
    #[cfg(feature = "PresentationConnectionClosedReason")]
    #[doc = "Get the `reason` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnectionCloseEventInit`, `PresentationConnectionClosedReason`*"]
    #[wasm_bindgen(method, getter = "reason")]
    pub fn get_reason(
        this: &PresentationConnectionCloseEventInit,
    ) -> PresentationConnectionClosedReason;
    #[cfg(feature = "PresentationConnectionClosedReason")]
    #[doc = "Change the `reason` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnectionCloseEventInit`, `PresentationConnectionClosedReason`*"]
    #[wasm_bindgen(method, setter = "reason")]
    pub fn set_reason(
        this: &PresentationConnectionCloseEventInit,
        val: PresentationConnectionClosedReason,
    );
}
impl PresentationConnectionCloseEventInit {
    #[cfg(feature = "PresentationConnectionClosedReason")]
    #[doc = "Construct a new `PresentationConnectionCloseEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnectionCloseEventInit`, `PresentationConnectionClosedReason`*"]
    pub fn new(reason: PresentationConnectionClosedReason) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_reason(reason);
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
    #[deprecated = "Use `set_message()` instead."]
    pub fn message(&mut self, val: &str) -> &mut Self {
        self.set_message(val);
        self
    }
    #[cfg(feature = "PresentationConnectionClosedReason")]
    #[deprecated = "Use `set_reason()` instead."]
    pub fn reason(&mut self, val: PresentationConnectionClosedReason) -> &mut Self {
        self.set_reason(val);
        self
    }
}
