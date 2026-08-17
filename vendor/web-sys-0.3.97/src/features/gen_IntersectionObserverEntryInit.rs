#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "IntersectionObserverEntryInit"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `IntersectionObserverEntryInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IntersectionObserverEntryInit`*"]
    pub type IntersectionObserverEntryInit;
    #[cfg(feature = "DomRectInit")]
    #[doc = "Get the `boundingClientRect` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRectInit`, `IntersectionObserverEntryInit`*"]
    #[wasm_bindgen(method, getter = "boundingClientRect")]
    pub fn get_bounding_client_rect(this: &IntersectionObserverEntryInit) -> DomRectInit;
    #[cfg(feature = "DomRectInit")]
    #[doc = "Change the `boundingClientRect` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRectInit`, `IntersectionObserverEntryInit`*"]
    #[wasm_bindgen(method, setter = "boundingClientRect")]
    pub fn set_bounding_client_rect(this: &IntersectionObserverEntryInit, val: &DomRectInit);
    #[cfg(feature = "DomRectInit")]
    #[doc = "Get the `intersectionRect` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRectInit`, `IntersectionObserverEntryInit`*"]
    #[wasm_bindgen(method, getter = "intersectionRect")]
    pub fn get_intersection_rect(this: &IntersectionObserverEntryInit) -> DomRectInit;
    #[cfg(feature = "DomRectInit")]
    #[doc = "Change the `intersectionRect` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRectInit`, `IntersectionObserverEntryInit`*"]
    #[wasm_bindgen(method, setter = "intersectionRect")]
    pub fn set_intersection_rect(this: &IntersectionObserverEntryInit, val: &DomRectInit);
    #[cfg(feature = "DomRectInit")]
    #[doc = "Get the `rootBounds` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRectInit`, `IntersectionObserverEntryInit`*"]
    #[wasm_bindgen(method, getter = "rootBounds")]
    pub fn get_root_bounds(this: &IntersectionObserverEntryInit) -> DomRectInit;
    #[cfg(feature = "DomRectInit")]
    #[doc = "Change the `rootBounds` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRectInit`, `IntersectionObserverEntryInit`*"]
    #[wasm_bindgen(method, setter = "rootBounds")]
    pub fn set_root_bounds(this: &IntersectionObserverEntryInit, val: &DomRectInit);
    #[cfg(feature = "Element")]
    #[doc = "Get the `target` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`, `IntersectionObserverEntryInit`*"]
    #[wasm_bindgen(method, getter = "target")]
    pub fn get_target(this: &IntersectionObserverEntryInit) -> Element;
    #[cfg(feature = "Element")]
    #[doc = "Change the `target` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`, `IntersectionObserverEntryInit`*"]
    #[wasm_bindgen(method, setter = "target")]
    pub fn set_target(this: &IntersectionObserverEntryInit, val: &Element);
    #[doc = "Get the `time` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IntersectionObserverEntryInit`*"]
    #[wasm_bindgen(method, getter = "time")]
    pub fn get_time(this: &IntersectionObserverEntryInit) -> f64;
    #[doc = "Change the `time` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IntersectionObserverEntryInit`*"]
    #[wasm_bindgen(method, setter = "time")]
    pub fn set_time(this: &IntersectionObserverEntryInit, val: f64);
}
impl IntersectionObserverEntryInit {
    #[cfg(all(feature = "DomRectInit", feature = "Element",))]
    #[doc = "Construct a new `IntersectionObserverEntryInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRectInit`, `Element`, `IntersectionObserverEntryInit`*"]
    pub fn new(
        bounding_client_rect: &DomRectInit,
        intersection_rect: &DomRectInit,
        root_bounds: &DomRectInit,
        target: &Element,
        time: f64,
    ) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_bounding_client_rect(bounding_client_rect);
        ret.set_intersection_rect(intersection_rect);
        ret.set_root_bounds(root_bounds);
        ret.set_target(target);
        ret.set_time(time);
        ret
    }
    #[cfg(feature = "DomRectInit")]
    #[deprecated = "Use `set_bounding_client_rect()` instead."]
    pub fn bounding_client_rect(&mut self, val: &DomRectInit) -> &mut Self {
        self.set_bounding_client_rect(val);
        self
    }
    #[cfg(feature = "DomRectInit")]
    #[deprecated = "Use `set_intersection_rect()` instead."]
    pub fn intersection_rect(&mut self, val: &DomRectInit) -> &mut Self {
        self.set_intersection_rect(val);
        self
    }
    #[cfg(feature = "DomRectInit")]
    #[deprecated = "Use `set_root_bounds()` instead."]
    pub fn root_bounds(&mut self, val: &DomRectInit) -> &mut Self {
        self.set_root_bounds(val);
        self
    }
    #[cfg(feature = "Element")]
    #[deprecated = "Use `set_target()` instead."]
    pub fn target(&mut self, val: &Element) -> &mut Self {
        self.set_target(val);
        self
    }
    #[deprecated = "Use `set_time()` instead."]
    pub fn time(&mut self, val: f64) -> &mut Self {
        self.set_time(val);
        self
    }
}
