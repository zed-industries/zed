#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "XRSessionInit")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `XrSessionInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrSessionInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type XrSessionInit;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `optionalFeatures` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrSessionInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "optionalFeatures")]
    pub fn get_optional_features(this: &XrSessionInit) -> Option<::js_sys::Array>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `optionalFeatures` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrSessionInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "optionalFeatures")]
    pub fn set_optional_features(this: &XrSessionInit, val: &[::wasm_bindgen::JsValue]);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `requiredFeatures` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrSessionInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "requiredFeatures")]
    pub fn get_required_features(this: &XrSessionInit) -> Option<::js_sys::Array>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `requiredFeatures` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrSessionInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "requiredFeatures")]
    pub fn set_required_features(this: &XrSessionInit, val: &[::wasm_bindgen::JsValue]);
}
#[cfg(web_sys_unstable_apis)]
impl XrSessionInit {
    #[doc = "Construct a new `XrSessionInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrSessionInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_optional_features()` instead."]
    pub fn optional_features(&mut self, val: &[::wasm_bindgen::JsValue]) -> &mut Self {
        self.set_optional_features(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_required_features()` instead."]
    pub fn required_features(&mut self, val: &[::wasm_bindgen::JsValue]) -> &mut Self {
        self.set_required_features(val);
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for XrSessionInit {
    fn default() -> Self {
        Self::new()
    }
}
