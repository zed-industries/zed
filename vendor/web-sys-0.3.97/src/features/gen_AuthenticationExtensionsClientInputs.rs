#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "AuthenticationExtensionsClientInputs"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AuthenticationExtensionsClientInputs` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientInputs`*"]
    pub type AuthenticationExtensionsClientInputs;
    #[doc = "Get the `appid` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientInputs`*"]
    #[wasm_bindgen(method, getter = "appid")]
    pub fn get_appid(
        this: &AuthenticationExtensionsClientInputs,
    ) -> Option<::alloc::string::String>;
    #[doc = "Change the `appid` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientInputs`*"]
    #[wasm_bindgen(method, setter = "appid")]
    pub fn set_appid(this: &AuthenticationExtensionsClientInputs, val: &str);
    #[doc = "Get the `appidExclude` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientInputs`*"]
    #[wasm_bindgen(method, getter = "appidExclude")]
    pub fn get_appid_exclude(
        this: &AuthenticationExtensionsClientInputs,
    ) -> Option<::alloc::string::String>;
    #[doc = "Change the `appidExclude` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientInputs`*"]
    #[wasm_bindgen(method, setter = "appidExclude")]
    pub fn set_appid_exclude(this: &AuthenticationExtensionsClientInputs, val: &str);
    #[doc = "Get the `credProps` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientInputs`*"]
    #[wasm_bindgen(method, getter = "credProps")]
    pub fn get_cred_props(this: &AuthenticationExtensionsClientInputs) -> Option<bool>;
    #[doc = "Change the `credProps` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientInputs`*"]
    #[wasm_bindgen(method, setter = "credProps")]
    pub fn set_cred_props(this: &AuthenticationExtensionsClientInputs, val: bool);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "AuthenticationExtensionsDevicePublicKeyInputs")]
    #[doc = "Get the `devicePubKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientInputs`, `AuthenticationExtensionsDevicePublicKeyInputs`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "devicePubKey")]
    pub fn get_device_pub_key(
        this: &AuthenticationExtensionsClientInputs,
    ) -> Option<AuthenticationExtensionsDevicePublicKeyInputs>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "AuthenticationExtensionsDevicePublicKeyInputs")]
    #[doc = "Change the `devicePubKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientInputs`, `AuthenticationExtensionsDevicePublicKeyInputs`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "devicePubKey")]
    pub fn set_device_pub_key(
        this: &AuthenticationExtensionsClientInputs,
        val: &AuthenticationExtensionsDevicePublicKeyInputs,
    );
    #[cfg(feature = "AuthenticationExtensionsLargeBlobInputs")]
    #[doc = "Get the `largeBlob` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientInputs`, `AuthenticationExtensionsLargeBlobInputs`*"]
    #[wasm_bindgen(method, getter = "largeBlob")]
    pub fn get_large_blob(
        this: &AuthenticationExtensionsClientInputs,
    ) -> Option<AuthenticationExtensionsLargeBlobInputs>;
    #[cfg(feature = "AuthenticationExtensionsLargeBlobInputs")]
    #[doc = "Change the `largeBlob` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientInputs`, `AuthenticationExtensionsLargeBlobInputs`*"]
    #[wasm_bindgen(method, setter = "largeBlob")]
    pub fn set_large_blob(
        this: &AuthenticationExtensionsClientInputs,
        val: &AuthenticationExtensionsLargeBlobInputs,
    );
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "AuthenticationExtensionsPrfInputs")]
    #[doc = "Get the `prf` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientInputs`, `AuthenticationExtensionsPrfInputs`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "prf")]
    pub fn get_prf(
        this: &AuthenticationExtensionsClientInputs,
    ) -> Option<AuthenticationExtensionsPrfInputs>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "AuthenticationExtensionsPrfInputs")]
    #[doc = "Change the `prf` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientInputs`, `AuthenticationExtensionsPrfInputs`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "prf")]
    pub fn set_prf(
        this: &AuthenticationExtensionsClientInputs,
        val: &AuthenticationExtensionsPrfInputs,
    );
    #[doc = "Get the `uvm` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientInputs`*"]
    #[wasm_bindgen(method, getter = "uvm")]
    pub fn get_uvm(this: &AuthenticationExtensionsClientInputs) -> Option<bool>;
    #[doc = "Change the `uvm` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientInputs`*"]
    #[wasm_bindgen(method, setter = "uvm")]
    pub fn set_uvm(this: &AuthenticationExtensionsClientInputs, val: bool);
}
impl AuthenticationExtensionsClientInputs {
    #[doc = "Construct a new `AuthenticationExtensionsClientInputs`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientInputs`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_appid()` instead."]
    pub fn appid(&mut self, val: &str) -> &mut Self {
        self.set_appid(val);
        self
    }
    #[deprecated = "Use `set_appid_exclude()` instead."]
    pub fn appid_exclude(&mut self, val: &str) -> &mut Self {
        self.set_appid_exclude(val);
        self
    }
    #[deprecated = "Use `set_cred_props()` instead."]
    pub fn cred_props(&mut self, val: bool) -> &mut Self {
        self.set_cred_props(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "AuthenticationExtensionsDevicePublicKeyInputs")]
    #[deprecated = "Use `set_device_pub_key()` instead."]
    pub fn device_pub_key(
        &mut self,
        val: &AuthenticationExtensionsDevicePublicKeyInputs,
    ) -> &mut Self {
        self.set_device_pub_key(val);
        self
    }
    #[cfg(feature = "AuthenticationExtensionsLargeBlobInputs")]
    #[deprecated = "Use `set_large_blob()` instead."]
    pub fn large_blob(&mut self, val: &AuthenticationExtensionsLargeBlobInputs) -> &mut Self {
        self.set_large_blob(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "AuthenticationExtensionsPrfInputs")]
    #[deprecated = "Use `set_prf()` instead."]
    pub fn prf(&mut self, val: &AuthenticationExtensionsPrfInputs) -> &mut Self {
        self.set_prf(val);
        self
    }
    #[deprecated = "Use `set_uvm()` instead."]
    pub fn uvm(&mut self, val: bool) -> &mut Self {
        self.set_uvm(val);
        self
    }
}
impl Default for AuthenticationExtensionsClientInputs {
    fn default() -> Self {
        Self::new()
    }
}
