#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "MediaKeySystemConfiguration")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaKeySystemConfiguration` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySystemConfiguration`*"]
    pub type MediaKeySystemConfiguration;
    #[doc = "Get the `audioCapabilities` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySystemConfiguration`*"]
    #[wasm_bindgen(method, getter = "audioCapabilities")]
    pub fn get_audio_capabilities(this: &MediaKeySystemConfiguration) -> Option<::js_sys::Array>;
    #[doc = "Change the `audioCapabilities` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySystemConfiguration`*"]
    #[wasm_bindgen(method, setter = "audioCapabilities")]
    pub fn set_audio_capabilities(
        this: &MediaKeySystemConfiguration,
        val: &::wasm_bindgen::JsValue,
    );
    #[cfg(feature = "MediaKeysRequirement")]
    #[doc = "Get the `distinctiveIdentifier` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySystemConfiguration`, `MediaKeysRequirement`*"]
    #[wasm_bindgen(method, getter = "distinctiveIdentifier")]
    pub fn get_distinctive_identifier(
        this: &MediaKeySystemConfiguration,
    ) -> Option<MediaKeysRequirement>;
    #[cfg(feature = "MediaKeysRequirement")]
    #[doc = "Change the `distinctiveIdentifier` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySystemConfiguration`, `MediaKeysRequirement`*"]
    #[wasm_bindgen(method, setter = "distinctiveIdentifier")]
    pub fn set_distinctive_identifier(
        this: &MediaKeySystemConfiguration,
        val: MediaKeysRequirement,
    );
    #[doc = "Get the `initDataTypes` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySystemConfiguration`*"]
    #[wasm_bindgen(method, getter = "initDataTypes")]
    pub fn get_init_data_types(this: &MediaKeySystemConfiguration) -> Option<::js_sys::Array>;
    #[doc = "Change the `initDataTypes` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySystemConfiguration`*"]
    #[wasm_bindgen(method, setter = "initDataTypes")]
    pub fn set_init_data_types(this: &MediaKeySystemConfiguration, val: &::wasm_bindgen::JsValue);
    #[doc = "Get the `label` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySystemConfiguration`*"]
    #[wasm_bindgen(method, getter = "label")]
    pub fn get_label(this: &MediaKeySystemConfiguration) -> Option<::alloc::string::String>;
    #[doc = "Change the `label` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySystemConfiguration`*"]
    #[wasm_bindgen(method, setter = "label")]
    pub fn set_label(this: &MediaKeySystemConfiguration, val: &str);
    #[cfg(feature = "MediaKeysRequirement")]
    #[doc = "Get the `persistentState` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySystemConfiguration`, `MediaKeysRequirement`*"]
    #[wasm_bindgen(method, getter = "persistentState")]
    pub fn get_persistent_state(this: &MediaKeySystemConfiguration)
        -> Option<MediaKeysRequirement>;
    #[cfg(feature = "MediaKeysRequirement")]
    #[doc = "Change the `persistentState` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySystemConfiguration`, `MediaKeysRequirement`*"]
    #[wasm_bindgen(method, setter = "persistentState")]
    pub fn set_persistent_state(this: &MediaKeySystemConfiguration, val: MediaKeysRequirement);
    #[doc = "Get the `sessionTypes` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySystemConfiguration`*"]
    #[wasm_bindgen(method, getter = "sessionTypes")]
    pub fn get_session_types(this: &MediaKeySystemConfiguration) -> Option<::js_sys::Array>;
    #[doc = "Change the `sessionTypes` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySystemConfiguration`*"]
    #[wasm_bindgen(method, setter = "sessionTypes")]
    pub fn set_session_types(this: &MediaKeySystemConfiguration, val: &::wasm_bindgen::JsValue);
    #[doc = "Get the `videoCapabilities` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySystemConfiguration`*"]
    #[wasm_bindgen(method, getter = "videoCapabilities")]
    pub fn get_video_capabilities(this: &MediaKeySystemConfiguration) -> Option<::js_sys::Array>;
    #[doc = "Change the `videoCapabilities` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySystemConfiguration`*"]
    #[wasm_bindgen(method, setter = "videoCapabilities")]
    pub fn set_video_capabilities(
        this: &MediaKeySystemConfiguration,
        val: &::wasm_bindgen::JsValue,
    );
}
impl MediaKeySystemConfiguration {
    #[doc = "Construct a new `MediaKeySystemConfiguration`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySystemConfiguration`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_audio_capabilities()` instead."]
    pub fn audio_capabilities(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_audio_capabilities(val);
        self
    }
    #[cfg(feature = "MediaKeysRequirement")]
    #[deprecated = "Use `set_distinctive_identifier()` instead."]
    pub fn distinctive_identifier(&mut self, val: MediaKeysRequirement) -> &mut Self {
        self.set_distinctive_identifier(val);
        self
    }
    #[deprecated = "Use `set_init_data_types()` instead."]
    pub fn init_data_types(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_init_data_types(val);
        self
    }
    #[deprecated = "Use `set_label()` instead."]
    pub fn label(&mut self, val: &str) -> &mut Self {
        self.set_label(val);
        self
    }
    #[cfg(feature = "MediaKeysRequirement")]
    #[deprecated = "Use `set_persistent_state()` instead."]
    pub fn persistent_state(&mut self, val: MediaKeysRequirement) -> &mut Self {
        self.set_persistent_state(val);
        self
    }
    #[deprecated = "Use `set_session_types()` instead."]
    pub fn session_types(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_session_types(val);
        self
    }
    #[deprecated = "Use `set_video_capabilities()` instead."]
    pub fn video_capabilities(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_video_capabilities(val);
        self
    }
}
impl Default for MediaKeySystemConfiguration {
    fn default() -> Self {
        Self::new()
    }
}
