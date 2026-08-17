#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "OpenWindowEventDetail")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `OpenWindowEventDetail` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OpenWindowEventDetail`*"]
    pub type OpenWindowEventDetail;
    #[doc = "Get the `features` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OpenWindowEventDetail`*"]
    #[wasm_bindgen(method, getter = "features")]
    pub fn get_features(this: &OpenWindowEventDetail) -> Option<::alloc::string::String>;
    #[doc = "Change the `features` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OpenWindowEventDetail`*"]
    #[wasm_bindgen(method, setter = "features")]
    pub fn set_features(this: &OpenWindowEventDetail, val: &str);
    #[cfg(feature = "Node")]
    #[doc = "Get the `frameElement` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`, `OpenWindowEventDetail`*"]
    #[wasm_bindgen(method, getter = "frameElement")]
    pub fn get_frame_element(this: &OpenWindowEventDetail) -> Option<Node>;
    #[cfg(feature = "Node")]
    #[doc = "Change the `frameElement` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`, `OpenWindowEventDetail`*"]
    #[wasm_bindgen(method, setter = "frameElement")]
    pub fn set_frame_element(this: &OpenWindowEventDetail, val: Option<&Node>);
    #[doc = "Get the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OpenWindowEventDetail`*"]
    #[wasm_bindgen(method, getter = "name")]
    pub fn get_name(this: &OpenWindowEventDetail) -> Option<::alloc::string::String>;
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OpenWindowEventDetail`*"]
    #[wasm_bindgen(method, setter = "name")]
    pub fn set_name(this: &OpenWindowEventDetail, val: &str);
    #[doc = "Get the `url` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OpenWindowEventDetail`*"]
    #[wasm_bindgen(method, getter = "url")]
    pub fn get_url(this: &OpenWindowEventDetail) -> Option<::alloc::string::String>;
    #[doc = "Change the `url` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OpenWindowEventDetail`*"]
    #[wasm_bindgen(method, setter = "url")]
    pub fn set_url(this: &OpenWindowEventDetail, val: &str);
}
impl OpenWindowEventDetail {
    #[doc = "Construct a new `OpenWindowEventDetail`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OpenWindowEventDetail`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_features()` instead."]
    pub fn features(&mut self, val: &str) -> &mut Self {
        self.set_features(val);
        self
    }
    #[cfg(feature = "Node")]
    #[deprecated = "Use `set_frame_element()` instead."]
    pub fn frame_element(&mut self, val: Option<&Node>) -> &mut Self {
        self.set_frame_element(val);
        self
    }
    #[deprecated = "Use `set_name()` instead."]
    pub fn name(&mut self, val: &str) -> &mut Self {
        self.set_name(val);
        self
    }
    #[deprecated = "Use `set_url()` instead."]
    pub fn url(&mut self, val: &str) -> &mut Self {
        self.set_url(val);
        self
    }
}
impl Default for OpenWindowEventDetail {
    fn default() -> Self {
        Self::new()
    }
}
