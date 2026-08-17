#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "HitRegionOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HitRegionOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HitRegionOptions`*"]
    pub type HitRegionOptions;
    #[cfg(feature = "Element")]
    #[doc = "Get the `control` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`, `HitRegionOptions`*"]
    #[wasm_bindgen(method, getter = "control")]
    pub fn get_control(this: &HitRegionOptions) -> Option<Element>;
    #[cfg(feature = "Element")]
    #[doc = "Change the `control` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`, `HitRegionOptions`*"]
    #[wasm_bindgen(method, setter = "control")]
    pub fn set_control(this: &HitRegionOptions, val: Option<&Element>);
    #[doc = "Get the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HitRegionOptions`*"]
    #[wasm_bindgen(method, getter = "id")]
    pub fn get_id(this: &HitRegionOptions) -> Option<::alloc::string::String>;
    #[doc = "Change the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HitRegionOptions`*"]
    #[wasm_bindgen(method, setter = "id")]
    pub fn set_id(this: &HitRegionOptions, val: &str);
    #[cfg(feature = "Path2d")]
    #[doc = "Get the `path` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HitRegionOptions`, `Path2d`*"]
    #[wasm_bindgen(method, getter = "path")]
    pub fn get_path(this: &HitRegionOptions) -> Option<Path2d>;
    #[cfg(feature = "Path2d")]
    #[doc = "Change the `path` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HitRegionOptions`, `Path2d`*"]
    #[wasm_bindgen(method, setter = "path")]
    pub fn set_path(this: &HitRegionOptions, val: Option<&Path2d>);
}
impl HitRegionOptions {
    #[doc = "Construct a new `HitRegionOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HitRegionOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(feature = "Element")]
    #[deprecated = "Use `set_control()` instead."]
    pub fn control(&mut self, val: Option<&Element>) -> &mut Self {
        self.set_control(val);
        self
    }
    #[deprecated = "Use `set_id()` instead."]
    pub fn id(&mut self, val: &str) -> &mut Self {
        self.set_id(val);
        self
    }
    #[cfg(feature = "Path2d")]
    #[deprecated = "Use `set_path()` instead."]
    pub fn path(&mut self, val: Option<&Path2d>) -> &mut Self {
        self.set_path(val);
        self
    }
}
impl Default for HitRegionOptions {
    fn default() -> Self {
        Self::new()
    }
}
