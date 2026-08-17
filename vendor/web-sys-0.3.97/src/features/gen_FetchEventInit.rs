#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "FetchEventInit")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `FetchEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FetchEventInit`*"]
    pub type FetchEventInit;
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FetchEventInit`*"]
    #[wasm_bindgen(method, getter = "bubbles")]
    pub fn get_bubbles(this: &FetchEventInit) -> Option<bool>;
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FetchEventInit`*"]
    #[wasm_bindgen(method, setter = "bubbles")]
    pub fn set_bubbles(this: &FetchEventInit, val: bool);
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FetchEventInit`*"]
    #[wasm_bindgen(method, getter = "cancelable")]
    pub fn get_cancelable(this: &FetchEventInit) -> Option<bool>;
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FetchEventInit`*"]
    #[wasm_bindgen(method, setter = "cancelable")]
    pub fn set_cancelable(this: &FetchEventInit, val: bool);
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FetchEventInit`*"]
    #[wasm_bindgen(method, getter = "composed")]
    pub fn get_composed(this: &FetchEventInit) -> Option<bool>;
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FetchEventInit`*"]
    #[wasm_bindgen(method, setter = "composed")]
    pub fn set_composed(this: &FetchEventInit, val: bool);
    #[doc = "Get the `clientId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FetchEventInit`*"]
    #[wasm_bindgen(method, getter = "clientId")]
    pub fn get_client_id(this: &FetchEventInit) -> Option<::alloc::string::String>;
    #[doc = "Change the `clientId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FetchEventInit`*"]
    #[wasm_bindgen(method, setter = "clientId")]
    pub fn set_client_id(this: &FetchEventInit, val: Option<&str>);
    #[doc = "Get the `isReload` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FetchEventInit`*"]
    #[wasm_bindgen(method, getter = "isReload")]
    pub fn get_is_reload(this: &FetchEventInit) -> Option<bool>;
    #[doc = "Change the `isReload` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FetchEventInit`*"]
    #[wasm_bindgen(method, setter = "isReload")]
    pub fn set_is_reload(this: &FetchEventInit, val: bool);
    #[cfg(feature = "Request")]
    #[doc = "Get the `request` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FetchEventInit`, `Request`*"]
    #[wasm_bindgen(method, getter = "request")]
    pub fn get_request(this: &FetchEventInit) -> Request;
    #[cfg(feature = "Request")]
    #[doc = "Change the `request` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FetchEventInit`, `Request`*"]
    #[wasm_bindgen(method, setter = "request")]
    pub fn set_request(this: &FetchEventInit, val: &Request);
}
impl FetchEventInit {
    #[cfg(feature = "Request")]
    #[doc = "Construct a new `FetchEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FetchEventInit`, `Request`*"]
    pub fn new(request: &Request) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_request(request);
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
    #[deprecated = "Use `set_client_id()` instead."]
    pub fn client_id(&mut self, val: Option<&str>) -> &mut Self {
        self.set_client_id(val);
        self
    }
    #[deprecated = "Use `set_is_reload()` instead."]
    pub fn is_reload(&mut self, val: bool) -> &mut Self {
        self.set_is_reload(val);
        self
    }
    #[cfg(feature = "Request")]
    #[deprecated = "Use `set_request()` instead."]
    pub fn request(&mut self, val: &Request) -> &mut Self {
        self.set_request(val);
        self
    }
}
