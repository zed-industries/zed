#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "AudioSinkOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AudioSinkOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioSinkOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type AudioSinkOptions;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "AudioSinkType")]
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioSinkOptions`, `AudioSinkType`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "type")]
    pub fn get_type(this: &AudioSinkOptions) -> AudioSinkType;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "AudioSinkType")]
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioSinkOptions`, `AudioSinkType`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "type")]
    pub fn set_type(this: &AudioSinkOptions, val: AudioSinkType);
}
#[cfg(web_sys_unstable_apis)]
impl AudioSinkOptions {
    #[cfg(feature = "AudioSinkType")]
    #[doc = "Construct a new `AudioSinkOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioSinkOptions`, `AudioSinkType`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(type_: AudioSinkType) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_type(type_);
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "AudioSinkType")]
    #[deprecated = "Use `set_type()` instead."]
    pub fn type_(&mut self, val: AudioSinkType) -> &mut Self {
        self.set_type(val);
        self
    }
}
