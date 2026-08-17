#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "PageTransitionEventInit")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PageTransitionEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PageTransitionEventInit`*"]
    pub type PageTransitionEventInit;
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PageTransitionEventInit`*"]
    #[wasm_bindgen(method, getter = "bubbles")]
    pub fn get_bubbles(this: &PageTransitionEventInit) -> Option<bool>;
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PageTransitionEventInit`*"]
    #[wasm_bindgen(method, setter = "bubbles")]
    pub fn set_bubbles(this: &PageTransitionEventInit, val: bool);
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PageTransitionEventInit`*"]
    #[wasm_bindgen(method, getter = "cancelable")]
    pub fn get_cancelable(this: &PageTransitionEventInit) -> Option<bool>;
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PageTransitionEventInit`*"]
    #[wasm_bindgen(method, setter = "cancelable")]
    pub fn set_cancelable(this: &PageTransitionEventInit, val: bool);
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PageTransitionEventInit`*"]
    #[wasm_bindgen(method, getter = "composed")]
    pub fn get_composed(this: &PageTransitionEventInit) -> Option<bool>;
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PageTransitionEventInit`*"]
    #[wasm_bindgen(method, setter = "composed")]
    pub fn set_composed(this: &PageTransitionEventInit, val: bool);
    #[doc = "Get the `inFrameSwap` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PageTransitionEventInit`*"]
    #[wasm_bindgen(method, getter = "inFrameSwap")]
    pub fn get_in_frame_swap(this: &PageTransitionEventInit) -> Option<bool>;
    #[doc = "Change the `inFrameSwap` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PageTransitionEventInit`*"]
    #[wasm_bindgen(method, setter = "inFrameSwap")]
    pub fn set_in_frame_swap(this: &PageTransitionEventInit, val: bool);
    #[doc = "Get the `persisted` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PageTransitionEventInit`*"]
    #[wasm_bindgen(method, getter = "persisted")]
    pub fn get_persisted(this: &PageTransitionEventInit) -> Option<bool>;
    #[doc = "Change the `persisted` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PageTransitionEventInit`*"]
    #[wasm_bindgen(method, setter = "persisted")]
    pub fn set_persisted(this: &PageTransitionEventInit, val: bool);
}
impl PageTransitionEventInit {
    #[doc = "Construct a new `PageTransitionEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PageTransitionEventInit`*"]
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
    #[deprecated = "Use `set_in_frame_swap()` instead."]
    pub fn in_frame_swap(&mut self, val: bool) -> &mut Self {
        self.set_in_frame_swap(val);
        self
    }
    #[deprecated = "Use `set_persisted()` instead."]
    pub fn persisted(&mut self, val: bool) -> &mut Self {
        self.set_persisted(val);
        self
    }
}
impl Default for PageTransitionEventInit {
    fn default() -> Self {
        Self::new()
    }
}
