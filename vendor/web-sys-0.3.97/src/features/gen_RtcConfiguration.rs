#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "RTCConfiguration")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcConfiguration` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcConfiguration`*"]
    pub type RtcConfiguration;
    #[cfg(feature = "RtcBundlePolicy")]
    #[doc = "Get the `bundlePolicy` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcBundlePolicy`, `RtcConfiguration`*"]
    #[wasm_bindgen(method, getter = "bundlePolicy")]
    pub fn get_bundle_policy(this: &RtcConfiguration) -> Option<RtcBundlePolicy>;
    #[cfg(feature = "RtcBundlePolicy")]
    #[doc = "Change the `bundlePolicy` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcBundlePolicy`, `RtcConfiguration`*"]
    #[wasm_bindgen(method, setter = "bundlePolicy")]
    pub fn set_bundle_policy(this: &RtcConfiguration, val: RtcBundlePolicy);
    #[doc = "Get the `certificates` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcConfiguration`*"]
    #[wasm_bindgen(method, getter = "certificates")]
    pub fn get_certificates(this: &RtcConfiguration) -> Option<::js_sys::Array>;
    #[doc = "Change the `certificates` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcConfiguration`*"]
    #[wasm_bindgen(method, setter = "certificates")]
    pub fn set_certificates(this: &RtcConfiguration, val: &::wasm_bindgen::JsValue);
    #[doc = "Get the `iceServers` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcConfiguration`*"]
    #[wasm_bindgen(method, getter = "iceServers")]
    pub fn get_ice_servers(this: &RtcConfiguration) -> Option<::js_sys::Array>;
    #[doc = "Change the `iceServers` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcConfiguration`*"]
    #[wasm_bindgen(method, setter = "iceServers")]
    pub fn set_ice_servers(this: &RtcConfiguration, val: &::wasm_bindgen::JsValue);
    #[cfg(feature = "RtcIceTransportPolicy")]
    #[doc = "Get the `iceTransportPolicy` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcConfiguration`, `RtcIceTransportPolicy`*"]
    #[wasm_bindgen(method, getter = "iceTransportPolicy")]
    pub fn get_ice_transport_policy(this: &RtcConfiguration) -> Option<RtcIceTransportPolicy>;
    #[cfg(feature = "RtcIceTransportPolicy")]
    #[doc = "Change the `iceTransportPolicy` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcConfiguration`, `RtcIceTransportPolicy`*"]
    #[wasm_bindgen(method, setter = "iceTransportPolicy")]
    pub fn set_ice_transport_policy(this: &RtcConfiguration, val: RtcIceTransportPolicy);
    #[doc = "Get the `peerIdentity` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcConfiguration`*"]
    #[wasm_bindgen(method, getter = "peerIdentity")]
    pub fn get_peer_identity(this: &RtcConfiguration) -> Option<::alloc::string::String>;
    #[doc = "Change the `peerIdentity` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcConfiguration`*"]
    #[wasm_bindgen(method, setter = "peerIdentity")]
    pub fn set_peer_identity(this: &RtcConfiguration, val: Option<&str>);
}
impl RtcConfiguration {
    #[doc = "Construct a new `RtcConfiguration`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcConfiguration`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(feature = "RtcBundlePolicy")]
    #[deprecated = "Use `set_bundle_policy()` instead."]
    pub fn bundle_policy(&mut self, val: RtcBundlePolicy) -> &mut Self {
        self.set_bundle_policy(val);
        self
    }
    #[deprecated = "Use `set_certificates()` instead."]
    pub fn certificates(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_certificates(val);
        self
    }
    #[deprecated = "Use `set_ice_servers()` instead."]
    pub fn ice_servers(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_ice_servers(val);
        self
    }
    #[cfg(feature = "RtcIceTransportPolicy")]
    #[deprecated = "Use `set_ice_transport_policy()` instead."]
    pub fn ice_transport_policy(&mut self, val: RtcIceTransportPolicy) -> &mut Self {
        self.set_ice_transport_policy(val);
        self
    }
    #[deprecated = "Use `set_peer_identity()` instead."]
    pub fn peer_identity(&mut self, val: Option<&str>) -> &mut Self {
        self.set_peer_identity(val);
        self
    }
}
impl Default for RtcConfiguration {
    fn default() -> Self {
        Self::new()
    }
}
