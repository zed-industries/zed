#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "LockManagerSnapshot")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `LockManagerSnapshot` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `LockManagerSnapshot`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type LockManagerSnapshot;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "LockInfo")]
    #[doc = "Get the `held` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `LockInfo`, `LockManagerSnapshot`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "held")]
    pub fn get_held(this: &LockManagerSnapshot) -> Option<::js_sys::Array<LockInfo>>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "LockInfo")]
    #[doc = "Change the `held` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `LockInfo`, `LockManagerSnapshot`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "held")]
    pub fn set_held(this: &LockManagerSnapshot, val: &[LockInfo]);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "LockInfo")]
    #[doc = "Get the `pending` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `LockInfo`, `LockManagerSnapshot`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "pending")]
    pub fn get_pending(this: &LockManagerSnapshot) -> Option<::js_sys::Array<LockInfo>>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "LockInfo")]
    #[doc = "Change the `pending` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `LockInfo`, `LockManagerSnapshot`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "pending")]
    pub fn set_pending(this: &LockManagerSnapshot, val: &[LockInfo]);
}
#[cfg(web_sys_unstable_apis)]
impl LockManagerSnapshot {
    #[doc = "Construct a new `LockManagerSnapshot`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `LockManagerSnapshot`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "LockInfo")]
    #[deprecated = "Use `set_held()` instead."]
    pub fn held(&mut self, val: &[LockInfo]) -> &mut Self {
        self.set_held(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "LockInfo")]
    #[deprecated = "Use `set_pending()` instead."]
    pub fn pending(&mut self, val: &[LockInfo]) -> &mut Self {
        self.set_pending(val);
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for LockManagerSnapshot {
    fn default() -> Self {
        Self::new()
    }
}
