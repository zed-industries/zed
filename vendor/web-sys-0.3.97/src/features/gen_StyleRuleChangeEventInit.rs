#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "StyleRuleChangeEventInit")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `StyleRuleChangeEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StyleRuleChangeEventInit`*"]
    pub type StyleRuleChangeEventInit;
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StyleRuleChangeEventInit`*"]
    #[wasm_bindgen(method, getter = "bubbles")]
    pub fn get_bubbles(this: &StyleRuleChangeEventInit) -> Option<bool>;
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StyleRuleChangeEventInit`*"]
    #[wasm_bindgen(method, setter = "bubbles")]
    pub fn set_bubbles(this: &StyleRuleChangeEventInit, val: bool);
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StyleRuleChangeEventInit`*"]
    #[wasm_bindgen(method, getter = "cancelable")]
    pub fn get_cancelable(this: &StyleRuleChangeEventInit) -> Option<bool>;
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StyleRuleChangeEventInit`*"]
    #[wasm_bindgen(method, setter = "cancelable")]
    pub fn set_cancelable(this: &StyleRuleChangeEventInit, val: bool);
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StyleRuleChangeEventInit`*"]
    #[wasm_bindgen(method, getter = "composed")]
    pub fn get_composed(this: &StyleRuleChangeEventInit) -> Option<bool>;
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StyleRuleChangeEventInit`*"]
    #[wasm_bindgen(method, setter = "composed")]
    pub fn set_composed(this: &StyleRuleChangeEventInit, val: bool);
    #[cfg(feature = "CssRule")]
    #[doc = "Get the `rule` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssRule`, `StyleRuleChangeEventInit`*"]
    #[wasm_bindgen(method, getter = "rule")]
    pub fn get_rule(this: &StyleRuleChangeEventInit) -> Option<CssRule>;
    #[cfg(feature = "CssRule")]
    #[doc = "Change the `rule` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssRule`, `StyleRuleChangeEventInit`*"]
    #[wasm_bindgen(method, setter = "rule")]
    pub fn set_rule(this: &StyleRuleChangeEventInit, val: Option<&CssRule>);
    #[cfg(feature = "CssStyleSheet")]
    #[doc = "Get the `stylesheet` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssStyleSheet`, `StyleRuleChangeEventInit`*"]
    #[wasm_bindgen(method, getter = "stylesheet")]
    pub fn get_stylesheet(this: &StyleRuleChangeEventInit) -> Option<CssStyleSheet>;
    #[cfg(feature = "CssStyleSheet")]
    #[doc = "Change the `stylesheet` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssStyleSheet`, `StyleRuleChangeEventInit`*"]
    #[wasm_bindgen(method, setter = "stylesheet")]
    pub fn set_stylesheet(this: &StyleRuleChangeEventInit, val: Option<&CssStyleSheet>);
}
impl StyleRuleChangeEventInit {
    #[doc = "Construct a new `StyleRuleChangeEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StyleRuleChangeEventInit`*"]
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
    #[cfg(feature = "CssRule")]
    #[deprecated = "Use `set_rule()` instead."]
    pub fn rule(&mut self, val: Option<&CssRule>) -> &mut Self {
        self.set_rule(val);
        self
    }
    #[cfg(feature = "CssStyleSheet")]
    #[deprecated = "Use `set_stylesheet()` instead."]
    pub fn stylesheet(&mut self, val: Option<&CssStyleSheet>) -> &mut Self {
        self.set_stylesheet(val);
        self
    }
}
impl Default for StyleRuleChangeEventInit {
    fn default() -> Self {
        Self::new()
    }
}
