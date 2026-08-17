#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "VideoConfiguration")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `VideoConfiguration` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoConfiguration`*"]
    pub type VideoConfiguration;
    #[doc = "Get the `bitrate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoConfiguration`*"]
    #[wasm_bindgen(method, getter = "bitrate")]
    pub fn get_bitrate(this: &VideoConfiguration) -> Option<f64>;
    #[doc = "Change the `bitrate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoConfiguration`*"]
    #[wasm_bindgen(method, setter = "bitrate")]
    pub fn set_bitrate(this: &VideoConfiguration, val: f64);
    #[doc = "Change the `bitrate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoConfiguration`*"]
    #[wasm_bindgen(method, setter = "bitrate")]
    pub fn set_bitrate_u32(this: &VideoConfiguration, val: u32);
    #[doc = "Change the `bitrate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoConfiguration`*"]
    #[wasm_bindgen(method, setter = "bitrate")]
    pub fn set_bitrate_f64(this: &VideoConfiguration, val: f64);
    #[doc = "Get the `contentType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoConfiguration`*"]
    #[wasm_bindgen(method, getter = "contentType")]
    pub fn get_content_type(this: &VideoConfiguration) -> Option<::alloc::string::String>;
    #[doc = "Change the `contentType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoConfiguration`*"]
    #[wasm_bindgen(method, setter = "contentType")]
    pub fn set_content_type(this: &VideoConfiguration, val: &str);
    #[doc = "Get the `framerate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoConfiguration`*"]
    #[wasm_bindgen(method, getter = "framerate")]
    pub fn get_framerate(this: &VideoConfiguration) -> Option<::alloc::string::String>;
    #[doc = "Change the `framerate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoConfiguration`*"]
    #[wasm_bindgen(method, setter = "framerate")]
    pub fn set_framerate(this: &VideoConfiguration, val: &str);
    #[doc = "Get the `height` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoConfiguration`*"]
    #[wasm_bindgen(method, getter = "height")]
    pub fn get_height(this: &VideoConfiguration) -> Option<u32>;
    #[doc = "Change the `height` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoConfiguration`*"]
    #[wasm_bindgen(method, setter = "height")]
    pub fn set_height(this: &VideoConfiguration, val: u32);
    #[doc = "Get the `width` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoConfiguration`*"]
    #[wasm_bindgen(method, getter = "width")]
    pub fn get_width(this: &VideoConfiguration) -> Option<u32>;
    #[doc = "Change the `width` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoConfiguration`*"]
    #[wasm_bindgen(method, setter = "width")]
    pub fn set_width(this: &VideoConfiguration, val: u32);
}
impl VideoConfiguration {
    #[doc = "Construct a new `VideoConfiguration`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoConfiguration`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_bitrate()` instead."]
    pub fn bitrate(&mut self, val: f64) -> &mut Self {
        self.set_bitrate(val);
        self
    }
    #[deprecated = "Use `set_content_type()` instead."]
    pub fn content_type(&mut self, val: &str) -> &mut Self {
        self.set_content_type(val);
        self
    }
    #[deprecated = "Use `set_framerate()` instead."]
    pub fn framerate(&mut self, val: &str) -> &mut Self {
        self.set_framerate(val);
        self
    }
    #[deprecated = "Use `set_height()` instead."]
    pub fn height(&mut self, val: u32) -> &mut Self {
        self.set_height(val);
        self
    }
    #[deprecated = "Use `set_width()` instead."]
    pub fn width(&mut self, val: u32) -> &mut Self {
        self.set_width(val);
        self
    }
}
impl Default for VideoConfiguration {
    fn default() -> Self {
        Self::new()
    }
}
