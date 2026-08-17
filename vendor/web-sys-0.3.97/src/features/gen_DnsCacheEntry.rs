#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "DnsCacheEntry")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DnsCacheEntry` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DnsCacheEntry`*"]
    pub type DnsCacheEntry;
    #[doc = "Get the `expiration` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DnsCacheEntry`*"]
    #[wasm_bindgen(method, getter = "expiration")]
    pub fn get_expiration(this: &DnsCacheEntry) -> Option<f64>;
    #[doc = "Change the `expiration` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DnsCacheEntry`*"]
    #[wasm_bindgen(method, setter = "expiration")]
    pub fn set_expiration(this: &DnsCacheEntry, val: f64);
    #[doc = "Get the `family` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DnsCacheEntry`*"]
    #[wasm_bindgen(method, getter = "family")]
    pub fn get_family(this: &DnsCacheEntry) -> Option<::alloc::string::String>;
    #[doc = "Change the `family` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DnsCacheEntry`*"]
    #[wasm_bindgen(method, setter = "family")]
    pub fn set_family(this: &DnsCacheEntry, val: &str);
    #[doc = "Get the `hostaddr` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DnsCacheEntry`*"]
    #[wasm_bindgen(method, getter = "hostaddr")]
    pub fn get_hostaddr(this: &DnsCacheEntry) -> Option<::js_sys::Array>;
    #[doc = "Change the `hostaddr` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DnsCacheEntry`*"]
    #[wasm_bindgen(method, setter = "hostaddr")]
    pub fn set_hostaddr(this: &DnsCacheEntry, val: &::wasm_bindgen::JsValue);
    #[doc = "Get the `hostname` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DnsCacheEntry`*"]
    #[wasm_bindgen(method, getter = "hostname")]
    pub fn get_hostname(this: &DnsCacheEntry) -> Option<::alloc::string::String>;
    #[doc = "Change the `hostname` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DnsCacheEntry`*"]
    #[wasm_bindgen(method, setter = "hostname")]
    pub fn set_hostname(this: &DnsCacheEntry, val: &str);
    #[doc = "Get the `trr` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DnsCacheEntry`*"]
    #[wasm_bindgen(method, getter = "trr")]
    pub fn get_trr(this: &DnsCacheEntry) -> Option<bool>;
    #[doc = "Change the `trr` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DnsCacheEntry`*"]
    #[wasm_bindgen(method, setter = "trr")]
    pub fn set_trr(this: &DnsCacheEntry, val: bool);
}
impl DnsCacheEntry {
    #[doc = "Construct a new `DnsCacheEntry`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DnsCacheEntry`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_expiration()` instead."]
    pub fn expiration(&mut self, val: f64) -> &mut Self {
        self.set_expiration(val);
        self
    }
    #[deprecated = "Use `set_family()` instead."]
    pub fn family(&mut self, val: &str) -> &mut Self {
        self.set_family(val);
        self
    }
    #[deprecated = "Use `set_hostaddr()` instead."]
    pub fn hostaddr(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_hostaddr(val);
        self
    }
    #[deprecated = "Use `set_hostname()` instead."]
    pub fn hostname(&mut self, val: &str) -> &mut Self {
        self.set_hostname(val);
        self
    }
    #[deprecated = "Use `set_trr()` instead."]
    pub fn trr(&mut self, val: bool) -> &mut Self {
        self.set_trr(val);
        self
    }
}
impl Default for DnsCacheEntry {
    fn default() -> Self {
        Self::new()
    }
}
