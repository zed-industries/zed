#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "AuthenticationExtensionsClientOutputs"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AuthenticationExtensionsClientOutputs` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientOutputs`*"]
    pub type AuthenticationExtensionsClientOutputs;
    #[doc = "Get the `appid` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientOutputs`*"]
    #[wasm_bindgen(method, getter = "appid")]
    pub fn get_appid(this: &AuthenticationExtensionsClientOutputs) -> Option<bool>;
    #[doc = "Change the `appid` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientOutputs`*"]
    #[wasm_bindgen(method, setter = "appid")]
    pub fn set_appid(this: &AuthenticationExtensionsClientOutputs, val: bool);
    #[doc = "Get the `appidExclude` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientOutputs`*"]
    #[wasm_bindgen(method, getter = "appidExclude")]
    pub fn get_appid_exclude(this: &AuthenticationExtensionsClientOutputs) -> Option<bool>;
    #[doc = "Change the `appidExclude` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientOutputs`*"]
    #[wasm_bindgen(method, setter = "appidExclude")]
    pub fn set_appid_exclude(this: &AuthenticationExtensionsClientOutputs, val: bool);
    #[cfg(feature = "CredentialPropertiesOutput")]
    #[doc = "Get the `credProps` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientOutputs`, `CredentialPropertiesOutput`*"]
    #[wasm_bindgen(method, getter = "credProps")]
    pub fn get_cred_props(
        this: &AuthenticationExtensionsClientOutputs,
    ) -> Option<CredentialPropertiesOutput>;
    #[cfg(feature = "CredentialPropertiesOutput")]
    #[doc = "Change the `credProps` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientOutputs`, `CredentialPropertiesOutput`*"]
    #[wasm_bindgen(method, setter = "credProps")]
    pub fn set_cred_props(
        this: &AuthenticationExtensionsClientOutputs,
        val: &CredentialPropertiesOutput,
    );
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "AuthenticationExtensionsDevicePublicKeyOutputs")]
    #[doc = "Get the `devicePubKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientOutputs`, `AuthenticationExtensionsDevicePublicKeyOutputs`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "devicePubKey")]
    pub fn get_device_pub_key(
        this: &AuthenticationExtensionsClientOutputs,
    ) -> Option<AuthenticationExtensionsDevicePublicKeyOutputs>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "AuthenticationExtensionsDevicePublicKeyOutputs")]
    #[doc = "Change the `devicePubKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientOutputs`, `AuthenticationExtensionsDevicePublicKeyOutputs`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "devicePubKey")]
    pub fn set_device_pub_key(
        this: &AuthenticationExtensionsClientOutputs,
        val: &AuthenticationExtensionsDevicePublicKeyOutputs,
    );
    #[cfg(feature = "AuthenticationExtensionsLargeBlobOutputs")]
    #[doc = "Get the `largeBlob` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientOutputs`, `AuthenticationExtensionsLargeBlobOutputs`*"]
    #[wasm_bindgen(method, getter = "largeBlob")]
    pub fn get_large_blob(
        this: &AuthenticationExtensionsClientOutputs,
    ) -> Option<AuthenticationExtensionsLargeBlobOutputs>;
    #[cfg(feature = "AuthenticationExtensionsLargeBlobOutputs")]
    #[doc = "Change the `largeBlob` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientOutputs`, `AuthenticationExtensionsLargeBlobOutputs`*"]
    #[wasm_bindgen(method, setter = "largeBlob")]
    pub fn set_large_blob(
        this: &AuthenticationExtensionsClientOutputs,
        val: &AuthenticationExtensionsLargeBlobOutputs,
    );
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "AuthenticationExtensionsPrfOutputs")]
    #[doc = "Get the `prf` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientOutputs`, `AuthenticationExtensionsPrfOutputs`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "prf")]
    pub fn get_prf(
        this: &AuthenticationExtensionsClientOutputs,
    ) -> Option<AuthenticationExtensionsPrfOutputs>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "AuthenticationExtensionsPrfOutputs")]
    #[doc = "Change the `prf` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientOutputs`, `AuthenticationExtensionsPrfOutputs`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "prf")]
    pub fn set_prf(
        this: &AuthenticationExtensionsClientOutputs,
        val: &AuthenticationExtensionsPrfOutputs,
    );
    #[doc = "Get the `uvm` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientOutputs`*"]
    #[wasm_bindgen(method, getter = "uvm")]
    pub fn get_uvm(this: &AuthenticationExtensionsClientOutputs) -> Option<::js_sys::Array>;
    #[doc = "Change the `uvm` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientOutputs`*"]
    #[wasm_bindgen(method, setter = "uvm")]
    pub fn set_uvm(this: &AuthenticationExtensionsClientOutputs, val: &::wasm_bindgen::JsValue);
}
impl AuthenticationExtensionsClientOutputs {
    #[doc = "Construct a new `AuthenticationExtensionsClientOutputs`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientOutputs`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_appid()` instead."]
    pub fn appid(&mut self, val: bool) -> &mut Self {
        self.set_appid(val);
        self
    }
    #[deprecated = "Use `set_appid_exclude()` instead."]
    pub fn appid_exclude(&mut self, val: bool) -> &mut Self {
        self.set_appid_exclude(val);
        self
    }
    #[cfg(feature = "CredentialPropertiesOutput")]
    #[deprecated = "Use `set_cred_props()` instead."]
    pub fn cred_props(&mut self, val: &CredentialPropertiesOutput) -> &mut Self {
        self.set_cred_props(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "AuthenticationExtensionsDevicePublicKeyOutputs")]
    #[deprecated = "Use `set_device_pub_key()` instead."]
    pub fn device_pub_key(
        &mut self,
        val: &AuthenticationExtensionsDevicePublicKeyOutputs,
    ) -> &mut Self {
        self.set_device_pub_key(val);
        self
    }
    #[cfg(feature = "AuthenticationExtensionsLargeBlobOutputs")]
    #[deprecated = "Use `set_large_blob()` instead."]
    pub fn large_blob(&mut self, val: &AuthenticationExtensionsLargeBlobOutputs) -> &mut Self {
        self.set_large_blob(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "AuthenticationExtensionsPrfOutputs")]
    #[deprecated = "Use `set_prf()` instead."]
    pub fn prf(&mut self, val: &AuthenticationExtensionsPrfOutputs) -> &mut Self {
        self.set_prf(val);
        self
    }
    #[deprecated = "Use `set_uvm()` instead."]
    pub fn uvm(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_uvm(val);
        self
    }
}
impl Default for AuthenticationExtensionsClientOutputs {
    fn default() -> Self {
        Self::new()
    }
}
