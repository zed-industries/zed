#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "AuthenticationExtensionsPRFOutputs"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AuthenticationExtensionsPrfOutputs` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsPrfOutputs`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type AuthenticationExtensionsPrfOutputs;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `enabled` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsPrfOutputs`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "enabled")]
    pub fn get_enabled(this: &AuthenticationExtensionsPrfOutputs) -> Option<bool>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `enabled` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsPrfOutputs`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "enabled")]
    pub fn set_enabled(this: &AuthenticationExtensionsPrfOutputs, val: bool);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "AuthenticationExtensionsPrfValues")]
    #[doc = "Get the `results` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsPrfOutputs`, `AuthenticationExtensionsPrfValues`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "results")]
    pub fn get_results(
        this: &AuthenticationExtensionsPrfOutputs,
    ) -> Option<AuthenticationExtensionsPrfValues>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "AuthenticationExtensionsPrfValues")]
    #[doc = "Change the `results` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsPrfOutputs`, `AuthenticationExtensionsPrfValues`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "results")]
    pub fn set_results(
        this: &AuthenticationExtensionsPrfOutputs,
        val: &AuthenticationExtensionsPrfValues,
    );
}
#[cfg(web_sys_unstable_apis)]
impl AuthenticationExtensionsPrfOutputs {
    #[doc = "Construct a new `AuthenticationExtensionsPrfOutputs`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsPrfOutputs`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_enabled()` instead."]
    pub fn enabled(&mut self, val: bool) -> &mut Self {
        self.set_enabled(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "AuthenticationExtensionsPrfValues")]
    #[deprecated = "Use `set_results()` instead."]
    pub fn results(&mut self, val: &AuthenticationExtensionsPrfValues) -> &mut Self {
        self.set_results(val);
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for AuthenticationExtensionsPrfOutputs {
    fn default() -> Self {
        Self::new()
    }
}
