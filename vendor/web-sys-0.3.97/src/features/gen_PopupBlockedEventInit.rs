#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "PopupBlockedEventInit")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PopupBlockedEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PopupBlockedEventInit`*"]
    pub type PopupBlockedEventInit;
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PopupBlockedEventInit`*"]
    #[wasm_bindgen(method, getter = "bubbles")]
    pub fn get_bubbles(this: &PopupBlockedEventInit) -> Option<bool>;
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PopupBlockedEventInit`*"]
    #[wasm_bindgen(method, setter = "bubbles")]
    pub fn set_bubbles(this: &PopupBlockedEventInit, val: bool);
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PopupBlockedEventInit`*"]
    #[wasm_bindgen(method, getter = "cancelable")]
    pub fn get_cancelable(this: &PopupBlockedEventInit) -> Option<bool>;
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PopupBlockedEventInit`*"]
    #[wasm_bindgen(method, setter = "cancelable")]
    pub fn set_cancelable(this: &PopupBlockedEventInit, val: bool);
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PopupBlockedEventInit`*"]
    #[wasm_bindgen(method, getter = "composed")]
    pub fn get_composed(this: &PopupBlockedEventInit) -> Option<bool>;
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PopupBlockedEventInit`*"]
    #[wasm_bindgen(method, setter = "composed")]
    pub fn set_composed(this: &PopupBlockedEventInit, val: bool);
    #[doc = "Get the `popupWindowFeatures` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PopupBlockedEventInit`*"]
    #[wasm_bindgen(method, getter = "popupWindowFeatures")]
    pub fn get_popup_window_features(
        this: &PopupBlockedEventInit,
    ) -> Option<::alloc::string::String>;
    #[doc = "Change the `popupWindowFeatures` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PopupBlockedEventInit`*"]
    #[wasm_bindgen(method, setter = "popupWindowFeatures")]
    pub fn set_popup_window_features(this: &PopupBlockedEventInit, val: &str);
    #[doc = "Get the `popupWindowName` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PopupBlockedEventInit`*"]
    #[wasm_bindgen(method, getter = "popupWindowName")]
    pub fn get_popup_window_name(this: &PopupBlockedEventInit) -> Option<::alloc::string::String>;
    #[doc = "Change the `popupWindowName` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PopupBlockedEventInit`*"]
    #[wasm_bindgen(method, setter = "popupWindowName")]
    pub fn set_popup_window_name(this: &PopupBlockedEventInit, val: &str);
    #[doc = "Get the `popupWindowURI` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PopupBlockedEventInit`*"]
    #[wasm_bindgen(method, getter = "popupWindowURI")]
    pub fn get_popup_window_uri(this: &PopupBlockedEventInit) -> Option<::alloc::string::String>;
    #[doc = "Change the `popupWindowURI` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PopupBlockedEventInit`*"]
    #[wasm_bindgen(method, setter = "popupWindowURI")]
    pub fn set_popup_window_uri(this: &PopupBlockedEventInit, val: Option<&str>);
    #[cfg(feature = "Window")]
    #[doc = "Get the `requestingWindow` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PopupBlockedEventInit`, `Window`*"]
    #[wasm_bindgen(method, getter = "requestingWindow")]
    pub fn get_requesting_window(this: &PopupBlockedEventInit) -> Option<Window>;
    #[cfg(feature = "Window")]
    #[doc = "Change the `requestingWindow` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PopupBlockedEventInit`, `Window`*"]
    #[wasm_bindgen(method, setter = "requestingWindow")]
    pub fn set_requesting_window(this: &PopupBlockedEventInit, val: Option<&Window>);
}
impl PopupBlockedEventInit {
    #[doc = "Construct a new `PopupBlockedEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PopupBlockedEventInit`*"]
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
    #[deprecated = "Use `set_popup_window_features()` instead."]
    pub fn popup_window_features(&mut self, val: &str) -> &mut Self {
        self.set_popup_window_features(val);
        self
    }
    #[deprecated = "Use `set_popup_window_name()` instead."]
    pub fn popup_window_name(&mut self, val: &str) -> &mut Self {
        self.set_popup_window_name(val);
        self
    }
    #[deprecated = "Use `set_popup_window_uri()` instead."]
    pub fn popup_window_uri(&mut self, val: Option<&str>) -> &mut Self {
        self.set_popup_window_uri(val);
        self
    }
    #[cfg(feature = "Window")]
    #[deprecated = "Use `set_requesting_window()` instead."]
    pub fn requesting_window(&mut self, val: Option<&Window>) -> &mut Self {
        self.set_requesting_window(val);
        self
    }
}
impl Default for PopupBlockedEventInit {
    fn default() -> Self {
        Self::new()
    }
}
