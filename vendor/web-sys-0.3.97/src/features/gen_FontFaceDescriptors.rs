#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "FontFaceDescriptors")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `FontFaceDescriptors` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceDescriptors`*"]
    pub type FontFaceDescriptors;
    #[doc = "Get the `display` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceDescriptors`*"]
    #[wasm_bindgen(method, getter = "display")]
    pub fn get_display(this: &FontFaceDescriptors) -> Option<::alloc::string::String>;
    #[doc = "Change the `display` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceDescriptors`*"]
    #[wasm_bindgen(method, setter = "display")]
    pub fn set_display(this: &FontFaceDescriptors, val: &str);
    #[doc = "Get the `featureSettings` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceDescriptors`*"]
    #[wasm_bindgen(method, getter = "featureSettings")]
    pub fn get_feature_settings(this: &FontFaceDescriptors) -> Option<::alloc::string::String>;
    #[doc = "Change the `featureSettings` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceDescriptors`*"]
    #[wasm_bindgen(method, setter = "featureSettings")]
    pub fn set_feature_settings(this: &FontFaceDescriptors, val: &str);
    #[doc = "Get the `stretch` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceDescriptors`*"]
    #[wasm_bindgen(method, getter = "stretch")]
    pub fn get_stretch(this: &FontFaceDescriptors) -> Option<::alloc::string::String>;
    #[doc = "Change the `stretch` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceDescriptors`*"]
    #[wasm_bindgen(method, setter = "stretch")]
    pub fn set_stretch(this: &FontFaceDescriptors, val: &str);
    #[doc = "Get the `style` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceDescriptors`*"]
    #[wasm_bindgen(method, getter = "style")]
    pub fn get_style(this: &FontFaceDescriptors) -> Option<::alloc::string::String>;
    #[doc = "Change the `style` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceDescriptors`*"]
    #[wasm_bindgen(method, setter = "style")]
    pub fn set_style(this: &FontFaceDescriptors, val: &str);
    #[doc = "Get the `unicodeRange` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceDescriptors`*"]
    #[wasm_bindgen(method, getter = "unicodeRange")]
    pub fn get_unicode_range(this: &FontFaceDescriptors) -> Option<::alloc::string::String>;
    #[doc = "Change the `unicodeRange` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceDescriptors`*"]
    #[wasm_bindgen(method, setter = "unicodeRange")]
    pub fn set_unicode_range(this: &FontFaceDescriptors, val: &str);
    #[doc = "Get the `variant` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceDescriptors`*"]
    #[wasm_bindgen(method, getter = "variant")]
    pub fn get_variant(this: &FontFaceDescriptors) -> Option<::alloc::string::String>;
    #[doc = "Change the `variant` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceDescriptors`*"]
    #[wasm_bindgen(method, setter = "variant")]
    pub fn set_variant(this: &FontFaceDescriptors, val: &str);
    #[doc = "Get the `variationSettings` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceDescriptors`*"]
    #[wasm_bindgen(method, getter = "variationSettings")]
    pub fn get_variation_settings(this: &FontFaceDescriptors) -> Option<::alloc::string::String>;
    #[doc = "Change the `variationSettings` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceDescriptors`*"]
    #[wasm_bindgen(method, setter = "variationSettings")]
    pub fn set_variation_settings(this: &FontFaceDescriptors, val: &str);
    #[doc = "Get the `weight` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceDescriptors`*"]
    #[wasm_bindgen(method, getter = "weight")]
    pub fn get_weight(this: &FontFaceDescriptors) -> Option<::alloc::string::String>;
    #[doc = "Change the `weight` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceDescriptors`*"]
    #[wasm_bindgen(method, setter = "weight")]
    pub fn set_weight(this: &FontFaceDescriptors, val: &str);
}
impl FontFaceDescriptors {
    #[doc = "Construct a new `FontFaceDescriptors`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceDescriptors`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_display()` instead."]
    pub fn display(&mut self, val: &str) -> &mut Self {
        self.set_display(val);
        self
    }
    #[deprecated = "Use `set_feature_settings()` instead."]
    pub fn feature_settings(&mut self, val: &str) -> &mut Self {
        self.set_feature_settings(val);
        self
    }
    #[deprecated = "Use `set_stretch()` instead."]
    pub fn stretch(&mut self, val: &str) -> &mut Self {
        self.set_stretch(val);
        self
    }
    #[deprecated = "Use `set_style()` instead."]
    pub fn style(&mut self, val: &str) -> &mut Self {
        self.set_style(val);
        self
    }
    #[deprecated = "Use `set_unicode_range()` instead."]
    pub fn unicode_range(&mut self, val: &str) -> &mut Self {
        self.set_unicode_range(val);
        self
    }
    #[deprecated = "Use `set_variant()` instead."]
    pub fn variant(&mut self, val: &str) -> &mut Self {
        self.set_variant(val);
        self
    }
    #[deprecated = "Use `set_variation_settings()` instead."]
    pub fn variation_settings(&mut self, val: &str) -> &mut Self {
        self.set_variation_settings(val);
        self
    }
    #[deprecated = "Use `set_weight()` instead."]
    pub fn weight(&mut self, val: &str) -> &mut Self {
        self.set_weight(val);
        self
    }
}
impl Default for FontFaceDescriptors {
    fn default() -> Self {
        Self::new()
    }
}
