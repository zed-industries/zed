#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "CookieInit")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CookieInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CookieInit`*"]
    pub type CookieInit;
    #[doc = "Get the `domain` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CookieInit`*"]
    #[wasm_bindgen(method, getter = "domain")]
    pub fn get_domain(this: &CookieInit) -> Option<::alloc::string::String>;
    #[doc = "Change the `domain` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CookieInit`*"]
    #[wasm_bindgen(method, setter = "domain")]
    pub fn set_domain(this: &CookieInit, val: Option<&str>);
    #[doc = "Get the `expires` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CookieInit`*"]
    #[wasm_bindgen(method, getter = "expires")]
    pub fn get_expires(this: &CookieInit) -> Option<f64>;
    #[doc = "Change the `expires` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CookieInit`*"]
    #[wasm_bindgen(method, setter = "expires")]
    pub fn set_expires(this: &CookieInit, val: Option<f64>);
    #[doc = "Get the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CookieInit`*"]
    #[wasm_bindgen(method, getter = "name")]
    pub fn get_name(this: &CookieInit) -> ::alloc::string::String;
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CookieInit`*"]
    #[wasm_bindgen(method, setter = "name")]
    pub fn set_name(this: &CookieInit, val: &str);
    #[doc = "Get the `partitioned` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CookieInit`*"]
    #[wasm_bindgen(method, getter = "partitioned")]
    pub fn get_partitioned(this: &CookieInit) -> Option<bool>;
    #[doc = "Change the `partitioned` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CookieInit`*"]
    #[wasm_bindgen(method, setter = "partitioned")]
    pub fn set_partitioned(this: &CookieInit, val: bool);
    #[doc = "Get the `path` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CookieInit`*"]
    #[wasm_bindgen(method, getter = "path")]
    pub fn get_path(this: &CookieInit) -> Option<::alloc::string::String>;
    #[doc = "Change the `path` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CookieInit`*"]
    #[wasm_bindgen(method, setter = "path")]
    pub fn set_path(this: &CookieInit, val: &str);
    #[cfg(feature = "CookieSameSite")]
    #[doc = "Get the `sameSite` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CookieInit`, `CookieSameSite`*"]
    #[wasm_bindgen(method, getter = "sameSite")]
    pub fn get_same_site(this: &CookieInit) -> Option<CookieSameSite>;
    #[cfg(feature = "CookieSameSite")]
    #[doc = "Change the `sameSite` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CookieInit`, `CookieSameSite`*"]
    #[wasm_bindgen(method, setter = "sameSite")]
    pub fn set_same_site(this: &CookieInit, val: CookieSameSite);
    #[doc = "Get the `value` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CookieInit`*"]
    #[wasm_bindgen(method, getter = "value")]
    pub fn get_value(this: &CookieInit) -> ::alloc::string::String;
    #[doc = "Change the `value` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CookieInit`*"]
    #[wasm_bindgen(method, setter = "value")]
    pub fn set_value(this: &CookieInit, val: &str);
}
impl CookieInit {
    #[doc = "Construct a new `CookieInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CookieInit`*"]
    pub fn new(name: &str, value: &str) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_name(name);
        ret.set_value(value);
        ret
    }
    #[deprecated = "Use `set_domain()` instead."]
    pub fn domain(&mut self, val: Option<&str>) -> &mut Self {
        self.set_domain(val);
        self
    }
    #[deprecated = "Use `set_expires()` instead."]
    pub fn expires(&mut self, val: Option<f64>) -> &mut Self {
        self.set_expires(val);
        self
    }
    #[deprecated = "Use `set_name()` instead."]
    pub fn name(&mut self, val: &str) -> &mut Self {
        self.set_name(val);
        self
    }
    #[deprecated = "Use `set_partitioned()` instead."]
    pub fn partitioned(&mut self, val: bool) -> &mut Self {
        self.set_partitioned(val);
        self
    }
    #[deprecated = "Use `set_path()` instead."]
    pub fn path(&mut self, val: &str) -> &mut Self {
        self.set_path(val);
        self
    }
    #[cfg(feature = "CookieSameSite")]
    #[deprecated = "Use `set_same_site()` instead."]
    pub fn same_site(&mut self, val: CookieSameSite) -> &mut Self {
        self.set_same_site(val);
        self
    }
    #[deprecated = "Use `set_value()` instead."]
    pub fn value(&mut self, val: &str) -> &mut Self {
        self.set_value(val);
        self
    }
}
