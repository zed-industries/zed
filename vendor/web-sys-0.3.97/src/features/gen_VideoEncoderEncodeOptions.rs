#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "VideoEncoderEncodeOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `VideoEncoderEncodeOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoEncoderEncodeOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type VideoEncoderEncodeOptions;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `keyFrame` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoEncoderEncodeOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "keyFrame")]
    pub fn get_key_frame(this: &VideoEncoderEncodeOptions) -> Option<bool>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `keyFrame` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoEncoderEncodeOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "keyFrame")]
    pub fn set_key_frame(this: &VideoEncoderEncodeOptions, val: bool);
}
#[cfg(web_sys_unstable_apis)]
impl VideoEncoderEncodeOptions {
    #[doc = "Construct a new `VideoEncoderEncodeOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoEncoderEncodeOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_key_frame()` instead."]
    pub fn key_frame(&mut self, val: bool) -> &mut Self {
        self.set_key_frame(val);
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for VideoEncoderEncodeOptions {
    fn default() -> Self {
        Self::new()
    }
}
