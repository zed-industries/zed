#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "MutationObserverInit")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MutationObserverInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationObserverInit`*"]
    pub type MutationObserverInit;
    #[doc = "Get the `animations` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationObserverInit`*"]
    #[wasm_bindgen(method, getter = "animations")]
    pub fn get_animations(this: &MutationObserverInit) -> Option<bool>;
    #[doc = "Change the `animations` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationObserverInit`*"]
    #[wasm_bindgen(method, setter = "animations")]
    pub fn set_animations(this: &MutationObserverInit, val: bool);
    #[doc = "Get the `attributeFilter` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationObserverInit`*"]
    #[wasm_bindgen(method, getter = "attributeFilter")]
    pub fn get_attribute_filter(this: &MutationObserverInit) -> Option<::js_sys::Array>;
    #[doc = "Change the `attributeFilter` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationObserverInit`*"]
    #[wasm_bindgen(method, setter = "attributeFilter")]
    pub fn set_attribute_filter(this: &MutationObserverInit, val: &::wasm_bindgen::JsValue);
    #[doc = "Get the `attributeOldValue` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationObserverInit`*"]
    #[wasm_bindgen(method, getter = "attributeOldValue")]
    pub fn get_attribute_old_value(this: &MutationObserverInit) -> Option<bool>;
    #[doc = "Change the `attributeOldValue` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationObserverInit`*"]
    #[wasm_bindgen(method, setter = "attributeOldValue")]
    pub fn set_attribute_old_value(this: &MutationObserverInit, val: bool);
    #[doc = "Get the `attributes` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationObserverInit`*"]
    #[wasm_bindgen(method, getter = "attributes")]
    pub fn get_attributes(this: &MutationObserverInit) -> Option<bool>;
    #[doc = "Change the `attributes` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationObserverInit`*"]
    #[wasm_bindgen(method, setter = "attributes")]
    pub fn set_attributes(this: &MutationObserverInit, val: bool);
    #[doc = "Get the `characterData` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationObserverInit`*"]
    #[wasm_bindgen(method, getter = "characterData")]
    pub fn get_character_data(this: &MutationObserverInit) -> Option<bool>;
    #[doc = "Change the `characterData` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationObserverInit`*"]
    #[wasm_bindgen(method, setter = "characterData")]
    pub fn set_character_data(this: &MutationObserverInit, val: bool);
    #[doc = "Get the `characterDataOldValue` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationObserverInit`*"]
    #[wasm_bindgen(method, getter = "characterDataOldValue")]
    pub fn get_character_data_old_value(this: &MutationObserverInit) -> Option<bool>;
    #[doc = "Change the `characterDataOldValue` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationObserverInit`*"]
    #[wasm_bindgen(method, setter = "characterDataOldValue")]
    pub fn set_character_data_old_value(this: &MutationObserverInit, val: bool);
    #[doc = "Get the `childList` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationObserverInit`*"]
    #[wasm_bindgen(method, getter = "childList")]
    pub fn get_child_list(this: &MutationObserverInit) -> Option<bool>;
    #[doc = "Change the `childList` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationObserverInit`*"]
    #[wasm_bindgen(method, setter = "childList")]
    pub fn set_child_list(this: &MutationObserverInit, val: bool);
    #[doc = "Get the `nativeAnonymousChildList` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationObserverInit`*"]
    #[wasm_bindgen(method, getter = "nativeAnonymousChildList")]
    pub fn get_native_anonymous_child_list(this: &MutationObserverInit) -> Option<bool>;
    #[doc = "Change the `nativeAnonymousChildList` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationObserverInit`*"]
    #[wasm_bindgen(method, setter = "nativeAnonymousChildList")]
    pub fn set_native_anonymous_child_list(this: &MutationObserverInit, val: bool);
    #[doc = "Get the `subtree` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationObserverInit`*"]
    #[wasm_bindgen(method, getter = "subtree")]
    pub fn get_subtree(this: &MutationObserverInit) -> Option<bool>;
    #[doc = "Change the `subtree` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationObserverInit`*"]
    #[wasm_bindgen(method, setter = "subtree")]
    pub fn set_subtree(this: &MutationObserverInit, val: bool);
}
impl MutationObserverInit {
    #[doc = "Construct a new `MutationObserverInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationObserverInit`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_animations()` instead."]
    pub fn animations(&mut self, val: bool) -> &mut Self {
        self.set_animations(val);
        self
    }
    #[deprecated = "Use `set_attribute_filter()` instead."]
    pub fn attribute_filter(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_attribute_filter(val);
        self
    }
    #[deprecated = "Use `set_attribute_old_value()` instead."]
    pub fn attribute_old_value(&mut self, val: bool) -> &mut Self {
        self.set_attribute_old_value(val);
        self
    }
    #[deprecated = "Use `set_attributes()` instead."]
    pub fn attributes(&mut self, val: bool) -> &mut Self {
        self.set_attributes(val);
        self
    }
    #[deprecated = "Use `set_character_data()` instead."]
    pub fn character_data(&mut self, val: bool) -> &mut Self {
        self.set_character_data(val);
        self
    }
    #[deprecated = "Use `set_character_data_old_value()` instead."]
    pub fn character_data_old_value(&mut self, val: bool) -> &mut Self {
        self.set_character_data_old_value(val);
        self
    }
    #[deprecated = "Use `set_child_list()` instead."]
    pub fn child_list(&mut self, val: bool) -> &mut Self {
        self.set_child_list(val);
        self
    }
    #[deprecated = "Use `set_native_anonymous_child_list()` instead."]
    pub fn native_anonymous_child_list(&mut self, val: bool) -> &mut Self {
        self.set_native_anonymous_child_list(val);
        self
    }
    #[deprecated = "Use `set_subtree()` instead."]
    pub fn subtree(&mut self, val: bool) -> &mut Self {
        self.set_subtree(val);
        self
    }
}
impl Default for MutationObserverInit {
    fn default() -> Self {
        Self::new()
    }
}
