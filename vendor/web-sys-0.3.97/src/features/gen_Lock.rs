#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "Lock",
        typescript_type = "Lock"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Lock` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Lock)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Lock`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type Lock;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, getter, js_class = "Lock", js_name = "name")]
    #[doc = "Getter for the `name` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Lock/name)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Lock`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn name(this: &Lock) -> ::alloc::string::String;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "LockMode")]
    #[wasm_bindgen(method, getter, js_class = "Lock", js_name = "mode")]
    #[doc = "Getter for the `mode` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Lock/mode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Lock`, `LockMode`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn mode(this: &Lock) -> LockMode;
}
