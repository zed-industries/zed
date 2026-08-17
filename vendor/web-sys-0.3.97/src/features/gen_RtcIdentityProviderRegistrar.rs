#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (is_type_of = | _ | false , extends = "::js_sys::Object" , js_name = "RTCIdentityProviderRegistrar" , typescript_type = "RTCIdentityProviderRegistrar")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcIdentityProviderRegistrar` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCIdentityProviderRegistrar)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityProviderRegistrar`*"]
    pub type RtcIdentityProviderRegistrar;
    #[cfg(feature = "RtcIdentityProvider")]
    #[wasm_bindgen(method, js_class = "RTCIdentityProviderRegistrar")]
    #[doc = "The `register()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCIdentityProviderRegistrar/register)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityProvider`, `RtcIdentityProviderRegistrar`*"]
    pub fn register(this: &RtcIdentityProviderRegistrar, idp: &RtcIdentityProvider);
}
