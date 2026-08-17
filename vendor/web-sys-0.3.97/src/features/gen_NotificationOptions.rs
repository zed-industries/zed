#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "NotificationOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `NotificationOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NotificationOptions`*"]
    pub type NotificationOptions;
    #[doc = "Get the `actions` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NotificationOptions`*"]
    #[wasm_bindgen(method, getter = "actions")]
    pub fn get_actions(this: &NotificationOptions) -> Option<::js_sys::Array>;
    #[doc = "Change the `actions` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NotificationOptions`*"]
    #[wasm_bindgen(method, setter = "actions")]
    pub fn set_actions(this: &NotificationOptions, val: &::wasm_bindgen::JsValue);
    #[doc = "Get the `badge` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NotificationOptions`*"]
    #[wasm_bindgen(method, getter = "badge")]
    pub fn get_badge(this: &NotificationOptions) -> Option<::alloc::string::String>;
    #[doc = "Change the `badge` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NotificationOptions`*"]
    #[wasm_bindgen(method, setter = "badge")]
    pub fn set_badge(this: &NotificationOptions, val: &str);
    #[doc = "Get the `body` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NotificationOptions`*"]
    #[wasm_bindgen(method, getter = "body")]
    pub fn get_body(this: &NotificationOptions) -> Option<::alloc::string::String>;
    #[doc = "Change the `body` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NotificationOptions`*"]
    #[wasm_bindgen(method, setter = "body")]
    pub fn set_body(this: &NotificationOptions, val: &str);
    #[doc = "Get the `data` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NotificationOptions`*"]
    #[wasm_bindgen(method, getter = "data")]
    pub fn get_data(this: &NotificationOptions) -> ::wasm_bindgen::JsValue;
    #[doc = "Change the `data` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NotificationOptions`*"]
    #[wasm_bindgen(method, setter = "data")]
    pub fn set_data(this: &NotificationOptions, val: &::wasm_bindgen::JsValue);
    #[cfg(feature = "NotificationDirection")]
    #[doc = "Get the `dir` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NotificationDirection`, `NotificationOptions`*"]
    #[wasm_bindgen(method, getter = "dir")]
    pub fn get_dir(this: &NotificationOptions) -> Option<NotificationDirection>;
    #[cfg(feature = "NotificationDirection")]
    #[doc = "Change the `dir` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NotificationDirection`, `NotificationOptions`*"]
    #[wasm_bindgen(method, setter = "dir")]
    pub fn set_dir(this: &NotificationOptions, val: NotificationDirection);
    #[doc = "Get the `icon` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NotificationOptions`*"]
    #[wasm_bindgen(method, getter = "icon")]
    pub fn get_icon(this: &NotificationOptions) -> Option<::alloc::string::String>;
    #[doc = "Change the `icon` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NotificationOptions`*"]
    #[wasm_bindgen(method, setter = "icon")]
    pub fn set_icon(this: &NotificationOptions, val: &str);
    #[doc = "Get the `image` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NotificationOptions`*"]
    #[wasm_bindgen(method, getter = "image")]
    pub fn get_image(this: &NotificationOptions) -> Option<::alloc::string::String>;
    #[doc = "Change the `image` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NotificationOptions`*"]
    #[wasm_bindgen(method, setter = "image")]
    pub fn set_image(this: &NotificationOptions, val: &str);
    #[doc = "Get the `lang` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NotificationOptions`*"]
    #[wasm_bindgen(method, getter = "lang")]
    pub fn get_lang(this: &NotificationOptions) -> Option<::alloc::string::String>;
    #[doc = "Change the `lang` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NotificationOptions`*"]
    #[wasm_bindgen(method, setter = "lang")]
    pub fn set_lang(this: &NotificationOptions, val: &str);
    #[doc = "Get the `renotify` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NotificationOptions`*"]
    #[wasm_bindgen(method, getter = "renotify")]
    pub fn get_renotify(this: &NotificationOptions) -> Option<bool>;
    #[doc = "Change the `renotify` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NotificationOptions`*"]
    #[wasm_bindgen(method, setter = "renotify")]
    pub fn set_renotify(this: &NotificationOptions, val: bool);
    #[doc = "Get the `requireInteraction` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NotificationOptions`*"]
    #[wasm_bindgen(method, getter = "requireInteraction")]
    pub fn get_require_interaction(this: &NotificationOptions) -> Option<bool>;
    #[doc = "Change the `requireInteraction` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NotificationOptions`*"]
    #[wasm_bindgen(method, setter = "requireInteraction")]
    pub fn set_require_interaction(this: &NotificationOptions, val: bool);
    #[doc = "Get the `silent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NotificationOptions`*"]
    #[wasm_bindgen(method, getter = "silent")]
    pub fn get_silent(this: &NotificationOptions) -> Option<bool>;
    #[doc = "Change the `silent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NotificationOptions`*"]
    #[wasm_bindgen(method, setter = "silent")]
    pub fn set_silent(this: &NotificationOptions, val: Option<bool>);
    #[doc = "Get the `tag` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NotificationOptions`*"]
    #[wasm_bindgen(method, getter = "tag")]
    pub fn get_tag(this: &NotificationOptions) -> Option<::alloc::string::String>;
    #[doc = "Change the `tag` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NotificationOptions`*"]
    #[wasm_bindgen(method, setter = "tag")]
    pub fn set_tag(this: &NotificationOptions, val: &str);
    #[doc = "Get the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NotificationOptions`*"]
    #[wasm_bindgen(method, getter = "timestamp")]
    pub fn get_timestamp(this: &NotificationOptions) -> Option<f64>;
    #[doc = "Change the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NotificationOptions`*"]
    #[wasm_bindgen(method, setter = "timestamp")]
    pub fn set_timestamp(this: &NotificationOptions, val: f64);
    #[doc = "Change the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NotificationOptions`*"]
    #[wasm_bindgen(method, setter = "timestamp")]
    pub fn set_timestamp_u32(this: &NotificationOptions, val: u32);
    #[doc = "Change the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NotificationOptions`*"]
    #[wasm_bindgen(method, setter = "timestamp")]
    pub fn set_timestamp_f64(this: &NotificationOptions, val: f64);
    #[doc = "Get the `vibrate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NotificationOptions`*"]
    #[wasm_bindgen(method, getter = "vibrate")]
    pub fn get_vibrate(this: &NotificationOptions) -> ::wasm_bindgen::JsValue;
    #[doc = "Change the `vibrate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NotificationOptions`*"]
    #[wasm_bindgen(method, setter = "vibrate")]
    pub fn set_vibrate(this: &NotificationOptions, val: &::wasm_bindgen::JsValue);
    #[doc = "Change the `vibrate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NotificationOptions`*"]
    #[wasm_bindgen(method, setter = "vibrate")]
    pub fn set_vibrate_u32(this: &NotificationOptions, val: u32);
    #[doc = "Change the `vibrate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NotificationOptions`*"]
    #[wasm_bindgen(method, setter = "vibrate")]
    pub fn set_vibrate_u32_sequence(this: &NotificationOptions, val: &::wasm_bindgen::JsValue);
}
impl NotificationOptions {
    #[doc = "Construct a new `NotificationOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NotificationOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_actions()` instead."]
    pub fn actions(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_actions(val);
        self
    }
    #[deprecated = "Use `set_badge()` instead."]
    pub fn badge(&mut self, val: &str) -> &mut Self {
        self.set_badge(val);
        self
    }
    #[deprecated = "Use `set_body()` instead."]
    pub fn body(&mut self, val: &str) -> &mut Self {
        self.set_body(val);
        self
    }
    #[deprecated = "Use `set_data()` instead."]
    pub fn data(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_data(val);
        self
    }
    #[cfg(feature = "NotificationDirection")]
    #[deprecated = "Use `set_dir()` instead."]
    pub fn dir(&mut self, val: NotificationDirection) -> &mut Self {
        self.set_dir(val);
        self
    }
    #[deprecated = "Use `set_icon()` instead."]
    pub fn icon(&mut self, val: &str) -> &mut Self {
        self.set_icon(val);
        self
    }
    #[deprecated = "Use `set_image()` instead."]
    pub fn image(&mut self, val: &str) -> &mut Self {
        self.set_image(val);
        self
    }
    #[deprecated = "Use `set_lang()` instead."]
    pub fn lang(&mut self, val: &str) -> &mut Self {
        self.set_lang(val);
        self
    }
    #[deprecated = "Use `set_renotify()` instead."]
    pub fn renotify(&mut self, val: bool) -> &mut Self {
        self.set_renotify(val);
        self
    }
    #[deprecated = "Use `set_require_interaction()` instead."]
    pub fn require_interaction(&mut self, val: bool) -> &mut Self {
        self.set_require_interaction(val);
        self
    }
    #[deprecated = "Use `set_silent()` instead."]
    pub fn silent(&mut self, val: Option<bool>) -> &mut Self {
        self.set_silent(val);
        self
    }
    #[deprecated = "Use `set_tag()` instead."]
    pub fn tag(&mut self, val: &str) -> &mut Self {
        self.set_tag(val);
        self
    }
    #[deprecated = "Use `set_timestamp()` instead."]
    pub fn timestamp(&mut self, val: f64) -> &mut Self {
        self.set_timestamp(val);
        self
    }
    #[deprecated = "Use `set_vibrate()` instead."]
    pub fn vibrate(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_vibrate(val);
        self
    }
}
impl Default for NotificationOptions {
    fn default() -> Self {
        Self::new()
    }
}
