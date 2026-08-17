#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "ClipboardUnsanitizedFormats")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ClipboardUnsanitizedFormats` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ClipboardUnsanitizedFormats`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type ClipboardUnsanitizedFormats;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `unsanitized` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ClipboardUnsanitizedFormats`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "unsanitized")]
    pub fn get_unsanitized(
        this: &ClipboardUnsanitizedFormats,
    ) -> Option<::js_sys::Array<::js_sys::JsString>>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `unsanitized` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ClipboardUnsanitizedFormats`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "unsanitized")]
    pub fn set_unsanitized(this: &ClipboardUnsanitizedFormats, val: &[::js_sys::JsString]);
}
#[cfg(web_sys_unstable_apis)]
impl ClipboardUnsanitizedFormats {
    #[doc = "Construct a new `ClipboardUnsanitizedFormats`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ClipboardUnsanitizedFormats`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_unsanitized()` instead."]
    pub fn unsanitized(&mut self, val: &[::js_sys::JsString]) -> &mut Self {
        self.set_unsanitized(val);
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for ClipboardUnsanitizedFormats {
    fn default() -> Self {
        Self::new()
    }
}
